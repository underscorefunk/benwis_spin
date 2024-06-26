use crate::functions;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Login(action: Action<functions::auth::Login, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
      <Meta property="og:title" content="Login"/>
      <Title text="Login"/>
      <Meta name="description" content="Login to the site"/>
      <Meta property="og:description" content="Login to the site"/>
      <Meta
        property="og:image"
        content="https://benwis.imgix.net/pictureofMe.jpeg"
      />
      <div class="flex min-h-full flex-col justify-center">
        <div class="mx-auto w-full max-w-md px-8">
          <h1 class="mb-4 text-3xl text-center font-bold tracking-tight text-black dark:text-white md:text-5xl">
            "Login"
          </h1>
          <ActionForm action=action class="space-y-6">
            <div>
              <label
                for="username"
                class="block text-sm font-medium text-gray-700 dark:text-white"
              >
                "Username"
              </label>
              <div class="mt-1">
                <input
                  id="username"
                  required
                  name="username"
                  type="username"
                  aria-describedby="username-error"
                  class="w-full rounded border border-gray-500 px-2 py-1 text-lg"
                />
              </div>
            </div>
            <div>
              <label
                for="password"
                class="block text-sm font-medium text-gray-700 dark:text-white"
              >
                "Password"
              </label>
              <div class="mt-1">
                <input
                  id="password"
                  name="password"
                  type="password"
                  autoComplete="current-password"
                  aria-describedby="password-error"
                  class="w-full rounded border border-gray-500 px-2 py-1 text-lg"
                />
              </div>
            </div>
            <button
              type="submit"
              class="w-full rounded bg-yellow-400 py-2 px-4 text-white dark:text-gray-700 hover:bg-yellow-600 focus:bg-yellow-400"
            >
              "Log in"
            </button>
            <div class="flex items-center justify-between">
              <div class="flex items-center">
                <input
                  id="remember"
                  name="remember"
                  type="checkbox"
                  class="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                />
                <label
                  for="remember"
                  class="ml-2 block text-sm text-gray-900 dark:text-white"
                >
                  "Remember me"
                </label>
              </div>
              <div class="text-center text-sm text-gray-500">
                "Don't have an account?"
                <a class="text-blue-500 underline" href="">
                  "Sign up"
                </a>
              </div>
            </div>
          </ActionForm>
        </div>
      </div>
    }
}

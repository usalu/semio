// #region 🧲️Header
// 💻️ .storybook/playwright.config.ts
// Specs: Run Playwright end-to-end coverage against the built workspace Storybook.
// Summary: `bun run test:storybook` builds, serves `storybook-static/` via `script.ts dev storybook-static`, then runs the owner-scoped Storybook cases with `PLAYWRIGHT_BASE_URL` set; this config does not start its own server.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

// #region 🔌️Adapters
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { defineConfig, devices } from "@playwright/test";

import { playwrightTestTimeoutMs } from "@semio-tech/repo-lib";
// #endregion 🔌️Adapters

const storybookDir = resolve(fileURLToPath(import.meta.url), "..");
const workspaceRoot = resolve(storybookDir, "..");
const storybookPort = process.env.STORYBOOK_PORT ?? "6010";
const playwrightTimeoutMs = playwrightTestTimeoutMs();
function withTrailingSlash(url: string): string {
  return url.endsWith("/") ? url : `${url}/`;
}
/** Trailing `/` so `page.goto("iframe.html")` resolves at the static server root. */
const baseURL = withTrailingSlash(process.env.PLAYWRIGHT_BASE_URL ?? `http://127.0.0.1:${storybookPort}/`);

export default defineConfig({
  testDir: workspaceRoot,
  testMatch: [
    "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/🧪️tests/🎞️storybook-deck/🟦️.ts",
    "✏️s/🔌️plugins/📐️cad/🧪️tests/🎨️storybook-renderer/🟦️.ts",
    "🧰️framework/🛜d️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📚️storybook-hosts-no-wasm/🟦️.ts",
    "🧰️framework/🛜d️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📚️storybook-hosts-wasm/🟦️.ts",
    "🧰️framework/🛜d️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/📚️storybook-plugins/🟦️.ts",
    "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/◻️storybook-2d/🟦️.ts",
    "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧊️storybook-spatial/🟦️.ts",
    "✏️s/🧪️tests/🎭️storybook-end-to-end/🟦️.ts",
    "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🎨️storybook/🟦️.ts",
    "🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-new-stories/🟦️.ts",
    "🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-uncovered-components/🟦️.ts",
  ],
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  timeout: playwrightTimeoutMs,
  expect: { timeout: Math.min(playwrightTimeoutMs, 120_000) },
  workers: 1,
  reporter: [["list"]],
  use: {
    baseURL,
    trace: "on-first-retry",
  },
  projects: [
    {
      name: "chromium",
      use: {
        ...devices["Desktop Chrome"],
        launchOptions: {
          args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"],
        },
      },
    },
  ],
});

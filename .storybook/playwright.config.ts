// #region 🧲Header
// 💻 .storybook/playwright.config.ts
// Specs: Run Playwright board end-to-end coverage against the built workspace board harness.
// Summary: `bun run test:storybook` builds, serves `storybook-static/` via `script.ts dev storybook-static`, then runs Playwright against board-only end-to-end specs with `PLAYWRIGHT_BASE_URL` set; this config does not start its own server.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

// #region 🔌Adapters
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { defineConfig, devices } from "@playwright/test";
// #endregion 🔌Adapters

const storybookDir = resolve(fileURLToPath(import.meta.url), "..");
const storybookPort = process.env.STORYBOOK_PORT ?? "6010";
function withTrailingSlash(url: string): string {
  return url.endsWith("/") ? url : `${url}/`;
}
/** Trailing `/` so `page.goto("iframe.html")` resolves at the static server root. */
const baseURL = withTrailingSlash(process.env.PLAYWRIGHT_BASE_URL ?? `http://127.0.0.1:${storybookPort}/`);

export default defineConfig({
  testDir: storybookDir,
  testMatch: ["puzzle-2d.spec.ts"],
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  timeout: 300000,
  expect: { timeout: 120_000 },
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
          args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader"],
        },
      },
    },
  ],
});

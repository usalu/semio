import {browserLaunchOptionsV1} from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🟦️.ts";
// #region 🧲️Header
// 💻️ .storybook/🧪️tests/🧪️browser-runner/🟦️.ts
// Specs: Run Playwright end-to-end coverage against the built workspace Storybook.
// Summary: `repo run repo-storybook:test-storybook` builds, serves `storybook-static/` via `script.ts dev storybook-static`, then runs the owner-scoped Storybook cases with `PLAYWRIGHT_BASE_URL` set; this config does not start its own server.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { defineConfig, devices } from "../🎭️playwright/🟦️.ts";

import { playwrightTestTimeoutMs } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️testing/🟦️.ts";
// #endregion 🔌️Adapters

const workspaceRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");
const storybookPort = process.env.STORYBOOK_PORT ?? "6010";
const playwrightTimeoutMs = playwrightTestTimeoutMs();
function withTrailingSlash(url: string): string {
  return url.endsWith("/") ? url : `${url}/`;
}
/** 🌐️ Trailing `/` so `page.goto("iframe.html")` resolves at the static server root. */
const baseURL = withTrailingSlash(process.env.PLAYWRIGHT_BASE_URL ?? `http://127.0.0.1:${storybookPort}/`);

export default defineConfig({
  testDir: resolve(dirname(fileURLToPath(import.meta.url)), "📚️suites"),
  outputDir: process.env.PLAYWRIGHT_OUTPUT_DIR ? resolve(workspaceRoot, process.env.PLAYWRIGHT_OUTPUT_DIR) : undefined,
  testMatch: ["🟦️.ts"],
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
          executablePath: process.env.STORYBOOK_CHROMIUM_PATH || undefined,
          args: browserLaunchOptionsV1().args,
        },
      },
    },
  ],
});

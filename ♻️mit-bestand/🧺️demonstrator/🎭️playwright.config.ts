// #region 🧲️Header
// 💻️ ♻️mit-bestand/🧺️demonstrator/🎭️playwright.config.ts
// Specs: Run Playwright acceptance coverage against a live "Entwerfen mit Bestand" demonstrator dev server.
// Summary: `bun nx run @semio-tech/mit-bestand-demonstrator:test e2e` spawns the demonstrator's own Vite dev
// server (`📜️script.ts`'s `runAcceptancePlaywright`), waits for it, then runs Playwright against every
// `*.acceptance.spec.ts` file in this directory with `PLAYWRIGHT_BASE_URL` set — mirrors
// `.storybook/playwright.config.ts`'s shape/timeout/Chromium launch args; this config does not start its own server.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

// #region 🔌️Adapters
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { defineConfig, devices } from "@playwright/test";

import { playwrightTestTimeoutMs } from "@semio-tech/repo-lib";
// #endregion 🔌️Adapters

const demonstratorDir = resolve(fileURLToPath(import.meta.url), "..");
const demonstratorPort = process.env.MIT_BESTAND_DEMONSTRATOR_PORT ?? "6029";
const playwrightTimeoutMs = playwrightTestTimeoutMs();
function withTrailingSlash(url: string): string {
  return url.endsWith("/") ? url : `${url}/`;
}
const baseURL = withTrailingSlash(process.env.PLAYWRIGHT_BASE_URL ?? `http://127.0.0.1:${demonstratorPort}/`);

export default defineConfig({
  testDir: demonstratorDir,
  testMatch: ["*.acceptance.spec.ts"],
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

// #region 🔖Header
// [👤semio📚js⚙️playwrightconfig](repo://p/u/semio/b/l/js/f/playwright.config.ts)

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Playwright end-to-end test configuration for the JavaScript workspace.

// #endregion 🔖Header

// #region 🔖Playwright Configuration
// [👤semio📚js⚙️playwrightconfig🔖playwrightconfiguration](repo://p/u/semio/b/l/js/f/playwright.config.ts/s/Playwright%20Configuration)
// Configures Playwright for end-to-end browser tests against the sketchpad dev server.
// MUST use a single worker to avoid port conflicts.
// Uses esbuild-based ESM loader hook (pw-loader.mjs) to handle CSS stubs, TypeScript
// type stripping, Vite import.meta.glob stubs, and JSON imports without type attributes.

import { fileURLToPath } from "node:url";
import { resolve } from "node:path";

const __dirname = resolve(fileURLToPath(import.meta.url), "..");
const loaderPath = resolve(__dirname, "pw-loader.mjs");
process.env.NODE_OPTIONS = `${process.env.NODE_OPTIONS || ""} --import file://${loaderPath}`.trim();

import { defineConfig, devices } from "@playwright/test";

const baseURL = process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:4181";

export default defineConfig({
  testMatch: ["**/*.spec.ts", "**/index.tsx"],
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  timeout: 300000,
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

  webServer: {
    command: "NODE_OPTIONS='' npx vite preview --port 4181 --host 0.0.0.0",
    url: baseURL,
    reuseExistingServer: !process.env.CI,
    timeout: 300000,
  },
});

// #endregion 🔖Playwright Configuration

// #region 🔖Header

// [⚙️semio/js/playwright.config.ts](semiorepo://file/semio/js/playwright.config.ts)

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Playwright end-to-end test configuration for the JavaScript workspace.

// #endregion 🔖Header

// #region 🔖Playwright Configuration
// Configures Playwright for end-to-end browser tests against the sketchpad dev server.
// MUST use a single worker to avoid port conflicts.

import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testMatch: ["**/*.spec.ts", "**/sketchpad.test.ts"],
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: 1,
  reporter: [["list"], ["json", { outputFile: "../../reports/playwright.json" }]],
  use: {
    baseURL: "http://localhost:5173",
    trace: "on-first-retry",
  },
  webServer: {
    command: "npx nx run @semio/js:dev:sketchpad",
    url: "http://localhost:5173",
    reuseExistingServer: !process.env.CI,
  },
  projects: [
    {
      name: "chromium",
      use: {
        ...devices["Desktop Chrome"],
        launchOptions: {
          args: [
            "--enable-gpu",
            "--disable-software-rasterizer",
          ],
        },
      },
    },

  ],

  webServer: {
    command: "npm run dev:sketchpad -- --port 5173",
    url: "http://localhost:5173",
    reuseExistingServer: !process.env.CI,
    timeout: 120000,
  },
});

// #endregion 🔖Playwright Configuration

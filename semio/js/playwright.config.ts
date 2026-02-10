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

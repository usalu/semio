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
  projects: [
    {
      name: "chromium",
      use: {
        ...devices["Desktop Chrome"],
        channel: "chrome",
        launchOptions: {
          args: [
            "--disable-gpu",
            "--disable-dev-shm-usage",
            "--no-sandbox",
            "--disable-setuid-sandbox",
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

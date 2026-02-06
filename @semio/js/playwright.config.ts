import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testMatch: "**/sketchpad.test.ts",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: [["list"], ["json", { outputFile: "../../reports/playwright.json" }]],
  use: {
    baseURL: "http://localhost:3000",
    trace: "on-first-retry",
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

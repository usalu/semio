import { defineConfig, devices } from "@playwright/test";

// https://playwright.dev/docs/test-configuration.
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
      name: "firefox",
      use: { ...devices["Desktop Firefox"], headless: true },
    },

    // {
    //   name: "chromium",
    //   use: {
    //     ...devices["Desktop Chrome"],
    //     headless: true,
    //     launchOptions: {
    //       args: ["--disable-gpu", "--no-sandbox", "--disable-setuid-sandbox"],
    //     },
    //   },
    // },

    // {
    //   name: "webkit",
    //   use: { ...devices["Desktop Safari"] },
    // },

    /* Test against mobile viewports. */
    // {
    //   name: 'Mobile Chrome',
    //   use: { ...devices['Pixel 5'] },
    // },
    // {
    //   name: 'Mobile Safari',
    //   use: { ...devices['iPhone 12'] },
    // },

    /* Test against branded browsers. */
    // {
    //   name: 'Microsoft Edge',
    //   use: { ...devices['Desktop Edge'], channel: 'msedge' },
    // },
    // {
    //   name: 'Google Chrome',
    //   use: { ...devices['Desktop Chrome'], channel: 'chrome' },
    // },
  ],

  webServer: {
    command: "npm run dev:sketchpad -- --port 5173",
    url: "http://localhost:5173",
    reuseExistingServer: !process.env.CI,
    timeout: 120000,
  },
});

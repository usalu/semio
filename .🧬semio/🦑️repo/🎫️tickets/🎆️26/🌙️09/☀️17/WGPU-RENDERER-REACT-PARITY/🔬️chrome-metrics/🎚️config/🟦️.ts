import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { defineConfig, devices } from "@playwright/test";

const directory = dirname(fileURLToPath(import.meta.url));
const baseURL = process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:6313/";

export default defineConfig({
  testDir: resolve(directory, "../🧪️draft"),
  outputDir: resolve(directory, "../../../🗑️generated/astra-runtime/chrome-metrics-playwright"),
  testMatch: ["🟦️.ts"],
  fullyParallel: false,
  workers: 1,
  reporter: [["list"]],
  use: { baseURL, trace: "retain-on-failure" },
  projects: [
    {
      name: "chromium",
      use: {
        ...devices["Desktop Chrome"],
        launchOptions: { args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader"] },
      },
    },
  ],
});

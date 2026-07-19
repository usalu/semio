// Scratch-only Playwright config for verifying .storybook/styling.spec.ts during ticket work.
// Not part of the shared .storybook/playwright.config.ts (owned by a separate workstream, testMatch
// there is hardcoded to "puzzle-2d.spec.ts" only) — this just points testMatch at styling.spec.ts
// against the same testDir/baseURL contract so it can be verified without editing shared config.
import { defineConfig, devices } from "@playwright/test";
import { resolve } from "node:path";

const storybookDir = resolve(process.cwd(), ".storybook");
const storybookPort = process.env.STORYBOOK_PORT ?? "6010";
const baseURL = process.env.PLAYWRIGHT_BASE_URL ?? `http://127.0.0.1:${storybookPort}/`;

export default defineConfig({
  testDir: storybookDir,
  testMatch: ["styling.spec.ts"],
  fullyParallel: false,
  timeout: 300000,
  expect: { timeout: 120_000 },
  workers: 1,
  reporter: [["list"]],
  use: { baseURL, trace: "off" },
  projects: [
    {
      name: "chromium",
      use: {
        ...devices["Desktop Chrome"],
        launchOptions: { args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] },
      },
    },
  ],
});

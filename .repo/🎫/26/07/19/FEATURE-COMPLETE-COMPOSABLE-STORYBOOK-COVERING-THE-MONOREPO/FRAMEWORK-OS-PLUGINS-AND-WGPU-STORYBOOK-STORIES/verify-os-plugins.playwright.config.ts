// 🧪 Ticket-scoped temp verification config — runs .storybook/os-plugins.spec.ts standalone against
// an already-running dev Storybook instance without touching the shared .storybook/playwright.config.ts
// (whose testMatch is owned by another workstream). Not a permanent project file.
import { defineConfig, devices } from "@playwright/test";

const storybookPort = process.env.STORYBOOK_PORT ?? "6055";

export default defineConfig({
  testDir: "/Users/ueli/Documents/semio/.storybook",
  testMatch: ["os-plugins.spec.ts"],
  fullyParallel: false,
  retries: 0,
  timeout: 120000,
  expect: { timeout: 60000 },
  workers: 1,
  reporter: [["list"]],
  use: {
    baseURL: `http://127.0.0.1:${storybookPort}/`,
    trace: "off",
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

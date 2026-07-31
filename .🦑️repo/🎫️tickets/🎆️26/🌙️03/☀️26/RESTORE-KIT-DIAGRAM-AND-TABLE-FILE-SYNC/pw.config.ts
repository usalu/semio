process.env.NODE_OPTIONS = process.env.NODE_OPTIONS = `${process.env.NODE_OPTIONS || ""} --import /workspaces/semio/.repo/🎫️/26/03/26/RESTORE-KIT-DIAGRAM-AND-TABLE-FILE-SYNC/css-hook.mjs`.trim();

import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testDir: "/workspaces/semio/compose/sketchpad",
  testMatch: ["**/index.tsx"],
  fullyParallel: false,
  timeout: 300000,
  workers: 1,
  reporter: [["list"]],
  use: {
    baseURL: "http://localhost:5173",
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
});

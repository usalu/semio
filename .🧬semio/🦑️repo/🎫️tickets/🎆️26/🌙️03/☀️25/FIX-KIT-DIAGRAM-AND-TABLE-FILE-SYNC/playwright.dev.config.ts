import { defineConfig, devices } from "@playwright/test";

process.env.NODE_OPTIONS =
  `${process.env.NODE_OPTIONS || ""} --import data:text/javascript,export%20async%20function%20load(url,context,nextLoad)%7Bif(url.endsWith(%22.css%22))return%7Bformat:%22module%22,shortCircuit:true,source:%22export%20default%20%7B%7D%22%7D;return%20nextLoad(url,context)%7D`.trim();

export default defineConfig({
  testDir: "/workspaces/semio/compose/sketchpad",
  testMatch: ["index.tsx"],
  fullyParallel: false,
  timeout: 300000,
  workers: 1,
  reporter: [["list"]],
  use: {
    baseURL: "http://127.0.0.1:4173",
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

// #region 🧲️Header
// 💻️ ♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/🎚️config/🟦️.ts
// Specs: Run Playwright acceptance coverage against a live "Entwerfen mit Bestand" demonstrator dev server.
// Summary: Nx prepares an isolated continuous service; the E2E consumer validates its generation
// before supplying PLAYWRIGHT_BASE_URL. Playwright consumes that server and does not start one.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { defineConfig, devices } from "@playwright/test";

import { playwrightTestTimeoutMs } from "@semio-tech/repo-lib";
// #endregion 🔌️Adapters

const demonstratorDir = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");
const playwrightTimeoutMs = playwrightTestTimeoutMs();
function withTrailingSlash(url: string): string {
  return url.endsWith("/") ? url : `${url}/`;
}
if (!process.env.PLAYWRIGHT_BASE_URL) throw new Error("Run the Demonstrator test-e2e target through Nx");
const baseURL = withTrailingSlash(process.env.PLAYWRIGHT_BASE_URL);

/** 🖥️ The demonstrator's six panes are all GPU surfaces, and one of them only boots on a real adapter.
 *
 * `--use-angle=swiftshader` is enough for the r3f/WebGL World3d panes (aussuchen's grid paints its beams
 * under it), but the wasm/wgpu `TiledMapHost` never finishes `attachCanvas` on it: measured 2026-09-16
 * against the same serve, verfolgen's map requested ZERO tiles under swiftshader and 65 (`/osm/0/0/0.png`,
 * `/vt/3/0/1.pbf`, …) under ANGLE-Metal, where it paints continents, labels and the marker. So the
 * default is the machine's real adapter; `DEMONSTRATOR_E2E_GPU=swiftshader` forces the software stack
 * back for a host that has none, knowing the map pane cannot be graded there. */
function browserLaunchArgs(): readonly string[] {
  if (process.env.DEMONSTRATOR_E2E_GPU === "swiftshader") return ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"];
  return [process.platform === "darwin" ? "--use-angle=metal" : "--use-angle=gl", "--enable-gpu", "--ignore-gpu-blocklist", "--enable-unsafe-webgpu"];
}

export default defineConfig({
  testDir: resolve(demonstratorDir, "🧪️tests"),
  testMatch: ["🎭️acceptance/🟦️.ts"],
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  timeout: playwrightTimeoutMs,
  expect: { timeout: Math.min(playwrightTimeoutMs, 120_000) },
  workers: 1,
  reporter: [["list"]],
  use: {
    baseURL,
    trace: "on-first-retry",
  },
  projects: [
    {
      name: "chromium",
      use: {
        ...devices["Desktop Chrome"],
        launchOptions: {
          args: [...browserLaunchArgs()],
        },
      },
    },
  ],
});

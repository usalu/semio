// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Playwright end-to-end test configuration for the JavaScript workspace.

// #endregion 🧲Header

// #region 🪄Playwright Configuration
// Configures Playwright for end-to-end browser tests against the sketchpad dev server.
// MUST use a single worker to avoid port conflicts.
// Uses esbuild-based ESM loader hook (pw-loader.mjs) to handle CSS stubs, TypeScript
// type stripping, Vite import.meta.glob stubs, and JSON imports without type attributes.

// #region 🔌Adapters
import { register } from "node:module";
import { existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
// #endregion 🔌Adapters

const __dirname = dirname(fileURLToPath(import.meta.url));
register(pathToFileURL(resolve(__dirname, "pw-loader.mjs")), pathToFileURL(__dirname));

const prevNodeOpts = process.env.NODE_OPTIONS ?? "";
const nodeOptsParts = prevNodeOpts.split(/\s+/).filter(Boolean);
if (!nodeOptsParts.includes("--no-strip-types")) nodeOptsParts.push("--no-strip-types");
process.env.NODE_OPTIONS = nodeOptsParts.join(" ").trim();

import { defineConfig, devices } from "@playwright/test";

const baseURL = process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:4181";
const repoRoot = resolve(__dirname, "../../../../../");
const viteBinCandidates = [
  resolve(__dirname, "node_modules/vite/bin/vite.js"),
  resolve(repoRoot, "node_modules/vite/bin/vite.js"),
];
const viteBin = viteBinCandidates.find((candidate) => existsSync(candidate)) ?? viteBinCandidates[0]!;
const previewHost = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";

export default defineConfig({
  testDir: __dirname,
  testMatch: ["index.ts"],
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  timeout: 300000,
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
          args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader"],
        },
      },
    },
  ],

  webServer: {
    cwd: __dirname,
    command: `node "${viteBin}" preview --port 4181 --host ${previewHost}`,
    env: { NODE_OPTIONS: "" },
    url: baseURL,
    reuseExistingServer: !process.env.CI,
    timeout: 300000,
  },
});

// #endregion 🪄Playwright Configuration

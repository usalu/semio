// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Vitest configuration for the standalone semio React bundle.

// #endregion 🧲Header

// #region 🗄️Configuration
import path from "path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const reactDir = path.dirname(fileURLToPath(import.meta.url));
const rsPkgDir = path.resolve(reactDir, "../../rs/pkg");
const semioWasmBg = path.resolve(rsPkgDir, "semio_bg.wasm");

export default defineConfig({
  resolve: {
    alias: {
      "@semio/rs-wasm": path.resolve(rsPkgDir, "semio.js"),
    },
  },
  test: {
    name: "semio-react",
    environment: "jsdom",
    testTimeout: 120000,
    include: ["index.tsx"],
    exclude: ["**/node_modules/**", "**/dist/**", "**/.storybook/**"],
    maxWorkers: 1,
    fileParallelism: false,
    pool: "forks",
    isolate: true,
    execArgv: ["--max-old-space-size=16384"],
    /** Lets `@semio/js` `KitStore.open` load `semio_bg.wasm` when Vitest bundles `import.meta.url` away from `semio/js`. */
    env: { SEMIO_WASM_BG_PATH: semioWasmBg },
  },
});
// #endregion 🗄️Configuration
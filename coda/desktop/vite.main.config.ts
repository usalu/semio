// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Vite build configuration for the Electron main process.

// #endregion 🧲Header

// #region 🗄️Configuration
// Vite configuration for the Electron main process.
// Configuration MUST export a valid Vite config object.

import { builtinModules } from "module";
import { defineConfig } from "vite";

export default defineConfig({
  build: {
    outDir: ".vite/build",
    lib: {
      entry: "main.ts",
      formats: ["cjs"],
      fileName: () => "main.js",
    },
    rollupOptions: {
      external: ["electron", ...builtinModules, ...builtinModules.map((m) => `node:${m}`)],
    },
    emptyOutDir: false,
    sourcemap: "inline",
  },
});

// #endregion 🗄️Configuration

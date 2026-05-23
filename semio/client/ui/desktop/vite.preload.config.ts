// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Vite build configuration for the Electron preload script.

// #endregion 🧲Header

// #region 🗄️Configuration
// Vite configuration for building the Electron preload script as a CJS library.
// Configuration MUST externalize Electron and Node.js built-in modules.

import { defineConfig } from "vite";
import { builtinModules } from "module";
import path from "path";

// Vite build configuration for the preload script with CJS output.
// Export MUST externalize electron and all Node.js builtins.
export default defineConfig({
  server: {
    watch: {
      usePolling: true,
      interval: 1000,
    },
  },
  build: {
    outDir: ".vite/build",
    lib: {
      entry: "preload.ts",
      formats: ["cjs"],
      fileName: () => "preload.js",
    },
    rollupOptions: {
      external: ["electron", ...builtinModules],
    },
    emptyOutDir: false,
    sourcemap: "inline",
  },
});

// #endregion 🗄️Configuration

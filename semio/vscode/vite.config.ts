// #region 🔖Header
// [👤semio🖱️vscode⚙️viteconfig](semiorepo://p/u/semio/b/u/vscode/f/vite.config.ts)

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Vite build configuration for the semio VS Code extension bundling.

// #endregion 🔖Header

// #region 🔖Configuration
// [👤semio🖱️vscode⚙️viteconfig🔖configuration](semiorepo://p/u/semio/b/u/vscode/f/vite.config.ts/s/Configuration)
// Vite build configuration for the semio VS Code extension.
// Configuration MUST output a CJS bundle targeting Node 18.

import path from "path";
import { fileURLToPath } from "url";
import { defineConfig } from "vite";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export default defineConfig({
  build: {
    lib: {
      entry: path.resolve(__dirname, "extension.ts"),
      formats: ["cjs"],
      fileName: () => "extension",
    },
    rollupOptions: {
      external: ["vscode"],
      output: {
        entryFileNames: "extension.js",
        format: "cjs",
        sourcemap: true,
      },
    },
    outDir: "out",
    emptyOutDir: true,
    minify: false,
    sourcemap: true,
    target: "node18",
    ssr: true,
  },
  ssr: {
    noExternal: true,
  },
});
// #endregion 🔖Configuration

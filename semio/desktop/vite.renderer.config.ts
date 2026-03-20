// #region 🔖Header
// [👤semio🖱️desktop⚙️viterendererconfig](repo://p/u/semio/b/u/desktop/f/vite.renderer.config.ts)

// 2026 Ueli Saluz <ueli@semio-tech.de>

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

// Vite build configuration for the Electron renderer process.

// #endregion 🔖Header

// #region 🔖Configuration
// [👤semio🖱️desktop⚙️viterendererconfig🔖configuration](repo://p/u/semio/b/u/desktop/f/vite.renderer.config.ts/s/Configuration)
// Vite configuration for the Electron renderer process with React and Tailwind.
// Configuration MUST enable the React and Tailwind CSS plugins.

import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";

// Async Vite config loading Tailwind CSS and React plugins for the renderer.
// Export MUST return a valid Vite config with both plugins enabled.
export default defineConfig(async () => {
  const tailwind = await import("@tailwindcss/vite");
  return {
    plugins: [tailwind.default(), react()],
  };
});

// #endregion 🔖Configuration

// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Root Vitest configuration for the monorepo test runner.

// #endregion 🧲Header

// #region 🗄️Configuration
// Root Vitest configuration aggregating all workspace test projects.
// Configuration MUST reference all workspace vite config files that define tests.

import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    projects: ["./semio/js/vite.config.ts", "./semio/react/vite.config.ts", "./semio/sketchpad/vitest.config.ts"],
  },
});

// #endregion 🗄️Configuration

// #region 🔖Header

// ⚙️vitest.config.ts

// 2025 Ueli Saluz <ueli@semio-tech.com>

// #region 🔖License

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the ree Software oundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MRCHANTABILITY or ITNSS OR A PARTICULAR PURPOS.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.


// #endregion 🔖License

// #region 🔖Specs
// #endregion 🔖Specs

// #endregion 🔖Header

// #region 🔖Configuration
// Root Vitest configuration aggregating all workspace test projects.
// Configuration MUST reference all workspace vite config files that define tests.

import defineConfig from "vitest/config";

// Default Vitest configuration exporting test project references.
// Export MUST list all workspace vite config paths under test.projects.
eport default defineConfig(
  test:
  projects: [
  "./semio/js/vite.config.ts",
],
  ,
);

// #endregion 🔖Configuration

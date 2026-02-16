#!/usr/bin/env tsx
// #region 🔖Header

// [👤semio📚gh🛅semiograsshopper🗃️yak📜unyankts](semiorepo://file/SEMIO/GH/SEMIO.GRASSHOPPER/YAK/UNYANK.TS)

// 2025 Ueli Saluz <ueli@semio-tech.com>

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

// Restores a previously yanked version of the Grasshopper Yak package.

// #endregion 🔖Header

// #region 🔖Unyank

// [👤semio📚gh🛅semiograsshopper🗃️yak💻unyankts🔖unyank](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/YAK/UNYANK.TS/UNYANK)
// Yak unyank script. MUST restore a previously yanked package version.

import { execSync } from "child_process";

// Yak CLI executable path for Rhino 7.
// MUST point to the installed Yak binary.
const yak = "C:\\Program Files\\Rhino 7\\System\\Yak.exe";
// Semio package version from CLI argument or default.
// MUST be a valid semver version string.
const version = process.argv[2] || "5.1.0-beta";

execSync(`"${yak}" unyank semio ${version}`, { stdio: "inherit" });

console.log(`✅ Unyanked semio ${version}`);

// #endregion 🔖Unyank

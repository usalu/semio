#!/usr/bin/env tsx
// #region 🔖Header

// [👤semio📚gh🛅semiograsshopper🗃️yak📜yankts](semiorepo://file/SEMIO/GH/SEMIO.GRASSHOPPER/YAK/YANK.TS)

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

// Yanks a specific version of the Grasshopper Yak package from the registry.

// #endregion 🔖Header

// #region 🔖Yank

// [🔖semio/gh/Semio.Grasshopper/yak/yank.ts#Yank](semiorepo://section/semio/gh/Semio.Grasshopper/yak/yank.ts/YANK)
// Yak yank script. MUST remove a package version from the Yak server.

import { execSync } from "child_process";

// Yak CLI executable path for Rhino 7.
// MUST point to the installed Yak binary.
const yak = "C:\\Program Files\\Rhino 7\\System\\Yak.exe";
// Semio package version from CLI argument or default.
// MUST be a valid semver version string.
const version = process.argv[2] || "5.1.0-beta";

execSync(`"${yak}" yank semio ${version}`, { stdio: "inherit" });

console.log(`✅ Yanked semio ${version}`);

// #endregion 🔖Yank

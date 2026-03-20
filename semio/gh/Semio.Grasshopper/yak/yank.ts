#!/usr/bin/env tsx
// #region 🔖Header
// [👤semio📚gh🛅semiograsshopper🗃️yak📜yank](repo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/yank.ts)

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
// [👤semio📚gh🛅semiograsshopper🗃️yak💻yank🔖yank](repo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/yank.ts/s/Yank)
// Yak yank script. MUST remove a package version from the Yak server.

import { execSync } from "child_process";

/**
 * Yak CLI executable path for Rhino 7.
// [👤semio📚gh🛅semiograsshopper🗃️yak💻yank🔖yank🪨yak](repo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/yank.ts/s/Yank/d/i/yak)
 * MUST point to the installed Yak binary.
 **/
const yak = "C:\\Program Files\\Rhino 7\\System\\Yak.exe";
/**
 * Semio package version from CLI argument or default.
// [👤semio📚gh🛅semiograsshopper🗃️yak💻yank🔖yank🪨version](repo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/yank.ts/s/Yank/d/i/version)
 * MUST be a valid semver version string.
 **/
const version = process.argv[2] || "5.1.0-beta";

execSync(`"${yak}" yank semio ${version}`, { stdio: "inherit" });

console.log(`✅ Yanked semio ${version}`);

// #endregion 🔖Yank

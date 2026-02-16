#!/usr/bin/env tsx
// #region 🔖Header

// [👤semio📚gh🛅semiograsshopper🗃️yak📜testpushts](semiorepo://file/SEMIO/GH/SEMIO.GRASSHOPPER/YAK/TEST-PUSH.TS)

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

// Tests the Yak package push workflow for the Grasshopper plugin.

// #endregion 🔖Header

// #region 🔖Test Push

// [👤semio📚gh🛅semiograsshopper🗃️yak💻testpushts🔖testpush](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/YAK/TEST-PUSH.TS/TEST-PUSH)
// Yak test push script. MUST push the package to the test Yak server.

import { execSync } from "child_process";

// Yak CLI executable path for Rhino 8.
// MUST point to the installed Yak binary.
const yak = "C:\\Program Files\\Rhino 8\\System\\Yak.exe";
// Yak package filename from CLI argument or default.
// MUST resolve to a valid .yak package file.
const packageFile = process.argv[2] || "semio-2.1.0-any-win.yak";

execSync(`"${yak}" push --source https://test.yak.rhino3d.com ${packageFile}`, { stdio: "inherit" });

console.log(`✅ Pushed ${packageFile} to test server`);

// #endregion 🔖Test Push

#!/usr/bin/env tsx
// #region 🔖Header

// [👤semio📚gh🛅semiograsshopper🗃️yak📜testsearchts](semiorepo://file/SEMIO/GH/SEMIO.GRASSHOPPER/YAK/TEST-SEARCH.TS)

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

// Tests Yak package search functionality for the Grasshopper plugin.

// #endregion 🔖Header

// #region 🔖Script

// [👤semio📚gh🛅semiograsshopper🗃️yak💻testsearchts🔖script](semiorepo://section/semio/gh/semio.grasshopper/yak/test-search.ts/script)
// Test script for searching the Yak package manager test server.
// Script MUST execute yak search against the test.yak.rhino3d.com server.

import { execSync } from "child_process";

// Path to the Yak package manager executable.
// Yak path MUST point to the Rhino 8 System directory.
const yak = "C:\\Program Files\\Rhino 8\\System\\Yak.exe";
execSync(`"${yak}" search --source https://test.yak.rhino3d.com --all --prerelease semio`, { stdio: "inherit" });
// #endregion 🔖Script

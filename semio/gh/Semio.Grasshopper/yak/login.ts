#!/usr/bin/env tsx
// #region 🔖Header
// [👤semio📚gh🛅semiograsshopper🗃️yak📜login](repo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/login.ts)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Authenticates with the Yak package server for plugin publishing.

// #endregion 🔖Header

// #region 🔖Login
// [👤semio📚gh🛅semiograsshopper🗃️yak💻login🔖login](repo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/login.ts/s/Login)
// Yak login script. MUST authenticate with the Yak package manager.

import { execSync } from "child_process";

/**
 * Yak CLI executable path for Rhino 8.
// [👤semio📚gh🛅semiograsshopper🗃️yak💻login🔖login🪨yak](repo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/login.ts/s/Login/d/i/yak)
 * MUST point to the installed Yak binary.
 **/
const yak = "C:\\Program Files\\Rhino 8\\System\\Yak.exe";
execSync(`"${yak}" login`, { stdio: "inherit" });

console.log("✅ Logged in to Yak");

// #endregion 🔖Login

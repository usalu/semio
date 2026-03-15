#!/usr/bin/env tsx
// #region 🔖Header
// [👤semio📚net🛅semio📜build](semiorepo://p/u/semio/b/l/net/fd/req/Semio/f/build.ts)

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

// Build script for the Semio .NET library assembly.

// #endregion 🔖Header

// #region 🔖Build
// [👤semio📚net🛅semio💻build🔖build](semiorepo://p/u/semio/b/l/net/fd/req/Semio/f/build.ts/s/Build)
// .NET build script. MUST compile the Semio C# project via MSBuild.

import { execSync } from "child_process";

/**
 * MSBuild executable path for Visual Studio 2022.
// [👤semio📚net🛅semio💻build🔖build🪨msbuild](semiorepo://p/u/semio/b/l/net/fd/req/Semio/f/build.ts/s/Build/d/i/msbuild)
 * MUST point to the installed MSBuild binary.
 **/
const msbuild = "C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\MSBuild\\Current\\Bin\\MSBuild.exe";

execSync(`"${msbuild}" Semio.csproj /p:Configuration=Debug`, {
  cwd: __dirname,
  stdio: "inherit",
});

console.log("✅ Semio.NET build complete");

// #endregion 🔖Build

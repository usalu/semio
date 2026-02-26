#!/usr/bin/env tsx
// #region 🔖Header
// [👤semio📚gh🛅semiograsshopper📜build](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/build.ts)

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

// Build script for the Grasshopper plugin assembly.

// #endregion 🔖Header

// #region 🔖Build
// [👤semio📚gh🛅semiograsshopper💻build🔖build](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/build.ts/s/Build)
// Grasshopper build script. MUST compile the solution and copy artifacts to the Yak distribution folder.

import { execSync } from "child_process";
import { copyFileSync, existsSync, mkdirSync, readdirSync, rmSync } from "fs";
import { join } from "path";

/**
 * Grasshopper build working directory.
// [👤semio📚gh🛅semiograsshopper💻build🔖build🪨cwd](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/build.ts/s/Build/d/i/cwd)
 *
 * MUST resolve to the Grasshopper project folder.
 **/
const cwd = __dirname;

execSync("tsx ./build-value-lists.ts", { cwd, stdio: "inherit" });

/**
 * MSBuild executable path for Visual Studio 2022.
// [👤semio📚gh🛅semiograsshopper💻build🔖build🪨msbuild](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/build.ts/s/Build/d/i/msbuild)
 *
 * MUST point to the installed MSBuild binary.
 **/
const msbuild = "C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\MSBuild\\Current\\Bin\\MSBuild.exe";

execSync(`"${msbuild}" Semio.sln /t:Clean`, { cwd, stdio: "inherit" });
execSync(`"${msbuild}" Semio.sln /p:Configuration=Debug`, { cwd, stdio: "inherit" });

/**
 * Yak distribution output folder path.
// [👤semio📚gh🛅semiograsshopper💻build🔖build🪨yakdistfolder](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/build.ts/s/Build/d/i/yakDistFolder)
 *
 * MUST be cleaned and recreated before copying build artifacts.
 **/
const yakDistFolder = join(cwd, "..", "..", "yak", "dist");
if (existsSync(yakDistFolder)) {
  rmSync(yakDistFolder, { recursive: true });
}
mkdirSync(yakDistFolder, { recursive: true });

/**
 * Debug build output folder containing compiled binaries.
// [👤semio📚gh🛅semiograsshopper💻build🔖build🪨binfolder](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/build.ts/s/Build/d/i/binFolder)
 *
 * MUST contain the .NET Framework 4.8 build output.
 **/
const binFolder = join(cwd, "bin", "Debug", "net48");
/**
 * List of all files in the build output folder.
// [👤semio📚gh🛅semiograsshopper💻build🔖build🪨files](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/build.ts/s/Build/d/i/files)
 *
 * MUST be copied to the Yak distribution folder.
 **/
const files = readdirSync(binFolder);
for (const file of files) {
  copyFileSync(join(binFolder, file), join(yakDistFolder, file));
}

console.log("✅ Grasshopper build complete");

// #endregion 🔖Build

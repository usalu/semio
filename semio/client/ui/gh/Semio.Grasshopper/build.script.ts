#!/usr/bin/env bun
// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Build script for the Grasshopper plugin assembly.

// #endregion 🧲Header

// #region 🐹Build
// Grasshopper build script. MUST compile the solution and copy artifacts to the Yak distribution folder.

import { execFileSync } from "node:child_process";
import { cpSync, existsSync, mkdirSync, readdirSync, rmSync } from "fs";
import { join } from "path";

/**
 * Grasshopper build working directory.
 * MUST resolve to the Grasshopper project folder.
 **/
const cwd = __dirname;

execFileSync(process.execPath, ["x", "tsx", "./build-value-lists.script.ts"], { cwd, stdio: "inherit" });

execFileSync("dotnet", ["clean", "Semio.Grasshopper.csproj", "-c", "Debug"], { cwd, stdio: "inherit" });
execFileSync("dotnet", ["build", "Semio.Grasshopper.csproj", "-c", "Debug"], { cwd, stdio: "inherit" });

/**
 * Yak distribution output folder path.
 * MUST be cleaned and recreated before copying build artifacts.
 **/
const yakDistFolder = join(cwd, "yak", "dist");
if (existsSync(yakDistFolder)) {
  rmSync(yakDistFolder, { recursive: true });
}
mkdirSync(yakDistFolder, { recursive: true });

/**
 * Debug build output folder containing compiled binaries.
 * MUST contain the .NET Framework 4.8 build output.
 **/
const binFolder = join(cwd, "bin", "Debug", "net48");
/**
 * List of all files in the build output folder.
 * MUST be copied to the Yak distribution folder.
 **/
const files = readdirSync(binFolder);
for (const file of files) {
  cpSync(join(binFolder, file), join(yakDistFolder, file), { force: true, recursive: true });
}

console.log("✅ Grasshopper build complete");

// #endregion 🐹Build

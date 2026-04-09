#!/usr/bin/env tsx
// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Post-build script for engine artifact processing and packaging.

// #endregion 🧲Header

// #region 💾Post Build
// Post-build script. MUST relocate the PyInstaller output to the Grasshopper bin folder.

import { copyFileSync, cpSync, existsSync, rmSync } from "fs";
import { join } from "path";

/**
 * Post-build working directory.
 * MUST resolve to the engine folder.
 **/
const cwd = __dirname;
/**
 * Platform-specific executable extension.
 * Windows uses .exe, other platforms have no extension.
 **/
const exeExt = process.platform === "win32" ? ".exe" : "";
/**
 * Path to the PyInstaller-produced engine executable.
 * MUST match the PyInstaller output name.
 **/
const exePath = join(cwd, "dist", "semio-engine", `semio-engine${exeExt}`);
/**
 * Path to the PyInstaller internal dependencies folder.
 * MUST be co-located with the executable.
 **/
const internalPath = join(cwd, "dist", "semio-engine", "_internal");
/**
 * Grasshopper plugin binary output directory.
 * MUST match the .NET build output path.
 **/
const grasshopperBinPath = join(cwd, "..", "gh", "Semio.Grasshopper", "bin", "Debug", "net48");
/**
 * Target path for the engine executable in the Grasshopper bin folder.
 * MUST use the same executable name as the PyInstaller output.
 **/
const grasshopperExePath = join(grasshopperBinPath, `semio-engine${exeExt}`);
/**
 * Target path for the internal dependencies in the Grasshopper bin folder.
 * MUST mirror the PyInstaller _internal directory structure.
 **/
const grasshopperInternalPath = join(grasshopperBinPath, "_internal");

if (existsSync(grasshopperExePath)) {
  rmSync(grasshopperExePath);
}
if (existsSync(grasshopperInternalPath)) {
  rmSync(grasshopperInternalPath, { recursive: true });
}

copyFileSync(exePath, grasshopperExePath);
cpSync(internalPath, grasshopperInternalPath, { force: true, recursive: true });

console.log("✅ Post-build complete");

// #endregion 💾Post Build

#!/usr/bin/env tsx
// #region 🔖Header
// [👤semio📚engine📜postbuild](semiorepo://p/u/semio/b/l/engine/f/post-build.ts)

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

// Post-build script for engine artifact processing and packaging.

// #endregion 🔖Header

// #region 🔖Post Build
// [👤semio📚engine💻postbuild🔖postbuild](semiorepo://p/u/semio/b/l/engine/f/post-build.ts/s/Post%20Build)
// Post-build script. MUST relocate the PyInstaller output to the Grasshopper bin folder.

import { existsSync, renameSync, rmSync } from "fs";
import { join } from "path";

/**
 * Post-build working directory.
 *
 * MUST resolve to the engine folder.
 **/
/**
// cwd holds the data fields for a cwd record.
 * [👤semio📚engine💻postbuild🔖postbuild🪨cwd](semiorepo://p/u/semio/b/l/engine/f/post-build.ts/s/Post%20Build/d/i/cwd)
 **/
const cwd = __dirname;
/**
 * Path to the PyInstaller-produced engine executable.
 *
 * MUST match the PyInstaller output name.
 **/
/**
// exePath holds the data fields for a exePath record.
 * [👤semio📚engine💻postbuild🔖postbuild🪨exepath](semiorepo://p/u/semio/b/l/engine/f/post-build.ts/s/Post%20Build/d/i/exePath)
 **/
const exePath = join(cwd, "dist", "semio-engine", "semio-engine.exe");
/**
 * Path to the PyInstaller internal dependencies folder.
// [👤semio📚engine💻postbuild🔖postbuild🪨internalpath](semiorepo://p/u/semio/b/l/engine/f/post-build.ts/s/Post%20Build/d/i/internalPath)
 *
 * MUST be co-located with the executable.
 **/
const internalPath = join(cwd, "dist", "semio-engine", "_internal");
/**
 * Grasshopper plugin binary output directory.
// [👤semio📚engine💻postbuild🔖postbuild🪨grasshopperbinpath](semiorepo://p/u/semio/b/l/engine/f/post-build.ts/s/Post%20Build/d/i/grasshopperBinPath)
 *
 * MUST match the .NET build output path.
 **/
const grasshopperBinPath = join(cwd, "..", "..", "net", "Semio.Grasshopper", "bin", "Debug", "net48");
/**
 * Target path for the engine executable in the Grasshopper bin folder.
 *
 * MUST use the same executable name as the PyInstaller output.
 **/
/**
// grasshopperExePath holds the data fields for a grasshopperExePath record.
 * [👤semio📚engine💻postbuild🔖postbuild🪨grasshopperexepath](semiorepo://p/u/semio/b/l/engine/f/post-build.ts/s/Post%20Build/d/i/grasshopperExePath)
 **/
const grasshopperExePath = join(grasshopperBinPath, "semio-engine.exe");
/**
 * Target path for the internal dependencies in the Grasshopper bin folder.
 *
 * MUST mirror the PyInstaller _internal directory structure.
 **/
/**
// grasshopperInternalPath holds the data fields for a grasshopperInternalPath record.
 * [👤semio📚engine💻postbuild🔖postbuild🪨grasshopperinternalpath](semiorepo://p/u/semio/b/l/engine/f/post-build.ts/s/Post%20Build/d/i/grasshopperInternalPath)
 **/
const grasshopperInternalPath = join(grasshopperBinPath, "_internal");

if (existsSync(grasshopperExePath)) {
  rmSync(grasshopperExePath);
}
if (existsSync(grasshopperInternalPath)) {
  rmSync(grasshopperInternalPath, { recursive: true });
}

renameSync(exePath, grasshopperExePath);
renameSync(internalPath, grasshopperInternalPath);

console.log("✅ Post-build complete");

// #endregion 🔖Post Build

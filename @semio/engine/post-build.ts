#!/usr/bin/env tsx
// #region Header

// py/engine/post-build.ts

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion Header

import { existsSync, renameSync, rmSync } from "fs";
import { join } from "path";

const cwd = __dirname;
const exePath = join(cwd, "dist", "semio-engine", "semio-engine.exe");
const internalPath = join(cwd, "dist", "semio-engine", "_internal");
const grasshopperBinPath = join(cwd, "..", "..", "net", "Semio.Grasshopper", "bin", "Debug", "net48");
const grasshopperExePath = join(grasshopperBinPath, "semio-engine.exe");
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

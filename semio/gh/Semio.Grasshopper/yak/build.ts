#!/usr/bin/env tsx
// #region 🔖Header

// [👤semio📚gh🛅semiograsshopper🗃️yak📜buildts](semiorepo://file/SEMIO/GH/SEMIO.GRASSHOPPER/YAK/BUILD.TS)

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

// Build script for Yak package distribution of the Grasshopper plugin.

// #endregion 🔖Header

// #region 🔖Build

// [👤semio📚gh🛅semiograsshopper🗃️yak💻buildts🔖build](semiorepo://section/semio/gh/semio.grasshopper/yak/build.ts/build)
// Yak package build script. MUST prepare the distribution folder and build the .yak package.

import { execSync } from "child_process";
import { copyFileSync, existsSync, mkdirSync, rmSync } from "fs";
import { join } from "path";

// Yak build working directory.
// MUST resolve to the yak folder.
const cwd = __dirname;
// Distribution directory for the Yak package output.
// MUST be cleaned and prepared before building.
const distDir = join(cwd, "dist");

if (existsSync(join(distDir, "semio_512x512.png"))) {
  rmSync(join(distDir, "semio_512x512.png"));
}
if (existsSync(join(distDir, "manifest.yml"))) {
  rmSync(join(distDir, "manifest.yml"));
}

if (!existsSync(distDir)) {
  mkdirSync(distDir);
}

copyFileSync(join(cwd, "..", "assets", "icons", "semio_512x512.png"), join(distDir, "semio_512x512.png"));
copyFileSync(join(cwd, "manifest.yml"), join(distDir, "manifest.yml"));

// Yak CLI executable path for Rhino 8.
// MUST point to the installed Yak binary.
const yak = "C:\\Program Files\\Rhino 8\\System\\Yak.exe";
execSync(`"${yak}" build --platform win`, { cwd: distDir, stdio: "inherit" });

console.log("✅ Yak package built");

// #endregion 🔖Build

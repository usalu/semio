#!/usr/bin/env tsx
// #region 🔖Header

// 💻︎ semio/gh/Semio.Grasshopper/yak/build.ts

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

// #endregion 🔖Header

import { execSync } from "child_process";
import { copyFileSync, existsSync, mkdirSync, rmSync } from "fs";
import { join } from "path";

const cwd = __dirname;
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

const yak = "C:\\Program Files\\Rhino 8\\System\\Yak.exe";
execSync(`"${yak}" build --platform win`, { cwd: distDir, stdio: "inherit" });

console.log("✅ Yak package built");

#!/usr/bin/env tsx
// #region 🔖Header

// 📜semio/gh/Semio.Grasshopper/yak/publish.ts

// 2025 Ueli Saluz <ueli@semio-tech.com>

// #region 🔖License

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


// #endregion 🔖License

// #region 🔖Specs
// #endregion 🔖Specs

// #endregion 🔖Header

import { execSync } from "child_process";
import { readFileSync } from "fs";
import { join } from "path";

const cwd = join(__dirname, "dist");

const manifestContent = readFileSync(join(cwd, "manifest.yml"), "utf-8");
const versionMatch = manifestContent.match(/version:\s*(.+)/);
if (!versionMatch) {
  throw new Error("Could not find version in manifest.yml");
}
const version = versionMatch[1].trim();
const buildName = `semio-${version}-rh8_10-win.yak`;

const yak = "C:\\Program Files\\Rhino 8\\System\\Yak.exe";
execSync(`"${yak}" push ${buildName}`, { cwd, stdio: "inherit" });

console.log("✅ Yak package published");

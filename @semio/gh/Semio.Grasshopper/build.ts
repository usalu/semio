#!/usr/bin/env tsx
// #region 🔖Header

// net/Semio.Grasshopper/build.ts

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

// #endregion 🔖Header

import { execSync } from "child_process";
import { copyFileSync, existsSync, mkdirSync, readdirSync, rmSync } from "fs";
import { join } from "path";

const cwd = __dirname;

execSync("tsx ./build-value-lists.ts", { cwd, stdio: "inherit" });

const msbuild = "C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\MSBuild\\Current\\Bin\\MSBuild.exe";

execSync(`"${msbuild}" Semio.sln /t:Clean`, { cwd, stdio: "inherit" });
execSync(`"${msbuild}" Semio.sln /p:Configuration=Debug`, { cwd, stdio: "inherit" });

const yakDistFolder = join(cwd, "..", "..", "yak", "dist");
if (existsSync(yakDistFolder)) {
  rmSync(yakDistFolder, { recursive: true });
}
mkdirSync(yakDistFolder, { recursive: true });

const binFolder = join(cwd, "bin", "Debug", "net48");
const files = readdirSync(binFolder);
for (const file of files) {
  copyFileSync(join(binFolder, file), join(yakDistFolder, file));
}

console.log("✅ Grasshopper build complete");

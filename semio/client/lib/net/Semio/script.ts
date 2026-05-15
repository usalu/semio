#!/usr/bin/env bun
// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Build script for the Semio .NET library assembly.

// #endregion 🧲Header

// #region 🐹Build
import { spawnSync } from "node:child_process";

const cwd = import.meta.dir;
const segs = process.argv.slice(2);

if (segs[0] !== "build") {
  console.error("usage: bun ./script.ts build");
  process.exit(1);
}

const buildResult = spawnSync("dotnet", ["build", "Semio.csproj", "-c", "Debug"], {
  cwd,
  stdio: "inherit",
});

if (buildResult.status !== 0) {
  process.exit(buildResult.status ?? 1);
}

console.log("✅ Semio.NET build complete");

// #endregion 🐹Build

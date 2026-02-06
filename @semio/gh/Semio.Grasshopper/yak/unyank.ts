#!/usr/bin/env tsx
// #region 🔖Header

// yak/unyank.ts

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

const yak = "C:\\Program Files\\Rhino 7\\System\\Yak.exe";
const version = process.argv[2] || "5.1.0-beta";

execSync(`"${yak}" unyank semio ${version}`, { stdio: "inherit" });

console.log(`✅ Unyanked semio ${version}`);

#!/usr/bin/env tsx
// #region 🔖Header

// 💻︎ semio/jsonschema/build.ts

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

import { readFileSync, writeFileSync } from "fs";
import { join } from "path";

const inputFilePath = join(__dirname, "kit.json");
const outputFilePath = join(__dirname, "kit_unescaped.json");

const jsonContent = readFileSync(inputFilePath, "utf-8");
const unescapedContent = jsonContent.replace(/\\(.)/g, "$1");
writeFileSync(outputFilePath, unescapedContent, "utf-8");

console.log(`✅ Unescaped ${inputFilePath} to ${outputFilePath}`);

#!/usr/bin/env tsx
// #region 🔖Header

// [👤semio🛂jsonschema📜buildts](semiorepo://file/SEMIO/JSONSCHEMA/BUILD.TS)

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

// Build script for generating and exporting JSON Schema definitions.

// #endregion 🔖Header

// #region 🔖Schema Export

// [🔖semio/jsonschema/build.ts#Schema Export](semiorepo://section/semio/jsonschema/build.ts/SCHEMA-EXPORT)
// JSON Schema export script. MUST unescape and write the kit schema file.

import { readFileSync, writeFileSync } from "fs";
import { join } from "path";

// Input JSON Schema file path.
// MUST point to the kit.json schema file.
const inputFilePath = join(__dirname, "kit.json");
// Output file path for the unescaped JSON Schema.
// MUST be written next to the input file.
const outputFilePath = join(__dirname, "kit_unescaped.json");

// Raw JSON content read from the input schema file.
// MUST be read as UTF-8.
const jsonContent = readFileSync(inputFilePath, "utf-8");
// Unescaped JSON content with backslash sequences resolved.
// MUST replace all escaped characters.
const unescapedContent = jsonContent.replace(/\\(.)/g, "$1");
writeFileSync(outputFilePath, unescapedContent, "utf-8");

console.log(`✅ Unescaped ${inputFilePath} to ${outputFilePath}`);

// #endregion 🔖Schema Export

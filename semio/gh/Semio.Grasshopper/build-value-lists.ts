#!/usr/bin/env tsx
// #region 🔖Header
// [👤semio📚gh🛅semiograsshopper📜buildvaluelists](repo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/build-value-lists.ts)

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

// Generates Grasshopper value list presets from domain data.

// #endregion 🔖Header

// #region 🔖Value List Generation
// [👤semio📚gh🛅semiograsshopper💻buildvaluelists🔖valuelistgeneration](repo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/build-value-lists.ts/s/Value%20List%20Generation)
// Value list generation script. MUST convert CSV data into Grasshopper value list text files.

import { parse } from "csv-parse/sync";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "fs";
import { join } from "path";

/**
 * Build output directory for generated value list files.
// [👤semio📚gh🛅semiograsshopper💻buildvaluelists🔖valuelistgeneration🪨builddir](repo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/build-value-lists.ts/s/Value%20List%20Generation/d/i/buildDir)
 * MUST be created if it does not exist.
 **/
const buildDir = join(__dirname, "build");
if (!existsSync(buildDir)) {
  mkdirSync(buildDir);
}

/**
 * Converts a CSV file into a Grasshopper value list text format.
// [👤semio📚gh🛅semiograsshopper💻buildvaluelists🔖valuelistgeneration🛠️convertcsvtovaluelist](repo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/build-value-lists.ts/s/Value%20List%20Generation/d/i/convertCsvToValueList)
 * MUST read the CSV, extract key-value pairs, and write the output file.
 **/
function convertCsvToValueList(csvPath: string, outputPath: string, keyColumn: string, valueColumn: string): void {
  const csvContent = readFileSync(csvPath, "utf-8");
  const records = parse(csvContent, { columns: true, skip_empty_lines: true });

  const lines = records.map((record: any) => {
    return `${record[keyColumn]} = "${record[valueColumn]}"`;
  });

  writeFileSync(outputPath, lines.join("\n"), "utf-8");
}

convertCsvToValueList(join(__dirname, "..", "..", "meta", "mimes.csv"), join(buildDir, "mimes.txt"), "Extension", "MIME");

convertCsvToValueList(join(__dirname, "..", "..", "meta", "licenses.csv"), join(buildDir, "licenses.txt"), "Name", "SPDX");

console.log("✅ Value lists generated");

// #endregion 🔖Value List Generation

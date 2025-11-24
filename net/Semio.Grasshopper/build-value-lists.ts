#!/usr/bin/env tsx
import { readFileSync, writeFileSync, mkdirSync, existsSync } from "fs";
import { join } from "path";
import { parse } from "csv-parse/sync";

const buildDir = join(__dirname, "build");
if (!existsSync(buildDir)) {
  mkdirSync(buildDir);
}

function convertCsvToValueList(
  csvPath: string,
  outputPath: string,
  keyColumn: string,
  valueColumn: string
): void {
  const csvContent = readFileSync(csvPath, "utf-8");
  const records = parse(csvContent, { columns: true, skip_empty_lines: true });
  
  const lines = records.map((record: any) => {
    return `${record[keyColumn]} = "${record[valueColumn]}"`;
  });
  
  writeFileSync(outputPath, lines.join("\n"), "utf-8");
}

convertCsvToValueList(
  join(__dirname, "..", "..", "meta", "mimes.csv"),
  join(buildDir, "mimes.txt"),
  "Extension",
  "MIME"
);

convertCsvToValueList(
  join(__dirname, "..", "..", "meta", "licenses.csv"),
  join(buildDir, "licenses.txt"),
  "Name",
  "SPDX"
);

console.log("✅ Value lists generated");

import { diagnose } from "./check.ts";
import { readFileSync } from "node:fs";
import { parseFile } from "./lib.ts";
import { relative } from "node:path";
const [listFile, baseFile] = process.argv.slice(2);
const files = readFileSync(listFile, "utf8").split("\n").filter(Boolean);
const baseFiles = baseFile.split(",");
const base = diagnose(baseFiles, new Set());
const ign = new Set(base.map((d) => d.msg));
// extra globals typical of no-lib
const names = new Set<string>();
for (const bf of baseFiles) for (const u of parseFile(bf).units) u.names.forEach((n) => names.add(n));
const res = diagnose(files, ign).filter((d) => d.code !== 2304 || (d.name && names.has(d.name)));
const counts = new Map<string, number>();
for (const d of res) {
  const key = `TS${d.code} ${d.msg}`;
  counts.set(key, (counts.get(key) ?? 0) + 1);
}
const shown = new Set<string>();
for (const d of res) {
  const key = `TS${d.code} ${d.msg}`;
  if ((counts.get(key) ?? 0) > 12 && shown.has(key)) continue;
  shown.add(key);
  console.log(`${relative("/Users/ueli/Documents/semio", d.file)}:${d.line} ${key}`);
}
console.log(`${res.length} diagnostics across ${files.length} files`);

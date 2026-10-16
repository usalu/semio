/** 🔁️ Replaces exact repository path strings in text files and re-sorts the surrounding JSON string-array block when it was sorted before.
 * Usage: bun repath.ts <map.json> <file>...   (map.json: {"old/path": "new/path"}; every old path must occur at least once in some file)
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const [mapPath, ...files] = process.argv.slice(2);
const map = JSON.parse(readFileSync(mapPath!, "utf8")) as Record<string, string>;
const seen = new Map<string, number>();
const itemLine = /^(\s*)"((?:[^"\\]|\\.)*)"(,?)\s*$/;
for (const file of files) {
  const abs = file.startsWith("/") ? file : join(ROOT, file);
  const text = readFileSync(abs, "utf8");
  const lines = text.split("\n");
  const touched = new Set<number>();
  for (let i = 0; i < lines.length; i++) {
    let line = lines[i]!;
    let changed = false;
    for (const [from, to] of Object.entries(map)) {
      if (line.includes(from)) { line = line.split(from).join(to); changed = true; seen.set(from, (seen.get(from) ?? 0) + 1); }
    }
    if (changed) { lines[i] = line; touched.add(i); }
  }
  const original = text.split("\n");
  const handled = new Set<number>();
  for (const i of [...touched].sort((a, b) => a - b)) {
    if (handled.has(i) || !itemLine.test(lines[i]!)) continue;
    let s = i, e = i;
    while (s > 0 && itemLine.test(original[s - 1]!)) s--;
    while (e + 1 < original.length && itemLine.test(original[e + 1]!)) e++;
    const items = (a: string[]) => a.map(l => itemLine.exec(l)![2]!);
    const before = items(original.slice(s, e + 1));
    const sortedBefore = before.every((v, k) => k === 0 || before[k - 1]! <= v);
    for (let k = s; k <= e; k++) handled.add(k);
    if (!sortedBefore) continue;
    const indent = itemLine.exec(original[s]!)![1]!;
    const sorted = items(lines.slice(s, e + 1)).sort((a, b) => (a < b ? -1 : a > b ? 1 : 0));
    const trailing = itemLine.exec(original[e]!)![3]!;
    for (let k = 0; k < sorted.length; k++) lines[s + k] = `${indent}"${sorted[k]}"${k === sorted.length - 1 ? trailing : ","}`;
  }
  const out = lines.join("\n");
  if (out !== text) { writeFileSync(abs, out); console.log(`edited ${file.slice(-70)} (${touched.size} lines)`); }
}
for (const from of Object.keys(map)) if (!seen.has(from)) { console.error(`NOT FOUND ${from}`); process.exitCode = 1; }

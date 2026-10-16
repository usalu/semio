/** 🆚 Compares two tsc logs ignoring line and column numbers and prints errors only in the second log (new) and only in the first (fixed).
 * Usage: bun 📜️script.ts <before.log> <after.log>
 */
import { readFileSync } from "node:fs";

const normalize = (path: string): Map<string, number> => {
  const counts = new Map<string, number>();
  const lines = readFileSync(path, "utf8").split("\n");
  for (let i = 0; i < lines.length; i += 1) {
    const m = /^(.*?)\(\d+,\d+\): (error TS\d+: .*)$/.exec(lines[i]!);
    if (!m) continue;
    const key = `${m[1]} | ${m[2]}`;
    counts.set(key, (counts.get(key) ?? 0) + 1);
  }
  return counts;
};
const [beforePath, afterPath] = process.argv.slice(2) as [string, string];
const before = normalize(beforePath);
const after = normalize(afterPath);
let added = 0;
let fixed = 0;
for (const [key, n] of after) {
  const delta = n - (before.get(key) ?? 0);
  if (delta > 0) {
    added += delta;
    console.log(`NEW   x${delta} ${key}`);
  }
}
for (const [key, n] of before) {
  const delta = n - (after.get(key) ?? 0);
  if (delta > 0) {
    fixed += delta;
    console.log(`FIXED x${delta} ${key}`);
  }
}
console.log(`before=${[...before.values()].reduce((a, b) => a + b, 0)} after=${[...after.values()].reduce((a, b) => a + b, 0)} new=${added} fixed=${fixed}`);

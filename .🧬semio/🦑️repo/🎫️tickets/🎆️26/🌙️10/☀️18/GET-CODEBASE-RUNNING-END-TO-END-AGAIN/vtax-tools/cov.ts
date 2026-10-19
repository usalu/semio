// merges lcov files (max hit count per line) and reports line coverage of the decomposed validator modules and of the pre-split original. usage: bun cov.ts <lcov>... [--uncovered]
import { readFileSync } from "node:fs";
const files = process.argv.slice(2).filter((a) => !a.startsWith("--")), showUncovered = process.argv.includes("--uncovered");
const merged = new Map<string, Map<number, number>>();
for (const file of files) {
  let current = "";
  for (const line of readFileSync(file, "utf8").split("\n")) {
    if (line.startsWith("SF:")) { current = line.slice(3).replace(/^(\.\.\/)+/, "/"); if (!merged.has(current)) merged.set(current, new Map()); }
    else if (line.startsWith("DA:")) { const [n, hits] = line.slice(3).split(",").map(Number) as [number, number]; const map = merged.get(current)!; map.set(n, Math.max(map.get(n) ?? 0, hits)); }
  }
}
const rows: { name: string; covered: number; total: number; uncovered: number[] }[] = [];
for (const [path, lines] of merged) {
  if (!/audit|taxonomy-authority|orig\/taxonomy-authority\.abs/.test(path)) continue;
  const name = path.includes("/orig/") ? "ORIGINAL(pre-split)" : path.split("/").slice(-2, -1)[0] ?? path;
  const total = lines.size, covered = [...lines.values()].filter((h) => h > 0).length;
  rows.push({ name, covered, total, uncovered: [...lines].filter(([, h]) => h === 0).map(([n]) => n).sort((a, b) => a - b) });
}
rows.sort((a, b) => a.name.localeCompare(b.name));
for (const r of rows) console.log(r.name.padEnd(48), `${r.covered}/${r.total}`.padStart(9), `${((100 * r.covered) / Math.max(1, r.total)).toFixed(1)}%`.padStart(7), showUncovered && r.uncovered.length ? "uncovered: " + r.uncovered.join(",") : "");
const sum = rows.filter((r) => r.name !== "ORIGINAL(pre-split)" && r.name !== "🔐️taxonomy-authority");
console.log("decomposed validator modules total", sum.reduce((a, r) => a + r.covered, 0) + "/" + sum.reduce((a, r) => a + r.total, 0));

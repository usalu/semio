/** 🔬️ hr2 tsc comparer: multisets of (file, code, message) ignoring positions; the after side is mapped back with --map new=old pairs. Usage: bun tscdiff.ts <before.txt> <after.txt> [--map new=old ...] */
import { readFileSync } from "node:fs";
const args = process.argv.slice(2);
const [a, b] = args;
const maps = args.slice(2).filter((x) => x.includes("=")).map((x) => x.split("=") as [string, string]);
const back = (s: string) => maps.reduce((acc, [n, o]) => acc.split(n).join(o), s);
const parse = (p: string, map: boolean) => {
  const m = new Map<string, number>();
  for (const line of readFileSync(p, "utf8").split("\n")) {
    const x = line.match(/^(.*?)\(\d+,\d+\): error (TS\d+): (.*)$/u);
    if (!x) continue;
    const text = map ? back(`${x[1]} ${x[2]} ${x[3]}`) : `${x[1]} ${x[2]} ${x[3]}`;
    m.set(text, (m.get(text) ?? 0) + 1);
  }
  return m;
};
const A = parse(a!, false), B = parse(b!, true);
let onlyA = 0, onlyB = 0;
const lines: string[] = [];
for (const [k, n] of A) { const d = n - (B.get(k) ?? 0); if (d > 0) { onlyA += d; lines.push(`- (${d}) ${k.slice(0, 300)}`); } }
for (const [k, n] of B) { const d = n - (A.get(k) ?? 0); if (d > 0) { onlyB += d; lines.push(`+ (${d}) ${k.slice(0, 300)}`); } }
console.log(`before=${[...A.values()].reduce((x, y) => x + y, 0)} after=${[...B.values()].reduce((x, y) => x + y, 0)} removed=${onlyA} added=${onlyB}`);
for (const l of lines) console.log(l);

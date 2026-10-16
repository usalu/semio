/** 🔬️ Compares two tsc outputs as multisets of (file, code, message) ignoring positions. Usage: bun tscdiff.ts <before.txt> <after.txt> */
import { readFileSync } from "node:fs";
const [a, b] = process.argv.slice(2);
const parse = (p: string) => {
  const m = new Map<string, number>();
  for (const line of readFileSync(p, "utf8").split("\n")) {
    const x = line.match(/^(.*?)\(\d+,\d+\): error (TS\d+): (.*)$/);
    if (!x) continue;
    const msg = x[3].replace(/(?:'(?:\.\.\/)+[^']*'|"(?:\.\.\/)+[^"]*")/g, "<relpath>").replace(/@semio-tech\/framework(?:\/[\w\/.-]+)?/g, "<sdk>");
    const key = `${x[1]} ${x[2]} ${msg}`;
    m.set(key, (m.get(key) ?? 0) + 1);
  }
  return m;
};
const A = parse(a), B = parse(b);
let onlyA = 0, onlyB = 0;
const lines: string[] = [];
for (const [k, n] of A) { const d = n - (B.get(k) ?? 0); if (d > 0) { onlyA += d; lines.push(`- (${d}) ${k.slice(0, 300)}`); } }
for (const [k, n] of B) { const d = n - (A.get(k) ?? 0); if (d > 0) { onlyB += d; lines.push(`+ (${d}) ${k.slice(0, 300)}`); } }
console.log(`before=${[...A.values()].reduce((x, y) => x + y, 0)} after=${[...B.values()].reduce((x, y) => x + y, 0)} removed=${onlyA} added=${onlyB}`);
for (const l of lines) console.log(l);

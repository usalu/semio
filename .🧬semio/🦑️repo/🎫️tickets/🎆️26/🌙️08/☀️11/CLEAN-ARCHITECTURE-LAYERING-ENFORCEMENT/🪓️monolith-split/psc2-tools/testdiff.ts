/** 🧪️ Compares two runtests outputs: per-file pass/fail counts and failing test names. Usage: bun testdiff.ts <before.json> <after.json> */
import { readFileSync } from "node:fs";
const [a, b] = process.argv.slice(2).map(p => JSON.parse(readFileSync(p, "utf8")) as Record<string, any>);
let same = 0, diff = 0, bp = 0, bf = 0, ap = 0, af = 0;
for (const k of Object.keys(a)) {
  const x = a[k], y = b[k];
  if (!y) { console.log("MISSING after", k.slice(-80)); diff++; continue; }
  bp += x.pass; bf += x.fail; ap += y.pass; af += y.fail;
  const setA = [...x.failed].sort().join("|"), setB = [...y.failed].sort().join("|");
  if (x.pass === y.pass && x.fail === y.fail && setA === setB && x.exit === y.exit) same++;
  else { diff++; console.log(`DIFF before ${x.pass}/${x.fail} (exit ${x.exit}) after ${y.pass}/${y.fail} (exit ${y.exit})`, k.slice(-90)); }
}
console.log({ files: Object.keys(a).length, same, diff, before: { pass: bp, fail: bf }, after: { pass: ap, fail: af } });

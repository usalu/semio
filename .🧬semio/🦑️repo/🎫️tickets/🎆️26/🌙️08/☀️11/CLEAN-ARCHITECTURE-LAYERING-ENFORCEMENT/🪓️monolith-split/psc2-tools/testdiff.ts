/** 🧪️ Compares two runtests outputs on the files present in both: per-file pass/fail counts and failing test names. Usage: bun testdiff.ts <before.json> <after.json> */
import { readFileSync } from "node:fs";
const [a, b] = process.argv.slice(2).map(p => JSON.parse(readFileSync(p, "utf8")) as Record<string, any>);
let same = 0, diff = 0, skipped = 0, bp = 0, bf = 0, ap = 0, af = 0, newFail = 0, fewer = 0;
for (const k of Object.keys(a)) {
  const x = a[k], y = b[k];
  if (!y) { skipped++; continue; }
  bp += x.pass; bf += x.fail; ap += y.pass; af += y.fail;
  const setA = new Set<string>(x.failed), setB = new Set<string>(y.failed);
  const added = [...setB].filter(n => !setA.has(n));
  if (x.pass === y.pass && x.fail === y.fail && x.exit === y.exit && added.length === 0 && setA.size === setB.size) same++;
  else { diff++; if (added.length || y.fail > x.fail || (y.pass + y.fail === 0 && x.pass + x.fail > 0)) newFail++; else fewer++; console.log(`DIFF before ${x.pass}/${x.fail} (exit ${x.exit}) after ${y.pass}/${y.fail} (exit ${y.exit})${added.length ? " NEW-FAIL " + added.join(" | ").slice(0, 200) : ""}`, k.slice(-100)); }
}
console.log({ compared: same + diff, same, diff, worse: newFail, better: fewer, notRunAfter: skipped, before: { pass: bp, fail: bf }, after: { pass: ap, fail: af } });

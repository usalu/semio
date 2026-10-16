/** 🧪️ Compares two vitest JSON reports: per-file totals and failing test names. Usage: node vitest-diff.cjs <before.json> <after.json> */
const fs = require("fs");
const [a, b] = process.argv.slice(2).map((p) => JSON.parse(fs.readFileSync(p, "utf8")));
const norm = (name) => name.replace("/Users/ueli/Documents/semio/", "");
const index = (r) => { const m = new Map(); for (const s of r.testResults) m.set(norm(s.name), s); return m; };
const A = index(a), B = index(b);
console.log("before", a.numTotalTests, a.numPassedTests, a.numFailedTests, "after", b.numTotalTests, b.numPassedTests, b.numFailedTests);
const names = (s) => new Set(s.assertionResults.filter((x) => x.status === "failed").map((x) => x.fullName));
for (const k of new Set([...A.keys(), ...B.keys()])) {
  const x = A.get(k), y = B.get(k);
  if (!x || !y) { console.log("ONLY", x ? "before" : "after", k.slice(-90)); continue; }
  const fx = names(x), fy = names(y);
  const added = [...fy].filter((n) => !fx.has(n)), removed = [...fx].filter((n) => !fy.has(n));
  const same = x.assertionResults.length === y.assertionResults.length && !added.length && !removed.length;
  console.log(same ? "SAME" : "DIFF", x.assertionResults.length, "->", y.assertionResults.length, "newFail", added.length, "fixed", removed.length, k.slice(-80));
}

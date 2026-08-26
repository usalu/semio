import { readFileSync } from "node:fs";
import { join } from "node:path";
import { discoverTestCases, parseFeature } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
const root = "/Users/ueli/Documents/semio";
const tally = new Map<string, { n: number; cases: Set<string> }>();
for (const c of discoverTestCases(root)) {
  const f = parseFeature(readFileSync(join(root, c.featurePath), "utf8"));
  if (!f.oracle) continue;
  for (const s of f.scenarios) {
    if (s.mode === "differential") continue;
    const thens = s.steps.filter((t) => /then|and/i.test(t.keyword) === true).map((t) => t.text.replace(/\s+/g, " "));
    // parseFeature keeps keyword; take everything after the first Then
    const idx = s.steps.findIndex((t) => t.keyword.trim().toLowerCase() === "then");
    const asserts = (idx >= 0 ? s.steps.slice(idx) : s.steps).map((t) => t.text.replace(/\s+/g, " "));
    const key = `${s.mode} ⇒ ${asserts.join(" ∧ ")}`;
    if (!tally.has(key)) tally.set(key, { n: 0, cases: new Set() });
    const g = tally.get(key)!; g.n += 1; g.cases.add(c.case);
  }
}
for (const [k, g] of [...tally].sort((a, b) => b[1].n - a[1].n)) console.log(`${String(g.n).padStart(4)}  ${String(g.cases.size).padStart(3)}c  ${k}`);

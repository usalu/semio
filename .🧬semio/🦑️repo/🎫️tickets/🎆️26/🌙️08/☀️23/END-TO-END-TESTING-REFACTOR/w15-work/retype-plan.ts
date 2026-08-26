import { readFileSync } from "node:fs";
import { join } from "node:path";
import { discoverTestCases, parseFeature } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
const root = "/Users/ueli/Documents/semio";
const CROSS = /(the oracle and the subject agree|both reproduce|both languages read|the reference implementation and this repository agree|matches the oracle's own)/i;
const IN_ROLE = /(asserted in role|independently read)/i;
const rows: any[] = [];
for (const c of discoverTestCases(root)) {
  const f = parseFeature(readFileSync(join(root, c.featurePath), "utf8"));
  if (!f.oracle) continue;
  if (/ifc|step-ap214/.test(c.case)) continue;
  for (const s of f.scenarios) {
    if (s.mode === "differential") continue;
    const idx = s.steps.findIndex((t) => t.keyword.trim().toLowerCase() === "then");
    const asserts = (idx >= 0 ? s.steps.slice(idx) : s.steps).map((t) => t.text.replace(/\s+/g, " "));
    if (asserts.some((a) => IN_ROLE.test(a))) continue;
    const onlyCross = asserts.length > 0 && asserts.every((a) => CROSS.test(a));
    if (!onlyCross) continue;
    rows.push({ case: c.case, path: c.featurePath, scenario: s.id, mode: s.mode, asserts: asserts.join(" ∧ ") });
  }
}
const byCase = new Map<string, { n: number; modes: Set<string>; path: string; texts: Set<string> }>();
for (const r of rows) { if (!byCase.has(r.case)) byCase.set(r.case, { n: 0, modes: new Set(), path: r.path, texts: new Set() }); const g = byCase.get(r.case)!; g.n += 1; g.modes.add(r.mode); g.texts.add(r.asserts); }
console.log("scenarios whose ONLY assertion is cross-producer agreement:", rows.length, "in", byCase.size, "cases");
for (const [k, g] of [...byCase].sort((a, b) => b[1].n - a[1].n)) console.log(`  ${String(g.n).padStart(3)}  ${[...g.modes].join(",")}  ${k}\n        ${[...g.texts].join("\n        ")}`);

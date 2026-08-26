import { readFileSync } from "node:fs";
import { join } from "node:path";
import { discoverTestCases, parseFeature } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
const root = "/Users/ueli/Documents/semio";
const CROSS = /(the oracle and the subject agree|both reproduce|both languages read|the two implementations agree|matches the oracle's own|and agree with (Pillow|three\.js))/i;
const IN_ROLE = /(asserted in role|independently read)/i;
const out: any[] = [];
for (const c of discoverTestCases(root)) {
  const f = parseFeature(readFileSync(join(root, c.featurePath), "utf8"));
  if (!f.oracle) continue;
  const ifcStep = /ifc|step-ap214/.test(c.case);
  for (const s of f.scenarios) {
    if (s.mode === "differential") continue;
    const idx = s.steps.findIndex((t) => t.keyword.trim().toLowerCase() === "then");
    const asserts = (idx >= 0 ? s.steps.slice(idx) : s.steps).map((t) => t.text.replace(/\s+/g, " ")).join(" ∧ ");
    const cross = CROSS.test(asserts), inRole = IN_ROLE.test(asserts);
    out.push({ case: c.case, owner: c.ownerName, featurePath: c.featurePath, scenario: s.id, mode: s.mode, cross, inRole, ifcStep, asserts });
  }
}
const retype = out.filter((r) => r.cross && !r.inRole && !r.ifcStep);
const byCase = new Map<string, { n: number; modes: Set<string>; path: string }>();
for (const r of retype) { if (!byCase.has(r.case)) byCase.set(r.case, { n: 0, modes: new Set(), path: r.featurePath }); const g = byCase.get(r.case)!; g.n += 1; g.modes.add(r.mode); }
console.log("non-differential oracle scenarios:", out.length);
console.log("  in ifc/step cases:", out.filter((r) => r.ifcStep).length);
console.log("  cross-producer assertion (retype):", retype.length, "in", byCase.size, "cases");
console.log("  in-role / independent-reader:", out.filter((r) => r.inRole && !r.ifcStep).length);
console.log("  neither (law asserted without naming roles):", out.filter((r) => !r.cross && !r.inRole && !r.ifcStep).length);
for (const [k, g] of [...byCase].sort((a, b) => b[1].n - a[1].n)) console.log(`  ${String(g.n).padStart(3)}  ${[...g.modes].join(",")}  ${k}`);
console.log("--- NEITHER, grouped:");
const neither = new Map<string, { n: number; cases: Set<string> }>();
for (const r of out.filter((r) => !r.cross && !r.inRole && !r.ifcStep)) { const k = `${r.mode} ⇒ ${r.asserts}`; if (!neither.has(k)) neither.set(k, { n: 0, cases: new Set() }); const g = neither.get(k)!; g.n += 1; g.cases.add(r.case); }
for (const [k, g] of [...neither].sort((a, b) => b[1].n - a[1].n)) console.log(`  ${String(g.n).padStart(3)}  ${g.cases.size}c  ${k}  [${[...g.cases].slice(0, 4).join(", ")}]`);

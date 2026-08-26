import { readFileSync } from "node:fs";
import { join } from "node:path";
import { discoverTestCases, parseFeature } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
const root = "/Users/ueli/Documents/semio";
const CROSS = /(the oracle and the subject agree|the reference implementation and this repository agree)/i;
const IN_ROLE = /(asserted in role|independently read)/i;
const plan: any[] = [];
for (const c of discoverTestCases(root)) {
  const src = readFileSync(join(root, c.featurePath), "utf8");
  const f = parseFeature(src);
  if (!f.oracle) continue;
  if (/ifc|step-ap214/.test(c.case)) continue;
  // group scenarios by their @id- tag (one outline block per id)
  const byId = new Map<string, any[]>();
  for (const s of f.scenarios) {
    const id = (s.tags ?? []).find((t: string) => t.startsWith("id-")) ?? s.id;
    if (!byId.has(id)) byId.set(id, []);
    byId.get(id)!.push(s);
  }
  for (const [id, group] of byId) {
    if (!group.every((s) => s.mode === "property")) continue;
    const ok = group.every((s) => {
      const idx = s.steps.findIndex((t: any) => t.keyword.trim().toLowerCase() === "then");
      const asserts = (idx >= 0 ? s.steps.slice(idx) : s.steps).map((t: any) => t.text.replace(/\s+/g, " "));
      return asserts.length > 0 && asserts.every((a: string) => CROSS.test(a) && !IN_ROLE.test(a));
    });
    if (ok) plan.push({ case: c.case, path: c.featurePath, id, scenarios: group.length });
  }
}
console.log(JSON.stringify(plan, null, 1));
console.log("blocks:", plan.length, "scenarios:", plan.reduce((n, p) => n + p.scenarios, 0), "cases:", new Set(plan.map((p) => p.case)).size);

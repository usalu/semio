import { readFileSync } from "node:fs";
import { join } from "node:path";
import { discoverTestCases, parseFeature } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
const root = "/Users/ueli/Documents/semio";
const groups = new Map<string, { cases: Set<string>; scenarios: number; modes: Set<string> }>();
for (const c of discoverTestCases(root)) {
  const f = parseFeature(readFileSync(join(root, c.featurePath), "utf8"));
  if (!f.oracle) continue;
  for (const s of f.scenarios) {
    if (s.mode === "differential") continue;
    const key = `${s.mode} ‖ ${s.steps.map((t) => t.keyword.trim() + " " + t.text.replace(/\s+/g, " ").slice(0, 120)).join(" · ")}`;
    if (!groups.has(key)) groups.set(key, { cases: new Set(), scenarios: 0, modes: new Set() });
    const g = groups.get(key)!;
    g.cases.add(c.case); g.scenarios += 1; g.modes.add(s.mode);
  }
}
const rows = [...groups.entries()].sort((a, b) => b[1].scenarios - a[1].scenarios);
for (const [key, g] of rows) console.log(`${String(g.scenarios).padStart(4)} scen  ${String(g.cases.size).padStart(3)} cases  ${key}`);
console.log("groups:", rows.length, "total:", rows.reduce((n, r) => n + r[1].scenarios, 0));

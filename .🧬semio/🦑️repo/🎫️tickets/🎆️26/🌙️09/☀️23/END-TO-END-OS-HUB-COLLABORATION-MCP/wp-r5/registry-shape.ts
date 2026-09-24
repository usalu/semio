import { loadOracleRegistry, repoRootFromHere } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const r = loadOracleRegistry(root);
const bad = r.oracles.filter((o: any) => !Array.isArray(o.comparisonProfiles) || !Array.isArray(o.capabilities) || !o.license || o.testOnly !== true);
console.log("oracles", r.oracles.length, "bad", bad.length);
for (const o of bad) console.log(" ", (o as any).id, Object.keys(o).join(","), (o as any).source ?? "");
const badD = r.noOracleDecisions.filter((d: any) => typeof d.rationale !== "string");
console.log("decisions", r.noOracleDecisions.length, "bad", badD.length);
for (const d of badD) console.log(" ", JSON.stringify(d).slice(0, 300));
for (const c of r.contributions) {
  for (const o of c.oracles) if (!Array.isArray((o as any).comparisonProfiles)) console.log("  from", (c as any).path ?? (c as any).owner ?? Object.keys(c));
  for (const d of c.noOracleDecisions) if (typeof (d as any).rationale !== "string") console.log("  decision from", (c as any).path ?? (c as any).owner);
}

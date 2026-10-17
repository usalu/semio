// 🚧️ c4p30: runs verifyAppliedLayout for one artifact with a large sample limit and writes every unique only-combined diagnostic. usage: bun gate-dump.ts <artifact dir> <sandbox root> <out.json>
import { writeFileSync } from "node:fs";
import { verifyAppliedLayout } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧫️sandbox/🟦️.ts";
const [artifact, sandbox, out] = process.argv.slice(2) as [string, string, string];
const report = await verifyAppliedLayout("/Users/ueli/Documents/semio", artifact, sandbox, "f856dff0813", false, 1000);
writeFileSync(out, JSON.stringify(report, null, 1));
console.log(JSON.stringify({ layout: Object.fromEntries(Object.entries(report.layout).map(([k, v]) => [k, { base: v.base, combined: v.combined, onlyCombined: v.onlyCombined }])) }));

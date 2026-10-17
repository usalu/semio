import { verifyAppliedLayout } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧫️sandbox/🟦️.ts";
const [repoRoot, artifactDir, sandbox] = process.argv.slice(2) as [string, string, string];
const report = await verifyAppliedLayout(repoRoot, artifactDir, sandbox, "HEAD", false);
console.log("ABGATE " + JSON.stringify({ crates: report.crates, layout: Object.fromEntries(Object.entries(report.layout).map(([k, v]) => [k, { base: v.base, combined: v.combined, onlyCombined: v.onlyCombined, sample: v.sampleOnlyCombined }])) }));

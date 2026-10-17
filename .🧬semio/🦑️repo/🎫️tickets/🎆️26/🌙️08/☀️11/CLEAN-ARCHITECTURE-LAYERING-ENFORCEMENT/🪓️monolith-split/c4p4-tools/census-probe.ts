/** 🧮️ c4p4 tool-job census probe: runs the gate's tool-job functions over the plugin production sources twice (the raster editor with its pre-split text, and the live split tree) and prints both digests. */
import { readFileSync } from "node:fs";
import { toolJobProofs, toolJobStaticRows, toolJobDispositions, toolJobAppOwnedRows, toolJobScanThenMonolithRows, toolJobProductionProtocolSource } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
import { policyModuleTreeFiles, policyReadFileSafe } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts";
import { toolJobProofCatalogFailures, toolJobPluginProductionSources } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const RASTER = "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs";
const ORIG = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p4-tools/orig/raster-editor.rs.txt";
const live = toolJobPluginProductionSources(root);
const base = new Map(live);
base.set(RASTER, toolJobProductionProtocolSource(readFileSync(ORIG, "utf8")));
const CHILDREN = ["🧰️scene-helpers", "📬️store-preparation", "🎞️reserved-import", "🔌️app-assembly"].map((name) => RASTER.replace(/🦀️\.rs$/u, `${name}/🦀️.rs`));
for (const file of policyModuleTreeFiles(root, RASTER).slice(1)) {
  if (CHILDREN.includes(file) || file.includes("/🧪️tests/") || !file.endsWith(".rs")) continue;
  base.set(file, toolJobProductionProtocolSource(policyReadFileSafe(root, file)));
}
const digest = (sources: Map<string, string>) => {
  const rows = toolJobStaticRows(sources);
  const dispositions = toolJobDispositions(sources);
  const proofs = toolJobProofs(sources);
  const appOwned = toolJobAppOwnedRows(sources);
  const failures = toolJobProofCatalogFailures(sources, rows, dispositions, proofs);
  const here = (file: string) => file.includes("🖨️raster");
  return {
    files: sources.size,
    rasterPresent: sources.has(RASTER),
    rows: rows.filter((row) => here(row.file)).map((row) => `${row.file.slice(-40)}|${row.id}|${row.source}`).sort(),
    dispositions: [...dispositions].filter(([key]) => here(key)).map(([key, value]) => `${key.slice(-60)}=${value}`).sort(),
    proofs: proofs.filter((proof) => here(proof.sourceFile) || here(proof.ownerFile)).map((proof) => `${proof.sourceFile === proof.ownerFile ? "same" : "DIFF"}|${proof.ownerTypeName}|${proof.factory}|${proof.factoryType ?? ""}|${proof.toolId}|${proof.execution}|${proof.values.join(",")}`).sort(),
    appOwned: appOwned.filter((row) => here(row.ownerFile)).map((row) => `${row.factoryFile === row.ownerFile ? "same" : "DIFF"}|${row.factory}|${row.toolId}|${row.publicationLanes.join("+")}|${row.publicationReady}`).sort(),
    scanThenMonolith: toolJobScanThenMonolithRows(sources, proofs).filter((row) => here(row.file)).map((row) => `${row.id}|${row.helper}`),
    failuresAll: failures.length,
    failuresRaster: failures.filter((failure) => /raster/i.test(failure)).map((failure) => failure.replaceAll(root, "")),
  };
};
const a = digest(base), b = digest(live);
console.log(JSON.stringify({ base: a, live: b, identical: JSON.stringify(a) === JSON.stringify(b) }, null, 1));

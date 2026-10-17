/** 🧮️ c4p4 tool-job census probe: runs the gate's tool-job functions over the plugin production sources twice (the raster editor with its pre-split text, and the live split tree) and prints both digests. */
import { readFileSync } from "node:fs";
import { toolJobProofs, toolJobStaticRows, toolJobDispositions, toolJobAppOwnedRows, toolJobScanThenMonolithRows, toolJobProductionProtocolSource } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
import { policyModuleTreeFiles, policyReadFileSafe } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts";
import { toolJobProofCatalogFailures, toolJobPluginProductionSources } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p12-tools/orig/";
const ROOTS: Record<string, string> = {
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs": F + "3.rs",
  "✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🦀️.rs": F + "7.rs",
  "✏️s/🔌️plugins/📕️norm/⚖️compliance/🦀️.rs": F + "6.rs",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs": F + "5.rs",
};
const live = toolJobPluginProductionSources(root);
const base = new Map(live);
const childrenOf = (rootFile: string) => policyModuleTreeFiles(root, rootFile).slice(1).filter((file) => !file.includes("/🧪️tests/") && file.endsWith(".rs"));
for (const [rootFile, orig] of Object.entries(ROOTS)) {
  base.set(rootFile, toolJobProductionProtocolSource(readFileSync(orig, "utf8")));
}
const digest = (sources: Map<string, string>) => {
  const rows = toolJobStaticRows(sources);
  const dispositions = toolJobDispositions(sources);
  const proofs = toolJobProofs(sources);
  const appOwned = toolJobAppOwnedRows(sources);
  const failures = toolJobProofCatalogFailures(sources, rows, dispositions, proofs);
  const here = (file: string) => file.includes("📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor") || file.includes("📕️norm/📇️registry/🧬️contract/🖥️app-surface") || file.includes("📕️norm/⚖️compliance") || file.includes("📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences");
  return {
    files: sources.size,
    rootsPresent: Object.keys(ROOTS).filter((file) => sources.has(file)),
    rows: rows.filter((row) => here(row.file)).map((row) => `${row.file.slice(-40)}|${row.id}|${row.source}`).sort(),
    dispositions: [...dispositions].filter(([key]) => here(key)).map(([key, value]) => `${key.slice(-60)}=${value}`).sort(),
    proofs: proofs.filter((proof) => here(proof.sourceFile) || here(proof.ownerFile)).map((proof) => `${proof.sourceFile === proof.ownerFile ? "same" : "DIFF"}|${proof.ownerTypeName}|${proof.factory}`).sort(),
    appOwned: appOwned.filter((row) => here(row.ownerFile)).map((row) => `${row.factoryFile === row.ownerFile ? "same" : "DIFF"}|${row.factory}|${row.toolId}|${row.publicationLanes.join("+")}`).sort(),
    scanThenMonolith: toolJobScanThenMonolithRows(sources, proofs).filter((row) => here(row.file)).map((row) => `${row.id}|${row.helper}`),
    failuresAll: failures.length,
    failuresHere: failures.filter((failure) => /cad|norm/i.test(failure)).map((failure) => failure.replaceAll(root, "")),
  };
};
const a = digest(base), b = digest(live);
console.log(JSON.stringify({ base: a, live: b, identical: JSON.stringify(a) === JSON.stringify(b) }, null, 1));

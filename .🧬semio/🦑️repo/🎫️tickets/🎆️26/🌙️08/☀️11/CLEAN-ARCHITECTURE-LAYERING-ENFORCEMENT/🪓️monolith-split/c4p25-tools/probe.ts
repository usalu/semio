// 🔬️ c4p25 reader probe: runs the source-text readers that name the energy editor split files and prints deterministic JSON, so a run before and a run after the split can be compared byte for byte.
// usage: bun probe.ts > <out.json>
import {toolJobFactoryProofActivationScan, toolJobPluginProductionSources, toolJobProofCatalogFailures} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
import {toolJobAppOwnedRows, toolJobDispositions, toolJobProofs, toolJobStaticRows} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
import {completionRejectionOracle} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️artifact-admission-and-completion-oracles/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const out: Record<string, unknown> = {};
const key = "🔋️energy/🗿️artifacts/🔋️model";
const files = toolJobPluginProductionSources(root);
const proofs = toolJobProofs(files);
const rows = toolJobStaticRows(files);
const dispositions = toolJobDispositions(files);
const failures = toolJobProofCatalogFailures(files, rows, dispositions, proofs);
out.toolJob = {
  sources: files.size,
  proofs: proofs.length,
  proofsEnergy: proofs.filter(p => p.ownerFile.includes(key)).map(p => [p.ownerFile.slice(-40), p.ownerTypeName, p.controllerId, p.factory, p.factoryType ?? null, p.toolId, p.sourceFile === p.ownerFile]),
  rowsEnergy: rows.filter(r => r.file.includes(key)).length,
  appOwnedEnergy: toolJobAppOwnedRows(files).filter(r => r.ownerFile.includes(key)).map(r => [r.toolId, r.publicationReady]),
  failuresTotal: failures.length,
  failuresEnergy: failures.filter(f => f.includes("🔋️energy") || /energy/i.test(f)).sort(),
  failuresHash: Bun.hash(failures.slice().sort().join("\n")).toString(),
  scan: toolJobFactoryProofActivationScan(root),
};
try { out.completion = completionRejectionOracle(root); } catch (error) { out.completion = "FAIL " + String((error as Error).message).slice(0, 300); }
console.log(JSON.stringify(out, null, 1));

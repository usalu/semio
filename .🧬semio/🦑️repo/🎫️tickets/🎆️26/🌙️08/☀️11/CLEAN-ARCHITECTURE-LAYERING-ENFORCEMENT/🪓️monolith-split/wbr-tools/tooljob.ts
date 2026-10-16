import { toolJobPluginProductionSources, toolJobProofCatalogFailures } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
import { toolJobDispositions, toolJobProofs, toolJobStaticRows } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const sources = toolJobPluginProductionSources(root);
const proofs = toolJobProofs(sources);
const failures = toolJobProofCatalogFailures(sources, toolJobStaticRows(sources), toolJobDispositions(sources), proofs);
const energy = (text: string) => text.includes("energy");
console.log(JSON.stringify({ sources: sources.size, proofs: proofs.length, energyProofs: proofs.filter((p) => energy(p.ownerFile)).map((p) => p.ownerFile.split("/").slice(-3).join("/") + "|" + p.toolId), failures: failures.length, energyFailures: failures.filter(energy) }, null, 1));

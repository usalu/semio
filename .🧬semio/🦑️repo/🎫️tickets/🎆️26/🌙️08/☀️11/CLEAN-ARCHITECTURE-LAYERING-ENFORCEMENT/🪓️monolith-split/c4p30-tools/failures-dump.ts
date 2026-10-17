// 🔬 c4p30: prints every tool-job proof catalog failure, one per line. usage: bun failures-dump.ts
import { toolJobPluginProductionSources, toolJobProofCatalogFailures } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
import { toolJobStaticRows, toolJobDispositions, toolJobProofs } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
const sources = toolJobPluginProductionSources("/Users/ueli/Documents/semio");
const failures = toolJobProofCatalogFailures(sources, toolJobStaticRows(sources), toolJobDispositions(sources), toolJobProofs(sources));
console.log(failures.slice().sort().join("\n"));

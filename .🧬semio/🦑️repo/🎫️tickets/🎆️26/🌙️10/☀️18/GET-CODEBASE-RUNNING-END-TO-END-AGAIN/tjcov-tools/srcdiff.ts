import { toolJobPluginProductionSources } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
import { toolJobProofs } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
const files = toolJobPluginProductionSources("/Users/ueli/Documents/semio");
const proofs = toolJobProofs(files);
const bad = new Map<string,string>(); for (const p of proofs) if (p.sourceFile!==p.ownerFile) bad.set(p.sourceFile, p.ownerFile);
for (const [s,o] of bad) console.log(s,"\n  =>",o, "\n  ownerInMap", files.has(o), "len", files.get(o)?.length);

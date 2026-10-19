import { toolJobOwnerSourceText } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
import { toolJobProductionProtocolSource, toolJobProductionSource } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
const R = "/Users/ueli/Documents/semio";
const f = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs";
const raw = toolJobOwnerSourceText(R, f);
const prod = toolJobProductionSource(raw);
const proto = toolJobProductionProtocolSource(raw);
for (const [n, t] of [["raw", raw], ["production", prod], ["protocol", proto]] as const) console.log(n, t.length, t.includes("impl ArtifactEditor for Fem2dPlayApp"), t.includes("impl semio_framework_plugin::ArtifactEditor for Fem2dPlayApp"), (t.match(/bounded_first_step_tool_proofs!/g) ?? []).length, t.includes("app_commands!"));
import { toolJobProofs, toolJobStaticRows, toolJobDispositions } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
import { toolJobProofCatalogFailures } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
const files = new Map([[f, proto]]);
const proofs = toolJobProofs(files);
console.log("proofs", proofs.length, proofs[0]?.sourceFile === proofs[0]?.ownerFile, proofs[0]?.ownerFile.slice(-40));
const failures = toolJobProofCatalogFailures(files, toolJobStaticRows(files), toolJobDispositions(files), proofs);
const cat = new Map<string, number>(); for (const x of failures) { const k = x.split(" ").slice(0, 5).join(" "); cat.set(k, (cat.get(k) ?? 0) + 1); }
console.log([...cat]);

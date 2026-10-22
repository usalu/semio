// 🧪️ Prints the authority observations a farm worker holds after opening its session, the same way the parent harness prints its own (AUTH).
import { join } from "node:path";
import { writeFileSync } from "node:fs";
const W = "/Users/ueli/Documents/semio", LIB = join(W, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const service = await import(join(LIB, "🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/👷️service/🟨️.mjs"));
const observation = await import(join(LIB, "🏗️build/👁️observation/🟨️.mjs"));
await service.open({ workspaceRoot: W, registry: null, commandInputs: null, projectsByRoot: [], componentRoots: [], stagingBase: "/private/tmp/gpp2/auth-staging" }, { slot: 0, generation: 1 });
const cap = await observation.captureObservedSourceInputsV1(async () => 0, undefined, true);
writeFileSync(process.env.AUTH, JSON.stringify(cap.authorityObservations.map(r => [r.kind, r.path.replace(W, "<WS>")]), null, 0).replaceAll("],[", "],\n["));
console.log("IDENTITY", observation.inferenceRecordIdentityV1(), observation.inferenceAuthorityDigestV1(), cap.authorityObservations.length);
process.exit(0);

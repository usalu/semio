import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { createReadStream, readFileSync, realpathSync, statSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const root = process.cwd();
const subsystem = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const loadOwner = (owner: string) => import(pathToFileURL(join(root, subsystem, owner)).href);
const { componentStageTargetV1, settleComponentDescriptorV1 } = await loadOwner("🖨️describe/🧩️component-stage/🟦️.ts");
const { readComponentReceiptV1 } = await loadOwner("🖨️describe/🏷️component-receipt/🟦️.ts");
const { verifySemanticDescriptorPairBytesV1 } = await loadOwner("📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts");

assert(process.argv[2], "Supply the repository-published GIS component model directory");
const directory = realpathSync(process.argv[2]);
const manifest = "🌎️hub/🧩️compositions/🌍️gis/🧩️component/📦️packages/🦀️rust/Cargo.toml";
const target = componentStageTargetV1(root, manifest);
const marker = JSON.parse(readFileSync(join(directory, ".semio-artifact.json"), "utf8"));
assert.equal(marker.owner, manifest + ":component-model");
const receipt = readComponentReceiptV1(join(directory, "🧾️receipt.json"));
assert.equal(receipt.pluginId, target.pluginId);
assert.equal(receipt.packageId, target.packageId);
const wasm = join(directory, "📦️component.wasm");
assert.equal(statSync(wasm).size, receipt.wasmBytes);
const hash = createHash("sha256");
for await (const bytes of createReadStream(wasm)) hash.update(bytes);
assert.equal(hash.digest("hex"), receipt.wasmSha256);

const jsonBytes = readFileSync(join(directory, "🔣️.json"));
const packBytes = readFileSync(join(directory, "🛂️.descriptor.semio"));
const pair = verifySemanticDescriptorPairBytesV1(jsonBytes, packBytes);
assert.equal(pair.pluginId, receipt.pluginId);
assert.equal(pair.packageId, receipt.packageId);
assert.equal(pair.role, target.role);
assert.equal(pair.semanticSha256, receipt.semanticDescriptorSha256);
const drift = settleComponentDescriptorV1(root, target, { profile: receipt.profile, directory }, { jsonBytes, packBytes }, "write");
console.log(JSON.stringify({ owner: target.ownerRoot, sha256: pair.semanticSha256, differences: drift.length }));

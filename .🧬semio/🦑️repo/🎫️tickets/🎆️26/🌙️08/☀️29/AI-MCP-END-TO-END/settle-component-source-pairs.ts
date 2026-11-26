import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { componentStageTargetV1, settleComponentDescriptorV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧩️component-stage/🟦️.ts";
import { verifySemanticDescriptorPairBytesV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts";
import { actorPreparationImageIdentityV1, actorPreparationPolicyV1, assertActorPreparationBindingV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/🌱️actor-preparation/🟦️.ts";
import { fileDigest, readArtifactPrefixV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts";

const [owner, modelPath, previous, emitted] = process.argv.slice(2);
const catalog = JSON.parse(readFileSync("✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🦀️catalog.json","utf8"));
const registration = catalog.plugins.find((row: { pluginId: string }) => row.pluginId===owner);
assert(registration);
const root = process.cwd();
const target = componentStageTargetV1(root,join(registration.cratePath,"Cargo.toml"));
const directory = resolve(modelPath);
const readPair = (directory: string) => ({
  packBytes: readFileSync(join(directory,"🛂️.descriptor.semio")),
  jsonBytes: readFileSync(join(directory,"🔣️.json")),
});
const fresh = readPair(directory);
const pair = verifySemanticDescriptorPairBytesV1(fresh.jsonBytes,fresh.packBytes);
assert.equal(pair.pluginId,owner);
assert.equal(pair.semanticSha256,emitted);
const signal = AbortSignal.timeout(60_000);
const image = join(directory,actorPreparationPolicyV1.image);
const actor = await actorPreparationImageIdentityV1(image,signal);
assertActorPreparationBindingV1(await readArtifactPrefixV1(image,64,actorPreparationPolicyV1.maximumBytes,signal),await fileDigest(join(directory,"📦️component.wasm"),signal));
const current = readPair(target.ownerRoot);
const existing = verifySemanticDescriptorPairBytesV1(current.jsonBytes,current.packBytes).semanticSha256;
assert(existing===previous || existing===emitted,"Source descriptor changed after the comparison");
const drift = settleComponentDescriptorV1(root,target,{profile:"prototype",directory},fresh,"write");
const written = readPair(target.ownerRoot);
assert(written.packBytes.equals(fresh.packBytes) && written.jsonBytes.equals(fresh.jsonBytes));
console.log(JSON.stringify({owner,previous:existing,current:emitted,actor:actor.imageSha256,changedPaths:drift.length}));

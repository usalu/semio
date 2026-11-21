import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { canonicalDescriptorDiffV1, componentStageTargetV1, settleComponentDescriptorV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧩️component-stage/🟦️.ts";
import { componentDeliverableV1, SHIPPED_COMPONENT_PROFILE } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🟦️.ts";
import { verifySemanticDescriptorPairBytesV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts";

const args = process.argv.slice(2);
const wfc = args[0] === "--wfc";
if (wfc) args.shift();
assert(args.length === 0 || (args.length === 1 && args[0] === "--write"));
const repoRoot = process.cwd();
const target = componentStageTargetV1(repoRoot, `🌎️hub/🧩️compositions/${wfc ? "🀄️wfc" : "🗒️note"}/🧩️component/📦️packages/🦀️rust/Cargo.toml`);
const staged = resolve(import.meta.dirname, wfc ? "🗑️generated/m-action/browser/prototype/components/f0caf2db1142c1ba93f50ac6" : "🗑️generated/m-prep/browser/prototype/components/6d8a4f7ae2584b94ed7f1643");
const json = "🔣️.json";
const pack = "🛂️.descriptor.semio";
const jsonBytes = readFileSync(join(staged, json));
const packBytes = readFileSync(join(staged, pack));
const verified = verifySemanticDescriptorPairBytesV1(jsonBytes, packBytes);
const originalJson = readFileSync(join(target.ownerRoot, json));
const originalPack = readFileSync(join(target.ownerRoot, pack));
const recorded = verifySemanticDescriptorPairBytesV1(originalJson, originalPack);
const componentSha256 = wfc ? "83b51df3b34f6cd21c3e6a45f9e63874c5571e6af6e5d746b348be68fd42cff8" : "76ad7393aebbe39291097cfa25528dd4fa0041f424b4a25eee7f5f638cd22796";
const oldSemantic = wfc ? "e87360531b895c739aea4c8e458e5e11f527daabd43d5b9085c9d242afd3d5cf" : "0bfd7c7fe6c3c8fb842950d629621d918d6f715cdfc88ba8577e8ce96982995f";
const newSemantic = wfc ? "94cde94f083e500f76c6a29d46f8aa6a62eee2109b8190ae1c692e8888edae9e" : "d82dd839a4594c7fe9a85dbf440d89161dcc777cc458d362f2c5e03b08990a42";
assert.deepEqual([verified.pluginId, verified.packageId, verified.role], [target.pluginId, target.packageId, target.role]);
const componentHash = new Bun.CryptoHasher("sha256");
for await (const bytes of Bun.file(join(staged, "📦️component.wasm")).stream()) componentHash.update(bytes);
assert.equal(componentHash.digest("hex"), componentSha256);
assert.equal(verified.semanticSha256, newSemantic);
assert([oldSemantic, newSemantic].includes(recorded.semanticSha256), "Committed descriptor changed independently");
const current = JSON.parse(originalJson.toString());
const emitted = JSON.parse(jsonBytes.toString());
const changes = canonicalDescriptorDiffV1(current, emitted);
if (recorded.semanticSha256 === oldSemantic) {
  const windows = wfc
    ? [[4, 0, 10, "wfc-graph"], [4, 1, 2, "wfc-2d-preview"], [6, 0, 23, "wfc-grid3d-grid"], [6, 1, 2, "wfc-grid3d-preview"], [8, 0, 10, "wfc-graph"], [8, 1, 2, "wfc-3d-preview"]] as const
    : [[0, 0, 1, "note-composite"], [0, 1, 1, "note-navigator"]] as const;
  const expectedPaths = windows.flatMap(([app, window, action]) =>
    ["[2].required", "[3]", "[4]"].map(suffix => `.manifest.apps[${app}].windowKinds[${window}].actions[${action}].args${suffix}`));
  assert.deepEqual(changes.map(row => row.split(": ", 1)[0]), expectedPaths);
  for (const [app, index, action, expected] of windows) {
    const window = emitted.manifest.apps[app].windowKinds[index];
    assert.equal(window.id, expected);
    assert.equal(window.actions[action].id, "interactionHover");
    assert.deepEqual(window.actions[action].args.map((arg: { id: string; required: boolean }) => [arg.id, arg.required]), [
      ["domainId", true], ["channel", true], ["targets", false], ["targetId", false], ["granularityId", false],
    ]);
  }
}
console.log("[DEBUG]", target.pluginId, "semantic descriptor", recorded.semanticSha256, "→", verified.semanticSha256, changes);
if (args[0] === "--write") {
  assert(readFileSync(join(target.ownerRoot, json)).equals(originalJson));
  assert(readFileSync(join(target.ownerRoot, pack)).equals(originalPack));
  const deliverable = componentDeliverableV1(target.crateRoot, target.packageName, SHIPPED_COMPONENT_PROFILE);
  const emission = { ...verified, componentSha256, jsonBytes, packBytes };
  settleComponentDescriptorV1(repoRoot, target, deliverable, emission, "write");
  assert.deepEqual(settleComponentDescriptorV1(repoRoot, target, deliverable, emission, "verify"), []);
  assert(readFileSync(join(target.ownerRoot, json)).equals(jsonBytes));
  assert(readFileSync(join(target.ownerRoot, pack)).equals(packBytes));
  console.log("[DEBUG] Published and reverified exact JSON and Pack", target.pluginId);
}

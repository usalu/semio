import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { createReadStream, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";

const root = process.cwd();
const [out, wasm, mode] = process.argv.slice(2);
assert.ok(out && wasm && (mode === "verify" || mode === "write"));

const plugin = resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
const { componentStageTargetV1, settleComponentDescriptorV1 } = await import(join(plugin, "🖨️describe/🧩️component-stage/🟦️.ts"));
const { componentDeliverableV1 } = await import(join(plugin, "🖨️describe/🏷️component-receipt/🟦️.ts"));
const { verifySemanticDescriptorPairBytesV1 } = await import(join(plugin, "📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts"));
const contract = JSON.parse(readFileSync(join(plugin, "📇️registry/🛂️descriptor-verification/🧬️schema/🔣️.json"), "utf8"));
const owner = resolve(root, "🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/💼️office/🧩️component");
const target = componentStageTargetV1(root, join(owner, "📦️packages/🦀️rust/Cargo.toml"));
const packageName = target.packageName;
const jsonBytes = readFileSync(join(out, "🔣️.json"));
const packBytes = readFileSync(join(out, "🛂️.descriptor.semio"));
const pair = verifySemanticDescriptorPairBytesV1(jsonBytes, packBytes);
assert.deepEqual([pair.pluginId, pair.packageId, pair.role], [target.pluginId, target.packageId, target.role]);

const descriptor = JSON.parse(jsonBytes.toString("utf8"));
const validate = new Ajv({ strict: false, allErrors: true }).compile(contract);
assert.ok(validate(descriptor), JSON.stringify(validate.errors));
for (const [kind, standard, subsets] of [
  ["s.stdio.xlsx", "ecma-376", ["*", "strict", "transitional"]],
  ["s.stdio.idml", "8.0", ["*"]],
] as const) {
  const format = `${kind}@${standard}`;
  const apps = descriptor.manifest.apps.filter((app: { dialect: { artifactKind: string } }) => app.dialect?.artifactKind === kind);
  const roles = ["editor", "viewer"];
  assert.deepEqual(apps.map((app: { id: string; role: string }) => [app.id, app.role]), subsets.flatMap(subset =>
    roles.map(role => [`${format}/${subset}#${role}`, role])));
  assert.ok(descriptor.manifest.hostedArtifactKinds.some((entry: { id: string }) => entry.id === kind));
  assert.ok(descriptor.activationEvents.some((event: { onArtifactKind?: { kind: string } }) => event.onArtifactKind?.kind === kind));
  assert.ok(descriptor.contributions.fileTypes.some((file: { formatKind: string; imports: boolean; exports: boolean }) =>
    file.formatKind === format && file.imports && file.exports));
}

const componentHash = createHash("sha256");
for await (const chunk of createReadStream(wasm)) componentHash.update(chunk);
const componentSha256 = componentHash.digest("hex");
if (mode === "write") {
  const deliverable = componentDeliverableV1(target.crateRoot, packageName, "prototype");
  settleComponentDescriptorV1(root, target, deliverable, { ...pair, jsonBytes, packBytes, componentSha256 }, "write");
  assert.deepEqual(readFileSync(join(owner, "🔣️.json")), jsonBytes);
  assert.deepEqual(readFileSync(join(owner, "🛂️.descriptor.semio")), packBytes);
}
console.log(`Verified semantic descriptor ${pair.semanticSha256} for Office component ${componentSha256} with XLSX and IDML import, export and app contributions`);

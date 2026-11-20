import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";

const verifier = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts");
const { decodeDescriptorPairBytesV1, verifySemanticDescriptorPairBytesV1 } = await import(verifier);
const [jsonPath, packPath] = process.argv.slice(2);
const sourcePath = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🔗️linked-asset/🗣️.dsl.semio";
const [json, pack, source] = await Promise.all([
  readFile(jsonPath),
  readFile(packPath),
  readFile(sourcePath, "utf8"),
]);
const verified = verifySemanticDescriptorPairBytesV1(json, pack);
assert.equal(verified.packageId, "semio:layout");
assert.equal(verified.pluginId, "layout");
const { record } = decodeDescriptorPairBytesV1(json, pack);
const examples = record.manifest.examples;
assert.ok(examples.some((example: { id: string }) => example.id === "demo"));
const linked = examples.filter((example: { id: string }) => example.id === "linked-asset");
assert.equal(linked.length, 1, "the compiled component must publish the Linked Asset example");
assert.equal(linked[0].artifactJson, source);
console.log(`Verified canonical Layout JSON and Pack with Linked Asset example (${verified.semanticSha256})`);

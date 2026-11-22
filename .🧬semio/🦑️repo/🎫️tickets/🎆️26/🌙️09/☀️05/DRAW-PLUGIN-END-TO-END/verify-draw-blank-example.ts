/** 🧾️ Publishes and checks the authored Blank Drawing browseable example in the descriptor pair. */
import assert from "node:assert/strict";
import { readFileSync, writeFileSync } from "node:fs";
import { verifySemanticDescriptorPairBytesV1 } from "@semio-tech/plugin-registry/catalog-verification/descriptor-pair";
import { encodePackValue, type PackValue } from "../../../../../../../🧰️framework/🔨️modules/🎁️packs/🟦️.ts";
import Ajv from "ajv";

const app = "✏️s/🔌️plugins/🖍️draw";
const browser = app + "/🧪️tests/🌐️browser/";
const fixture = JSON.parse(readFileSync(browser + "🔣️.json", "utf8"));
const schema = JSON.parse(readFileSync(browser + "📐️.json", "utf8"));
assert(new Ajv({ strict: true }).compile(schema)(fixture));
const asset = "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/📄️blank/🗣️.dsl.semio";
const artifactJson = readFileSync(asset, "utf8");
const component = "🌎️hub/🧩️compositions/🖍️draw/🧩️component/";
const jsonPath = component + "🔣️.json";
const packPath = component + "🛂️.descriptor.semio";
const json = readFileSync(jsonPath);
const pack = readFileSync(packPath);
const before = verifySemanticDescriptorPairBytesV1(json, pack);
const record = JSON.parse(json.toString());
const [demo, existing] = record.manifest.examples;
assert.equal(demo.id, fixture.example.id);
const labels = fixture.blankExample.labels;
const blank = {
  id: fixture.blankExample.id,
  label: { native: labels, reuse: labels },
  iconId: demo.iconId,
  artifactJson,
  dialect: demo.dialect,
};
if (existing) {
  assert.deepEqual(record.manifest.examples, [demo, blank]);
  console.log(JSON.stringify({ semantic: before.semanticSha256, authoredBytes: Buffer.byteLength(artifactJson), examples: [demo.id, blank.id] }));
} else {
  const raw = json.toString();
  const start = raw.indexOf('"examples": [\n');
  const end = raw.indexOf('\n    ],', start);
  assert(start >= 0 && end > start);
  const appended = JSON.stringify(blank, null, 2).split("\n").map(line => "      " + line).join("\n");
  const changedJson = Buffer.from(raw.slice(0, end) + ",\n" + appended + raw.slice(end));
  const manifest = before.descriptor.manifest as { examples: PackValue[] };
  assert.equal(manifest.examples.length, 1);
  const changedPack = encodePackValue({ ...before.descriptor, manifest: { ...manifest, examples: [...manifest.examples, blank] } } as PackValue);
  const after = verifySemanticDescriptorPairBytesV1(changedJson, changedPack);
  assert.deepEqual(JSON.parse(changedJson.toString()).manifest.examples, [demo, blank]);
  assert(readFileSync(jsonPath).equals(json));
  assert(readFileSync(packPath).equals(pack));
  assert.equal(readFileSync(asset, "utf8"), artifactJson);
  writeFileSync(jsonPath, changedJson);
  writeFileSync(packPath, changedPack);
  console.log(JSON.stringify({ semantic: after.semanticSha256, authoredBytes: Buffer.byteLength(artifactJson), examples: [demo.id, blank.id] }));
}

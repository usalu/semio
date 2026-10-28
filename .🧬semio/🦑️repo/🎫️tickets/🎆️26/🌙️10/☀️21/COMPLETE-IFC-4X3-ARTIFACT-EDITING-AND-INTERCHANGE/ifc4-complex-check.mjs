import fs from "node:fs/promises";
import path from "node:path";
import { createRequire } from "node:module";
import assert from "node:assert/strict";

export async function run(root) {
  const base = path.join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any");
  const read = async relative => JSON.parse(await fs.readFile(path.join(base, relative), "utf8"));
  const require = createRequire(path.join(root, "package.json"));
  const Ajv = require("ajv");
  const validator = new Ajv({ strict: false, allErrors: true });
  const snapshot = await read("🧬️schema/📸️snapshot/🔣️.json");
  validator.addSchema(snapshot);
  const artifact = await read("🧬️schema/🔣️.json");
  const mutations = await read("🧬️schema/🧬️mutations/🔣️.json");
  const mutationRoot = path.join(base, "🧬️schema/🧬️mutations");
  for (const entry of await fs.readdir(mutationRoot, { withFileTypes: true })) {
    if (!entry.isDirectory()) continue;
    try { validator.addSchema(JSON.parse(await fs.readFile(path.join(mutationRoot, entry.name, "🧬️schema/🔣️.json"), "utf8"))); }
    catch (error) { if (error.code !== "ENOENT") throw error; }
  }
  const acceptsSnapshot = validator.compile(snapshot);
  const acceptsArtifact = validator.compile(artifact);
  const acceptsMutation = validator.compile(mutations);
  const fixture = "🧪️tests/🧫️fixtures/🧬️mutations/🧩️set-entity-complex/🔁️replaces/";
  const before = await read(fixture + "📸️snapshot/⬅️before/🔣️.json");
  const after = await read(fixture + "📸️snapshot/➡️after/🔣️.json");
  const mutation = await read(fixture + "🦠️mutation/🔣️.json");
  const diff = await read(fixture + "🔺️diff/🔣️.json");
  assert(acceptsSnapshot(before), JSON.stringify(acceptsSnapshot.errors));
  assert(acceptsSnapshot(after), JSON.stringify(acceptsSnapshot.errors));
  assert(acceptsArtifact(after), JSON.stringify(acceptsArtifact.errors));
  assert(acceptsMutation(mutation), JSON.stringify(acceptsMutation.errors));
  const expected = JSON.parse(JSON.stringify(before));
  expected.entities.find(entity => entity.id === mutation.id).complex = mutation.complex;
  assert.deepEqual(expected, after);
  assert.deepEqual(diff, { entities: { modified: [{ id: mutation.id, diff: { complex: mutation.complex } }] } });
  for (const invalid of [
    { kind: "integer", value: "3" },
    { kind: "reference", value: -1 },
    { kind: "typedValue", value: ["IFCLENGTHMEASURE", []] },
    { kind: "aggregate", value: [{ kind: "real" }] },
    { kind: "unset", value: 0 },
    { kind: "unknown" },
  ]) {
    const candidate = JSON.parse(JSON.stringify(after));
    candidate.entities[0].complex[0].args = [invalid];
    assert.equal(acceptsSnapshot(candidate), false, JSON.stringify(invalid));
    assert.equal(acceptsMutation({ ...mutation, complex: candidate.entities[0].complex }), false, JSON.stringify(invalid));
  }
  assert.equal(acceptsArtifact({ schema: "stdio.ifc", text: "legacy" }), false);
  assert.equal(acceptsMutation({ mutation: "setEntityComplex", complex: [] }), false);
  assert(acceptsMutation({ mutation: "setEntityComplex", id: 1, complex: [] }));
  const descriptor = await read("🧬️schema/🧬️mutations/🧩️set-entity-complex/🔣️.json");
  const protocol = await fs.readFile(path.join(base, "🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"), "utf8");
  assert(protocol.includes("record set-entity-complex tag=" + descriptor.binaryTag));
  const catalog = await read("🔮️oracles/🔣️.json");
  const kinds = catalog.mutationCatalogs.find(entry => entry.id === "ifc-4-any").kinds;
  assert.equal(kinds.filter(kind => kind === "set-entity-complex").length, 1);
  const typeScript = await fs.readFile(path.join(base, "🧬️schema/🧬️mutations/🟦️.ts"), "utf8");
  assert(typeScript.includes('mutation: "setEntityComplex"'));
  return { validator: "AJV " + require("ajv/package.json").version, validFixtures: 4, rejectedMalformedValues: 6, mutationKinds: kinds.length, result: "passed" };
}

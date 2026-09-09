import valueSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json" with { type: "json" };
/** 🧪️ Canonical Wires document contracts agree with independent schema validation. */
import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import ioSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import childSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json" with { type: "json" };
import artifactSchema from "../../🔣️.json" with { type: "json" };
import snapshotSchema from "../../📸️snapshot/🔣️.json" with { type: "json" };
import diffSchema from "../../🔺️diff/🔣️.json" with { type: "json" };
import mutationSchema from "../../🧬️mutations/🔣️.json" with { type: "json" };
import snapshot from "../../🧬️mutations/🧭move-node/🧪️tests/📓️reports-a-no-op-when-ba77ae/📸️snapshot/⬅️before/🔣️.json" with { type: "json" };
import diff from "../../🧬️mutations/🧭move-node/🧪️tests/📓️reports-a-no-op-when-ba77ae/🔺️diff/🔣️.json" with { type: "json" };
import { parseWiresArtifact } from "../../🟦️.ts";
import { parseWiresSnapshot } from "../../📸️snapshot/🟦️.ts";

export function testWiresDocumentContractOracle(): void {
  const ajv = new Ajv({ strict: false, allErrors: true });
  ajv.addSchema(valueSchema).addSchema(ioSchema).addSchema(childSchema).addSchema(artifactSchema);
  for (const [schema, parse] of [[artifactSchema, parseWiresArtifact], [snapshotSchema, parseWiresSnapshot]] as const) {
    const validate = ajv.compile(schema);
    assert.equal(validate(snapshot), true, JSON.stringify(validate.errors));
    assert.deepEqual(parse(snapshot), snapshot);
    assert.equal(validate({ ...snapshot, boardFixture: {} }), false);
    assert.throws(() => parse({ ...snapshot, boardFixture: {} }));
  }
  const validateDiff = ajv.compile(diffSchema);
  assert.equal(validateDiff(diff), true, JSON.stringify(validateDiff.errors));
  assert.deepEqual(Object.keys(diffSchema.properties).sort(), Object.keys(diff).sort());
  const mutations = join(import.meta.dir, "../../🧬️mutations");
  for (const directory of readdirSync(mutations, { withFileTypes: true }).filter((entry) => entry.isDirectory())) {
    const path = join(mutations, directory.name, "🧬️schema/🔣️.json");
    if (directory.name === "🧪️tests") continue;
    const payload = JSON.parse(readFileSync(path, "utf8"));
    ajv.addSchema(payload);
    const source = ts.createSourceFile("payload.ts", readFileSync(join(mutations, directory.name, "🧬️schema/🟦️.ts"), "utf8"), ts.ScriptTarget.Latest, true);
    const declaration = source.statements.find((node) => ts.isInterfaceDeclaration(node) && node.name.text === payload.title) as ts.InterfaceDeclaration;
    assert(declaration, `${directory.name}: missing leaf-owned TypeScript payload`);
    assert.deepEqual(declaration.members.map((member) => member.name!.getText(source)).sort(), Object.keys(payload.properties).sort());
  }
  const validateMutation = ajv.compile(mutationSchema);
  const fixtures = readdirSync(mutations, { recursive: true }).map((path) => String(path).replaceAll("\\", "/")).filter((path) => path.endsWith("/🦠️mutation/🔣️.json"));
  for (const path of fixtures) {
    const mutation = JSON.parse(readFileSync(join(mutations, path), "utf8"));
    assert.equal(validateMutation(mutation), true, `${path}: ${JSON.stringify(validateMutation.errors)}`);
    assert.equal(validateMutation({ ...mutation, locale: "de" }), false);
  }
  assert.equal(fixtures.length > 0, true);
  console.log("[DEBUG] Wires document contract matched canonical native artifact/snapshot/diff fixtures and rejected editor-era boardFixture fields");
  console.log(`[DEBUG] Wires aggregate mutation schema validated ${fixtures.length} committed wire inputs through their leaf-owned payload definitions`);
}

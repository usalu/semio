import assert from "node:assert/strict";
import Ajv from "ajv";
import documentAdmission from "../../../🧪️tests/🧫️fixtures/🧬️document-admission/🔣️.json" with { type: "json" };
import { applyPatch, getValueByPointer } from "fast-json-patch";
import fem2dDocumentSchema from "../../../🧬️schema/🔣️.json" with { type: "json" };
import { parseFem2dArtifact } from "../../../🧬️schema/🟦️.ts";
import { testFem2dModelWindowConfigContract } from "../../🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts";
import { testFem2dResultsWindowConfigContract } from "../../🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts";

export function testFem2dWindowConfigContract(): void {
  testFemDocumentAdmission();
  testFem2dModelWindowConfigContract();
  testFem2dResultsWindowConfigContract();
}

function testFemDocumentAdmission(): void {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addKeyword("x-semio-state").addKeyword("x-semio-invariant").addKeyword("x-semio-ui");
  ajv.addFormat("double", { type: "number", validate: Number.isFinite });
  ajv.addFormat("uint32", { type: "number", validate: (value: number) => Number.isInteger(value) && value >= 0 && value <= 0xffffffff });
  const schema = fem2dDocumentSchema;
  const parse = parseFem2dArtifact;
  const validate = ajv.compile(schema);
  const row = documentAdmission;
  assert(validate(row.document), JSON.stringify(validate.errors));
  const native = structuredClone(row.document) as unknown as Record<string, unknown>;
  for (const { pointer, word } of row.nativeWords) {
    assert.equal(Buffer.from(word, "hex").readDoubleBE(), getValueByPointer(row.document, pointer), pointer);
    applyPatch(native, [{ op: "replace", path: pointer, value: { bits: BigInt(`0x${word}`) } }], true, true);
  }
  for (const { pointer, value } of row.nativeCounts) {
    assert.equal(BigInt(value), BigInt(getValueByPointer(row.document, pointer)), pointer);
    applyPatch(native, [{ op: "replace", path: pointer, value: BigInt(value) }], true, true);
  }
  assert.deepEqual(parse(native), native);
  console.log(`[DEBUG] FEM2d document admission: ${row.nativeWords.length} binary64 oracle words, ${row.nativeCounts.length} integer counts`);
  for (const field of row.foreignFields) {
    const candidate = { ...structuredClone(row.document), [field.key]: field.value };
    assert(!validate(candidate), field.key);
    assert.throws(() => parse({ ...structuredClone(native), [field.key]: field.value }), field.key);
  }
}

if (import.meta.main) testFem2dWindowConfigContract();

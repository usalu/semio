import assert from "node:assert/strict";
import Ajv from "ajv";
import documentAdmission from "../../../🧫️fixtures/🧬️document-admission/🔣️.json" with { type: "json" };
import fem2dDocumentSchema from "../../../🧬️schema/🔣️.json" with { type: "json" };
import { parseFem2dArtifact } from "../../../🧬️schema/🟦️.ts";
import { testFem2dModelWindowConfigContract } from "../../🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🧪️tests/🪪️document/🟦️.ts";
import { testFem2dResultsWindowConfigContract } from "../../🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document/🟦️.ts";

export function testFem2dWindowConfigContract(): void {
  testFemDocumentAdmission();
  testFem2dModelWindowConfigContract();
  testFem2dResultsWindowConfigContract();
}

function testFemDocumentAdmission(): void {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addKeyword("x-semio-state").addKeyword("x-semio-invariant");
  ajv.addFormat("double", { type: "number", validate: Number.isFinite });
  ajv.addFormat("uint32", { type: "number", validate: (value: number) => Number.isInteger(value) && value >= 0 && value <= 0xffffffff });
  const schema = fem2dDocumentSchema;
  const parse = parseFem2dArtifact;
  const validate = ajv.compile(schema);
  const row = documentAdmission;
  assert(validate(row.document), JSON.stringify(validate.errors));
  assert.deepEqual(parse(row.document), row.document);
  for (const field of row.foreignFields) {
    const candidate = { ...structuredClone(row.document), [field.key]: field.value };
    assert(!validate(candidate), field.key);
    assert.throws(() => parse(candidate), field.key);
  }
}

if (import.meta.main) testFem2dWindowConfigContract();

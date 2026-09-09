import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { applyPatch, type Operation } from "fast-json-patch";
import { applyWriterMainWindowTransientMutation, type WriterMainWindowTransientMutation } from "../../🧬️schema/🧬️mutations/🟦️.ts";

/** 🧵️ Checks the native construction vectors against schema and independent JSON Patch. */
export function testWriterPartialConstructionOracle(): void {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🧩️partial-construction/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const mutationSchema = JSON.parse(readFileSync(new URL("../../🧬️schema/🧬️mutations/🔣️.json", import.meta.url), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addKeyword("x-semio-state");
  ajv.addKeyword("x-semio-owner");
  const validateState = ajv.compile(schema);
  const validateMutation = ajv.compile(mutationSchema);
  const input = fixture.payload.text.repeat(fixture.payload.repeat);
  assert.equal(new TextEncoder().encode(input).length, fixture.payload.utf8Bytes);
  const before = { ...fixture.base, engagementInput: input };
  assert(validateState(before), JSON.stringify(validateState.errors));
  for (const row of fixture.cases) {
    assert(validateMutation(row.mutation), JSON.stringify(validateMutation.errors));
    const actual = applyWriterMainWindowTransientMutation(before, row.mutation as WriterMainWindowTransientMutation);
    const operations = Object.entries(row.patch).map(([key, value]) => ({ op: "replace", path: `/${key}`, value })) as Operation[];
    const expected = applyPatch(before, operations, true, false).newDocument;
    assert(validateState(actual), JSON.stringify(validateState.errors));
    assert.deepEqual(actual, expected);
    assert.equal(before.engagementInput, input);
  }
  console.log("[DEBUG] Writer partial construction: three 12KiB UTF-8 vectors match TypeScript, Ajv and JSON Patch");
}

import assert from "node:assert/strict";
import Ajv from "ajv";
import { applyPatch, type Operation } from "fast-json-patch";
import schema from "../../🔣️.json" with { type: "json" };
import fixture from "../../🧫️fixtures/🪪️document-contract/🔣️.json" with { type: "json" };
import { parseCadWorldWindowConfig } from "../../🟦️.ts";

export function testCadWorldWindowConfigContract(): void {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  for (const value of fixture.valid) {
    assert(validate(value), JSON.stringify(validate.errors));
    assert.deepEqual(parseCadWorldWindowConfig(value), value);
  }
  for (const row of fixture.invalid) {
    const value = structuredClone(fixture.valid[0]) as Record<string, any>;
    if (row.kind === "unknown") value[row.path!] = row.value;
    if (row.kind === "position") value.camera.position = row.value;
    if (row.kind === "sun") value.sun = row.value;
    assert(!validate(value), row.kind);
    assert.throws(() => parseCadWorldWindowConfig(value), row.kind);
  }
  const patched = applyPatch(structuredClone(fixture.valid[0]), fixture.patch as Operation[], true).newDocument;
  assert.equal(patched.camera.zoom, fixture.patchedZoom);
  assert(validate(patched), JSON.stringify(validate.errors));
  assert.deepEqual(parseCadWorldWindowConfig(patched), patched);
  console.log("[DEBUG] CAD exact world-window config agrees with Ajv and fast-json-patch vectors");
}

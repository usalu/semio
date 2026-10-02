import assert from "node:assert/strict";
import Ajv from "ajv";
import schema from "../../🔣️.json" with { type: "json" };
import fixture from "../../🧫️fixtures/🪪️document/🔣️.json" with { type: "json" };
import { applyCadWorldWindowTransientMutation, parseCadWorldWindowTransient } from "../../🟦️.ts";

/** ⚖️ The CAD world-window transient contract: Ajv (third-party) and the TypeScript twin agree on every fixture row. */
export function testCadWorldWindowTransientContract(): number {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  for (const value of fixture.valid) {
    assert(validate(value), JSON.stringify(validate.errors));
    assert.deepEqual(parseCadWorldWindowTransient(value), value);
    assert.deepEqual(applyCadWorldWindowTransientMutation(fixture.valid[0]!, { kind: "snapshot", transient: value }), value);
  }
  for (const row of fixture.invalid) {
    assert(!validate(row.value), row.id);
    assert.throws(() => parseCadWorldWindowTransient(row.value), row.id);
  }
  return fixture.valid.length + fixture.invalid.length;
}

/** 🧪️ Independent AJV admission for the neutral IO authority corpus. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

/** 🔎️ Checks closed source cases independently of the native syntax parser. */
export function testGeneration3dIoAuthorityFixture(): number {
  const fixture = JSON.parse(readFileSync(new URL("../../../🧫️fixtures/🛡️authority/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const oracle = new Ajv({ strict: true });
  oracle.addKeyword({ keyword: "x-semio-formats", schemaType: "array", valid: true });
  const validate = oracle.compile(schema.$defs.Generation3dIoAuthority);
  assert(validate(fixture), JSON.stringify(validate.errors));
  assert.equal(new Set(fixture.cases.map((entry: { id: string }) => entry.id)).size, fixture.cases.length);
  for (const entry of fixture.cases) {
    assert(validate({ ...fixture, cases: [entry] }), entry.id);
    assert.equal(validate({ ...fixture, cases: [{ ...entry, allowed: "true" }] }), false);
  }
  assert.equal(validate({ ...fixture, forbiddenSegments: [] }), false);
  assert.equal(validate({ ...fixture, unexpected: true }), false);
  return 4 + fixture.cases.length * 2;
}

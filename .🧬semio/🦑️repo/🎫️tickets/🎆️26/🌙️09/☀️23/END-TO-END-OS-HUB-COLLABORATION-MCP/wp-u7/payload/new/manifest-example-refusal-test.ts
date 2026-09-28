/** 📚️ The example-seat predicate and the example-refusal notices over `🧫️fixtures/📚️example-refusal.json`, the rows the
 * Rust twin (`🦀️.rs` beside this file) reads too; oracle: ajv validates the fixture against the manifest schema — its
 * notices against the schema's `const` table — and rejects hostile rows. */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import toolRunSchema from "../../../⏯️tool-run/🧬️schema/🔣️.json";
import fixture from "../../🧫️fixtures/📚️example-refusal.json";
import schema from "../../🧬️schema/🔣️.json";
import { EXAMPLE_REFUSAL_CODES_V1, EXAMPLE_REFUSAL_MESSAGES_V1, exampleRefusalNoticeTextV1, exampleSeatOutcomeV1, isExampleRefusalCodeV1, type ExampleDecodeV1 } from "../../🟦️.ts";

const ajv = new Ajv({ strict: true, allErrors: true }).addSchema(toolRunSchema).addSchema(schema);
const validator = (name: string) => ajv.getSchema(`${schema.$id}#/$defs/${name}`)!;

describe("📚️ example refusal", () => {
  test("the fixture validates against the manifest schema", () => {
    const validate = validator("ExampleRefusalFixture");
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  test("hostile rows and refusals are rejected", () => {
    const validate = validator("ExampleRefusalFixture");
    const refusal = validator("ExampleRefusalV1");
    const row = fixture.cases[0]!;
    expect(validate({ ...fixture, cases: [{ ...row, decode: "partial" }] })).toBe(false);
    expect(validate({ ...fixture, cases: [{ ...row, expected: { refusal: "example.stale" } }] })).toBe(false);
    expect(validate({ ...fixture, cases: [{ ...row, expected: { seat: "genesis", refusal: "example.unknown" } }] })).toBe(false);
    expect(validate({ ...fixture, messages: { ...fixture.messages, "example.unknown": { en: "Unknown.", de: "Unbekannt." } } })).toBe(false);
    expect(refusal({ code: "example.undecodable", exampleId: "demo", detail: "line 1" })).toBe(true);
    expect(refusal({ code: "example.undecodable", exampleId: "", detail: "line 1" })).toBe(false);
    expect(refusal({ code: "mutation.rejected", exampleId: "demo", detail: "" })).toBe(false);
  });

  test("every example switch seats or refuses as the fixture says", () => {
    for (const row of fixture.cases) {
      expect(exampleSeatOutcomeV1(row.exampleId, row.declared, row.decode as ExampleDecodeV1), row.name).toEqual(row.expected as ReturnType<typeof exampleSeatOutcomeV1>);
    }
  });

  test("every refusal code carries the fixture's English and German notice", () => {
    expect(Object.keys(fixture.messages).sort()).toEqual([...EXAMPLE_REFUSAL_CODES_V1].sort());
    for (const code of EXAMPLE_REFUSAL_CODES_V1) {
      expect(EXAMPLE_REFUSAL_MESSAGES_V1[code]).toEqual(fixture.messages[code]);
      expect(exampleRefusalNoticeTextV1(code, "en")).toBe(fixture.messages[code].en);
      expect(exampleRefusalNoticeTextV1(code, "de")).toBe(fixture.messages[code].de);
      expect(isExampleRefusalCodeV1(code)).toBe(true);
    }
    expect(isExampleRefusalCodeV1("mutation.rejected")).toBe(false);
  });
});

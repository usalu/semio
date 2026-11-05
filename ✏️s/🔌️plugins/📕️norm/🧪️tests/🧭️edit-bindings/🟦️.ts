/** 🧭️ Closed binding lookup vectors stay language-neutral and schema checked. */
import { expect, test } from "bun:test";
import { semioSchemaAjvV1 } from "@semio-tech/framework/schema/oracles/validator";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import fixture from "./🧫️cases/🔣️.json" with { type: "json" };
test("binding capability vectors have closed expected shapes", () => {
  const validate = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...fixture, extra: true })).toBe(false);
  expect(new Set(fixture.cases.map(row => row.path)).size).toBe(fixture.cases.length);
});

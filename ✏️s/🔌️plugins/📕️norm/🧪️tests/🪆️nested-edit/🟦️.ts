/** 🪆️ Checks the closed, renderer-neutral mutation-boundary corpus with the independent schema oracle. */
import { expect, test } from "bun:test";
import { semioSchemaAjvV1 } from "@semio-tech/framework/schema/oracles/validator";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import fixture from "./🧫️cases/🔣️.json" with { type: "json" };

test("nested edit cases have one unambiguous expected mutation or refusal", () => {
  const validate = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(new Set(fixture.cases.map(row => row.name)).size).toBe(fixture.cases.length);
  expect(fixture.cases.some(row => "expected" in row)).toBe(true);
  expect(fixture.cases.some(row => "error" in row)).toBe(true);
  expect(validate({ ...fixture, cases: [{ ...fixture.cases[0], error: "ambiguous" }] })).toBe(false);
  expect(validate({ ...fixture, cases: [{ ...fixture.cases[0], rules: "unknown" }] })).toBe(false);
  expect(validate({ ...fixture, cases: [{ name: "missing expectation", rules: "parent", path: "configuration.a", value: 3 }] })).toBe(false);
});

test("typed admission accepts the original carrier without UI numeric rounding", () => {
  const integer = semioSchemaAjvV1({ strict: true, allErrors: true }).compile({ type: "integer", minimum: 0 });
  for (const row of fixture.typedAdmission) {
    expect(integer(row.value), row.name).toBe(row.accept);
    if ("count" in row) expect(row.count).toBe(row.value);
  }
  const validate = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(schema);
  expect(validate({ ...fixture, typedAdmission: [{ name: "contradictory", value: 1, accept: false, count: 1 }] })).toBe(false);
});

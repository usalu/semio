/** 🧾️ Validates the renderer-neutral EN 1990 engineering journey before a browser is launched. */
import { expect, test } from "bun:test";
import { semioSchemaAjvV1 } from "@semio-tech/framework/schema/oracles/validator";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import witness from "../🧫️witness/🔣️.json" with { type: "json" };
import { timeTravelScenarioV1 } from "../🟦️.ts";

test("engineering witness is closed and bilingual under the independent schema oracle", () => {
  const validate = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(schema);
  expect(validate(witness), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...witness, locale: "en" })).toBe(false);
  expect(validate({ ...witness, locales: ["en", "en"] })).toBe(false);
  expect(validate({ ...witness, journey: ["boot", "edit", "reload"] })).toBe(false);
  expect(validate({ ...witness, states: { before: witness.states.before } })).toBe(false);
});

test("engineering witness changes the dependent utilization and preserves the SLS owner", () => {
  for (const state of Object.values(witness.states)) expect(state.value / witness.field.limit).toBeCloseTo(state.utilization, 12);
  expect(witness.states.after.value).not.toBe(witness.states.before.value);
  expect(witness.states.after.utilization).not.toBe(witness.states.before.utilization);
  expect(witness.field.key).toBe(`norm-inputs.${witness.field.path}`);
  expect(witness.field.owner).toBe("scope:1:9:bridgeSls6:sls-D1");
  expect(witness.field.ownerWindow).toBe(`norm-${witness.variant}-outliner`);
  for (const serve of Object.values(witness.serves)) {
    const url = new URL(serve);
    expect(url.searchParams.get("plugin")).toBe(witness.variant);
    expect(url.searchParams.get("example")).toBe(witness.example);
    expect(url.searchParams.get("role")).toBe("editor");
  }
});

test("browser scenario exposes every schema-owned journey step through the public harness", () => {
  expect(timeTravelScenarioV1.variant).toBe(witness.variant);
  expect(timeTravelScenarioV1.defaultServes).toEqual(witness.serves);
  expect(timeTravelScenarioV1.order).toEqual(witness.journey.map((_, index) => index + 1));
  expect(timeTravelScenarioV1.stepNumbers).toEqual(timeTravelScenarioV1.order);
  expect(Object.keys(timeTravelScenarioV1.steps).map(Number)).toEqual(timeTravelScenarioV1.order);
  expect(timeTravelScenarioV1.context()).toEqual({});
});

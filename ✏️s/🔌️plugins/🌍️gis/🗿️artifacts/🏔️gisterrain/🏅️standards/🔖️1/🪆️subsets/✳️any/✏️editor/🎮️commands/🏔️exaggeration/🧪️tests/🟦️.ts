import { expect, test } from "bun:test";
import Ajv from "ajv";
import { multiply, round } from "mathjs";
import schema from "../🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

const validate = new Ajv().compile(schema);

test("exaggeration actions inside the slider range satisfy the command schema", () => {
  for (const { action } of fixture.cases) expect(validate(action)).toBe(true);
  for (const action of [...fixture.outOfRange, ...fixture.unknownMembers]) expect(validate(action)).toBe(false);
});

test("the schema bounds are the fixture bounds", () => {
  const { minimum, maximum } = schema.properties.value;
  expect([minimum, maximum]).toEqual([fixture.bounds.minimum, fixture.bounds.maximum]);
  expect(fixture.bounds.staged).toBeGreaterThanOrEqual(minimum);
  expect(fixture.bounds.staged).toBeLessThanOrEqual(maximum);
});

test("every case scales the heights by its factor as an independent matrix product", () => {
  for (const { exaggeration, scaledMillimetres, action } of fixture.cases) {
    const factor = "value" in action ? (action as { value: number }).value : fixture.bounds.absent;
    expect(factor).toBe(exaggeration);
    expect(round(multiply(fixture.heightsMillimetres, factor) as number[])).toEqual(scaledMillimetres);
  }
});

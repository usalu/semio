import { expect, test } from "bun:test";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import { editInputValue } from "../../🟦️.ts";

for (const entry of fixture.cases) test(`widget input: ${entry.id}`, () => {
  const operation = () => editInputValue(entry.types, entry.current, entry.value, "component" in entry ? entry.component : undefined);
  if ("error" in entry) expect(operation).toThrow();
  else expect(operation()).toEqual(entry.expected);
});

test("widget input command matches independent JSON Schema validation", () => {
  const validate = new Ajv().compile(schema);
  for (const entry of fixture.cases) {
    const command = { widgetId: "shape", channel: "parameter", value: entry.value, ...("component" in entry ? {component: entry.component} : {}) };
    expect(validate(command)).toBe(entry.id !== "invalid-component");
  }
  expect(validate({widgetId:"shape",channel:"parameter",value:"1",extra:true})).toBe(false);
});

test("numeric and boolean fields agree with the native JSON oracle", () => {
  for (const entry of fixture.cases.filter(entry => !("error" in entry) && (entry.types[0] === "number" || entry.types[0] === "boolean"))) {
    expect(editInputValue(entry.types, entry.current, entry.value).value).toEqual(JSON.parse(entry.value));
  }
});


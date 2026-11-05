import assert from "node:assert/strict";
import Ajv from "ajv";
import {parseFormsConfig} from "../🧬️schema/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔁️mutation-contracts.json";

/** 🎚️ Both app surfaces share one schema-owned host contribution projection. */
export function testFormsConfig(): void {
  const validate = new Ajv().addKeyword("x-semio-state").compile(schema);
  for (const value of [fixture.base,...fixture.cases.map(row=>row.expected)]) {
    assert.equal(validate(value),true,JSON.stringify(validate.errors));
    assert.deepEqual(parseFormsConfig(value),value);
  }
  for (const value of [null,[],{contributionsJson:3}]) {
    assert.equal(validate(value),false);
    assert.throws(()=>parseFormsConfig(value));
  }
}

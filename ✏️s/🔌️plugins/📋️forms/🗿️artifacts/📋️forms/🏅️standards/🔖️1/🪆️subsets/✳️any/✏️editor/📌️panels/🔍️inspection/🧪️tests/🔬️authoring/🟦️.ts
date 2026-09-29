import assert from "node:assert/strict";
import Ajv from "ajv/dist/2020";
import schema from "../../🧬️schema/🔣️.json";
import fixture from "../../🧫️fixtures/🔣️authoring.json";
import { inspectionModel } from "../../🟦️.ts";

/** 🧪️ Both implementations share selection and field-availability vectors checked by Ajv. */
export function testFormsInspectionAuthoring(): void {
  const validate = new Ajv({ strict: true }).compile(schema);
  for (const item of fixture.cases) {
    const actual = inspectionModel(fixture.steps, item.selected);
    assert.ok(validate(actual), JSON.stringify(validate.errors));
    assert.deepEqual(actual, item.expected, item.name);
  }
}

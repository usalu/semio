import assert from "node:assert/strict";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json";
import responseSchema from "../../🔣️.json";
import { exportResponses, exportResponseChunks, responseRows } from "../🟦️.ts";

/** 📤️ Exports retain typed answer values and the labels captured at submission. */
export function testFormsResponseExport(): void {
  const validate = new Ajv().compile({ type: "array", items: responseSchema });
  for (const test of fixture.cases) {
    assert.deepEqual(responseRows(test.responses), test.rows, test.name);
    assert.equal(exportResponses(test.responses, "csv"), test.csv, test.name);
    assert.equal([...exportResponseChunks(test.responses, "csv")].join(""), test.csv, test.name);
    assert.deepEqual(JSON.parse([...exportResponseChunks(test.responses, "json")].join("")), test.responses, test.name);
    const interrupted = exportResponseChunks(test.responses, "json");
    interrupted.next();
    interrupted.return();
    assert.equal(interrupted.next().done, true, "cancelled export does not continue");
    const json = JSON.parse(exportResponses(test.responses, "json"));
    assert.equal(validate(json), true, test.name);
    assert.deepEqual(json, test.responses, test.name);
  }
  console.log("[DEBUG] Forms response JSON/CSV exports matched shared vectors and Ajv");
}

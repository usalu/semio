import {parseFormsJsonResponse} from "../../../🔣️json/🟦️.ts";
import assert from "node:assert/strict";
import { semioSchemaAjvV1 } from "../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json";
import responseSchema from "../../../../../../🧬️schema/📨️response/🔣️.json";
import { exportResponses, exportResponseChunks, responseRows } from "../🟦️.ts";

/** 📤️ Exports retain typed answer values and the labels captured at submission. */
export function testFormsResponseExport(): void {
  const validate = semioSchemaAjvV1().compile({ type: "array", items: responseSchema });
  for (const test of fixture.cases) {
    const responses=test.responses.map(parseFormsJsonResponse);
    assert.deepEqual(responseRows(responses), test.rows, test.name);
    assert.equal(exportResponses(responses, "csv"), test.csv, test.name);
    assert.equal([...exportResponseChunks(responses, "csv")].join(""), test.csv, test.name);
    assert.deepEqual(JSON.parse([...exportResponseChunks(responses, "json")].join("")), test.responses, test.name);
    const interrupted = exportResponseChunks(responses, "json");
    interrupted.next();
    interrupted.return();
    assert.equal(interrupted.next().done, true, "cancelled export does not continue");
    const json = JSON.parse(exportResponses(responses, "json"));
    assert.equal(validate(json), true, test.name);
    assert.deepEqual(json, test.responses, test.name);
  }
}

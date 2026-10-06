import assert from "node:assert/strict";
import Ajv from "ajv";
import { applyPatch, type Operation } from "fast-json-patch";
import corpus from "../../🧫️fixtures/📦️numerical-close/🔣️.json" with { type: "json" };


export function testFem3dNumericalCloseOwners(): void {
  
  
  for (const row of corpus.cases) {
    assert.deepEqual(applyPatch(row.before, row.patch as Operation[], true, false).newDocument, row.expected);
    assert.equal(row.expected.complete, false);
    assert.equal(row.expected.lane, row.lane + 1);
  }
}


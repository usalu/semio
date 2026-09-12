import resultModeSchema from "../../../../../../../../../../../../../../../⚙️engine/🖥️app-surface/🧬️schema/👁️result-mode/🔣️.json" with { type: "json" };
import assert from "node:assert/strict";
import Ajv from "ajv/dist/2020";
import draft7 from "ajv/dist/refs/json-schema-draft-07.json";
import viewportSchema from "../../../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🔣️.json";
import { applyPatch, type Operation } from "fast-json-patch";
import schema from "../../🔣️.json" with { type: "json" };
import fixture from "../../🧫️fixtures/🪪️document-contract/🔣️.json" with { type: "json" };
import { parseFem3dResultsWindowConfig } from "../../🟦️.ts";

export function testFem3dResultsWindowConfigContract(): void {
  const validate = new Ajv({ strict: true, allErrors: true }).addMetaSchema(draft7).addSchema(viewportSchema).addSchema(resultModeSchema).compile(schema);
  const base = fixture.valid[0];
  for (const candidate of fixture.valid) {
    assert(validate(candidate), JSON.stringify(validate.errors));
    assert.deepEqual(parseFem3dResultsWindowConfig(candidate), candidate);
  }
  for (const row of fixture.invalid) {
    const candidate = structuredClone(base) as Record<string, any>;
    if (row.kind === "unknown") candidate.locale = "de";
    if (row.kind === "camera-null") candidate.camera = null;
    if (row.kind === "camera-unknown") candidate.camera.extra = true;
    if (row.kind === "camera-opaque") candidate.camera = { json: "{}" };
    if (row.kind === "camera-short") candidate.camera.position = [8, -3];
    if (row.kind === "camera-zero") candidate.camera.zoom = 0;
    if (row.kind === "bad-mode") candidate.resultMode = "harmonic";
    assert(!validate(candidate), row.kind);
    assert.throws(() => parseFem3dResultsWindowConfig(candidate), row.kind);
  }
  const patched = applyPatch(structuredClone(base), fixture.patch as Operation[], true).newDocument;
  assert(validate(patched), JSON.stringify(validate.errors));
  assert.deepEqual(parseFem3dResultsWindowConfig(patched), patched);
  console.log("[DEBUG] FEM 3D results exact window config agrees with Ajv and fast-json-patch vectors");
}

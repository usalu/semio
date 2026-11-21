import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { parseGisTerrainWindowConfig } from "../../🧬️schema/🟦️.ts";

/** 🧪️ Compares the production parser and independent Ajv validation for exact Terrain-window camera ownership. */
export function testGisTerrainWindowConfigContract(): void {
  const here = fileURLToPath(new URL(".", import.meta.url));
  const fixture = JSON.parse(readFileSync(`${here}/../🧫️fixtures/🔬️window/🔣️.json`, "utf8")) as {
    windowKindId: string;
    base: unknown;
    accepted: unknown[];
    rejected: unknown[];
    cases: Array<{windowId: string; cameraJson: string; expected: Record<string, unknown>}>;
  };
  const schema = JSON.parse(readFileSync(`${here}/../../🧬️schema/🔣️.json`, "utf8"));
  const brushSchema = JSON.parse(readFileSync(`${here}/../../🧬️schema/brush.json`, "utf8"));
  const ajv = new Ajv({ strict: false, allErrors: true }).addSchema(brushSchema);
  const validate = ajv.compile(schema);
  const brushMutation = JSON.parse(readFileSync(`${here}/../../🧬️schema/🧬️mutations/🖌️brush/🧬️schema/🔣️.json`, "utf8"));
  const validateBrush = ajv.compile(brushMutation);
  assert.equal(validateBrush({ operation: "setTerrainBrush" }), true);
  assert.equal(validateBrush({ operation: "setCamera", brush: null }), false);
  for (const [rows, expected] of [[fixture.accepted, true], [fixture.rejected, false]] as const) {
    for (const value of rows) {
      if (!value || typeof value !== "object" || !("brush" in value)) continue;
      assert.equal(validateBrush({ operation: "setTerrainBrush", brush: value.brush }), expected, JSON.stringify(validateBrush.errors));
    }
  }
  const productionAccepts = (value: unknown): boolean => {
    try { parseGisTerrainWindowConfig(value); return true; } catch { return false; }
  };
  for (const value of fixture.accepted) {
    assert.equal(productionAccepts(value), true);
    assert.equal(validate(value), true, JSON.stringify(validate.errors));
  }
  for (const value of fixture.rejected) {
    assert.equal(productionAccepts(value), false);
    assert.equal(validate(value), false);
  }
  const states: Record<string, unknown> = {
    "terrain-left": fixture.base,
    "terrain-right": fixture.base,
  };
  for (const row of fixture.cases) {
    states[row.windowId] = { cameraJson: row.cameraJson };
    assert.deepEqual(states, row.expected);
  }
  assert.equal(schema.properties.cameraJson["x-semio-owner"], "window");
  assert.equal(schema.properties.brush["x-semio-owner"], "window");
  assert.equal(fixture.windowKindId, "gis3d-main");
  console.log(`gis-terrain-window-config accepted=${fixture.accepted.length} rejected=${fixture.rejected.length} windows=${Object.keys(states).length} owner=${fixture.windowKindId}`);
}

if (import.meta.main) testGisTerrainWindowConfigContract();

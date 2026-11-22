import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import Ajv from "ajv";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const config = resolve(process.argv[2], "✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config");
const surface = resolve("🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🖌️brush");
const browser = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗺️WorldTerrainLayer/🖌️brush");
const load = async (base, path) => JSON.parse(await readFile(resolve(base, path), "utf8"));
const [{ parseGisTerrainWindowConfig }, { parseTerrainBrushAction }, windowSchema, settingsSchema, windowFixture, actionSchema, actionFixture] = await Promise.all([
  import(pathToFileURL(resolve(config, "🧬️schema/🟦️.ts")).href),
  import(pathToFileURL(resolve(browser, "🟦️.ts")).href),
  load(config, "🧬️schema/🔣️.json"),
  load(config, "🧬️schema/brush.json"),
  load(config, "🧪️tests/🧫️fixtures/🔬️window/🔣️.json"),
  load(surface, "🔣️.json"),
  load(surface, "🧪️tests/🔣️.json"),
]);
const windowValidator = new Ajv({ strict: false }).addSchema(settingsSchema).compile(windowSchema);
const actionValidator = new Ajv({ multipleOfPrecision: 7 }).compile(actionSchema);
for (const row of windowFixture.accepted) {
  assert.equal(windowValidator(row), true, JSON.stringify(row));
  const expected = row.brush === null ? { cameraJson: row.cameraJson } : row;
  assert.deepEqual(parseGisTerrainWindowConfig(row), expected);
}
for (const row of windowFixture.rejected) {
  assert.equal(windowValidator(row), false, JSON.stringify(row));
  assert.throws(() => parseGisTerrainWindowConfig(row));
}
for (const row of actionFixture.accepted) {
  assert.equal(actionValidator(row), true, JSON.stringify(row));
  assert.deepEqual(parseTerrainBrushAction(row), row);
}
for (const row of actionFixture.refused) {
  assert.equal(actionValidator(row), false, JSON.stringify(row));
  assert.equal(parseTerrainBrushAction(row), undefined);
}
const minimum = { cameraJson: "", brush: { radius: 4, elevationStepMillimetres: 1, smoothStrength: 50 } };
assert.deepEqual(parseGisTerrainWindowConfig(minimum), minimum);
const step = windowFixture.brush.settings.elevationStep;
assert.equal(step, 0.031);
assert.equal(parseTerrainBrushAction(windowFixture.brush.expectedTarget).arguments.value, -step);
console.log("[DEBUG] Millimetre window settings and fractional World3d brush actions match the independent schema oracles.");

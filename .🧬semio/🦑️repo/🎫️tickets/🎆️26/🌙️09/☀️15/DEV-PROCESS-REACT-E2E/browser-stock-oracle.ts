import assert from "node:assert/strict";
import { Box3, Mesh, Vector3 } from "three";
import { OBJLoader } from "three/addons/loaders/OBJLoader.js";
import fixture from "./browser-import-fixture.json";

const stock = new OBJLoader().parse(await Bun.file(new URL("./browser-stock.obj", import.meta.url)).text());
const size = new Box3().setFromObject(stock).getSize(new Vector3());
assert.deepEqual(size.toArray(), [2, 2, 2]);
const surface = stock.children[0];
assert.ok(surface instanceof Mesh);
const positions = surface.geometry.getAttribute("position");
assert.equal(positions.count, 36);
const edgeUses = new Map<string, number>();
let volumeTimesSix = 0;
for (let index = 0; index < positions.count; index += 3) {
  const a = new Vector3().fromBufferAttribute(positions, index);
  const b = new Vector3().fromBufferAttribute(positions, index + 1);
  const c = new Vector3().fromBufferAttribute(positions, index + 2);
  for (const [start, end] of [[a, b], [b, c], [c, a]]) {
    const edge = [start.toArray().join(","), end.toArray().join(",")].sort().join("|");
    edgeUses.set(edge, (edgeUses.get(edge) ?? 0) + 1);
  }
  volumeTimesSix += a.dot(b.cross(c));
}
assert.ok([...edgeUses.values()].every(count => count === 2));
assert.equal(volumeTimesSix / 6, 8);
console.log("[DEBUG] Three.js parsed a 12-triangle closed OBJ stock with volume 8 m³");

const step = await Bun.file(fixture.step.source).text();
assert.equal(Buffer.byteLength(step), fixture.step.rawFileBytes);
assert.ok(fixture.step.rawFileBytes < fixture.maximumRawFileBytes);
assert.match(step, /LENGTH_UNIT\(\).*SI_UNIT\(\.MILLI\.,\.METRE\.\)/);
const points = new Map([...step.matchAll(/^#(\d+) = CARTESIAN_POINT\('',\(([^\n]+)\)\);$/gm)].map(([, id, coordinates]) => [id, coordinates.split(",").map(Number)]));
const vertices = [...step.matchAll(/^#\d+ = VERTEX_POINT\('',#(\d+)\);$/gm)].map(([, id]) => {
  const [x, y, z] = points.get(id)!;
  return new Vector3(x / 1000, y / 1000, z / 1000);
});
assert.equal(vertices.length, 8);
const stepBounds = new Box3().setFromPoints(vertices);
assert.deepEqual(stepBounds.getSize(new Vector3()).toArray(), fixture.step.extentMetres);
console.log("[DEBUG] Three.js verified the STEP fixture spans 20 mm on all three axes");

const extensionRoot = "✏️s/🔌️plugins/🏭️process/🧩️extensions";
const catalogs = new Map();
for (const file of new Bun.Glob("*/🧩️component/🔣️.json").scanSync({ cwd: extensionRoot })) {
  const descriptor = await Bun.file(`${extensionRoot}/${file}`).json();
  const { payload } = descriptor.manifest.topicContributions.find((row: { topic: string }) => row.topic === "process.machines");
  catalogs.set(payload.moduleId, payload);
}
const boxVolume = (bounds: Box3) => {
  const { x, y, z } = bounds.getSize(new Vector3());
  return x * y * z;
};
const retained = stepBounds.clone();
const original = stepBounds.getSize(new Vector3());
const stockQuantity = new Map([
  ["width", original.x],
  ["depth", original.y],
  ["height", original.z],
  ["maxDimension", Math.max(original.x, original.y, original.z)],
  ["minDimension", Math.min(original.x, original.y, original.z)],
]);
for (const row of fixture.workshop) {
  const catalog = catalogs.get(row.catalog);
  assert.equal(catalog.label, row.catalogLabel);
  const machine = JSON.parse(catalog.machinesJson).find((candidate: { id: string }) => candidate.id === row.machine);
  const capability = machine.capabilities.find((candidate: { id: string }) => candidate.id === row.capability);
  const getSize = (id: string) => capability.parameters.find((parameter: { id: string }) => parameter.id === id).value;
  const { recipe } = capability;
  const [width, depth, height] = row.toolSizeMetres;
  for (const rule of capability.rules) {
    const actual = stockQuantity.get(rule.quantity);
    assert.notEqual(actual, undefined);
    assert.equal(rule.kind, "max");
    assert.ok(actual! + rule.margin <= getSize(rule.parameter), `${row.machine} works with the original stock`);
  }
  let volume = boxVolume(retained);
  if (row.operation === "slab") {
    assert.equal(recipe.recipe, "discCut");
    assert.deepEqual(row.toolSizeMetres, [getSize(recipe.diameter), getSize(recipe.diameter), getSize(recipe.kerf)]);
    assert.ok(Math.hypot(retained.max.x, retained.max.y) < width / 2);
    retained.min.z = Math.max(retained.min.z, height / 2);
    volume = boxVolume(retained);
  } else {
    assert.equal(recipe.recipe, "pocketCut");
    assert.deepEqual(row.toolSizeMetres, [getSize(recipe.diameter), getSize(recipe.diameter), getSize(recipe.depth)]);
    const half = new Vector3(width / 2, depth / 2, height / 2);
    const pocket = new Box3(half.clone().negate(), half).intersect(retained);
    volume -= boxVolume(pocket);
  }
  assert.ok(Math.abs(volume - row.remainingVolumeMetresCubed) < 1e-12, row.machine);
}
console.log("[DEBUG] Three.js matched the workpiece volume at all four real catalog steps");

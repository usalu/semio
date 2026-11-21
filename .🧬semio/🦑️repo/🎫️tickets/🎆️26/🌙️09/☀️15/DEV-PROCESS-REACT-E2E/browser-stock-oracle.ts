import assert from "node:assert/strict";
import { Box3, CylinderGeometry, Mesh, Vector3 } from "three";
import { OBJLoader } from "three/addons/loaders/OBJLoader.js";
import fixture from "./browser-import-fixture.json";
import { objVolume } from "./obj-volume";

const stockSource = await Bun.file(new URL("./browser-stock.obj", import.meta.url)).text();
const stock = new OBJLoader().parse(stockSource);
const size = new Box3().setFromObject(stock).getSize(new Vector3());
assert.deepEqual(size.toArray(), [2, 2, 2]);
const surface = stock.children[0];
assert.ok(surface instanceof Mesh);
const positions = surface.geometry.getAttribute("position");
assert.equal(positions.count, 36);
const edgeUses = new Map<string, number>();
for (let index = 0; index < positions.count; index += 3) {
  const a = new Vector3().fromBufferAttribute(positions, index);
  const b = new Vector3().fromBufferAttribute(positions, index + 1);
  const c = new Vector3().fromBufferAttribute(positions, index + 2);
  for (const [start, end] of [[a, b], [b, c], [c, a]]) {
    const edge = [start.toArray().join(","), end.toArray().join(",")].sort().join("|");
    edgeUses.set(edge, (edgeUses.get(edge) ?? 0) + 1);
  }
}
assert.ok([...edgeUses.values()].every(count => count === 2));
assert.equal(objVolume(stockSource), 8);
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
  assert.equal(capability.label, row.stepLabel);
  const getSize = (id: string) => capability.parameters.find((parameter: { id: string }) => parameter.id === id).value;
  const { recipe } = capability;
  const [width, depth, height] = row.toolSizeMetres;
  for (const rule of capability.rules) {
    const actual = stockQuantity.get(rule.quantity);
    assert.notEqual(actual, undefined);
    assert.equal(rule.kind, "max");
    assert.ok(actual! + rule.margin <= getSize(rule.parameter), `${row.machine} works with the original stock`);
  }
  const before = retained.clone();
  let volume = boxVolume(retained);
  let editedVolume: number;
  if (row.operation === "slab") {
    assert.equal(recipe.recipe, "discCut");
    assert.deepEqual(row.toolSizeMetres, [getSize(recipe.diameter), getSize(recipe.diameter), getSize(recipe.kerf)]);
    assert.ok(Math.hypot(retained.max.x, retained.max.y) < width / 2);
    retained.min.z = Math.max(retained.min.z, height / 2);
    volume = boxVolume(retained);
    assert.equal(row.edit.field, "Height");
    before.min.z = Math.max(before.min.z, row.edit.sizeMetres / 2);
    editedVolume = boxVolume(before);
  } else {
    assert.equal(recipe.recipe, "pocketCut");
    assert.deepEqual(row.toolSizeMetres, [getSize(recipe.diameter), getSize(recipe.diameter), getSize(recipe.depth)]);
    const half = new Vector3(width / 2, depth / 2, height / 2);
    const pocket = new Box3(half.clone().negate(), half).intersect(retained);
    volume -= boxVolume(pocket);
    assert.equal(row.edit.field, "Width");
    half.x = row.edit.sizeMetres / 2;
    editedVolume = boxVolume(before) - boxVolume(new Box3(half.clone().negate(), half).intersect(before));
  }
  assert.ok(Math.abs(volume - row.remainingVolumeMetresCubed) < 1e-12, row.machine);
  assert.ok(Math.abs(editedVolume - row.edit.remainingVolumeMetresCubed) < 1e-12, `${row.machine} edited tool`);
  assert.ok(Math.abs(editedVolume - volume) > 1e-12, `${row.machine} edit changes the preview`);
}
console.log("[DEBUG] Three.js matched each catalog step's composed and edited workpiece volumes");

const attachment = fixture.demoAttachment;
const timber = await Bun.file(attachment.replayFixture).json();
const placement = fixture.demoPlacement;
const timberStock = timber.scene.stock;
const timberSize = new Vector3(timberStock.solid.width, timberStock.solid.depth, timberStock.solid.height);
const timberBounds = new Box3().setFromCenterAndSize(new Vector3().fromArray(timberStock.pose.position), timberSize);
const toolBounds = new Box3().setFromCenterAndSize(new Vector3().fromArray(placement.positionMetres), new Vector3().fromArray(placement.toolSizeMetres));
const placedCutVolume = boxVolume(timberBounds) - boxVolume(toolBounds.intersect(timberBounds));
assert.ok(Math.abs(placedCutVolume - placement.remainingVolumeMetresCubed) < 1e-12);
console.log("[DEBUG] Three.js verified the viewport-placed cut leaves 0.1775 m³ of timber");
const [lap, drill, dowel] = timber.scene.steps.slice(-3).map((row: { measure: unknown }) => row.measure);
assert.equal(timber.scene.steps.at(-1).id, attachment.step);
assert.equal(timber.scene.steps.at(-1).label, attachment.label);
assert.equal(dowel.component.height, attachment.originalHeightMetres);
assert.equal(drill.radius, dowel.component.radius);
assert.deepEqual(drill.pose.position.slice(0, 2), dowel.pose.position.slice(0, 2));
const lapFloor = lap.pose.position[2] - lap.tool.height / 2;
const drillFloor = drill.pose.position[2] - drill.depth / 2;
const editedHalfHeight = attachment.editedHeightMetres / 2;
const editedBottom = dowel.pose.position[2] - editedHalfHeight;
const editedTop = dowel.pose.position[2] + editedHalfHeight;
const occupiedLength = Math.max(0, Math.min(drillFloor, lapFloor, editedTop) - editedBottom);
function cylinderVolume(radius: number, height: number, segments: number): number {
  const geometry = new CylinderGeometry(radius, radius, height, segments);
  const positions = geometry.getAttribute("position");
  const indices = geometry.getIndex()!;
  let volumeTimesSix = 0;
  for (let i = 0; i < indices.count; i += 3) {
    const a = new Vector3().fromBufferAttribute(positions, indices.getX(i));
    const b = new Vector3().fromBufferAttribute(positions, indices.getX(i + 1));
    const c = new Vector3().fromBufferAttribute(positions, indices.getX(i + 2));
    volumeTimesSix += a.dot(b.cross(c));
  }
  return Math.abs(volumeTimesSix / 6);
}
const drilledTimber = timber.volumes.at(-2);
const editedTimberVolume = drilledTimber + cylinderVolume(dowel.component.radius, attachment.editedHeightMetres - occupiedLength, timber.circularSegments);
assert.ok(Math.abs(editedTimberVolume - attachment.editedVolumeMetresCubed) < timber.volumeTolerance);
assert.ok(editedTimberVolume > timber.volumes.at(-1));
const originalTop = dowel.pose.position[2] + attachment.originalHeightMetres / 2;
const drillTop = drill.pose.position[2] + drill.depth / 2;
const exposedLengthAfterDrill = originalTop - Math.max(drillTop, lapFloor);
const reorderedTimberVolume = drilledTimber + cylinderVolume(dowel.component.radius, exposedLengthAfterDrill, timber.circularSegments);
assert.ok(Math.abs(reorderedTimberVolume - attachment.reorderedVolumeMetresCubed) < timber.volumeTolerance);
assert.ok(reorderedTimberVolume < timber.volumes.at(-1));
console.log("[DEBUG] Three.js verified the edited dowel and the effect of drilling after attachment");

import assert from "node:assert/strict";
import { Box3, CylinderGeometry, Triangle, Vector3 } from "three";

type Coordinate = [number, number, number];
type Step = { catalog: string; machine: string; capability: string; positionMetres: Coordinate };
type Capability = { id: string; recipe: Record<string, string>; parameters: { id: string; value: number }[] };
const roots = "✏️s/🔌️plugins/🏭️process";
const fixture = await Bun.file(roots + "/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧫️fixtures/mixed-capability-replay.json").json() as {
  stockMetres: Coordinate;
  steps: Step[];
  prefixesCubicMetres: number[];
  drillEdit: { radiusMetres: number; volumeCubicMetres: number };
};

async function shippedRecipe(step: Step, folder: string, expectedRecipe: string) {
  const descriptor = await Bun.file(roots + "/🧩️extensions/" + folder + "/🧩️component/🔣️.json").json();
  const topics = descriptor.manifest.topicContributions;
  const topic = topics.find((entry: { topic: string }) => entry.topic === "process.machines");
  assert.equal(topic.payload.moduleId, step.catalog);
  const machines = JSON.parse(topic.payload.machinesJson) as { id: string; capabilities: Capability[] }[];
  const capability = machines.find(machine => machine.id === step.machine)?.capabilities.find(capability => capability.id === step.capability);
  assert.ok(capability);
  assert.equal(capability.recipe.recipe, expectedRecipe);
  return (dimension: string) => {
    const parameter = capability.parameters.find(parameter => parameter.id === capability.recipe[dimension]);
    assert.ok(parameter);
    return parameter.value;
  };
}

function box(center: Coordinate, size: Coordinate) {
  return new Box3().setFromCenterAndSize(new Vector3(...center), new Vector3(...size));
}

function volume(bounds: Box3) {
  const size = bounds.getSize(new Vector3());
  return size.x * size.y * size.z;
}

function cylinderVolume(radius: number, height: number) {
  const geometry = new CylinderGeometry(radius, radius, height, 8192);
  const vertices = geometry.getAttribute("position");
  const indices = geometry.getIndex()!;
  const triangle = new Triangle();
  const cross = new Vector3();
  let sixTimesVolume = 0;
  for (let i = 0; i < indices.count; i += 3) {
    triangle.a.fromBufferAttribute(vertices, indices.getX(i));
    triangle.b.fromBufferAttribute(vertices, indices.getX(i + 1));
    triangle.c.fromBufferAttribute(vertices, indices.getX(i + 2));
    sixTimesVolume += triangle.a.dot(cross.crossVectors(triangle.b, triangle.c));
  }
  geometry.dispose();
  return Math.abs(sixTimesVolume) / 6;
}

const [cut, drill, attach] = fixture.steps;
const cutDimension = await shippedRecipe(cut, "🧱️concrete", "discCut");
const drillDimension = await shippedRecipe(drill, "🤖️robotic", "boreDrill");
const attachDimension = await shippedRecipe(attach, "🔩️metal", "boxAttach");

const stock = box([0, 0, 0], fixture.stockMetres);
const discRadius = cutDimension("diameter") / 2;
assert.ok(Math.hypot(stock.max.x, stock.max.y) < discRadius);
assert.deepEqual(cut.positionMetres.slice(0, 2), [0, 0]);
assert.ok(cut.positionMetres[2] + cutDimension("kerf") / 2 > stock.max.z);
const shaved = stock.clone();
shaved.max.z = cut.positionMetres[2] - cutDimension("kerf") / 2;

const plate = box(attach.positionMetres, [attachDimension("width"), attachDimension("depth"), attachDimension("height")]);
const overlap = shaved.clone().intersect(plate);
assert.ok(volume(overlap) > 0);
const addedPlate = volume(plate) - volume(overlap);

const drilled = (radius: number) => {
  const bore = box(drill.positionMetres, [radius * 2, radius * 2, drillDimension("depth")]);
  assert.ok(bore.min.x > shaved.min.x && bore.max.x < shaved.max.x);
  assert.ok(bore.min.y > shaved.min.y && bore.max.y < shaved.max.y);
  assert.ok(bore.min.z < shaved.min.z && bore.max.z > shaved.max.z);
  assert.ok(!bore.intersectsBox(plate));
  return volume(shaved) - cylinderVolume(radius, shaved.getSize(new Vector3()).z);
};
const original = drilled(drillDimension("radius"));
const actual = [volume(stock), volume(shaved), original, original + addedPlate, drilled(fixture.drillEdit.radiusMetres) + addedPlate];
const expected = [...fixture.prefixesCubicMetres, fixture.drillEdit.volumeCubicMetres];
for (const [index, value] of actual.entries()) {
  assert.ok(Math.abs(value - expected[index]) < 1e-10, `replay volume ${index}: ${value} != ${expected[index]}`);
}
console.log("[DEBUG] Three.js geometry agrees with all five mixed-extension fixture volumes");

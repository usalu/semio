import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import { Matrix4, Quaternion, Vector3 } from "three";
import fixture from "./🔣️original-box-section.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vec3 = [number, number, number];
type Mat4 = Parameters<ReturnType<typeof Manifold.cube>["transform"]>[0];

function stockFor(row: { rotation?: number; clip?: { origin: number[]; normal: number[] } }) {
  const cube = Manifold.cube(fixture.box.size as Vec3);
  const rotated = cube.rotate([0, 0, row.rotation ?? 0]);
  const stock = rotated.translate(fixture.box.origin as Vec3);
  cube.delete();
  rotated.delete();
  if (!row.clip) return stock;
  const normal = new Vector3().fromArray(row.clip.normal).normalize();
  const [positive, negative] = stock.splitByPlane(normal.toArray(), normal.dot(new Vector3().fromArray(row.clip.origin)));
  stock.delete();
  negative.delete();
  return positive;
}

for (const row of fixture.cases) {
  test(`Manifold slice verifies the ${row.name}`, () => {
    const stock = stockFor(row);
    const origin = new Vector3().fromArray(row.origin);
    const rotation = new Quaternion().setFromUnitVectors(new Vector3().fromArray(row.normal).normalize(), new Vector3(0, 0, 1));
    const local = stock.translate(origin.clone().negate().toArray());
    const projected = local.transform(new Matrix4().makeRotationFromQuaternion(rotation).toArray() as Mat4);
    const section = projected.slice(0);
    const boundary = section.simplify(1e-8);
    expect(boundary.area()).toBeCloseTo(row.area, 7);
    const polygons = boundary.toPolygons();
    expect(polygons).toHaveLength(1);
    expect(polygons[0]).toHaveLength(row.vertices);
    const inverse = rotation.clone().invert();
    const world = polygons[0].map(([x, y]) => new Vector3(x, y, 0).applyQuaternion(inverse).add(origin).toArray());
    for (let axis = 0; axis < 3; axis++) {
      expect(Math.min(...world.map(point => point[axis]))).toBeCloseTo(row.bounds[0][axis], 7);
      expect(Math.max(...world.map(point => point[axis]))).toBeCloseTo(row.bounds[1][axis], 7);
    }
    for (const item of [boundary, section, projected, local, stock]) item.delete();
  });
}
for (const row of fixture.refused) {
  test(`Manifold verifies that ${row.name ?? row.origin} does not cross the stock interior`, () => {
    const normal = new Vector3().fromArray(row.normal);
    if (normal.lengthSq() === 0) {
      expect(normal.length()).toBe(0);
      return;
    }
    normal.normalize();
    const stock = stockFor(row);
    const [positive, negative] = stock.splitByPlane(normal.toArray(), normal.dot(new Vector3().fromArray(row.origin)));
    expect(Math.min(Math.abs(positive.volume()), Math.abs(negative.volume()))).toBeLessThan(1e-9);
    for (const item of [positive, negative, stock]) item.delete();
  });
}

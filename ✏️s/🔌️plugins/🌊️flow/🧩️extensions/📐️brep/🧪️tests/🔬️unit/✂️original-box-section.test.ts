import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import { Matrix4, Quaternion, Vector3 } from "three";
import fixture from "./🔣️original-box-section.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vec3 = [number, number, number];
type Mat4 = Parameters<ReturnType<typeof Manifold.cube>["transform"]>[0];

for (const row of fixture.cases) {
  test(`Manifold slice verifies the ${row.name}`, () => {
    const cube = Manifold.cube(fixture.box.size as Vec3);
    const stock = cube.translate(fixture.box.origin as Vec3);
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
    for (const item of [boundary, section, projected, local, stock, cube]) item.delete();
  });
}

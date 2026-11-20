import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import { Vector3 } from "three";
import fixture from "./🔣️original-box-split.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vec3 = [number, number, number];

for (const row of fixture.cases) {
  test(`Manifold splits the ${row.name}`, () => {
    const cube = Manifold.cube(fixture.box.size as Vec3);
    const stock = cube.translate(fixture.box.origin as Vec3);
    const normal = new Vector3().fromArray(row.normal).normalize();
    const distance = normal.dot(new Vector3().fromArray(row.origin));
    const [positive, negative] = stock.splitByPlane(normal.toArray(), distance);
    for (const [solid, expected] of [[positive, row.positive], [negative, row.negative]] as const) {
      expect(solid.volume()).toBeCloseTo(expected.volume, 8);
      expect(solid.boundingBox()).toEqual({ min: expected.bounds[0], max: expected.bounds[1] });
      expect(solid.genus()).toBe(0);
    }
    expect(positive.volume() + negative.volume()).toBeCloseTo(stock.volume(), 8);
    for (const item of [positive, negative, stock, cube]) item.delete();
  });
}

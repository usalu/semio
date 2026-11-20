import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import fixture from "./🔣️original-offset-face.json";

const wasm = await loadManifold();
wasm.setup();
const { CrossSection } = wasm;

for (const row of fixture.cases) {
  test(`Manifold translates the exact-trim reference for ${row.kind}`, () => {
    const [major, minor] = row.radii;
    const circle = CrossSection.circle(major, 1024);
    const profile = circle.scale([1, minor / major]);
    const extrusion = profile.extrude(0.25);
    const oriented = extrusion.rotate(row.axis === 0 ? [0, 90, 0] : [0, 0, 90]);
    const offset = row.center.map((value, axis) => value + row.normal[axis] * row.distance) as [number, number, number];
    const solid = oriented.translate(offset);
    const bounds = solid.boundingBox();
    expect(offset).toEqual(row.target);
    expect(Math.abs(profile.area() / row.area - 1)).toBeLessThan(0.00001);
    for (let axis = 0; axis < 3; axis++) {
      expect(bounds.min[axis]).toBeCloseTo(row.bounds[0][axis], 5);
      if (axis !== row.axis) expect(bounds.max[axis]).toBeCloseTo(row.bounds[1][axis], 5);
    }
    for (const item of [solid, oriented, extrusion, profile, circle]) item.delete();
  });
}

import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import fixture from "./🔣️original-thicken.json";

const wasm = await loadManifold();
wasm.setup();
const { CrossSection } = wasm;

for (const row of fixture.cases) {
  test(`Manifold thickens the ${row.profile} to the expected signed bounds and volume`, () => {
    const circle = row.profile === "circle";
    const profile = circle
      ? CrossSection.circle(row.inputs.radius!, 1024)
      : CrossSection.square([row.inputs.width!, row.inputs.height!]);
    const extrusion = profile.extrude(Math.abs(row.thickness));
    const oriented = circle ? extrusion.rotate([0, 90, 0]) : extrusion;
    const offset = circle
      ? row.inputs.center as [number, number, number]
      : [0, 0, Math.min(0, row.thickness)] as [number, number, number];
    const solid = oriented.translate(offset);
    const bounds = solid.boundingBox();
    expect(solid.genus()).toBe(0);
    expect(Math.abs(solid.volume() / row.volume - 1)).toBeLessThan(0.00001);
    for (let axis = 0; axis < 3; axis++) {
      expect(bounds.min[axis]).toBeCloseTo(row.bounds[0][axis], 5);
      expect(bounds.max[axis]).toBeCloseTo(row.bounds[1][axis], 5);
    }
    solid.delete();
    if (oriented !== extrusion) oriented.delete();
    extrusion.delete();
    profile.delete();
  });
}

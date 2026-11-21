import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import fixture from "./🔣️original-box-all-edge-fillet.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vec3 = [number, number, number];

test("Manifold independently rounds the inset stock by a spherical Minkowski sum", () => {
  for (const row of fixture.cases) {
    const inset = Manifold.cube(row.size.map(size => size - 2 * row.radius) as Vec3);
    const sphere = Manifold.sphere(row.radius, 192);
    const rounded = inset.minkowskiSum(sphere);
    const placed = rounded.translate(row.origin.map(origin => origin + row.radius) as Vec3);
    try {
      expect(placed.status()).toBe("NoError");
      expect(placed.genus()).toBe(0);
      for (const [index, bounds] of [placed.boundingBox().min, placed.boundingBox().max].entries()) {
        for (const axis of [0, 1, 2]) expect(bounds[axis]).toBeCloseTo(row.bounds[index]![axis]!, 8);
      }
      expect(Math.abs(placed.volume() / row.volume - 1)).toBeLessThan(0.001);
      expect(Math.abs(placed.surfaceArea() / row.area - 1)).toBeLessThan(0.001);
    } finally {
      placed.delete();
      rounded.delete();
      sphere.delete();
      inset.delete();
    }
  }
});

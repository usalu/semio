import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import { Matrix4, Vector3 } from "three";
import fixture from "./🔣️original-oriented-fillet.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vec3 = [number, number, number];
type Matrix = Parameters<ReturnType<typeof Manifold.cube>["transform"]>[0];

for (const row of fixture.cases) {
  test(`Manifold verifies the selected fillet along ${row.name}`, () => {
    const { radius } = fixture;
    const { size, origin, rotateY } = fixture.stock;
    const u = (row.axis + 1) % 3;
    const v = (row.axis + 2) % 3;
    const height = size[row.axis];
    let shape = Manifold.cube([size[u], size[v], height]);
    for (const [point] of row.edges) {
      const lower = [point[u] === 0 ? 0 : size[u] - radius, point[v] === 0 ? 0 : size[v] - radius];
      const center = [point[u] === 0 ? radius : size[u] - radius, point[v] === 0 ? radius : size[v] - radius];
      const corner = Manifold.cube([radius, radius, height + 2]);
      const prism = corner.translate([lower[0], lower[1], -1]);
      const cylinder = Manifold.cylinder(height + 2, radius, radius, 512);
      const tool = cylinder.translate([center[0], center[1], -1]);
      const trimmed = prism.subtract(tool);
      const remaining = shape.subtract(trimmed);
      for (const item of [shape, trimmed, tool, cylinder, prism, corner]) item.delete();
      shape = remaining;
    }
    const basis = [0, 1, 2].map(i => new Vector3().setComponent(i, 1));
    const axial = shape.transform(new Matrix4().makeBasis(basis[u], basis[v], basis[row.axis]).toArray() as Matrix);
    const rotated = axial.rotate([0, rotateY * 180 / Math.PI, 0]);
    const pitched = rotated.rotate([(row.rotateX ?? 0) * 180 / Math.PI, 0, 0]);
    const reflected = pitched.transform(new Matrix4().makeScale(row.mirror ? -1 : 1, 1, 1).toArray() as Matrix);
    const placed = reflected.translate(origin as Vec3);
    expect(placed.volume()).toBeCloseTo(row.volume, 3);
    expect(placed.surfaceArea()).toBeCloseTo(row.area, 3);
    expect(placed.genus()).toBe(0);
    for (const item of [placed, reflected, pitched, rotated, axial, shape]) item.delete();
  });
}

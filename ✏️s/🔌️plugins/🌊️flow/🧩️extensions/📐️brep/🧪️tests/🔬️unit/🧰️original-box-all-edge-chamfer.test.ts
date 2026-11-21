import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import { Matrix4, Vector3 } from "three";
import fixture from "./🔣️original-box-all-edge-chamfer.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vec3 = [number, number, number];
type Matrix = Parameters<ReturnType<typeof Manifold.cube>["transform"]>[0];

test("Manifold independently intersects the stock, edge bevel and corner-cap half spaces", () => {
  for (const row of [...fixture.cases, ...fixture.orientedCases]) {
    let solid = Manifold.cube(row.size as Vec3);
    const normal = new Vector3();
    const lo = [0, 0, 0];
    const hi = row.size;
    const bevel = (sides: [number, number][], inset: number) => {
      normal.set(0, 0, 0);
      let support = 0;
      for (const [axis, sign] of sides) {
        normal.setComponent(axis, -sign);
        support += sign * (sign > 0 ? hi[axis]! : lo[axis]!);
      }
      const clipped = solid.trimByPlane(normal.normalize().toArray(), (inset - support) / Math.sqrt(sides.length));
      solid.delete();
      solid = clipped;
    };
    try {
      for (const a of [0, 1, 2]) {
        const b = (a + 1) % 3;
        for (const u of [-1, 1]) for (const v of [-1, 1]) bevel([[a, u], [b, v]], row.distance);
      }
      for (const x of [-1, 1]) for (const y of [-1, 1]) for (const z of [-1, 1]) {
        bevel([[0, x], [1, y], [2, z]], 2 * row.distance);
      }
      const yaw = "rotateY" in row ? row.rotateY : 0;
      const pitch = "rotateX" in row ? row.rotateX : 0;
      const mirror = "mirror" in row && row.mirror;
      const tilted = solid.rotate([0, yaw * 180 / Math.PI, 0]);
      const pitched = tilted.rotate([pitch * 180 / Math.PI, 0, 0]);
      const reflected = pitched.transform(new Matrix4().makeScale(mirror ? -1 : 1, 1, 1).toArray() as Matrix);
      const placed = reflected.translate(row.origin as Vec3);
      const bounds = placed.boundingBox();
      for (const [i, actual] of [bounds.min, bounds.max].entries()) {
        for (const [axis, value] of actual.entries()) expect(value).toBeCloseTo(row.bounds[i][axis], 9);
      }
      expect(placed.volume()).toBeCloseTo(row.volume, 10);
      expect(placed.surfaceArea()).toBeCloseTo(row.area, 10);
      expect(placed.genus()).toBe(0);
      for (const item of [placed, reflected, pitched, tilted]) item.delete();
    } finally {
      solid.delete();
    }
  }
});

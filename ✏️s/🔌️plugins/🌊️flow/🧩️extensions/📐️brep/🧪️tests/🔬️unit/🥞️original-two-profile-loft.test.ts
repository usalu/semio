import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import fixture from "./🔣️original-two-profile-loft.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vec3 = [number, number, number];
type Solid = ReturnType<typeof Manifold.hull>;

function verify(shape: Solid, row: { volume: number; bounds: number[][] }) {
  const bounds = shape.boundingBox();
  expect(Math.abs(shape.volume() / row.volume - 1)).toBeLessThan(0.00001);
  expect(shape.genus()).toBe(0);
  for (let axis = 0; axis < 3; axis++) {
    expect(bounds.min[axis]).toBeCloseTo(row.bounds[0][axis], 5);
    expect(bounds.max[axis]).toBeCloseTo(row.bounds[1][axis], 5);
  }
}

test("Manifold reproduces the reverse coaxial circular loft", () => {
  const row = fixture.circle;
  const cylinder = Manifold.cylinder(Math.abs(row.distance), row.radius, row.radius, 1024);
  const rotated = cylinder.rotate([-90, 0, 0]);
  const center = row.center.map((value, axis) => value + row.normal[axis] * row.distance) as Vec3;
  const loft = rotated.translate(center);
  verify(loft, row);
  for (const shape of [loft, rotated, cylinder]) shape.delete();
});

test("Manifold reproduces the translated triangular loft with one exact prismatic boundary", () => {
  const row = fixture.polygon;
  const loft = Manifold.hull([...row.first, ...row.last] as Vec3[]);
  verify(loft, row);
  expect(loft.numVert()).toBe(6);
  loft.delete();
});

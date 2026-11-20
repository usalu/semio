import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import fixture from "./🔣️original-convex-hull.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vec3 = [number, number, number];

for (const row of fixture.cases) {
  test(`Manifold verifies the convex hull of ${row.name}`, () => {
    const actual = Manifold.hull(row.points as Vec3[]);
    const bounds = actual.boundingBox();
    expect(bounds.min).toEqual(row.bounds[0]);
    expect(bounds.max).toEqual(row.bounds[1]);
    expect(actual.volume()).toBeCloseTo(row.volume, 10);
    expect(actual.numVert()).toBe(row.vertices);
    expect(actual.numTri()).toBe(2 * row.vertices - 4);
    expect(actual.genus()).toBe(0);
    actual.delete();
  });
}

test("a coplanar Manifold hull encloses no volume", () => {
  const actual = Manifold.hull(fixture.coplanar as Vec3[]);
  expect(actual.volume()).toBe(0);
  const bounds = actual.boundingBox();
  expect(bounds.min[2]).toBe(bounds.max[2]);
  actual.delete();
});

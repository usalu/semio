import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import fixture from "./🔣️original-rotated-box-sets.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vec3 = [number, number, number];

test("Manifold independently measures partial rotated-box sets, holes and islands", () => {
  for (const row of fixture.cases) {
    const stock = Manifold.cube(fixture.stock as Vec3);
    const unplaced = Manifold.cube(row.tool as Vec3);
    const rotated = unplaced.rotate([0, 0, row.angleDegrees]);
    const tool = rotated.translate(row.translation as Vec3);
    try {
      const results = { difference: stock.subtract(tool), union: stock.add(tool), intersection: stock.intersect(tool) };
      for (const operation of ["difference", "union", "intersection"] as const) {
        const solid = results[operation];
        const expected = row.operations[operation];
        const components = solid.decompose();
        try {
          expect(solid.status()).toBe("NoError");
          expect(solid.volume()).toBeCloseTo(expected.volume, 9);
          expect(solid.surfaceArea()).toBeCloseTo(expected.area, 9);
          expect(components).toHaveLength(expected.components);
          expect(components.reduce((sum, component) => sum + component.genus(), 0)).toBe(expected.holes);
          if ("pieces" in expected) {
            const pieces = components.map(component => [component.volume(), component.surfaceArea()]).sort(([a], [b]) => a! - b!);
            for (const [i, piece] of expected.pieces.entries()) {
              expect(pieces[i]?.[0]).toBeCloseTo(piece[0]!, 9);
              expect(pieces[i]?.[1]).toBeCloseTo(piece[1]!, 9);
            }
          }
        } finally {
          for (const component of components) component.delete();
          solid.delete();
        }
      }
    } finally {
      tool.delete();
      rotated.delete();
      unplaced.delete();
      stock.delete();
    }
  }
});

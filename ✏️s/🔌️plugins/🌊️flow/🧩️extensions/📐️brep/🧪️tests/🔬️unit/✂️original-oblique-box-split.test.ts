import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import fixture from "./🔣️original-oblique-box-split.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vec3 = [number, number, number];

test("Manifold independently splits placed boxes in both authored normal orderings", () => {
  for (const row of fixture.cases) {
    const theta = (row.rotationDegrees * Math.PI) / 180;
    const c = Math.cos(theta), s = Math.sin(theta);
    const rotate = ([x, y, z]: Vec3): Vec3 => [c * x - s * y, s * x + c * y, z];
    const normal = rotate(row.normal as Vec3);
    const plane = rotate(row.point as Vec3).map((coordinate, i) => coordinate + row.origin[i]!);
    const unit = normal.map(value => value / Math.hypot(...normal)) as Vec3;
    const local = Manifold.cube(row.size as Vec3);
    const rotated = local.rotate([0, 0, row.rotationDegrees]);
    const stock = rotated.translate(row.origin as Vec3);
    try {
      for (const sign of [1, -1]) {
        const offset = plane.reduce((sum, value, i) => sum + value * unit[i]!, 0);
        const results = stock.splitByPlane(unit.map(v => sign * v) as Vec3, sign * offset);
        try {
          let conserved = 0;
          for (const [i, half] of results.entries()) {
            const expected = row.halves[sign > 0 ? i : 1 - i]!;
            expect(half.status()).toBe("NoError");
            expect(half.genus()).toBe(0);
            expect(half.volume()).toBeCloseTo(expected.volume, 9);
            expect(half.surfaceArea()).toBeCloseTo(expected.area, 9);
            conserved += half.volume();
            const bounds = half.boundingBox();
            for (const [side, limits] of [bounds.min, bounds.max].entries()) {
              for (let axis = 0; axis < 3; axis++) expect(limits[axis]).toBeCloseTo(expected.bounds[side]![axis]!, 9);
            }
          }
          expect(conserved).toBeCloseTo(stock.volume(), 9);
        } finally {
          for (const half of results) half.delete();
        }
      }
    } finally {
      stock.delete();
      rotated.delete();
      local.delete();
    }
  }
});

test("Manifold gives zero volume on one side of tangent and external planes", () => {
  const stock = Manifold.cube(fixture.cases[0]!.size as Vec3);
  try {
    for (const row of fixture.refused) {
      const length = Math.hypot(...row.normal);
      if (length === 0) continue;
      const normal = row.normal.map(value => value / length) as Vec3;
      const offset = row.point.reduce((sum, value, i) => sum + value * normal[i]!, 0);
      const results = stock.splitByPlane(normal, offset);
      try {
        const volumes = results.map(shape => shape.volume()).sort((a, b) => a - b);
        expect(volumes[0]).toBeCloseTo(0, 10);
        expect(volumes[1]).toBeCloseTo(stock.volume(), 10);
      } finally {
        for (const half of results) half.delete();
      }
    }
  } finally {
    stock.delete();
  }
});

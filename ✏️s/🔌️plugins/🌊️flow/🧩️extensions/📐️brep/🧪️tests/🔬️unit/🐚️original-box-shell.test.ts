import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import fixture from "./🔣️original-box-shell.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vec3 = [number, number, number];

for (const row of fixture.cases) {
  test(`Manifold independently verifies ${row.name}`, () => {
    const local = Manifold.cube(fixture.size as Vec3);
    const stock = local.translate(fixture.origin as Vec3);
    const minimum = fixture.origin.map((value, axis) => {
      const inset = row.open.includes(`-${"xyz"[axis]}`) ? -fixture.thickness : fixture.thickness;
      return value + inset;
    }) as Vec3;
    const dimensions = fixture.size.map((size, axis) => {
      const inset = row.open.includes(`+${"xyz"[axis]}`) ? -fixture.thickness : fixture.thickness;
      return size + fixture.origin[axis] - minimum[axis] - inset;
    }) as Vec3;
    const unit = Manifold.cube(dimensions);
    const cavity = unit.translate(minimum);
    const shell = stock.subtract(cavity);
    expect(shell.volume()).toBeCloseTo(row.volume, 12);
    expect(2 - 2 * shell.genus()).toEqual(row.euler);
    expect(shell.boundingBox()).toEqual(stock.boundingBox());
    const center = fixture.origin.map((low, axis) => low + fixture.size[axis] / 2) as Vec3;
    expect(shell.rayCast(center, [20.13, 18.91, 26.237]).length % 2).toBe(0);
    for (const item of [shell, cavity, unit, stock, local]) item.delete();
  });
}

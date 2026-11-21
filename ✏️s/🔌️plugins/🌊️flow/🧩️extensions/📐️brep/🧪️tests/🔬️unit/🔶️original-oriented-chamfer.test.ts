import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import { Matrix4, Vector3 } from "three";
import fixture from "./🔣️original-oriented-chamfer.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vector = [number, number, number];
type Matrix = Parameters<ReturnType<typeof Manifold.cube>["transform"]>[0];

for (const row of fixture.cases) {
  test(`Manifold clips the chamfer plane along ${row.name}`, () => {
    const { size, origin, rotateY } = fixture.stock;
    const u = (row.axis + 1) % 3;
    const v = (row.axis + 2) % 3;
    const [corner] = row.edges[0];
    const stock = Manifold.cube([size[u], size[v], size[row.axis]]);
    const inward = [corner[u] === 0 ? 1 : -1, corner[v] === 0 ? 1 : -1];
    const normal: Vector = [inward[0] * Math.SQRT1_2, inward[1] * Math.SQRT1_2, 0];
    const offset = corner[u] * normal[0] + corner[v] * normal[1] + fixture.distance * Math.SQRT1_2;
    const [beveled, scrap] = stock.splitByPlane(normal, offset);
    const basis = [0, 1, 2].map(i => new Vector3().setComponent(i, 1));
    const axial = beveled.transform(new Matrix4().makeBasis(basis[u], basis[v], basis[row.axis]).toArray() as Matrix);
    const tilted = axial.rotate([0, rotateY * 180 / Math.PI, 0]);
    const pitched = tilted.rotate([(row.rotateX ?? 0) * 180 / Math.PI, 0, 0]);
    const reflected = pitched.transform(new Matrix4().makeScale(row.mirror ? -1 : 1, 1, 1).toArray() as Matrix);
    const placed = reflected.translate(origin as Vector);
    expect(placed.volume()).toBeCloseTo(row.volume, 9);
    expect(placed.surfaceArea()).toBeCloseTo(row.area, 9);
    expect(placed.genus()).toBe(0);
    expect(beveled.volume() + scrap.volume()).toBeCloseTo(stock.volume(), 9);
    for (const item of [placed, reflected, pitched, tilted, axial, beveled, scrap, stock]) item.delete();
  });
}

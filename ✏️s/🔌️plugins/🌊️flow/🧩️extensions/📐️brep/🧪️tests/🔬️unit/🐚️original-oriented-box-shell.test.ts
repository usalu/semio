import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import { Matrix4, Triangle, Vector3 } from "three";
import fixture from "./🔣️original-oriented-box-shell.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vector = [number, number, number];
type Matrix = Parameters<ReturnType<typeof Manifold.cube>["transform"]>[0];

for (const row of fixture.cases) {
  test(`Manifold shells the authored frame with ${row.name}`, () => {
    const { size, origin, rotateY } = fixture.stock;
    const { thickness } = fixture;
    const low = size.map((_, i) => row.open.includes(`-${"xyz"[i]}`) ? -thickness : thickness);
    const high = size.map((n, i) => n + (row.open.includes(`+${"xyz"[i]}`) ? thickness : -thickness));
    const stock = Manifold.cube(size as Vector);
    const tool = Manifold.cube(high.map((value, i) => value - low[i]) as Vector);
    const cavity = tool.translate(low as Vector);
    const shell = stock.subtract(cavity);
    const tilted = shell.rotate([0, rotateY * 180 / Math.PI, 0]);
    const pitched = tilted.rotate([row.rotateX * 180 / Math.PI, 0, 0]);
    const reflected = pitched.transform(new Matrix4().makeScale(row.mirror ? -1 : 1, 1, 1).toArray() as Matrix);
    const placed = reflected.translate(origin as Vector);
    expect(placed.volume()).toBeCloseTo(row.volume, 9);
    expect(placed.surfaceArea()).toBeCloseTo(row.area, 9);
    expect(2 - 2 * placed.genus()).toBe(row.euler);
    const box = placed.boundingBox();
    for (const [side, values] of [box.min, box.max].entries()) {
      for (const [axis, value] of values.entries()) expect(value).toBeCloseTo(row.bounds[side][axis], 9);
    }
    const { vertProperties, triVerts, numProp } = placed.getMesh();
    const triangle = new Triangle();
    const nearest = new Vector3();
    for (const [point, expected] of row.classify) {
      const query = new Vector3().fromArray(point as Vector);
      query.applyAxisAngle(new Vector3(0, 1, 0), rotateY).applyAxisAngle(new Vector3(1, 0, 0), row.rotateX);
      if (row.mirror) query.x = -query.x;
      query.add(new Vector3().fromArray(origin));
      let boundary = false;
      for (let i = 0; i < triVerts.length; i += 3) {
        triangle.a.fromArray(vertProperties, triVerts[i]! * numProp);
        triangle.b.fromArray(vertProperties, triVerts[i + 1]! * numProp);
        triangle.c.fromArray(vertProperties, triVerts[i + 2]! * numProp);
        if (triangle.closestPointToPoint(query, nearest).distanceTo(query) < 1e-6) {
          boundary = true;
          break;
        }
      }
      const classification = boundary ? 2 : placed.rayCast(query.toArray(), [39, -27, 52]).length % 2 === 1 ? 0 : 1;
      expect(classification).toBe(expected as number);
    }
    for (const item of [placed, reflected, pitched, tilted, shell, cavity, tool, stock]) item.delete();
  });
}

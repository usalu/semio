import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import { Triangle, Vector3 } from "three";
import fixture from "./🔣️original-cylinder-shell.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vec3 = [number, number, number];

for (const row of fixture.cases) {
  test(`Manifold independently verifies the cylindrical Shell with ${row.name}`, () => {
    const { origin, radius, rotateY } = fixture.stock;
    const { thickness } = fixture;
    const stock = Manifold.cylinder(row.height, radius, radius, 512);
    const bottom = row.open.includes("bottom") ? -thickness : thickness;
    const top = row.open.includes("top") ? row.height + thickness : row.height - thickness;
    const inset = radius - thickness;
    const tool = Manifold.cylinder(top - bottom, inset, inset, 512);
    const cavity = tool.translate([0, 0, bottom]);
    const result = stock.subtract(cavity);
    const tilt = result.rotate([0, rotateY * 180 / Math.PI, 0]);
    const placed = tilt.translate(origin as Vec3);
    const exterior = stock.rotate([0, rotateY * 180 / Math.PI, 0]);
    const placedStock = exterior.translate(origin as Vec3);
    expect(placed.volume()).toBeCloseTo(row.volume, 2);
    expect(placed.surfaceArea()).toBeCloseTo(row.area, 2);
    expect(2 - 2 * placed.genus()).toBe(row.euler);
    const box = placed.boundingBox();
    const stockBounds = placedStock.boundingBox();
    for (const key of ["min", "max"] as const) {
      for (let axis = 0; axis < 3; axis++) expect(box[key][axis]).toBeCloseTo(stockBounds[key][axis], 8);
    }
    const { vertProperties, triVerts, numProp } = placed.getMesh();
    const triangle = new Triangle();
    const nearest = new Vector3();
    for (const [coords, expected] of row.classify) {
      const query = new Vector3().fromArray(coords as Vec3)
        .applyAxisAngle(new Vector3(0, 1, 0), rotateY).add(new Vector3().fromArray(origin));
      let boundary = false;
      if (expected === 2) {
        for (let i = 0; i < triVerts.length; i += 3) {
          triangle.a.fromArray(vertProperties, triVerts[i]! * numProp);
          triangle.b.fromArray(vertProperties, triVerts[i + 1]! * numProp);
          triangle.c.fromArray(vertProperties, triVerts[i + 2]! * numProp);
          if (triangle.closestPointToPoint(query, nearest).distanceTo(query) < 2e-6) {
            boundary = true;
            break;
          }
        }
      }
      const classification = boundary ? 2 : placed.rayCast(query.toArray(), [50, 41, 27]).length % 2 === 1 ? 0 : 1;
      expect(classification).toBe(expected as number);
    }
    for (const item of [placedStock, exterior, placed, tilt, result, cavity, tool, stock]) item.delete();
  });
}

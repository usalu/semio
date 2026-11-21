import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import { Vector3 } from "three";
import cylinder from "./🔣️original-cylinder-split.json";
import cone from "./🔣️original-cone-split.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vec3 = [number, number, number];

for (const [name, fixture, primitive, tapered] of [
  ["cylinder", cylinder, cylinder.cylinder, false],
  ["cone", cone, cone.cone, true],
] as const) test(`Manifold returns both ordered halves of the placed ${name}`, () => {
  const { radius, height, origin, angleY } = primitive;
  const canonical = Manifold.cylinder(height, radius, tapered ? 0 : radius, 1024);
  const rotated = canonical.rotate([0, angleY * 180 / Math.PI, 0]);
  const stock = rotated.translate(origin as Vec3);
  try {
    for (const row of fixture.cases) {
      const normal = new Vector3().fromArray(row.normal).normalize();
      const distance = normal.dot(new Vector3().fromArray(row.origin));
      const [positive, negative] = stock.splitByPlane(normal.toArray(), distance);
      try {
        for (const [solid, expected] of [[positive, row.positive], [negative, row.negative]] as const) {
          const { min, max } = solid.boundingBox();
          for (const [actual, bounds] of [[min, expected.bounds[0]], [max, expected.bounds[1]]] as const) {
            for (const [axis, coordinate] of actual.entries()) {
              expect(coordinate, row.name).toBeCloseTo(bounds[axis]!, 4);
            }
          }
          expect(solid.volume() / expected.volume, row.name).toBeCloseTo(1, 4);
          expect(solid.genus(), row.name).toBe(0);
        }
        expect(positive.volume() + negative.volume()).toBeCloseTo(stock.volume(), 8);
      } finally {
        positive.delete();
        negative.delete();
      }
    }
    for (const row of fixture.refused.filter(({ reason }) => reason === "tangent" || reason === "external")) {
      const normal = new Vector3().fromArray(row.normal).normalize();
      const [positive, negative] = stock.splitByPlane(normal.toArray(), normal.dot(new Vector3().fromArray(row.origin)));
      try {
        expect(Math.min(positive.volume(), negative.volume())).toBeCloseTo(0, 8);
        expect(positive.volume() + negative.volume()).toBeCloseTo(stock.volume(), 8);
      } finally {
        positive.delete();
        negative.delete();
      }
    }
  } finally {
    stock.delete();
    rotated.delete();
    canonical.delete();
  }
});

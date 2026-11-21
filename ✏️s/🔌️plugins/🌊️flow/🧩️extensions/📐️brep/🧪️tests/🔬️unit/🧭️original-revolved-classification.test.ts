import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import { Triangle, Vector3 } from "three";
import fixture from "./🔣️original-revolved-classification.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold, CrossSection } = wasm;
type Vec3 = [number, number, number];
const segments = 128;

function make(row: (typeof fixture.cases)[number]) {
  const [small, large] = row.radii as [number, number];
  if (row.kind === "frustum") return Manifold.cylinder(row.height!, small, large, segments);
  if (row.kind === "torus" || row.kind === "annular-sector") {
    const torus = row.kind === "torus";
    const profile = torus ? CrossSection.circle(large, segments) : CrossSection.square([large - small, row.height!]);
    const placed = profile.translate([small, 0]);
    const result = placed.revolve(segments, row.rotationDegrees);
    placed.delete();
    profile.delete();
    return result;
  }
  const [lo, hi] = row.span as [number, number];
  if (row.kind === "bore") {
    const stock = Manifold.cylinder(row.height!, large, large, segments);
    const narrow = Manifold.cylinder(hi - lo, small, small, segments);
    const placed = narrow.translate([row.offset!, 0, lo]);
    const result = stock.subtract(placed);
    placed.delete();
    narrow.delete();
    stock.delete();
    return result;
  }
  const broad = Manifold.cylinder(hi - lo, large, large, segments);
  const placed = broad.translate([0, 0, lo]);
  const tall = Manifold.cylinder(row.height!, small, small, segments);
  const collar = tall.translate([row.offset!, 0, 0]);
  const result = placed.add(collar);
  collar.delete();
  tall.delete();
  placed.delete();
  broad.delete();
  return result;
}

test("Manifold boundaries independently classify finite revolutions and eccentric shoulders", () => {
  const triangle = new Triangle();
  const target = new Vector3(), nearest = new Vector3();
  for (const row of fixture.cases) {
    const shape = make(row);
    try {
      expect(shape.status()).toBe("NoError");
      const { triVerts, vertProperties, numProp } = shape.getMesh();
      for (const [point, expected] of row.probes as [Vec3, number][]) {
        target.fromArray(point);
        let squared = Infinity;
        for (let i = 0; i < triVerts.length; i += 3) {
          triangle.a.fromArray(vertProperties, numProp * triVerts[i]!);
          triangle.b.fromArray(vertProperties, numProp * triVerts[i + 1]!);
          triangle.c.fromArray(vertProperties, numProp * triVerts[i + 2]!);
          triangle.closestPointToPoint(target, nearest);
          squared = Math.min(squared, target.distanceToSquared(nearest));
        }
        const hits = shape.rayCast(point, [63.27, 91.18, 57.43]);
        const measured = Math.sqrt(squared) * (hits.length % 2 === 0 ? 1 : -1);
        expect(Math.abs(measured - expected)).toBeLessThan(0.003);
      }
    } finally {
      shape.delete();
    }
  }
});

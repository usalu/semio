import { expect, test } from "bun:test";
import { Line3, Vector3 } from "three";
import loadManifold from "manifold-3d";
import fixture from "./🔣️original-straight-sweeps.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vec3 = [number, number, number];

for (const row of fixture.cases) {
  test(`Manifold reproduces the closed straight ${row.guide ? "guided Pipe" : row.kind + " sweep"}`, () => {
    const displacement = row.path[1].map((value, axis) => value - row.path[0][axis]) as Vec3;
    const intermediates = [];
    let swept;
    if (row.kind === "circle") {
      const cylinder = Manifold.cylinder(Math.hypot(...displacement), row.radius!, row.radius!, 1024);
      const angle = -Math.atan2(row.normal![1], row.normal![2]) * 180 / Math.PI;
      const tilted = cylinder.rotate([angle, 0, 0]);
      const bottom = row.center!.map((value, axis) => value + displacement[axis]) as Vec3;
      swept = tilted.translate(bottom);
      intermediates.push(cylinder, tilted);
    } else {
      const [width, depth] = row.size!;
      const original = [[0, 0, 0], [width, 0, 0], [width, depth, 0], [0, depth, 0]] as Vec3[];
      const points = row.guide ? row.path.flatMap(station => {
        const origin = new Vector3().fromArray(station);
        const rail = new Line3(...row.guide!.map(point => new Vector3().fromArray(point)) as [Vector3, Vector3]);
        const z = new Vector3().fromArray(displacement).normalize();
        const x = rail.closestPointToPoint(origin, true, new Vector3()).sub(origin).projectOnPlane(z).normalize();
        const y = z.clone().cross(x);
        const [base, u, v] = row.profileFrame!.map(point => new Vector3().fromArray(point));
        return original.map(point => {
          const local = new Vector3().fromArray(point).sub(base);
          return origin.clone().addScaledVector(x, local.dot(u)).addScaledVector(y, local.dot(v)).toArray();
        });
      }) : [...original, ...original.map(point => point.map((value, axis) => value + displacement[axis]) as Vec3)];
      swept = Manifold.hull(points);
    }
    const bounds = swept.boundingBox();
    for (let axis = 0; axis < 3; axis++) {
      expect(bounds.min[axis]).toBeCloseTo(row.bounds[0][axis], 5);
      expect(bounds.max[axis]).toBeCloseTo(row.bounds[1][axis], 5);
    }
    expect(Math.abs(swept.volume() / row.volume - 1)).toBeLessThan(0.00001);
    expect(swept.genus()).toBe(0);
    for (const item of [swept, ...intermediates]) item.delete();
  });
}

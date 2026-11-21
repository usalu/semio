import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import { Line3, Plane, Vector3 } from "three";
import fixture from "./🔣️original-axial-solid-offset.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vec3 = [number, number, number];

function clippedSupports(kind: string, distance: number, cut: number) {
  const { radius, height } = fixture.stock;
  const slope = kind === "cylinder" ? 0 : -radius / height;
  const tangent = new Vector3(slope, 0, 1);
  const outward = tangent.clone().cross(new Vector3(0, 1, 0)).negate().normalize();
  const rim = new Vector3(radius, 0, 0).addScaledVector(outward, distance);
  const line = new Line3(rim.clone().addScaledVector(tangent, -32), rim.clone().addScaledVector(tangent, 32));
  const bottom = new Plane(new Vector3(0, 0, 1), distance);
  const top = kind === "cone"
    ? new Plane(new Vector3(1, 0, 0), 0)
    : new Plane(new Vector3(0, 0, 1), -(kind === "frustum" ? cut : height) - distance);
  return [bottom, top].map(plane => {
    const intersection = plane.intersectLine(line, new Vector3());
    if (!intersection) throw new Error("The displaced conic support exceeds the oracle segment");
    return intersection;
  });
}

for (const row of fixture.cases) {
  test(`Manifold measures the ${row.kind} with signed support offset ${row.distance}`, () => {
    const [bottom, top] = clippedSupports(row.kind, row.distance, fixture.stock.cut);
    const degrees = fixture.stock.rotateY * 180 / Math.PI;
    expect(bottom.z).toBeCloseTo(row.levels[0], 10);
    expect(top.z).toBeCloseTo(row.levels[1], 10);
    expect(bottom.x).toBeCloseTo(row.radii[0], 10);
    expect(top.x).toBeCloseTo(row.radii[1], 10);
    const cut = Manifold.cylinder(top.z - bottom.z, bottom.x, Math.max(0, top.x), 1024);
    const local = cut.translate([0, 0, bottom.z]);
    const tilted = local.rotate([0, degrees, 0]);
    const shape = tilted.translate(fixture.stock.origin as Vec3);
    expect(shape.volume()).toBeCloseTo(row.volume, 2);
    expect(shape.surfaceArea()).toBeCloseTo(row.area, 2);
    expect(shape.genus()).toBe(0);
    const box = shape.boundingBox();
    for (const [index, bounds] of [box.min, box.max].entries()) {
      for (const [axis, limit] of bounds.entries()) expect(limit).toBeCloseTo(row.bounds[index][axis], 3);
    }
    for (const item of [shape, tilted, local, cut]) item.delete();
  });
}

for (const row of fixture.refused.filter(row => row.distance !== 0)) {
  test(`Three verifies collapsed ${row.kind} supports at offset ${row.distance}`, () => {
    const [bottom, top] = clippedSupports(row.kind, row.distance, row.cut ?? fixture.stock.cut);
    expect(top.z <= bottom.z || bottom.x <= 0 || (row.kind !== "cone" && top.x <= 0)).toBe(true);
  });
}

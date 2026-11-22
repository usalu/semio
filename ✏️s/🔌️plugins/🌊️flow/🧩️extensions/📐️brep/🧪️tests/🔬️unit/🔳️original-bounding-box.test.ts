import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import fixture from "./🔣️original-bounding-box.json";
import revolve from "./🔣️original-revolve.json";

const wasm = await loadManifold();
wasm.setup();
const { CrossSection, Manifold } = wasm;
type Vec3 = [number, number, number];

function checkBounds(bounds: { min: Vec3; max: Vec3 }, minimum: number[], maximum: number[]) {
  for (let axis = 0; axis < 3; axis++) {
    expect(bounds.min[axis]).toBeCloseTo(minimum[axis], 5);
    expect(bounds.max[axis]).toBeCloseTo(maximum[axis], 5);
  }
}

test("Manifold verifies the world bounds of a tilted cylinder and translated circle", () => {
  const row = fixture.cylinder;
  const angle = -Math.atan2(row.normal[1], row.normal[2]);
  const height = Math.hypot(...row.vector);
  const original = Manifold.cylinder(height, row.radius, row.radius, 1024);
  const rotated = original.rotate([angle * 180 / Math.PI, 0, 0]);
  const moved = rotated.translate(row.center as Vec3);
  checkBounds(moved.boundingBox(), row.bounds[0], row.bounds[1]);

  const profile = CrossSection.circle(row.radius, 1024);
  const coordinates = profile.toPolygons()[0].map(([x, y]) => {
    const placed = [x, y * Math.cos(angle), y * Math.sin(angle)];
    return placed.map((value, axis) => value + row.center[axis] + fixture.curve.offset[axis]);
  });
  const columns = [0, 1, 2].map(axis => coordinates.map(point => point[axis]));
  checkBounds(
    { min: columns.map(values => Math.min(...values)) as Vec3, max: columns.map(values => Math.max(...values)) as Vec3 },
    fixture.curve.bounds[0],
    fixture.curve.bounds[1],
  );
  for (const item of [profile, moved, rotated, original]) item.delete();
});

test("Manifold verifies the placed torus world bounds", () => {
  const { expected } = revolve;
  const circle = CrossSection.circle(expected.minorRadius, 256);
  const profile = circle.translate([expected.majorRadius, 0]);
  const torus = profile.revolve(384);
  const rotated = torus.rotate([-90, 0, 0]);
  const moved = rotated.translate(expected.center as Vec3);
  checkBounds(moved.boundingBox(), expected.minimum, expected.maximum);
  for (const item of [moved, rotated, torus, profile, circle]) item.delete();
});


test("Manifold checks exact annular supports and trimmed sector bounds", () => {
  for (const row of fixture.annular) {
    if (row.operator === "brep.sweep.revolve") {
      const rectangle = CrossSection.square([row.outer! - row.inner!, row.height!]);
      const profile = rectangle.translate([row.inner!, 0]);
      const sweep = row.angle!;
      const swept = profile.revolve(1536, Math.abs(sweep) * 180 / Math.PI);
      const oriented = sweep < 0 ? swept.mirror([0, 1, 0]) : swept;
      const placed = oriented.translate(row.origin as Vec3);
      try {
        checkBounds(placed.boundingBox(), row.bounds[0], row.bounds[1]);
      } finally {
        if (oriented !== swept) oriented.delete();
        for (const value of [placed, swept, profile, rectangle]) value.delete();
      }
    } else {
      const cylinder = (spec: { radius: number; height: number; origin: number[] }) => {
        const raw = Manifold.cylinder(spec.height, spec.radius, spec.radius, 1536);
        const translated = raw.translate(spec.origin as Vec3);
        raw.delete();
        return translated;
      };
      const a = cylinder(row.a!);
      const b = cylinder(row.b!);
      const material = row.operator === "brep.bool.cut" ? a.subtract(b) : a.add(b);
      const rotated = material.rotate([0, (row.rotation ?? 0) * 180 / Math.PI, 0]);
      const placed = rotated.translate((row.offset ?? [0, 0, 0]) as Vec3);
      try {
        checkBounds(placed.boundingBox(), row.bounds[0], row.bounds[1]);
      } finally {
        for (const value of [placed, rotated, material, b, a]) value.delete();
      }
    }
  }
});

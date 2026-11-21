import { expect, test } from "bun:test";
import { Box3, Line3, Sphere, Vector3 } from "three";
import fixture from "./🔣️original-sphere-distance.json";
import cylinders from "./🔣️original-cylinder-distance.json";
import cavities from "./🔣️original-sphere-cavity-distance.json";

function sphere(spec: { center: number[]; radius: number }) {
  return new Sphere(new Vector3().fromArray(spec.center), spec.radius);
}

test("Three.js agrees with the exact solid separation fixtures", () => {
  for (const { name, left, right, expected } of fixture.spherePairs) {
    const a = sphere(left);
    const b = sphere(right);
    const distance = a.intersectsSphere(b) ? 0 : a.distanceToPoint(b.center) - b.radius;
    expect(distance, name).toBeCloseTo(expected, 12);
  }
  const normal = new Vector3(0, 0, 1);
  for (const { name, origin, size, rotation, sphere: spec, expected } of fixture.sphereBoxes) {
    const grid = new Box3(new Vector3(), new Vector3().fromArray(size));
    const local = sphere(spec);
    local.center.sub(new Vector3().fromArray(origin)).applyAxisAngle(normal, -rotation);
    const distance = grid.intersectsSphere(local) ? 0 : grid.distanceToPoint(local.center) - local.radius;
    expect(distance, name).toBeCloseTo(expected, 12);
  }
  for (const row of fixture.curvedSolids) {
    const probe = sphere(row.sphere);
    probe.center.sub(new Vector3().fromArray(row.origin));
    const meridian = new Vector3(Math.hypot(probe.center.x, probe.center.y), 0, probe.center.z);
    const { radius, height, major, minor } = row.parameters;
    let gap: number;
    if (row.kind === "cylinder") {
      const region = new Box3(new Vector3(), new Vector3(radius!, 0, height!));
      gap = region.distanceToPoint(meridian);
    } else if (row.kind === "cone") {
      const rim = new Vector3(radius!, 0, 0);
      const spans = [new Line3(rim, new Vector3(0, 0, height!)), new Line3(new Vector3(), rim)];
      gap = Math.min(...spans.map((span) => span.closestPointToPoint(meridian, true, new Vector3()).distanceTo(meridian)));
    } else {
      const tube = new Sphere(new Vector3(major!, 0, 0), minor!);
      gap = tube.distanceToPoint(meridian);
    }
    expect(Math.max(0, gap - probe.radius), row.name).toBeCloseTo(row.expected, 12);
  }
});

test("Three.js reproduces the cylinder axial and radial gap fixtures", () => {
  const first = cylinders.stock;
  const probe = new Vector3();
  const stockDisk = new Sphere(new Vector3().fromArray(first.center).setZ(0), first.radius);
  for (const row of cylinders.cases) {
    const [x, y, z] = row.center;
    const radial = stockDisk.distanceToPoint(probe.set(x, y, 0)) - row.radius;
    const low = row.reverse ? z - row.height : z;
    const high = row.reverse ? z : z + row.height;
    const zStock = first.center[2];
    const stockAxis = new Box3(new Vector3(0, 0, zStock), new Vector3(0, 0, zStock + first.height));
    const otherAxis = new Box3(new Vector3(0, 0, low), new Vector3(0, 0, high));
    const axial = stockAxis.intersectsBox(otherAxis) ? 0
      : Math.min(stockAxis.distanceToPoint(otherAxis.min), stockAxis.distanceToPoint(otherAxis.max));
    expect(Math.hypot(Math.max(0, radial), axial), row.name).toBeCloseTo(row.expected, 12);
  }
});

test("Three.js independently measures sphere separation from material around a cavity", () => {
  for (const shell of cavities.shells) {
    const voidSphere = sphere(shell.void);
    const origin = new Vector3().fromArray(shell.origin);
    const outerSphere = shell.radius === undefined ? null : new Sphere(origin, shell.radius);
    const outerBox = shell.size === undefined ? null : new Box3(origin, new Vector3().fromArray(shell.size).add(origin));
    for (const row of shell.cases) {
      const probe = sphere(row);
      const exterior = outerSphere?.distanceToPoint(probe.center) ?? outerBox!.distanceToPoint(probe.center);
      const interior = -voidSphere.distanceToPoint(probe.center);
      expect(Math.max(0, exterior - probe.radius, interior - probe.radius), row.name).toBeCloseTo(row.expected, 12);
    }
  }
});

test("Three.js agrees with cylinder separation from placed rectangular stock", () => {
  const axis = new Vector3(0, 0, 1);
  for (const row of cylinders.boxes) {
    const origin = new Vector3().fromArray(row.box.origin);
    const center = new Vector3().fromArray(row.cylinder.center).sub(origin).applyAxisAngle(axis, -row.box.rotation);
    const [x, y, z] = row.box.size;
    const section = new Box3(new Vector3(), new Vector3(x, y, 0));
    const axial = Math.max(center.z - z, -center.z - row.cylinder.height, 0);
    const radial = Math.max(section.distanceToPoint(center.setZ(0)) - row.cylinder.radius, 0);
    expect(Math.hypot(radial, axial), row.name).toBeCloseTo(row.expected, 12);
  }
});

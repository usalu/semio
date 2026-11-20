import { expect, test } from "bun:test";
import { Line3, Plane, Vector3 } from "three";
import fixture from "./🔣️line-intersections.json";

const vector = (xyz: number[]) => new Vector3().fromArray(xyz);
const line = ({ start, end }: { start: number[]; end: number[] }) => new Line3(vector(start), vector(end));
const toleranceError = "tolerance must be positive and finite";
const noIntersection = "no intersection";
const infinite = "infinitely many intersections";

function check(actual: Vector3 | string, row: { hit?: number[]; refusal?: string }) {
  if (typeof actual === "string") {
    expect(actual).toBe(row.refusal);
    return;
  }
  expect(actual.distanceTo(vector(row.hit!))).toBeLessThan(1e-12);
}

for (const row of fixture.curveCurve) {
  test(row.id, () => {
    const tol = row.tolerance ?? fixture.tolerance;
    const a = line(row.a);
    const b = line(row.b);
    const da = a.delta(new Vector3());
    const db = b.delta(new Vector3());
    const cross = da.clone().cross(db);
    const delta = b.start.clone().sub(a.start);
    if (tol <= 0) return check(toleranceError, row);
    if (cross.length() <= tol * da.length() * db.length()) {
      return check(delta.clone().cross(da).length() <= tol * da.length() ? infinite : noIntersection, row);
    }
    const scale = cross.lengthSq();
    const pa = a.at(delta.clone().cross(db).dot(cross) / scale, new Vector3());
    const pb = b.at(delta.clone().cross(da).dot(cross) / scale, new Vector3());
    check(pa.distanceTo(pb) <= tol ? pa.lerp(pb, 0.5) : noIntersection, row);
  });
}

for (const row of fixture.curveSurface) {
  test(row.id, () => {
    const tol = row.tolerance ?? fixture.tolerance;
    const a = line(row.line);
    const dir = a.delta(new Vector3());
    const normal = vector(row.plane.normal).normalize();
    const plane = new Plane().setFromNormalAndCoplanarPoint(normal, vector(row.plane.origin));
    if (tol <= 0) return check(toleranceError, row);
    const distance = plane.distanceToPoint(a.start);
    const speed = normal.dot(dir);
    if (Math.abs(speed) <= tol * dir.length()) {
      return check(Math.abs(distance) <= tol ? infinite : noIntersection, row);
    }
    check(a.at(-distance / speed, new Vector3()), row);
  });
}

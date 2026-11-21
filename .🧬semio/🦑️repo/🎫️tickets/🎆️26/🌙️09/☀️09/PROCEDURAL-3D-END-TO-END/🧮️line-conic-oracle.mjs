import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { Line3, Matrix4, Plane, Ray, Sphere, Vector3 } from "three";

const fixture = JSON.parse(readFileSync("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️line-conic-intersections.json", "utf8"));
const { tolerance } = fixture;
const vector = (xyz) => new Vector3(...xyz);
const anglePeriod = 2 * Math.PI;
const unitSphere = new Sphere(new Vector3(), 1);

function contacts(test) {
  const line = new Line3(vector(test.line.start), vector(test.line.end));
  const direction = line.delta(new Vector3()).normalize();
  const center = vector(test.conic.center);
  const normal = vector(test.conic.normal);
  const major = vector(test.conic.axis);
  const minor = normal.clone().cross(major);
  const [rx, ry] = test.conic.radii;
  const shapeFrame = new Matrix4().makeBasis(major.multiplyScalar(rx), minor.multiplyScalar(ry), normal).setPosition(center);
  const localFrame = shapeFrame.clone().invert();
  const local = (world) => world.clone().applyMatrix4(localFrame);
  const world = (coordinates) => coordinates.clone().applyMatrix4(shapeFrame);
  const plane = new Plane().setFromNormalAndCoplanarPoint(normal, center);
  let candidates = [];
  if (Math.abs(direction.dot(normal)) > 1e-14) {
    for (const sign of [-1, 1]) {
      const onPlane = new Ray(line.start, direction.clone().multiplyScalar(sign)).intersectPlane(plane, new Vector3());
      if (!onPlane) continue;
      const point = local(onPlane);
      if (Math.abs(point.lengthSq() - 1) <= tolerance) candidates.push(point);
    }
  } else if (Math.abs(plane.distanceToPoint(line.start)) <= tolerance) {
    const origin = local(line.start).setZ(0);
    const axis = local(line.end).setZ(0).sub(origin).normalize();
    const near = origin.clone().addScaledVector(axis, -origin.dot(axis));
    for (const sign of [-1, 1]) {
      const ray = new Ray(near, axis.clone().multiplyScalar(sign));
      const pass = ray.intersectSphere(unitSphere, new Vector3());
      if (pass) candidates.push(pass);
    }
  }

  const selected = [];
  for (const point of candidates) {
    const angle = Math.atan2(point.y, point.x);
    if (test.conicRange) {
      const [low, high] = test.conicRange;
      const turns = Math.ceil((low - angle - tolerance) / anglePeriod);
      if (angle + turns * anglePeriod > high + tolerance) continue;
    }
    const candidate = world(point);
    const t = line.closestPointToPointParameter(candidate, false);
    if (test.lineRange && (t < test.lineRange[0] - tolerance || t > test.lineRange[1] + tolerance)) continue;
    if (!selected.some(({ candidate: known }) => known.distanceTo(candidate) <= tolerance)) selected.push({ t, candidate });
  }
  return selected.sort((a, b) => a.t - b.t).map(({ candidate }) => candidate);
}

for (const test of fixture.cases) {
  const actual = contacts(test);
  assert.equal(actual.length, test.points.length, test.id);
  for (let i = 0; i < actual.length; i++) assert.ok(actual[i].distanceTo(vector(test.points[i])) <= tolerance, test.id);
}

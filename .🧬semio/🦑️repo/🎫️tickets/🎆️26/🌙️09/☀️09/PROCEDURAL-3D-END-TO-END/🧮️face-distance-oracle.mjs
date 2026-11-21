import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { EllipseCurve, Line3, Plane, Ray, ShapeUtils, Sphere, Triangle, Vector2, Vector3 } from "three";

const vector = (values) => new Vector3(...values);
const root = "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/";
const planar = JSON.parse(readFileSync(root + "🔣️planar-face-projection.json", "utf8"));
const surfaces = JSON.parse(readFileSync(root + "🔣️surface-vertex-distance.json", "utf8"));
const tau = 2 * Math.PI;

function region(rings, origin, u, v) {
  const rows = rings.map((ring) => ring.map((point) => {
    const p = point.clone().sub(origin);
    return new Vector2(p.dot(u), p.dot(v));
  }));
  const faces = ShapeUtils.triangulateShape(rows[0], rows.slice(1));
  const positions = rings.flat();
  return faces.map(([a, b, c]) => new Triangle(positions[a], positions[b], positions[c]));
}

function closest(triangles, target) {
  let best;
  for (const triangle of triangles) {
    const point = triangle.closestPointToPoint(target, new Vector3());
    if (!best || point.distanceToSquared(target) < best.distanceToSquared(target)) best = point;
  }
  return best;
}

const normal = vector(planar.frame.normal).normalize();
const u = vector(planar.frame.xAxis).normalize();
const v = normal.clone().cross(u).normalize();
const triangles = region([planar.outer, ...planar.holes].map((ring) => ring.map(vector)), vector(planar.frame.origin), u, v);
assert.ok(Math.abs(triangles.reduce((sum, triangle) => sum + triangle.getArea(), 0) - planar.area) < 1e-10);
for (const row of planar.cases) {
  const target = vector(row.target);
  const point = closest(triangles, target);
  assert.ok(point.distanceTo(vector(row.point)) < 1e-10, row.name);
  assert.ok(Math.abs(target.distanceTo(point) - row.distance) < 1e-10, row.name);
}

function surfacePoint(row) {
  const origin = vector(row.origin);
  const target = vector(row.target);
  const z = vector(surfaces.normal);
  const x = vector([1, 0, 0]);
  const y = z.clone().cross(x).normalize();
  const axis = new Line3(origin.clone().addScaledVector(z, -64), origin.clone().addScaledVector(z, 64));
  const foot = axis.closestPointToPoint(target, true, new Vector3());
  const radial = target.clone().sub(foot).normalize();
  const targetInPlane = new Vector3(target.x, target.y, origin.z);
  switch (row.kind) {
    case "plane": return new Plane().setFromNormalAndCoplanarPoint(z, origin).projectPoint(target, new Vector3());
    case "cylinder": return new Sphere(foot, row.radius).clampPoint(target, new Vector3());
    case "sphere": return new Ray(origin, target.clone().sub(origin).normalize()).intersectSphere(new Sphere(origin, row.radius), new Vector3());
    case "cone": {
      const end = origin.clone().addScaledVector(z, 64).addScaledVector(radial, 64 * Math.tan(row.halfAngle));
      return new Line3(origin, end).closestPointToPoint(target, true, new Vector3());
    }
    case "torus": {
      const theta = Math.atan2(targetInPlane.y - origin.y, targetInPlane.x - origin.x);
      const center = new EllipseCurve(0, 0, row.major, row.major, 0, tau).getPoint(theta / tau);
      return new Sphere(origin.clone().addScaledVector(x, center.x).addScaledVector(y, center.y), row.minor)
        .clampPoint(target, new Vector3());
    }
    case "ellipse-face":
    case "circle-face": {
      const curve = new EllipseCurve(0, 0, row.minor ?? row.radius, row.major ?? row.radius, 0, tau);
      const ring = curve.getPoints(256).slice(0, -1).map((p) => origin.clone().addScaledVector(x, p.x).addScaledVector(y, p.y));
      return closest(region([ring], origin, x, y), target);
    }
    default: throw new Error(row.kind);
  }
}

for (const row of surfaces.cases) {
  const point = surfacePoint(row);
  assert.ok(point.distanceTo(vector(row.point)) < 1e-10, row.id);
  assert.ok(Math.abs(point.distanceTo(vector(row.target)) - row.distance) < 1e-10, row.id);
}

for (const row of planar.linearDistance) {
  const origin = vector(row.origin);
  const direction = vector(row.direction);
  const [lo, hi] = row.range ?? [-1024, 1024];
  let best = Infinity;
  for (const triangle of triangles) {
    const objective = (t) => {
      const point = origin.clone().addScaledVector(direction, t);
      return point.distanceToSquared(triangle.closestPointToPoint(point, new Vector3()));
    };
    let [left, right] = [lo, hi];
    for (let i = 0; i < 144; i++) {
      const a = left + (right-left)/3;
      const b = right - (right-left)/3;
      if (a === left || b === right || a >= b) break;
      if (objective(a) < objective(b)) right = b;
      else left = a;
    }
    best = Math.min(best, objective(lo), objective(hi), objective((left+right)/2));
  }
  assert.ok(Math.abs(Math.sqrt(best) - row.distance) < 1e-10, row.id + ": " + Math.sqrt(best));
}

import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { Matrix4, Plane, Ray, Sphere, Vector3 } from "three";

const fixture = JSON.parse(readFileSync("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️circle-pair-intersections.json", "utf8"));
const { tolerance } = fixture;
const spatial = JSON.parse(readFileSync("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️spatial-circle-intersections.json", "utf8"));
const spatialConics = JSON.parse(readFileSync("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️spatial-conic-intersections.json", "utf8"));
const coplanarConics = JSON.parse(readFileSync("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️coplanar-conic-intersections.json", "utf8"));
const unitSphere = new Sphere(new Vector3(), 1);
const turn = 2 * Math.PI;
const vector = (xyz) => new Vector3(...xyz);
const angleTolerance = 1e-12;
const normalize = (angle) => ((angle % turn) + turn) % turn;

function conic(raw) {
  const center = vector(raw.center);
  const n = vector(raw.normal).normalize();
  const x = vector(raw.axis).normalize();
  const y = n.clone().cross(x).normalize();
  const [rx, ry] = raw.radii ?? [raw.radius, raw.radius];
  const frame = new Matrix4().makeBasis(x.clone().multiplyScalar(rx), y.clone().multiplyScalar(ry), n).setPosition(center);
  const inverse = frame.clone().invert();
  const local = (point) => point.clone().applyMatrix4(inverse);
  const world = (point) => point.clone().applyMatrix4(frame);
  const at = (t) => world(new Vector3(Math.cos(t), Math.sin(t), 0));
  const velocity = (t) => x.clone().multiplyScalar(-rx * Math.sin(t)).addScaledVector(y, ry * Math.cos(t));
  const angle = (point) => {
    const p = local(point);
    return normalize(Math.atan2(p.y, p.x));
  };
  const tangent = (point) => {
    const p = local(point);
    const gradient = x.clone().multiplyScalar(p.x / rx).addScaledVector(y, p.y / ry);
    return n.clone().cross(gradient).normalize();
  };
  function parameter(point) {
    const t = angle(point);
    if (!raw.range) return t;
    const [lo, hi] = raw.range;
    const shifted = t + turn * Math.ceil((lo - t - angleTolerance) / turn);
    return shifted <= hi + angleTolerance ? Math.max(shifted, lo) : null;
  }
  return { center, n, at, velocity, angle, parameter, tangent, local, world, inverse, radius: raw.radius, range: raw.range ?? [0, turn] };
}

function portContacts(a, b, points) {
  const selected = [];
  for (const point of points) {
    const parameter = a.parameter(point);
    if (parameter === null || b.parameter(point) === null) continue;
    if (!selected.some((known) => Math.abs(Math.sin((known.parameter - parameter) / 2)) <= angleTolerance && known.value.distanceTo(point) <= tolerance)) selected.push({ parameter, value: point });
  }
  return selected.sort((first, second) => first.parameter - second.parameter).map(({ value }) => value);
}

function coincident(a, b) {
  const breakpoints = [0, turn, ...a.range.map(normalize), ...b.range.map((theta) => a.angle(b.at(theta)))].sort((x,y) => x - y);
  for (let i = 1; i < breakpoints.length; i++) {
    const low = breakpoints[i - 1];
    const high = breakpoints[i];
    if (high - low <= angleTolerance) continue;
    const sample = a.at((low + high) * 0.5);
    if (a.parameter(sample) !== null && b.parameter(sample) !== null) return { fault: "infinitely many intersections" };
  }
  return { points: portContacts(a, b, breakpoints.map(a.at)) };
}

function spatialContacts(a, b, direction) {
  const secondPlane = new Plane().setFromNormalAndCoplanarPoint(b.n, b.center);
  const lateral = direction.clone().cross(a.n).normalize();
  const offset = -secondPlane.distanceToPoint(a.center) / secondPlane.normal.dot(lateral);
  const origin = a.center.clone().addScaledVector(lateral, offset);
  const axis = direction.clone().transformDirection(a.inverse);
  const normalizedOrigin = a.local(origin);
  const closest = normalizedOrigin.clone().addScaledVector(axis, -normalizedOrigin.dot(axis));
  const candidates = [-1, 1].map((sign) => new Ray(closest, axis.clone().multiplyScalar(sign)).intersectSphere(unitSphere, new Vector3()));
  const points = candidates.filter(Boolean).map(a.world).filter((point) => Math.abs(secondPlane.distanceToPoint(point)) <= tolerance
    && b.at(b.angle(point)).distanceTo(point) <= tolerance);
  return { points: portContacts(a, b, points) };
}

function separateRoot(fn, lo, hi) {
  let sign = Math.sign(fn(lo));
  for (let i = 0; i < 70; i++) {
    const middle = (lo + hi) / 2;
    if (middle === lo || middle === hi) break;
    const split = Math.sign(fn(middle));
    if (split === 0) return middle;
    if (split === sign) lo = middle;
    else hi = middle;
  }
  return (lo + hi) / 2;
}

function ellipticContacts(a, b) {
  if (Array.from({ length: 8 }, (_, index) => a.at(index * turn / 8))
      .every((point) => b.at(b.angle(point)).distanceTo(point) <= 32 * Number.EPSILON * Math.max(1, point.distanceTo(a.center)))) {
    return coincident(a, b);
  }
  const derivative = (t) => {
    const point = b.local(a.at(t));
    const velocity = b.local(b.center.clone().add(a.velocity(t)));
    return point.x * velocity.x + point.y * velocity.y;
  };
  const residual = (t) => {
    const point = b.local(a.at(t));
    return point.x * point.x + point.y * point.y - 1;
  };
  const extrema = [0];
  let previous = derivative(0);
  for (let i = 1; i < 8192; i++) {
    const t = i * turn / 8192;
    const next = derivative(t);
    if (next === 0) extrema.push(t);
    else if (previous * next < 0) extrema.push(separateRoot(derivative, (i - 1) * turn / 8192, t));
    previous = next;
  }
  if (previous * derivative(turn) < 0) extrema.push(separateRoot(derivative, turn * (8191 / 8192), turn));
  extrema.push(turn);
  const values = extrema.map((t) => {
    const value = residual(t);
    return Math.abs(value) <= 16 * Number.EPSILON ? 0 : value;
  });
  const angles = [];
  for (let index = 0; index < extrema.length; index++) {
    if (values[index] === 0) angles.push(extrema[index]);
    if (index + 1 < extrema.length && values[index] * values[index + 1] < 0) {
      angles.push(separateRoot(residual, extrema[index], extrema[index + 1]));
    }
  }
  return { points: portContacts(a, b, angles.map(a.at)) };
}

function geometry(row) {
  const a = conic(row.a);
  const b = conic(row.b);
  const line = a.n.clone().cross(b.n);
  if (line.length() > angleTolerance) return spatialContacts(a, b, line.normalize());
  const plane = new Plane().setFromNormalAndCoplanarPoint(a.n, a.center);
  if (Math.abs(plane.distanceToPoint(b.center)) > tolerance) return { points: [] };
  if (a.radius === undefined || b.radius === undefined) return ellipticContacts(a, b);
  const delta = b.center.clone().sub(a.center);
  const distance = delta.length();
  if (distance === 0) return Math.abs(a.radius - b.radius) <= tolerance ? coincident(a, b) : { points: [] };
  const direction = delta.normalize();
  const sliceOffset = (a.radius ** 2 - b.radius ** 2 + distance ** 2) / (2 * distance);
  const onSlice = a.center.clone().addScaledVector(direction, sliceOffset);
  const across = a.n.clone().cross(direction).normalize();
  const sphere = new Sphere(a.center, a.radius);
  const raw = [-1, 1].map((sign) => new Ray(onSlice, across.clone().multiplyScalar(sign)).intersectSphere(sphere, new Vector3()));
  const points = raw.filter((point) => point && Math.abs(point.distanceTo(b.center) - b.radius) <= tolerance);
  return { points: portContacts(a, b, points) };
}

for (const row of [...fixture.cases, ...spatial.cases, ...spatialConics.cases, ...coplanarConics.cases]) {
  const actual = geometry(row);
  assert.equal(actual.fault, row.fault, row.id);
  if (actual.fault) continue;
  assert.equal(actual.points.length, row.points.length, row.id);
  const a = conic(row.a);
  const b = conic(row.b);
  for (let i = 0; i < row.points.length; i++) {
    const point = actual.points[i];
    assert.ok(point.distanceTo(vector(row.points[i])) <= (row.pointTolerance ?? tolerance), row.id);
    const tangentA = a.tangent(point);
    const tangentB = b.tangent(point);
    assert.equal(tangentA.cross(tangentB).length() <= angleTolerance, row.tangencies?.[i] ?? row.tangent ?? false, row.id);
  }
}

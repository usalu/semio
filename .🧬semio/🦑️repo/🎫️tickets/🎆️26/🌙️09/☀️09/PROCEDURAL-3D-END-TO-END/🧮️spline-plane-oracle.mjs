import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { EllipseCurve, LineCurve3, Plane, Vector3, Vector4 } from "three";
import { NURBSCurve } from "three/addons/curves/NURBSCurve.js";
import { calcNURBSDerivatives } from "three/addons/curves/NURBSUtils.js";

const fixture = JSON.parse(readFileSync("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️spline-plane-intersections.json", "utf8"));
const vector = (tuple) => new Vector3(...tuple);

function characterize(row) {
  const raw = row.curve;
  if (raw.line) {
    const curve = new LineCurve3(...raw.line.map(vector));
    return [row.range, (t) => curve.getPoint(t), () => curve.getTangent(0)];
  }
  if (raw.circle) {
    const turn = 2 * Math.PI;
    const curve = new EllipseCurve(0, 0, raw.circle, raw.circle, 0, turn);
    const at = (t) => vector([...curve.getPoint(t / turn), 0]);
    return [row.range, at, (t) => vector([-at(t).y, at(t).x, 0])];
  }
  const end = raw.knots.length - raw.degree - 1;
  const curve = new NURBSCurve(raw.degree, raw.knots, raw.controls.map(([x, y, z], i) => new Vector4(x, y, z, raw.weights[i])), raw.degree, end);
  const start = raw.knots[raw.degree];
  const width = raw.knots[end] - start;
  const derivative = (t) => calcNURBSDerivatives(raw.degree, raw.knots, curve.controlPoints, t, 1)[1];
  return [raw.knots.slice(raw.degree, end + 1), (t) => curve.getPoint((t - start) / width), derivative];
}

function bisect(fn, lo, hi, a) {
  for (let step = 0; step < 64; step++) {
    const middle = lo + (hi - lo) / 2;
    if (middle === lo || middle === hi) return middle;
    const value = fn(middle);
    if (value === 0) return middle;
    if (Math.sign(a) === Math.sign(value)) lo = middle;
    else hi = middle;
  }
  return lo + (hi - lo) / 2;
}

function study(row) {
  const [knots, pointAt, derivative] = characterize(row);
  const specification = row.plane ?? fixture.plane;
  const plane = new Plane().setFromNormalAndCoplanarPoint(vector(specification.normal).normalize(), vector(specification.origin));
  const signed = (t) => plane.distanceToPoint(pointAt(t));
  const directional = (t) => plane.normal.dot(derivative(t));
  const [lo, hi] = row.range ?? [knots[0], knots.at(-1)];
  const limits = [lo, ...new Set(knots.filter((k) => lo < k && k < hi)), hi];
  const roots = [];
  const append = (t) => {
    if (roots.every((other) => Math.abs(t - other) > 1e-11)) roots.push(t);
  };
  for (let j = 1; j < limits.length; j++) {
    const [left, right] = limits.slice(j - 1, j + 1);
    const coordinates = Array.from({ length: 513 }, (_, i) => i === 512 ? right : left + (right - left) * i / 512);
    const samples = coordinates.map(signed);
    const eps = 16 * Number.EPSILON * Math.max(1, ...coordinates.map((t) => pointAt(t).length()));
    if (samples.every((sample) => Math.abs(sample) <= eps)) return { continuum: true };
    const slopes = coordinates.map(directional);
    const cuts = [left];
    for (let i = 1; i < slopes.length; i++) {
      if (i + 1 < slopes.length && slopes[i] === 0) cuts.push(coordinates[i]);
      if (slopes[i - 1] * slopes[i] < 0) cuts.push(bisect(directional, coordinates[i - 1], coordinates[i], slopes[i - 1]));
    }
    cuts.push(right);
    const signs = cuts.map((t) => {
      const distance = signed(t);
      return Math.abs(distance) <= eps ? 0 : distance;
    });
    for (let i = 0; i < cuts.length; i++) {
      if (signs[i] === 0) append(cuts[i]);
      if (i + 1 < cuts.length && signs[i] * signs[i + 1] < 0) append(bisect(signed, cuts[i], cuts[i + 1], signs[i]));
    }
  }
  roots.sort((a, b) => a - b);
  return { roots, points: roots.map(pointAt), plane };
}

for (const row of fixture.cases) {
  const result = study(row);
  if (row.fault === "infinitely many intersections") {
    assert.ok(result.continuum, row.id);
  } else if (row.fault?.includes("more than four")) {
    assert.equal(result.roots.length, 5, row.id);
  } else if (row.fault?.includes("requires a supported")) {
    const { controls, degree, knots } = row.curve;
    const periodic = knots[0] !== knots[1] && knots.at(-1) !== knots.at(-2);
    assert.ok(degree > 3 || controls.length > 32 || periodic, row.id);
    assert.ok(result.roots.length > 0, row.id);
  } else if (row.fault?.includes("positive finite weights")) {
    assert.ok(row.curve.weights.some((weight) => weight <= 0), row.id);
  } else {
    assert.equal(result.roots.length, row.parameters.length, row.id);
    assert.ok(result.roots.every((t, i) => Math.abs(t - row.parameters[i]) < 1e-9), row.id);
    for (const [i, point] of result.points.entries()) {
      assert.ok(point.distanceTo(vector(row.points[i])) < fixture.tolerance, row.id);
      assert.ok(Math.abs(result.plane.distanceToPoint(point)) < fixture.tolerance, row.id);
    }
  }
}

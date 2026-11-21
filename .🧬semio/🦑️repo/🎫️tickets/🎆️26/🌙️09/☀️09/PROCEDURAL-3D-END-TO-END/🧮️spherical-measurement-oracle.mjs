import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { Quaternion, Ray, Sphere, SphereGeometry, Triangle, Vector3 } from "three";

const data = JSON.parse(readFileSync("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️spherical-measurement.json", "utf8"));

function chartArea(radius, lower, upper) {
  const start = Math.acos(Math.min(1, Math.max(-1, upper / radius)));
  const end = Math.acos(Math.min(1, Math.max(-1, lower / radius)));
  const geometry = new SphereGeometry(radius, 256, 128, 0, 2 * Math.PI, start, end - start);
  const position = geometry.getAttribute("position");
  const indices = geometry.getIndex();
  const triangle = new Triangle();
  let area = 0;
  for (let i = 0; i < indices.count; i += 3) {
    triangle.a.fromBufferAttribute(position, indices.getX(i));
    triangle.b.fromBufferAttribute(position, indices.getX(i + 1));
    triangle.c.fromBufferAttribute(position, indices.getX(i + 2));
    area += triangle.getArea();
  }
  geometry.dispose();
  return area;
}

function oracle(row) {
  const shapes = row.centers.map((origin, i) => new Sphere(new Vector3(...origin), row.radii[i]));
  const [left, right] = shapes;
  const axis = right.center.clone().sub(left.center);
  const distance = axis.length();
  if (distance === 0) axis.set(1, 0, 0);
  else axis.multiplyScalar(1 / distance);
  const offsets = [0, distance];
  const radii = shapes.map((s) => s.radius);
  const [r, s] = radii;
  const limits = new Set([-r, r, distance - s, distance + s]);
  if (distance > 0) {
    const crossing = (distance + (r - s) * (r + s) / distance) / 2;
    if (Math.max(-r, distance - s) < crossing && crossing < Math.min(r, distance + s)) limits.add(crossing);
  }
  const cuts = [...limits].sort((a, b) => a - b);
  const disk = (i, x) => {
    const offset = Math.abs(x - offsets[i]);
    return Math.max(0, (radii[i] - offset) * (radii[i] + offset));
  };
  const slice = (x) => {
    const a = disk(0, x);
    const b = disk(1, x);
    if (row.kind === "fuse") return Math.max(a, b);
    if (row.kind === "intersect") return Math.min(a, b);
    if (a > 0 && b > 0) return Math.max(0, (r - s) * (r + s) - distance * (2 * x - distance));
    return Math.max(0, a - b);
  };

  const areas = [0, 0];
  const charts = [[], []];
  let volume = 0;
  let moment = 0;
  for (let i = 1; i < cuts.length; i++) {
    const [lo, hi] = [cuts[i - 1], cuts[i]];
    const m = (lo + hi) * 0.5;
    const d = (hi - lo) * 0.5;
    const a = disk(0, m);
    const b = disk(1, m);
    const active = row.kind === "fuse" ? [a >= b && a > 0, b > a]
      : row.kind === "intersect" ? [a > 0 && b > 0 && a <= b, a > 0 && b > 0 && b < a]
      : [a > b, a > b && b > 0];
    for (let j = 0; j < 2; j++) {
      if (!active[j]) continue;
      const bounds = [lo - offsets[j], hi - offsets[j]];
      charts[j].push(bounds);
      areas[j] += chartArea(radii[j], ...bounds);
    }
    for (const [abscissa, weight] of [[-Math.sqrt(3 / 5), 5 / 9], [0, 8 / 9], [Math.sqrt(3 / 5), 5 / 9]]) {
      const x = m + d * abscissa;
      const sliceVolume = slice(x) * Math.PI * d * weight;
      volume += sliceVolume;
      moment += sliceVolume * x;
    }
  }
  const center = left.center.clone().addScaledVector(axis, moment / volume);
  return { area: areas[0] + areas[1], areas, volume, center, shapes, charts, axis };
}

function checkNearest(row, { shapes, charts, axis }, probe) {
  const shape = shapes[probe.face];
  const bounds = charts[probe.face];
  const lo = bounds[0][0];
  const hi = bounds.at(-1)[1];
  const target = new Vector3(...probe.target);
  const delta = target.clone().sub(shape.center);
  const exact = delta.lengthSq() > 0
    ? new Ray(shape.center, delta.normalize()).intersectSphere(shape, new Vector3())
    : null;
  if (exact) {
    const axial = exact.clone().sub(shape.center).dot(axis);
    if (axial >= lo && axial <= hi) {
      assert.ok(exact.distanceTo(new Vector3(...probe.point)) < data.relativeTolerance, row.id + " radial nearest");
      assert.ok(Math.abs(exact.distanceTo(target) - probe.distance) < data.relativeTolerance, row.id + " radial distance");
      return;
    }
  }
  const start = Math.acos(Math.min(1, hi / shape.radius));
  const end = Math.acos(Math.max(-1, lo / shape.radius));
  const geometry = new SphereGeometry(shape.radius, 512, 128, 0, 2 * Math.PI, start, end - start);
  const rotation = new Quaternion().setFromUnitVectors(new Vector3(0, 1, 0), axis);
  const position = geometry.getAttribute("position");
  const indices = geometry.getIndex();
  const triangle = new Triangle();
  const candidate = new Vector3();
  const best = new Vector3();
  let distance = Infinity;
  for (let i = 0; i < indices.count; i += 3) {
    for (const [row, point] of [triangle.a, triangle.b, triangle.c].entries()) {
      point.fromBufferAttribute(position, indices.getX(i + row)).applyQuaternion(rotation).add(shape.center);
    }
    triangle.closestPointToPoint(target, candidate);
    const separation = candidate.distanceTo(target);
    if (separation < distance) {
      distance = separation;
      best.copy(candidate);
    }
  }
  geometry.dispose();
  assert.ok(Math.abs(distance - probe.distance) < 3e-4 * shape.radius, row.id + " triangulated distance");
  if (probe.point) assert.ok(best.distanceTo(new Vector3(...probe.point)) < 0.01 * shape.radius, row.id + " triangulated rim point");
}

for (const row of data.cases) {
  const result = oracle(row);
  assert.ok(Math.abs(result.volume - row.volume) < data.relativeTolerance * Math.max(1, row.volume), row.id + " volume");
  assert.ok(result.center.distanceTo(new Vector3(...row.center)) < data.relativeTolerance, row.id + " centroid");
  assert.ok(Math.abs(result.area - row.area) < 2e-4 * row.area, row.id + " area");
  for (let i = 0; i < 2; i++) assert.ok(Math.abs(result.areas[i] - row.faces[i]) < 2e-4 * row.faces[i], row.id + " face " + i);
  for (const probe of row.nearest ?? []) checkNearest(row, result, probe);
}

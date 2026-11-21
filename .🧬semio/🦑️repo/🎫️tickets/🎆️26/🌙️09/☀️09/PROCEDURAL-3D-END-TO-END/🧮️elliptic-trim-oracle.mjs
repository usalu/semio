import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { EllipseCurve, Vector3 } from "three";

const fixture = JSON.parse(readFileSync("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️elliptic-trim.json", "utf8"));
const turn = 2 * Math.PI;
const vector = (coords) => new Vector3(...coords);

function oracle(row) {
  const [a, b] = row.curve.ellipse;
  const origin = vector(row.curve.origin ?? [0, 0, 0]);
  const x = vector([1, 0, 0]);
  const y = vector(row.curve.normal ?? [0, 0, 1]).cross(x).normalize();
  const target = vector(row.target);
  const [lo, hi] = row.range;
  const ellipse = new EllipseCurve(0, 0, a, b, 0, turn);
  const pointAt = (t) => {
    const point = ellipse.getPoint(t / turn);
    return origin.clone().addScaledVector(x, point.x).addScaledVector(y, point.y);
  };
  const direction = (t) => {
    const h = 1e-5;
    return pointAt(t + h).sub(pointAt(t - h)).dot(pointAt(t).sub(target));
  };
  const candidates = [lo, hi];
  let previous = lo;
  let sign = direction(lo);
  for (let i = 1; i <= 16_384; i++) {
    const t = lo + (hi - lo) * i / 16_384;
    const next = direction(t);
    if (next === 0) candidates.push(t);
    if (sign * next < 0) {
      let left = previous;
      let right = t;
      for (let k = 0; k < 56; k++) {
        const mid = left + (right - left) / 2;
        if (mid === left || mid === right) break;
        const result = direction(mid);
        if (result === 0) { left = right = mid; break; }
        if (sign * result < 0) right = mid;
        else left = mid;
      }
      candidates.push(left + (right - left) / 2);
    }
    previous = t;
    sign = next;
  }
  let best;
  for (const t of candidates.sort((a, b) => a - b)) {
    const point = pointAt(t);
    const distance = point.distanceTo(target);
    if (!best || distance + 16 * Number.EPSILON * Math.max(1, distance) < best.distance) {
      best = { t, point, distance };
    }
  }
  const pieces = Math.ceil((hi - lo) / (Math.PI / 2));
  let length = 0;
  for (let i = 0; i < pieces; i++) {
    const start = lo + (hi - lo) * i / pieces;
    const end = lo + (hi - lo) * (i + 1) / pieces;
    const arc = new EllipseCurve(0, 0, a, b, start, end);
    length += arc.getLengths(32768).at(-1);
  }
  return { ...best, length };
}

for (const row of fixture.cases) {
  const result = oracle(row);
  assert.ok(Math.abs(result.t - row.parameter) < 1e-8, row.id);
  assert.ok(result.point.distanceTo(vector(row.point)) < 1e-6, row.id);
  assert.ok(Math.abs(result.distance - row.distance) < 1e-8, row.id);
  assert.ok(Math.abs(result.length - row.length) < 1e-9 * Math.max(1, row.length), row.id);
}

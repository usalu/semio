import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { Vector3, Vector4 } from "three";
import { NURBSCurve } from "three/addons/curves/NURBSCurve.js";
import { calcNURBSDerivatives } from "three/addons/curves/NURBSUtils.js";

const fixture = JSON.parse(readFileSync("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️spline-nearest.json", "utf8"));

function closest(row) {
  const { degree, knots, controls, weights } = row.curve;
  const last = knots.length - degree - 1;
  const weightScale = Math.max(...weights);
  const curve = new NURBSCurve(degree, knots, controls.map(([x, y, z], i) => new Vector4(x, y, z, weights[i] / weightScale)), degree, last);
  const start = knots[degree];
  const width = knots[last] - start;
  const target = new Vector3(...row.target);
  const [lo, hi] = row.range ?? [start, knots[last]];
  const limits = [lo, ...new Set(knots.filter((k) => lo < k && k < hi)), hi];
  const raw = (t) => calcNURBSDerivatives(degree, knots, curve.controlPoints, t, 1);
  const slope = (t) => {
    const [point, tangent] = raw(t);
    return point.sub(target).dot(tangent);
  };
  const pointAt = (t) => curve.getPoint((t - start) / width);
  const candidates = [...limits];
  for (let index = 1; index < limits.length; index++) {
    const left = limits[index - 1];
    const right = limits[index];
    let last = left;
    let prior = slope(left);
    const samples = 65_536;
    for (let sample = 1; sample <= samples; sample++) {
      const parameter = left + (right - left) * sample / samples;
      const direction = slope(parameter);
      if (direction === 0) candidates.push(parameter);
      if (direction * prior < 0) {
        let [a, b] = [last, parameter];
        for (let n = 0; n < 64; n++) {
          const mid = a + (b - a) / 2;
          const at = slope(mid);
          if (at === 0 || mid === a || mid === b) { a = b = mid; break; }
          if (prior * at < 0) b = mid;
          else a = mid;
        }
        candidates.push(a + (b - a) / 2);
      }
      last = parameter;
      prior = direction;
    }
  }
  let best;
  for (const parameter of candidates.sort((a, b) => a - b)) {
    const point = pointAt(parameter);
    const distance = point.distanceTo(target);
    if (!best || distance < best.distance - 32 * Number.EPSILON * Math.max(1, distance, best.distance)) {
      best = { parameter, point, distance };
    }
  }
  return best;
}

for (const row of fixture.cases) {
  if (row.fault) {
    const { degree, knots } = row.curve;
    assert.ok(degree > 3 || (knots[0] !== knots[1] && knots.at(-1) !== knots.at(-2)), row.id);
    continue;
  }
  const best = closest(row);
  const tolerance = fixture.absoluteTolerance;
  assert.ok(Math.abs(best.parameter - row.parameter) <= tolerance, row.id);
  assert.ok(best.point.distanceTo(new Vector3(...row.point)) <= tolerance, row.id);
  assert.ok(Math.abs(best.distance - row.distance) <= tolerance, row.id);
}

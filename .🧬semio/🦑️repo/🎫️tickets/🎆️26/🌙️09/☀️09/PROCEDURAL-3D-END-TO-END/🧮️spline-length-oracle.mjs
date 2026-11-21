import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { EllipseCurve, LineCurve3, Vector3, Vector4 } from "three";
import { NURBSCurve } from "three/addons/curves/NURBSCurve.js";
import { calcNURBSDerivatives } from "three/addons/curves/NURBSUtils.js";

const fixture = JSON.parse(readFileSync("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️spline-length.json", "utf8"));

function integrate(speed, a, b) {
  const count = 8192;
  const width = (b - a) / count;
  let total = 0;
  for (let i = 0; i < count; i++) {
    const middle = a + (i + 0.5) * width;
    const offset = width * Math.sqrt(3 / 5) / 2;
    total += 5 * speed(middle - offset) + 8 * speed(middle) + 5 * speed(middle + offset);
  }
  return total * width / 18;
}

function arcLength(raw, range) {
  if (raw.ellipse) return new EllipseCurve(0, 0, ...raw.ellipse, ...raw.range).getLengths(262144).at(-1);
  if (raw.line) return new LineCurve3(...raw.line.map((point) => new Vector3(...point))).getLength();
  const { degree, knots, controls, weights } = raw;
  const last = knots.length - degree - 1;
  const weightScale = Math.max(...weights);
  const spline = new NURBSCurve(degree, knots, controls.map(([x, y, z], i) => new Vector4(x, y, z, weights[i] / weightScale)), degree, last);
  const [lo, hi] = range ?? [knots[degree], knots[last]];
  const spans = [lo, ...new Set(knots.filter((k) => lo < k && k < hi)), hi];
  const speed = (t) => calcNURBSDerivatives(degree, knots, spline.controlPoints, t, 1)[1].length();
  let total = 0;
  for (let i = 1; i < spans.length; i++) total += integrate(speed, spans[i - 1], spans[i]);
  return total;
}

for (const row of fixture.cases) {
  if (row.fault) {
    const { knots, degree } = row.curve;
    assert.ok(degree > 3 || (knots[0] !== knots[1] && knots.at(-1) !== knots.at(-2)), row.id);
    assert.ok(Number.isFinite(arcLength(row.curve)), row.id);
    continue;
  }
  const length = row.wire
    ? [...new Set(row.members)].reduce((sum, id) => sum + arcLength(row.wire[id]), 0)
    : arcLength(row.curve, row.range);
  assert.ok(Math.abs(length - row.length) < 1e-10, row.id);
}

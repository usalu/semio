import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { Line3, Vector3 } from "three";

const fixture = JSON.parse(readFileSync("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️linear-distance.json", "utf8"));
const pointAt = (line, parameter) => new Vector3(...line.origin).addScaledVector(new Vector3(...line.direction), parameter);

function closest(a, b, extent) {
  if (!a.range && b.range) return closest(b, a, extent);
  const [lo, hi] = a.range ?? [-extent, extent];
  const axis = b.direction.reduce((best, value, axis) => Math.abs(value) > Math.abs(b.direction[best]) ? axis : best, 0);
  const direction = new Vector3(...b.direction);
  const zero = new Vector3();
  const objective = (parameter) => {
    const point = pointAt(a, parameter);
    if (b.direction[axis] === 0) return point.distanceToSquared(pointAt(b, 0));
    const anchor = (point.getComponent(axis) - b.origin[axis]) / b.direction[axis];
    const initial = pointAt(b, anchor).sub(point);
    const support = new Line3(initial, initial.clone().add(direction));
    const parameterOnLine = support.closestPointToPointParameter(zero, false);
    const trimmed = b.range ? Math.min(b.range[1] - anchor, Math.max(b.range[0] - anchor, parameterOnLine)) : parameterOnLine;
    return support.at(trimmed, new Vector3()).lengthSq();
  };
  let minimum = Math.min(objective(lo), objective(hi));
  let [left, right] = [lo, hi];
  for (let i = 0; i < 192; i++) {
    const a = left + (right - left) / 3;
    const b = right - (right - left) / 3;
    if (a === left || b === right || a >= b) break;
    const first = objective(a);
    const second = objective(b);
    minimum = Math.min(minimum, first, second);
    if (first < second) right = b;
    else left = a;
  }
  return Math.sqrt(Math.min(minimum, objective(left + (right - left) / 2)));
}

for (const row of fixture.cases) {
  for (const [a, b] of [[row.a, row.b], [row.b, row.a]]) {
    const actual = closest(a, b, row.oracleExtent ?? 1024);
    assert.ok(Math.abs(actual - row.distance) <= fixture.tolerance * Math.max(1, row.distance), row.id + ": " + actual);
  }
}

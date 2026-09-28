import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { Matrix4, Quaternion, Vector3 } from "three";
import Ajv from "ajv/dist/2020.js";
import { composeRotation, composeScale, type AxisAngle } from "../../🟦️.ts";

const fixtures = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
const validate = new Ajv().compile(JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8")));
for (const fixture of fixtures.scales) test(`per-axis scale: ${fixture.current} then ${fixture.next}`, () => {
  const actual = composeScale(fixture.current, fixture.next);
  expect(actual).toEqual(fixture.expected);
  const oracle = new Matrix4().makeScale(...fixture.next as [number, number, number]).multiply(new Matrix4().makeScale(...fixture.current as [number, number, number]));
  expect(new Vector3(1, 1, 1).applyMatrix4(oracle).toArray()).toEqual(actual);
});
test("scaling rejects collapse and overflow", () => {
  expect(() => composeScale([1, 1, 1], [0, 1, 1])).toThrow();
  expect(() => composeScale([1e200, 1, 1], [1e200, 1, 1])).toThrow();
  expect(() => composeScale([1e-200, 1, 1], [1e-200, 1, 1])).toThrow();
});
const quaternion = (rotation: AxisAngle) => {
  const scale = Math.max(...rotation.axis.map(Math.abs));
  const axis = new Vector3(...rotation.axis.map(value => value / scale) as [number, number, number]).normalize();
  return new Quaternion().setFromAxisAngle(axis, rotation.angle);
};
for (const fixture of fixtures.cases) test(`composed rotation: ${fixture.name}`, () => {
  let result: AxisAngle = { axis: [0, 0, 1], angle: 0 };
  const oracle = new Quaternion();
  for (const rotation of fixture.rotations) {
    expect(validate(rotation)).toBe(true);
    result = composeRotation(result, rotation);
    oracle.premultiply(quaternion(rotation));
    const actual = new Vector3(...fixture.point).applyQuaternion(quaternion(result));
    const reference = new Vector3(...fixture.point).applyQuaternion(oracle);
    for (let axis = 0; axis < 3; axis++) expect(actual.getComponent(axis)).toBeCloseTo(reference.getComponent(axis), 10);
  }
  const actual = new Vector3(...fixture.point).applyQuaternion(quaternion(result));
  for (let axis = 0; axis < 3; axis++) expect(actual.getComponent(axis)).toBeCloseTo(fixture.expected[axis], 10);
});
for (const invalid of fixtures.invalid) test(`invalid rotation: ${JSON.stringify(invalid)}`, () => expect(validate(invalid)).toBe(false));
test("rotation rejects invalid numeric parameters", () => {
  const identity: AxisAngle = { axis: [0, 0, 1], angle: 0 };
  for (const bad of [{ axis: [0, 0, 0], angle: 1 }, { axis: [Infinity, 0, 0], angle: 1 }, { axis: [1, 0, 0], angle: NaN }] as AxisAngle[]) {
    expect(() => composeRotation(identity, bad)).toThrow();
  }
});

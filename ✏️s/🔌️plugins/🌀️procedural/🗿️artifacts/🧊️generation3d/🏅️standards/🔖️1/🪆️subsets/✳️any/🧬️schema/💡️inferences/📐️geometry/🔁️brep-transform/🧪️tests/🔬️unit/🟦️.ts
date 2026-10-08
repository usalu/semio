import { expect, test } from "bun:test";
import { Matrix4, Plane, Vector3 } from "three";
import { casesOf, fixtureDisagreements, loadFixture, type BrepCase } from "../../../⏱️phased-job/🧰️test-support/🟦️.ts";

const fixture = loadFixture(import.meta.url);

const vector = (value: unknown): Vector3 => new Vector3(...(value as [number, number, number]));

const threeMatrix = (fixtureCase: BrepCase): Matrix4 => {
  const inputs = fixtureCase.inputs;
  switch (fixtureCase.kind) {
    case "brep.transform.translate":
      return new Matrix4().makeTranslation(vector(inputs.offset));
    case "brep.transform.rotate":
      return new Matrix4().makeRotationAxis(vector(inputs.axis).normalize(), inputs.angle as number);
    case "brep.transform.rotateAbout": {
      const origin = vector(inputs.origin);
      return new Matrix4().makeTranslation(origin).multiply(new Matrix4().makeRotationAxis(vector(inputs.axis).normalize(), inputs.angle as number)).multiply(new Matrix4().makeTranslation(origin.clone().negate()));
    }
    case "brep.transform.scale": {
      const center = vector(inputs.center);
      const factor = vector(inputs.factor);
      return new Matrix4().makeTranslation(center).multiply(new Matrix4().makeScale(factor.x, factor.y, factor.z)).multiply(new Matrix4().makeTranslation(center.clone().negate()));
    }
    case "brep.transform.mirror": {
      const plane = inputs.plane as { origin: number[]; normal: number[] };
      const normal = vector(plane.normal).normalize();
      const reflection = new Matrix4().identity();
      const e = reflection.elements;
      e[0] = 1 - 2 * normal.x * normal.x; e[4] = -2 * normal.x * normal.y; e[8] = -2 * normal.x * normal.z;
      e[1] = -2 * normal.y * normal.x; e[5] = 1 - 2 * normal.y * normal.y; e[9] = -2 * normal.y * normal.z;
      e[2] = -2 * normal.z * normal.x; e[6] = -2 * normal.z * normal.y; e[10] = 1 - 2 * normal.z * normal.z;
      const origin = new Plane().setFromNormalAndCoplanarPoint(normal, vector(plane.origin)).coplanarPoint(new Vector3());
      return new Matrix4().makeTranslation(origin).multiply(reflection).multiply(new Matrix4().makeTranslation(origin.clone().negate()));
    }
    default:
      return new Matrix4().identity();
  }
};

test("three agrees with the fixture on volume, area, bounds, closedness and the matrix that maps each pinned input onto its pinned output", () => {
  expect(fixtureDisagreements(fixture)).toEqual([]);
});

test("the stated matrix of every box motion equals the matrix three builds from the case inputs", () => {
  let checked = 0;
  for (const fixtureCase of fixture.cases) {
    const shape = fixtureCase.outputs?.shape as { matrix?: number[] } | undefined;
    if (!shape?.matrix) continue;
    const expected = threeMatrix(fixtureCase).transpose().toArray();
    shape.matrix.forEach((value, index) => expect(Math.abs(value - expected[index]!), `${fixtureCase.name}[${index}]`).toBeLessThan(1e-8));
    checked += 1;
  }
  expect(checked).toBeGreaterThanOrEqual(10);
});

test("a pattern of disjoint cubes has the volume of its copies and the bounds of its extreme copies", () => {
  const [linear] = casesOf(fixture, "brep.transform.linearPattern");
  const shape = linear!.outputs!.shape as { volume: number; bbox: number[][] };
  const spacing = linear!.inputs.spacing as number;
  const count = linear!.inputs.count as number;
  expect(shape.volume).toBe(count);
  expect(shape.bbox[1]![0]).toBe((count - 1) * spacing + 1);
});

test("every fault case names a code under the geometry prefix", () => {
  for (const fixtureCase of fixture.cases.filter(item => item.fault)) expect(fixtureCase.fault!.code.startsWith("generation3d.geometry."), fixtureCase.name).toBe(true);
});

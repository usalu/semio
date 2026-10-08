import { expect, test } from "bun:test";
import { Line3, Plane, Vector3 } from "three";
import { agree, casesOf, fixtureDisagreements, loadFixture, measure, type BrepCase } from "../../../⏱️phased-job/🧰️test-support/🟦️.ts";

const fixture = loadFixture(import.meta.url);

type Recipe = { recipe: string; start?: number[]; end?: number[]; origin?: number[]; normal?: number[] };
const vector = (value: number[]): Vector3 => new Vector3(...(value as [number, number, number]));
const lineOf = (recipe: Recipe): Line3 => new Line3(vector(recipe.start!), vector(recipe.end!));

test("three agrees with the fixture on volume, area, bounds and closedness of every pinned section face, split half and intersection wire", () => {
  expect(fixtureDisagreements(fixture)).toEqual([]);
});

test("the two halves of every split add up to the box and sit on the stated sides of the plane", () => {
  for (const fixtureCase of casesOf(fixture, "brep.intersect.split").filter(item => item.outputs)) {
    const plane = fixtureCase.inputs.plane as { origin: number[]; normal: number[] };
    const reference = new Plane().setFromNormalAndCoplanarPoint(vector(plane.normal).normalize(), vector(plane.origin));
    const sides = ["positive", "negative"].map(port => measure((fixtureCase.outputs![port] as { mesh: Parameters<typeof measure>[0] }).mesh));
    expect(agree(sides[0]!.volume + sides[1]!.volume, 8, 1e-9), fixtureCase.name).toBe(true);
    const corners = (side: ReturnType<typeof measure>) => [vector(side.min), vector(side.max)];
    for (const corner of corners(sides[0]!)) expect(reference.distanceToPoint(corner), fixtureCase.name).toBeGreaterThanOrEqual(-1e-6);
    for (const corner of corners(sides[1]!)) expect(reference.distanceToPoint(corner), fixtureCase.name).toBeLessThanOrEqual(1e-6);
  }
});

test("the stated crossing of two lines and of a line with a plane is where three computes it", () => {
  const crossing = casesOf(fixture, "brep.intersect.curveCurve").find(item => item.name === "two crossing lines meet once")!;
  const [a, b] = [lineOf(crossing.inputs.a as Recipe), lineOf(crossing.inputs.b as Recipe)];
  const mid = a.closestPointToPoint(b.at(0.5, new Vector3()), false, new Vector3());
  expect(mid.distanceTo(new Vector3(...((crossing.outputs!.points as number[][])[0] as [number, number, number])))).toBeLessThan(1e-9);
  const piercing = casesOf(fixture, "brep.intersect.curveSurface").find(item => item.name === "a line pierces a plane")!;
  const plane = piercing.inputs.surface as Recipe;
  const hit = new Plane().setFromNormalAndCoplanarPoint(vector(plane.normal!).normalize(), vector(plane.origin!)).intersectLine(lineOf(piercing.inputs.curve as Recipe), new Vector3());
  expect(hit?.distanceTo(vector((piercing.outputs!.points as number[][])[0]!))).toBeLessThan(1e-9);
});

test("the stated circle and line crossings lie on both curves", () => {
  const circleLine = casesOf(fixture, "brep.intersect.curveCurve").find(item => item.name.startsWith("a circle and a line"))! as BrepCase;
  const line = lineOf(circleLine.inputs.b as Recipe);
  for (const point of circleLine.outputs!.points as number[][]) {
    const found = vector(point);
    expect(Math.abs(found.length() - 1)).toBeLessThan(1e-9);
    expect(line.closestPointToPoint(found, false, new Vector3()).distanceTo(found)).toBeLessThan(1e-9);
  }
});

test("every fault case names a code under the geometry prefix", () => {
  for (const fixtureCase of fixture.cases.filter(item => item.fault)) expect(fixtureCase.fault!.code.startsWith("generation3d.geometry."), fixtureCase.name).toBe(true);
});

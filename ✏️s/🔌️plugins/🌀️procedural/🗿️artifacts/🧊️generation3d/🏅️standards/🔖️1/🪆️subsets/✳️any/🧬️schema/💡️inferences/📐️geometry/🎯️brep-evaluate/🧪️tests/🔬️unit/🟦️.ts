import { expect, test } from "bun:test";
import { EllipseCurve, Line3, Plane, Vector2, Vector3 } from "three";
import { casesOf, loadFixture, type BrepCase } from "../../../⏱️phased-job/🧰️test-support/🟦️.ts";

const fixture = loadFixture(import.meta.url);
const close = (actual: number[], expected: number[], tolerance = fixture.tolerance) => actual.every((value, index) => Math.abs(value - expected[index]!) <= tolerance * Math.max(1, Math.abs(expected[index]!)));

type Circle = { recipe: "circle"; radius: number };
type Line = { recipe: "line"; start: number[]; end: number[] };
const circleOf = (recipe: Circle) => new EllipseCurve(0, 0, recipe.radius, recipe.radius, 0, 2 * Math.PI, false, 0);
const lineOf = (recipe: Line) => new Line3(new Vector3(...(recipe.start as [number, number, number])), new Vector3(...(recipe.end as [number, number, number])));

test("every circle point and tangent equals three's EllipseCurve at the same angle", () => {
  let checked = 0;
  for (const kind of ["brep.evaluate.curvePoint", "brep.evaluate.curveTangent"]) {
    for (const fixtureCase of casesOf(fixture, kind).filter(item => item.outputs && (item.inputs.curve as Circle).recipe === "circle")) {
      const circle = circleOf(fixtureCase.inputs.curve as Circle);
      const angle = fixtureCase.inputs.parameter as number;
      const turn = angle / (2 * Math.PI);
      const expected = kind.endsWith("Point") ? circle.getPoint(turn) : circle.getTangent(turn);
      const found = (kind.endsWith("Point") ? fixtureCase.outputs!.point : fixtureCase.outputs!.tangent) as number[];
      expect(close(found.slice(0, 2), [expected.x, expected.y], kind.endsWith("Point") ? fixture.tolerance : 2e-3), fixtureCase.name).toBe(true);
      expect(found[2]).toBe(0);
      checked += 1;
    }
  }
  expect(checked).toBeGreaterThanOrEqual(4);
});

test("line points, tangents and closest parameters equal three's Line3", () => {
  const [pointCase] = casesOf(fixture, "brep.evaluate.curvePoint").filter(item => (item.inputs.curve as Line).recipe === "line");
  const line = lineOf(pointCase!.inputs.curve as Line);
  expect(close(line.at(pointCase!.inputs.parameter as number, new Vector3()).toArray(), pointCase!.outputs!.point as number[])).toBe(true);
  const closest = casesOf(fixture, "brep.evaluate.curveClosestParameter").find(item => (item.inputs.curve as Line).recipe === "line")!;
  const target = new Vector3(...(closest.inputs.point as [number, number, number]));
  expect(Math.abs(lineOf(closest.inputs.curve as Line).closestPointToPointParameter(target, false) - (closest.outputs!.parameter as number))).toBeLessThan(1e-9);
});

test("circle domain, curvature and closest parameter follow the closed forms", () => {
  const domain = casesOf(fixture, "brep.evaluate.curveDomain").find(item => (item.inputs.curve as Circle).recipe === "circle")!;
  expect(domain.outputs).toEqual({ start: 0, end: 2 * Math.PI, span: 2 * Math.PI });
  const curvature = casesOf(fixture, "brep.evaluate.curveCurvature").find(item => (item.inputs.curve as Circle).recipe === "circle")!;
  expect(curvature.outputs!.curvature).toBe(1 / (curvature.inputs.curve as Circle).radius);
  const closest = casesOf(fixture, "brep.evaluate.curveClosestParameter").find(item => (item.inputs.curve as Circle).recipe === "circle")!;
  const target = new Vector2(...((closest.inputs.point as number[]).slice(0, 2) as [number, number]));
  expect(Math.abs(Math.atan2(target.y, target.x) - (closest.outputs!.parameter as number))).toBeLessThan(1e-9);
  expect(Math.abs(target.length() - (closest.inputs.curve as Circle).radius - (closest.outputs!.distance as number))).toBeLessThan(1e-9);
});

test("plane normals and closest points equal three's Plane", () => {
  const plane = (fixtureCase: BrepCase) => {
    const recipe = fixtureCase.inputs.surface as { origin: number[]; normal: number[] };
    return new Plane().setFromNormalAndCoplanarPoint(new Vector3(...(recipe.normal as [number, number, number])), new Vector3(...(recipe.origin as [number, number, number])));
  };
  const normal = casesOf(fixture, "brep.evaluate.surfaceNormal").find(item => item.name === "plane normal")!;
  expect(close(plane(normal).normal.toArray(), normal.outputs!.normal as number[])).toBe(true);
  const closest = casesOf(fixture, "brep.evaluate.surfaceClosestUv").find(item => item.name === "closest parameters on a plane")!;
  const target = new Vector3(...(closest.inputs.point as [number, number, number]));
  const projected = plane(closest).projectPoint(target, new Vector3());
  expect(close(projected.toArray(), closest.outputs!.point as number[])).toBe(true);
  expect(Math.abs(Math.abs(plane(closest).distanceToPoint(target)) - (closest.outputs!.distance as number))).toBeLessThan(1e-9);
});

test("every fault case names a code under the geometry prefix", () => {
  for (const fixtureCase of fixture.cases.filter(item => item.fault)) expect(fixtureCase.fault!.code.startsWith("generation3d.geometry."), fixtureCase.name).toBe(true);
});

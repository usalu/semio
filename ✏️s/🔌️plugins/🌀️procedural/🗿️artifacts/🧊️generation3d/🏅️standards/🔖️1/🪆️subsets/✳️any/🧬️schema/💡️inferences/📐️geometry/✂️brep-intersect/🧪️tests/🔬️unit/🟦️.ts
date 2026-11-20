import { expect, test } from "bun:test";
import { Box3, Line3, Plane, Sphere, Vector3 } from "three";
import { agree, casesOf, fixtureDisagreements, loadFixture, type BrepCase, type ShapeExpectation } from "../../../⏱️phased-job/🧪️tests/🧰️support/🟦️.ts";

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
    const stock = fixtureCase.inputs.solid as { size: number[] };
    let totalVolume = 0;
    for (const [port, sign] of [["positive", 1], ["negative", -1]] as const) {
      const side = fixtureCase.outputs![port] as Required<Pick<ShapeExpectation, "bbox" | "volume">>;
      const bounds = new Box3(vector(side.bbox[0]), vector(side.bbox[1]));
      const size = bounds.getSize(new Vector3());
      expect(agree(size.x * size.y * size.z, side.volume, fixture.tolerance), fixtureCase.name).toBe(true);
      totalVolume += side.volume;
      const [min, max] = [bounds.min, bounds.max];
      for (const x of [min.x, max.x]) for (const y of [min.y, max.y]) for (const z of [min.z, max.z]) {
        expect(sign * reference.distanceToPoint(new Vector3(x, y, z)), fixtureCase.name).toBeGreaterThanOrEqual(-fixture.tolerance);
      }
    }
    expect(agree(totalVolume, stock.size.reduce((a, b) => a * b, 1), fixture.tolerance), fixtureCase.name).toBe(true);
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

test("Three reconstructs the retained plane intersection line and classifies parallel planes", () => {
  type PlaneInput = { origin: number[]; normal: number[] };
  type Case = { a: PlaneInput; b: PlaneInput; fault?: string; samples?: [number,number[]][] };
  const rows = (fixture as typeof fixture & { retainedPlaneIntersection: Case[] }).retainedPlaneIntersection;
  const plane = ({ origin, normal }: PlaneInput): Plane => new Plane().setFromNormalAndCoplanarPoint(vector(normal).normalize(), vector(origin));
  for (const row of rows) {
    const a = plane(row.a), b = plane(row.b);
    const direction = a.normal.clone().cross(b.normal);
    if (row.fault) {
      expect(direction.lengthSq()).toBe(0);
      const separation = Math.abs(a.distanceToPoint(b.coplanarPoint(new Vector3())));
      expect(separation <= fixture.tolerance).toBe(row.fault.includes("infinitely many"));
      continue;
    }
    const origin = a.projectPoint(new Vector3(), new Vector3());
    const search = b.normal.clone().addScaledVector(a.normal, -b.normal.dot(a.normal)).normalize();
    const start = origin.clone().addScaledVector(search, -100);
    const end = origin.clone().addScaledVector(search, 100);
    const hit = b.intersectLine(new Line3(start, end), new Vector3())!;
    for (const [t, expected] of row.samples!) {
      const observed = hit.clone().addScaledVector(direction.clone().normalize(), t);
      expect(observed.distanceTo(vector(expected))).toBeLessThan(fixture.tolerance);
      expect(Math.abs(a.distanceToPoint(observed))).toBeLessThan(fixture.tolerance);
      expect(Math.abs(b.distanceToPoint(observed))).toBeLessThan(fixture.tolerance);
    }
  }
});

test("Three verifies continuous sphere and perpendicular cylinder section circles", () => {
  type Row = { plane: { origin: number[]; normal: number[] }; axial: { kind: string; origin: number[]; axis: number[]; radius: number }; center?: number[]; radius?: number; fault?: string };
  for (const row of (fixture as typeof fixture & { retainedAxialIntersection: Row[] }).retainedAxialIntersection) {
    const normal = vector(row.plane.normal).normalize();
    const plane = new Plane().setFromNormalAndCoplanarPoint(normal, vector(row.plane.origin));
    const origin = vector(row.axial.origin);
    if (row.axial.kind === "sphere") {
      const sphere = new Sphere(origin, row.axial.radius);
      const distance = Math.abs(plane.distanceToPoint(origin));
      expect(sphere.intersectsPlane(plane)).toBe(row.fault !== "no intersection wire");
      if (row.fault === "tangent configuration") expect(distance).toBe(sphere.radius);
      if (!row.fault) {
        expect(plane.projectPoint(origin, new Vector3()).distanceTo(vector(row.center!))).toBeLessThan(fixture.tolerance);
        expect(Math.hypot(row.radius!, distance)).toBeCloseTo(sphere.radius);
      }
    } else {
      const axis = vector(row.axial.axis).normalize();
      if (row.fault) {
        expect(axis.cross(normal).length()).toBeGreaterThan(fixture.tolerance);
      } else {
        const center = plane.intersectLine(new Line3(origin.clone().addScaledVector(axis, -100), origin.clone().addScaledVector(axis, 100)), new Vector3())!;
        expect(center.distanceTo(vector(row.center!))).toBeLessThan(fixture.tolerance);
        expect(row.radius).toBe(row.axial.radius);
      }
    }
  }
});

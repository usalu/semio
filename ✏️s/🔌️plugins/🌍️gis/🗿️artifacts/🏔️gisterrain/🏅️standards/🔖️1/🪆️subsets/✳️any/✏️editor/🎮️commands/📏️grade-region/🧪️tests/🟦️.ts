import { expect, test } from "bun:test";
import Ajv from "ajv";
import { Line3, ShapeUtils, Triangle, Vector2, Vector3 } from "three";
import schema from "../🔣️.json";
import fixture from "./🔣️.json";

const validate = new Ajv({ multipleOfPrecision: 6 }).compile(schema);

test("grading accepts signed millimetres entered in metres", () => {
  expect(validate(fixture.command)).toBe(true);
  for (const { command, heightMillimetres } of fixture.heightLimits) {
    expect(validate(command)).toBe(true);
    expect(Math.round(command.height * 1000)).toBe(heightMillimetres);
  }
  for (const command of fixture.invalidActions) expect(validate(command)).toBe(false);
});

test("grading limits match an independently triangulated region and hole boundary", () => {
  const { bounds, columns, rows, initialHeightMillimetres } = fixture;
  const [west, south, east, north] = bounds as [number, number, number, number];
  const [outer, ...holes] = fixture.rings.map(ring => ring.map(([x, y]) => new Vector2(x, y)));
  const vertices = [outer!, ...holes].flat().map(({x, y}) => new Vector3(x, y, 0));
  const triangles = ShapeUtils.triangulateShape(outer!, holes).map(([a, b, c]) =>
    new Triangle(vertices[a]!, vertices[b]!, vertices[c]!));
  const exclusion = holes.flatMap(hole => hole.map(({x, y}, index) => {
    const next = hole[(index + 1) % hole.length]!;
    return new Line3(new Vector3(x, y, 0), new Vector3(next.x, next.y, 0));
  }));
  const heightMillimetres = Math.round(fixture.command.height * 1000);
  const closest = new Vector3();
  const sample = (column: number, row: number) => {
    const point = new Vector3(west + (east - west) * column / (columns - 1), south + (north - south) * row / (rows - 1), 0);
    return triangles.some(triangle => triangle.closestPointToPoint(point, closest).distanceToSquared(point) < Number.EPSILON)
      && !exclusion.some(line => line.closestPointToPoint(point, true, closest).distanceToSquared(point) < Number.EPSILON)
      ? heightMillimetres : initialHeightMillimetres;
  };
  let graded = 0;
  for (let row = 0; row < rows; row++) {
    for (let column = 0; column < columns; column++) {
      if (sample(column, row) !== initialHeightMillimetres) graded++;
    }
  }
  expect(graded).toBe(fixture.graded);
  for (const [column, row, expected] of fixture.samples) expect(sample(column!, row!)).toBe(expected);
});

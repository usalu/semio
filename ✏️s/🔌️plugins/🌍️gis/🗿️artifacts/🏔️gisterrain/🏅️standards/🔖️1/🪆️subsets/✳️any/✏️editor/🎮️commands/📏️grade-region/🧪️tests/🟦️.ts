import { expect, test } from "bun:test";
import Ajv from "ajv";
import { Line3, ShapeUtils, Triangle, Vector2, Vector3 } from "three";
import schema from "../🔣️.json";
import fixture from "./🔣️.json";

test("grading limits match an independently triangulated region and hole boundary", () => {
  expect(new Ajv().compile(schema)(fixture.command)).toBe(true);
  const [outer, ...holes] = fixture.rings.map(ring => ring.map(([x, y]) => new Vector2(x, y)));
  const vertices = [outer!, ...holes].flat().map(({x, y}) => new Vector3(x, y, 0));
  const triangles = ShapeUtils.triangulateShape(outer!, holes).map(([a, b, c]) =>
    new Triangle(vertices[a]!, vertices[b]!, vertices[c]!));
  const exclusion = holes.flatMap(hole => hole.map(({x, y}, index) => {
    const next = hole[(index + 1) % hole.length]!;
    return new Line3(new Vector3(x, y, 0), new Vector3(next.x, next.y, 0));
  }));
  const closest = new Vector3();
  const sample = (column: number, row: number) => {
    const point = new Vector3(column, row, 0);
    return triangles.some(triangle => triangle.containsPoint(point))
      && !exclusion.some(line => line.closestPointToPoint(point, true, closest).distanceToSquared(point) === 0)
      ? fixture.command.height : fixture.initialHeight;
  };
  let graded = 0;
  for (let row = 0; row < 33; row++) {
    for (let column = 0; column < 33; column++) {
      if (sample(column, row) !== fixture.initialHeight) graded++;
    }
  }
  expect(graded).toBe(fixture.graded);
  for (const [column, row, expected] of fixture.samples) expect(sample(column!, row!)).toBe(expected);
});

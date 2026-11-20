import { expect, test } from "bun:test";
import { ShapeUtils, Triangle, Vector2, Vector3 } from "three";
import fixture from "./🔣️original-validation-report.json";

test("Three.js agrees on the valid planar boundary and the report JSON shape", () => {
  const points = fixture.face.map(([x,y])=>new Vector2(x,y));
  const vertices = fixture.face.map(([x,y,z])=>new Vector3(x,y,z));
  const triangles = ShapeUtils.triangulateShape(points,[]);
  expect(triangles.length).toBe(vertices.length-2);
  const area = triangles.reduce((sum,[a,b,c])=>sum+new Triangle(vertices[a],vertices[b],vertices[c]).getArea(),0);
  expect(area).toBe(fixture.area);
  expect(JSON.parse(JSON.stringify(fixture.expected))).toEqual(fixture.expected);
  const failure = { ok: false, issueCount: 1, issues: [fixture.finding] };
  expect(JSON.parse(JSON.stringify(failure))).toEqual(failure);
});

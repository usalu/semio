import { expect, test } from "bun:test";
import { Matrix4, ShapeUtils, Triangle, Vector2, Vector3 } from "three";
import fixture from "./🔣️planar-face-projection.json";

const vector = (axes: number[]) => new Vector3(axes[0], axes[1], axes[2]);

test("Three.js triangulation agrees on closest points of a concave planar face with holes", () => {
  const { origin, xAxis, yAxis, normal } = fixture.frame;
  const basis = new Matrix4().makeBasis(vector(xAxis), vector(yAxis), vector(normal)).setPosition(vector(origin));
  const inverse = basis.clone().invert();
  const rings = [fixture.outer, ...fixture.holes].map(ring => ring.map(axes => {
    const { x, y } = vector(axes).applyMatrix4(inverse);
    return new Vector2(x, y);
  }));
  const triangles = ShapeUtils.triangulateShape(rings[0]!, rings.slice(1));
  const vertices = rings.flat().map(({ x, y }) => new Vector3(x, y, 0).applyMatrix4(basis));
  expect(triangles.length).toBeGreaterThan(0);
  for (const row of fixture.cases) {
    const target = vector(row.target);
    const nearest = triangles.map(([a, b, c]) => new Triangle(vertices[a!]!, vertices[b!]!, vertices[c!]!)
      .closestPointToPoint(target, new Vector3()))
      .reduce((first, second) => first.distanceToSquared(target) <= second.distanceToSquared(target) ? first : second);
    expect(nearest.distanceTo(vector(row.point)), row.name).toBeLessThan(1e-10);
  }
});

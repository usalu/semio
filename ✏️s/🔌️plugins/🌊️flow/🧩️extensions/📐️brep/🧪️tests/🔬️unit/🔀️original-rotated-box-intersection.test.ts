import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import { Matrix4, Quaternion, Triangle, Vector3 } from "three";
import fixture from "./🔣️original-rotated-box-intersection.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vec3 = [number, number, number];

function box({ size, angle, origin }: typeof fixture.cases[number]["a"]) {
  const canonical = Manifold.cube(size as Vec3);
  const rotated = canonical.rotate([0, 0, angle * 180 / Math.PI]);
  const result = rotated.translate(origin as Vec3);
  rotated.delete();
  canonical.delete();
  return result;
}

function checkClassification(solid: ReturnType<typeof Manifold.cube>, probes: (number | number[])[][]) {
  const { vertProperties, triVerts, numProp } = solid.getMesh();
  const triangle = new Triangle();
  const nearest = new Vector3();
  const query = new Vector3();
  const load = (point: Vector3, index: number) => point.fromArray(vertProperties, triVerts[index]! * numProp);
  for (const [coords, expected] of probes) {
    query.fromArray(coords as number[]);
    let boundary = false;
    for (let i = 0; i < triVerts.length; i += 3) {
      load(triangle.a, i); load(triangle.b, i + 1); load(triangle.c, i + 2);
      if (triangle.closestPointToPoint(query, nearest).distanceTo(query) <= 1e-6) {
        boundary = true;
        break;
      }
    }
    const actual = boundary ? 2 : solid.rayCast(coords as Vec3, [12.35, -10.11, 16.08]).length % 2 === 1 ? 0 : 1;
    expect(actual, String(coords)).toBe(expected as number);
  }
}

function evaluate(a: ReturnType<typeof Manifold.cube>, b: ReturnType<typeof Manifold.cube>, operator?: string) {
  if (operator === "brep.bool.fuse") return a.add(b);
  if (operator === "brep.bool.cut") return a.subtract(b);
  return a.intersect(b);
}

test("Manifold matches the retained rotated box Boolean volume, area and bounds", () => {
  for (const row of fixture.cases) {
    const a = box(row.a);
    const b = box(row.b);
    const result = evaluate(a, b, row.operator);
    try {
      expect(result.volume(), row.name).toBeCloseTo(row.volume, 9);
      expect(result.surfaceArea(), row.name).toBeCloseTo(row.area, 9);
      const bounds = result.boundingBox();
      for (const [index, actual] of [bounds.min, bounds.max].entries()) {
        for (const [axis, coordinate] of actual.entries()) expect(coordinate).toBeCloseTo(row.bounds[index]![axis]!, 9);
      }
      expect(result.genus()).toBe(0);
      if (row.classify) {
        checkClassification(result, row.classify.result);
        checkClassification(b, row.classify.operandB);
      }
    } finally {
      result.delete();
      b.delete();
      a.delete();
    }
  }
});

test("Manifold finds no volume for tangential or separated boxes", () => {
  for (const row of fixture.empty) {
    const a = box(row.a);
    const b = box(row.b);
    const result = evaluate(a, b, row.operator);
    try {
      expect(result.volume(), row.name).toBeCloseTo(0, 10);
    } finally {
      result.delete();
      b.delete();
      a.delete();
    }
  }
});

for (const row of fixture.cases) {
  for (const section of row.sections ?? []) {
    test(`Manifold independently sections the rotated intersection at the ${section.name}`, () => {
      const a = box(row.a);
      const b = box(row.b);
      const solid = a.intersect(b);
      const origin = new Vector3().fromArray(section.origin);
      const normal = new Vector3().fromArray(section.normal).normalize();
      if (section.vertices === undefined) {
        const [positive, negative] = solid.splitByPlane(normal.toArray(), origin.dot(normal));
        expect(Math.min(positive.volume(), negative.volume())).toBeCloseTo(0, 9);
        for (const item of [positive, negative, solid, a, b]) item.delete();
        return;
      }
      const rotation = new Quaternion().setFromUnitVectors(normal, new Vector3(0, 0, 1));
      const moved = solid.translate(origin.clone().negate().toArray());
      type Matrix = Parameters<typeof moved.transform>[0];
      const transformed = moved.transform(new Matrix4().makeRotationFromQuaternion(rotation).toArray() as Matrix);
      const slice = transformed.slice(0);
      const contour = slice.simplify(1e-8);
      expect(contour.area()).toBeCloseTo(section.area!, 8);
      const [polygon, surplus] = contour.toPolygons();
      expect(surplus).toBeUndefined();
      expect(polygon).toHaveLength(section.vertices);
      const inverse = rotation.invert();
      const points = polygon.map(([x, y]) => new Vector3(x, y, 0).applyQuaternion(inverse).add(origin).toArray());
      for (let i = 0; i < 3; i++) {
        expect(Math.min(...points.map(point => point[i]))).toBeCloseTo(section.bounds![0][i], 8);
        expect(Math.max(...points.map(point => point[i]))).toBeCloseTo(section.bounds![1][i], 8);
      }
      for (const item of [contour, slice, transformed, moved, solid, a, b]) item.delete();
    });
  }
}

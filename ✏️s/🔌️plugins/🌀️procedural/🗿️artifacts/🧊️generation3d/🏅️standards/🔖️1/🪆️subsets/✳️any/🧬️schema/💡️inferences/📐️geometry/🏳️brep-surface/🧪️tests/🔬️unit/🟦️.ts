import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { CylinderGeometry, LineCurve3, Matrix3, Matrix4, Plane, QuadraticBezierCurve3, ShapeUtils, Triangle, Vector2, Vector3, Vector4 } from "three";
import { NURBSCurve } from "three/addons/curves/NURBSCurve.js";
import { NURBSSurface } from "three/addons/curves/NURBSSurface.js";
import { disagreements, type FixtureCase } from "../../../🧪️tests/🧰️oracle-support/🟦️.ts";
import { oracleCase } from "../../../🧪️tests/🧰️construction-support/🟦️.ts";

type XYZ = [number, number, number];
type Fixture = {
  tolerance: number;
  cases: FixtureCase[];
  polygonTransforms: { source: XYZ[]; cases: {
    operator: string; parameters: { offset?: XYZ; origin?: XYZ; center?: XYZ; axis?: XYZ; angle?: number; factor?: XYZ; normal?: XYZ };
    vertices: XYZ[]; normal: XYZ; area: number; flipped: boolean;
  }[] };
  polygonOffset: { name: string; points: XYZ[]; normal: XYZ; distance: number; area: number; triangles: number; offsetPoints: XYZ[] }[];
  axialFacePreview: { kind: "cylinder" | "cone"; radius: number; height: number; deflection: number; area: number; edges: number; boundaryCircles: number; vertices: number }[];
  planarSplineTrim: { degree: number; knots: number[]; controls: XYZ[]; weights: number[];
    continuous: { trim: [number, number]; ends: [XYZ, XYZ]; area: number };
    clippedBeforeJump: { trim: [number, number]; ends: [XYZ, XYZ]; area: number };
    jumpControl: number; jumpDelta: XYZ;
  };
  exactSplineFace: { degree: number; knots: number[]; controls: XYZ[]; weights: number[]; trim: [number, number]; third: XYZ; ends: [XYZ, XYZ]; area: number };
  pointFacePreview: { name: string; triangles: number; inside: XYZ; notch: XYZ; notchDistance: number };
};
const fixture = JSON.parse(readFileSync(new URL("../../../../../../🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏳️brep-surface/🔣️.json", import.meta.url), "utf8")) as Fixture;
const catalogue = JSON.parse(readFileSync(new URL("../../../../../🗂️catalogue/🔣️brep-surface.json", import.meta.url), "utf8")) as { kinds: { id: string }[] };

test("the fixture states every brep.surface kind of the catalogue and no other", () => {
  expect([...new Set(fixture.cases.map(({ kind }) => kind))].sort()).toEqual(catalogue.kinds.map(({ id }) => id).sort());
});

test("every fixture case states a fault or at least one measure", () => {
  for (const { name, outputs, fault } of fixture.cases) expect(fault !== undefined || Object.keys((outputs?.shape ?? {}) as object).length > 1, name).toBe(true);
});

test("three recomputes the area, bounding box, corners, centre and plane of every surface and face case (ShapeUtils areas, Plane distances, CatmullRomCurve3 Coons boundaries)", () => {
  expect(disagreements(fixture.cases, oracleCase, fixture.tolerance)).toEqual([]);
}, { timeout: 120000 });


test("Three.js evaluates the bilinear, quadratic and cubic interpolation fixtures", () => {
  type XYZ = [number, number, number];
  for (const name of ["bilinear grid of four samples", "grid of three by three points", "bicubic grid of six by six planar samples"]) {
    const item = fixture.cases.find((row) => row.name === name)!;
    const { points, rows } = item.inputs as { points: XYZ[]; rows: number };
    const count = rows === 6 ? 2 : rows;
    const degree = count - 1;
    const step = (rows - 1) / degree;
    const knots = [...Array(count).fill(0), ...Array(count).fill(1)];
    const controls = Array.from({ length: count }, (_, i) => Array.from({ length: count }, (_, j) => {
      const [x, y, z] = points[i * step * rows + j * step]!;
      return new Vector4(x, y, rows === 3 && i === 1 && j === 1 ? 4 * z : z, 1);
    }));
    const surface = new NURBSSurface(degree, degree, knots, knots, controls);
    const middle = new Vector3();
    surface.getPoint(0.5, 0.5, middle);
    expect(middle.toArray()).toEqual((item.outputs!.shape as { center: XYZ }).center);
  }
});


test("Three.js boundary interpolation preserves both Coons patch witnesses", () => {
  type XYZ = [number, number, number];
  const curve = (samples: XYZ[]) => {
    const points = samples.map((sample) => new Vector3(...sample));
    const start = points[0]!;
    const end = points.at(-1)!;
    if (points.length === 2) return new LineCurve3(start, end);
    const middleControl = points[1]!.clone().multiplyScalar(2).addScaledVector(start, -0.5).addScaledVector(end, -0.5);
    return new QuadraticBezierCurve3(start, middleControl, end);
  };
  for (const name of ["bilinear Coons patch with mixed straight boundary degrees", "coons patch between four boundaries"]) {
    const row = fixture.cases.find((item) => item.name === name)!;
    const inputs = row.inputs as Record<string, XYZ[]>;
    const bottom = curve(inputs.bottom!);
    const right = curve(inputs.right!);
    const top = curve(inputs.top!);
    const left = curve(inputs.left!);
    const corners = [bottom.getPoint(0), bottom.getPoint(1), top.getPoint(0), top.getPoint(1)];
    const across = bottom.getPoint(0.5).lerp(top.getPoint(0.5), 0.5);
    const along = left.getPoint(0.5).lerp(right.getPoint(0.5), 0.5);
    const bilinear = corners.reduce((sum, point) => sum.addScaledVector(point, 0.25), new Vector3());
    const expected = row.outputs!.shape as { corners: XYZ[]; center: XYZ };
    expect(corners.map((point) => point.toArray())).toEqual(expected.corners);
    expect(across.add(along).sub(bilinear).distanceTo(new Vector3(...expected.center))).toBeLessThan(1e-12);
  }
});

test("Three.js triangulation of the point-list face preserves its concave notch", () => {
  const expected = fixture.pointFacePreview;
  const row = fixture.cases.find((item) => item.name === expected.name)!;
  const points = (row.inputs as { points: XYZ[] }).points;
  const triangles = ShapeUtils.triangulateShape(points.map(([x, y]) => new Vector2(x, y)), []);
  const mesh = triangles.map(([a, b, c]) => new Triangle(new Vector3(...points[a]!), new Vector3(...points[b]!), new Vector3(...points[c]!)));
  const distance = (xyz: XYZ) => {
    const point = new Vector3(...xyz);
    return Math.min(...mesh.map((triangle) => triangle.closestPointToPoint(point, new Vector3()).distanceTo(point)));
  };
  expect(mesh).toHaveLength(expected.triangles);
  expect(mesh.reduce((sum, triangle) => sum + triangle.getArea(), 0)).toBe((row.outputs!.shape as { area: number }).area);
  expect(distance(expected.inside)).toBeLessThan(fixture.tolerance);
  expect(distance(expected.notch)).toBeCloseTo(expected.notchDistance, 12);
});

test("Three.js checks translated polygon trims, area and winding", () => {
  for (const row of fixture.polygonOffset) {
    const points = row.points.map((point) => new Vector3(...point));
    const normal = new Triangle(points[0]!, points[1]!, points[2]!).getNormal(new Vector3());
    expect(normal.distanceTo(new Vector3(...row.normal))).toBeLessThan(1e-12);
    const translated = points.map((point) => point.clone().addScaledVector(normal, row.distance));
    expect(translated.map((point) => point.toArray())).toEqual(row.offsetPoints);
    const x = points[1]!.clone().sub(points[0]!).normalize();
    const y = normal.clone().cross(x);
    const triangles = ShapeUtils.triangulateShape(translated.map((point) => new Vector2(point.dot(x), point.dot(y))), []);
    expect(triangles.length).toBe(row.triangles);
    let area = 0;
    for (const [a, b, c] of triangles) {
      const triangle = new Triangle(translated[a]!, translated[b]!, translated[c]!);
      expect(triangle.getNormal(new Vector3()).dot(normal)).toBeCloseTo(1, 12);
      area += triangle.getArea();
    }
    expect(area).toBeCloseTo(row.area, 12);
  }
});

test("Three.js checks transformed vertices and inverse-transpose face normals", () => {
  const { source, cases } = fixture.polygonTransforms;
  const corners = source.map((point) => new Vector3(...point));
  const unitNormal = new Triangle(corners[0]!, corners[1]!, corners[2]!).getNormal(new Vector3());
  const about = (map: Matrix4, pivot: XYZ) => new Matrix4().makeTranslation(...pivot)
    .multiply(map).multiply(new Matrix4().makeTranslation(-pivot[0], -pivot[1], -pivot[2]));
  for (const row of cases) {
    const p = row.parameters;
    const affine = new Matrix4();
    if (p.offset) affine.makeTranslation(...p.offset);
    if (p.axis) affine.copy(about(new Matrix4().makeRotationAxis(new Vector3(...p.axis).normalize(), p.angle!), p.origin ?? [0, 0, 0]));
    if (p.factor) affine.copy(about(new Matrix4().makeScale(...p.factor), p.center!));
    if (p.normal) {
      const plane = new Plane().setFromNormalAndCoplanarPoint(new Vector3(...p.normal).normalize(), new Vector3(...p.origin!));
      const reflect = (point: Vector3) => plane.projectPoint(point, new Vector3()).multiplyScalar(2).sub(point);
      const origin = reflect(new Vector3());
      affine.makeBasis(reflect(new Vector3(1, 0, 0)).sub(origin), reflect(new Vector3(0, 1, 0)).sub(origin),
        reflect(new Vector3(0, 0, 1)).sub(origin)).setPosition(origin);
    }
    const transformed = corners.map((point) => point.clone().applyMatrix4(affine));
    transformed.forEach((point, i) => expect(point.distanceTo(new Vector3(...row.vertices[i]!)), row.operator).toBeLessThan(1e-12));
    const normal = unitNormal.clone().applyNormalMatrix(new Matrix3().getNormalMatrix(affine));
    expect(normal.distanceTo(new Vector3(...row.normal).normalize())).toBeLessThan(1e-12);
    const triangle = new Triangle(transformed[0]!, transformed[1]!, transformed[2]!);
    expect(triangle.getArea()).toBeCloseTo(row.area, 12);
    expect(triangle.getNormal(new Vector3()).dot(normal)).toBeCloseTo(row.flipped ? -1 : 1, 12);
  }
});

test("Three.js verifies the original axial trims without caps or duplicated seams", () => {
  for (const row of fixture.axialFacePreview) {
    const topRadius = row.kind === "cone" ? 0 : row.radius;
    const surface = new CylinderGeometry(topRadius, row.radius, row.height, 1024, 1, true).rotateX(Math.PI / 2).translate(0, 0, row.height / 2);
    const vertices = surface.getAttribute("position");
    const order = surface.getIndex()!.array;
    const point = (index: number) => new Vector3().fromBufferAttribute(vertices, index);
    let area = 0;
    for (let i = 0; i < order.length; i += 3) {
      area += new Triangle(point(order[i]!), point(order[i + 1]!), point(order[i + 2]!)).getArea();
    }
    const boundaryHeights = new Set(Array.from({ length: vertices.count }, (_, i) => point(i))
      .filter((point) => Math.hypot(point.x, point.y) > 1e-6).map((point) => Math.round(point.z * 1e6)));
    expect(area).toBeCloseTo(row.area, 3);
    expect(boundaryHeights.size).toBe(row.boundaryCircles);
    expect(row.edges).toBe(boundaryHeights.size + 1);
    expect(row.vertices).toBe(2);
  }
});

test("Three.js reproduces the planar trimmed spline area and repeated-knot limits", () => {
  const row = fixture.planarSplineTrim;
  const controls = row.controls.map(([x, y, z], i) => new Vector4(x, y, z, row.weights[i]!));
  const curve = new NURBSCurve(row.degree, row.knots, controls);
  const changed = controls.map((point) => point.clone());
  const moved = changed[row.jumpControl]!;
  const [dx, dy, dz] = row.jumpDelta;
  moved.set(moved.x + dx, moved.y + dy, moved.z + dz, moved.w);
  const jumping = new NURBSCurve(row.degree, row.knots, changed);
  const limits = [0.5 - 1e-9, 0.5 + 1e-9] as const;
  expect(curve.getPoint(limits[0]).distanceTo(curve.getPoint(limits[1]))).toBeLessThan(1e-7);
  expect(jumping.getPoint(limits[0]).distanceTo(jumping.getPoint(limits[1]))).toBeCloseTo(Math.hypot(...row.jumpDelta), 7);
  for (const [sampled, expected] of [[curve, row.continuous], [jumping, row.clippedBeforeJump]] as const) {
    const [start, end] = expected.trim;
    const points = Array.from({ length: 4097 }, (_, i) => sampled.getPoint(start + (end - start) * (i / 4096)));
    for (const [point, reference] of [[points[0]!, expected.ends[0]], [points.at(-1)!, expected.ends[1]]] as const) {
      expect(point.distanceTo(new Vector3(...reference))).toBeLessThan(1e-10);
    }
    const ring = [...points.map(({ x, y }) => new Vector2(x, y)), new Vector2()];
    expect(Math.abs(ShapeUtils.area(ring))).toBeCloseTo(expected.area, 7);
  }
});

test("Three preserves a trimmed rational face boundary in its exact plane coordinates", () => {
  const row = fixture.exactSplineFace;
  const domain = row.knots.at(-1)! - row.knots[0];
  const origin = new Vector3(...row.ends[0]);
  const x = new Vector3(...row.ends[1]).sub(origin).normalize();
  const y = new Vector3(-x.y, x.x, 0);
  const coordinates = (point: Vector3) => new Vector2(point.clone().sub(origin).dot(x), point.clone().sub(origin).dot(y));
  const curve = new NURBSCurve(row.degree, row.knots, row.controls.map(([x, y, z], i) => new Vector4(x, y, z, row.weights[i])));
  const pcurve = new NURBSCurve(row.degree, row.knots, row.controls.map((point, i) => {
    const uv = coordinates(new Vector3(...point));
    return new Vector4(uv.x, uv.y, 0, row.weights[i]);
  }));
  const ring = Array.from({ length: 4097 }, (_, i) => {
    const t = (row.trim[0] + (row.trim[1] - row.trim[0]) * (i / 4096) - row.knots[0]) / domain;
    const uv = coordinates(curve.getPoint(t));
    const projected = pcurve.getPoint(t);
    if (i % 1024 === 0) expect(projected.distanceTo(new Vector3(uv.x, uv.y, 0))).toBeLessThan(1e-12);
    return new Vector2(projected.x, projected.y);
  });
  ring.push(coordinates(new Vector3(...row.third)));
  expect(Math.abs(ShapeUtils.area(ring))).toBeCloseTo(row.area, 7);
});

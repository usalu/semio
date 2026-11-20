import { expect, test } from "bun:test";
import { EdgesGeometry, EllipseCurve, Line3, Matrix4, Plane, PlaneGeometry, Vector2, Vector3 } from "three";
import { casesOf, loadFixture, type BrepCase } from "../../../⏱️phased-job/🧪️tests/🧰️support/🟦️.ts";

const fixture = loadFixture(import.meta.url);

test("live analytic transform cases agree with Three.js affine maps", () => {
  for (const row of fixture.analyticTransforms as {name:string;vector?:number[];origin?:number[];angle?:number;expected:number[]}[]) {
    const p = new Vector3(2, 1, 0);
    const axis = new Vector3(...row.vector ?? [0,0,1]);
    const origin = new Vector3(...row.origin ?? [0,0,0]);
    switch (row.name) {
      case "translate": p.applyMatrix4(new Matrix4().makeTranslation(axis.x, axis.y, axis.z)); break;
      case "rotate":
      case "rotateAbout": p.sub(origin).applyMatrix4(new Matrix4().makeRotationAxis(axis, row.angle!)).add(origin); break;
      case "scale": p.sub(origin).applyMatrix4(new Matrix4().makeScale(axis.x, axis.y, axis.z)).add(origin); break;
      case "mirror": p.sub(origin).reflect(axis).add(origin); break;
      case "copy": break;
      default: throw new Error(row.name);
    }
    expect(p.toArray().every((value, i) => Math.abs(value - row.expected[i]!) < 1e-12)).toBe(true);
  }
});
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
  const source = lineOf(closest.inputs.curve as Line);
  expect(Math.abs(source.closestPointToPointParameter(target, false) - (closest.outputs!.parameter as number))).toBeLessThan(1e-9);
  expect(close(source.closestPointToPoint(target, false, new Vector3()).toArray(), closest.outputs!.point as number[])).toBe(true);
});

test("circle domain, curvature and closest parameter follow the closed forms", () => {
  const domain = casesOf(fixture, "brep.evaluate.curveDomain").find(item => (item.inputs.curve as Circle).recipe === "circle")!;
  expect(domain.outputs).toEqual({ start: 0, end: 2 * Math.PI, span: 2 * Math.PI });
  const curvature = casesOf(fixture, "brep.evaluate.curveCurvature").find(item => (item.inputs.curve as Circle).recipe === "circle")!;
  expect(curvature.outputs!.curvature).toBe(1 / (curvature.inputs.curve as Circle).radius);
  const closest = casesOf(fixture, "brep.evaluate.curveClosestParameter").find(item => (item.inputs.curve as Circle).recipe === "circle")!;
  const target = new Vector2(...((closest.inputs.point as number[]).slice(0, 2) as [number, number]));
  const angle = Math.atan2(target.y, target.x);
  expect(Math.abs(angle - (closest.outputs!.parameter as number))).toBeLessThan(1e-9);
  const projected = circleOf(closest.inputs.curve as Circle).getPoint(angle / (2 * Math.PI));
  expect(close([projected.x, projected.y, 0], closest.outputs!.point as number[])).toBe(true);
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


test("the datum plane preview has two triangles and four boundary edges", () => {
  const { planePreview: preview } = fixture as typeof fixture & {
    planePreview: { source: string; size: [number, number]; bbox: [number[], number[]]; triangles: number; edges: number };
  };
  const source = casesOf(fixture, "brep.evaluate.surfacePoint").find(({ name }) => name === preview.source)!;
  const { origin } = source.inputs.surface as { origin: [number, number, number] };
  const plane = new PlaneGeometry(...preview.size).translate(...origin);
  plane.computeBoundingBox();
  expect([plane.boundingBox!.min.toArray(), plane.boundingBox!.max.toArray()]).toEqual(preview.bbox);
  expect(plane.index!.count / 3).toBe(preview.triangles);
  expect(new EdgesGeometry(plane).getAttribute("position").count / 2).toBe(preview.edges);
});

test("Three.js reconstructs scaled and reflected standalone curves and analytic surfaces", () => {
  type XYZ = [number, number, number];
  type AffineCase = {
    kind: string; center?: XYZ; x?: XYZ; y?: XYZ; radii?: number[]; radius?: number; halfAngle?: number;
    major?: number; minor?: number; operation: string; origin: XYZ; vector: XYZ; angle?: number;
    witnesses: [[number, ...number[]], XYZ][];
  };
  const rows = (fixture as unknown as { standaloneAffineTransforms: AffineCase[] }).standaloneAffineTransforms;
  for (const row of rows) {
    const center = new Vector3(...row.origin);
    const parameter = new Vector3(...row.vector);
    for (const [[u, v], expected] of row.witnesses) {
      let point: Vector3;
      if (row.radii) {
        const plane = new EllipseCurve(0, 0, row.radii[0]!, row.radii[1]!);
        const uv = plane.getPoint(u / (2 * Math.PI));
        point = new Vector3(...row.center!).addScaledVector(new Vector3(...row.x!), uv.x).addScaledVector(new Vector3(...row.y!), uv.y);
      } else {
        const height = v!;
        const [r, z] = row.kind === "cylinder" ? [row.radius!, height]
          : row.kind === "cone" ? [height * Math.tan(row.halfAngle!), height]
          : row.kind === "sphere" ? [row.radius! * Math.cos(height), row.radius! * Math.sin(height)]
          : [row.major! + row.minor! * Math.cos(height), row.minor! * Math.sin(height)];
        point = new Vector3(r, 0, z).applyAxisAngle(new Vector3(0, 0, 1), u);
      }
      point.sub(center);
      if (row.operation === "scale") point.multiply(parameter);
      else if (row.operation === "mirror") point.reflect(parameter);
      else point.applyAxisAngle(parameter, row.angle!);
      expect(point.add(center).distanceTo(new Vector3(...expected)), row.kind).toBeLessThan(1e-12);
    }
  }
});

import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { CubicBezierCurve3, EllipseCurve, LineCurve3, QuadraticBezierCurve3, Vector3, Vector4 } from "three";
import { NURBSCurve } from "three/addons/curves/NURBSCurve.js";
import { disagreements, type FixtureCase } from "../../../🧪️tests/🧰️oracle-support/🟦️.ts";
import { oracleCase } from "../../../🧪️tests/🧰️construction-support/🟦️.ts";

type XYZ = [number, number, number];
type PeriodicQuery = { span: number; parameter: number; curvature: number };
type Fixture = {
  tolerance: number;
  cases: FixtureCase[];
  originalHelix: { name: string; origin: XYZ; axis: XYZ; radial: XYZ; radius: number; pitch: number; turns: number; segments: number; length: number; samples: [number, XYZ][] }[];
  originalApproximation: { name: string; points: XYZ[]; degree: number; controlPoints: number; knots: number[]; controls: XYZ[]; samples: [number, XYZ][] }[];
  interpolationParameters: Record<string, number[]>;
  wirePreview: Record<string, { edgeLabels: number; vertexLabels: number; segments: number }>;
  splineEdgePreview: { degree: number; knots: number[]; controls: XYZ[]; trim: [number, number]; segments: [XYZ, XYZ][] };
  edgePreview: { startAngle: number; endAngle: number; radius: number; deflection: number; start: XYZ; end: XYZ; length: number };
  queries: { line: { parameter: number; point: XYZ; tangent: XYZ; curvature: number }; circle: PeriodicQuery; ellipse: PeriodicQuery };
};
const fixture = JSON.parse(readFileSync(new URL("../../../../../../🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/〰️brep-curve/🔣️.json", import.meta.url), "utf8")) as Fixture;
const catalogue = JSON.parse(readFileSync(new URL("../../../../../🗂️catalogue/🔣️brep-curve.json", import.meta.url), "utf8")) as { kinds: { id: string }[] };

test("the fixture states every brep.curve kind of the catalogue and no other", () => {
  expect([...new Set(fixture.cases.map(({ kind }) => kind))].sort()).toEqual(catalogue.kinds.map(({ id }) => id).sort());
});

test("every fixture case states a fault or at least one measure", () => {
  for (const { name, outputs, fault } of fixture.cases) expect(fault !== undefined || Object.keys((outputs?.shape ?? {}) as object).length > 1, name).toBe(true);
});

test("three recomputes the length, ends, closure, bounding box and spline passage of every curve and wire case (EllipseCurve, CatmullRomCurve3 and plain polyline sums)", () => {
  expect(disagreements(fixture.cases, oracleCase, fixture.tolerance)).toEqual([]);
}, { timeout: 120000 });

test("Three.js reproduces line samples and conic spans and curvature", () => {
  const line = fixture.cases.find(({ name }) => name === "line")!.inputs as { start: XYZ; end: XYZ };
  const segment = new LineCurve3(new Vector3(...line.start), new Vector3(...line.end));
  const { parameter, point, tangent, curvature } = fixture.queries.line;
  expect(segment.getTangent(1).angleTo(segment.getTangent(0))).toBe(curvature);
  expect(segment.getPoint(parameter).toArray()).toEqual(point);
  expect(segment.getPoint(1).sub(segment.getPoint(0)).toArray()).toEqual(tangent);

  for (const name of ["circle", "ellipse"] as const) {
    const { radius, semiMajor, semiMinor } = fixture.cases.find((row) => row.name === name)!.inputs as { radius?: number; semiMajor?: number; semiMinor?: number };
    const curve = new EllipseCurve(0, 0, semiMajor ?? radius!, semiMinor ?? radius!);
    const { span, parameter, curvature } = fixture.queries[name];
    expect(curve.aEndAngle - curve.aStartAngle).toBe(span);
    const t = parameter / span;
    const h = 1e-4;
    const left = curve.getPoint(t - h);
    const center = curve.getPoint(t);
    const right = curve.getPoint(t + h);
    const first = right.clone().sub(left).multiplyScalar(0.5 / h);
    const second = right.clone().add(left).addScaledVector(center, -2).multiplyScalar(1 / (h * h));
    expect(first.cross(second) ** 2 / first.length() ** 6).toBeCloseTo(curvature ** 2, 4);
  }
});


test("wire preview references agree with Three.js segment and vertex geometry", () => {
  for (const [name, reference] of Object.entries(fixture.wirePreview)) {
    const item = fixture.cases.find((row) => row.name === name)!;
    const input = item.inputs as { points?: XYZ[]; width?: number; height?: number; radius?: number; sides?: number };
    const points = input.points ?? (input.width !== undefined
      ? [[0, 0, 0], [input.width, 0, 0], [input.width, input.height!, 0], [0, input.height!, 0], [0, 0, 0]] as XYZ[]
      : Array.from({ length: input.sides! + 1 }, (_, index) => {
        const turn = (index % input.sides!) * 2 * Math.PI / input.sides!;
        return [input.radius! * Math.cos(turn), input.radius! * Math.sin(turn), 0] as XYZ;
      }));
    const vertices = points.map((position) => new Vector3(...position));
    const segments = vertices.slice(1).map((point, index) => new LineCurve3(vertices[index]!, point));
    expect(segments.length).toBe(reference.segments);
    expect(segments.length).toBe(reference.edgeLabels);
    expect(new Set(vertices.map((point) => JSON.stringify(point.toArray()))).size).toBe(reference.vertexLabels);
    expect(segments.reduce((total, segment) => total + segment.getLength(), 0)).toBeCloseTo((item.outputs!.shape as { length: number }).length);
  }
});

test("the analytic edge fixture matches a trimmed Three.js circle", () => {
  const edge = fixture.edgePreview;
  const trim = new EllipseCurve(0, 0, edge.radius, edge.radius, edge.startAngle, edge.endAngle);
  for (const [t, point] of [[0, edge.start], [1, edge.end]] as const) {
    expect(new Vector3(...trim.getPoint(t).toArray(), 0).distanceTo(new Vector3(...point))).toBeLessThan(1e-12);
  }
  trim.arcLengthDivisions = 2048;
  expect(trim.getLength()).toBeCloseTo(edge.length, 6);
});


test("Three.js reproduces the original centripetal spline sample points", () => {
  for (const [name, parameters] of Object.entries(fixture.interpolationParameters)) {
    const item = fixture.cases.find((row) => row.name === name)!;
    const sample = (item.inputs as { points: XYZ[] }).points.map((point) => new Vector3(...point));
    const steps = sample.slice(1).map((point, index) => Math.sqrt(point.distanceTo(sample[index]!)));
    const total = steps.reduce((length, step) => length + step, 0);
    let distance = 0;
    steps.forEach((step, index) => {
      distance += step;
      expect(distance / total).toBeCloseTo(parameters[index + 1]!, 12);
    });
    const a = sample[0]!;
    const d = sample.at(-1)!;
    const curve = (() => {
      if (sample.length === 2 || name === "interpolated cubic with interior knots") return new LineCurve3(a, d);
      if (sample.length === 3) {
        const t = parameters[1]!;
        const control = sample[1]!.clone().addScaledVector(a, -((1 - t) ** 2))
          .addScaledVector(d, -(t ** 2)).multiplyScalar(1 / (2 * t * (1 - t)));
        return new QuadraticBezierCurve3(a, control, d);
      }
      const first = sample[1]!.clone().multiplyScalar(27).addScaledVector(a, -8).sub(d);
      const second = sample[2]!.clone().multiplyScalar(27).sub(a).addScaledVector(d, -8);
      const b = first.clone().multiplyScalar(2).sub(second).divideScalar(18);
      const c = second.clone().multiplyScalar(2).sub(first).divideScalar(18);
      return new CubicBezierCurve3(a, b, c, d);
    })();
    parameters.forEach((parameter, index) => {
      expect(curve.getPoint(parameter).distanceTo(sample[index]!)).toBeLessThan(1e-12);
    });
  }
});

test("Three.js reproduces clipped span endpoints on each side of a spline jump", () => {
  const { degree, knots, controls, trim, segments } = fixture.splineEdgePreview;
  expect(degree).toBe(1);
  const clipped = controls.filter((_, i) => i % 2 === 0).map((point, i) => {
    const start = knots[i * 2 + 1]!;
    const end = knots[i * 2 + 2]!;
    const line = new LineCurve3(new Vector3(...point), new Vector3(...controls[i * 2 + 1]!));
    return [Math.max(start, trim[0]), Math.min(end, trim[1])].map((u) => line.getPoint((u - start) / (end - start)).toArray());
  });
  expect(clipped).toEqual(segments);
  expect(clipped[0]![1]).not.toEqual(clipped[1]![0]);
});

test("Three.js evaluates the least-squares fit witnesses and fixed endpoints", () => {
  for (const row of fixture.originalApproximation) {
    const controls = row.controls.map((point) => new Vector4(...point, 1));
    const degree = Math.max(1, Math.min(row.degree, controls.length - 1));
    const curve = new NURBSCurve(degree, row.knots, controls);
    for (const [parameter, point] of row.samples) {
      expect(curve.getPoint(parameter).distanceTo(new Vector3(...point)), row.name).toBeLessThan(1e-12);
    }
    expect(curve.getPoint(0).toArray()).toEqual(row.points[0]!);
    expect(curve.getPoint(1).toArray()).toEqual(row.points.at(-1)!);
    if (controls.length === degree + 1) {
      const witnesses = row.points.map((point, i) => curve.getPoint(i / (row.points.length - 1)).sub(new Vector3(...point)));
      for (let j = 1; j < controls.length - 1; j++) {
        const basis = new NURBSCurve(degree, row.knots, controls.map((_, k) => new Vector4(Number(k === j), 0, 0, 1)));
        const gradient = witnesses.reduce((sum, delta, i) => sum.addScaledVector(delta, basis.getPoint(i / (row.points.length - 1)).x), new Vector3());
        expect(gradient.length(), row.name).toBeLessThan(1e-12);
      }
    }
  }
});

test("Three.js independently reconstructs the sampled helix pitch and handedness", () => {
  for (const row of fixture.originalHelix) {
    const axis = new Vector3(...row.axis);
    if (axis.lengthSq() === 0) axis.set(0, 0, 1);
    axis.normalize();
    const radial = new Vector3(...row.radial).multiplyScalar(row.radius);
    const origin = new Vector3(...row.origin);
    const count = Math.max(8, Math.ceil(Math.abs(row.turns) * 32));
    expect(count).toBe(row.segments);
    const points = Array.from({ length: count + 1 }, (_, i) => {
      const turn = row.turns * i / count;
      return radial.clone().applyAxisAngle(axis, 2 * Math.PI * turn).add(origin).addScaledVector(axis, row.pitch * turn);
    });
    const chords = points.slice(1).map((point, i) => new LineCurve3(points[i]!, point));
    expect(chords.reduce((sum, chord) => sum + chord.getLength(), 0)).toBeCloseTo(row.length, 12);
    for (const [index, expected] of row.samples) {
      expect(points[index]!.distanceTo(new Vector3(...expected)), row.name).toBeLessThan(1e-12);
    }
    const a = points[0]!.clone().sub(origin);
    const b = points[1]!.clone().sub(origin);
    expect(Math.sign(a.cross(b).dot(axis))).toBe(Math.sign(row.turns));
    expect(Math.sign(points.at(-1)!.clone().sub(origin).dot(axis))).toBe(Math.sign(row.turns * row.pitch));
  }
});

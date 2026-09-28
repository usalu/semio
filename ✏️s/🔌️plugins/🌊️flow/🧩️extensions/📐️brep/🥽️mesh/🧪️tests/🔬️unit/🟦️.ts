import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { BoxGeometry, Box3, IcosahedronGeometry, Line3, Matrix4, Plane, ShapeUtils, Triangle, Vector2, Vector3 } from "three";
import Ajv from "ajv/dist/2020.js";
import { analyzePolygonMesh, componentVertexIds, knifeCutMesh, loopCutMesh, parsePolygonMesh, transformMeshComponents, type PolygonMesh } from "../../🟦️.ts";

const fixtures = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
const knives = JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/✂️knife-cut/🔣️.json", import.meta.url), "utf8"));
const validateKnife = new Ajv().compile(JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧬️schema/✂️knife-cut/🔣️.json", import.meta.url), "utf8")));
for (const fixture of knives.cases) test(`knife cut and Three.js surface oracle: ${fixture.name}`, () => {
  const input: PolygonMesh = structuredClone(fixture.mesh);
  expect(validateKnife(fixture.cut)).toBe(true);
  const result = knifeCutMesh(input, fixture.cut);
  expect(input).toEqual(fixture.mesh);
  expect(result.vertices.slice(0, input.vertices.length)).toEqual(input.vertices.map(point => point.map(Math.fround)));
  const report = analyzePolygonMesh(result);
  for (const key of ["vertices", "faces", "boundaryEdges"] as const) if (fixture.expected[key] !== undefined) expect(report[key]).toBe(fixture.expected[key]);
  if (fixture.expected.minimumFaces) expect(report.faces).toBeGreaterThanOrEqual(fixture.expected.minimumFaces);
  expect(report.nonManifoldEdges).toBe(0); expect(report.inconsistentEdges).toBe(0); expect(report.degenerateTriangles).toBe(0);
  expect(new Set(result.faces.flat()).size).toBe(result.vertices.length);
  expect(report.area / fixture.expected.area).toBeCloseTo(1, 6);
  if (fixture.expected.volume) expect(report.volume! / fixture.expected.volume).toBeCloseTo(1, 6);
  for (const face of fixture.unchangedFaces ?? []) expect(result.faces).toContainEqual(face);
  const points = input.faces[fixture.cut.face].map(id => new Vector3(...input.vertices[id]));
  const normal = points.reduce((sum, point, i) => sum.add(new Vector3().crossVectors(point.clone().sub(points[0]), points[(i + 1) % points.length].clone().sub(points[0]))), new Vector3()).normalize();
  const start = new Vector3(...fixture.cut.start), direction = new Vector3(...fixture.cut.end).sub(start);
  const plane = new Plane().setFromNormalAndCoplanarPoint(direction.cross(normal).normalize(), start);
  const selected = new Set(input.faces[fixture.cut.face]), extent = Math.max(...points.map(point => point.distanceTo(points[0])));
  for (const face of result.faces) if (face.every(id => id >= input.vertices.length || selected.has(id))) {
    const distances = face.map(id => plane.distanceToPoint(new Vector3(...result.vertices[id])));
    expect(distances.some(d => d > extent * 1e-6) && distances.some(d => d < -extent * 1e-6)).toBe(false);
  }
  const hits: number[][] = [];
  for (let i = 0; i < points.length; i++) {
    const hit = plane.intersectLine(new Line3(points[i], points[(i + 1) % points.length]), new Vector3());
    if (hit && !points.some(point => point.equals(hit))) hits.push(hit.toArray().map(Math.fround));
  }
  for (const point of fixture.boundaryPoints) {
    const rounded = point.map(Math.fround);
    expect(hits).toContainEqual(rounded);
    expect(result.vertices).toContainEqual(rounded);
  }
  let oracleArea = 0, oracleVolume = 0;
  for (const face of result.faces) {
    const points = face.map(id => new Vector3(...result.vertices[id]));
    const normal = points.reduce((sum, point, i) => sum.add(new Vector3().crossVectors(point.clone().sub(points[0]), points[(i + 1) % points.length].clone().sub(points[0]))), new Vector3());
    const axis = normal.toArray().map(Math.abs).indexOf(Math.max(...normal.toArray().map(Math.abs)));
    const projected = points.map(point => new Vector2(...point.toArray().filter((_, i) => i !== axis) as [number, number]));
    for (const triangle of ShapeUtils.triangulateShape(projected, [])) {
      const [a, b, c] = triangle.map(id => points[id]);
      const surface = new Triangle(a, b, c), winding = Math.sign(surface.getNormal(new Vector3()).dot(normal));
      oracleArea += surface.getArea(); oracleVolume += winding * a.dot(new Vector3().crossVectors(b, c)) / 6;
      if (fixture.normal) expect(normal.dot(new Vector3(...fixture.normal))).toBeGreaterThan(0);
    }
  }
  expect(oracleArea / fixture.expected.area).toBeCloseTo(1, 6);
  if (fixture.expected.volume) expect(oracleVolume / fixture.expected.volume).toBeCloseTo(1, 6);
  console.log(`[DEBUG] knife cut ${fixture.name}: ${JSON.stringify(report)}`);
});
test("knife cut rejects invalid or non-finite input without mutation", () => {
  const input = structuredClone(knives.cases[0].mesh);
  for (const cut of [...knives.invalid.map((fixture: { cut: unknown }) => fixture.cut), { face: 0, start: [Infinity, 0, 0], end: [1, 3, 0] }, { face: 0, start: [NaN, 0, 0], end: [1, 3, 0] }]) {
    expect(() => knifeCutMesh(input, cut)).toThrow();
    expect(input).toEqual(knives.cases[0].mesh);
  }
});
const componentTransforms = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🧭️component-transform/🔣️.json", import.meta.url), "utf8"));
const validateComponentTransform = new Ajv().compile(JSON.parse(readFileSync(new URL("../../🧬️schema/🧭️component-transform/🔣️.json", import.meta.url), "utf8")));
for (const fixture of componentTransforms.cases) test(`component transform and Three.js oracle: ${fixture.name}`, () => {
  const input = structuredClone(componentTransforms.mesh), transform = fixture.transform;
  expect(validateComponentTransform(transform)).toBe(true);
  const ids = componentVertexIds(input, transform.mode, transform.selection);
  expect(ids).toEqual(fixture.vertices);
  const actual = transformMeshComponents(input, transform);
  expect(input).toEqual(componentTransforms.mesh);
  expect(actual.faces).toEqual(input.faces);
  const pivot = transform.pivot === "selection" ? ids.reduce((sum, id) => sum.add(new Vector3(...input.vertices[id])), new Vector3()).divideScalar(ids.length) : new Vector3(...transform.pivot);
  expect(pivot.toArray()).toEqual(fixture.pivot);
  const matrix = transform.operation === "translate" ? new Matrix4().makeTranslation(...transform.vector)
    : new Matrix4().makeTranslation(...pivot.toArray()).multiply(transform.operation === "rotate" ? new Matrix4().makeRotationAxis(new Vector3(...transform.vector).normalize(), transform.angle) : new Matrix4().makeScale(...transform.vector)).multiply(new Matrix4().makeTranslation(...pivot.clone().negate().toArray()));
  for (let id = 0; id < input.vertices.length; id++) {
    const oracle = new Vector3(...input.vertices[id]);
    if (ids.includes(id)) oracle.applyMatrix4(matrix);
    for (let axis = 0; axis < 3; axis++) {
      expect(actual.vertices[id][axis]).toBeCloseTo(fixture.expected[id][axis], 6);
      expect(actual.vertices[id][axis]).toBeCloseTo(oracle.getComponent(axis), 6);
    }
  }
  console.log(`[DEBUG] component transform ${fixture.name}: vertices=${ids}, pivot=${pivot.toArray()}`);
});
test("component transforms reject invalid targets and nonfinite or collapsed transforms atomically", () => {
  const mesh = structuredClone(componentTransforms.mesh), base = componentTransforms.cases[0].transform;
  for (const patch of [{ selection: [] }, { selection: [0, 999] }, { mode: "object" }, { vector: [Infinity, 0, 0] }, { operation: "scale", vector: [1, 0, 1] }, { operation: "rotate", vector: [0, 0, 0] }, { pivot: [NaN, 0, 0] }]) {
    expect(() => transformMeshComponents(mesh, { ...base, ...patch })).toThrow();
    expect(mesh).toEqual(componentTransforms.mesh);
  }
});
for (const fixture of fixtures.brepScales) test(`Three.js affine box oracle: ${fixture.name}`, () => {
  const center = new Vector3(...fixture.center);
  const geometry = new BoxGeometry(1, 1, 1).translate(0.5, 0.5, 0.5).translate(...center.clone().negate().toArray()).scale(...fixture.factors as [number, number, number]).translate(...center.toArray());
  geometry.computeBoundingBox();
  expect(geometry.boundingBox!.min.toArray()).toEqual(fixture.minimum);
  expect(geometry.boundingBox!.max.toArray()).toEqual(fixture.maximum);
  const size = geometry.boundingBox!.getSize(new Vector3());
  expect(size.x * size.y * size.z).toBe(fixture.volume);
  geometry.dispose();
});
for (const fixture of fixtures.reflections) test(`Three.js reflection oracle: ${fixture.name}`, () => {
  const geometry = new BoxGeometry(1, 1, 1).applyMatrix4(new Matrix4().makeScale(...fixture.factors as [number, number, number]));
  const positions = geometry.getAttribute("position"), indices = geometry.getIndex()!;
  let volume = 0;
  for (let i = 0; i < indices.count; i += 3) {
    const [a, b, c] = [0, 1, 2].map(offset => new Vector3().fromBufferAttribute(positions, indices.getX(i + offset)));
    volume += a.dot(b.cross(c)) / 6;
  }
  const orientation = fixture.factors.filter((value: number) => value < 0).length % 2 ? -1 : 1;
  expect(volume * orientation / fixture.volume).toBeCloseTo(1, 6);
  geometry.dispose();
});
const validate = new Ajv().compile(JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8")));

describe("portable mesh widgets", () => {
  for (const fixture of fixtures.meshes) test(fixture.name, () => {
    expect(validate(fixture.mesh)).toBe(true);
    const mesh = parsePolygonMesh(JSON.stringify(fixture.mesh));
    const report = analyzePolygonMesh(mesh);
    for (const [key, expected] of Object.entries(fixture.expected)) {
      if (expected === null) expect(report[key as keyof typeof report]).toBeUndefined();
      else if (expected === 0) expect(report[key as keyof typeof report]).toBe(0);
      else expect((report[key as keyof typeof report] as number) / (expected as number)).toBeCloseTo(1, 6);
    }
    const positions = mesh.vertices.map(point => new Vector3(...point));
    const bounds = new Box3().setFromPoints(positions);
    expect(report.minimum).toEqual(bounds.min.toArray()); expect(report.maximum).toEqual(bounds.max.toArray());
    let area = 0, signedVolume = 0;
    for (const face of mesh.faces) {
      const normal = new Vector3();
      for (let i = 0; i < face.length; i++) normal.add(new Vector3().crossVectors(positions[face[i]], positions[face[(i + 1) % face.length]]));
      const axis = normal.toArray().map(Math.abs).indexOf(Math.max(...normal.toArray().map(Math.abs)));
      const projected = face.map(id => new Vector2(...mesh.vertices[id].filter((_, i) => i !== axis) as [number, number]));
      for (const triangle of ShapeUtils.triangulateShape(projected, [])) {
        const [a, b, c] = triangle.map(index => positions[face[index]]);
        area += new Triangle(a, b, c).getArea();
        signedVolume += a.dot(new Vector3().crossVectors(b, c)) / 6;
      }
    }
    if (area === 0) expect(report.area).toBe(0);
    else expect(report.area / area).toBeCloseTo(1, 6);
    if (report.volume !== undefined) expect(report.volume / Math.abs(signedVolume)).toBeCloseTo(1, 6);
    console.log(`[DEBUG] ${fixture.name}: ${JSON.stringify(report)}`);
  });
  for (const fixture of fixtures.invalid) test(fixture.name, () => expect(() => parsePolygonMesh(JSON.stringify(fixture.mesh))).toThrow());
  test("closed winding faults withhold volume", () => {
    const mesh: PolygonMesh = structuredClone(fixtures.meshes[0].mesh); mesh.faces[0].reverse();
    expect(analyzePolygonMesh(mesh).inconsistentEdges).toBe(3);
    expect(analyzePolygonMesh(mesh).volume).toBeUndefined();
  });
});

const modeling = JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json", import.meta.url), "utf8"));
const validateLoopCut = new Ajv().compile(JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧬️schema/✂️loop-cut/🔣️.json", import.meta.url), "utf8")));
for (const fixture of modeling.loopCuts) test(`connected loop cut: ${fixture.name}`, () => {
  const input = structuredClone(fixture.mesh) as PolygonMesh;
  const halfedges = input.faces.flatMap(face => face.map((a, i) => [a, face[(i + 1) % face.length]]));
  const edges = fixture.edges.map(([a, b]: number[]) => {
    const directed = halfedges.findIndex(edge => edge[0] === a && edge[1] === b);
    return directed < 0 ? halfedges.findIndex(edge => edge[0] === b && edge[1] === a) : directed;
  });
  expect(validateLoopCut({ edges, cuts: fixture.cuts })).toBe(true);
  const result = loopCutMesh(input, edges, fixture.cuts);
  expect(input).toEqual(fixture.mesh);
  if (fixture.addedPositions) expect(result.vertices.slice(input.vertices.length).sort()).toEqual([...fixture.addedPositions].sort());
  if (fixture.unchangedFaces) for (const face of fixture.unchangedFaces) expect(result.faces).toContainEqual(face);
  const report = analyzePolygonMesh(result);
  for (const [key, value] of Object.entries(fixture.expected)) expect(report[key as keyof typeof report]).toBeCloseTo(value as number, 6);
  expect(report.nonManifoldEdges).toBe(0); expect(report.inconsistentEdges).toBe(0); expect(report.degenerateTriangles).toBe(0);
  expect(new Set(result.faces.flat()).size).toBe(result.vertices.length);
  let area = 0, volume = 0;
  for (const face of result.faces) {
    const vertices = face.map(id => new Vector3(...result.vertices[id]));
    const normal = vertices.reduce((sum, point, i) => sum.add(new Vector3().crossVectors(point, vertices[(i + 1) % vertices.length])), new Vector3());
    const axis = normal.toArray().map(Math.abs).indexOf(Math.max(...normal.toArray().map(Math.abs)));
    const contour = vertices.map(point => new Vector2(...point.toArray().filter((_, i) => i !== axis) as [number, number]));
    for (const triangle of ShapeUtils.triangulateShape(contour, [])) {
      const [a, b, c] = triangle.map(id => vertices[id]);
      const surface = new Triangle(a, b, c);
      const orientation = Math.sign(surface.getNormal(new Vector3()).dot(normal));
      area += surface.getArea(); volume += orientation * a.dot(new Vector3().crossVectors(b, c)) / 6;
    }
  }
  expect(area).toBeCloseTo(fixture.expected.area, 6);
  if (fixture.expected.volume !== undefined) expect(volume).toBeCloseTo(fixture.expected.volume, 6);
  console.log(`[DEBUG] loop cut ${fixture.name}: ${JSON.stringify(report)}`);
});
test("loop cut rejects invalid input without changing its mesh", () => {
  for (const fixture of modeling.invalidLoopCuts) {
    const input = structuredClone(fixture.mesh);
    expect(() => loopCutMesh(input, fixture.edges, fixture.cuts)).toThrow();
    expect(input).toEqual(fixture.mesh);
  }
  const mesh = structuredClone(modeling.loopCuts[0].mesh);
  for (const [edges, cuts] of [[[], 1], [[0], 0], [[0], 257], [[0, 999], 1], [[0, 1, 8], 256], [[0], 1.5], [[-1], 1]] as [number[], number][]) {
    expect(() => loopCutMesh(mesh, edges, cuts)).toThrow();
    expect(mesh).toEqual(modeling.loopCuts[0].mesh);
  }
  expect(() => loopCutMesh({ vertices: [[0, 0, 0], [1, 0, 0], [0, 1, 0]], faces: [[0, 1, 2]] }, [0], 1)).toThrow();
});
for (const direction of modeling.directions) for (const scale of modeling.scales) test(`Three.js vector oracle: ${direction.vector} at ${scale}`, () => {
  const unit = new Vector3(...direction.vector).multiplyScalar(scale).normalize().toArray();
  for (let axis = 0; axis < 3; axis++) expect(unit[axis]).toBeCloseTo(direction.unit[axis], 6);
});
for (const fixture of modeling.spheres) test(`Three.js sphere oracle: level ${fixture.subdivisions}, radius ${fixture.radius}`, () => {
  const geometry = new IcosahedronGeometry(fixture.radius, 2 ** fixture.subdivisions - 1);
  const positions = geometry.getAttribute("position");
  expect(positions.count / 3).toBe(fixture.faces);
  const vertices = new Set<string>();
  for (let i = 0; i < positions.count; i++) {
    const point = new Vector3().fromBufferAttribute(positions, i).divideScalar(fixture.radius);
    expect(point.length()).toBeCloseTo(1, 6);
    vertices.add(point.toArray().map(value => value.toFixed(5)).join(","));
  }
  expect(vertices.size).toBe(fixture.vertices);
  geometry.dispose();
});
for (const fixture of modeling.cases) test(`Three.js modeling oracle: ${fixture.name}`, () => {
  const geometry = new BoxGeometry(1, 1, fixture.operation === "extrude" ? 2 : 1);
  const position = geometry.getAttribute("position"), index = geometry.index!;
  let area = 0, volume = 0;
  for (let i = 0; i < index.count; i += 3) {
    const [a, b, c] = [0, 1, 2].map(offset => new Vector3().fromBufferAttribute(position, index.getX(i + offset)));
    area += new Triangle(a, b, c).getArea(); volume += a.dot(new Vector3().crossVectors(b, c)) / 6;
  }
  expect(area).toBeCloseTo(fixture.area, 6); expect(volume).toBeCloseTo(fixture.volume, 6);
  geometry.dispose();
});

for (const fixture of modeling.polygons) for (const scale of modeling.scales) test(`Three.js concave subdivision oracle: ${fixture.name} at ${scale}`, () => {
  const vertices = fixture.vertices.map((point: number[]) => point.map(coordinate => coordinate * scale));
  const projected = vertices.map(([x, y]: number[]) => new Vector2(x, y));
  const triangles = ShapeUtils.triangulateShape(projected, []);
  const area = triangles.reduce((sum, triangle) => sum + new Triangle(...triangle.map(id => new Vector3(...vertices[id])) as [Vector3, Vector3, Vector3]).getArea(), 0);
  expect(area / (scale * scale)).toBeCloseTo(fixture.area, 6);
  expect(triangles.length * 3).toBe(fixture.expectedFaces);
  expect(fixture.vertices.length + triangles.length).toBe(fixture.expectedVertices);
});

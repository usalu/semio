import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { BoxGeometry, PlaneGeometry, CylinderGeometry, ConeGeometry, Box3, IcosahedronGeometry, Line3, Matrix4, Plane, ShapeUtils, Triangle, Vector2, Vector3 } from "three";
import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader.js";
import Ajv from "ajv/dist/2020.js";
import { inspectMeshComponent, analyzePolygonMesh, componentVertexIds, knifeCutMesh, loopCutMesh, parsePolygonMesh, transformMeshComponents, type PolygonMesh } from "../../🟦️.ts";

const fixtures = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
const attributes = JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🏗️mesh-engine/🧫️fixtures/🎨️attributes/🔣️.json", import.meta.url), "utf8"));
test("indexed owned samples preserve UV seams and shared custom values", () => {
  const mesh = parsePolygonMesh(JSON.stringify(attributes.indexedMesh));
  const schema = JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🏗️mesh-engine/🧬️schema/🥽️polygon/🔣️.json", import.meta.url), "utf8"));
  expect(new Ajv().compile(schema)(mesh)).toBe(true);
  const normal = mesh.attributes!.normal;
  const expanded = normal.indices!.map((index: number) => normal.values[index]);
  expect(expanded).toEqual(attributes.mesh.attributes.normal.values);
  const independent = new Float32Array(expanded.flat() as number[]);
  expect(independent.length).toBe(attributes.expected.cornerCount * 3);
  const uv = mesh.attributes!.uv;
  expect(uv.values[uv.indices![0]]).not.toEqual(uv.values[uv.indices![3]]);
  const labels = mesh.attributes!.labels;
  expect(labels.values.length).toBe(1);
  expect(labels.values[labels.indices![0]]).toBe(labels.values[labels.indices![1]]);
  for (const indices of [[0], [0, 0, 0, 0, 0, 1], [0, 0, 0, 0, 0, -1]]) {
    const invalid = structuredClone(mesh);invalid.attributes!.normal.indices = indices;
    expect(() => parsePolygonMesh(JSON.stringify(invalid))).toThrow();
  }
});
test("polygon parser preserves owned channels and refuses invalid cardinality and references", () => {
  expect(parsePolygonMesh(JSON.stringify(attributes.mesh))).toEqual(attributes.mesh);
  for (const invalid of attributes.invalidCases) {
    const mesh = structuredClone(attributes.mesh);
    if (invalid.channel) mesh.attributes[invalid.channel].values = invalid.values;
    if (invalid.material) mesh.materials[invalid.material].baseColorTexture = invalid.baseColorTexture;
    expect(() => parsePolygonMesh(JSON.stringify(mesh))).toThrow();
  }
});
test("owned polygon attribute declaration and independent inverse-transpose oracle", () => {
  const schema = JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🏗️mesh-engine/🧬️schema/🥽️polygon/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv().compile(schema);
  expect(validate(attributes.mesh)).toBe(true);
  const normal = new Vector3(...attributes.mesh.attributes.normal.values[0]);
  const inverseTranspose = new Matrix4().makeScale(...attributes.normalTransform.scale).invert().transpose();
  normal.set(...attributes.mesh.attributes.normal.values[0]).transformDirection(inverseTranspose);
  attributes.expected.normalAfterScale.forEach((value: number, axis: number) => expect(normal.toArray()[axis]).toBeCloseTo(value, 6));
  expect(attributes.mesh.attributes.uv.values[0]).not.toEqual(attributes.mesh.attributes.uv.values[3]);
  const nonNumeric = structuredClone(attributes.mesh);
  nonNumeric.attributes.temperature.values = ["a", "b", "c", "d"];
  expect(validate(nonNumeric)).toBe(false);
  const faceNormal = structuredClone(attributes.mesh);
  faceNormal.attributes.normal.domain = "face";
  expect(validate(faceNormal)).toBe(true);
  const wrongDomain = structuredClone(attributes.mesh);
  wrongDomain.attributes.normal.domain = "edge";
  expect(validate(wrongDomain)).toBe(false);
  const wrongDimension = structuredClone(attributes.mesh);
  wrongDimension.attributes.uv.values[0] = [0, 0, 0];
  expect(validate(wrongDimension)).toBe(false);
  const directed = attributes.mesh.attributes.edgeLabel.values;
  expect(directed[attributes.expected.oppositeHalfedges[0]]).not.toEqual(directed[attributes.expected.oppositeHalfedges[1]]);
});
const outputBudget = JSON.parse(readFileSync(new URL("../../🧫️fixtures/⏱️output-budget/🔣️.json", import.meta.url), "utf8"));
test("retained JSON source neutral Unicode and translated projection match independent JSON and Three.js",()=>{
  const law=outputBudget.sourceParsing;const source={...law.mesh,materials:{paint:{label:law.textUnit.repeat(law.textRepeats)}}};const text=JSON.stringify(source);const mesh=parsePolygonMesh(text);expect(JSON.parse(text)).toEqual(source);expect(mesh.materials).toEqual(source.materials);
  const offset=new Vector3(...law.offset as [number,number,number]);const points=source.vertices.map((point:number[])=>new Vector3(...point as [number,number,number]).add(offset));expect(points.map((point:Vector3)=>point.toArray())).toEqual(law.expectedVertices);expect(new Triangle(...points as [Vector3,Vector3,Vector3]).getArea()).toBe(law.area);
});
test("owned reconstruction refusal fixture preserves valid wire geometry and rejects its invalid typed index",()=>{
  const law=outputBudget.reconstructionFault;
  expect(parsePolygonMesh(JSON.stringify(law.mesh))).toEqual(law.mesh);
  expect(new Triangle(...law.mesh.vertices.map((point:number[])=>new Vector3(...point)) as [Vector3,Vector3,Vector3]).getArea()).toBe(0.5);
  expect(()=>parsePolygonMesh(JSON.stringify({...law.mesh,faces:law.ownedFaces}))).toThrow();
  expect(outputBudget.importAdmissionFault.positionStride).toBe(new BoxGeometry().getAttribute("position").itemSize);
  expect(outputBudget.importAdmissionFault.triangleStride).toBe(3);
});
test("retained analysis fixture matches Three.js box measurements",()=>{
  const box=new BoxGeometry(1,1,1),positions=box.getAttribute("position"),indices=box.index!;
  let area=0,volume=0;
  for(let i=0;i<indices.count;i+=3) { const [a,b,c]=[0,1,2].map(offset=>new Vector3().fromBufferAttribute(positions,indices.getX(i+offset)));area+=new Triangle(a,b,c).getArea();volume+=a.dot(new Vector3().crossVectors(b,c))/6; }
  expect(area).toBeCloseTo(outputBudget.analysis.area,12);expect(volume).toBeCloseTo(outputBudget.analysis.volume,12);expect(indices.count/3).toBe(outputBudget.analysis.triangles);
});
for (const fixture of outputBudget.cases) test(`retained output surface and Three.js triangulation oracle: ${fixture.name}`, () => {
  const mesh = parsePolygonMesh(JSON.stringify(fixture.mesh));
  const report = analyzePolygonMesh(mesh);
  let triangles = 0, area = 0;
  for (const face of mesh.faces) {
    const points = face.map(id => new Vector3(...mesh.vertices[id]));
    const projected = points.map(point => new Vector2(point.x,point.y));
    for (const indices of ShapeUtils.triangulateShape(projected,[])) {
      triangles++; const [a,b,c] = indices.map(index => points[index]); area += new Triangle(a,b,c).getArea();
    }
  }
  expect(triangles).toBe(fixture.triangles); expect(report.triangles).toBe(triangles);
  expect(area).toBeCloseTo(fixture.area,6); expect(report.area).toBeCloseTo(area,6);
});
test("retained mirror seam fixture matches independent Three.js reflection", () => {
  const fixture = outputBudget.mirroring;
  const source = new BoxGeometry(1,1,1).translate(...fixture.translation);
  const positions = source.getAttribute("position"), indices = source.index!;
  const reflection = new Matrix4().makeScale(-1,1,1);
  const vertices = new Set<string>(); let triangles = 0, area = 0, volume = 0;
  for (let i = 0; i < indices.count; i += 3) {
    const points = [0,1,2].map(offset => new Vector3().fromBufferAttribute(positions,indices.getX(i+offset)));
    if (points.every(point => point.x === 0)) continue;
    for (const mirrored of [false,true]) {
      let corners = points.map(point => mirrored ? point.clone().applyMatrix4(reflection) : point);
      if (mirrored) corners = [corners[2],corners[1],corners[0]];
      const [a,b,c] = corners; triangles++; area += new Triangle(a,b,c).getArea(); volume += a.dot(new Vector3().crossVectors(b,c))/6;
      for (const point of corners) vertices.add(point.toArray().join(","));
    }
  }
  expect(vertices.size).toBe(fixture.vertices); expect(triangles/2).toBe(fixture.faces);
  expect(area).toBeCloseTo(fixture.area,6); expect(volume).toBeCloseTo(fixture.volume,6);
});
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
const validate = new Ajv().compile(JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🏗️mesh-engine/🧬️schema/🥽️polygon/🔣️.json", import.meta.url), "utf8")));

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
  });
  for (const fixture of fixtures.invalid) test(fixture.name, () => expect(() => parsePolygonMesh(JSON.stringify(fixture.mesh))).toThrow());
  test("closed winding faults withhold volume", () => {
    const mesh: PolygonMesh = structuredClone(fixtures.meshes[0].mesh); mesh.faces[0].reverse();
    expect(analyzePolygonMesh(mesh).inconsistentEdges).toBe(3);
    expect(analyzePolygonMesh(mesh).volume).toBeUndefined();
  });
});

const modeling = JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json", import.meta.url), "utf8"));
for (const fixture of modeling.merges) test(`retained merge portable fixture and Three.js distances: ${fixture.name}`, () => {
  const points = fixture.mesh.vertices.map((point: number[]) => new Vector3(...point));
  const selected = fixture.selection as number[], roots = selected.map((_,index) => index);
  const root = (index: number): number => roots[index] === index ? index : root(roots[index]);
  if (fixture.mode === "distance") {
    for (let i = 0; i < selected.length; i++) for (let j = 0; j < i; j++) if (points[selected[i]].distanceTo(points[selected[j]]) <= fixture.threshold) {
      const a = root(i), b = root(j); roots[Math.max(a,b)] = Math.min(a,b);
    }
  } else roots.fill(0);
  const remap = new Map(selected.map((id,index) => [id,selected[root(index)]]));
  const faces = fixture.mesh.faces.map((face:number[]) => face.map(id => remap.get(id) ?? id).filter((id,index,ids) => index === 0 || id !== ids[index-1])).map((face:number[]) => face[0] === face.at(-1) ? face.slice(0,-1) : face).filter((face:number[]) => face.length >= 3);
  let area = 0;
  for (const face of faces) for (const triangle of ShapeUtils.triangulateShape(face.map((id:number) => new Vector2(points[id].x,points[id].y)),[])) {
    const [a,b,c] = triangle.map(index => points[face[index]]); area += new Triangle(a,b,c).getArea();
  }
  expect(new Set(faces.flat()).size).toBe(fixture.vertices); expect(faces.length).toBe(fixture.faces); expect(area).toBeCloseTo(fixture.area,6);
});
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

const inspections = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔎️inspection/🔣️.json", import.meta.url), "utf8"));
const validateInspection = new Ajv().compile(JSON.parse(readFileSync(new URL("../../🧬️schema/🔎️inspection/🔣️.json", import.meta.url), "utf8")));
for (const fixture of inspections.cases) test(`mesh inspection and Three.js oracle: ${fixture.query.kind}`, () => {
  expect(validateInspection(fixture.query)).toBe(true);
  const input = structuredClone(inspections.mesh), result = inspectMeshComponent(input, fixture.query);
  expect(input).toEqual(inspections.mesh);
  for (const [key, expected] of Object.entries(fixture.expected)) {
    if (Array.isArray(expected)) expected.forEach((value, axis) => expect((result[key] as number[])[axis]).toBeCloseTo(value, 6));
    else expect(result[key]).toBe(expected);
  }
  const points = input.vertices.map((point: number[]) => new Vector3(...point as [number, number, number]));
  if (fixture.query.kind === "vertex") expect(result.point).toEqual(points[fixture.query.index].toArray());
  if (fixture.query.kind === "edge") {
    const edge = new Line3(points[fixture.query.index], points[(fixture.query.index + 1) % points.length]);
    expect(result.start).toEqual(edge.start.toArray()); expect(result.end).toEqual(edge.end.toArray()); expect(result.length).toBe(edge.distance());
  }
  if (fixture.query.kind === "face") {
    const center = points.reduce((sum: Vector3, point: Vector3) => sum.add(point), new Vector3()).divideScalar(points.length);
    for (let axis = 0; axis < 3; axis++) expect(result.center![axis]).toBeCloseTo(center.getComponent(axis), 6);
    const normal = points.reduce((sum: Vector3, point: Vector3, index: number) => sum.add(new Vector3().crossVectors(point.clone().sub(points[0]), points[(index + 1) % points.length].clone().sub(points[0]))), new Vector3()).normalize();
    expect(result.normal).toEqual(normal.toArray());
  }
});
test("mesh inspection rejects invalid queries", () => {
  for (const query of inspections.invalid) expect(() => inspectMeshComponent(inspections.mesh, query)).toThrow();
});

const channelContract = JSON.parse(readFileSync(new URL("../../../🧬️schema/🪪️channels/🔣️.json", import.meta.url), "utf8"));
const channelFixtures = JSON.parse(readFileSync(new URL("../../../🧫️fixtures/🪪️channels/🔣️.json", import.meta.url), "utf8"));
test("packaged BRep channels satisfy typed schema and portable identities", () => {
  const descriptor = JSON.parse(readFileSync(new URL("../../../🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv().compile(channelContract);
  for (const topic of descriptor.manifest.topicContributions) {
    const manifest = JSON.parse(topic.payload.manifestJson);
    for (const operator of manifest.contributes.operators) for (const direction of ["inputs", "outputs"]) {
      const channels = operator[direction];
      for (const channel of channels) expect(validate(channel), `${operator.id} ${direction} ${channel.name}: ${JSON.stringify(validate.errors)}`).toBe(true);
      for (const key of ["code", "abbreviation", "name", "fullName"]) expect(new Set(channels.map((channel: Record<string, unknown>) => channel[key])).size, `${operator.id} ${direction} ${key}`).toBe(channels.length);
    }
    for (const fixture of channelFixtures.cases) {
      const operator = manifest.contributes.operators.find((operator: { id: string }) => operator.id === fixture.operator);
      for (const [key, fields] of Object.entries(fixture.expected)) {
        const channel = operator[fixture.direction].find((channel: { name: string }) => channel.name === key);
        for (const [field, expected] of Object.entries(fields as object)) expect(channel[field]).toEqual(expected);
      }
    }
    for (const kind of ["Vertex", "Edge", "Face"]) expect(manifest.contributes.operators.some((operator: { id: string }) => operator.id === `brep.mesh.inspect${kind}`)).toBe(true);
  }
});

const mergedFaces = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🧵️merge-faces/🔣️.json", import.meta.url), "utf8"));
for (const fixture of mergedFaces.cases) test(`merge faces Three.js surface oracle: ${fixture.operation}`, () => {
  const points = mergedFaces.mesh.vertices.map((point: [number, number, number]) => new Vector3(...point));
  const original = mergedFaces.mesh.faces.reduce((area: number, face: number[]) => area + new Triangle(...face.map(id => points[id]) as [Vector3, Vector3, Vector3]).getArea(), 0);
  const triangles = ShapeUtils.triangulateShape(points.map((point: Vector3) => new Vector2(point.x, point.y)), []);
  const merged = triangles.reduce((area, face) => area + new Triangle(...face.map(id => points[id]) as [Vector3, Vector3, Vector3]).getArea(), 0);
  expect(original).toBe(fixture.expected.area); expect(merged).toBe(fixture.expected.area);
  expect(new Set(triangles.flat()).size).toBe(fixture.expected.vertices);
});

test("indexed normal component transforms preserve unselected aliases against Three.js", () => {
  const transformed = transformMeshComponents(attributes.indexedMesh, {mode:"vertex",selection:[0],operation:"scale",vector:[2,1,1],angle:0,pivot:[0,0,0]});
  const normal = transformed.attributes!.normal;
  const matrix = new Matrix4().makeScale(2,1,1).invert().transpose();
  const expected = new Vector3(...attributes.indexedMesh.attributes.normal.values[0]).transformDirection(matrix);
  const sample = normal.values[normal.indices![0]] as number[];
  sample.forEach((value, axis) => expect(value).toBeCloseTo(expected.toArray()[axis],6));
  expect(normal.values[normal.indices![1]]).toEqual(attributes.indexedMesh.attributes.normal.values[0]);
  expect(normal.indices![0]).toBe(normal.indices![3]);expect(normal.indices![0]).not.toBe(normal.indices![1]);
});

test("orientation fixture keeps corner identity distinct from directed edge identity", () => {
  const fixture = attributes.orientation, vertices = attributes.mesh.vertices.map((point: number[]) => new Vector3(...point));
  const normals = fixture.outputFaces.map((face: number[]) => new Triangle(vertices[face[0]], vertices[face[1]], vertices[face[2]]).getNormal(new Vector3()));
  expect(normals[0].dot(normals[1])).toBeCloseTo(1,12);
  const sourceCorners = fixture.inputFaces.flat();
  const sourceEdges = fixture.inputFaces.flatMap((face: number[]) => face.map((a, index) => [a, face[(index + 1) % face.length]]));
  const corners = fixture.outputFaces.flat(), edges = fixture.outputFaces.flatMap((face: number[]) => face.map((a, index) => [a, face[(index + 1) % face.length]]));
  fixture.cornerRemap.forEach((source: number, target: number) => expect(sourceCorners[source]).toBe(corners[target]));
  fixture.directedEdgeRemap.forEach((source: number, target: number) => expect([...sourceEdges[source]].sort()).toEqual([...edges[target]].sort()));
  expect(fixture.cornerRemap).not.toEqual(fixture.directedEdgeRemap);
});

test("retained component neutral vectors match independent selected geometry and normal matrices",()=>{const fixture=JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json",import.meta.url),"utf8")).componentJobs;const source:PolygonMesh={vertices:fixture.positions,faces:fixture.faces,attributes:{normal:{domain:"corner",semantic:"normal",interpolation:"linear",values:[fixture.normal],indices:[0,0,0,0,0,0]}}};for(const entry of fixture.cases){const operation=entry.operation==="move"?"translate":entry.operation,vector=operation==="translate"?[1,0,0]:operation==="rotate"?[0,0,1]:[2,1,1],output=transformMeshComponents(source,{mode:"face",selection:[fixture.selectedFace],operation,vector:vector as [number,number,number],angle:fixture.angle,pivot:[0,0,0]});for(let id=0;id<source.vertices.length;id++){const point=new Vector3(...source.vertices[id]);if(id<3){if(operation==="translate")point.add(new Vector3(...vector));else if(operation==="rotate")point.applyAxisAngle(new Vector3(...vector),Math.fround(fixture.angle));else point.multiply(new Vector3(...vector));}point.toArray().forEach((value,axis)=>{expect(output.vertices[id][axis]).toBeCloseTo(value,6);expect(entry.expected[id][axis]).toBeCloseTo(value,6);});}const area=output.faces.reduce((sum:number,face:number[])=>sum+new Triangle(...face.map(id=>new Vector3(...output.vertices[id])) as [Vector3,Vector3,Vector3]).getArea(),0);expect(area).toBeCloseTo(entry.area,6);if(operation!=="translate"){expect(output.attributes!.normal.indices).toEqual(fixture.normalIndices);const matrix=operation==="rotate"?new Matrix4().makeRotationZ(Math.fround(fixture.angle)):new Matrix4().makeScale(...vector as [number,number,number]).invert().transpose(),normal=new Vector3(...fixture.normal).transformDirection(matrix);(output.attributes!.normal.values[0] as number[]).forEach((value,axis)=>expect(value).toBeCloseTo(normal.toArray()[axis],6));expect(output.attributes!.normal.values[1]).toEqual(fixture.normal);}}});


test("generated channel neutral vectors match Three.js interpolation and actual cut geometry",()=>{const fixture=JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json",import.meta.url),"utf8")).generatedAttributeJobs;const source:PolygonMesh={vertices:fixture.positions,faces:fixture.faces};for(const entry of fixture.cases){const generated=entry.operation==="subdivide"?[fixture.positions.reduce((sum:Vector3,point:number[])=>sum.add(new Vector3(...point)),new Vector3()).divideScalar(4)]:[new Vector3(...fixture.positions[0]).lerp(new Vector3(...fixture.positions[1]),.5),new Vector3(...fixture.positions[2]).lerp(new Vector3(...fixture.positions[3]),.5)];generated.forEach((point:Vector3,index:number)=>{expect(point.toArray()).toEqual(entry.generated[index]);const weights=entry.operation==="subdivide"?[.25,.25,.25,.25]:index===0?[.5,.5,0,0]:[0,0,.5,.5],uv=fixture.uv.reduce((sum:Vector2,value:number[],id:number)=>sum.addScaledVector(new Vector2(...value),weights[id]),new Vector2()),linear=fixture.linear.reduce((sum:number,value:number,id:number)=>sum+value*weights[id],0),normal=weights.reduce((sum:Vector3,weight:number,id:number)=>sum.addScaledVector(new Vector3(...fixture.normal[fixture.normalIndices[id]]),weight),new Vector3()).normalize();expect(uv.toArray()).toEqual(point.toArray().slice(0,2));expect(linear).toBe(entry.generatedLinear[index]);normal.toArray().forEach((value,axis)=>expect(value).toBeCloseTo(fixture.normalMidpoint[axis],12));});if(entry.operation!=="subdivide"){const output=entry.operation==="loopCut"?loopCutMesh(source,[0],1):knifeCutMesh(source,{face:0,start:[.5,-1,0],end:[.5,2,0]});expect(output.vertices.length).toBe(entry.vertices);expect(output.faces.length).toBe(entry.faces);for(const point of generated)expect(output.vertices.some(value=>new Vector3(...value).distanceTo(point)<1e-12)).toBe(true);const area=output.faces.reduce((sum:number,face:number[])=>sum+ShapeUtils.triangulateShape(face.map(id=>new Vector2(output.vertices[id][0],output.vertices[id][1])),[]).reduce((faceArea:number,triangle:number[])=>faceArea+new Triangle(...triangle.map(id=>new Vector3(...output.vertices[face[id]])) as [Vector3,Vector3,Vector3]).getArea(),0),0);expect(area).toBeCloseTo(entry.area,12);}}});


test("retained triangulation neutral shape matches Three.js independent concave triangulation",()=>{const fixture=JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json",import.meta.url),"utf8")).triangulationJob;let area=0,triangles=0;fixture.faces.forEach((face:number[],source:number)=>{const points=face.map(id=>new Vector3(...fixture.positions[id])),result=ShapeUtils.triangulateShape(points.map(point=>new Vector2(point.x,point.y)),[]);expect(result.length).toBe(fixture.sourceFaces[source]);triangles+=result.length;area+=result.reduce((sum:number,ids:number[])=>sum+new Triangle(...ids.map(id=>points[id]) as [Vector3,Vector3,Vector3]).getArea(),0);for(const ids of result)for(const id of ids)expect(fixture.positions[face[id]].slice(0,2)).toEqual([points[id].x,points[id].y]);});expect(triangles).toBe(fixture.triangles);expect(area).toBeCloseTo(fixture.area,12);});


test("retained position policy neutral vectors match Three.js radial distances and surface areas",()=>{const fixture=JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json",import.meta.url),"utf8")).positionJobs;for(const entry of fixture.cases){const selected=new Set(entry.selected),points=entry.positions.map((value:number[],id:number)=>{const point=new Vector3(...value);if(entry.operation==="snap"){if(selected.has(id))point.set(...point.toArray().map(coordinate=>Math.round(coordinate/entry.grid)*entry.grid) as [number,number,number]);}else {const distance=point.distanceTo(new Vector3(...entry.pivot)),weight=selected.has(id)?1:Math.max(0,1-distance/entry.radius);point.addScaledVector(new Vector3(...entry.offset),weight);}point.toArray().forEach((coordinate,axis)=>expect(coordinate).toBeCloseTo(entry.expected[id][axis],12));return point;});expect(new Triangle(...points as [Vector3,Vector3,Vector3]).getArea()).toBeCloseTo(entry.area,12);}});


test("column-major affine neutral vectors match independent Three.js positions, normal matrices and winding",()=>{
  const fixture=JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json",import.meta.url),"utf8")).affineJobs;
  for(const entry of fixture.cases){const matrix=new Matrix4().fromArray(entry.matrix),normalMatrix=matrix.clone().invert().transpose();const points=fixture.positions.map((point:number[],id:number)=>{const result=new Vector3(...point).applyMatrix4(matrix);result.toArray().forEach((value,axis)=>expect(value).toBeCloseTo(entry.positions[id][axis],12));return result;});const normal=new Vector3(...fixture.normal).transformDirection(normalMatrix);normal.toArray().forEach((value,axis)=>expect(value).toBeCloseTo(entry.normal[axis],12));expect(matrix.determinant()<0).toBe(entry.flipped);const triangle=new Triangle(...(entry.flipped?[points[2],points[1],points[0]]:points) as [Vector3,Vector3,Vector3]);expect(triangle.getNormal(new Vector3()).z).toBe(1);expect(triangle.getArea()).toBeCloseTo(entry.area,12);}
});


test("coplanar retained cleanup fixture matches independent Three.js source and boundary areas",()=>{const fixture=JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json",import.meta.url),"utf8")).coplanarJob;const points=fixture.positions.map((point:number[])=>new Vector3(...point));const sourceArea=fixture.faces.reduce((sum:number,face:number[])=>sum+new Triangle(points[face[0]],points[face[1]],points[face[2]]).getArea()+new Triangle(points[face[0]],points[face[2]],points[face[3]]).getArea(),0);const boundary=[0,3,7,4].map(id=>points[id]),triangles=ShapeUtils.triangulateShape(boundary.map(point=>new Vector2(point.x,point.y)),[]);const outputArea=triangles.reduce((sum,ids)=>sum+new Triangle(...ids.map(id=>boundary[id]) as [Vector3,Vector3,Vector3]).getArea(),0);expect(sourceArea).toBe(fixture.area);expect(outputArea).toBe(fixture.area);expect(boundary.length).toBe(fixture.corners);expect(fixture.faces.length-fixture.outputFaces).toBe(fixture.merges);});

test("coplanar authored boundary and face interpolation match independent Three.js",()=>{
  const fixtures=JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json",import.meta.url),"utf8"));
  const fixture=fixtures.coplanarAttributeJob, source=fixtures[fixture.source], points=source.positions.map((point:number[])=>new Vector3(...point));
  const boundary=[0,1,2,3,7,6,5,4].map(id=>points[id]);const triangles=ShapeUtils.triangulateShape(boundary.map(point=>new Vector2(point.x,point.y)),[]);
  const area=triangles.reduce((sum,ids)=>sum+new Triangle(...ids.map(id=>boundary[id])as [Vector3,Vector3,Vector3]).getArea(),0);
  expect(area).toBe(fixture.area);expect(boundary.length).toBe(fixture.corners);
  expect(new Vector3(...fixture.faceValues).dot(new Vector3(1/3,1/3,1/3))).toBeCloseTo(fixture.faceLinear,10);
  expect(Math.min(...source.faces.map((_:unknown,id:number)=>id))).toBe(fixture.faceNearest);
  expect(source.faces.length-fixture.outputFaces).toBe(fixture.merges);
  for(const point of boundary){const uv=new Vector2(point.x,point.y);expect(uv.x).toBe(point.x);expect(uv.y).toBe(point.y);}
});


test("retained dissolution and weld neutral vectors match independent Three.js surfaces and contributors",()=>{
  const cases=JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json",import.meta.url),"utf8")).retainedDissolutionJobs;
  for(const [name,entry] of Object.entries(cases) as [string,any][]){
    for(const source of [entry,{positions:entry.output.positions,faces:entry.output.faces}]){
      const mesh={vertices:source.positions,faces:source.faces};let area=0;
      for(const face of source.faces){const points=face.map((id:number)=>new Vector3(...source.positions[id]));const triangles=ShapeUtils.triangulateShape(points.map((point:Vector3)=>new Vector2(point.x,point.y)),[]);for(const ids of triangles)area+=new Triangle(...ids.map(id=>points[id]) as [Vector3,Vector3,Vector3]).getArea();}
      expect(area).toBeCloseTo(entry.area,12);expect(analyzePolygonMesh(mesh).area).toBeCloseTo(area,12);
    }
    expect(entry.output.positions.length).toBe(entry.vertices);expect(entry.output.faces.length).toBe(entry.outputFaces);expect(entry.output.faces.flat().length).toBe(entry.corners);
    if(name==="weld")for(let vertex=0;vertex<entry.output.positions.length;vertex++){const point=new Vector3(...entry.output.positions[vertex]);const contributors=entry.positions.map((position:[number,number,number],id:number)=>new Vector3(...position).equals(point)?id:-1).filter((id:number)=>id>=0);const total=contributors.reduce((sum:Vector3,id:number)=>sum.add(new Vector3(entry.vertexValues[id],0,0)),new Vector3()).divideScalar(contributors.length);expect(total.x).toBeCloseTo(entry.vertexLinear[vertex],12);expect(Math.min(...contributors)).toBe(entry.vertexNearest[vertex]);}
    if(name==="weld"){const group=entry.largeGroup;const center=new Vector3(...group.output.positions[0]);const contributors=group.positions.map((point:[number,number,number],id:number)=>new Vector3(...point).equals(center)?id:-1).filter((id:number)=>id>=0);expect(contributors.length).toBe(6);const mean=contributors.reduce((sum:Vector3,id:number)=>sum.add(new Vector3(group.vertexValues[id],0,0)),new Vector3()).divideScalar(contributors.length);expect(mean.x).toBe(group.vertexLinear[0]);expect(Math.min(...contributors)).toBe(group.vertexNearest[0]);}
    else{const mean=entry.faceValues.reduce((sum:Vector3,value:number)=>sum.add(new Vector3(value,0,0)),new Vector3()).divideScalar(entry.faceValues.length);expect(mean.x).toBeCloseTo(entry.faceLinear,12);expect(entry.faceNearest).toBe(0);}
  }
});


test("retained decimation directed linear sample fixture matches independent Three.js scalar mapping",()=>{const fixture=attributes.decimatedAttributeJob;for(let id=0;id<fixture.edgeLinearValues.length;id++){expect(new Vector3(id,0,0).divideScalar(fixture.edgeLinearScale).x).toBeCloseTo(fixture.edgeLinearValues[id],12);expect(new Vector3(fixture.edgeSharedLinearValues[fixture.edgeSharedLinearIndices[id]],0,0).x).toBe(id%2===0?2:4);}expect(new Set(fixture.edgeSharedLinearIndices).size).toBe(2);});

test("canonical tangent affine neutral basis matches independent Three.js direction and determinant",()=>{const c=JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json",import.meta.url),"utf8")).tangentJobs;const m=new Matrix4().fromArray(c.matrix);const direction=new Vector3(c.tangent[0],c.tangent[1],c.tangent[2]).transformDirection(m);for(let axis=0;axis<3;axis++)expect(direction.getComponent(axis)).toBeCloseTo(c.expected[axis],12);expect(c.tangent[3]*Math.sign(m.determinant())).toBe(c.expected[3]);});

test("canonical tangent parser and selected aliases obey schema and independent Three.js",()=>{const c=JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json",import.meta.url),"utf8")).tangentJobs;const validate=new Ajv().compile(JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🏗️mesh-engine/🧬️schema/🥽️polygon/🔣️.json",import.meta.url),"utf8")));for(const domain of ["vertex","corner","face"] as const){const count=domain==="vertex"?c.positions.length:domain==="face"?c.faces.length:c.faces.flat().length;const mesh:PolygonMesh={vertices:c.positions,faces:c.faces,attributes:{tangent:{domain,semantic:"custom",interpolation:"linear",values:[c.tangent],indices:Array(count).fill(0)}}};expect(validate(mesh)).toBe(true);expect(parsePolygonMesh(JSON.stringify(mesh))).toEqual(mesh);for(const value of [[0,0,0,1],[1,0,0,0],[1,0,0],[1,0,0,2]]){const invalid=structuredClone(mesh);invalid.attributes!.tangent.values=[value];expect(validate(invalid)).toBe(false);expect(()=>parsePolygonMesh(JSON.stringify(invalid))).toThrow();}const request={mode:"vertex",selection:[0],operation:"scale",vector:[-2,1,3],angle:0,pivot:[0,0,0]} as const;if(domain==="face"){expect(()=>transformMeshComponents(mesh,request as any)).toThrow();continue;}const result=transformMeshComponents(mesh,request as any);for(let id=0;id<count;id++){const affected=domain==="vertex"?id===0:c.faces.flat()[id]===0;const attribute=result.attributes!.tangent;const value=attribute.values[attribute.indices![id]] as number[];expect(value.length).toBe(4);for(let axis=0;axis<4;axis++)expect(value[axis]).toBeCloseTo((affected?c.expected:c.tangent)[axis],6);}expect(mesh.attributes!.tangent.values).toEqual([c.tangent]);}});

test("selected face flip neutral winding preserves Three.js area",()=>{const c=JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json",import.meta.url),"utf8")).flipJob;const faces=c.faces.map((face:number[],id:number)=>c.selection.includes(id)?face.toReversed():face);expect(faces).toEqual(c.outputFaces);const points=c.positions.map((point:number[])=>new Vector3(...point));expect(faces.reduce((sum:number,face:number[])=>sum+new Triangle(...face.map(id=>points[id]) as [Vector3,Vector3,Vector3]).getArea(),0)).toBe(c.area);});

test("retained OBJ text neutral fixture parses with independent Three.js UV and area",()=>{const c=JSON.parse(readFileSync(new URL("../../🧫️fixtures/⏱️output-budget/🔣️.json",import.meta.url),"utf8")).objExport;const object=new OBJLoader().parse(c.expected);const geometry=(object.children[0] as any).geometry;expect(Array.from(geometry.attributes.position.array)).toEqual(c.mesh.vertices.flat());expect(Array.from(geometry.attributes.uv.array)).toEqual(c.mesh.attributes.uv.values.flat());const points=c.mesh.vertices.map((point:number[])=>new Vector3(...point));expect(new Triangle(...points as [Vector3,Vector3,Vector3]).getArea()).toBe(c.area);});

test("retained primitive neutral admission matches independent Three.js geometry",()=>{const cases=JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json",import.meta.url),"utf8")).primitiveJobs;for(const c of cases){const [a,b,d]=c.dimensions;const geometry=c.kind==="box"?new BoxGeometry(a,b,d):c.kind==="plane"?new PlaneGeometry(a,b):c.kind==="cylinder"?new CylinderGeometry(a,a,b,c.parameter):c.kind==="cone"?new ConeGeometry(a,b,c.parameter):new IcosahedronGeometry(a,2**c.parameter-1);const position=geometry.attributes.position;const indices=geometry.index?Array.from(geometry.index.array):Array.from({length:position.count},(_,i)=>i);let area=0;for(let i=0;i<indices.length;i+=3){const points=indices.slice(i,i+3).map(id=>new Vector3().fromBufferAttribute(position,id));area+=new Triangle(...points as [Vector3,Vector3,Vector3]).getArea();}expect(area).toBeGreaterThan(0);if(c.kind==="sphere"){expect(indices.length/3).toBe(c.faces);const unique=new Set(Array.from({length:position.count},(_,i)=>[position.getX(i),position.getY(i),position.getZ(i)].map(value=>Math.round(value*1e5)).join(",")));expect(unique.size).toBe(c.vertices);}if(c.kind==="box")expect(area).toBe(2*(a*b+a*d+b*d));if(c.kind==="plane")expect(area).toBe(a*b);geometry.dispose();}});


test("faceted BRep neutral import matches independent Three.js planar triangle area",()=>{const c=outputBudget.toBrep;const points=c.mesh.vertices.map((point:number[])=>new Vector3(...point));let area=0,triangles=0;for(const face of c.mesh.faces){const projected=face.map((id:number)=>new Vector2(points[id].x,points[id].y));for(const triangle of ShapeUtils.triangulateShape(projected,[])){triangles++;area+=new Triangle(...triangle.map(id=>points[face[id]]) as [Vector3,Vector3,Vector3]).getArea();}}expect(triangles).toBe(c.triangles);expect(area).toBeCloseTo(c.area,12);expect(c.quality).toBe("MeshDerivedBRep");});


test("reserved tangent schema refuses f32 overflow for every sampling policy",()=>{const c=JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json",import.meta.url),"utf8")).tangentJobs;const validate=new Ajv().compile(JSON.parse(readFileSync(new URL("../../../../../../../../🧰️framework/🔨️modules/🏗️mesh-engine/🧬️schema/🥽️polygon/🔣️.json",import.meta.url),"utf8")));for(const interpolation of ["nearest","constant","linear"]){const source={vertices:c.positions,faces:c.faces,attributes:{tangent:{domain:"vertex",semantic:"custom",interpolation,values:[[1e40,0,0,1]],indices:Array(c.positions.length).fill(0)}}};expect(()=>parsePolygonMesh(JSON.stringify(source))).toThrow();expect(validate(source)).toBe(false);}});

test("retained source preparation neutral translated bounds match Three.js",()=>{const c=JSON.parse(readFileSync(new URL("../../🧫️fixtures/⏱️output-budget/🔣️.json",import.meta.url),"utf8")).inputPreparation;const geometry=new BoxGeometry(1,1,1).translate(...c.offset as [number,number,number]);geometry.computeBoundingBox();expect(geometry.boundingBox!.min.toArray()).toEqual(c.minimum);expect(geometry.boundingBox!.max.toArray()).toEqual(c.maximum);let area=0;const positions=geometry.attributes.position;const indices=geometry.index!.array;for(let i=0;i<indices.length;i+=3){area+=new Triangle(...Array.from(indices.slice(i,i+3)).map(id=>new Vector3().fromBufferAttribute(positions,id)) as [Vector3,Vector3,Vector3]).getArea();}expect(area).toBe(c.area);geometry.dispose();});


test("closed faceted BRep neutral fixture matches Three.js surface area and signed volume",()=>{const c=outputBudget.toBrep,points=c.closedMesh.vertices.map((point:number[])=>new Vector3(...point));let area=0,volume=0;for(const face of c.closedMesh.faces){const [a,b,d]=face.map((id:number)=>points[id]);area+=new Triangle(a,b,d).getArea();volume+=a.dot(new Vector3().crossVectors(b,d))/6;}expect(c.closedMesh.faces.length).toBe(c.closedTriangles);expect(area).toBeCloseTo(c.closedArea,12);expect(volume).toBeCloseTo(c.closedVolume,12);expect(c.closedKind).toBe("solid");});

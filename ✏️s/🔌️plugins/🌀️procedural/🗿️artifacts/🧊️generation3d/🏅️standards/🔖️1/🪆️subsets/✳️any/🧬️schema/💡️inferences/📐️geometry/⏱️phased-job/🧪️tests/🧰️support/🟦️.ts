/** 🧰️ Shared helpers of the third-party oracle tests of the O3 B-Rep categories: they read the pinned tessellations of a fixture and recompute volume, area, bounds, closedness and transform matrices with `three`, independent of the Rust kernel that produced them. */
import { readFileSync } from "node:fs";
import { Box3, BufferAttribute, BufferGeometry, Matrix4, Triangle, Vector3 } from "three";

export type PinnedMesh = { deflection: number; positions: number[]; indices: number[] };
export type ShapeExpectation = {
  kind?: string;
  volume?: number;
  area?: number;
  bbox?: [number[], number[]];
  matrix?: number[];
  closed?: boolean;
  mesh?: PinnedMesh;
  pin?: boolean;
};
export type BrepCase = {
  name: string;
  kind: string;
  inputs: Record<string, unknown>;
  outputs?: Record<string, unknown>;
  fault?: { code: string; port?: string };
  tolerance?: number;
  meshTolerance?: number;
  inputMeshes?: Record<string, PinnedMesh>;
};
export type BrepFixture = { tolerance: number; meshTolerance?: number; cases: BrepCase[] };

export type Measure = { volume: number; area: number; min: number[]; max: number[]; triangles: number; boundaryEdges: number; nonManifoldEdges: number };

/** 🔺️ The BufferGeometry of a pinned mesh. */
export const geometryOf = (mesh: PinnedMesh): BufferGeometry => {
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(mesh.positions), 3));
  geometry.setIndex(new BufferAttribute(new Uint32Array(mesh.indices), 1));
  return geometry;
};

const weldKey = (point: Vector3): string => [point.x, point.y, point.z].map(value => Math.round(value * 1e5)).join(",");

/** 📏️ Volume (divergence theorem), area (three's Triangle), bounds (three's Box3) and edge use of a pinned mesh; coincident corners weld at 1e-5. */
export const measure = (mesh: PinnedMesh): Measure => {
  const geometry = geometryOf(mesh);
  const position = geometry.getAttribute("position");
  const corners = (index: number) => new Vector3().fromBufferAttribute(position, mesh.indices[index]!);
  const box = new Box3().setFromBufferAttribute(position as BufferAttribute);
  let volume = 0;
  let area = 0;
  const uses = new Map<string, number>();
  for (let first = 0; first + 2 < mesh.indices.length; first += 3) {
    const [a, b, c] = [corners(first), corners(first + 1), corners(first + 2)];
    area += new Triangle(a, b, c).getArea();
    volume += a.dot(new Vector3().crossVectors(b, c)) / 6;
    const keys = [weldKey(a), weldKey(b), weldKey(c)];
    for (const [from, to] of [[0, 1], [1, 2], [2, 0]] as const) {
      const edge = keys[from]! < keys[to]! ? `${keys[from]}|${keys[to]}` : `${keys[to]}|${keys[from]}`;
      uses.set(edge, (uses.get(edge) ?? 0) + 1);
    }
  }
  const counts = [...uses.values()];
  return {
    volume,
    area,
    min: [box.min.x, box.min.y, box.min.z],
    max: [box.max.x, box.max.y, box.max.z],
    triangles: mesh.indices.length / 3,
    boundaryEdges: counts.filter(count => count === 1).length,
    nonManifoldEdges: counts.filter(count => count > 2).length,
  };
};

const close = (actual: number, expected: number, tolerance: number): boolean => Number.isFinite(actual) && Math.abs(actual - expected) <= tolerance * Math.max(1, Math.abs(expected));

/** 🔎️ The corners of a mesh after a row-major 4x4 matrix, welded and sorted so two meshes of the same surface compare regardless of triangle order. */
export const transformedCorners = (mesh: PinnedMesh, rowMajor: number[]): string[] => {
  const matrix = new Matrix4().set(...(rowMajor as Parameters<Matrix4["set"]>));
  const keys = new Set<string>();
  const position = geometryOf(mesh).getAttribute("position");
  for (let index = 0; index < position.count; index += 1) keys.add(weldKey(new Vector3().fromBufferAttribute(position, index).applyMatrix4(matrix)));
  return [...keys].sort();
};

/** 🔎️ The welded, sorted corners of a mesh as they are. */
export const corners = (mesh: PinnedMesh): string[] => transformedCorners(mesh, [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]);

/** 🧪️ Every disagreement between the fixture's analytic numbers and `three`'s reading of the pinned mesh of one shape output. */
export const shapeDisagreements = (label: string, expected: ShapeExpectation, tolerance: number, input?: PinnedMesh): string[] => {
  const mesh = expected.mesh;
  if (!mesh) return [];
  const failures: string[] = [];
  const read = measure(mesh);
  if (expected.volume !== undefined && !close(read.volume, expected.volume, tolerance)) failures.push(`${label}: three reads volume ${read.volume}, the fixture states ${expected.volume}`);
  if (expected.area !== undefined && !close(read.area, expected.area, tolerance)) failures.push(`${label}: three reads area ${read.area}, the fixture states ${expected.area}`);
  if (expected.bbox) {
    const [low, high] = expected.bbox;
    for (let axis = 0; axis < 3; axis += 1) {
      if (!close(read.min[axis]!, low![axis]!, tolerance) || !close(read.max[axis]!, high![axis]!, tolerance)) failures.push(`${label}: three reads bounds ${read.min}..${read.max}, the fixture states ${low}..${high}`);
    }
  }
  if ((expected.kind === "solid" || expected.kind === "compound") && (read.boundaryEdges > 0 || read.nonManifoldEdges > 0)) failures.push(`${label}: the surface has ${read.boundaryEdges} boundary and ${read.nonManifoldEdges} non-manifold edges`);
  if (expected.matrix) {
    if (!input) failures.push(`${label}: a matrix expectation needs the pinned input mesh`);
    else if (JSON.stringify(transformedCorners(input, expected.matrix)) !== JSON.stringify(corners(mesh))) failures.push(`${label}: the pinned output is not the pinned input under the stated matrix`);
  }
  return failures;
};

/** 🧪️ Runs the shape expectations of every case of a fixture and returns all disagreements. */
export const fixtureDisagreements = (fixture: BrepFixture): string[] => {
  const failures: string[] = [];
  const tolerance = fixture.meshTolerance ?? fixture.tolerance;
  for (const fixtureCase of fixture.cases) {
    for (const [port, value] of Object.entries(fixtureCase.outputs ?? {})) {
      const items = Array.isArray(value) ? value : [value];
      items.forEach((item, index) => {
        if (item === null || typeof item !== "object") return;
        failures.push(...shapeDisagreements(`${fixtureCase.name}.${port}[${index}]`, item as ShapeExpectation, fixtureCase.meshTolerance ?? tolerance, fixtureCase.inputMeshes?.shape));
      });
    }
  }
  return failures;
};

/** 📚️ The fixture of a category folder, read from `🧫️fixtures/🔣️.json` beside the test folder. */
export const loadFixture = (testFile: string): BrepFixture => JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", testFile), "utf8")) as BrepFixture;

/** 🔎️ The cases of a fixture for one catalogue kind. */
export const casesOf = (fixture: BrepFixture, kind: string): BrepCase[] => fixture.cases.filter(fixtureCase => fixtureCase.kind === kind);

/** 🔢️ Whether two numbers agree with the fixture tolerance: absolute below magnitude one, relative above. */
export const agree = (actual: number, expected: number, tolerance: number): boolean => close(actual, expected, tolerance);

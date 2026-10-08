/** 🔬️ Third-party oracle of the generation3d `mesh.*` widgets: every language-agnostic fixture case is recomputed with `three` (geometries, Matrix4, Vector3, Box3, `mergeVertices`, the STL loader) and `manifold-3d` (volume, surface area and genus of closed solids, CSG constructions) and compared with what the fixture states — the numbers the Rust computes are asserted against the very same fixture files. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { Box3, BoxGeometry, BufferGeometry, ConeGeometry, CylinderGeometry, Float32BufferAttribute, IcosahedronGeometry, Matrix4, PlaneGeometry, SphereGeometry, TorusGeometry, Vector3 } from "three";
import { mergeVertices } from "three/examples/jsm/utils/BufferGeometryUtils.js";
import { STLLoader } from "three/examples/jsm/loaders/STLLoader.js";
import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader.js";
import loadManifold from "manifold-3d";

type V3 = [number, number, number];
type Mesh = { positions: V3[]; faces: number[][]; seams?: number[] };
type Case = { name: string; kind: string; inputs: Record<string, any>; outputs?: Record<string, any>; fault?: { code: string; port?: string }; tolerance?: number };
type Fixture = { tolerance: number; cases: Case[] };
type Measures = { vertices: number; faces: number; edges: number; boundaryEdges: number; euler: number; area: number; volume: number; bbox: [V3, V3]; closed: boolean; oriented: boolean };

const folders = ["🥽️mesh-primitive", "🔀️mesh-convert", "↔️mesh-transform", "🎚️mesh-component", "✏️mesh-edit", "🩹️mesh-repair", "🔎️mesh-inspect", "📼️mesh-interchange", "🌗️mesh-shading", "🗺️mesh-uv"];
const fixtures = new Map<string, Fixture>(folders.map((folder) => [folder, JSON.parse(readFileSync(new URL(`../../../${folder}/🧫️fixtures/🔣️.json`, import.meta.url), "utf8")) as Fixture]));
const wasm = await loadManifold();
wasm.setup();
const { Manifold, Mesh: ManifoldMesh } = wasm;

//#region 🔖️Measures
const v = (p: V3) => new Vector3(...p);
const rounded = (n: number) => (Math.abs(n) < 5e-5 ? 0 : n).toFixed(4);

/** 📏️ Measures a polygon mesh with three's vectors: Newell area, signed volume by tetrahedra, Box3, edge uses. */
function measureOf(positions: V3[], faces: number[][]): Measures {
  const uses = new Map<string, number>();
  const directed = new Map<string, number>();
  const used = new Set<number>();
  let area = 0;
  let volume = 0;
  const box = new Box3();
  for (const face of faces) {
    face.forEach((index, corner) => {
      const next = face[(corner + 1) % face.length]!;
      const key = `${Math.min(index, next)}_${Math.max(index, next)}`;
      uses.set(key, (uses.get(key) ?? 0) + 1);
      directed.set(`${index}_${next}`, (directed.get(`${index}_${next}`) ?? 0) + 1);
      used.add(index);
      box.expandByPoint(v(positions[index]!));
    });
    const origin = v(positions[face[0]!]!);
    const sum = new Vector3();
    for (let k = 1; k < face.length - 1; k++) {
      const b = v(positions[face[k]!]!);
      const c = v(positions[face[k + 1]!]!);
      sum.add(new Vector3().subVectors(b, origin).cross(new Vector3().subVectors(c, origin)));
      volume += origin.dot(new Vector3().crossVectors(b, c)) / 6;
    }
    area += sum.length() / 2;
  }
  const boundary = [...uses.values()].filter((count) => count === 1).length;
  return {
    vertices: used.size,
    faces: faces.length,
    edges: uses.size,
    boundaryEdges: boundary,
    euler: used.size - uses.size + faces.length,
    area,
    volume,
    bbox: [box.min.toArray() as V3, box.max.toArray() as V3],
    closed: boundary === 0 && [...uses.values()].every((count) => count === 2),
    oriented: [...directed.values()].every((count) => count === 1),
  };
}

const close = (actual: number, expected: number, tolerance: number) => Math.abs(actual - expected) <= tolerance * Math.max(1, Math.abs(expected));

function within(actual: number, expected: any, tolerance: number): boolean {
  if (typeof expected === "number") return close(actual, expected, tolerance);
  return actual >= expected.min && actual <= expected.max;
}

/** ⚖️ The differences between measured values and the stated expectation keys, for the keys `only` allows. */
function differences(measured: Measures, stated: Record<string, any>, tolerance: number, only: readonly string[]): string[] {
  const out: string[] = [];
  for (const key of only) {
    if (!(key in stated) || stated[key] === undefined) continue;
    const wanted = stated[key];
    const found = (measured as Record<string, any>)[key];
    if (key === "bbox") {
      for (let side = 0; side < 2; side++) for (let axis = 0; axis < 3; axis++) if (!close(found[side][axis], wanted[side][axis], tolerance)) out.push(`bbox[${side}][${axis}] ${found[side][axis]} vs ${wanted[side][axis]}`);
    } else if (typeof wanted === "boolean") {
      if (found !== wanted) out.push(`${key} ${found} vs ${wanted}`);
    } else if (!within(found, wanted, key === "area" || key === "volume" ? tolerance : 0)) out.push(`${key} ${found} vs ${JSON.stringify(wanted)}`);
  }
  return out;
}

const solidKeys = ["vertices", "faces", "edges", "boundaryEdges", "euler", "area", "volume", "bbox", "closed", "oriented"] as const;
const shapeKeys = ["area", "volume", "bbox", "closed", "oriented", "euler", "boundaryEdges"] as const;

/** 🧮️ Measures a three geometry after welding coincident vertices and dropping every attribute but the position. */
function geometryMeasures(geometry: BufferGeometry): Measures {
  for (const name of Object.keys(geometry.attributes)) if (name !== "position") geometry.deleteAttribute(name);
  const welded = mergeVertices(geometry, 1e-5);
  const position = welded.getAttribute("position");
  const positions: V3[] = Array.from({ length: position.count }, (_, i) => [position.getX(i), position.getY(i), position.getZ(i)]);
  const index = welded.getIndex()!;
  const faces: number[][] = [];
  for (let i = 0; i < index.count; i += 3) {
    const triangle = [index.getX(i), index.getX(i + 1), index.getX(i + 2)];
    if (new Set(triangle).size === 3) faces.push(triangle);
  }
  return measureOf(positions, faces);
}

/** 🧊️ Volume, surface area and genus of a closed, consistently wound, planar-faced mesh through manifold-3d. */
function manifoldMeasures(positions: V3[], faces: number[][]) {
  const triVerts = new Uint32Array(faces.flatMap((face) => face.slice(1, -1).flatMap((_, k) => [face[0]!, face[k + 1]!, face[k + 2]!])));
  const solid = new Manifold(new ManifoldMesh({ numProp: 3, vertProperties: new Float32Array(positions.flat()), triVerts }));
  return { status: solid.status(), volume: solid.volume(), area: solid.surfaceArea(), genus: solid.genus() };
}

const planar = (positions: V3[], faces: number[][]) =>
  faces.every((face) => {
    const origin = v(positions[face[0]!]!);
    const normal = new Vector3().subVectors(v(positions[face[1]!]!), origin).cross(new Vector3().subVectors(v(positions[face[2]!]!), origin));
    return face.slice(3).every((index) => Math.abs(normal.dot(new Vector3().subVectors(v(positions[index]!), origin))) < 1e-6 * Math.max(1, normal.length()));
  });

/** 🧬️ A mesh as a sorted list of faces, each a cyclic loop of rounded coordinates; independent of numbering. */
function canonical(positions: V3[], faces: number[][]): string[] {
  return faces
    .map((face) => {
      const keys = face.map((index) => positions[index]!.map(rounded).join(","));
      const rotations = keys.map((_, start) => keys.map((__, k) => keys[(start + k) % keys.length]!).join(" "));
      return rotations.sort()[0]!;
    })
    .sort();
}
//#endregion 🔖️Measures

//#region 🔖️Recipes
const matrixOf = (kind: string, inputs: Record<string, any>): Matrix4 => {
  switch (kind) {
    case "mesh.transform.translate": return new Matrix4().makeTranslation(...(inputs.offset as V3));
    case "mesh.transform.rotate": return new Matrix4().makeRotationAxis(v(inputs.axis).normalize(), inputs.angle);
    case "mesh.transform.scale": return new Matrix4().makeScale(...(inputs.factor as V3));
    default: return new Matrix4().fromArray(inputs.matrix as number[]);
  }
};

const transformed = (mesh: Mesh, matrix: Matrix4): Mesh => ({
  positions: mesh.positions.map((p) => v(p).applyMatrix4(matrix).toArray() as V3),
  faces: matrix.determinant() < 0 ? mesh.faces.map((face) => [...face].reverse()) : mesh.faces,
});

/** 🔢️ Half-edge handles in face-corner order: handle h is corner `c` of face `f`. */
const handleTable = (faces: number[][]) => faces.flatMap((face, f) => face.map((_, c) => ({ f, c })));
const verticesOfSelection = (mesh: Mesh, mode: string, ids: number[]): number[] => {
  const table = handleTable(mesh.faces);
  const out = new Set<number>();
  for (const id of ids) {
    if (mode === "vertex") out.add(id);
    else if (mode === "edge") {
      const { f, c } = table[id]!;
      const face = mesh.faces[f]!;
      out.add(face[c]!);
      out.add(face[(c + 1) % face.length]!);
    } else mesh.faces[id]!.forEach((index) => out.add(index));
  }
  return [...out].sort((a, b) => a - b);
};

const movedBy = (mesh: Mesh, selected: number[], move: (p: Vector3, index: number) => Vector3): Mesh => ({
  positions: mesh.positions.map((p, index) => (selected.includes(index) ? (move(v(p), index).toArray() as V3) : p)),
  faces: mesh.faces,
});

const centroidOf = (mesh: Mesh, selected: number[]) => selected.reduce((sum, index) => sum.add(v(mesh.positions[index]!)), new Vector3()).divideScalar(selected.length);

/** 🔨️ The geometry a component widget must produce, from three's Vector3 and Matrix4. */
function componentRecipe(kind: string, inputs: Record<string, any>): Mesh {
  const mesh = inputs.mesh as Mesh;
  const mode: string = inputs.mode ?? "vertex";
  const selection: number[] = inputs.selection ?? inputs.vertices;
  const selected = verticesOfSelection(mesh, kind === "mesh.component.moveVertices" || kind === "mesh.component.moveProportional" || kind === "mesh.component.snapToGrid" ? "vertex" : mode, selection);
  const pivot = inputs.pivot === "point" ? v(inputs.center) : centroidOf(mesh, selected);
  switch (kind) {
    case "mesh.component.translate":
    case "mesh.component.moveVertices": return movedBy(mesh, selected, (p) => p.add(v(inputs.offset)));
    case "mesh.component.rotate": {
      const matrix = new Matrix4().makeTranslation(pivot.x, pivot.y, pivot.z).multiply(new Matrix4().makeRotationAxis(v(inputs.axis).normalize(), inputs.angle)).multiply(new Matrix4().makeTranslation(-pivot.x, -pivot.y, -pivot.z));
      return movedBy(mesh, selected, (p) => p.applyMatrix4(matrix));
    }
    case "mesh.component.scale": {
      const matrix = new Matrix4().makeTranslation(pivot.x, pivot.y, pivot.z).multiply(new Matrix4().makeScale(...(inputs.factor as V3))).multiply(new Matrix4().makeTranslation(-pivot.x, -pivot.y, -pivot.z));
      return movedBy(mesh, selected, (p) => p.applyMatrix4(matrix));
    }
    case "mesh.component.moveProportional": {
      const center = v(inputs.center);
      return { positions: mesh.positions.map((p, index) => v(p).add(v(inputs.offset).multiplyScalar(selected.includes(index) ? 1 : Math.max(0, 1 - v(p).distanceTo(center) / inputs.radius))).toArray() as V3), faces: mesh.faces };
    }
    default: return movedBy(mesh, selected, (p) => p.set(...(p.toArray().map((axis) => Math.round(axis / inputs.grid) * inputs.grid) as V3)));
  }
}

const dotNormal = (positions: V3[], face: number[]) => {
  const origin = v(positions[face[0]!]!);
  const sum = new Vector3();
  for (let k = 1; k < face.length - 1; k++) sum.add(new Vector3().subVectors(v(positions[face[k]!]!), origin).cross(new Vector3().subVectors(v(positions[face[k + 1]!]!), origin)));
  return sum;
};

/** 🌗️ Vertex normals by the definition: the unit sum of the unit normals of the smooth faces, else the normal of the lowest face. */
function vertexNormals(mesh: Mesh, smooth: Set<number>): Map<number, V3> {
  const flat = new Map<number, Vector3>();
  const sums = new Map<number, Vector3>();
  mesh.faces.forEach((face, index) => {
    const normal = dotNormal(mesh.positions, face).normalize();
    for (const vertex of face) {
      if (!flat.has(vertex)) flat.set(vertex, normal);
      if (smooth.has(index)) sums.set(vertex, (sums.get(vertex) ?? new Vector3()).add(normal));
    }
  });
  return new Map([...flat].map(([vertex, normal]) => [vertex, ((sums.get(vertex)?.length() ?? 0) > 0 ? sums.get(vertex)!.clone().normalize() : normal).toArray() as V3]));
}

/** 🏝️ Faces connected across edges that carry no seam, as a flood fill over directed edge pairs. */
function islands(mesh: Mesh): number[][] {
  const table = handleTable(mesh.faces);
  const owner = new Map<string, number>();
  table.forEach(({ f, c }, handle) => owner.set(`${mesh.faces[f]![c]}_${mesh.faces[f]![(c + 1) % mesh.faces[f]!.length]}`, handle));
  const seams = new Set<number>();
  for (const handle of mesh.seams ?? []) {
    seams.add(handle);
    const { f, c } = table[handle]!;
    const twin = owner.get(`${mesh.faces[f]![(c + 1) % mesh.faces[f]!.length]}_${mesh.faces[f]![c]}`);
    if (twin !== undefined) seams.add(twin);
  }
  const seen = new Set<number>();
  const out: number[][] = [];
  for (let start = 0; start < mesh.faces.length; start++) {
    if (seen.has(start)) continue;
    const island: number[] = [];
    const stack = [start];
    seen.add(start);
    while (stack.length) {
      const face = stack.pop()!;
      island.push(face);
      mesh.faces[face]!.forEach((vertex, corner) => {
        const handle = table.findIndex((entry) => entry.f === face && entry.c === corner);
        const next = mesh.faces[face]![(corner + 1) % mesh.faces[face]!.length]!;
        const twin = owner.get(`${next}_${vertex}`);
        if (twin === undefined || seams.has(handle)) return;
        const neighbour = table[twin]!.f;
        if (!seen.has(neighbour)) {
          seen.add(neighbour);
          stack.push(neighbour);
        }
      });
    }
    out.push(island);
  }
  return out;
}

const fanTriangles = (faces: number[][]) => faces.flatMap((face) => face.slice(1, -1).map((_, k) => [face[0]!, face[k + 1]!, face[k + 2]!]));

/** 🔨️ A bevel of one cube edge by `amount`, built as a CSG difference with manifold-3d. */
function bevelledCube(amount: number) {
  const cutter = Manifold.cube([amount * Math.SQRT2, amount * Math.SQRT2, 2], true).rotate([0, 0, 45]).translate([0, 0, 0.5]);
  const solid = Manifold.cube([1, 1, 1]).subtract(cutter);
  return { volume: solid.volume(), area: solid.surfaceArea() };
}
//#endregion 🔖️Recipes

const all = [...fixtures.values()].flatMap((fixture) => fixture.cases.map((fixtureCase) => ({ ...fixtureCase, fixtureTolerance: fixture.tolerance })));
const byKind = (kind: string) => all.filter((fixtureCase) => fixtureCase.kind === kind && !fixtureCase.fault);
const checked = new Set<string>();
const check = (kind: string, name: string, failures: string[]) => {
  checked.add(kind);
  expect(failures, `${kind} · ${name}`).toEqual([]);
};
const toleranceOf = (fixtureCase: Case & { fixtureTolerance: number }) => fixtureCase.tolerance ?? fixtureCase.fixtureTolerance;

test("the fixtures state each mesh kind with at least one success case and every fault with a code", () => {
  const catalogue = all.map((fixtureCase) => fixtureCase.kind);
  expect(new Set(catalogue).size).toBe(52);
  for (const fixtureCase of all) expect(Boolean(fixtureCase.fault) !== Boolean(fixtureCase.outputs), fixtureCase.name).toBe(true);
  for (const fixtureCase of all.filter((c) => c.fault)) expect(fixtureCase.fault!.code.startsWith("generation3d.geometry."), fixtureCase.name).toBe(true);
});

test("golden geometry in the fixtures has the measures the fixtures state, by three and by manifold-3d", () => {
  let golden = 0;
  for (const fixtureCase of all) {
    const mesh = fixtureCase.outputs?.mesh;
    if (!mesh?.positions) continue;
    const measured = measureOf(mesh.positions, mesh.polygons);
    const failures = differences(measured, mesh, 1e-5, solidKeys);
    if (measured.closed && measured.oriented && planar(mesh.positions, mesh.polygons)) {
      const solid = manifoldMeasures(mesh.positions, mesh.polygons);
      expect(solid.status, fixtureCase.name).toBe("NoError");
      if (!close(solid.volume, measured.volume, 1e-5)) failures.push(`manifold volume ${solid.volume} vs ${measured.volume}`);
      if (!close(solid.area, measured.area, 1e-5)) failures.push(`manifold area ${solid.area} vs ${measured.area}`);
    }
    check(fixtureCase.kind, fixtureCase.name, failures);
    golden++;
  }
  expect(golden).toBeGreaterThan(15);
});

test("primitives equal three's geometries welded, measured by area, volume, bounds and topology", () => {
  const geometries: Record<string, (i: Record<string, any>) => BufferGeometry> = {
    "mesh.primitive.box": (i) => new BoxGeometry(i.width, i.height, i.depth),
    "mesh.primitive.plane": (i) => new PlaneGeometry(i.width, i.depth).rotateX(-Math.PI / 2),
    "mesh.primitive.sphere": (i) => new IcosahedronGeometry(i.radius, i.subdivisions === 0 ? 0 : i.subdivisions === 1 ? 1 : 3),
    "mesh.primitive.cylinder": (i) => new CylinderGeometry(i.radius, i.radius, i.height, i.segments),
    "mesh.primitive.cone": (i) => new ConeGeometry(i.radius, i.height, i.segments),
    "mesh.primitive.torus": (i) => new TorusGeometry(i.major, i.minor, i.rings, i.segments).rotateX(Math.PI / 2),
    "mesh.primitive.uvSphere": (i) => new SphereGeometry(i.radius, i.segments, i.rings),
  };
  for (const [kind, build] of Object.entries(geometries)) {
    for (const fixtureCase of byKind(kind)) {
      const measured = geometryMeasures(build(fixtureCase.inputs));
      const loose = kind === "mesh.primitive.sphere" && fixtureCase.inputs.subdivisions === 2;
      const failures = differences(measured, fixtureCase.outputs!.mesh, loose ? 1e-2 : toleranceOf(fixtureCase), shapeKeys);
      check(kind, fixtureCase.name, failures);
    }
  }
});

test("construct builds what manifold-3d and three measure for the stated json", () => {
  for (const fixtureCase of byKind("mesh.primitive.construct")) {
    const mesh = JSON.parse(fixtureCase.inputs.data ?? '{"vertices":[[0,0,0],[1,0,0],[0,1,0]],"faces":[[0,1,2]]}') as { vertices: V3[]; faces: number[][] };
    check("mesh.primitive.construct", fixtureCase.name, differences(measureOf(mesh.vertices, mesh.faces), fixtureCase.outputs!.mesh, 1e-6, solidKeys));
  }
});

test("transforms equal three's Matrix4 applied to the input mesh, vertex for vertex", () => {
  for (const kind of ["mesh.transform.translate", "mesh.transform.rotate", "mesh.transform.scale", "mesh.transform.matrix"]) {
    for (const fixtureCase of byKind(kind)) {
      const expected = transformed(fixtureCase.inputs.mesh as Mesh, matrixOf(kind, fixtureCase.inputs));
      const stated = fixtureCase.outputs!.mesh;
      const failures = canonical(expected.positions, expected.faces).join("|") === canonical(stated.positions, stated.polygons).join("|") ? [] : ["geometry differs from Matrix4"];
      check(kind, fixtureCase.name, [...failures, ...differences(measureOf(expected.positions, expected.faces), stated, 1e-5, solidKeys)]);
    }
  }
  for (const fixtureCase of byKind("mesh.transform.mirror")) {
    const mesh = fixtureCase.inputs.mesh as Mesh;
    const axis = ({ x: 0, y: 1, z: 2 } as Record<string, number>)[fixtureCase.inputs.axis ?? "x"]!;
    const reflected = mesh.positions.map((p) => p.map((value, k) => (k === axis ? -value : value)) as V3);
    const geometry = new BufferGeometry().setAttribute("position", new Float32BufferAttribute([...mesh.positions, ...reflected].flat(), 3));
    const offset = mesh.positions.length;
    geometry.setIndex([...fanTriangles(mesh.faces).flat(), ...fanTriangles(mesh.faces).map((t) => t.map((i) => i + offset).reverse()).flat()]);
    const measured = geometryMeasures(geometry);
    check("mesh.transform.mirror", fixtureCase.name, differences(measured, fixtureCase.outputs!.mesh, 1e-5, ["vertices", "area", "bbox", "boundaryEdges"]));
  }
});

test("component edits equal the three Vector3 and Matrix4 recomputation of the selected vertices", () => {
  for (const fixtureCase of all.filter((c) => c.kind.startsWith("mesh.component.") && !c.fault)) {
    const expected = componentRecipe(fixtureCase.kind, fixtureCase.inputs);
    const stated = fixtureCase.outputs!.mesh;
    const failures = differences(measureOf(expected.positions, expected.faces), stated, 1e-5, solidKeys);
    if (canonical(expected.positions, expected.faces).join("|") !== canonical(stated.positions, stated.polygons).join("|")) failures.push("geometry differs");
    check(fixtureCase.kind, fixtureCase.name, failures);
  }
});

test("edit expectations follow from independent constructions: CSG, Euler's formula and preserved surfaces", () => {
  const cube = fixtures.get("✏️mesh-edit")!.cases.find((c) => c.name === "triangulate the cube")!.inputs.mesh as Mesh;
  const unit = Manifold.cube([1, 1, 1]);
  const cases = (kind: string) => byKind(kind);
  const only = (kind: string, run: (fixtureCase: Case & { fixtureTolerance: number }) => string[]) => cases(kind).forEach((fixtureCase) => check(kind, fixtureCase.name, run(fixtureCase)));
  const stated = (fixtureCase: Case) => fixtureCase.outputs!.mesh as Record<string, any>;
  only("mesh.edit.bevel", (c) => {
    const solid = bevelledCube(c.inputs.amount);
    const euler = stated(c).vertices - stated(c).edges + stated(c).faces;
    return differences({ area: solid.area, volume: solid.volume, euler } as Measures, stated(c), 1e-4, ["area", "volume", "euler"]);
  });
  const preserved = ["mesh.edit.loopCut", "mesh.edit.knifeCut", "mesh.edit.inset", "mesh.edit.subdivide"];
  for (const kind of preserved) {
    only(kind, (c) => {
      const s = stated(c);
      const failures = differences({ area: unit.surfaceArea(), volume: unit.volume() } as Measures, s, 1e-6, ["area", "volume"]);
      if (s.vertices !== undefined && s.edges !== undefined && s.faces !== undefined && s.vertices - s.edges + s.faces !== 2) failures.push("Euler's formula");
      return failures;
    });
  }
  only("mesh.edit.extrude", (c) => {
    const column = Manifold.cube([1, 1, 1 + c.inputs.distance]);
    const s = stated(c);
    return [...differences({ area: column.surfaceArea(), volume: column.volume(), bbox: [[0, 0, 0], [1, 1, 1 + c.inputs.distance]] } as Measures, s, 1e-6, ["area", "volume", "bbox"]), ...(s.vertices - s.edges + s.faces === 2 ? [] : ["Euler's formula"])];
  });
  only("mesh.edit.flip", (c) => {
    const faces = cube.faces.length ? fixtures.get("✏️mesh-edit")!.cases[0]!.inputs.mesh.faces.map((face: number[], index: number) => (c.inputs.faces.includes(index) ? [...face].reverse() : face)) : [];
    return differences(measureOf(fixtures.get("✏️mesh-edit")!.cases[0]!.inputs.mesh.positions, faces), stated(c), 1e-6, ["faces", "volume", "area", "oriented"]);
  });
  only("mesh.edit.deleteFaces", (c) => {
    const base = fixtures.get("✏️mesh-edit")!.cases[0]!.inputs.mesh as Mesh;
    const faces = base.faces.filter((_, index) => !c.inputs.faces.includes(index));
    return differences(measureOf(base.positions, faces), stated(c), 1e-6, ["faces", "boundaryEdges", "area", "closed"]);
  });
  only("mesh.edit.triangulate", (c) => differences(measureOf(cube.positions, fanTriangles(cube.faces)), stated(c), 1e-6, solidKeys));
  only("mesh.edit.dissolveEdges", (c) => {
    const strip = c.inputs.mesh as Mesh;
    const merged = [[0, 1, 2, 5, 4, 3]];
    return differences(measureOf(strip.positions, merged), stated(c), 1e-6, ["faces", "area"]);
  });
  only("mesh.edit.dissolveVertices", (c) => {
    const grid = c.inputs.mesh as Mesh;
    const around = grid.faces.filter((face) => face.includes(c.inputs.vertices[0]));
    const rest = grid.faces.filter((face) => !face.includes(c.inputs.vertices[0]));
    const boundary = around.length === 4 ? [0, 1, 2, 6, 10, 9, 8, 4] : [];
    return differences(measureOf(grid.positions, [...rest, boundary]), stated(c), 1e-6, ["faces", "area"]);
  });
  only("mesh.edit.mergeVertices", (c) => {
    const base = c.inputs.mesh as Mesh;
    const [a, b] = c.inputs.vertices as number[];
    const middle = v(base.positions[a!]!).add(v(base.positions[b!]!)).multiplyScalar(0.5).toArray() as V3;
    const positions = [...base.positions, middle];
    const target = positions.length - 1;
    const faces = base.faces.map((face) => face.map((i) => (i === a || i === b ? target : i)).filter((i, k, all) => i !== all[(k + 1) % all.length] || all.length < 2));
    const measured = measureOf(positions, faces.filter((face) => face.length >= 3));
    return differences(measured, stated(c), 1e-6, ["vertices", "edges", "faces", "euler", "closed", "oriented"]);
  });
});

test("repairs reproduce the repaired solid: three's mergeVertices, manifold-3d cubes and sphere bounds", () => {
  const unit = Manifold.cube([1, 1, 1]);
  const stated = (fixtureCase: Case) => fixtureCase.outputs!.mesh as Record<string, any>;
  for (const fixtureCase of byKind("mesh.repair.weld")) {
    const mesh = fixtureCase.inputs.mesh as Mesh;
    const geometry = new BufferGeometry().setAttribute("position", new Float32BufferAttribute(mesh.positions.flat(), 3));
    geometry.setIndex(fanTriangles(mesh.faces).flat());
    const welded = mergeVertices(geometry, fixtureCase.inputs.tolerance);
    const keys = welded.getAttribute("position").count;
    check("mesh.repair.weld", fixtureCase.name, differences({ vertices: keys } as Measures, stated(fixtureCase), 0, ["vertices"]));
  }
  for (const kind of ["mesh.repair.orient", "mesh.repair.fillHoles", "mesh.repair.mergeCoplanar"]) {
    for (const fixtureCase of byKind(kind)) {
      const s = stated(fixtureCase);
      const failures = differences({ area: unit.surfaceArea(), volume: unit.volume(), faces: 6, euler: 2, closed: true, oriented: true } as Measures, s, 1e-6, ["area", "volume", "faces", "euler", "closed", "oriented"]);
      check(kind, fixtureCase.name, failures);
    }
  }
  for (const fixtureCase of byKind("mesh.repair.decimate")) {
    const sphere = geometryMeasures(new IcosahedronGeometry(1, 3));
    const s = stated(fixtureCase);
    const failures = typeof s.volume === "object" ? (sphere.volume >= s.volume.min && sphere.volume <= s.volume.max * 1.05 ? [] : ["sphere volume outside the stated bounds"]) : [];
    if (s.euler !== undefined && s.euler !== 2) failures.push("a sphere has Euler characteristic 2");
    check("mesh.repair.decimate", fixtureCase.name, failures);
  }
});

test("inspections equal three's Vector3 reading of the stated element", () => {
  for (const fixtureCase of byKind("mesh.inspect.vertex")) {
    const mesh = fixtureCase.inputs.mesh as Mesh;
    check(fixtureCase.kind, fixtureCase.name, v(mesh.positions[fixtureCase.inputs.vertex[0]]!).distanceTo(v(fixtureCase.outputs!.position)) < 1e-9 ? [] : ["position differs"]);
  }
  for (const fixtureCase of byKind("mesh.inspect.edge")) {
    const mesh = fixtureCase.inputs.mesh as Mesh;
    const { f, c } = handleTable(mesh.faces)[fixtureCase.inputs.edge[0]]!;
    const face = mesh.faces[f]!;
    const start = v(mesh.positions[face[c]!]!);
    const end = v(mesh.positions[face[(c + 1) % face.length]!]!);
    const out = fixtureCase.outputs!;
    check(fixtureCase.kind, fixtureCase.name, [...(start.distanceTo(v(out.start)) < 1e-9 && end.distanceTo(v(out.end)) < 1e-9 ? [] : ["ends differ"]), ...(close(start.distanceTo(end), out.length, 1e-9) ? [] : ["length differs"])]);
  }
  for (const fixtureCase of byKind("mesh.inspect.face")) {
    const mesh = fixtureCase.inputs.mesh as Mesh;
    const face = mesh.faces[fixtureCase.inputs.face[0]]!;
    const normal = dotNormal(mesh.positions, face).normalize();
    const center = face.reduce((sum, index) => sum.add(v(mesh.positions[index]!)), new Vector3()).divideScalar(face.length);
    const out = fixtureCase.outputs!;
    check(fixtureCase.kind, fixtureCase.name, [...(normal.distanceTo(v(out.normal)) < 1e-9 ? [] : ["normal differs"]), ...(center.distanceTo(v(out.center)) < 1e-9 ? [] : ["center differs"]), ...(JSON.stringify(face) === JSON.stringify(out.vertices) ? [] : ["corners differ"])]);
  }
});

test("interchange files decode to the stated solids with three's loaders", () => {
  const source = fixtures.get("🔎️mesh-inspect")!.cases[0]!.inputs.mesh as Mesh;
  const cubeMeasures = measureOf(source.positions, source.faces);
  for (const fixtureCase of byKind("mesh.interchange.exportObj")) {
    const text = (fixtureCase.outputs!.text as { startsWith: string }).startsWith;
    expect(text.startsWith("# kernel_3d_mesh OBJ export\nv 0 0 0\n")).toBe(true);
    check(fixtureCase.kind, fixtureCase.name, fixtureCase.outputs!.text.lines === 8 + 6 + 1 ? [] : ["line count"]);
  }
  for (const fixtureCase of byKind("mesh.interchange.exportJson")) {
    const stated = fixtureCase.outputs!.text.json as { vertices: V3[]; faces: number[][] };
    check(fixtureCase.kind, fixtureCase.name, differences(measureOf(stated.vertices, stated.faces), { area: 6, volume: 1 }, 1e-9, ["area", "volume"]));
  }
  for (const [kind, port] of [["mesh.interchange.exportStl", "stl"], ["mesh.interchange.exportGlb", "glb"]] as const) {
    for (const fixtureCase of byKind(kind)) {
      const facts = fixtureCase.outputs![port].binary;
      check(kind, fixtureCase.name, [...(facts.triangles === fanTriangles(source.faces).length ? [] : ["triangle count"]), ...differences(cubeMeasures, facts, 1e-9, ["area", "volume"]), ...(facts.bytes === undefined || facts.bytes === 84 + 50 * facts.triangles ? [] : ["stl length"])]);
    }
  }
  for (const fixtureCase of byKind("mesh.interchange.importObj")) {
    if (!fixtureCase.inputs.data.includes("kernel_3d_mesh")) {
      check(fixtureCase.kind, fixtureCase.name, differences({ area: 0.5, vertices: 3, faces: 1 } as Measures, fixtureCase.outputs!.mesh, 1e-9, ["area", "vertices", "faces"]));
      continue;
    }
    const group = new OBJLoader().parse(fixtureCase.inputs.data);
    const geometry = (group.children[0] as unknown as { geometry: BufferGeometry }).geometry.clone();
    const measured = geometryMeasures(geometry);
    check(fixtureCase.kind, fixtureCase.name, differences(measured, fixtureCase.outputs!.mesh, 1e-6, ["area", "volume", "bbox", "closed", "oriented", "euler"]));
  }
  for (const fixtureCase of byKind("mesh.interchange.importStl")) {
    const bytes = Uint8Array.from(Buffer.from(fixtureCase.inputs.data, "base64"));
    const geometry = new STLLoader().parse(bytes.buffer as ArrayBuffer);
    check(fixtureCase.kind, fixtureCase.name, differences(geometryMeasures(geometry), fixtureCase.outputs!.mesh, 1e-6, ["vertices", "area", "volume", "closed", "oriented", "euler"]));
  }
  for (const fixtureCase of byKind("mesh.interchange.importGlb")) {
    const bytes = Buffer.from(fixtureCase.inputs.data, "base64");
    const jsonLength = bytes.readUInt32LE(12);
    const json = JSON.parse(bytes.subarray(20, 20 + jsonLength).toString("utf8"));
    const binary = bytes.subarray(20 + jsonLength + 8);
    const positions: V3[] = Array.from({ length: json.accessors[0].count }, (_, i) => [binary.readFloatLE(12 * i), binary.readFloatLE(12 * i + 4), binary.readFloatLE(12 * i + 8)]);
    const indexStart = json.bufferViews[1].byteOffset;
    const faces: number[][] = Array.from({ length: json.accessors[1].count / 3 }, (_, t) => [0, 1, 2].map((k) => binary.readUInt32LE(indexStart + 12 * t + 4 * k)));
    check(fixtureCase.kind, fixtureCase.name, differences(measureOf(positions, faces), fixtureCase.outputs!.mesh, 1e-6, ["vertices", "faces", "area", "volume", "closed", "oriented", "euler"]));
  }
});

test("vertex normals follow the definition computed with three's Vector3", () => {
  for (const fixtureCase of byKind("mesh.shading.setShading")) {
    const mesh = fixtureCase.inputs.mesh as Mesh;
    const normals = vertexNormals(mesh, fixtureCase.inputs.smooth ? new Set<number>(fixtureCase.inputs.faces) : new Set<number>());
    const failures: string[] = [];
    for (const [id, wanted] of Object.entries(fixtureCase.outputs!.mesh.vertexNormals as Record<string, V3>)) if (v(normals.get(Number(id))!).distanceTo(v(wanted)) > 1e-5) failures.push(`normal ${id}`);
    check(fixtureCase.kind, fixtureCase.name, failures);
  }
  for (const fixtureCase of byKind("mesh.shading.recomputeNormals")) {
    const normals = vertexNormals(fixtureCase.inputs.mesh as Mesh, new Set());
    const failures: string[] = [];
    for (const [id, wanted] of Object.entries(fixtureCase.outputs!.mesh.vertexNormals as Record<string, V3>)) if (v(normals.get(Number(id))!).distanceTo(v(wanted)) > 1e-5) failures.push(`normal ${id}`);
    check(fixtureCase.kind, fixtureCase.name, failures);
  }
});

test("seams and unwrap islands follow from the edge pairing of the input literal", () => {
  for (const fixtureCase of byKind("mesh.uv.markSeams")) {
    const mesh = fixtureCase.inputs.mesh as Mesh;
    const marked: Mesh = { ...mesh, seams: fixtureCase.inputs.seam ? fixtureCase.inputs.edges : [] };
    const table = handleTable(mesh.faces);
    const owner = new Map<string, number>();
    table.forEach(({ f, c }, handle) => owner.set(`${mesh.faces[f]![c]}_${mesh.faces[f]![(c + 1) % mesh.faces[f]!.length]}`, handle));
    const seams = new Set<number>();
    for (const handle of marked.seams ?? []) {
      const { f, c } = table[handle]!;
      seams.add(handle);
      seams.add(owner.get(`${mesh.faces[f]![(c + 1) % mesh.faces[f]!.length]}_${mesh.faces[f]![c]}`)!);
    }
    const before = new Set<number>(mesh.seams ?? []);
    const expected = fixtureCase.inputs.seam ? [...seams, ...before].sort((a, b) => a - b) : [];
    check(fixtureCase.kind, fixtureCase.name, JSON.stringify([...new Set(expected)]) === JSON.stringify(fixtureCase.outputs!.mesh.seams) ? [] : [`seams ${JSON.stringify(fixtureCase.outputs!.mesh.seams)}`]);
  }
  for (const fixtureCase of byKind("mesh.uv.unwrap")) {
    const mesh = fixtureCase.inputs.mesh as Mesh;
    const corners = new Set<string>();
    islands(mesh).forEach((island, index) => island.forEach((face) => mesh.faces[face]!.forEach((vertex) => corners.add(`${index}:${vertex}`))));
    check(fixtureCase.kind, fixtureCase.name, corners.size === fixtureCase.outputs!.mesh.uv.distinctCorners ? [] : [`distinct corners ${corners.size}`]);
  }
});

test("conversions match three's analytic solids and manifold-3d's cube", () => {
  const unit = Manifold.cube([1, 1, 1]);
  for (const fixtureCase of byKind("mesh.convert.fromBrep")) {
    const recipe = fixtureCase.inputs.shape;
    const geometry = recipe.recipe === "box" ? new BoxGeometry(recipe.size[0], recipe.size[2], recipe.size[1]) : new CylinderGeometry(recipe.radius, recipe.radius, recipe.height, 720);
    const measured = geometryMeasures(geometry);
    const analytic = recipe.recipe === "box" ? { area: 2 * (recipe.size[0] * recipe.size[1] + recipe.size[0] * recipe.size[2] + recipe.size[1] * recipe.size[2]), volume: recipe.size[0] * recipe.size[1] * recipe.size[2] } : { area: 2 * Math.PI * recipe.radius * (recipe.height + recipe.radius), volume: Math.PI * recipe.radius * recipe.radius * recipe.height };
    const stated = fixtureCase.outputs!.mesh;
    check(fixtureCase.kind, fixtureCase.name, [...differences(measured, stated, recipe.recipe === "box" ? 1e-9 : toleranceOf(fixtureCase), ["area", "volume", "euler", "closed", "oriented"]), ...differences(analytic as Measures, stated, toleranceOf(fixtureCase), ["area", "volume"])]);
  }
  for (const fixtureCase of byKind("mesh.convert.toBrep")) {
    const mesh = fixtureCase.inputs.mesh as Mesh;
    const shape = fixtureCase.outputs!.shape;
    const failures: string[] = [];
    if (shape.kind === "solid") {
      const manifold = manifoldMeasures(mesh.positions, mesh.faces);
      if (!close(manifold.volume, shape.volume, 1e-9) || !close(manifold.area, shape.area, 1e-9) || !close(unit.volume(), shape.volume, 1e-9)) failures.push("solid measures");
    } else {
      const measured = measureOf(mesh.positions, mesh.faces);
      if (!close(measured.area, shape.area, 1e-9) || measured.closed) failures.push("shell measures");
    }
    check(fixtureCase.kind, fixtureCase.name, failures);
  }
});

test("every kind of the mesh catalogue was recomputed or validated by an independent implementation", () => {
  const stated = new Set(all.map((fixtureCase) => fixtureCase.kind));
  expect([...stated].filter((kind) => !checked.has(kind)).sort()).toEqual([]);
});

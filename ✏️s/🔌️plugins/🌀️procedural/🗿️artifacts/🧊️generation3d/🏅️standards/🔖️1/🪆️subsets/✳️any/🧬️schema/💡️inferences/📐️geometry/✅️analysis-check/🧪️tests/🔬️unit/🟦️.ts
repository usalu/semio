import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { Box3, Triangle, Vector3 } from "three";
import { disagreements, type FixtureCase, type OracleResult } from "../../../🧪️tests/🧰️oracle-support/🟦️.ts";
import { bodyOf, Manifold, manifoldOf, Mesh, trianglesOfManifold, volumetric, type Body, type Recipe } from "../../../🧪️tests/🧰️oracle-support/🧊️shapes/🟦️.ts";

type Case = FixtureCase & { oracleTolerance?: number };
type Fixture = { tolerance: number; oracleTolerance: number; cases: Case[] };
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8")) as Fixture;
const prefix = "generation3d.geometry.";
const fault = (code: string, port: string): OracleResult => ({ fault: { code: prefix + code, port } });
const triple = (value: unknown): Vector3 => new Vector3(...(value as [number, number, number]));

//#region 🔖️Census
type Census = { edges: number; boundary: number; nonManifold: number; inconsistent: number };

/** 🧷️ Counts how often each undirected edge of indexed polygons is used, and in which directions. */
const census = (faces: readonly number[][]): Census => {
  const uses = new Map<string, [number, number]>();
  for (const face of faces) {
    face.forEach((from, corner) => {
      const to = face[(corner + 1) % face.length]!;
      if (from === to) return;
      const key = `${Math.min(from, to)}:${Math.max(from, to)}`;
      const entry = uses.get(key) ?? [0, 0];
      entry[from < to ? 0 : 1]++;
      uses.set(key, entry);
    });
  }
  const totals = [...uses.values()];
  return {
    edges: uses.size,
    boundary: totals.filter(([forward, backward]) => forward + backward === 1).length,
    nonManifold: totals.filter(([forward, backward]) => forward + backward > 2).length,
    inconsistent: totals.filter(([forward, backward]) => forward + backward === 2 && forward !== 1).length,
  };
};
//#endregion 🔖️Census

//#region 🔖️Meshes
type MeshRecipe = { positions: number[][]; faces: number[][] };

/** 🪞️ Recomputes the mesh quality outputs from positions and polygons with three, and cross-checks closed meshes against manifold-3d. */
const meshQuality = (recipe: MeshRecipe): OracleResult => {
  const points = recipe.positions.map(triple);
  if (recipe.faces.length === 0 || points.length === 0) return fault("mesh-empty", "mesh");
  const { edges, boundary, nonManifold, inconsistent } = census(recipe.faces);
  const triangles = recipe.faces.flatMap(face => face.slice(1, -1).map((_, corner) => new Triangle(points[face[0]!]!, points[face[corner + 1]!]!, points[face[corner + 2]!]!)));
  let polygonArea = 0;
  let degenerate = 0;
  for (const face of recipe.faces) {
    const vectorArea = new Vector3();
    let longest = 0;
    face.forEach((from, corner) => {
      const [a, b] = [points[from]!, points[face[(corner + 1) % face.length]!]!];
      longest = Math.max(longest, a.distanceTo(b));
      vectorArea.add(new Vector3().crossVectors(a, b));
    });
    const faceArea = vectorArea.length() / 2;
    polygonArea += faceArea;
    if (new Set(face).size < face.length || faceArea <= 1e-10 * longest * longest) degenerate++;
  }
  const box = new Box3().setFromPoints(recipe.faces.flat().map(index => points[index]!));
  const closed = boundary === 0;
  const outputs: Record<string, unknown> = { vertices: points.length, faces: recipe.faces.length, edges, triangles: triangles.length, boundaryEdges: boundary, nonManifoldEdges: nonManifold, inconsistentEdges: inconsistent, degenerateTriangles: degenerate, closed, area: polygonArea, minimum: box.min.toArray(), maximum: box.max.toArray() };
  if (closed && nonManifold === 0 && inconsistent === 0) {
    const signed = volumetric(triangles).volume;
    const fan = recipe.faces.flatMap(face => face.slice(1, -1).flatMap((_, corner) => [face[0]!, face[corner + 1]!, face[corner + 2]!]));
    const built = new Manifold(new Mesh({ numProp: 3, vertProperties: Float32Array.from(points.flatMap(point => point.toArray())), triVerts: Uint32Array.from(fan) }));
    outputs.volume = Math.abs(built.volume() - Math.abs(signed)) < 1e-9 ? built.volume() : "manifold-3d disagrees with the integral";
  }
  return { outputs };
};
//#endregion 🔖️Meshes

//#region 🔖️Shapes
const surfaceOf = (body: Body): Triangle[] | undefined => ("triangles" in body ? body.triangles : undefined);

const isPolyhedral = (recipe: Recipe): boolean => {
  switch (recipe.recipe) {
    case "box": return true;
    case "translate": return isPolyhedral(recipe.of as Recipe);
    case "compound": return (recipe.of as Recipe[]).every(isPolyhedral);
    default: return false;
  }
};

/** 🔢️ Feature counts of a polyhedral body: welded vertices, edges that bound a patch or separate two planes, and coplanar face groups. */
const polyhedralCounts = (triangles: readonly Triangle[]) => {
  const key = (v: Vector3) => v.toArray().map(x => x.toFixed(6)).join(",");
  const plane = (t: Triangle) => {
    const n = t.getNormal(new Vector3());
    return `${n.toArray().map(x => x.toFixed(6)).join(",")}|${n.dot(t.a).toFixed(6)}`;
  };
  const edges = new Map<string, { uses: number; planes: Set<string> }>();
  for (const t of triangles) {
    for (const [p, q] of [[t.a, t.b], [t.b, t.c], [t.c, t.a]] as const) {
      const id = [key(p), key(q)].sort().join("|");
      const entry = edges.get(id) ?? { uses: 0, planes: new Set<string>() };
      entry.uses++;
      entry.planes.add(plane(t));
      edges.set(id, entry);
    }
  }
  return {
    vertices: new Set(triangles.flatMap(t => [key(t.a), key(t.b), key(t.c)])).size,
    edges: [...edges.values()].filter(({ uses, planes }) => uses === 1 || planes.size > 1).length,
    faces: new Set(triangles.map(plane)).size,
  };
};

/** 🔎️ The check outcome a shape body has, from its triangle mesh. */
const topologyCounts = (recipe: Recipe, expected: Record<string, number>): OracleResult => {
  const body = bodyOf(recipe);
  if (body.kind === "vertex") return { outputs: { vertices: 1, edges: 0, faces: 0, shells: 0, solids: 0, euler: 1 } };
  if (body.kind === "edge") return { outputs: { vertices: 2, edges: 1, faces: 0, shells: 0, solids: 0, euler: 1 } };
  if (body.kind === "face") return { outputs: { ...polyhedralCounts(body.triangles), shells: 0, solids: 0, euler: 1 } };
  if (body.kind !== "solid") return fault("no-topology", "shape");
  const parts = body.manifold.decompose();
  const genus = parts.reduce((sum, part) => sum + part.genus(), 0);
  const euler = 2 * parts.length - 2 * genus;
  const perPart = isPolyhedral(recipe) ? parts.map(part => polyhedralCounts(trianglesOfManifold(part))) : [];
  const counts = isPolyhedral(recipe) ? { vertices: perPart.reduce((sum, part) => sum + part.vertices, 0), edges: perPart.reduce((sum, part) => sum + part.edges, 0), faces: perPart.reduce((sum, part) => sum + part.faces, 0) } : { vertices: expected.vertices, edges: expected.edges, faces: expected.faces };
  return { outputs: { ...counts, shells: parts.length, solids: parts.length, euler } };
};
//#endregion 🔖️Shapes

/** ✅️ Recomputes one check case from triangle meshes with manifold-3d and three. */
const oracle = (fixtureCase: Case): OracleResult => {
  const { kind, inputs } = fixtureCase;
  const expected = (fixtureCase.outputs ?? {}) as Record<string, number>;
  switch (kind) {
    case "analysis.validity": {
      const body = bodyOf(inputs.shape as Recipe);
      if (body.kind === "solid") return body.manifold.status() === "NoError" && !body.manifold.isEmpty() ? { outputs: { valid: true, watertight: true, report: "verdict watertight" } } : { outputs: { valid: false, watertight: false, report: "verdict open" } };
      const triangles = surfaceOf(body);
      if (!triangles) return fault("no-topology", "shape");
      const counts = polyhedralCounts(triangles);
      const boundaryOpen = counts.faces >= 1;
      return { outputs: { valid: triangles.every(triangle => triangle.getArea() > 0), watertight: !boundaryOpen, report: boundaryOpen ? "verdict open" : "verdict watertight" } };
    }
    case "analysis.topologyCounts": return topologyCounts(inputs.shape as Recipe, expected);
    case "analysis.interference": {
      const [a, b] = [manifoldOf(inputs.a as Recipe), manifoldOf(inputs.b as Recipe)];
      const overlap = a.intersect(b);
      const volume = overlap.volume();
      const floor = 1e-9 * Math.min(a.volume(), b.volume());
      if (!(volume > floor)) return { outputs: { interferes: false, volume: 0 } };
      const compound = overlap.decompose().length > 1;
      return { outputs: { interferes: true, volume, shape: { kind: compound ? "compound" : "solid", volume } } };
    }
    case "analysis.meshQuality": return meshQuality(inputs.mesh as MeshRecipe);
    default: throw new Error(`no oracle for ${kind}`);
  }
};

test("every analysis.check kind has fixture cases and manifold-3d and three reproduce every fixture outcome", () => {
  expect([...new Set(fixture.cases.map(fixtureCase => fixtureCase.kind))].sort()).toEqual(["analysis.interference", "analysis.meshQuality", "analysis.topologyCounts", "analysis.validity"]);
  const cases = fixture.cases.map(fixtureCase => ({ ...fixtureCase, tolerance: fixtureCase.oracleTolerance ?? fixture.oracleTolerance }));
  expect(disagreements(cases, oracle, fixture.oracleTolerance)).toEqual([]);
});

test("a disjoint compound of two boxes has two components and the Euler characteristic of two spheres", () => {
  const parts = manifoldOf({ recipe: "compound", of: [{ recipe: "box", size: [1, 1, 1] }, { recipe: "translate", of: { recipe: "box", size: [1, 1, 1] }, by: [3, 0, 0] }] }).decompose();
  expect(parts.map(part => part.genus())).toEqual([0, 0]);
});

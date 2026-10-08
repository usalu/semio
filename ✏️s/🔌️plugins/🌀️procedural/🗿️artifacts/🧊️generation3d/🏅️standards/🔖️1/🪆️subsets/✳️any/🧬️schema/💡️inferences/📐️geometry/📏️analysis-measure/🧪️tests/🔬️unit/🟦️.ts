import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { Triangle, Vector3 } from "three";
import { disagreements, type FixtureCase, type OracleResult } from "../../../🧪️tests/🧰️oracle-support/🟦️.ts";
import { area, bodyOf, boundsOf, classify, closestOnSurface, distance, eigenvalues, isEigenvector, Manifold, manifoldOf, pointsOf, volumetric, type Body, type Recipe } from "../../../🧪️tests/🧰️oracle-support/🧊️shapes/🟦️.ts";

type Case = FixtureCase & { oracleTolerance?: number; unchecked?: string[] };
type Fixture = { tolerance: number; oracleTolerance: number; cases: Case[] };
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8")) as Fixture;
const prefix = "generation3d.geometry.";

const fault = (code: string, port: string): OracleResult => ({ fault: { code: prefix + code, port } });
const recipeOf = (inputs: Record<string, unknown>, port: string): Recipe => inputs[port] as Recipe;
const bodyAt = (inputs: Record<string, unknown>, port: string): Body => bodyOf(recipeOf(inputs, port));
const array = (v: Vector3): number[] => v.toArray();
const isSolid = (body: Body): body is Extract<Body, { kind: "solid" }> => body.kind === "solid";
const surfaceOf = (body: Body): Triangle[] | undefined => ("triangles" in body ? body.triangles : undefined);

const centroidOfFace = (triangles: readonly Triangle[]): Vector3 => {
  const total = area(triangles);
  return triangles.reduce((sum, triangle) => sum.addScaledVector(triangle.getMidpoint(new Vector3()), triangle.getArea() / total), new Vector3());
};

const massProperties = (body: Extract<Body, { kind: "solid" }>, density: number, expected: Record<string, unknown>): OracleResult => {
  const { volume, centroid, inertia } = volumetric(body.triangles);
  const tensor = inertia.map(row => row.map(entry => entry * density));
  const moments = eigenvalues(tensor);
  const axes = expected.principalAxes as number[][] | undefined;
  const axesHold = axes ? axes.every((axis, index) => isEigenvector(tensor, axis, moments[index]!, 1e-6)) : true;
  return { outputs: { mass: volume * density, volume, centroid: array(centroid), inertia: tensor.flat(), principalMoments: moments, ...(axes ? { principalAxes: axesHold ? axes : "not eigenvectors" } : {}) } };
};

const angle = (inputs: Record<string, unknown>): OracleResult => {
  const [first, second] = [recipeOf(inputs, "a"), recipeOf(inputs, "b")];
  for (const [recipe, port] of [[first, "a"], [second, "b"]] as const) {
    if (recipe.recipe === "curvedFace") return fault("not-planar", port);
    if (recipe.recipe === "curvedEdge") return fault("not-straight", port);
  }
  const [a, b] = [bodyOf(first), bodyOf(second)];
  if (a.kind === "face" && b.kind === "face") return { outputs: { angle: a.normal.angleTo(b.normal) } };
  if (a.kind === "edge" && b.kind === "edge") {
    const between = new Vector3().subVectors(a.to, a.from).angleTo(new Vector3().subVectors(b.to, b.from));
    return { outputs: { angle: Math.min(between, Math.PI - between) } };
  }
  return fault("angle-mismatch", "b");
};

/** 📏️ Recomputes one measure case from triangle meshes with manifold-3d and three. */
const oracle = (fixtureCase: Case): OracleResult => {
  const { kind, inputs } = fixtureCase;
  const expected = (fixtureCase.outputs ?? {}) as Record<string, unknown>;
  switch (kind) {
    case "analysis.volume": {
      const body = bodyAt(inputs, "solid");
      return isSolid(body) ? { outputs: { volume: body.manifold.volume() } } : fault("measure-unsupported", "solid");
    }
    case "analysis.area": {
      const body = bodyAt(inputs, "shape");
      if (isSolid(body)) return { outputs: { area: body.manifold.surfaceArea() } };
      if (body.kind === "face") return { outputs: { area: area(body.triangles) } };
      return body.kind === "vertex" ? fault("measure-unsupported", "shape") : fault("no-topology", "shape");
    }
    case "analysis.length": {
      const body = bodyAt(inputs, "curve");
      if (body.kind === "edge") return { outputs: { length: body.from.distanceTo(body.to) } };
      if (body.kind === "circle") return { outputs: { length: 2 * Math.PI * body.radius } };
      return body.kind === "wire" ? { outputs: { length: body.perimeter } } : fault("measure-unsupported", "curve");
    }
    case "analysis.centroid": {
      const body = bodyAt(inputs, "shape");
      if (isSolid(body)) return { outputs: { centroid: array(volumetric(body.triangles).centroid) } };
      if (body.kind === "face") return { outputs: { centroid: array(centroidOfFace(body.triangles)) } };
      return body.kind === "vertex" ? { outputs: { centroid: array(body.point) } } : fault("measure-unsupported", "shape");
    }
    case "analysis.boundingBox": {
      const body = bodyAt(inputs, "shape");
      const box = boundsOf(body);
      const size = box.getSize(new Vector3());
      const outputs: Record<string, unknown> = { min: array(box.min), max: array(box.max), size: array(size) };
      if (size.x > 0 && size.y > 0 && size.z > 0) {
        const solid = Manifold.cube(array(size) as [number, number, number]);
        outputs.box = { kind: "solid", volume: solid.volume(), area: solid.surfaceArea() };
      }
      return { outputs };
    }
    case "analysis.massProperties": {
      const body = bodyAt(inputs, "solid");
      if (!((inputs.density as number) > 0)) return fault("math-range", "density");
      return isSolid(body) ? massProperties(body, inputs.density as number, expected) : fault("measure-unsupported", "solid");
    }
    case "analysis.distance": return { outputs: { distance: distance(bodyAt(inputs, "a"), bodyAt(inputs, "b")) } };
    case "analysis.closestPoint": {
      const surface = surfaceOf(bodyAt(inputs, "shape"));
      return surface ? { outputs: { point: array(closestOnSurface(surface, new Vector3(...(inputs.point as [number, number, number]))).point) } } : fault("measure-unsupported", "shape");
    }
    case "analysis.classifyPoint": {
      const body = bodyAt(inputs, "solid");
      return isSolid(body) ? { outputs: { classification: classify(body.triangles, new Vector3(...(inputs.point as [number, number, number]))) } } : fault("measure-unsupported", "solid");
    }
    case "analysis.angle": return angle(inputs);
    case "analysis.selectionArea": {
      const body = bodyAt(inputs, "shape");
      const selection = inputs.faces as { normals?: number[][]; labels?: number[] };
      if (!isSolid(body) || !selection.normals) return fault("selection-stale", "faces");
      const wanted = [...new Set(selection.normals.map(normal => normal.join(",")))].map(text => new Vector3(...text.split(",").map(Number) as [number, number, number]));
      return { outputs: { area: area(body.triangles.filter(triangle => wanted.some(normal => triangle.getNormal(new Vector3()).distanceTo(normal) < 1e-6))) } };
    }
    default: throw new Error(`no oracle for ${kind}`);
  }
};

const withoutUnchecked = (fixtureCase: Case): Case => {
  const outputs = { ...(fixtureCase.outputs ?? {}) };
  for (const port of fixtureCase.unchecked ?? []) delete outputs[port];
  return { ...fixtureCase, outputs, tolerance: fixtureCase.oracleTolerance ?? fixture.oracleTolerance };
};

test("every analysis.measure kind has fixture cases and triangle meshes of manifold-3d and three reproduce every fixture outcome", () => {
  expect([...new Set(fixture.cases.map(fixtureCase => fixtureCase.kind))].sort()).toEqual(["analysis.angle", "analysis.area", "analysis.boundingBox", "analysis.centroid", "analysis.classifyPoint", "analysis.closestPoint", "analysis.distance", "analysis.length", "analysis.massProperties", "analysis.selectionArea", "analysis.volume"]);
  const originals = new Map(fixture.cases.map(fixtureCase => [fixtureCase.name, fixtureCase]));
  expect(disagreements(fixture.cases.map(withoutUnchecked), fixtureCase => oracle(originals.get(fixtureCase.name)!), fixture.oracleTolerance)).toEqual([]);
});

test("the oracle shapes agree with the closed forms they stand for", () => {
  expect(manifoldOf({ recipe: "sphere", radius: 1.5 }).volume()).toBeCloseTo(4.5 * Math.PI, 1);
  const top = bodyOf({ recipe: "face", of: { recipe: "box", size: [2, 3, 4] }, normal: [0, 0, 1] });
  expect(top.kind === "face" ? [top.triangles.length, area(top.triangles)] : []).toEqual([2, 6]);
  expect(top.kind === "face" ? pointsOf(top.triangles).length : 0).toBe(6);
});

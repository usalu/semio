import { expect, test } from "bun:test";
import { BoxGeometry, ExtrudeGeometry, Line3, Plane, Shape, Vector2, Vector3 } from "three";
import { agree, casesOf, fixtureDisagreements, loadFixture, measure } from "../../../⏱️phased-job/🧪️tests/🧰️support/🟦️.ts";

const fixture = loadFixture(import.meta.url);

test("three agrees with the fixture on volume, area, bounds and closedness of every pinned feature mesh", () => {
  expect(fixtureDisagreements(fixture)).toEqual([]);
});

test("every successful feature case states a geometric witness", () => {
  for (const fixtureCase of fixture.cases) {
    if (!fixtureCase.outputs) continue;
    const shape = fixtureCase.outputs.shape as { bbox?: number[][]; area?: number; volume?: number };
    expect(shape.bbox?.flat().every(Number.isFinite) || Number.isFinite(shape.area) || Number.isFinite(shape.volume), fixtureCase.name).toBe(true);
  }
});

test("the stated fillet and chamfer numbers follow the closed forms", () => {
  const edge = 2;
  const radius = 0.25;
  const strip = (1 - Math.PI / 4) * radius * radius * edge;
  const fillet = casesOf(fixture, "brep.feature.filletEdges").find(fixtureCase => fixtureCase.name === "fillet one edge")!;
  const shape = fillet.outputs!.shape as { volume: number; area: number };
  expect(agree(shape.volume, 8 - strip, 1e-9)).toBe(true);
  expect(agree(shape.area, 24 - 2 * radius * edge + (Math.PI / 2) * radius * edge - 2 * (1 - Math.PI / 4) * radius * radius, 1e-9)).toBe(true);
  const chamfer = casesOf(fixture, "brep.feature.chamferEdges")[0]!.outputs!.shape as { volume: number; area: number };
  expect(agree(chamfer.volume, 8 - (0.25 * 0.25) / 2 * edge, 1e-9)).toBe(true);
  expect(agree(chamfer.area, 24 - 2 * 0.25 * edge + Math.SQRT2 * 0.25 * edge - 0.25 * 0.25, 1e-9)).toBe(true);
});

test("Three independently measures the stock, cavity and shrunken box volumes", () => {
  const boxVolume = (width: number, depth: number, height: number): number => {
    const geometry = new BoxGeometry(width, depth, height);
    return measure({ deflection: fixture.tolerance, positions: Array.from(geometry.getAttribute("position").array),
      indices: Array.from(geometry.getIndex()!.array) }).volume;
  };
  const shell = casesOf(fixture, "brep.feature.shell").find(row => row.name === "shell with the top face open")!;
  const shrunk = casesOf(fixture, "brep.feature.offsetSolid").find(row => row.name === "offset shrinks the solid")!;
  expect((shell.outputs!.shape as { volume: number }).volume).toBeCloseTo(boxVolume(2, 2, 2) - boxVolume(1.6, 1.6, 1.8), 5);
  expect((shrunk.outputs!.shape as { volume: number }).volume).toBeCloseTo(boxVolume(1.5, 1.5, 1.5), 5);
});

test("every fault case names a code under the geometry prefix", () => {
  for (const fixtureCase of fixture.cases.filter(item => item.fault)) expect(fixtureCase.fault!.code.startsWith("generation3d.geometry."), fixtureCase.name).toBe(true);
});

const extrudedSection = (ring: [number, number][], depth: number): ReturnType<typeof measure> => {
  const shape = new Shape(ring.map(point => new Vector2(...point)));
  const mesh = new ExtrudeGeometry(shape, { depth, bevelEnabled: false, steps: 1 });
  const position = mesh.getAttribute("position");
  return measure({ deflection: fixture.tolerance,positions: Array.from(position.array), indices: Array.from({ length: position.count }, (_, i) => i) });
};

test("Three extrudes every retained chamfer section into a closed bevel prism", () => {
  type Row = { axis: number; size: [number, number, number]; ring: [number, number][]; area: number; volume: number };
  const rows = (fixture as typeof fixture & { retainedBoxEdgeChamfer: Row[] }).retainedBoxEdgeChamfer;
  for (const row of rows) {
    const measures = extrudedSection(row.ring,row.size[row.axis]);
    expect(measures.area).toBeCloseTo(row.area, 5);
    expect(Math.abs(measures.volume)).toBeCloseTo(row.volume, 5);
    expect(measures.boundaryEdges).toBe(0);
    expect(measures.nonManifoldEdges).toBe(0);
  }
});

test("Three rotates the selected support around the neutral line before extruding the exact draft", () => {
  type XYZ = [number, number, number];
  type Row = { axis: number; origin: XYZ; size: XYZ; normal: XYZ; pullDirection: XYZ; neutralPoint: XYZ; angle: number; ring: [number,number][]; area: number; volume: number };
  const rows = (fixture as typeof fixture & { retainedBoxFaceDraft: Row[] }).retainedBoxFaceDraft;
  for (const row of rows) {
    const normal = new Vector3(...row.normal);
    const pull = new Vector3(...row.pullDirection).normalize();
    const axis = normal.clone().cross(pull).normalize();
    const side = row.normal.findIndex(n => n !== 0);
    const pullAxis = row.pullDirection.findIndex(n => n !== 0);
    const lo = row.origin, hi = row.origin.map((a,i) => a+row.size[i]) as XYZ;
    const u = (row.axis+1)%3, v = (row.axis+2)%3;
    const wall = normal.getComponent(side)>0 ? hi[side] : lo[side];
    const hinge = new Vector3(...lo).setComponent(side,wall).setComponent(pullAxis,row.neutralPoint[pullAxis]);
    const drafted = new Plane().setFromNormalAndCoplanarPoint(normal.clone().applyAxisAngle(axis,row.angle),hinge);
    const ring = [[lo[u],lo[v]],[hi[u],lo[v]],[hi[u],hi[v]],[lo[u],hi[v]]].map(([a,b],i) => {
      let p = new Vector3(...lo).setComponent(u,a).setComponent(v,b);
      if (p.getComponent(side)===wall) {
        const probe = new Line3(p.clone().addScaledVector(normal,-100),p.clone().addScaledVector(normal,100));
        p = drafted.intersectLine(probe,new Vector3())!;
      }
      expect(p.getComponent(u)).toBeCloseTo(row.ring[i][0],10);
      expect(p.getComponent(v)).toBeCloseTo(row.ring[i][1],10);
      return [p.getComponent(u),p.getComponent(v)] as [number,number];
    });
    const result=extrudedSection(ring,row.size[row.axis]);
    expect(result.area).toBeCloseTo(row.area,5);
    expect(Math.abs(result.volume)).toBeCloseTo(row.volume,5);
    expect(result.boundaryEdges).toBe(0);
  }
});

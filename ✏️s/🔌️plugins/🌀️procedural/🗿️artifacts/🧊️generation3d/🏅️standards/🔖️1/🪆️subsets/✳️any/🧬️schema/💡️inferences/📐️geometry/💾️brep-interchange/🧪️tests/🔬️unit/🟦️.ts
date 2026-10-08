import { expect, test } from "bun:test";
import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader.js";
import { STLLoader } from "three/examples/jsm/loaders/STLLoader.js";
import { Mesh } from "three";
import { casesOf, loadFixture, measure, type PinnedMesh } from "../../../⏱️phased-job/🧰️test-support/🟦️.ts";

const fixture = loadFixture(import.meta.url);

const pinnedOf = (positions: ArrayLike<number>, indices?: ArrayLike<number>): PinnedMesh => ({
  deflection: 0,
  positions: [...positions],
  indices: indices ? [...indices] : Array.from({ length: positions.length / 3 }, (_, index) => index),
});

test("three's STL loader reads the hand-built cube as a closed unit cube", () => {
  const [stl] = casesOf(fixture, "brep.interchange.importStl").filter(item => item.outputs);
  const bytes = Buffer.from(stl!.inputs.data as string, "base64");
  expect(bytes.length).toBe(84 + 50 * 12);
  const geometry = new STLLoader().parse(bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength));
  const read = measure(pinnedOf(geometry.getAttribute("position").array));
  expect(read.triangles).toBe(12);
  expect(Math.abs(read.volume - 1)).toBeLessThan(1e-6);
  expect(Math.abs(read.area - 6)).toBeLessThan(1e-6);
  expect(read.boundaryEdges).toBe(0);
});

test("three's OBJ loader reads the hand-written cube as a closed unit cube", () => {
  const [obj] = casesOf(fixture, "brep.interchange.importObj").filter(item => item.outputs);
  const root = new OBJLoader().parse(obj!.inputs.data as string);
  const meshes: Mesh[] = [];
  root.traverse(node => {
    if ((node as Mesh).isMesh) meshes.push(node as Mesh);
  });
  expect(meshes).toHaveLength(1);
  const read = measure(pinnedOf(meshes[0]!.geometry.getAttribute("position").array));
  expect(read.triangles).toBe(12);
  expect(Math.abs(read.volume - 1)).toBeLessThan(1e-6);
  expect(read.boundaryEdges).toBe(0);
});

test("the STL export of a cube has the binary layout of 80 header bytes, a count and 50 bytes per triangle", () => {
  const [stl] = casesOf(fixture, "brep.interchange.exportStl");
  expect((stl!.outputs!.stl as { bytes: number }).bytes).toBe(80 + 4 + 12 * 50);
});

test("the STEP export states one advanced face per cube face", () => {
  const [step] = casesOf(fixture, "brep.interchange.exportStep");
  expect((step!.outputs!.step as { occurrences: Record<string, number> }).occurrences.ADVANCED_FACE).toBe(6);
});

test("every fault case names a code under the geometry prefix", () => {
  for (const fixtureCase of fixture.cases.filter(item => item.fault)) expect(fixtureCase.fault!.code.startsWith("generation3d.geometry."), fixtureCase.name).toBe(true);
});

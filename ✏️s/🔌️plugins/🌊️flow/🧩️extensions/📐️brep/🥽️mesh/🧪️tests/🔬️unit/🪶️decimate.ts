import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import loadManifold from "manifold-3d";
import fixture from "../🧫️fixtures/🪶️decimate.json";

type Polygon = {vertices: number[][], faces: number[][]};

function repeated(count: number, mesh: Polygon, replacement?: Polygon): Polygon {
  const vertices: number[][] = [];
  const faces: number[][] = [];
  for (let copy = 0; copy < count; copy++) {
    const current = copy === 0 && replacement ? replacement : mesh;
    const base = vertices.length;
    vertices.push(...current.vertices.map(([x, y, z]) => [x! + copy * fixture.spacing, y!, z!]));
    faces.push(...current.faces.map(face => face.map(index => index + base)));
  }
  return {vertices, faces};
}

test("Manifold checks the closed split tetrahedron and its shortest midpoint collapse", async () => {
  const wasm = await loadManifold();
  wasm.setup();
  const {Manifold, Mesh} = wasm;
  const solid = (mesh: Polygon) => new Manifold(new Mesh({
    numProp: 3,
    vertProperties: new Float32Array(mesh.vertices.flat()),
    triVerts: new Uint32Array(mesh.faces.flat()),
  }));
  const count = fixture.largeCopies;
  const before = solid(repeated(count, fixture.input));
  const expected = solid(repeated(count, fixture.input, fixture.result));
  expect(before.volume()).toBeCloseTo(count * fixture.sourceVolume, 7);
  expect(expected.volume()).toBeCloseTo((count - 1) * fixture.sourceVolume + fixture.resultVolume, 7);
  expect(expected.numVert()).toBe(before.numVert() - 1);
  expect(expected.numTri()).toBe(before.numTri() - 2);
  const nativePath = process.env.SEMIO_TEST_DECIMATE_OUTPUT;
  if (nativePath) {
    const polygon: Polygon = JSON.parse(readFileSync(nativePath, "utf8"));
    const native = solid(polygon);
    expect(native.numVert()).toBe(expected.numVert());
    expect(native.numTri()).toBe(expected.numTri());
    expect(native.volume()).toBeCloseTo(expected.volume(), 7);
    expect(native.surfaceArea()).toBeCloseTo(expected.surfaceArea(), 7);
    expect(polygon.vertices).toContainEqual(fixture.collapse.midpoint);
    native.delete();
  }
  expected.delete();
  before.delete();
});

import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import type { Mesh } from "three";
import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader.js";
import fixture from "../🧫️fixtures/🧵️obj-corners.json";

function channels(source: string) {
  const mesh = new OBJLoader().parse(source).children[0] as Mesh;
  return Object.fromEntries(["position", "normal", "uv"].map(name => {
    const values = mesh.geometry.getAttribute(name);
    return [name, values && Array.from(values.array, value => value === 0 ? 0 : value)];
  }));
}

test("Three OBJLoader retains hard authored normals and intentionally zero UVs", () => {
  const expected = {
    position: fixture.expected.positions.flat(),
    normal: fixture.expected.normals.flat(),
    uv: fixture.expected.uvs.flat(),
  };
  expect(channels(fixture.referenceObj)).toEqual(expected);
  if (process.env.SEMIO_TEST_OBJ_CORNERS) {
    expect(channels(readFileSync(process.env.SEMIO_TEST_OBJ_CORNERS, "utf8"))).toEqual(expected);
  }
  if (process.env.SEMIO_TEST_OBJ_NORMAL_ONLY) {
    const withoutUv = readFileSync(process.env.SEMIO_TEST_OBJ_NORMAL_ONLY, "utf8");
    expect(channels(withoutUv)).toEqual({ ...expected, uv: undefined });
    expect(withoutUv.split("\n").filter(line => line.startsWith("f ")).every(line => /\d+\/\/\d+/.test(line))).toBe(true);
  }
});

test("Three OBJLoader matches the relative OBJ tuple and quad triangulation", () => {
  expect(channels(fixture.relativeObj)).toEqual(channels(fixture.referenceObj));
  const source = channels(fixture.quadObj);
  expect(source.position?.length).toBe(fixture.quad.triangles * 9);
  expect(source.uv).toEqual([0,0, 1,0, 1,1, 0,0, 1,1, 0,1]);
  if (process.env.SEMIO_TEST_OBJ_IMPORTED) {
    expect(channels(readFileSync(process.env.SEMIO_TEST_OBJ_IMPORTED, "utf8"))).toEqual(channels(fixture.relativeObj));
  }
  if (process.env.SEMIO_TEST_OBJ_QUAD) {
    expect(channels(readFileSync(process.env.SEMIO_TEST_OBJ_QUAD, "utf8"))).toEqual(source);
  }
});

test("Three OBJLoader retains independent streams through a wide relative-index source", () => {
  const wide = channels(fixture.relativeObj.repeat(fixture.growth.copies));
  const expected = channels(fixture.relativeObj);
  const nativePath = process.env.SEMIO_TEST_OBJ_WIDE;
  const collected = nativePath ? JSON.parse(readFileSync(nativePath, "utf8")) as Record<string, number | number[]>[] : [];
  for (const name of ["position", "normal", "uv"]) {
    const sample = expected[name]!;
    expect(wide[name]?.length).toBe(sample.length * fixture.growth.copies);
    for (const copy of [0, Math.floor(fixture.growth.copies / 2), fixture.growth.copies - 1]) {
      const actual = wide[name]?.slice(copy * sample.length, (copy + 1) * sample.length);
      expect(actual).toEqual(sample);
      if (nativePath) expect(collected.find(row => row.copy === copy)?.[name]).toEqual(actual);
    }
  }
});

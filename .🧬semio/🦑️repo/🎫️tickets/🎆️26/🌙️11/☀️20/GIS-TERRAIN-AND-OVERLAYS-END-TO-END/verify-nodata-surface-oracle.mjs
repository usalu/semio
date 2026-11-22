import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { BufferGeometry, DoubleSide, Float32BufferAttribute, Mesh, MeshBasicMaterial, Raycaster, Vector3 } from "three";

const folder = process.argv[2];
const cases = JSON.parse(readFileSync(folder + "/fixture.json", "utf8"));
const observed = new Map(readFileSync(folder + "/witness.jsonl", "utf8").trim().split("\n").map((line) => {
  const result = JSON.parse(line);
  return [result.name, result];
}));
const near = (actual, expected, tolerance) => assert.ok(Math.abs(actual - expected) <= tolerance, actual + " differs from " + expected);

for (const { name, source, facesNorthFirst } of cases) {
  const result = observed.get(name);
  const [west, south, east, north] = source.bounds;
  const width = source.rowsMillimetres[0].length;
  const height = source.rowsMillimetres.length;
  const indices = facesNorthFirst.flat();
  const positions = [];
  for (let row = 0; row < height; row++) {
    const lat = north + (south - north) * row / (height - 1);
    for (let col = 0; col < width; col++) {
      const lon = west + (east - west) * col / (width - 1);
      positions.push(Math.fround((lon - west) * 111_320 * Math.cos(north * Math.PI / 180)),
        Math.fround((lat - north) * 111_320), Math.fround(source.rowsMillimetres[height - 1 - row][col] / 1000));
    }
  }
  assert.deepEqual(result.indices, indices);
  if (!indices.length) {
    assert.deepEqual(result.positions, []);
    continue;
  }
  for (const vertex of new Set(indices)) {
    for (let axis = 0; axis < 3; axis++) near(result.positions[3 * vertex + axis], positions[3 * vertex + axis], 0.06);
  }
  const geometry = new BufferGeometry().setIndex(indices).setAttribute("position", new Float32BufferAttribute(positions, 3));
  const material = new MeshBasicMaterial({ side: DoubleSide });
  const mesh = new Mesh(geometry, material);
  try {
    const heightAt = (x, y) => new Raycaster(new Vector3(x, y, 3_000_000), new Vector3(0, 0, -1)).intersectObject(mesh)[0]?.point.z;
    for (const [x, y, expected] of result.picks) {
      const actual = heightAt(x, y);
      if (expected === null) assert.equal(actual, undefined);
      else near(actual, expected, 0.01);
    }
    const maximumHeight = Math.max(...indices.map((vertex) => Math.abs(positions[3 * vertex + 2])));
    const drapeTolerance = Math.max(0.01, maximumHeight * 4 * 2 ** -23);
    for (const path of result.drapes) for (const [a, b] of path) {
      const [x, y, z] = a.map((value, axis) => (value + b[axis]) / 2);
      near(heightAt(x, y), z, drapeTolerance);
    }
  } finally {
    geometry.dispose();
    material.dispose();
  }
}
console.log("Three agreed with staged native faces, picks, and draped segments for the NoData fixture");

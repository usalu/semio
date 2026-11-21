import assert from "node:assert/strict";
import { BufferGeometry, DoubleSide, Float32BufferAttribute, Mesh, MeshBasicMaterial, Raycaster, Vector3 } from "three";

const chunks = [];
for await (const data of process.stdin) chunks.push(data);
const { fixture, actual } = JSON.parse(Buffer.concat(chunks).toString());
const { grid, shape, expected, origin, exaggeration, maximumBytes, maximumShape } = fixture;
const [west, south, east, north] = grid.bounds;
const [width, height] = shape;
const oraclePositions = [];
for (let y = 0; y < height; y++) {
  const lat = north - (north - south) * y / (height - 1);
  for (let x = 0; x < width; x++) {
    const lon = west + (east - west) * x / (width - 1);
    oraclePositions.push((lon - origin[0]) * 111_320 * Math.cos(origin[1] * Math.PI / 180),
      (lat - origin[1]) * 111_320, grid.rowsMillimetres[height - 1 - y][x] * exaggeration / 1_000);
  }
}
const near = (value, expected, epsilon = 1e-5) => assert.ok(Math.abs(value - expected) <= epsilon, value + " differs from " + expected);
assert.deepEqual(actual.indices, expected.indices);
assert.deepEqual(actual.sample, expected.originSample);
assert.equal(actual.maximumBytes, maximumBytes);
assert.equal(actual.maximumIndices, 6 * (maximumShape[0] - 1) * (maximumShape[1] - 1));
new Float32Array(oraclePositions).forEach((value, index) => near(actual.positions[index], value, 1e-4));

const geometry = new BufferGeometry().setIndex(expected.indices);
geometry.setAttribute("position", new Float32BufferAttribute(oraclePositions, 3));
const material = new MeshBasicMaterial({ side: DoubleSide });
const mesh = new Mesh(geometry, material);
const elevation = (x, y) => new Raycaster(new Vector3(x, y, 20), new Vector3(0, 0, -1)).intersectObject(mesh)[0].point.z;
near(expected.originHeight, elevation(0, 0));
near(actual.pick[2], elevation(0, 0));
near(actual.originElevation, elevation(0, 0));
near(actual.marker[2], elevation(0, 0));
for (const path of [actual.route, actual.region]) {
  assert.ok(path.length > 3);
  for (const [a, b] of path) {
    const [x, y, z] = a.map((value, axis) => (value + b[axis]) / 2);
    near(z, elevation(x, y));
  }
}
geometry.dispose();
material.dispose();
console.log("Three.js agrees with the fractional rectangular mesh, origin, ray pick, route and region draping. The authored buffer bound is 233,504 bytes.");

import { Buffer } from "node:buffer";
import { expect, test } from "bun:test";
import { BoxGeometry, BufferGeometry, Mesh, Triangle, Vector3 } from "three";
import { STLExporter } from "three/addons/exporters/STLExporter.js";
import { OBJLoader } from "three/addons/loaders/OBJLoader.js";
import { STLLoader } from "three/addons/loaders/STLLoader.js";
import fixture from "./🔣️original-mesh-export.json";

const point = (positions: ArrayLike<number>, i: number) =>
  new Vector3(positions[i * 3], positions[i * 3 + 1], positions[i * 3 + 2]);

function measure(geometry: BufferGeometry) {
  const positions = geometry.getAttribute("position").array;
  const indices = geometry.getIndex()?.array ?? Array.from({ length: positions.length / 3 }, (_, index) => index);
  let area = 0;
  let volume = 0;
  for (let i = 0; i < indices.length; i += 3) {
    const [a, b, c] = [point(positions, indices[i]!), point(positions, indices[i + 1]!), point(positions, indices[i + 2]!)];
    area += new Triangle(a, b, c).getArea();
    volume += a.dot(b.clone().cross(c)) / 6;
  }
  return { area, volume, triangles: indices.length / 3 };
}

test("Three.js loads the retained writer fixture and agrees with the binary STL facet bytes", () => {
  const obj = new OBJLoader().parse(fixture.obj);
  const mesh = obj.children[0] as Mesh;
  expect(measure(mesh.geometry).area).toBe(fixture.area);
  expect(Array.from(mesh.geometry.getAttribute("position").array)).toEqual(
    fixture.mesh.indices.flatMap(index => fixture.mesh.positions.slice(index * 3, index * 3 + 3)),
  );
  const actual = Uint8Array.from(Buffer.from(fixture.stl, "base64"));
  const independent = new STLExporter().parse(mesh, { binary: true });
  for (let start = 84; start < independent.byteLength; start += 50) {
    for (let offset = start; offset < start + 48; offset += 4) {
      if (independent.getFloat32(offset, true) === 0) independent.setFloat32(offset, 0, true);
    }
  }
  const independentBytes = new Uint8Array(independent.buffer, independent.byteOffset, independent.byteLength);
  expect(actual.slice(80)).toEqual(independentBytes.slice(80));
  expect(measure(new STLLoader().parse(actual.buffer)).area).toBe(fixture.area);
});

test("Three.js independently measures the certified box used in the retained exchange roundtrip", () => {
  const [width, depth, height] = fixture.closedBox.size;
  const { area, volume, triangles } = measure(new BoxGeometry(width, depth, height));
  expect({ area, triangles }).toEqual({ area: fixture.closedBox.area, triangles: fixture.closedBox.triangles });
  expect(volume).toBeCloseTo(fixture.closedBox.volume, 12);
});

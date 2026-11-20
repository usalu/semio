import { Buffer } from "node:buffer";
import { expect, test } from "bun:test";
import { Triangle, Vector3 } from "three";
import { STLLoader } from "three/addons/loaders/STLLoader.js";
import fixture from "./🔣️original-stl-import.json";

test("Three.js STLLoader agrees for binary and ASCII triangles", () => {
  for (const row of fixture.cases) {
    const raw = Uint8Array.from(Buffer.from(row.base64.trim(), "base64"));
    const geometry = new STLLoader().parse(raw.buffer);
    const positions = geometry.getAttribute("position");
    const vertices = new Set<string>();
    const edges = new Set<string>();
    let area = 0;
    let volume = 0;
    for (let index = 0; index < positions.count; index += 3) {
      const [a, b, c] = [0, 1, 2].map(offset => new Vector3().fromBufferAttribute(positions, index + offset)) as [Vector3, Vector3, Vector3];
      area += new Triangle(a, b, c).getArea();
      volume += a.dot(b.clone().cross(c)) / 6;
      const keys = [a, b, c].map(p => p.toArray().join(","));
      keys.forEach((key, i) => {
        vertices.add(key);
        edges.add([key, keys[(i + 1) % 3]].sort().join("/"));
      });
    }
    expect([vertices.size, edges.size, positions.count / 3], row.name).toEqual([row.vertices, row.edges, row.faces]);
    expect(area, row.name).toBeCloseTo(row.area, 12);
    expect(volume, row.name).toBeCloseTo(row.volume, 12);
  }
});

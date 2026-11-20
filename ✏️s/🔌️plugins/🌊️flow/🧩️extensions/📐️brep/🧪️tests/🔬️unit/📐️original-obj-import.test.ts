import { expect, test } from "bun:test";
import { BufferAttribute, Mesh, Triangle, Vector3 } from "three";
import { OBJLoader } from "three/addons/loaders/OBJLoader.js";
import fixture from "./🔣️original-obj-import.json";

test("Three.js OBJLoader agrees on triangle counts, area and signed volume", () => {
  for (const row of fixture.cases) {
    const parsed = new OBJLoader().parse(row.obj);
    const vertices = new Set<string>();
    const edges = new Set<string>();
    let faces = 0;
    let area = 0;
    let volume = 0;
    parsed.traverse(object => {
      if (!(object instanceof Mesh)) return;
      const positions = object.geometry.getAttribute("position") as BufferAttribute;
      const point = (index: number) => new Vector3().fromBufferAttribute(positions, index);
      for (let i = 0; i < positions.count; i += 3) {
        const [a, b, c] = [point(i), point(i + 1), point(i + 2)];
        ++faces;
        area += new Triangle(a, b, c).getArea();
        volume += a.dot(b.clone().cross(c)) / 6;
        const keys = [a, b, c].map(position => position.toArray().join(","));
        keys.forEach((key, k) => {
          vertices.add(key);
          edges.add([key, keys[(k + 1) % 3]].sort().join("/"));
        });
      }
    });
    expect([vertices.size, edges.size, faces], row.name).toEqual([row.vertices, row.edges, row.faces]);
    expect(area, row.name).toBeCloseTo(row.area, 12);
    expect(volume, row.name).toBeCloseTo(row.volume, 12);
  }
});

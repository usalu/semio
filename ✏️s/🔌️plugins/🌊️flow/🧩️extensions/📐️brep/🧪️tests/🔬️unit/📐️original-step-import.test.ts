import { readFileSync } from "node:fs";
import { expect, test } from "bun:test";
import { BoxGeometry, Triangle, Vector3 } from "three";
import fixture from "./🔣️original-step-import.json";

test("Three.js agrees with the original STEP box extents, boundary and measures", () => {
  for (const row of fixture.cases) {
    const source = readFileSync(new URL(row.file, import.meta.url), "utf8");
    expect(source).toContain("Open CASCADE STEP translator");
    const [w, d, h] = row.size as [number, number, number];
    const [x, y, z] = row.origin as [number, number, number];
    const mesh = new BoxGeometry(w, d, h).translate(x + w / 2, y + d / 2, z + h / 2);
    mesh.computeBoundingBox();
    expect(mesh.boundingBox!.min.toArray()).toEqual(row.origin);
    expect(mesh.boundingBox!.max.toArray()).toEqual([x + w, y + d, z + h]);
    const positions = mesh.getAttribute("position");
    const vertex = (i: number) => new Vector3().fromBufferAttribute(positions, i);
    const indices = mesh.getIndex()!;
    let area = 0;
    let volume = 0;
    const corners = new Set<string>();
    const edgeUses = new Map<string, { count: number; sense: number }>();
    for (let i = 0; i < indices.count; i += 3) {
      const [a, b, c] = [vertex(indices.getX(i)), vertex(indices.getX(i + 1)), vertex(indices.getX(i + 2))];
      area += new Triangle(a!, b!, c!).getArea();
      volume += a!.dot(b!.clone().cross(c!)) / 6;
      for (const [from, to] of [[a!, b!], [b!, c!], [c!, a!]]) {
        const f = from!.toArray().join(",");
        const t = to!.toArray().join(",");
        corners.add(f);
        const edge = f < t ? f + "|" + t : t + "|" + f;
        const seen = edgeUses.get(edge) ?? { count: 0, sense: 0 };
        seen.count++;
        seen.sense += f < t ? 1 : -1;
        edgeUses.set(edge, seen);
      }
    }
    expect(corners.size).toBe(row.vertices);
    expect(indices.count / 6).toBe(row.faces);
    expect(edgeUses.size - row.faces).toBe(row.edges);
    for (const uses of edgeUses.values()) expect(uses).toEqual({ count: 2, sense: 0 });
    expect(area).toBe(row.area);
    expect(volume).toBeCloseTo(row.volume, 10);
  }
});

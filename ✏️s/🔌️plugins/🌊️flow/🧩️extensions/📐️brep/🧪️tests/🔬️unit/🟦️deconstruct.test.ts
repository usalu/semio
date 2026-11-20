import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import fixture from "./🔣️deconstruct.json";

test("a Manifold box verifies the reachable BRep component counts", async () => {
  const wasm = await loadManifold();
  wasm.setup();
  const box = wasm.Manifold.cube(fixture.box as [number, number, number]);
  try {
    const mesh = box.getMesh();
    const vertices = new Set<string>();
    const edges = new Map<string, Set<string>>();
    const faces = new Set<string>();
    for (let i = 0; i < mesh.triVerts.length; i += 3) {
      const points = Array.from(mesh.triVerts.subarray(i, i + 3), vertex =>
        Array.from(mesh.vertProperties.subarray(vertex * mesh.numProp, vertex * mesh.numProp + 3)));
      const axis = points[0].findIndex((coordinate, index) =>
        points.every(point => point[index] === coordinate));
      expect(axis).toBeGreaterThanOrEqual(0);
      const face = String(axis) + "=" + String(points[0][axis]);
      faces.add(face);
      const corners = points.map(point => point.join(","));
      corners.forEach((corner, index) => {
        vertices.add(corner);
        const edge = [corner, corners[(index + 1) % 3]].sort().join("|");
        const adjacent = edges.get(edge) ?? new Set<string>();
        adjacent.add(face);
        edges.set(edge, adjacent);
      });
    }
    expect(vertices.size).toBe(fixture.counts.vertex);
    expect([...edges.values()].filter(faces => faces.size === 2).length).toBe(fixture.counts.edge);
    expect(faces.size).toBe(fixture.counts.face);
    const shells = box.decompose();
    expect(shells.length).toBe(fixture.counts.shell);
    shells.forEach(shell => shell.delete());
  } finally {
    box.delete();
  }
});

test("source label text is exact positive uint64 JSON and sorts numerically", () => {
  const labels = (text: string): string[] => {
    const values: unknown = JSON.parse(text);
    if (!Array.isArray(values) || values.some(value =>
      typeof value !== "string" || !/^[1-9][0-9]*$/.test(value) || BigInt(value) > (1n << 64n) - 1n)) {
      throw new RangeError("Invalid persistent label array");
    }
    return [...new Set(values as string[])].sort((a, b) => BigInt(a) < BigInt(b) ? -1 : 1);
  };
  for (const { text, unique } of fixture.labels.accepted) expect(labels(text)).toEqual(unique);
  for (const text of fixture.labels.rejected) expect(() => labels(text)).toThrow();
});

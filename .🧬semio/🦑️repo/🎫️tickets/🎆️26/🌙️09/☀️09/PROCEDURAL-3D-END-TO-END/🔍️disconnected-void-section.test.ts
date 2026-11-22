/** 🕳️ Measures one material face cut across both arms of an enclosed concave void. */
import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import fixture from "./🔣️disconnected-void-section.json";

test("one concave void contributes two independent holes", async () => {
  const wasm = await loadManifold();
  wasm.setup();
  const stock = wasm.Manifold.cube(fixture.stock as [number, number, number]);
  const profile = new wasm.CrossSection(fixture.void.profile as [number, number][]);
  const prism = profile.extrude(fixture.void.height);
  const opening = prism.translate([0, 0, fixture.void.base]);
  const solid = stock.subtract(opening);
  const turned = solid.rotate([90, 0, 0]);
  const slice = turned.slice(fixture.cut);
  const faces = slice.decompose();
  try {
    expect(faces).toHaveLength(1);
    expect(slice.area()).toBe(fixture.area);
    const signed = faces[0].toPolygons().map(ring => ring.reduce((sum, [x, y], i) => {
      const [nextX, nextY] = ring[(i + 1) % ring.length];
      return sum + x * nextY - y * nextX;
    }, 0) / 2);
    expect(signed.sort((a, b) => b - a)).toEqual(fixture.rings);
  } finally {
    for (const shape of [stock, profile, prism, opening, solid, turned, slice, ...faces]) shape.delete();
  }
});

test("an annular cavity leaves a separate material island", async () => {
  const wasm = await loadManifold();
  wasm.setup();
  const row = fixture.island;
  const stock = wasm.Manifold.cube(row.stock as [number, number, number]);
  const exterior = new wasm.CrossSection(row.outer as [number, number][]);
  const interior = new wasm.CrossSection(row.inner as [number, number][]);
  const ring = exterior.subtract(interior);
  const prism = ring.extrude(row.height);
  const cavity = prism.translate([0, 0, row.base]);
  const solid = stock.subtract(cavity);
  const slice = solid.slice(row.cut);
  const faces = slice.decompose();
  try {
    const components = faces.map(face => ({
      area: face.area(),
      rings: face.toPolygons().map(points => points.reduce((sum, [x, y], i) => {
        const [nextX, nextY] = points[(i + 1) % points.length];
        return sum + x * nextY - y * nextX;
      }, 0) / 2).sort((a, b) => b - a),
    })).sort((a, b) => a.area - b.area);
    expect(components).toEqual(row.components);
  } finally {
    for (const shape of [stock, exterior, interior, ring, prism, cavity, solid, slice, ...faces]) shape.delete();
  }
});

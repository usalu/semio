import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import fixture from "./🔣️retained-defeature.json";

const wasm = await loadManifold();
wasm.setup();
type Vec3 = [number, number, number];

test("Manifold independently measures each restored box", () => {
  for (const row of fixture.cases) {
    const stock = wasm.Manifold.cube(row.size as Vec3);
    const placed = stock.translate(row.origin as Vec3);
    try {
      const { min, max } = placed.boundingBox();
      expect([min, max]).toEqual(row.bounds);
      expect(placed.volume()).toBeCloseTo(row.volume, 10);
      expect(placed.surfaceArea()).toBeCloseTo(row.area, 10);
      expect(placed.genus()).toBe(0);
      expect(placed.numVert()).toBe(fixture.vertexCount);
    } finally {
      placed.delete();
      stock.delete();
    }
  }
});
test("Manifold restores the original stock across both through-bore openings", () => {
  const row = fixture.throughBore;
  const stock = wasm.Manifold.cube(row.size as Vec3);
  const base = stock.translate(row.origin as Vec3);
  const cylinder = wasm.Manifold.cylinder(row.boreHeight, row.radius, row.radius, 96);
  const tool = cylinder.translate(row.boreOrigin as Vec3);
  const cut = base.subtract(tool);
  const removed = base.intersect(tool);
  const filled = cut.add(removed);
  try {
    expect(cut.genus()).toBe(1);
    expect(filled.genus()).toBe(0);
    const { min, max } = filled.boundingBox();
    expect([min, max]).toEqual(row.bounds);
    expect(filled.volume()).toBeCloseTo(row.volume, 10);
    expect(filled.surfaceArea()).toBeCloseTo(row.area, 10);
  } finally {
    for (const solid of [filled, removed, cut, tool, cylinder, base, stock]) solid.delete();
  }
});


test("Manifold restores both disjoint through bores in one box", () => {
  const row = fixture.twoThroughBores;
  const stock = wasm.Manifold.cube(row.size as Vec3);
  const cutters = row.tools.map(tool => {
    const cylinder = wasm.Manifold.cylinder(tool.height, tool.radius, tool.radius, 96);
    try {
      return cylinder.translate(tool.origin as Vec3);
    } finally {
      cylinder.delete();
    }
  });
  const [a, b] = cutters;
  const tools = a.add(b);
  const bored = stock.subtract(tools);
  const removed = stock.intersect(tools);
  const repaired = bored.add(removed);
  try {
    expect(bored.genus()).toBe(2);
    expect(repaired.genus()).toBe(0);
    const { min, max } = repaired.boundingBox();
    expect([min, max]).toEqual(row.bounds);
    expect(repaired.volume()).toBeCloseTo(row.volume, 10);
    expect(repaired.surfaceArea()).toBeCloseTo(row.area, 10);
  } finally {
    for (const solid of [repaired, removed, bored, tools, a, b, stock]) solid.delete();
  }
});


test("Manifold restores blind bores opening through either cap", () => {
  for (const row of fixture.blindBores) {
    const stock = wasm.Manifold.cube(row.size as Vec3);
    const cylinder = wasm.Manifold.cylinder(row.boreHeight, row.radius, row.radius, 96);
    const tool = cylinder.translate(row.boreOrigin as Vec3);
    const bored = stock.subtract(tool);
    const removed = stock.intersect(tool);
    const repaired = bored.add(removed);
    try {
      const boreLength = row.openEnd === "top" ? row.size[2] - row.floorZ : row.floorZ;
      const missing = Math.PI * row.radius ** 2 * boreLength;
      expect(stock.volume() - bored.volume()).toBeCloseTo(missing, 3);
      expect(bored.genus()).toBe(0);
      const { min, max } = repaired.boundingBox();
      expect([min, max]).toEqual(row.bounds);
      expect(repaired.volume()).toBeCloseTo(row.volume, 10);
      expect(repaired.surfaceArea()).toBeCloseTo(row.area, 10);
      expect(repaired.genus()).toBe(0);
    } finally {
      for (const solid of [repaired, removed, bored, tool, cylinder, stock]) solid.delete();
    }
  }
});

import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import fixture from "./🔣️original-solid-offset.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;

test("Manifold confirms sharp expansion and erosion of a translated box", () => {
  const canonical = Manifold.cube(fixture.source.size as [number, number, number]);
  const original = canonical.translate(fixture.source.origin as [number, number, number]);
  for (const row of fixture.cases) {
    const span = 2 * Math.abs(row.distance);
    const cutter = Manifold.cube([span, span, span], true);
    const offset = row.distance > 0 ? original.minkowskiSum(cutter) : original.minkowskiDifference(cutter);
    const bounds = offset.boundingBox();
    expect(bounds.min).toEqual(row.bounds[0]);
    expect(bounds.max).toEqual(row.bounds[1]);
    expect(offset.volume()).toBeCloseTo(row.volume, 12);
    expect(offset.genus()).toBe(0);
    offset.delete();
    cutter.delete();
  }
  const span = 2 * Math.abs(fixture.collapse);
  const cutter = Manifold.cube([span, span, span], true);
  const collapsed = original.minkowskiDifference(cutter);
  expect(collapsed.isEmpty()).toBe(true);
  expect(original.volume()).toBeCloseTo(fixture.source.volume, 12);
  for (const item of [collapsed, cutter, original, canonical]) item.delete();
});

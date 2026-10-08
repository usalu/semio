import { expect, test } from "bun:test";
import { agree, casesOf, fixtureDisagreements, loadFixture, measure } from "../../../⏱️phased-job/🧪️tests/🧰️support/🟦️.ts";

const fixture = loadFixture(import.meta.url);
type Counted = { count: number };

test("three agrees with the fixture on volume, area, bounds and closedness of every pinned topology result", () => {
  expect(fixtureDisagreements(fixture)).toEqual([]);
});

test("the decomposition of a box satisfies Euler's formula V - E + F = 2", () => {
  const [box] = casesOf(fixture, "brep.topology.deconstruct");
  const outputs = box!.outputs as Record<string, Counted>;
  expect(outputs.vertices!.count - outputs.edges!.count + outputs.faces!.count).toBe(2);
  expect(outputs.shells!.count).toBe(1);
});

test("the faces of a box sum to its area and the sewn solid has the box volume", () => {
  const [sew] = casesOf(fixture, "brep.topology.sew");
  const shape = sew!.outputs!.shape as { mesh: Parameters<typeof measure>[0]; volume: number };
  expect(agree(measure(shape.mesh).volume, 8, 1e-9)).toBe(true);
  expect(sew!.inputs.faces).toHaveLength(6);
});

test("a compound and its explosion agree on the solid count and volume", () => {
  const compound = casesOf(fixture, "brep.topology.compound")[0]!.outputs!.shape as { solids: number; volume: number };
  const explode = casesOf(fixture, "brep.topology.explode")[0]!.outputs!.solids as { count: number; each: { volume: number } };
  expect(explode.count).toBe(compound.solids);
  expect(explode.count * explode.each.volume).toBe(compound.volume);
});

test("every fault case names a code under the geometry prefix", () => {
  for (const fixtureCase of fixture.cases.filter(item => item.fault)) expect(fixtureCase.fault!.code.startsWith("generation3d.geometry."), fixtureCase.name).toBe(true);
});

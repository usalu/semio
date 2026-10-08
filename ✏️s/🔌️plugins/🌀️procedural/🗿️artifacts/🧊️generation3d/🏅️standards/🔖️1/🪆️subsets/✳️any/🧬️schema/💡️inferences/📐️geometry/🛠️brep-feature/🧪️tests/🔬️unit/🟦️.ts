import { expect, test } from "bun:test";
import { agree, casesOf, fixtureDisagreements, loadFixture, measure } from "../../../⏱️phased-job/🧰️test-support/🟦️.ts";

const fixture = loadFixture(import.meta.url);

test("three agrees with the fixture on volume, area, bounds and closedness of every pinned feature mesh", () => {
  expect(fixtureDisagreements(fixture)).toEqual([]);
});

test("every shape output of a successful feature case is pinned with a tessellation", () => {
  for (const fixtureCase of fixture.cases) {
    if (!fixtureCase.outputs) continue;
    const shape = fixtureCase.outputs.shape as { mesh?: unknown; kind?: string };
    expect(shape.mesh, fixtureCase.name).toBeDefined();
  }
});

test("the stated fillet and chamfer numbers follow the closed forms", () => {
  const edge = 2;
  const radius = 0.25;
  const strip = (1 - Math.PI / 4) * radius * radius * edge;
  const fillet = casesOf(fixture, "brep.feature.filletEdges").find(fixtureCase => fixtureCase.name === "fillet one edge")!;
  const shape = fillet.outputs!.shape as { volume: number; area: number };
  expect(agree(shape.volume, 8 - strip, 1e-9)).toBe(true);
  expect(agree(shape.area, 24 - 2 * radius * edge + (Math.PI / 2) * radius * edge - 2 * (1 - Math.PI / 4) * radius * radius, 1e-9)).toBe(true);
  const chamfer = casesOf(fixture, "brep.feature.chamferEdges")[0]!.outputs!.shape as { volume: number; area: number };
  expect(agree(chamfer.volume, 8 - (0.25 * 0.25) / 2 * edge, 1e-9)).toBe(true);
  expect(agree(chamfer.area, 24 - 2 * 0.25 * edge + Math.SQRT2 * 0.25 * edge - 0.25 * 0.25, 1e-9)).toBe(true);
});

test("a shell and an offset keep the stated volumes in the pinned meshes", () => {
  const shell = casesOf(fixture, "brep.feature.shell").find(fixtureCase => fixtureCase.name === "shell with the top face open")!;
  const mesh = (shell.outputs!.shape as { mesh: Parameters<typeof measure>[0] }).mesh;
  expect(agree(measure(mesh).volume, 8 - 1.6 * 1.6 * 1.8, 1e-6)).toBe(true);
  const shrunk = casesOf(fixture, "brep.feature.offsetSolid").find(fixtureCase => fixtureCase.name === "offset shrinks the solid")!;
  expect(agree(measure((shrunk.outputs!.shape as { mesh: Parameters<typeof measure>[0] }).mesh).volume, 1.5 ** 3, 1e-6)).toBe(true);
});

test("every fault case names a code under the geometry prefix", () => {
  for (const fixtureCase of fixture.cases.filter(item => item.fault)) expect(fixtureCase.fault!.code.startsWith("generation3d.geometry."), fixtureCase.name).toBe(true);
});

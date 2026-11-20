import { expect, test } from "bun:test";
import { LineCurve3, Vector3 } from "three";
import { readFileSync } from "node:fs";
import { disagreements, type FixtureCase } from "../../../🧪️tests/🧰️oracle-support/🟦️.ts";
import { oracleCase } from "../../../🧪️tests/🧰️construction-support/🟦️.ts";

type Fixture = { tolerance: number; cases: FixtureCase[] };
const fixture = JSON.parse(readFileSync(new URL("../../../../../../🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏗️brep-solid/🔣️.json", import.meta.url), "utf8")) as Fixture;
const catalogue = JSON.parse(readFileSync(new URL("../../../../../🗂️catalogue/🔣️brep-solid.json", import.meta.url), "utf8")) as { kinds: { id: string }[] };

test("the fixture states every brep.solid kind of the catalogue and no other", () => {
  expect([...new Set(fixture.cases.map(({ kind }) => kind))].sort()).toEqual(catalogue.kinds.map(({ id }) => id).sort());
});

test("every fixture case states a fault or at least one measure", () => {
  for (const { name, outputs, fault } of fixture.cases) expect(fault !== undefined || Object.keys((outputs?.shape ?? {}) as object).length > 1, name).toBe(true);
});

test("three recomputes the volume, area, face count and bounding box of every solid case (signed tetrahedra on ruled solids, LatheGeometry, Pappus integrals)", () => {
  expect(disagreements(fixture.cases, oracleCase, fixture.tolerance)).toEqual([]);
}, { timeout: 120000 });


test("Three.js detects the open line and in-plane circular extrusion fixtures", () => {
  type XYZ = [number, number, number];
  const open = fixture.cases.find(({ name }) => name === "refuse an open line as an extrusion profile")!;
  const { start, end } = (open.inputs as { wire: { inputs: { start: XYZ; end: XYZ } } }).wire.inputs;
  const line = new LineCurve3(new Vector3(...start), new Vector3(...end));
  expect(line.getPoint(0).distanceTo(line.getPoint(1))).toBeGreaterThan(0);
  expect(open.fault?.code).toBe("generation3d.geometry.wire-open");

  const flat = fixture.cases.find(({ name }) => name === "refuse a circular extrusion in the profile plane")!;
  const { wire, vector } = flat.inputs as { wire: { inputs: { normal: XYZ } }; vector: XYZ };
  expect(new Vector3(...wire.inputs.normal).dot(new Vector3(...vector))).toBe(0);
  expect(flat.fault?.code).toBe("generation3d.geometry.extrusion-in-plane");
});

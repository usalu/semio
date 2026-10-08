import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { disagreements, type FixtureCase } from "../../../🧪️tests/🧰️oracle-support/🟦️.ts";
import { oracleCase } from "../../../🧪️tests/🧰️construction-support/🟦️.ts";

type Fixture = { tolerance: number; cases: FixtureCase[] };
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8")) as Fixture;
const catalogue = JSON.parse(readFileSync(new URL("../../../../../🗂️catalogue/🔣️brep-curve.json", import.meta.url), "utf8")) as { kinds: { id: string }[] };

test("the fixture states every brep.curve kind of the catalogue and no other", () => {
  expect([...new Set(fixture.cases.map(({ kind }) => kind))].sort()).toEqual(catalogue.kinds.map(({ id }) => id).sort());
});

test("every fixture case states a fault or at least one measure", () => {
  for (const { name, outputs, fault } of fixture.cases) expect(fault !== undefined || Object.keys((outputs?.shape ?? {}) as object).length > 1, name).toBe(true);
});

test("three recomputes the length, ends, closure, bounding box and spline passage of every curve and wire case (EllipseCurve, CatmullRomCurve3 and plain polyline sums)", { timeout: 120000 }, () => {
  expect(disagreements(fixture.cases, oracleCase, fixture.tolerance)).toEqual([]);
});

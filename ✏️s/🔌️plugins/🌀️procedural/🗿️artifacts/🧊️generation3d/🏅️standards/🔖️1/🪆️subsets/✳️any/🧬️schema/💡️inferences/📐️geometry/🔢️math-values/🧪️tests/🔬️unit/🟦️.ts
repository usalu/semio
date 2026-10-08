import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { Plane, Vector3 } from "three";
import { disagreements, type FixtureCase, type OracleResult } from "../../../🧪️tests/🧰️oracle-support/🟦️.ts";

type Triple = [number, number, number];
type Fixture = { tolerance: number; cases: FixtureCase[] };
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8")) as Fixture;
const catalogue = JSON.parse(readFileSync(new URL("../../../../../🗂️catalogue/🔣️math-values.json", import.meta.url), "utf8")) as { kinds: { id: string }[] };

const triple = (v: Vector3): Triple => [v.x, v.y, v.z];

/** 🔢️ Recomputes one case with three's Vector3 and Plane: a constant provides exactly the value it is given. */
const oracle = ({ kind, inputs }: FixtureCase): OracleResult => {
  switch (kind) {
    case "math.number":
    case "math.integer":
    case "math.angle":
    case "math.length":
    case "math.boolean": return { outputs: { value: inputs.value } };
    case "math.vector":
    case "math.point": return { outputs: { value: triple(new Vector3(...(inputs.value as Triple))) } };
    case "math.plane": {
      const { origin, normal } = inputs.value as { origin: Triple; normal: Triple };
      const plane = new Plane().setFromNormalAndCoplanarPoint(new Vector3(...normal).normalize(), new Vector3(...origin));
      return { outputs: { value: { origin, normal: triple(plane.normal) } } };
    }
    default: throw new Error(`no oracle for ${kind}`);
  }
};

test("the fixture covers exactly the math.values catalogue kinds", () => {
  expect([...new Set(fixture.cases.map(({ kind }) => kind))].sort()).toEqual(catalogue.kinds.map(({ id }) => id).sort());
});

test("three recomputes every math.values case", () => {
  expect(disagreements(fixture.cases, oracle, fixture.tolerance)).toEqual([]);
});

test("a plane through the fixture origin with the fixture normal contains its origin in three", () => {
  const { origin, normal } = fixture.cases.find(({ kind }) => kind === "math.plane")!.inputs.value as { origin: Triple; normal: Triple };
  const plane = new Plane().setFromNormalAndCoplanarPoint(new Vector3(...normal), new Vector3(...origin));
  expect(Math.abs(plane.distanceToPoint(new Vector3(...origin)))).toBeLessThan(1e-12);
});

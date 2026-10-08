import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { MathUtils } from "three";
import { disagreements, type FixtureCase, type OracleResult } from "../../../🧪️tests/🧰️oracle-support/🟦️.ts";

type Fixture = { tolerance: number; cases: FixtureCase[] };
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8")) as Fixture;
const prefix = "generation3d.geometry.";

const overflowing = (numbers: number[]): OracleResult => (numbers.every(Number.isFinite) ? { outputs: { numbers } } : { fault: { code: `${prefix}math-overflow` } });

/** 📚️ Recomputes one list case with three's MathUtils. */
const oracle = ({ kind, inputs }: FixtureCase): OracleResult => {
  const count = inputs.count as number;
  switch (kind) {
    case "math.range": return overflowing(Array.from({ length: count }, (_, index) => (index === count - 1 ? (inputs.end as number) : MathUtils.lerp(inputs.start as number, inputs.end as number, index / (count - 1)))));
    case "math.series": return overflowing(Array.from({ length: count }, (_, index) => (inputs.start as number) + (inputs.step as number) * index));
    case "math.listItem": {
      const list = inputs.list as unknown[];
      const index = inputs.index as number;
      return Number.isInteger(index) && index >= 0 && index < list.length ? { outputs: { item: list[index] } } : { fault: { code: `${prefix}list-index`, port: "index" } };
    }
    case "math.listLength": return { outputs: { count: (inputs.list as unknown[]).length } };
    default: throw new Error(`no oracle for ${kind}`);
  }
};

test("every math.list kind has a fixture case and three reproduces every fixture outcome", () => {
  expect([...new Set(fixture.cases.map(fixtureCase => fixtureCase.kind))].sort()).toEqual(["math.listItem", "math.listLength", "math.range", "math.series"]);
  expect(disagreements(fixture.cases, oracle, fixture.tolerance)).toEqual([]);
});

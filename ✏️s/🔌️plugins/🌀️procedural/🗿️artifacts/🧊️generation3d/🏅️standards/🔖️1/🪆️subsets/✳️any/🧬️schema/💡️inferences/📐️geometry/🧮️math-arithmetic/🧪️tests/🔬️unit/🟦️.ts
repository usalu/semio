import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { MathUtils } from "three";
import { disagreements, type FixtureCase, type OracleResult } from "../../../🧪️tests/🧰️oracle-support/🟦️.ts";

type Fixture = { tolerance: number; faults: Record<"divisionByZero" | "domain" | "overflow" | "range", string>; cases: FixtureCase[] };
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8")) as Fixture;
const { divisionByZero, domain, overflow, range } = fixture.faults;

const value = (inputs: Record<string, unknown>, port: string): number => inputs[port] as number;
const finite = (outputs: Record<string, unknown>, port = "result"): OracleResult => (Number.isFinite(outputs[port] as number) ? { outputs } : { fault: { code: overflow } });
const nearest = (x: number): number => Math.sign(x) * Math.round(Math.abs(x));

/** 🧮️ Recomputes one arithmetic case with three's MathUtils and the JavaScript Math builtins. */
const oracle = ({ kind, inputs }: FixtureCase): OracleResult => {
  const [a, b] = [value(inputs, "a"), value(inputs, "b")];
  switch (kind) {
    case "math.add": return finite({ result: a + b });
    case "math.subtract": return finite({ result: a - b });
    case "math.multiply": return finite({ result: a * b });
    case "math.divide": return b === 0 ? { fault: { code: divisionByZero, port: "b" } } : finite({ result: a / b });
    case "math.modulo": return b === 0 ? { fault: { code: divisionByZero, port: "b" } } : finite({ result: a % b });
    case "math.power": {
      const [base, exponent] = [value(inputs, "base"), value(inputs, "exponent")];
      const result = Math.pow(base, exponent);
      if (Number.isNaN(result) || (base === 0 && exponent < 0)) return { fault: { code: domain, port: "base" } };
      return finite({ result });
    }
    case "math.negate": return { outputs: { result: 0 - value(inputs, "value") } };
    case "math.absolute": return { outputs: { result: Math.abs(value(inputs, "value")) } };
    case "math.squareRoot": return value(inputs, "value") < 0 ? { fault: { code: domain, port: "value" } } : { outputs: { result: Math.sqrt(value(inputs, "value")) } };
    case "math.round": {
      const x = value(inputs, "value");
      const rounded = inputs.mode === "down" ? Math.floor(x) : inputs.mode === "up" ? Math.ceil(x) : nearest(x);
      return Number.isSafeInteger(rounded) ? { outputs: { result: rounded } } : { fault: { code: overflow, port: "value" } };
    }
    case "math.minimum": return { outputs: { result: Math.min(a, b) } };
    case "math.maximum": return { outputs: { result: Math.max(a, b) } };
    case "math.clamp": {
      const [x, lower, upper] = [value(inputs, "value"), value(inputs, "lower"), value(inputs, "upper")];
      return lower > upper ? { fault: { code: range, port: "lower" } } : { outputs: { result: MathUtils.clamp(x, lower, upper) } };
    }
    case "math.interpolate": return finite({ result: MathUtils.lerp(a, b, value(inputs, "t")) });
    case "math.sine": return { outputs: { result: Math.sin(value(inputs, "angle")) } };
    case "math.cosine": return { outputs: { result: Math.cos(value(inputs, "angle")) } };
    case "math.tangent": return Math.abs(Math.cos(value(inputs, "angle"))) < 1e-12 ? { fault: { code: domain, port: "angle" } } : { outputs: { result: Math.tan(value(inputs, "angle")) } };
    default: throw new Error(`no oracle for ${kind}`);
  }
};

test("every math.arithmetic kind has a fixture case and three/Math reproduce every fixture outcome", () => {
  const kinds = new Set(fixture.cases.map(fixtureCase => fixtureCase.kind));
  expect([...kinds].sort()).toEqual(["math.absolute", "math.add", "math.clamp", "math.cosine", "math.divide", "math.interpolate", "math.maximum", "math.minimum", "math.modulo", "math.multiply", "math.negate", "math.power", "math.round", "math.sine", "math.squareRoot", "math.subtract", "math.tangent"]);
  expect(disagreements(fixture.cases, oracle, fixture.tolerance)).toEqual([]);
});

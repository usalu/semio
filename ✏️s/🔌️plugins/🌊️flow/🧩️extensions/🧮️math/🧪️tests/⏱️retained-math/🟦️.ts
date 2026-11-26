import { expect, test } from "bun:test"
import { abs, add, ceil, cos, divide, floor, max, min, multiply, pow, sin, sqrt, subtract, tan, unaryMinus, type MathType } from "mathjs"
import fixture from "../🧫️fixtures/⏱️retained-math/🔣️.json"

type NumberValue = { $schema: "number"; value: number }
type Coordinates = { $schema: "point" | "vector"; x: number; y: number; z: number }
type Operand = NumberValue | Coordinates
type Inputs = Record<string, unknown>

function numeric(value: MathType): number {
  if (typeof value !== "number") throw new TypeError("the numeric fixture expects a scalar")
  return value
}

const number = (value: MathType): NumberValue => ({ $schema: "number", value: numeric(value) })
const components = ({ x, y, z }: Coordinates): number[] => [x, y, z]
const point = (kind: Coordinates["$schema"], [x, y, z]: number[]): Coordinates => ({ $schema: kind, x, y, z })
const list = (value: unknown): Operand[] => {
  const entries = Object.entries(value as Record<string, Operand>)
  return entries
    .filter(([key]) => /^\d+$/.test(key))
    .sort(([left], [right]) => Number(left) - Number(right))
    .map(([, entry]) => entry)
}

function seedSample(seed: number): number {
  const bytes = new DataView(new ArrayBuffer(8))
  bytes.setFloat64(0, seed)
  let stage = bytes.getBigUint64(0)
  stage = BigInt.asUintN(64, stage + 0x9e3779b97f4a7c15n)
  stage = BigInt.asUintN(64, (stage ^ (stage >> 30n)) * 0xbf58476d1ce4e5b9n)
  stage = BigInt.asUintN(64, (stage ^ (stage >> 27n)) * 0x94d049bb133111ebn)
  return Number(stage ^ (stage >> 31n)) / 2 ** 64
}

function output(operator: string, input: Inputs): Record<string, Operand | null> {
  const channel = (key: string) => (input[key] as NumberValue).value
  const scalar = (key: string, value: MathType) => ({ [key]: number(value) })
  const binary = (key: string, operation: (a: number, b: number) => MathType) => scalar(key, operation(channel("a"), channel("b")))
  const unary = (key: string, operation: (n: number) => MathType) => scalar(key, operation(channel("number")))
  const pair = (key: string, operation: typeof add | typeof subtract) => {
    const { a, b } = input as { a: Operand; b: Operand }
    return a.$schema === "number" ? binary(key, operation) : { [key]: point(a.$schema, operation(components(a), components(b as Coordinates)) as number[]) }
  }
  const fold = (key: string) => {
    const items = list(input[key])
    const first = items[0]
    if (first === undefined || first.$schema === "number") {
      return scalar("sum", (items as NumberValue[]).reduce((total, value) => numeric(add(total, value.value)), 0))
    }
    const total = (items as Coordinates[]).reduce((total, value) => add(total, components(value)) as number[], [0, 0, 0])
    return { sum: point(first.$schema, total) }
  }

  switch (operator) {
    case "math.add": return pair("sum", add)
    case "math.subtract": return pair("difference", subtract)
    case "math.addVariadic": return fold("items")
    case "math.sum": return fold("list")
    case "math.move": {
      const { subject, offset } = input as { subject: Coordinates; offset: Coordinates }
      const kind = subject.$schema
      return { [kind]: point(kind, add(components(subject), components(offset)) as number[]), [kind === "point" ? "vector" : "point"]: null }
    }
    case "math.multiply": return binary("product", multiply)
    case "math.divide": return binary("quotient", divide)
    case "math.power": return binary("power", pow)
    case "math.modulo": return binary("modulo", (a, b) => subtract(a, multiply(Math.trunc(numeric(divide(a, b))), b)))
    case "math.min": return binary("minimum", min)
    case "math.max": return binary("maximum", max)
    case "math.negate": return unary("negated", unaryMinus)
    case "math.abs": return unary("absolute", abs)
    case "math.sqrt": return unary("root", sqrt)
    case "math.floor": return unary("floor", floor)
    case "math.ceil": return unary("ceiling", ceil)
    case "math.round": return unary("rounded", value => multiply(Math.sign(value), floor(numeric(add(abs(value), 0.5)))))
    case "math.sin": return unary("sine", sin)
    case "math.cos": return unary("cosine", cos)
    case "math.tan": return unary("tangent", tan)
    case "math.passThrough": return unary("numberOut", value => value)
    case "math.remap": {
      const fromMin = channel("fromMin")
      const toMin = channel("toMin")
      const fraction = divide(subtract(channel("value"), fromMin), subtract(channel("fromMax"), fromMin))
      return scalar("remapped", add(toMin, multiply(fraction, subtract(channel("toMax"), toMin))))
    }
    case "math.random": {
      const low = channel("min")
      return scalar("random", add(low, multiply(subtract(channel("max"), low), seedSample(channel("seed")))))
    }
    default: throw new Error(`No independent oracle for ${operator}`)
  }
}

test("the retained Math fixture agrees with mathjs", () => {
  expect(fixture.rows).toHaveLength(fixture.caseCount)
  for (const row of fixture.rows) {
    const actual = output(row.operator, row.input)
    expect(Object.keys(actual).sort(), row.id).toEqual(Object.keys(row.output).sort())
    for (const [key, payload] of Object.entries(row.output)) {
      const result = actual[key]
      if (payload?.$schema === "number" && result?.$schema === "number") {
        expect(result.value, row.id).toBeCloseTo(payload.value, 12)
      } else {
        expect(result, row.id).toEqual(payload)
      }
    }
  }
})

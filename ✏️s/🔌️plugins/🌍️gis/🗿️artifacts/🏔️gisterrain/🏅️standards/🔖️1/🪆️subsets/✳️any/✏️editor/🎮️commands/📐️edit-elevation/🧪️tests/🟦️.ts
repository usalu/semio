import { expect, test } from "bun:test";
import { abs, add, divide, floor, mean, multiply, sign, square, subtract } from "mathjs";
import Ajv from "ajv";
import schema from "../🔣️.json";
import fixture from "./🔣️.json";
import fillFixture from "./🌫️fill-missing/🔣️.json";

type Command = {
  operation: string;
  column: number;
  row: number;
  radius: number;
  value: number;
  stroke?: number[][];
};

const validate = new Ajv({ multipleOfPrecision: 6 }).compile(schema);
const { columns, rows, initialHeightMillimetres, seed } = fixture;

function initialGrid(): number[][] {
  const grid = Array.from({ length: rows }, () => Array<number>(columns).fill(initialHeightMillimetres));
  for (const [x, y, value] of seed) grid[y!]![x!] = value!;
  return grid;
}

function stamps(anchors: number[][], radius: number): number[][] {
  const spacing = Math.max(1, Math.ceil(radius / 2));
  return anchors.flatMap(([x, y], index) => {
    if (index === 0) return [[x!, y!]];
    const [fromX, fromY] = anchors[index - 1]!;
    const dx = x! - fromX!;
    const dy = y! - fromY!;
    const count = Math.ceil(Math.max(Math.abs(dx), Math.abs(dy)) / spacing);
    return Array.from({ length: count }, (_, step) => [
      Math.round(fromX! + dx * (step + 1) / count),
      Math.round(fromY! + dy * (step + 1) / count),
    ]);
  });
}

function project(input: number[][], command: Command): number[][] {
  const { operation, column, row, radius, value } = command;
  const amount = operation === "smooth" ? value : Math.round(value * 1000);
  return input.map((line, y) => line.map((sample, x) => {
    if (operation === "offset") return sample + amount;
    if (operation === "set-sample") return x === column && y === row ? amount : sample;
    const distanceSquared = add(square(x - column), square(y - row));
    const weight = Math.max(0, Number(subtract(1, divide(distanceSquared, square(radius + 1)))));
    const neighbors = input.slice(Math.max(0, y - 1), y + 2).flatMap(neighbor => neighbor.slice(Math.max(0, x - 1), x + 2));
    const adjustment = operation === "raise-lower" ? amount
      : operation === "flatten" ? amount - sample
      : Number(multiply(subtract(mean(neighbors), sample), divide(amount, 100)));
    const change = Number(multiply(adjustment, weight));
    return sample + Number(multiply(sign(change), floor(add(abs(change), 0.5))));
  }));
}

function apply(input: number[][], command: Command): number[][] {
  const { column, row, radius, operation, stroke } = command;
  const centers = stroke ? stamps(stroke, radius) : [[column, row]];
  if (stroke && (stroke[0]?.[0] !== column || stroke[0]?.[1] !== row || centers.length > schema.properties.stroke.maxItems)) {
    throw new Error("invalid stroke path");
  }
  if (operation !== "offset" && centers.some(([x, y]) => x! >= columns || y! >= rows)) {
    throw new Error("sample outside the terrain grid");
  }
  return centers.reduce((current, [x, y]) => {
    const next = project(current, { ...command, column: x!, row: y! });
    const { minimum, maximum } = schema.definitions.metres;
    if (next.some(line => line.some(height => height < minimum * 1000 || height > maximum * 1000))) {
      throw new Error("sample overflow");
    }
    return next;
  }, input);
}

test("terrain edits and strokes match an independent rectangular raster calculation", () => {
  const input = initialGrid();
  for (const { command, changed, samples } of [...fixture.cases, ...fixture.strokeEdits]) {
    expect(validate(command)).toBe(true);
    const result = apply(input, command);
    expect(result.flatMap((line, y) => line.filter((height, x) => height !== input[y]![x])).length).toBe(changed);
    for (const [x, y, expected] of samples) expect(result[y!]![x!]).toBe(expected);
  }
  for (const gesture of fixture.strokeGestures) {
    if (gesture.anchors !== null) expect(stamps(gesture.anchors, gesture.radius)).toEqual(gesture.stamps);
  }
});

test("terrain edits enforce the action contract, stroke paths, grid bounds and sample range", () => {
  const input = initialGrid();
  for (const command of fixture.invalidActions) expect(validate(command)).toBe(false);
  for (const command of fixture.invalidStrokePaths) {
    expect(validate(command)).toBe(true);
    expect(() => apply(input, command)).toThrow("invalid stroke path");
  }
  for (const command of fixture.refusedEdits) {
    expect(validate(command)).toBe(true);
    expect(() => apply(input, command)).toThrow();
  }
  for (const { seed: [x, y, height], command } of fixture.overflowStroke) {
    const seeded = initialGrid();
    seeded[y!]![x!] = height!;
    expect(validate(command)).toBe(true);
    expect(() => apply(seeded, command)).toThrow("sample overflow");
    expect(seeded[y!]![x!]).toBe(height!);
  }
});

test("filling a raster gap changes only missing samples inside the brush footprint", () => {
  let raster = fillFixture.source.rowsMillimetres;
  let mask = Buffer.from(fillFixture.source.missingSamples, "hex");
  const width = raster[0]!.length;
  for (const { command, expected } of fillFixture.cases) {
    expect(validate(command)).toBe(true);
    const remaining = Buffer.alloc(mask.length);
    raster = raster.map((line, y) => line.map((sample, x) => {
      const ordinal = y * width + x;
      const byte = Math.floor(ordinal / 8);
      const bit = 1 << ordinal % 8;
      if (!(mask[byte]! & bit)) return sample;
      const distance = add(square(x - command.column), square(y - command.row));
      if (Number(distance) < Number(square(command.radius + 1))) return Number(multiply(command.value, 1000));
      remaining[byte] = remaining[byte]! | bit;
      return sample;
    }));
    expect(raster).toEqual(expected.rowsMillimetres);
    expect(remaining.some(byte => byte) ? remaining.toString("hex") : undefined).toEqual(
      "missingSamples" in expected ? expected.missingSamples : undefined,
    );
    mask = remaining;
  }
  for (const command of fillFixture.invalid) expect(validate(command)).toBe(false);
  expect(validate(fillFixture.outsideSource)).toBe(true);
  expect(fillFixture.outsideSource.column).toBe(width);
});

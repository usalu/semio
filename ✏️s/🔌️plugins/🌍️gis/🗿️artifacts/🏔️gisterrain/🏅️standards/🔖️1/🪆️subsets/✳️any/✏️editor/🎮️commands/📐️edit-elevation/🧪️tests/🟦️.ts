import { expect, test } from "bun:test";
import { abs, add, divide, floor, mean, multiply, sign, square, subtract } from "mathjs";
import Ajv from "ajv";
import schema from "../🔣️.json";
import fixture from "./🔣️.json";

const validate = new Ajv().compile(schema);
type Command = (typeof fixture.cases)[number]["command"];

test("terrain sample edit vectors match an independent raster calculation", () => {
  const { size, height, seed } = fixture;
  const input = Array.from({ length: size }, () => Array<number>(size).fill(height));
  for (const [x, y, value] of seed) input[y]![x] = value;
  const project = ({ operation, column, row, radius, value }: Command): number[][] => input.map((source, y) =>
    source.map((sample, x) => {
      const distanceSquared = add(square(x - column), square(y - row));
      const weight = Math.max(0, Number(subtract(1, divide(distanceSquared, square(radius + 1)))));
      const neighbors = input.slice(Math.max(0, y - 1), y + 2).flatMap(line => line.slice(Math.max(0, x - 1), x + 2));
      if (operation === "offset") return sample + value;
      if (operation === "set-sample") return x === column && y === row ? value : sample;
      const adjustment = operation === "raise-lower" ? value
        : operation === "flatten" ? value - sample
        : Number(multiply(subtract(mean(neighbors), sample), divide(value, 100)));
      const change = Number(multiply(adjustment, weight));
      return sample + Number(multiply(sign(change), floor(add(abs(change), 0.5))));
    }));
  for (const { command, changed, samples } of fixture.cases) {
    expect(validate(command)).toBe(true);
    const result = project(command);
    expect(result.flatMap((line, y) => line.filter((height, x) => height !== input[y]![x])).length).toBe(changed);
    for (const [x, y, expected] of samples) expect(result[y]![x]).toBe(expected);
  }
  expect(project(fixture.refused[0]!).flat().some(height => height > 32767)).toBe(true);
  for (const command of fixture.refused.slice(1)) expect(validate(command)).toBe(false);
});

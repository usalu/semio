import { expect, test } from "bun:test";
import Ajv from "ajv";
import { map, size, sum, zeros } from "mathjs";
import schema from "../🔣️.json";
import fixture from "./🔣️.json";

test("flat terrain grid matches the geographic and raster oracle", () => {
  const validate = new Ajv().compile(schema);
  const { command, expected } = fixture;
  expect(validate(command)).toBe(true);
  expect(validate(fixture.invalidHeight)).toBe(false);
  expect(fixture.invalidOrder.west).toBeGreaterThan(fixture.invalidOrder.east);
  expect([command.west, command.south, command.east, command.north]).toEqual(expected.bounds);
  const raster = map(zeros([expected.rows, expected.columns]), () => command.height);
  expect(size(raster)).toEqual([33, 33]);
  expect(sum(raster)).toBe(expected.rows * expected.columns * expected.height);
});

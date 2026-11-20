import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import fixture from "./🔣️original-box-booleans.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;

test("Manifold matches the original box Boolean world bounds and solid volumes", () => {
  const box = ({ size, origin }: typeof fixture.a) =>
    Manifold.cube(size as [number, number, number]).translate(origin as [number, number, number]);
  for (const data of [fixture, fixture.orthogonal, fixture.cavity]) {
    const a = box(data.a);
    const b = box(data.b);
    const results = {
      "brep.bool.fuse": () => a.add(b),
      "brep.bool.intersect": () => a.intersect(b),
      "brep.bool.cut": () => a.subtract(b),
    };
    for (const entry of data.cases) {
      const shape = results[entry.operator as keyof typeof results]();
      const bounds = shape.boundingBox();
      expect(Array.from(bounds.min)).toEqual(entry.minimum);
      expect(Array.from(bounds.max)).toEqual(entry.maximum);
      expect(shape.volume()).toBeCloseTo(entry.volume, 10);
      shape.delete();
    }
    a.delete();
    b.delete();
  }
});


test("Manifold agrees with the retained coaxial cylinder set operations", () => {
  const fixtureCase = fixture.cylinders;
  const a = Manifold.cylinder(fixtureCase.aHeight, fixtureCase.radius, fixtureCase.radius, 512);
  const originB = Manifold.cylinder(fixtureCase.bHeight, fixtureCase.radius, fixtureCase.radius, 512);
  const b = originB.translate([0, 0, fixtureCase.bStart]);
  originB.delete();
  const results = {
    "brep.bool.fuse": () => a.add(b),
    "brep.bool.intersect": () => a.intersect(b),
    "brep.bool.cut": () => a.subtract(b),
  };
  for (const entry of fixtureCase.cases) {
    const shape = results[entry.operator as keyof typeof results]();
    expect(Math.abs(shape.volume() / entry.volume - 1)).toBeLessThan(0.00005);
    shape.delete();
  }
  a.delete();
  b.delete();
});


test("Manifold agrees with the analytic box through-bore volume", () => {
  const { stock, tool, volume } = fixture.bore;
  const originStock = Manifold.cube(stock.size as [number, number, number]);
  const originTool = Manifold.cylinder(tool.length, tool.radius, tool.radius, 1024);
  const a = originStock.translate(stock.origin as [number, number, number]);
  const b = originTool.translate(tool.origin as [number, number, number]);
  const result = a.subtract(b);
  expect(Math.abs(result.volume() - volume)).toBeLessThan(0.00005);
  expect(result.genus()).toEqual(1);
  const bounds = result.boundingBox();
  expect(Array.from(bounds.min)).toEqual(stock.origin);
  expect(Array.from(bounds.max)).toEqual(stock.origin.map((v, axis) => v + stock.size[axis]));
  for (const shape of [result, a, b, originStock, originTool]) shape.delete();
});

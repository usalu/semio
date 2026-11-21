import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import { Box3, Vector3 } from "three";
import fixture from "./🔣️original-orthogonal-chain.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vec3 = [number, number, number];
type Box = { origin: number[]; size: number[] };
type Operand = { stock: Box; void?: Box };
type Shape = ReturnType<typeof Manifold.cube>;

function box(row: Box) {
  const initial = Manifold.cube(row.size as Vec3);
  const result = initial.translate(row.origin as Vec3);
  initial.delete();
  return result;
}

function solid(row: Operand) {
  const stock = box(row.stock);
  if (!row.void) return stock;
  const tool = box(row.void);
  const body = stock.subtract(tool);
  stock.delete();
  tool.delete();
  return body;
}

function operate(a: Shape, b: Shape, op: string) {
  switch (op) {
    case "fuse": return a.add(b);
    case "intersect": return a.intersect(b);
    case "cut": return a.subtract(b);
    default: throw Error(op);
  }
}

for (const row of fixture.cases) {
  test(`Manifold chain verifies ${row.name}`, () => {
    const a = solid(row.a);
    const b = solid(row.b);
    const result = operate(a, b, row.operation);
    const expected = new Box3(new Vector3(...row.bounds[0] as Vec3), new Vector3(...row.bounds[1] as Vec3));
    expect(result.boundingBox()).toEqual({ min: expected.min.toArray(), max: expected.max.toArray() });
    expect(result.volume()).toBeCloseTo(row.volume, 8);
    const shells = result.decompose();
    expect(shells).toHaveLength(row.shells);
    expect(shells.reduce((genus, shell) => genus + shell.genus(), 0)).toBe(row.genus);
    for (const item of [...shells, result, b, a]) item.delete();
  });
}

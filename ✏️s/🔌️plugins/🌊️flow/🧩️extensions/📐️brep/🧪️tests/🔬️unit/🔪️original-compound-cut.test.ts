import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import { Box3, Vector3 } from "three";
import fixture from "./🔣️original-compound-cut.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vec3 = [number, number, number];

for (const row of fixture.cases) {
  test(`Manifold Compound Cut verifies ${row.name}`, () => {
    const target = Manifold.cube(fixture.target.size as Vec3);
    const stock = target.translate(fixture.target.origin as Vec3);
    const cubes = row.tools.map(tool => Manifold.cube(tool.size as Vec3));
    const tools = cubes.map((box, i) => box.translate(row.tools[i].origin as Vec3));
    const union = tools[0].add(tools[1]);
    const cut = stock.subtract(union);
    const bounds = cut.boundingBox();
    const expected = new Box3(new Vector3(...fixture.target.origin as Vec3), new Vector3(...fixture.target.origin as Vec3).add(new Vector3(...fixture.target.size as Vec3)));
    expect(bounds).toEqual({ min: expected.min.toArray(), max: expected.max.toArray() });
    expect(cut.volume()).toBeCloseTo(row.volume, 8);
    const shells = cut.decompose();
    expect(shells).toHaveLength(row.shells);
    expect(shells.reduce((genus, shell) => genus + shell.genus(), 0)).toBe(row.genus);
    for (const item of [...shells, cut, union, ...tools, ...cubes, stock, target]) item.delete();
  });
}

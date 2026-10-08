import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { BoxGeometry, type BufferGeometry, SphereGeometry, Vector3 } from "three";
import { disagreements, type FixtureCase, type OracleResult } from "../../../🧪️tests/🧰️oracle-support/🟦️.ts";

type Fixture = { tolerance: number; cases: FixtureCase[] };
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8")) as Fixture;
const catalogue = JSON.parse(readFileSync(new URL("../../../../../🗂️catalogue/🔣️brep-primitive.json", import.meta.url), "utf8")) as { kinds: { id: string }[] };
const prefix = "generation3d.geometry.";

/** 📐️ Signed enclosed volume and surface area of an outward-wound triangle mesh, summed per triangle with three's Vector3. */
const measure = (geometry: BufferGeometry): { volume: number; area: number } => {
  const position = geometry.getAttribute("position");
  const index = geometry.getIndex();
  const count = index ? index.count : position.count;
  const corner = (at: number): Vector3 => new Vector3().fromBufferAttribute(position, index ? index.getX(at) : at);
  let volume = 0;
  let area = 0;
  for (let at = 0; at < count; at += 3) {
    const [a, b, c] = [corner(at), corner(at + 1), corner(at + 2)];
    volume += a.dot(new Vector3().crossVectors(b, c)) / 6;
    area += new Vector3().crossVectors(new Vector3().subVectors(b, a), new Vector3().subVectors(c, a)).length() / 2;
  }
  return { volume, area };
};

const solid = (volume: number, area: number): OracleResult => ({ outputs: { shape: { kind: "solid", volume, area } } });

/** 🧊️ Recomputes one case: the box with three's BoxGeometry, the sphere from its closed forms. */
const oracle = ({ kind, inputs }: FixtureCase): OracleResult => {
  switch (kind) {
    case "brep.primitive.box": {
      const [width, depth, height] = [inputs.width, inputs.depth, inputs.height] as number[];
      if (!(width > 0 && depth > 0 && height > 0)) return { fault: { code: `${prefix}kernel` } };
      const { volume, area } = measure(new BoxGeometry(width, height, depth));
      return solid(volume, area);
    }
    case "brep.primitive.sphere": {
      const radius = inputs.radius as number;
      return radius > 0 ? solid((4 / 3) * Math.PI * radius ** 3, 4 * Math.PI * radius ** 2) : { fault: { code: `${prefix}kernel` } };
    }
    default: throw new Error(`no oracle for ${kind}`);
  }
};

test("the fixture states the seed kinds, all of them catalogue kinds of brep.primitive", () => {
  const kinds = new Set(catalogue.kinds.map(({ id }) => id));
  expect([...new Set(fixture.cases.map(({ kind }) => kind))].sort()).toEqual(["brep.primitive.box", "brep.primitive.sphere"]);
  for (const { kind } of fixture.cases) expect(kinds.has(kind)).toBe(true);
});

test("three recomputes the volume and area of every box and sphere case", () => {
  expect(disagreements(fixture.cases, oracle, fixture.tolerance)).toEqual([]);
});

test("a faceted three SphereGeometry converges on the closed forms the kernel's exact sphere states", () => {
  for (const radius of [1, 1.5]) {
    const { volume, area } = measure(new SphereGeometry(radius, 256, 128));
    expect(Math.abs(volume / ((4 / 3) * Math.PI * radius ** 3) - 1)).toBeLessThan(5e-4);
    expect(Math.abs(area / (4 * Math.PI * radius ** 2) - 1)).toBeLessThan(5e-4);
  }
});

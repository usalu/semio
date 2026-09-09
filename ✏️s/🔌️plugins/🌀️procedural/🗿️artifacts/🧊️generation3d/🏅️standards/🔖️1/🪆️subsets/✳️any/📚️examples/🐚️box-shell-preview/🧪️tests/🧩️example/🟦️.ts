import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "bun:test";
import { assertExpectedBoundingBox, assertExpectedPerimeter, assertExpectedVolume, assertFixtureContract, assertOpChainDeclared, loadExample, simpson, sliders } from "../../../🧪️tests/🧩️geometry/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const { dsl, fixture } = loadExample(here, "🐚️box-shell-preview", "box-shell-preview");
const knob = sliders(dsl);

describe("box-shell-preview", () => {
  it("ships primary asset", () => {
    expect(dsl.length).toBeGreaterThan(8);
  });

  it("commits a well-formed expected-geometry fixture", () => {
    assertFixtureContract(fixture, "box-shell-preview");
  });

  it("declares exactly the op chain its dsl wires", () => {
    assertOpChainDeclared(dsl, fixture);
  });

  it("recomputes the committed hollow-box volume from the dsl sliders", () => {
    const inner = knob.size - 2 * knob.thickness;
    assertExpectedVolume(fixture, knob.size ** 3 - inner ** 3);
    assertExpectedBoundingBox(fixture, [0, 0, 0], [knob.size, knob.size, knob.size]);
  });
});

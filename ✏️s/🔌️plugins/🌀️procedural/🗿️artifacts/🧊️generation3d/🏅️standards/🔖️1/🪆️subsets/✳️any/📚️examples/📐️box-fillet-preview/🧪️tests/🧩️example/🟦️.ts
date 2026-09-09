import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "bun:test";
import { assertExpectedBoundingBox, assertExpectedPerimeter, assertExpectedVolume, assertFixtureContract, assertOpChainDeclared, loadExample, simpson, sliders } from "../../../🧪️tests/🧩️geometry/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const { dsl, fixture } = loadExample(here, "📐️box-fillet-preview", "box-fillet-preview");
const knob = sliders(dsl);

describe("box-fillet-preview", () => {
  it("ships primary asset", () => {
    expect(dsl.length).toBeGreaterThan(8);
  });

  it("commits a well-formed expected-geometry fixture", () => {
    assertFixtureContract(fixture, "box-fillet-preview");
  });

  it("declares exactly the op chain its dsl wires", () => {
    assertOpChainDeclared(dsl, fixture);
  });

  it("recomputes the committed rounded-box volume from the dsl sliders", () => {
    const core = knob.size - 2 * knob.radius;
    const minkowski = core ** 3 + 6 * knob.radius * core ** 2 + 3 * Math.PI * knob.radius ** 2 * core + (4 / 3) * Math.PI * knob.radius ** 3;
    assertExpectedVolume(fixture, minkowski);
    assertExpectedBoundingBox(fixture, [0, 0, 0], [knob.size, knob.size, knob.size]);
  });
});

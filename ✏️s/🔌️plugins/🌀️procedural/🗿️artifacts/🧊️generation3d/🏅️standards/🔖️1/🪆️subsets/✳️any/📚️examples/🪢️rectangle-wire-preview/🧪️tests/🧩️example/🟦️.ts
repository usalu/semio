import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "bun:test";
import { assertExpectedBoundingBox, assertExpectedPerimeter, assertExpectedVolume, assertFixtureContract, assertOpChainDeclared, loadExample, simpson, sliders } from "../../../🧪️tests/🧩️geometry/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const { dsl, fixture } = loadExample(here, "🪢️rectangle-wire-preview", "rectangle-wire-preview");
const knob = sliders(dsl);

describe("rectangle-wire-preview", () => {
  it("ships primary asset", () => {
    expect(dsl.length).toBeGreaterThan(8);
  });

  it("commits a well-formed expected-geometry fixture", () => {
    assertFixtureContract(fixture, "rectangle-wire-preview");
  });

  it("declares exactly the op chain its dsl wires", () => {
    assertOpChainDeclared(dsl, fixture);
  });

  it("recomputes the committed wire perimeter and extent from the dsl sliders", () => {
    assertExpectedPerimeter(fixture, 2 * (knob.width + knob.height));
    assertExpectedBoundingBox(fixture, [0, 0, 0], [knob.width, knob.height, 0]);
    expect(fixture.expect.closed).toBe(false);
    expect(fixture.expect.volume).toBeNull();
  });
});

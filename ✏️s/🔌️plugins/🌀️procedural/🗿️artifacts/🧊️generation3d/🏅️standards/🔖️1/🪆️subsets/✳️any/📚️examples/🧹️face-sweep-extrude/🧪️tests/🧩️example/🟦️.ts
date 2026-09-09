import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "bun:test";
import { assertExpectedBoundingBox, assertExpectedPerimeter, assertExpectedVolume, assertFixtureContract, assertOpChainDeclared, loadExample, simpson, sliders } from "../../../🧪️tests/🧩️geometry/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const { dsl, fixture } = loadExample(here, "🧹️face-sweep-extrude", "face-sweep-extrude");
const knob = sliders(dsl);

describe("face-sweep-extrude", () => {
  it("ships primary asset", () => {
    expect(dsl.length).toBeGreaterThan(8);
  });

  it("commits a well-formed expected-geometry fixture", () => {
    assertFixtureContract(fixture, "face-sweep-extrude");
  });

  it("declares exactly the op chain its dsl wires", () => {
    assertOpChainDeclared(dsl, fixture);
  });

  it("recomputes the committed prism volume and extent from the dsl sliders", () => {
    assertExpectedVolume(fixture, knob.width * knob.height * knob.distance);
    assertExpectedBoundingBox(fixture, [0, 0, 0], [knob.width, knob.height, knob.distance]);
  });
});

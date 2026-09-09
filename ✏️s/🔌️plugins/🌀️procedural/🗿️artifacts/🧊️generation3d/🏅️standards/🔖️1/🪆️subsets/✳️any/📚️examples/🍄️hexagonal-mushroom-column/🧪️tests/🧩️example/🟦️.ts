import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "bun:test";
import { assertExpectedBoundingBox, assertExpectedPerimeter, assertExpectedVolume, assertFixtureContract, assertOpChainDeclared, loadExample, simpson, sliders } from "../../../🧪️tests/🧩️geometry/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const { dsl, fixture } = loadExample(here, "🍄️hexagonal-mushroom-column", "hexagonal-mushroom-column");
const knob = sliders(dsl);

describe("hexagonal-mushroom-column", () => {
  it("ships primary asset", () => {
    expect(dsl.length).toBeGreaterThan(8);
  });

  it("commits a well-formed expected-geometry fixture", () => {
    assertFixtureContract(fixture, "hexagonal-mushroom-column");
  });

  it("declares exactly the op chain its dsl wires", () => {
    assertOpChainDeclared(dsl, fixture);
  });

  it("recomputes the committed regular-polygon prism volume from the dsl sliders", () => {
    const area = (knob.sides / 2) * knob.radius * knob.radius * Math.sin((2 * Math.PI) / knob.sides);
    assertExpectedVolume(fixture, area * knob.height);
    expect(fixture.expect.boundingBoxMax[2] - fixture.expect.boundingBoxMin[2]).toBeCloseTo(knob.height, 6);
    const inradius = knob.radius * Math.cos(Math.PI / knob.sides);
    for (const axis of [0, 1]) {
      const extent = (fixture.expect.boundingBoxMax[axis] - fixture.expect.boundingBoxMin[axis]) / 2;
      expect(extent).toBeGreaterThanOrEqual(inradius - fixture.expect.boundingBoxTolerance);
      expect(extent).toBeLessThanOrEqual(knob.radius + fixture.expect.boundingBoxTolerance);
    }
  });
});

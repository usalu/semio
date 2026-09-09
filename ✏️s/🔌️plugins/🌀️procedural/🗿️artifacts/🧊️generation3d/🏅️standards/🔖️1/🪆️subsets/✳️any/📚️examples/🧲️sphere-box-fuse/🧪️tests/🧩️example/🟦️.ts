import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "bun:test";
import { assertExpectedBoundingBox, assertExpectedPerimeter, assertExpectedVolume, assertFixtureContract, assertOpChainDeclared, loadExample, simpson, sliders } from "../../../🧪️tests/🧩️geometry/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const { dsl, fixture } = loadExample(here, "🧲️sphere-box-fuse", "sphere-box-fuse");
const knob = sliders(dsl);

describe("sphere-box-fuse", () => {
  it("ships primary asset", () => {
    expect(dsl.length).toBeGreaterThan(8);
  });

  it("commits a well-formed expected-geometry fixture", () => {
    assertFixtureContract(fixture, "sphere-box-fuse");
  });

  it("declares exactly the op chain its dsl wires", () => {
    assertOpChainDeclared(dsl, fixture);
  });

  it("recomputes the committed union volume by its own quadrature", () => {
    const ball = knob.radius;
    const side = knob.size;
    // 🧲️ `brep.prim3d.box` spans `[0, side]³` from its own CORNER, and the ball is centred on that
    // corner — so the overlap is the ball's POSITIVE OCTANT clipped to the box, not the concentric
    // intersection the committed extent (`[-ball, side]`, which this same test asserts) never
    // described. Each `z` slice contributes a quarter disc of radius `√(ball² − z²)` clipped to the
    // square `[0, side]²`.
    const clipped = (z: number) => {
      const squared = ball * ball - z * z;
      if (squared <= 0) return 0;
      const radius = Math.sqrt(squared);
      if (radius <= side) return (Math.PI * squared) / 4;
      if (radius >= side * Math.SQRT2) return side * side;
      const chord = Math.sqrt(squared - side * side);
      return side * chord + (squared / 2) * (Math.asin(side / radius) - Math.asin(chord / radius));
    };
    const overlap = simpson(clipped, 0, Math.min(ball, side), 8000);
    assertExpectedVolume(fixture, (4 / 3) * Math.PI * ball ** 3 + side ** 3 - overlap);
    assertExpectedBoundingBox(fixture, [-ball, -ball, -ball], [Math.max(ball, side), Math.max(ball, side), Math.max(ball, side)]);
    expect(fixture.kernelStatus).toBe("green");
  });
});

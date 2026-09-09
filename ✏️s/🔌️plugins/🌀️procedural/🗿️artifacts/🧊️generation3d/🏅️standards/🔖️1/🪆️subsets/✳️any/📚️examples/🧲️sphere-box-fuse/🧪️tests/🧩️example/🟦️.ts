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
    const half = knob.size / 2;
    const clipped = (z: number) => {
      const squared = ball * ball - z * z;
      if (squared <= 0) return 0;
      const r = Math.sqrt(squared);
      if (r <= half) return Math.PI * r * r;
      if (r >= half * Math.SQRT2) return 4 * half * half;
      const chord = Math.sqrt(r * r - half * half);
      return 4 * (half * chord + r * r * (Math.PI / 4 - Math.atan(chord / half)));
    };
    const overlap = simpson(clipped, -Math.min(ball, half), Math.min(ball, half), 8000);
    assertExpectedVolume(fixture, (4 / 3) * Math.PI * ball ** 3 + knob.size ** 3 - overlap);
    assertExpectedBoundingBox(fixture, [-ball, -ball, -ball], [Math.max(ball, knob.size), Math.max(ball, knob.size), Math.max(ball, knob.size)]);
    expect(fixture.kernelStatus).toBe("blocked-on-boolean-kernel");
  });
});

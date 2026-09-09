import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "bun:test";
import { assertExpectedBoundingBox, assertExpectedPerimeter, assertExpectedVolume, assertFixtureContract, assertOpChainDeclared, loadExample, simpson, sliders } from "../../../🧪️tests/🧩️geometry/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const { dsl, fixture } = loadExample(here, "🍩️sphere-cut-with-torus", "sphere-cut-with-torus");
const knob = sliders(dsl);

describe("sphere-cut-with-torus", () => {
  it("ships primary asset", () => {
    expect(dsl.length).toBeGreaterThan(8);
  });

  it("commits a well-formed expected-geometry fixture", () => {
    assertFixtureContract(fixture, "sphere-cut-with-torus");
  });

  it("declares exactly the op chain its dsl wires", () => {
    assertOpChainDeclared(dsl, fixture);
  });

  it("recomputes the committed difference volume by its own quadrature", () => {
    const ball = knob.slider_2;
    const major = 2.0;
    const minor = 0.5;
    const shell = (angle: number) => {
      const cosine = Math.cos(angle);
      const discriminant = major * major * cosine * cosine - major * major + ball * ball;
      if (discriminant <= 0) return 0;
      const reach = Math.min(minor, Math.max(0, -major * cosine + Math.sqrt(discriminant)));
      return 2 * Math.PI * ((major * reach * reach) / 2 + (cosine * reach ** 3) / 3);
    };
    const inside = simpson(shell, 0, 2 * Math.PI, 16000);
    assertExpectedVolume(fixture, (4 / 3) * Math.PI * ball ** 3 - inside);
    // 📦️ The tube reaches radius `major + minor = 2.5 > ball`, so the torus grooves the ball's own
    // equator away: the difference's widest surviving circle sits where the ball's surface is
    // exactly `minor` from the tube's centre circle, `sin θ = (ball² + major² − minor²) / (2·major·ball)`,
    // i.e. a cylindrical radius of `(ball² + major² − minor²) / (2·major)`. Only the poles still
    // reach `±ball`.
    const reachRadius = (ball * ball + major * major - minor * minor) / (2 * major);
    assertExpectedBoundingBox(fixture, [-reachRadius, -reachRadius, -ball], [reachRadius, reachRadius, ball]);
    expect(fixture.expect.kernelVolumeNode).toBe("brep_measure_volume_2");
    expect(fixture.expect.kernelVolumeTolerance).toBeLessThanOrEqual(1e-5);
    expect(fixture.kernelStatus).toBe("green");
  });
});

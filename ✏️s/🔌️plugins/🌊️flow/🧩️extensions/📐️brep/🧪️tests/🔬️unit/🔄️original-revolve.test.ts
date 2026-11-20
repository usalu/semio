import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import fixture from "./🔣️original-revolve.json";

const wasm = await loadManifold();
wasm.setup();
const { CrossSection } = wasm;

test("Manifold sweeps the circular profile to the native torus bounds and volume", () => {
  const { axis, profile, expected } = fixture;
  const axialCenter = [axis.origin[0], profile.center[1], axis.origin[2]] as [number, number, number];
  const radialCenter = Math.hypot(profile.center[0] - axialCenter[0], profile.center[2] - axialCenter[2]);
  const circle = CrossSection.circle(profile.radius, 256);
  const radialProfile = circle.translate([radialCenter, 0]);
  const canonical = radialProfile.revolve(384, fixture.angle * 180 / Math.PI);
  const oriented = canonical.rotate([-90, 0, 0]);
  const shape = oriented.translate(axialCenter);
  expect(shape.genus()).toEqual(1);
  expect(Math.abs(shape.volume() / expected.volume - 1)).toBeLessThan(0.00025);
  const bounds = shape.boundingBox();
  for (let i = 0; i < 3; i++) {
    expect(bounds.min[i]).toBeCloseTo(expected.minimum[i], 5);
    expect(bounds.max[i]).toBeCloseTo(expected.maximum[i], 5);
  }
  for (const item of [shape, oriented, canonical, radialProfile, circle]) item.delete();
});

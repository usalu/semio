import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import fixture from "./🔣️original-placed-solid-transforms.json";

const wasm = await loadManifold();
wasm.setup();
const { CrossSection, Manifold } = wasm;
type Vec3 = [number, number, number];
type Solid = ReturnType<typeof Manifold.cube>;

for (const row of fixture.cases) {
  test(`Manifold preserves the certified ${row.kind} through world transforms`, () => {
    const size = row.inputs;
    let shape: Solid;
    switch (row.kind) {
      case "sphere": shape = Manifold.sphere(size.radius!, 256); break;
      case "cylinder": shape = Manifold.cylinder(size.height!, size.radius!, size.radius!, 512); break;
      case "cone": shape = Manifold.cylinder(size.height!, size.radius!, 0, 512); break;
      case "torus": {
        const disk = CrossSection.circle(size.minor!, 128);
        const profile = disk.translate([size.major!, 0]);
        shape = profile.revolve(256);
        profile.delete();
        disk.delete();
        break;
      }
      default: shape = Manifold.cube([size.width!, size.depth!, size.height!]);
    }
    for (const { operator, inputs } of row.transforms) {
      const origin = (inputs.origin ?? [0, 0, 0]) as Vec3;
      const previous = shape;
      if (operator.endsWith("translate")) {
        shape = previous.translate(inputs.offset as Vec3);
      } else {
        const local = previous.translate(origin.map(value => -value) as Vec3);
        const placed = operator.endsWith("mirror")
          ? local.mirror(inputs.normal as Vec3)
          : local.rotate(inputs.axis!.map(value => value * inputs.angle! * 180 / Math.PI) as Vec3);
        shape = placed.translate(origin);
        placed.delete();
        local.delete();
      }
      previous.delete();
    }
    const bounds = shape.boundingBox();
    for (let i = 0; i < 3; i++) {
      expect(bounds.min[i]).toBeCloseTo(row.bounds[0][i], 5);
      expect(bounds.max[i]).toBeCloseTo(row.bounds[1][i], 5);
    }
    expect(shape.genus()).toBe(row.genus);
    expect(Math.abs(shape.volume() / row.volume - 1)).toBeLessThan(0.001);
    shape.delete();
  });
}

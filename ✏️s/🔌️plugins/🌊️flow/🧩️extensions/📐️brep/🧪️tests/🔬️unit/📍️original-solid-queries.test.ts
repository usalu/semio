import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import { Triangle, Vector3 } from "three";
import fixture from "./🔣️original-solid-queries.json";
import bounds from "./🔣️original-bounding-box.json";
import booleans from "./🔣️original-box-booleans.json";
import revolve from "./🔣️original-revolve.json";

const wasm = await loadManifold();
wasm.setup();
const { CrossSection, Manifold } = wasm;
type Vec3 = [number, number, number];

function translatedBox(size: number[], origin: number[]) {
  const canonical = Manifold.cube(size as Vec3);
  const placed = canonical.translate(origin as Vec3);
  canonical.delete();
  return placed;
}

const triangle = new Triangle();
const query = new Vector3();
const nearest = new Vector3();

test("Manifold meshes agree with primitive and bore point classifications", () => {
  const { cylinder: cylinderData } = bounds;
  const tube = Manifold.cylinder(Math.hypot(...cylinderData.vector), cylinderData.radius, cylinderData.radius, 256);
  const tilted = tube.rotate([-Math.atan2(cylinderData.normal[1], cylinderData.normal[2]) * 180 / Math.PI, 0, 0]);
  const circle = CrossSection.circle(revolve.expected.minorRadius, 64);
  const radial = circle.translate([revolve.expected.majorRadius, 0]);
  const toroidal = radial.revolve(128);
  const turned = toroidal.rotate([-90, 0, 0]);
  const { stock, tool } = booleans.bore;
  const outer = translatedBox(stock.size, stock.origin);
  const cutter = Manifold.cylinder(tool.length, tool.radius, tool.radius, 256);
  const drill = cutter.translate(tool.origin as Vec3);
  const shapes = {
    box: translatedBox(fixture.box.size, fixture.box.origin),
    sphere: Manifold.sphere(fixture.primitives.sphere.radius, 64),
    cone: Manifold.cylinder(fixture.primitives.cone.height, fixture.primitives.cone.radius, 0, 128),
    cylinder: tilted.translate(cylinderData.center as Vec3),
    torus: turned.translate(revolve.expected.center as Vec3),
    bore: outer.subtract(drill),
  };
  for (const [name, shape] of Object.entries(shapes)) {
    const mesh = shape.getMesh();
    const copy = (dest: Vector3, index: number) => {
      const start = mesh.triVerts[index] * mesh.numProp;
      dest.set(mesh.vertProperties[start], mesh.vertProperties[start + 1], mesh.vertProperties[start + 2]);
    };
    for (const [point, expected] of fixture.cases[name as keyof typeof shapes]) {
      query.fromArray(point as number[]);
      let onBoundary = false;
      for (let i = 0; i < mesh.triVerts.length; i += 3) {
        copy(triangle.a, i);
        copy(triangle.b, i + 1);
        copy(triangle.c, i + 2);
        if (triangle.closestPointToPoint(query, nearest).distanceTo(query) <= fixture.tolerance) {
          onBoundary = true;
          break;
        }
      }
      const actual = onBoundary ? 2 : shape.rayCast(point as Vec3, [18.037, -18.712, 20.322]).length % 2 === 1 ? 0 : 1;
      expect(actual, `${name} ${point}`).toBe(expected as number);
    }
    shape.delete();
  }
  for (const item of [drill, cutter, outer, turned, toroidal, radial, circle, tilted, tube]) item.delete();
});

test("Manifold confirms Euclidean separation and zero overlap distance", () => {
  const source = translatedBox(fixture.box.size, fixture.box.origin);
  for (const row of fixture.boxDistance) {
    const target = translatedBox(row.size, row.origin);
    const intersection = source.intersect(target);
    const distance = intersection.isEmpty() ? source.minGap(target, 100) : 0;
    expect(distance).toBeCloseTo(row.expected, 12);
    intersection.delete();
    target.delete();
  }
  source.delete();
});

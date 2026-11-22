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

function expectClassifications(shape: ReturnType<typeof Manifold.cube>, name: string, probes: (number | number[])[][]) {
  const mesh = shape.getMesh();
  const copy = (dest: Vector3, index: number) => {
    const start = mesh.triVerts[index] * mesh.numProp;
    dest.set(mesh.vertProperties[start], mesh.vertProperties[start + 1], mesh.vertProperties[start + 2]);
  };
  for (const [point, expected] of probes) {
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
}

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
    expectClassifications(shape, name, fixture.cases[name as keyof typeof shapes]);
    shape.delete();
  }
  for (const item of [drill, cutter, outer, turned, toroidal, radial, circle, tilted, tube]) item.delete();
});

test("Manifold classifies the closed material of orthogonal Boolean results", () => {
  for (const row of fixture.orthogonalClassify) {
    const { a, b } = booleans[row.source as "orthogonal" | "cavity"];
    const stock = translatedBox(a.size, a.origin);
    const tool = translatedBox(b.size, b.origin);
    const result = row.operator === "brep.bool.fuse" ? stock.add(tool) : stock.subtract(tool);
    try {
      if (row.clip) {
        const clip = translatedBox(row.clip.size, row.clip.origin);
        const clipped = result.intersect(clip);
        try {
          expectClassifications(clipped, row.name, row.points);
        } finally {
          clipped.delete();
          clip.delete();
        }
      } else {
        expectClassifications(result, row.name, row.points);
      }
    } finally {
      result.delete();
      tool.delete();
      stock.delete();
    }
  }
});

type CurvedOperand = { kind: string; origin: number[]; size?: number[]; radius?: number; height?: number };
type CurvedBoolean = { name: string; operator: string; a: CurvedOperand; b: CurvedOperand; points: (number | number[])[][] };

function expectCurvedClassifications(rows: CurvedBoolean[]) {
  const build = (spec: CurvedOperand) => {
    if (spec.kind === "box") return translatedBox(spec.size!, spec.origin);
    const canonical = spec.kind === "sphere"
      ? Manifold.sphere(spec.radius!, 128)
      : Manifold.cylinder(spec.height!, spec.radius!, spec.radius!, 256);
    const placed = canonical.translate(spec.origin as Vec3);
    canonical.delete();
    return placed;
  };
  for (const row of rows) {
    const a = build(row.a);
    const b = build(row.b);
    const result = row.operator === "brep.bool.intersect" ? a.intersect(b)
      : row.operator === "brep.bool.fuse" ? a.add(b) : a.subtract(b);
    try {
      expectClassifications(result, row.name, row.points);
    } finally {
      result.delete();
      b.delete();
      a.delete();
    }
  }
}

test("Manifold agrees on the boundaries and material of spherical Boolean results", () => {
  expectCurvedClassifications(fixture.sphericalClassify);
});

test("Manifold agrees on the boundaries and material of cylindrical Boolean results", () => {
  expectCurvedClassifications(fixture.cylindricalClassify);
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


test("Manifold and Three.js project onto orthogonal Boolean material and measure spherical clearance", () => {
  for (const row of fixture.orthogonalSphereDistance) {
    const shell = row.stock;
    const shape = shell
      ? { a: shell, b: {
        origin: shell.origin.map(value => value + shell.thickness),
        size: shell.size.map(value => value - 2 * shell.thickness),
      } }
      : booleans[row.source as "orthogonal" | "cavity"];
    const stock = translatedBox(shape.a.size, shape.a.origin);
    const tool = translatedBox(shape.b.size, shape.b.origin);
    const solid = row.operator === "brep.bool.fuse" ? stock.add(tool) : stock.subtract(tool);
    const axis = new Vector3(0, 0, 1);
    const offset = new Vector3().fromArray(row.placement?.offset ?? [0, 0, 0]);
    const angle = row.placement?.angle ?? 0;
    const rotated = solid.rotate([0, 0, angle * 180 / Math.PI]);
    const placed = rotated.translate(offset.toArray() as Vec3);
    try {
      const mesh = placed.getMesh();
      const vertex = (out: Vector3, corner: number) => {
        const start = mesh.triVerts[corner] * mesh.numProp;
        out.fromArray(mesh.vertProperties, start);
      };
      const winner = new Vector3();
      const world = (coords: number[]) => new Vector3().fromArray(coords).applyAxisAngle(axis, angle).add(offset);
      const project = (coords: number[]) => {
        query.copy(world(coords));
        let gap = Infinity;
        for (let corner = 0; corner < mesh.triVerts.length; corner += 3) {
          vertex(triangle.a, corner);
          vertex(triangle.b, corner + 1);
          vertex(triangle.c, corner + 2);
          const distance = triangle.closestPointToPoint(query, nearest).distanceTo(query);
          if (distance < gap) {
            gap = distance;
            winner.copy(nearest);
          }
        }
        return gap;
      };
      for (const probe of row.probes) {
        const gap = project(probe.center);
        const rayEnd = query.clone().add(new Vector3(21.374, 17.53, 24.184));
        const inside = placed.rayCast(query.toArray() as Vec3, rayEnd.toArray() as Vec3).length % 2 === 1;
        expect(inside ? 0 : Math.max(0, gap - probe.radius), row.name).toBeCloseTo(probe.expected, 6);
      }
      for (const [target, expected] of row.closestPoints) {
        project(target);
        expect(winner.distanceTo(world(expected)), row.name).toBeLessThan(1e-6);
      }
    } finally {
      for (const value of [placed, rotated, solid, tool, stock]) value.delete();
    }
  }
});

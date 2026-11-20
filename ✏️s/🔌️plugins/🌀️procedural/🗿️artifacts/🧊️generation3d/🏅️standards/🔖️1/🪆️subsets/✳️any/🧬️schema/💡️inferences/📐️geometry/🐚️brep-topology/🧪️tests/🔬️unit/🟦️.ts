import { expect, test } from "bun:test";
import { BoxGeometry, Triangle, Vector3 } from "three";
import { agree, casesOf, fixtureDisagreements, loadFixture, measure } from "../../../⏱️phased-job/🧪️tests/🧰️support/🟦️.ts";

const fixture = loadFixture(import.meta.url);
type Counted = { count: number };

test("three agrees with the fixture on volume, area, bounds and closedness of every pinned topology result", () => {
  expect(fixtureDisagreements(fixture)).toEqual([]);
});

test("the decomposition of a box satisfies Euler's formula V - E + F = 2", () => {
  const [box] = casesOf(fixture, "brep.topology.deconstruct");
  const outputs = box!.outputs as Record<string, Counted>;
  expect(outputs.vertices!.count - outputs.edges!.count + outputs.faces!.count).toBe(2);
  expect(outputs.shells!.count).toBe(1);
});

test("the faces of a box sum to its area and the sewn solid has the box volume", () => {
  const [sew] = casesOf(fixture, "brep.topology.sew");
  const shape = sew!.outputs!.shape as { volume: number; area: number; faces: number; bbox: [number[], number[]] };
  const faces = sew!.inputs.faces as { of: { size: [number, number, number] }; normal: [number, number, number] }[];
  const [width, height, depth] = faces[0]!.of.size;
  const box = new BoxGeometry(width, height, depth).translate(width / 2, height / 2, depth / 2);
  const order = box.getIndex()!.array;
  const positions = Array.from(box.getAttribute("position").array);
  const normals = box.getAttribute("normal");
  const selectFace = (normal: [number, number, number]) => Array.from(order).filter((index) =>
    normals.getX(index) === normal[0] && normals.getY(index) === normal[1] && normals.getZ(index) === normal[2]);
  expect(faces).toHaveLength(shape.faces);
  expect(faces.map(face => face.of.size)).toEqual(Array(shape.faces).fill([width, height, depth]));
  const indices = faces.flatMap(face => selectFace(face.normal));
  const mesh = measure({ deflection: 0, positions, indices });
  expect(agree(mesh.volume, shape.volume, fixture.tolerance)).toBe(true);
  expect(agree(mesh.area, shape.area, fixture.tolerance)).toBe(true);
  expect([mesh.min, mesh.max]).toEqual(shape.bbox);
  expect(mesh.boundaryEdges + mesh.nonManifoldEdges).toBe(0);
});

test("a compound and its explosion agree on the solid count and volume", () => {
  const compound = casesOf(fixture, "brep.topology.compound")[0]!.outputs!.shape as { solids: number; volume: number };
  const explode = casesOf(fixture, "brep.topology.explode")[0]!.outputs!.solids as { count: number; each: { volume: number } };
  expect(explode.count).toBe(compound.solids);
  expect(explode.count * explode.each.volume).toBe(compound.volume);
});

test("every fault case names a code under the geometry prefix", () => {
  for (const fixtureCase of fixture.cases.filter(item => item.fault)) expect(fixtureCase.fault!.code.startsWith("generation3d.geometry."), fixtureCase.name).toBe(true);
});

test("Three verifies the retained Sew fixture has exact opposed halfedge incidence", () => {
  type Point = [number, number, number];
  const { rings, bounds, area, volume } = (fixture as typeof fixture & {
    retainedCuboidSew: { rings: Point[][]; bounds: [Point, Point]; area: number; volume: number };
  }).retainedCuboidSew;
  const vertices = new Set<string>();
  const incidence = new Map<string, number[]>();
  let ringArea = 0;
  for (const ring of rings) {
    const [a, b, c, d] = ring.map(point => new Vector3(...point)) as [Vector3, Vector3, Vector3, Vector3];
    ringArea += new Triangle(a, b, c).getArea() + new Triangle(a, c, d).getArea();
    for (let i = 0; i < 4; i++) {
      const start = JSON.stringify(ring[i]), end = JSON.stringify(ring[(i + 1) % 4]);
      vertices.add(start);
      const edge = [start, end].sort().join("/");
      const uses = incidence.get(edge) ?? [];
      uses.push(start < end ? 1 : -1);
      incidence.set(edge, uses);
    }
  }
  const size = new Vector3(...bounds[1]).sub(new Vector3(...bounds[0]));
  const center = new Vector3(...bounds[0]).addScaledVector(size, 0.5);
  const box = new BoxGeometry(...size.toArray()).translate(...center.toArray());
  const reference = measure({
    deflection: fixture.tolerance,
    positions: Array.from(box.getAttribute("position").array),
    indices: Array.from(box.getIndex()!.array),
  });
  expect(ringArea).toBe(area);
  expect(reference.area).toBe(area);
  expect(Math.abs(reference.volume - volume)).toBeLessThan(fixture.tolerance);
  expect(vertices.size).toBe(8);
  expect(incidence.size).toBe(12);
  for (const uses of incidence.values()) expect(uses.sort()).toEqual([-1,1]);
});

import { expect, test } from "bun:test";
import { Box3, Line3, Vector3 } from "three";

const fixture = await Bun.file("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️retained-heal.json").json();

test("the selected corner falls within tolerance of every incident curve endpoint", () => {
  const corner = new Vector3(...fixture.original);
  const damaged = corner.clone().add(new Vector3(...fixture.shift));
  const box = new Box3(new Vector3(), new Vector3(...fixture.size));
  expect(box.min.toArray()).toEqual(fixture.original);
  expect(damaged.distanceTo(corner)).toBeLessThan(fixture.tolerance);
  expect(damaged.distanceTo(corner)).toBeGreaterThan(fixture.refusalTolerance);
  for (const axis of [0, 1, 2]) {
    const opposite = corner.clone().setComponent(axis, box.max.getComponent(axis));
    const edge = new Line3(corner, opposite);
    expect(edge.at(0, new Vector3()).toArray()).toEqual(fixture.original);
    expect(edge.distance()).toBeGreaterThan(fixture.tolerance);
  }
});

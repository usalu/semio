import { expect, test } from "bun:test";
import { Box3, Vector3 } from "three";

const fixture = await Bun.file("🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🧵️sew/🧪️tests/🧫️fixtures/🔣️scope.json").json();
const points = (dimensions: number[]) => [0, 1].flatMap(x => [0, 1].flatMap(y => [0, 1].map(z =>
  new Vector3(dimensions[0] * x, dimensions[1] * y, dimensions[2] * z))));
const nearestPositive = (a: Vector3[], b: Vector3[]) => Math.min(...a.flatMap(p => b.map(q => p.distanceTo(q))).filter(distance => distance > 0));

test("the healing scope fixture distinguishes nearby solids from a collapsing selected edge", () => {
  const selected = points(fixture.selectedSize);
  const other = points(fixture.otherSize);
  expect(new Box3().setFromPoints(selected).getSize(new Vector3()).toArray()).toEqual(fixture.selectedSize);
  expect(new Box3().setFromPoints(other).getSize(new Vector3()).toArray()).toEqual(fixture.otherSize);
  expect(nearestPositive(selected, other)).toBeLessThan(fixture.healingTolerance);
  expect(nearestPositive(selected, selected)).toBeGreaterThan(fixture.healingTolerance);
  expect(nearestPositive(selected, selected)).toBeLessThan(fixture.collapseTolerance);
});

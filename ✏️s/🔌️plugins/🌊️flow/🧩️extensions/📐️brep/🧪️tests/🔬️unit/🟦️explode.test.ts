import { expect, test } from "bun:test";
import { Box3, BoxGeometry, Vector3 } from "three";
import fixture from "./🔣️explode.json";

test("compound order preserves repeated original solid identity", () => {
  const operands = fixture.boxes.map(([x, y, z]) => new BoxGeometry(x, y, z));
  const members = fixture.members.map(index => operands[index]!);
  expect(members[0]).toBe(members[2]);
  expect(members[0]).not.toBe(members[1]);
  const volumes = members.map(geometry => {
    geometry.computeBoundingBox();
    const extent = (geometry.boundingBox as Box3).getSize(new Vector3());
    return extent.x * extent.y * extent.z;
  });
  expect(volumes).toEqual(fixture.expectedVolumes);
  for (const operand of operands) operand.dispose();
});

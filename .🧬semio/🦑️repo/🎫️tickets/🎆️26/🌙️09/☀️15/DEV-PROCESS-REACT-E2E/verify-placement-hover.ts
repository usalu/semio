import assert from "node:assert/strict";
import { Box3, BoxGeometry, CylinderGeometry, Matrix4, Vector3 } from "three";

const fixture = await Bun.file("✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧫️fixtures/workpiece-placement.json").json();
for (const row of fixture.cases) {
  const [width, depth, height] = row.dimensions;
  const geometry = row.shape === "box"
    ? new BoxGeometry(width, depth, height)
    : new CylinderGeometry(width / 2, width / 2, height, 12).rotateX(Math.PI / 2);
  geometry.computeBoundingBox();
  const positions = new Box3().copy(geometry.boundingBox!).applyMatrix4(new Matrix4().makeTranslation(row.position[0], row.position[1], row.position[2]));
  const center = positions.getCenter(new Vector3()).toArray();
  const size = positions.getSize(new Vector3()).toArray();
  for (let axis = 0; axis < 3; axis += 1) {
    assert.ok(Math.abs(center[axis]! - row.position[axis]) < 1e-7, row.utility);
    assert.ok(Math.abs(size[axis]! - row.dimensions[axis]) < 1e-7, row.utility);
  }
  geometry.dispose();
}
console.log("[DEBUG] Three.js verified the three authored Workpiece preview extents and centers.");

/** 🧪️ Three.js geometry and JSON Patch independently validate the shared sparse transform vector. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { applyPatch } from "fast-json-patch";
import { Matrix4, Vector3 } from "three";

/** 🧭️ Confirms the authored transform and peer edits through independent reference libraries. */
export function testFem3dTransformIntentOracle(): void {
  const vector = JSON.parse(readFileSync(new URL("../../../../../🧫️fixtures/🧭️intent/🔣️.json", import.meta.url), "utf8"));
  const transform = new Matrix4().makeTranslation(vector.move.dx, vector.move.dy, vector.move.dz);
  const node = vector.base.nodes[0];
  const [x, y, z] = new Vector3(node.x, node.y, node.z).applyMatrix4(transform).toArray();
  assert.deepEqual({ ...node, x, y, z }, { ...vector.combinedNode, y: node.y });
  const outline = vector.base.solids[0].outline.map(([x, y]: number[]) => new Vector3(x, y, vector.base.solids[0].baseZ).applyMatrix4(transform).toArray().slice(0, 2));
  assert.deepEqual(outline, vector.solidPatch.outline);
  assert.deepEqual(vector.nodePatch, { x });
  const patches = [
    { op: "replace", path: "/nodes/0/x", value: x },
    { op: "replace", path: "/solids/0/outline", value: outline },
    { op: "replace", path: "/nodes/0/y", value: 7 },
    { op: "replace", path: "/solids/0/name", value: "Peer Slab" },
    { op: "replace", path: "/solids/0/height", value: 5 },
    { op: "replace", path: "/solids/0/layers", value: 2 },
    { op: "replace", path: "/solids/0/meshSize", value: 0.5 },
  ] as const;
  const combined = applyPatch(structuredClone(vector.base), [...patches], true).newDocument;
  assert.deepEqual(combined.nodes[0], vector.combinedNode);
  assert.deepEqual(combined.solids[0], vector.combinedSolid);
  const undone = applyPatch(structuredClone(combined), [
    { op: "replace", path: "/nodes/0/x", value: vector.inverse[1].patch.x },
    { op: "replace", path: "/solids/0/outline", value: vector.inverse[0].patch.outline },
  ], true).newDocument;
  assert.deepEqual(undone.nodes[0], vector.undoneNode);
  assert.deepEqual(undone.solids[0], vector.undoneSolid);
  assert.equal(new Vector3(vector.overflowNode.x, 2, 3).applyMatrix4(new Matrix4().makeTranslation(vector.overflowOffset, 0, 0)).x, Infinity);
  for (const motion of vector.motions) {
    const reference = new Vector3(node.x, node.y, node.z).multiply(new Vector3(motion.sx, 1, 1)).applyAxisAngle(new Vector3(motion.axisX, 0, motion.axisZ).normalize(), motion.angle).toArray();
    reference.forEach((coordinate, index) => assert.ok(Math.abs(coordinate - motion.position[index]) < 1e-12, `${motion.name} coordinate ${index}`));
  }
  console.log("[DEBUG] FEM3d transform intent vector agrees with Three.js and JSON Patch");
}

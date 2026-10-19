/** 🧮️ Three.js independently produces the language-neutral wide selection centroid. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { Vector3 } from "three";

export function testFem3dSelectionProjectionOracle(): void {
  const vector = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎯️projection/🔣️.json", import.meta.url), "utf8"));
  const nodes = Array.from({ length: vector.nodeCount }, (_, index) => ({ id: `node:${index}`, point: new Vector3(index, 0, 0) }));
  const selected = nodes.toReversed().flatMap(node => Array.from({ length: vector.selectionMultiplicity }, () => node.id));
  const identities = new Set(selected);
  const points = nodes.filter(node => identities.has(node.id));
  const centroid = points.reduce((sum, node) => sum.add(node.point), new Vector3()).divideScalar(points.length);
  assert.deepEqual(centroid.toArray(), vector.pivot);
  assert.equal(points.length, vector.nodeCount);
  assert.deepEqual(points.map(node => node.id), nodes.map(node => node.id));
  console.log(`[DEBUG] FEM3d ${selected.length} selection identities project to ${points.length} document-ordered nodes and the Three.js centroid`);
}

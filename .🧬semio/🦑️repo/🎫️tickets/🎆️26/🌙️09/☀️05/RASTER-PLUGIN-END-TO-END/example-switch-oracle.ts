import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import patch from "fast-json-patch";
import isEqual from "lodash/isEqual";

const cases: [string, string, string][] = JSON.parse(readFileSync("✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎬️set-active-example/🧪️tests/🔬️unit/fixture.json", "utf8"));
const reference = {
  id: "semio-demo",
  title: "Semio Raster Demo",
  layers: [{ id: "emblem", locked: false }],
  assets: { "semio-emblem": { childId: "art-by-content" } },
  materialized: ["semio-emblem"],
};
const edits: Record<string, patch.Operation[]> = {
  none: [],
  id: [{ op: "replace", path: "/id", value: "another-document" }],
  title: [{ op: "replace", path: "/title", value: "Edited title" }],
  asset: [{ op: "replace", path: "/assets/semio-emblem/childId", value: "different-content" }],
  owner: [{ op: "remove", path: "/materialized/0" }],
  lock: [{ op: "replace", path: "/layers/0/locked", value: true }],
};
for (const [name, change, expected] of cases) {
  const current = patch.applyPatch(reference, edits[change], true, false).newDocument;
  const actual = current.layers.some(layer => layer.locked) ? "refused" : isEqual(current, reference) ? "unchanged" : "load";
  assert.equal(actual, expected, name);
}
console.log(`[DEBUG] Raster example switch oracle passed ${cases.length} cases.`);

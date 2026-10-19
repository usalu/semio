/** 🧪️ JSON Patch independently validates the selected carrier and referrer deletion fixture. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { applyPatch } from "fast-json-patch";

/** 🗂️ Confirms exact load removal and complete document removal through the reference library. */
export function testFem3dRemovalOracle(): void {
  const vector = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const loads = applyPatch(structuredClone(vector.snapshot), [
    { op: "remove", path: "/loadCases/0/loads/2" },
    { op: "remove", path: "/loadCases/0/loads/1" },
    { op: "remove", path: "/loadCases/0/loads/0" },
  ], true).newDocument;
  assert.deepEqual(loads, { ...vector.snapshot, loadCases: [{ ...vector.snapshot.loadCases[0], loads: [] }] });
  const paths = ["combinations", "supports", "elements", "solids", "loadCases", "nodes", "sections", "materials"];
  const mutations = paths.flatMap(key => vector.snapshot[key].map((_: unknown, index: number) => ({ op: "remove" as const, path: `/${key}/${vector.snapshot[key].length - index - 1}` })));
  const empty = applyPatch(structuredClone(loads), mutations, true).newDocument;
  assert.deepEqual(empty, Object.fromEntries(Object.entries(vector.snapshot).map(([key, value]) => [key, key === "analysis" ? value : []])));
  assert.equal(vector.orderedKinds.length, new Set(vector.selection.filter((id: string) => id !== "scope:1:4:node7:missing")).size);
  console.log("[DEBUG] FEM3d removal vector agrees with JSON Patch for loads, nine granularities and dependency order");
}

import assert from "node:assert/strict";
import { applyPatch, type Operation } from "fast-json-patch";
import corpus from "../🧫️fixtures/🎞️results/🔣️.json" with { type: "json" };

/** 🎞️ Independently applies result-window publication and source-refusal transitions. */
export function testFem3dRetainedResultsOracle(): void {
  for (const kind of corpus.terminalKinds) {
    const state = applyPatch({ terminal: null as null | { kind: string; revision: string } }, [{ op: "replace", path: "/terminal", value: { kind, revision: "old" } }], true, false).newDocument;
    assert.equal(state.terminal?.revision === "old", true); assert.equal(state.terminal?.revision === "new", false);
  }
  const owners = new Map(corpus.appIds.map(id => [id, { id, running: true }]));
  for (const id of corpus.closeOrder) {
    assert(owners.delete(id));
    for (const [key, owner] of owners) assert.equal(owner.id, key);
  }
  assert.equal(owners.size, 0);
  const requests = new Map<string, Set<string>>();
  for (const row of corpus.requestWindows) {
    const key = JSON.stringify([row.family, row.namespace, row.source, row.index]);
    const windows = requests.get(key) ?? new Set<string>(); windows.add(row.window); requests.set(key, windows);
  }
  assert.equal(requests.size, 6);
  const shared = requests.get(JSON.stringify([0, 0, "a", 0]))!;
  assert(shared.delete("case-a")); assert.deepEqual([...shared], ["case-a-shared"]);
  const source = "a".repeat(corpus.wideSourceLength), window = "w".repeat(corpus.wideWindowLength);
  const original = { source, window }, projected = applyPatch({ source: "", window: "" }, [{ op: "replace", path: "/source", value: source }, { op: "replace", path: "/window", value: window }], true, false).newDocument;
  assert.deepEqual(projected, original);
  applyPatch(projected, [{ op: "replace", path: "/source", value: "" }, { op: "replace", path: "/window", value: "" }], true, false);
  assert.equal(original.source.length, corpus.wideSourceLength); assert.equal(original.window.length, corpus.wideWindowLength);
  for (const row of corpus.cases) {
    const operations: Operation[] = [];
    if (row.retainedSource) operations.push({ op: "replace", path: "/display", value: 1 });
    if (row.currentReady && row.currentRevision && row.currentSource) operations.push({ op: "replace", path: "/display", value: 2 });
    assert.equal(applyPatch({ display: null }, operations, true, false).newDocument.display, row.expected, row.id);
  }
}

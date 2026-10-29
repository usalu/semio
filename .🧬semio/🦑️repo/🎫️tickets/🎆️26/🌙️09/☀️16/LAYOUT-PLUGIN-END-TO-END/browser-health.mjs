/** 🩺️ Validate only status channels actually published by visible surface hosts. */
import assert from "node:assert/strict";
export function assertSurfaceHealthy(host) {
  const parse = (raw, label) => {
    if (raw === null || raw === undefined) return null;
    let value;
    try { value = JSON.parse(raw); } catch { assert.fail(label + " is malformed JSON"); }
    assert.ok(value !== null && typeof value === "object" && !Array.isArray(value), label + " must contain an object");
    return value;
  };
  const status = parse(host.statusJson, "surface status");
  const board = parse(host.boardStatusJson, "board status");
  if (board) {
    assert.notEqual(board.snapshotParsed, false, "board snapshot was refused");
    assert.ok(!board.refusalReason, "board refusal: " + board.refusalReason);
  }
  const fault = status?.fault ?? host.fault;
  const phase = status?.phase ?? host.phase;
  assert.ok(!fault, "surface fault payload: " + JSON.stringify(fault));
  assert.ok(!/fault|error|failed|panic|refus/i.test(phase ?? ""), "surface failure phase: " + phase);
}

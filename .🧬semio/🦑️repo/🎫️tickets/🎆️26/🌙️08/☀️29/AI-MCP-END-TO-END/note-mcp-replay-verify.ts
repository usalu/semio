import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const stages = ["baseline", "invoke", "undo", "redo", "rollback"] as const;
type Stage = typeof stages[number];
type SnapshotPair = Readonly<{ packBase64: string; sprBase64: string }>;
type Block = Readonly<{ id: string; kind: string; x: number; y: number }>;
type Snapshot = Readonly<{ schema: string; id: string; blocks: readonly Block[] }>;
type NoteJourney = Readonly<{
  artifactId: string;
  input: Readonly<{ kind?: string; x?: number; y?: number }>;
  observations: Readonly<Record<Stage, SnapshotPair>>;
}>;

export function verifyNoteJourney(executable: string, journey: NoteJourney): void {
  const input = stages.map(stage => {
    const pair = journey.observations[stage];
    assert(pair && isStandardBase64(pair.packBase64) && isStandardBase64(pair.sprBase64), "Expected the exact MCP snapshot pair at " + stage);
    return pair.packBase64 + "\t" + pair.sprBase64 + "\n";
  }).join("");
  const process = Bun.spawnSync([resolve(executable)], { stdin: Buffer.from(input), stderr: "inherit" });
  assert.equal(process.exitCode, 0, "Canonical Note snapshot replay failed");
  const rows = process.stdout.toString().trimEnd().split("\n");
  assert.equal(rows.length, stages.length, "Expected one replay result per observation");
  const [baseline, invoked, undone, redone, rolledBack] = rows.map(row => JSON.parse(row) as Snapshot);
  for (const snapshot of [baseline, invoked, undone, redone, rolledBack]) {
    assert.equal(snapshot.schema, "note.document");
    assert.equal(snapshot.id, journey.artifactId);
    assert(Array.isArray(snapshot.blocks));
  }

  assert.deepEqual(undone, baseline, "Undo must restore the baseline Note");
  assert.deepEqual(redone, invoked, "Redo must restore the Note produced by invocation");
  assert.deepEqual(rolledBack, invoked, "Rollback must preserve the redone Note");

  const oldIds = new Set(baseline.blocks.map(block => block.id));
  const inserted = invoked.blocks.filter(block => !oldIds.has(block.id));
  assert.equal(inserted.length, 1, "Add Block must insert one block with a fresh identity");
  const [block] = inserted;
  assert(block.id.length > 0);
  const { kind = "text", x = 0, y = 0 } = journey.input;
  const materializedKind = ["image", "table", "math", "stroke", "group"].includes(kind) ? kind : "text";
  assert.deepEqual({ kind: block.kind, x: block.x, y: block.y }, { kind: materializedKind, x, y });
  assert.deepEqual({ ...invoked, blocks: invoked.blocks.filter(row => row.id !== block.id) }, baseline,
    "Add Block must preserve the other content of the Note");
}

function isStandardBase64(value: unknown): value is string {
  return typeof value === "string" && value.length > 0 && Buffer.from(value, "base64").toString("base64") === value;
}

if (import.meta.main) {
  const [executable, capture] = Bun.argv.slice(2);
  assert(executable && capture, "Expected the canonical replay executable and the MCP capture JSON file");
  verifyNoteJourney(executable, JSON.parse(readFileSync(capture, "utf8")));
  console.log("Verified the materialized Note across baseline, invoke, undo, redo and rollback");
}

import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

/** 🔒️ One preview window's latch, re-implemented from the fixture's prose — the independent half of the
 * scheduling law the Rust laws drive against the real `FlowEvalSession`s. */
interface Latch {
  armed: boolean;
  inFlight: number;
  unfinished: boolean;
}

type Target = "document" | "generation";
type Window = { id: string; target: Target };
type Step = { step: string; window: string; more?: boolean; parked?: number; count?: number };

class LatchTables {
  readonly tables = new Map<Target, Map<string, Latch>>([["document", new Map()], ["generation", new Map()]]);

  table(target: Target): Map<string, Latch> {
    return this.tables.get(target) as Map<string, Latch>;
  }

  at(window: Window): Latch {
    const table = this.table(window.target);
    let latch = table.get(window.id);
    if (!latch) {
      latch = { armed: false, inFlight: 0, unfinished: false };
      table.set(window.id, latch);
    }
    return latch;
  }

  owes(window: Window): boolean {
    const latch = this.table(window.target).get(window.id);
    return latch === undefined || (latch.unfinished && !latch.armed && latch.inFlight === 0);
  }
}

function replay(tables: LatchTables, windows: readonly Window[], step: Step): void {
  const window = windows.find((candidate) => candidate.id === step.window) ?? { id: step.window, target: "document" as const };
  const latch = tables.at(window);
  switch (step.step) {
    case "arm":
      assert.ok(!latch.armed && latch.inFlight === 0, `${step.window}: arming needs a quiet latch`);
      latch.armed = true;
      return;
    case "begin":
      latch.armed = false;
      return;
    case "outcome":
      latch.unfinished = step.more === true || (step.parked ?? 0) > 0;
      return;
    case "inFlight":
      latch.inFlight += step.count ?? 0;
      return;
    case "settle":
      latch.inFlight = Math.max(0, latch.inFlight - 1);
      return;
    case "owe":
      latch.unfinished = true;
      return;
    case "abort": {
      const table = tables.table(window.target);
      for (const key of [...table.keys()]) table.set(key, { armed: false, inFlight: 0, unfinished: false });
      table.set(step.window, { armed: false, inFlight: 0, unfinished: false });
      return;
    }
    default:
      throw new Error(`unknown latch step ${step.step}`);
  }
}

function nextHop(tables: LatchTables, windows: readonly Window[]): { hop: string; window?: string } {
  const owed = windows.find((window) => tables.owes(window));
  if (owed !== undefined) return { hop: "dispatch", window: owed.id };
  return windows.some((window) => tables.at(window).armed || tables.at(window).inFlight > 0) ? { hop: "wait" } : { hop: "settled" };
}

/** 🪪️ FNV-1a 64 over UTF-8, written out independently of the Rust key function. */
function fnv1a64(text: string): bigint {
  let hash = 0xcbf29ce484222325n;
  for (const byte of new TextEncoder().encode(text)) {
    hash ^= BigInt(byte);
    hash = (hash * 0x100000001b3n) & 0xffffffffffffffffn;
  }
  return hash;
}

const UNSETTLEDNESS: Record<string, number> = { computing: 5, queued: 4, failed: 3, blocked: 2, evaluated: 1 };

function observe(targets: readonly unknown[]): { nodes: Record<string, string>; stage: string; progress: [number, number] } {
  const nodes: Record<string, string> = {};
  let settled = 0;
  let total = 0;
  for (const target of targets) {
    let status: unknown = target;
    if (typeof target === "string") {
      try {
        status = JSON.parse(target);
      } catch {
        continue;
      }
    }
    if (typeof status !== "object" || status === null || Array.isArray(status)) continue;
    for (const [node, entry] of Object.entries(status as Record<string, { status?: string }>)) {
      const reason = ({ ok: "evaluated", computing: "computing", error: "failed", blocked: "blocked" } as Record<string, string>)[entry.status ?? ""] ?? "queued";
      total += 1;
      settled += reason === "queued" || reason === "computing" ? 0 : 1;
      const held = nodes[node];
      nodes[node] = held === undefined || UNSETTLEDNESS[reason] > UNSETTLEDNESS[held] ? reason : held;
    }
  }
  return { nodes, stage: "evaluate", progress: [settled, total] };
}

/** ⚖️ Independent TypeScript twin of generation2d's `⏯️preview-eval-run.json`: entity keys, the target
 * table, the merged observation, the scheduling law over per-target latches, what pending_effects owes the
 * run and what a moved document owes — nothing shared with Rust but the fixture. */
export function testGeneration2dPreviewEvalRunContract(): void {
  const here = fileURLToPath(new URL(".", import.meta.url));
  const fixture = JSON.parse(readFileSync(`${here}/../../../🧫️fixtures/⏯️preview-eval-run.json`, "utf8"));
  assert.equal(fixture.format, "semio.generation2d.preview-eval-run");
  assert.equal(fixture.version, 1);
  const record = JSON.parse(readFileSync(`${here}/../../🔣️.json`, "utf8"));
  assert.deepEqual(record.definition.reasons.map((reason: { code: number; id: string; verdict: string }) => ({ code: reason.code, id: reason.id, verdict: reason.verdict })), fixture.vocabulary.reasons, "the source of record and the fixture name the same reasons");
  assert.deepEqual(record.definition.counters.map((counter: { id: string }) => counter.id), fixture.vocabulary.counters);
  for (const reason of record.definition.reasons) assert.notEqual(reason.template.native.en, reason.template.native.de, `${reason.id} is translated`);
  for (const row of fixture.entityKeys) assert.equal(fnv1a64(row.nodeId).toString(), row.entity, `entity key of ${JSON.stringify(row.nodeId)}`);
  const targetKinds = new Map<string, string>([["generation2d-preview", "document"], ["generation2d-generate-preview", "generation"]]);
  for (const row of fixture.targets.rows) assert.equal(targetKinds.get(row.kind) ?? null, row.target, `${row.kind}: target`);
  for (const row of fixture.observations) assert.deepEqual(observe(row.targets), row.expected, `${row.id}: observation`);
  for (const row of fixture.scheduling) {
    const tables = new LatchTables();
    for (const step of row.steps as Step[]) replay(tables, row.windows, step);
    assert.deepEqual(nextHop(tables, row.windows), row.expected, `${row.id}: hop`);
  }
  for (const row of fixture.runEffects) {
    const servable = row.servable ?? true;
    const actions: string[] = [];
    if (row.windows.length > 0 && servable) {
      const state = row.run?.state ?? null;
      if (state === "complete") actions.push("toolRunFinalize");
      else if ((state === null || state === "finalized" || state === "aborted" || state === "faulted") && (row.owed || row.restartOwed === true)) actions.push("toolRunStart");
    }
    assert.deepEqual(actions, row.expected, `${row.id}: owed run actions`);
  }
  for (const row of fixture.documentMoved.rows) {
    const owed = (row.windows as Window[]).filter((window) => row.evaluated[window.target] !== row.current[window.target]).map((window) => window.id);
    assert.deepEqual(owed, row.expected.owedWindows, `${row.id}: owed windows`);
  }
  console.log(`generation2d preview-eval-run entityKeys=${fixture.entityKeys.length} targets=${fixture.targets.rows.length} observations=${fixture.observations.length} scheduling=${fixture.scheduling.length} runEffects=${fixture.runEffects.length} documentMoved=${fixture.documentMoved.rows.length}`);
}

import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

/** ⚖️ Third-party twin of the contribution-gated arming fixture: `JSON.parse` must see the same two
 * arms the Rust law drives — an uncontributed graph owing the run no hop, a served graph owing exactly
 * one — and `setContributions` as the one route that carries a settled preview its restart. */
export function testGeneration3dContributionGatedArmingContract(): void {
  const here = fileURLToPath(new URL(".", import.meta.url));
  const fixture = JSON.parse(readFileSync(`${here}/../../../../🧫️fixtures/🚧️contribution-gated-arming.json`, "utf8")) as {
    format: string;
    version: number;
    unservedKind: string;
    servedKindCandidates: string[];
    tickStepBudget: number;
    cases: Array<{ id: string; kindSource: string; widgets: number; unfinishedTick: boolean; mayRearm: boolean; owedHops: number }>;
    resume: { command: string; carriedRunAction: string };
  };
  assert.equal(fixture.format, "semio.generation3d.contribution-gated-arming");
  assert.equal(fixture.version, 1);
  const blocked = fixture.cases.find((row) => row.kindSource === "unservedKind");
  const served = fixture.cases.find((row) => row.kindSource === "servedKindCandidates");
  assert.equal(blocked?.mayRearm, false);
  assert.equal(blocked?.owedHops, 0);
  assert.equal(served?.mayRearm, true);
  assert.equal(served?.owedHops, 1);
  for (const row of [blocked, served]) {
    assert.equal(row?.unfinishedTick, true, "both arms must outrun one tick, or the continuation decision is never reached");
    assert.ok((row?.widgets ?? 0) > fixture.tickStepBudget, "a graph that fits in one tick never owes a continuation");
  }
  assert.equal(fixture.resume.command, "setContributions");
  assert.equal(fixture.resume.carriedRunAction, "toolRunStart");
  console.log(`generation3d contribution-gated arming blocked=${blocked?.owedHops} served=${served?.owedHops} resume=${fixture.resume.command}->${fixture.resume.carriedRunAction} budget=${fixture.tickStepBudget}`);
}

/** 🔒️ One preview window's latch, re-implemented from `⏯️preview-eval-run.json`'s own prose — the
 * independent half of the scheduling law. Rust drives the identical rows against the real
 * `FlowEvalSession`; both must answer the same hop. */
interface Latch {
  armed: boolean;
  inFlight: number;
  unfinished: boolean;
}

class LatchTable {
  readonly latches = new Map<string, Latch>();

  at(windowId: string): Latch {
    let latch = this.latches.get(windowId);
    if (!latch) {
      latch = { armed: false, inFlight: 0, unfinished: false };
      this.latches.set(windowId, latch);
    }
    return latch;
  }

  owes(windowId: string): boolean {
    const latch = this.latches.get(windowId);
    return latch === undefined || (latch.unfinished && !latch.armed && latch.inFlight === 0);
  }
}

type Step = { step: string; window: string; more?: boolean; parked?: number; count?: number };

function replay(table: LatchTable, step: Step): void {
  const latch = table.at(step.window);
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
    case "abort":
      for (const key of [...table.latches.keys()]) table.latches.set(key, { armed: false, inFlight: 0, unfinished: false });
      table.latches.set(step.window, { armed: false, inFlight: 0, unfinished: false });
      return;
    default:
      throw new Error(`unknown latch step ${step.step}`);
  }
}

function nextHop(table: LatchTable, windows: readonly string[]): { hop: string; window?: string } {
  const owed = windows.find((windowId) => table.owes(windowId));
  if (owed !== undefined) return { hop: "dispatch", window: owed };
  return windows.some((windowId) => table.at(windowId).armed || table.at(windowId).inFlight > 0) ? { hop: "wait" } : { hop: "settled" };
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

type Observation = { nodes: Record<string, string>; meshes: number; tessellated: number; stage: string; progress: [number, number] };

function observe(statusJson: unknown, evalJson: Record<string, unknown>, previewWidgetIds: readonly string[], meshStates: Record<string, string>, settled: boolean): Observation {
  const observation: Observation = { nodes: {}, meshes: 0, tessellated: 0, stage: "evaluate", progress: [0, 0] };
  let status: unknown = statusJson;
  if (typeof statusJson === "string") {
    try {
      status = JSON.parse(statusJson);
    } catch {
      return observation;
    }
  }
  if (typeof status !== "object" || status === null || Array.isArray(status)) return observation;
  const handlesOf = (value: unknown, found: string[]): void => {
    if (Array.isArray(value)) return value.forEach((entry) => handlesOf(entry, found));
    if (typeof value !== "object" || value === null) return;
    const record = value as Record<string, unknown>;
    if (typeof record.handle === "string" && /^(solid|shell|face|wire|edge|vertex|compound|curve|surface)-/.test(record.handle)) {
      found.push(record.handle);
      return;
    }
    if (record.$schema === "list") {
      Object.keys(record).filter((key) => /^\d+$/.test(key)).sort((a, b) => Number(a) - Number(b)).forEach((key) => handlesOf(record[key], found));
    }
  };
  for (const [node, entry] of Object.entries(status as Record<string, { status?: string }>)) {
    let reason = ({ ok: "evaluated", computing: "computing", error: "failed", blocked: "blocked" } as Record<string, string>)[entry.status ?? ""] ?? "queued";
    if (reason === "evaluated" && previewWidgetIds.includes(node)) {
      const widget = evalJson[node] as { out?: Record<string, unknown>; in?: Record<string, unknown> } | undefined;
      const channels = widget?.out ?? widget?.in ?? {};
      const handles: string[] = [];
      for (const key of Object.keys(channels).sort()) handlesOf(channels[key], handles);
      const states = handles.map((handle) => meshStates[handle] ?? "pending");
      observation.meshes += states.length;
      observation.tessellated += states.filter((state) => state === "ready").length;
      if (states.length > 0) reason = states.includes("diagnostics") ? "meshDiagnostics" : states.every((state) => state === "ready") ? "tessellated" : settled ? "meshMissing" : "tessellating";
    }
    observation.nodes[node] = reason;
  }
  const reasons = Object.values(observation.nodes);
  const unsettled = reasons.filter((reason) => reason === "queued" || reason === "computing").length;
  observation.stage = unsettled > 0 || observation.meshes === observation.tessellated ? "evaluate" : "tessellate";
  observation.progress = [reasons.length - unsettled + observation.tessellated, reasons.length + observation.meshes];
  return observation;
}

/** ⚖️ Independent TypeScript twin of `⏯️preview-eval-run.json`: the scheduling law over the per-window
 * latches, the node verdict observation, the FNV-1a entity keys, the abort affordance and what a
 * surface's pending_effects owes the run — nothing shared with Rust but the fixture. */
export function testGeneration3dPreviewEvalRunContract(): void {
  const here = fileURLToPath(new URL(".", import.meta.url));
  const fixture = JSON.parse(readFileSync(`${here}/../../../../../🧫️fixtures/⏯️preview-eval-run.json`, "utf8"));
  assert.equal(fixture.format, "semio.generation3d.preview-eval-run");
  assert.equal(fixture.version, 1);
  for (const row of fixture.entityKeys) assert.equal(fnv1a64(row.nodeId).toString(), row.entity, `entity key of ${JSON.stringify(row.nodeId)}`);
  for (const row of fixture.observations) {
    const observed = observe(row.statusJson, row.evalJson, row.previewWidgetIds, row.meshStates, row.settled);
    assert.deepEqual(observed.nodes, row.expected.nodes, `${row.id}: nodes`);
    assert.deepEqual([observed.meshes, observed.tessellated, observed.stage], [row.expected.meshes, row.expected.tessellated, row.expected.stage], `${row.id}: census`);
    assert.deepEqual(observed.progress, row.expected.progress, `${row.id}: progress`);
  }
  for (const row of fixture.scheduling) {
    const table = new LatchTable();
    for (const step of row.steps as Step[]) replay(table, step);
    assert.deepEqual(nextHop(table, row.windows), row.expected, `${row.id}: hop`);
  }
  const abortable = new Set(["starting", "running", "paused"]);
  for (const row of fixture.status) {
    const run = row.run;
    const cancellable = run !== null && run.toolId === fixture.toolId && abortable.has(run.state);
    assert.equal(cancellable, row.expected.cancellable, `${row.id}: cancellable`);
    assert.deepEqual(cancellable ? { runId: String(run.run), generation: run.generation } : null, row.expected.cancelArgs, `${row.id}: cancelArgs`);
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
  console.log(`generation3d preview-eval-run entityKeys=${fixture.entityKeys.length} observations=${fixture.observations.length} scheduling=${fixture.scheduling.length} status=${fixture.status.length} runEffects=${fixture.runEffects.length}`);
}

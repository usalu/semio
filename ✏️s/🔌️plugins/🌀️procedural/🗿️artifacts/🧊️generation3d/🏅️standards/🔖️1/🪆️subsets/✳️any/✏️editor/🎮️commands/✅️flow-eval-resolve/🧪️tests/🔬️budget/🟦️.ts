import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

/** ⏱️ The independent TypeScript twin of the `evaluate` BUDGET law, re-implemented from
 * `🧫️fixtures/⏱️evaluate-budget.json`'s own prose — the second half of the language-agnostic
 * contract. Rust drives the identical rows against the real `FlowEvalSession`, the real fold and the
 * real status projection; nothing is shared between the two implementations but the fixture
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️extension-evaluate-budget-2026-09-12.md`). */

interface Envelope {
  done?: boolean;
  cancellable?: boolean;
  phase?: string;
  unitsDone?: number;
  unitsTotal?: number;
  outputJson?: string;
}

interface Row {
  id: string;
  envelope?: Envelope;
  bareOutputJson?: string;
  outcome: string;
  seeds: boolean;
  seededOutputJson?: string;
  owesHop: boolean;
  status: {
    phase: string;
    inFlight: number;
    evalUnitsDone: number;
    evalUnitsTotal: number;
    ratio: number;
  };
}

interface Fixture {
  format: string;
  version: number;
  capability: string;
  cancelCapability: string;
  responseAction: string;
  requestFields: string[];
  envelopeFields: string[];
  phaseLabels: Record<string, { en: string; de: string }>;
  jobPhaseTags: Record<string, string>;
  rows: Row[];
  steppedOperator: { operatorId: string; minimumRoundTripsUnderATightBudget: number; phaseOrder: string[] };
  inlineContinuation: InlineContinuation;
}

interface InlineContinuationExample {
  id: string;
  contributedNodes: number;
  waveWidths: number[];
  beforeCoalescing: number;
  afterCoalescing: number;
  afterInline: number;
}

interface InlineContinuationRow {
  id: string;
  waveWidths: number[];
  cancelBeforeHop?: number;
  spentTurnBeforeHop?: number;
  expectedDispatchedHops: number;
  expectedInlineWaves: number;
  expectedContinuationsAfterCancel?: number;
}

interface InlineContinuation {
  examples: InlineContinuationExample[];
  rows: InlineContinuationRow[];
}

/** 🏷️ The surface's own phase vocabulary, written out from the fixture's `laws` rather than read
 * from its `jobPhaseTags` table — an oracle that read the answer key would prove nothing. */
function surfacePhaseFor(jobTag: string): string {
  if (jobTag === "imprint") return "imprinting";
  if (jobTag === "applyA" || jobTag === "applyB") return "splitting";
  if (jobTag === "classifyA" || jobTag === "classifyB") return "classifying";
  if (jobTag === "stitch") return "stitching";
  if (jobTag === "validate") return "validating";
  if (jobTag === "complete") return "complete";
  if (jobTag === "cancelled") return "cancelled";
  if (jobTag === "idle") return "idle";
  // 🩺️ `anUnknownPhaseIsComputing`: the envelope that carried it said the job is still working.
  return "computing";
}

/** ⏱️ The independent model: one node-cache seed slot, one evaluation progress row keyed by node
 * hash, and whether the window still owes the run a hop after the fold (`answersArmNothing`). */
class BudgetSession {
  seeded: string | null = null;
  progress: { unitsDone: number; unitsTotal: number; phase: string } | null = null;
  owesHop = true;

  /** ✅️ Folds one answer body. `laws.aBareDictionaryIsAFinishedEvaluation`: a body that is not an
   * envelope is a finished evaluation whose output IS that body. */
  fold(body: string): string {
    let parsed: unknown;
    try {
      parsed = JSON.parse(body) as unknown;
    } catch {
      return this.complete(body);
    }
    if (typeof parsed !== "object" || parsed === null || !("done" in parsed) || typeof (parsed as Envelope).done !== "boolean") {
      return this.complete(body);
    }
    const envelope = parsed as Envelope;
    const phase = surfacePhaseFor(envelope.phase ?? "computing");
    if (envelope.done !== true) {
      // 🚧️ `workingSeedsNothing` + `oneRequestOneStep`: park the progress and owe one more identical
      // round trip.
      this.progress = { unitsDone: envelope.unitsDone ?? 0, unitsTotal: envelope.unitsTotal ?? 0, phase };
      return "working";
    }
    this.progress = null;
    if (phase === "cancelled") {
      // 🛑️ `cancelledOwesNothing`.
      this.owesHop = false;
      return "cancelled";
    }
    return this.complete(envelope.outputJson ?? "");
  }

  private complete(outputJson: string): string {
    this.seeded = outputJson;
    return "complete";
  }

  /** 📈️ The status a preview window publishes right now.
   *
   * ⛓️️ TWO ledgers, one object. The budgeted-eval ledger names the PHASE and carries
   * `evalUnitsDone`/`evalUnitsTotal` while a round trip is parked. The moment it is empty — which is
   * every hop boundary of a chain — the CHAIN ledger still knows the window owes a hop, and it owns
   * the published phase and fraction there. A session with no node census of its own (no `FlowHost`
   * synced, which is exactly these rows) has no denominator, so a working chain publishes `0.0` and
   * NEVER `1.0`: "done" is the one answer live work may not give. Saying `idle` for a window that
   * owes another hop is the defect this fixture exists to forbid
   * (`📓️wgpu-progress-visibility-2026-09-14.md`). */
  status(): { phase: string; inFlight: number; evalUnitsDone: number; evalUnitsTotal: number; ratio: number } {
    const inFlight = this.progress ? 1 : 0;
    const unitsDone = this.progress?.unitsDone ?? 0;
    const unitsTotal = this.progress?.unitsTotal ?? 0;
    const chainWorking = this.owesHop;
    if (this.progress) {
      const ratio = unitsTotal === 0 ? 0 : Math.min(1, Math.max(0, unitsDone / unitsTotal));
      return { phase: this.progress.phase, inFlight, evalUnitsDone: unitsDone, evalUnitsTotal: unitsTotal, ratio };
    }
    return { phase: chainWorking ? "computing" : "idle", inFlight, evalUnitsDone: 0, evalUnitsTotal: 0, ratio: chainWorking ? 0 : 1 };
  }
}

function fixture(): Fixture {
  const path = fileURLToPath(new URL("../../../../../🧫️fixtures/⏱️evaluate-budget.json", import.meta.url));
  return JSON.parse(readFileSync(path, "utf8")) as Fixture;
}

/** ⚖️ Replays every row of the fixture against the independent model. */
export function testGeneration3dEvaluateBudgetContract(): void {
  const loaded = fixture();
  assert.equal(loaded.format, "semio.generation3d.evaluate-budget");
  assert.equal(loaded.version, 1);
  assert.equal(loaded.capability, "evaluate");
  assert.equal(loaded.cancelCapability, "evaluateCancel");
  assert.equal(loaded.responseAction, "flowEvalResolve");
  for (const field of ["operatorId", "inputJson", "nodeHash", "budget", "wallMicros"]) {
    assert.ok(loaded.requestFields.includes(field), `the request declares ${field}`);
  }
  for (const field of ["done", "phase", "unitsDone", "unitsTotal", "outputJson"]) {
    assert.ok(loaded.envelopeFields.includes(field), `the envelope declares ${field}`);
  }
  // 🏷️ The declared job→surface phase table must agree with the independently written vocabulary.
  for (const [jobTag, surfaceTag] of Object.entries(loaded.jobPhaseTags)) {
    assert.equal(surfacePhaseFor(jobTag), surfaceTag, `job phase ${jobTag} projects to ${surfaceTag}`);
  }
  assert.equal(surfacePhaseFor("a-phase-no-surface-has-heard-of"), "computing");
  for (const row of loaded.rows) {
    const session = new BudgetSession();
    const body = row.envelope !== undefined ? JSON.stringify(row.envelope) : row.bareOutputJson;
    assert.ok(body !== undefined, `${row.id}: a row declares either an envelope or a bare output body`);
    const outcome = session.fold(body);
    assert.equal(outcome, row.outcome, `${row.id}: outcome`);
    assert.equal(session.seeded !== null, row.seeds, `${row.id}: seeds`);
    if (row.seededOutputJson !== undefined) {
      assert.deepEqual(JSON.parse(session.seeded ?? "null"), JSON.parse(row.seededOutputJson), `${row.id}: seeded output`);
    }
    assert.equal(session.owesHop, row.owesHop, `${row.id}: the window owes the run a hop`);
    const status = session.status();
    assert.equal(status.phase, row.status.phase, `${row.id}: published phase`);
    assert.equal(status.inFlight, row.status.inFlight, `${row.id}: inFlight`);
    assert.equal(status.evalUnitsDone, row.status.evalUnitsDone, `${row.id}: evalUnitsDone`);
    assert.equal(status.evalUnitsTotal, row.status.evalUnitsTotal, `${row.id}: evalUnitsTotal`);
    assert.ok(Math.abs(status.ratio - row.status.ratio) < 1e-9, `${row.id}: ratio ${status.ratio} != ${row.status.ratio}`);
    const labels = loaded.phaseLabels[row.status.phase];
    assert.ok(labels, `${row.id}: the fixture declares a label for ${row.status.phase}`);
    assert.ok(labels.en.length > 0 && labels.de.length > 0, `${row.id}: both languages, with no default`);
  }
  // 📈️ `progressIsMonotone`, replayed as a SEQUENCE — monotonicity is a property of the sequence.
  const session = new BudgetSession();
  const order = loaded.steppedOperator.phaseOrder;
  assert.ok(order.length >= loaded.steppedOperator.minimumRoundTripsUnderATightBudget, "the declared phase order admits the declared minimum of round trips");
  let previousDone = 0;
  for (const [index, phase] of order.entries()) {
    const done = index === order.length - 1;
    const outcome = session.fold(JSON.stringify({ done, cancellable: !done, phase, unitsDone: index + 1, unitsTotal: order.length, outputJson: done ? '{"solid":"brep:solid-9"}' : "" }));
    assert.equal(outcome, done ? "complete" : "working", `${phase}: outcome`);
    const status = session.status();
    if (done) {
      assert.ok(session.seeded !== null, "the terminal answer seeds");
      assert.equal(status.evalUnitsDone, 0, "a finished evaluation leaves no in-flight row behind");
    } else {
      assert.equal(session.seeded, null, `${phase} is still working, so nothing may be seeded`);
      assert.ok(status.evalUnitsDone >= previousDone, `unitsDone never decreases (${previousDone} -> ${status.evalUnitsDone} at ${phase})`);
      assert.equal(status.inFlight, 1, `${phase} is live work`);
      assert.notEqual(status.phase, "idle", `${phase} must never publish as idle`);
      previousDone = status.evalUnitsDone;
    }
    assert.equal(session.owesHop, true, `${phase} owes exactly one continuation`);
  }
}

/** 🔒️ The independent model of ONE preview window's tick latch, written from the fixture's
 * `inlineContinuation.admission` prose alone. `armed` is the single outstanding hop, `inFlight`
 * counts the answers the last hop parked, `unfinished` is what that hop reported, and `cancelled` is
 * the banner a gesture raised. Nothing here is shared with the Rust `FlowEvalSession`. */
class WindowLatch {
  armed = false;
  inFlight = 0;
  unfinished = true;
  cancelled = false;
  everTicked = false;

  /** 🔎️ `owedHopOnly`: a window owes a hop nothing is chasing when it has never ticked, or its own
   * last hop reported more work and neither a hop nor an answer is outstanding. */
  owesHop(): boolean {
    if (!this.everTicked) return true;
    return this.unfinished && !this.armed && this.inFlight === 0;
  }

  /** 🔁️ The admission rule, all three clauses. */
  continuationAdmitted(turnSpent: boolean): boolean {
    return !this.cancelled && this.owesHop() && !turnSpent;
  }

  claim(): void {
    this.armed = true;
  }

  /** ▶️ One hop runs: it begins (retiring the cancelled banner and freeing the latch) and parks
   * `width` answers, or parks none and records the window finished. */
  runHop(width: number | null): void {
    this.armed = false;
    this.everTicked = true;
    this.cancelled = false;
    if (width === null) {
      this.unfinished = false;
      return;
    }
    this.unfinished = true;
    this.inFlight += width;
  }

  settle(): void {
    this.inFlight = Math.max(0, this.inFlight - 1);
  }

  /** 🛑 A cancel defaults the latch and raises the banner, so every answer still crossing settles
   * into a window that owes nothing. */
  cancel(): void {
    this.armed = false;
    this.inFlight = 0;
    this.unfinished = false;
    this.cancelled = true;
  }
}

interface ChainLadder {
  dispatchedHops: number;
  inlineWaves: number;
  continuationsAfterCancel: number;
}

/** ⛓️ The hop ladder of one chain, driven through the independent latch exactly as the run job and
 * the two window-addressed folds drive the real one. */
function replayChain(waveWidths: number[], cancelBeforeHop?: number, spentTurnBeforeHop?: number): ChainLadder {
  const latch = new WindowLatch();
  const ladder: ChainLadder = { dispatchedHops: 0, inlineWaves: 0, continuationsAfterCancel: 0 };
  assert.ok(latch.owesHop(), "the gesture leaves the window owing its first hop");
  latch.claim();
  ladder.dispatchedHops += 1;
  let cancelled = false;
  for (const [index, width] of waveWidths.entries()) {
    const hop = index + 1;
    latch.runHop(width);
    let continued = false;
    for (let answer = 0; answer < width; answer += 1) {
      const last = answer + 1 === width;
      if (last && cancelBeforeHop === hop + 1) {
        latch.cancel();
        cancelled = true;
      }
      latch.settle();
      const turnSpent = last && spentTurnBeforeHop === hop + 1;
      if (latch.continuationAdmitted(turnSpent)) {
        latch.claim();
        continued = true;
        ladder.inlineWaves += 1;
        if (cancelled) ladder.continuationsAfterCancel += 1;
      }
    }
    if (cancelled) break;
    if (!continued) {
      assert.ok(latch.owesHop(), "a declined continuation leaves the debt where the scheduler reads it");
      latch.claim();
      ladder.dispatchedHops += 1;
    }
  }
  if (!cancelled) {
    latch.runHop(null);
    assert.ok(!latch.owesHop(), "the terminal walk owes nothing and the run settles");
  }
  return ladder;
}

/** ⚖️ LAW: the inline-continuation section of the same fixture, answered by the independent model —
 * one dispatched hop per example however deep the graph, and one handed back per declined
 * continuation (a cancel between waves, a turn with no wall left). */
export function testGeneration3dInlineContinuationContract(): void {
  const loaded = fixture();
  const inline = loaded.inlineContinuation;
  assert.equal(inline.examples.length, 8, "all eight bundled examples declare their wave shape");
  for (const example of inline.examples) {
    assert.equal(example.contributedNodes, example.waveWidths.reduce((total, width) => total + width, 0), `${example.id}: the waves account for every contributed node`);
    assert.equal(example.afterCoalescing, example.waveWidths.length + 1, `${example.id}: coalescing bottoms out at one hop per level plus a terminal one`);
    assert.ok(example.beforeCoalescing >= example.afterCoalescing, `${example.id}: coalescing never made an example worse`);
    const ladder = replayChain(example.waveWidths);
    assert.equal(ladder.dispatchedHops, example.afterInline, `${example.id}: dispatched hops`);
    assert.equal(ladder.dispatchedHops, 1, `${example.id}: one gesture, one hop, however deep the graph`);
    assert.equal(ladder.inlineWaves, example.waveWidths.length, `${example.id}: every wave after the first hop runs inside an answer's own turn`);
  }
  assert.equal(inline.rows.length, 3, "the fixture declares all three interference rows");
  for (const row of inline.rows) {
    const ladder = replayChain(row.waveWidths, row.cancelBeforeHop, row.spentTurnBeforeHop);
    assert.equal(ladder.dispatchedHops, row.expectedDispatchedHops, `${row.id}: dispatched hops`);
    assert.equal(ladder.inlineWaves, row.expectedInlineWaves, `${row.id}: inline waves`);
    if (row.expectedContinuationsAfterCancel !== undefined) {
      assert.equal(ladder.continuationsAfterCancel, row.expectedContinuationsAfterCancel, `${row.id}: a cancelled chain admits no continuation at all`);
    }
  }
}

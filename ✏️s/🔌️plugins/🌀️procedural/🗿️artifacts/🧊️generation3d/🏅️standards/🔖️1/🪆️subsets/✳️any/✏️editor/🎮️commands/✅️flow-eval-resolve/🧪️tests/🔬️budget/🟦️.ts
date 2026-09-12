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
  rearmsTicks: number;
  status: {
    phase: string;
    inFlight: number;
    evalUnitsDone: number;
    evalUnitsTotal: number;
    ratio: number;
    cancellable: boolean;
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

const CANCELLABLE_SURFACE_PHASES = new Set(["computing", "imprinting", "splitting", "classifying", "stitching", "validating"]);

/** ⏱️ The independent model: one node-cache seed slot, one evaluation progress row keyed by node
 * hash, one per-window arming latch, and the continuation each fold owes. */
class BudgetSession {
  seeded: string | null = null;
  progress: { unitsDone: number; unitsTotal: number; phase: string } | null = null;
  armed = false;
  rearms = 0;

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
      this.owe();
      return "working";
    }
    this.progress = null;
    if (phase === "cancelled") {
      // 🛑️ `cancelledOwesNothing`.
      return "cancelled";
    }
    return this.complete(envelope.outputJson ?? "");
  }

  private complete(outputJson: string): string {
    this.seeded = outputJson;
    this.owe();
    return "complete";
  }

  private owe(): void {
    if (this.armed) return;
    this.armed = true;
    this.rearms += 1;
  }

  /** ▶️ The armed tick actually runs, freeing the latch for the next round trip. */
  beginTick(): void {
    this.armed = false;
  }

  /** 📈️ The status a preview window publishes right now. */
  status(): { phase: string; inFlight: number; evalUnitsDone: number; evalUnitsTotal: number; ratio: number; cancellable: boolean } {
    const inFlight = this.progress ? 1 : 0;
    const unitsDone = this.progress?.unitsDone ?? 0;
    const unitsTotal = this.progress?.unitsTotal ?? 0;
    const ratio = unitsTotal === 0 ? (inFlight === 0 ? 1 : 0) : Math.min(1, Math.max(0, unitsDone / unitsTotal));
    const phase = this.progress ? this.progress.phase : "idle";
    return { phase, inFlight, evalUnitsDone: unitsDone, evalUnitsTotal: unitsTotal, ratio, cancellable: inFlight > 0 || CANCELLABLE_SURFACE_PHASES.has(phase) };
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
    assert.equal(session.rearms, row.rearmsTicks, `${row.id}: re-armed tick count`);
    const status = session.status();
    assert.equal(status.phase, row.status.phase, `${row.id}: published phase`);
    assert.equal(status.inFlight, row.status.inFlight, `${row.id}: inFlight`);
    assert.equal(status.evalUnitsDone, row.status.evalUnitsDone, `${row.id}: evalUnitsDone`);
    assert.equal(status.evalUnitsTotal, row.status.evalUnitsTotal, `${row.id}: evalUnitsTotal`);
    assert.ok(Math.abs(status.ratio - row.status.ratio) < 1e-9, `${row.id}: ratio ${status.ratio} != ${row.status.ratio}`);
    assert.equal(status.cancellable, row.status.cancellable, `${row.id}: cancellable`);
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
      assert.equal(status.cancellable, true, `${phase} is stoppable`);
      assert.notEqual(status.phase, "idle", `${phase} must never publish as idle`);
      previousDone = status.evalUnitsDone;
    }
    assert.equal(session.rearms, index + 1, `${phase} owes exactly one continuation`);
    session.beginTick();
  }
}

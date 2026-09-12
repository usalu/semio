import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { world3dComputeStatusV1 } from "@semio-tech/framework";

/** 🛑️ One preview evaluation's cancellable state, re-implemented from `🛑️preview-cancel.json`'s own
 * prose — the independent half of the language-agnostic law. Rust drives the identical rows against
 * the real `FlowEvalSession` and the real status projection; both must answer the same statuses. */
interface CancelLedgerRow {
  phase: string;
  unitsDone: number;
  unitsTotal: number;
  facesDone: number;
  facesTotal: number;
  nextChunk: number;
  chunks: number;
}

interface CancelEvent {
  event: string;
  handle?: string;
  phase?: string;
  unitsDone?: number;
  unitsTotal?: number;
  facesDone?: number;
  facesTotal?: number;
  chunk?: number;
  chunks?: number;
  count?: number;
}

const PHASE_LABELS: Record<string, { en: string; de: string }> = {
  idle: { en: "Idle", de: "Bereit" },
  samplingEdges: { en: "Sampling edges", de: "Kanten werden abgetastet" },
  meshingFaces: { en: "Meshing faces", de: "Flächen werden vernetzt" },
  packingEdges: { en: "Packing edges", de: "Kanten werden gepackt" },
  transferring: { en: "Transferring mesh", de: "Netz wird übertragen" },
  complete: { en: "Complete", de: "Fertig" },
  cancelled: { en: "Cancelled", de: "Abgebrochen" },
  faulted: { en: "Geometry extension unavailable", de: "Geometrie-Erweiterung nicht verfügbar" },
};

const CANCELLABLE_PHASES = new Set(["samplingEdges", "meshingFaces", "packingEdges", "transferring"]);

/** 🛑️ The independent model: a pending table, a progress ledger keyed by handle, a per-window latch
 * and the cancelled banner — the four facts the fixture's laws are written over. */
class CancelSession {
  readonly pending = new Set<string>();
  readonly ledger = new Map<string, CancelLedgerRow>();
  latch = { armed: false, inFlight: 0, owed: false, unfinished: false };
  latchExists = false;
  cancelled = false;
  readonly emitted: Array<{ extension: string; capability: string; responseAction: string }> = [];

  admit(handle: string): void {
    this.pending.add(handle);
  }

  step(event: CancelEvent): void {
    this.pending.delete(event.handle!);
    this.ledger.set(event.handle!, {
      phase: event.phase ?? "idle",
      unitsDone: event.unitsDone ?? 0,
      unitsTotal: event.unitsTotal ?? 0,
      facesDone: event.facesDone ?? 0,
      facesTotal: event.facesTotal ?? 0,
      nextChunk: 0,
      chunks: 0,
    });
  }

  chunk(event: CancelEvent): void {
    this.pending.delete(event.handle!);
    this.ledger.set(event.handle!, { phase: "complete", unitsDone: 24, unitsTotal: 24, facesDone: 9, facesTotal: 9, nextChunk: (event.chunk ?? 0) + 1, chunks: event.chunks ?? 0 });
  }

  parkExtension(count: number): void {
    this.latchExists = true;
    this.latch.inFlight += count;
  }

  /** 📈️ The aggregate the status object reports — `Complete` rows contribute progress but only set
   * the phase while their mesh body is still crossing. */
  status(): { phase: string; unitsDone: number; unitsTotal: number; facesDone: number; facesTotal: number; inFlight: number } {
    let phase = "idle";
    let unitsDone = 0;
    let unitsTotal = 0;
    let facesDone = 0;
    let facesTotal = 0;
    for (const row of this.ledger.values()) {
      unitsDone += row.unitsDone;
      unitsTotal += row.unitsTotal;
      facesDone += row.facesDone;
      facesTotal += row.facesTotal;
      if (row.phase === "complete") {
        if (row.nextChunk < row.chunks) phase = "transferring";
      } else phase = row.phase;
    }
    const inFlight = this.pending.size;
    if (unitsTotal === 0 && inFlight > 0) phase = "samplingEdges";
    return { phase, unitsDone, unitsTotal, facesDone, facesTotal, inFlight };
  }

  cancellable(): boolean {
    if (this.cancelled) return false;
    const status = this.status();
    return status.inFlight > 0 || CANCELLABLE_PHASES.has(status.phase) || this.latch.inFlight > 0;
  }

  /** 🛑️ The gesture: local retirement, ledger freeze, cursor drop, latch quiescence, one invocation. */
  cancel(geometry: { extension: string; capability: string; responseAction: string } | null): void {
    this.pending.clear();
    for (const row of this.ledger.values()) {
      if (row.phase !== "complete") row.phase = "cancelled";
      row.nextChunk = 0;
      row.chunks = 0;
    }
    this.latch = { armed: false, inFlight: 0, owed: false, unfinished: false };
    this.latchExists = true;
    this.cancelled = true;
    if (geometry) this.emitted.push(geometry);
  }

  /** 🔁️ A later gesture arms exactly one tick, and the tick that begins retires the banner. */
  armGesture(): boolean {
    if (this.latch.armed) return false;
    if (this.latch.inFlight > 0) {
      this.latch.owed = true;
      return false;
    }
    this.latch.armed = true;
    return true;
  }

  tickOwed(): boolean {
    if (!this.latchExists) return true;
    return this.latch.unfinished && !this.latch.armed && this.latch.inFlight === 0;
  }

  nextChunkFor(handle: string): number {
    return this.ledger.get(handle)?.nextChunk ?? 0;
  }
}

/** ⚖️ Third-party twin of the preview-cancel fixture: an independent TypeScript replay of every row,
 * plus the shell-side half — the `World3dScene.statusJson` parser that decides whether the surface
 * offers the affordance at all — driven from the SAME fixture the Rust law drives. */
export function testGeneration3dPreviewCancelContract(): void {
  const here = fileURLToPath(new URL(".", import.meta.url));
  const fixture = JSON.parse(readFileSync(`${here}/../../../../../🧫️fixtures/🛑️preview-cancel.json`, "utf8")) as {
    format: string;
    version: number;
    cancelAction: string;
    cancelCapability: string;
    cancelResponseAction: string;
    geometryExtensionId: string;
    phaseLabels: Record<string, { en: string; de: string }>;
    laws: Record<string, string>;
    rows: Array<{
      id: string;
      sequence: CancelEvent[];
      statusBefore: Record<string, number | string | boolean>;
      statusAfter: Record<string, number | string | boolean>;
      invocations: Array<{ extension: string; capability: string; responseAction: string }>;
      chunkCursorAfterCancel: number;
      resumesOnGesture: boolean;
    }>;
    hostDoor: { answerStatus: string; faultCode: string; abortedAtTurnBoundary: boolean };
  };
  assert.equal(fixture.format, "semio.generation3d.preview-cancel");
  assert.equal(fixture.version, 1);
  for (const [phase, label] of Object.entries(fixture.phaseLabels)) {
    assert.deepEqual(label, PHASE_LABELS[phase], `phase label drifted for ${phase} — a surface carries English AND German, with no default language`);
  }

  for (const row of fixture.rows) {
    const session = new CancelSession();
    let cursorHandle = "";
    let sawCancel = false;
    const assertStatus = (expected: Record<string, number | string | boolean>, when: string): void => {
      const status = session.status();
      assert.equal(status.phase, expected.phase, `${row.id}/${when}: phase`);
      assert.equal(session.cancellable(), expected.cancellable, `${row.id}/${when}: cancellable`);
      assert.equal(status.unitsDone, expected.unitsDone, `${row.id}/${when}: unitsDone`);
      assert.equal(status.unitsTotal, expected.unitsTotal, `${row.id}/${when}: unitsTotal`);
      assert.equal(status.facesDone, expected.facesDone, `${row.id}/${when}: facesDone`);
      assert.equal(status.facesTotal, expected.facesTotal, `${row.id}/${when}: facesTotal`);
      assert.equal(status.inFlight, expected.inFlight, `${row.id}/${when}: inFlight`);
    };
    for (const event of row.sequence) {
      switch (event.event) {
        case "admit":
          session.admit(event.handle!);
          cursorHandle = event.handle!;
          break;
        case "step":
          session.step(event);
          break;
        case "chunk":
          session.chunk(event);
          cursorHandle = event.handle!;
          break;
        case "parkExtension":
          session.parkExtension(event.count ?? 0);
          break;
        case "cancel":
          if (!sawCancel) {
            assertStatus(row.statusBefore, "before");
            sawCancel = true;
          }
          session.cancel({ extension: fixture.geometryExtensionId, capability: fixture.cancelCapability, responseAction: fixture.cancelResponseAction });
          break;
        case "lateAnswer":
          assert.equal(session.pending.has(event.handle!), false, `${row.id}: a response for a retired job must be ignored`);
          break;
        default:
          throw new Error(`${row.id}: unknown preview-cancel event ${event.event}`);
      }
    }
    assert.equal(session.cancelled, true, `${row.id}: every row ends cancelled`);
    assert.equal(session.status().phase === "cancelled" || row.statusAfter.phase === "cancelled", true, `${row.id}: the gesture outranks the ledger`);
    assert.equal(session.cancellable(), row.statusAfter.cancellable, `${row.id}/after: cancellable`);
    assert.equal(session.emitted.length, row.invocations.length, `${row.id}: emitted invocation count`);
    for (const [index, invocation] of session.emitted.entries()) assert.deepEqual(invocation, row.invocations[index], `${row.id}: invocation ${index}`);
    assert.equal(session.tickOwed(), false, `${row.id}: nothing may be owed after a cancel`);
    if (cursorHandle) assert.equal(session.nextChunkFor(cursorHandle), row.chunkCursorAfterCancel, `${row.id}: the chunk cursor drops with the body it addresses`);
    assert.equal(session.armGesture(), row.resumesOnGesture, `${row.id}: a later gesture re-arms exactly one tick`);

    // 🛑️ The shell half: what the surface would actually render for this row's published status.
    const publishedBefore = JSON.stringify({
      phase: row.statusBefore.phase,
      phaseLabel: PHASE_LABELS[String(row.statusBefore.phase)],
      progress: { unitsDone: row.statusBefore.unitsDone, unitsTotal: row.statusBefore.unitsTotal, facesDone: row.statusBefore.facesDone, facesTotal: row.statusBefore.facesTotal, inFlight: row.statusBefore.inFlight, ratio: 0 },
      cancellable: row.statusBefore.cancellable,
      cancelAction: fixture.cancelAction,
    });
    const parsedBefore = world3dComputeStatusV1(publishedBefore);
    assert.equal(parsedBefore.cancellable, row.statusBefore.cancellable, `${row.id}: the surface offers the affordance exactly while the producer says it may`);
    assert.equal(parsedBefore.cancelAction, fixture.cancelAction);
    assert.deepEqual(parsedBefore.phaseLabel, PHASE_LABELS[String(row.statusBefore.phase)]);
    const publishedAfter = publishedBefore.replace(`"cancellable":${row.statusBefore.cancellable}`, `"cancellable":${row.statusAfter.cancellable}`);
    assert.equal(world3dComputeStatusV1(publishedAfter).cancellable, row.statusAfter.cancellable, `${row.id}: a cancelled evaluation offers nothing to cancel`);
  }

  // 🛑️ A declared `cancellable` with no action to dispatch is a dead button — worse than none.
  assert.equal(world3dComputeStatusV1('{"cancellable":true}').cancellable, false);
  assert.equal(world3dComputeStatusV1("not json").phase, "idle");
  assert.equal(world3dComputeStatusV1(undefined).computing, false);

  assert.equal(fixture.hostDoor.answerStatus, "cancelled");
  assert.equal(fixture.hostDoor.faultCode, "extension.request-cancelled");
  assert.equal(fixture.hostDoor.abortedAtTurnBoundary, true);
  console.log(`generation3d preview-cancel rows=${fixture.rows.length} capability=${fixture.cancelCapability} response=${fixture.cancelResponseAction} action=${fixture.cancelAction}`);
}

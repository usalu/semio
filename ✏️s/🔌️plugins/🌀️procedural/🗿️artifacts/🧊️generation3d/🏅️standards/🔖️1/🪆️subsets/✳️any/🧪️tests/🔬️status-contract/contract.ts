import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { world3dComputeStatusV1 } from "@semio-tech/framework";

/** 📈️ Third-party twin of the PREVIEW-WINDOW STATUS CONTRACT (`🧫️fixtures/🛑️preview-cancel.json`,
 * `statusContract`) — an independent TypeScript model of the status object every generation3d World3d
 * preview window publishes, driven from the SAME fixture the Rust laws drive. Rust replays each state
 * against the real `FlowEvalSession` and the real surface-neutral projection; this file rebuilds the
 * expected object from the fixture's own prose and then pushes it through the SHELL's total parser,
 * which is what decides whether the surface offers the cancel affordance at all.
 *
 * 🪟️ Surface-neutral on purpose: the contract is a property of the WINDOW, not of a surface, and all
 * three windows — the editor's edit preview, the editor's generate preview and the viewer's read-only
 * preview — must answer identically (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */

interface RatioSpec {
  done: number;
  total: number;
}

interface StatusEvent {
  event: string;
  handle?: string;
  phase?: string;
  unitsDone?: number;
  unitsTotal?: number;
  facesDone?: number;
  facesTotal?: number;
  faultCode?: string;
  faultMessage?: string;
}

interface ExpectedStatus {
  phase: string;
  cancellable: boolean;
  unitsDone: number;
  unitsTotal: number;
  facesDone: number;
  facesTotal: number;
  inFlight: number;
  ratio: RatioSpec;
}

interface StatusState {
  id: string;
  addressable: boolean;
  sequence: StatusEvent[];
  status: ExpectedStatus;
  fault: { code: string; extensionId: string; capability?: string } | null;
}

interface StatusContract {
  objectKeys: string[];
  progressKeys: string[];
  debugKeys: string[];
  evaluateFaultCode: string;
  addressMissCode: string;
  surfaces: Array<{ surface: string; windowKindId: string; cancelAction: string; declaresCancelCommand: boolean }>;
  states: StatusState[];
}

const CANCELLABLE_PHASES = new Set(["samplingEdges", "meshingFaces", "packingEdges", "transferring"]);

/** 📈️ The independent model: a pending table, a per-handle progress ledger, the cancelled banner and
 * the retained evaluate fault — the four facts the projection is written over. */
class StatusSession {
  readonly pending = new Set<string>();
  readonly ledger = new Map<string, { phase: string; unitsDone: number; unitsTotal: number; facesDone: number; facesTotal: number }>();
  cancelled = false;
  evaluateFault: { faultCode: string; faultMessage: string } | null = null;

  apply(event: StatusEvent): void {
    switch (event.event) {
      case "admit":
        this.pending.add(event.handle!);
        break;
      case "step":
        this.pending.delete(event.handle!);
        this.ledger.set(event.handle!, {
          phase: event.phase ?? "idle",
          unitsDone: event.unitsDone ?? 0,
          unitsTotal: event.unitsTotal ?? 0,
          facesDone: event.facesDone ?? 0,
          facesTotal: event.facesTotal ?? 0,
        });
        break;
      case "evaluateFault":
        this.evaluateFault = { faultCode: event.faultCode ?? "", faultMessage: event.faultMessage ?? "" };
        break;
      case "cancel":
        this.pending.clear();
        for (const row of this.ledger.values()) if (row.phase !== "complete") row.phase = "cancelled";
        this.cancelled = true;
        break;
      default:
        throw new Error(`unknown status-contract event ${event.event}`);
    }
  }

  ledgerStatus(): { phase: string; unitsDone: number; unitsTotal: number; facesDone: number; facesTotal: number; inFlight: number } {
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
      if (row.phase !== "complete") phase = row.phase;
    }
    const inFlight = this.pending.size;
    if (unitsTotal === 0 && inFlight > 0) phase = "samplingEdges";
    return { phase, unitsDone, unitsTotal, facesDone, facesTotal, inFlight };
  }

  /** 📈️ The projection's precedence: an addressing miss outranks everything, then a live evaluate
   * fault, then the explicit gesture, then the ledger. */
  phase(addressable: boolean): string {
    if (!addressable || this.evaluateFault) return "faulted";
    if (this.cancelled) return "cancelled";
    return this.ledgerStatus().phase;
  }

  cancellable(addressable: boolean): boolean {
    if (!addressable || this.evaluateFault || this.cancelled) return false;
    const ledger = this.ledgerStatus();
    return ledger.inFlight > 0 || CANCELLABLE_PHASES.has(ledger.phase);
  }

  /** 📈️ The fixture's own `ratioLaw`. */
  ratio(spec: RatioSpec, inFlight: number): number {
    if (spec.total === 0) return inFlight === 0 ? 1 : 0;
    return spec.done / spec.total;
  }
}

export function testGeneration3dPreviewStatusContract(): void {
  const here = fileURLToPath(new URL(".", import.meta.url));
  const fixture = JSON.parse(readFileSync(`${here}/../../🧫️fixtures/🛑️preview-cancel.json`, "utf8")) as {
    format: string;
    version: number;
    cancelAction: string;
    phaseLabels: Record<string, { en: string; de: string }>;
    statusContract: StatusContract;
  };
  assert.equal(fixture.format, "semio.generation3d.preview-cancel");
  assert.equal(fixture.version, 1);
  const contract = fixture.statusContract;

  // 🪟️ Three windows, one schema, one verb.
  assert.deepEqual(
    contract.surfaces.map((row) => row.surface).sort(),
    ["editor", "generate", "viewer"],
    "all three World3d preview windows must be named — the contract is a property of the window, not the surface",
  );
  for (const surface of contract.surfaces) {
    assert.equal(surface.cancelAction, fixture.cancelAction, `${surface.surface}: every surface names the same cancel verb`);
    assert.equal(surface.declaresCancelCommand, true, `${surface.surface}: a surface that publishes cancelAction must declare that command or the gesture is dropped`);
    assert.ok(surface.windowKindId.length > 0, `${surface.surface}: a window kind id is the address the status is published under`);
  }
  assert.equal(new Set(contract.surfaces.map((row) => row.windowKindId)).size, contract.surfaces.length, "each preview window kind appears once");

  for (const state of contract.states) {
    const session = new StatusSession();
    for (const event of state.sequence) session.apply(event);
    const ledger = session.ledgerStatus();
    const phase = session.phase(state.addressable);
    const cancellable = session.cancellable(state.addressable);
    assert.equal(phase, state.status.phase, `${state.id}: phase`);
    assert.equal(cancellable, state.status.cancellable, `${state.id}: cancellable`);
    assert.equal(ledger.unitsDone, state.status.unitsDone, `${state.id}: unitsDone`);
    assert.equal(ledger.unitsTotal, state.status.unitsTotal, `${state.id}: unitsTotal`);
    assert.equal(ledger.facesDone, state.status.facesDone, `${state.id}: facesDone`);
    assert.equal(ledger.facesTotal, state.status.facesTotal, `${state.id}: facesTotal`);
    assert.equal(state.status.cancellable ? ledger.inFlight : state.status.inFlight, state.status.inFlight, `${state.id}: inFlight`);
    const label = fixture.phaseLabels[phase];
    assert.ok(label, `${state.id}: the fixture must declare a label for phase ${phase}`);

    // 🛑️ The shell half: what a surface publishing this state would actually offer the user, read
    // through the one total parser `World3dHost` uses.
    const published = JSON.stringify({
      phase,
      phaseLabel: label,
      progress: {
        unitsDone: state.status.unitsDone,
        unitsTotal: state.status.unitsTotal,
        facesDone: state.status.facesDone,
        facesTotal: state.status.facesTotal,
        inFlight: state.status.inFlight,
        ratio: session.ratio(state.status.ratio, state.status.inFlight),
      },
      cancellable,
      cancelAction: fixture.cancelAction,
      ...(state.fault ? { fault: state.fault } : {}),
    });
    const parsed = world3dComputeStatusV1(published);
    assert.equal(parsed.phase, state.status.phase, `${state.id}: the shell reads back the producer's phase`);
    assert.equal(parsed.cancellable, state.status.cancellable, `${state.id}: the shell offers the affordance exactly while the producer says it may`);
    assert.equal(parsed.cancelAction, state.status.cancellable ? fixture.cancelAction : parsed.cancelAction, `${state.id}: cancelAction`);
    assert.deepEqual(parsed.phaseLabel, label, `${state.id}: a surface carries English AND German, with no default language`);
    if (state.fault) {
      const expectedCode = state.addressable ? contract.evaluateFaultCode : contract.addressMissCode;
      assert.equal(state.fault.code, expectedCode, `${state.id}: an unaddressable kernel and a refusing one are different faults`);
      assert.equal(state.status.cancellable, false, `${state.id}: a faulted preview offers nothing to cancel`);
    }
  }

  // 📈️ The declared shape itself — a key dropped from the projection is the defect this guards.
  for (const key of ["phase", "phaseLabel", "progress", "cancellable", "cancelAction", "debug"]) {
    assert.ok(contract.objectKeys.includes(key), `the contract must declare ${key}`);
  }
  for (const key of ["unitsDone", "unitsTotal", "facesDone", "facesTotal", "inFlight", "ratio"]) {
    assert.ok(contract.progressKeys.includes(key), `progress must declare ${key}`);
  }
  for (const key of ["evalLen", "meshesLen", "instancesLen", "evalHead"]) {
    assert.ok(contract.debugKeys.includes(key), `debug must declare ${key}`);
  }

  // 🛑️ A declared `cancellable` with no action to dispatch is a dead button — worse than none.
  assert.equal(world3dComputeStatusV1('{"cancellable":true}').cancellable, false);
  assert.equal(world3dComputeStatusV1("not json").phase, "idle");
  assert.equal(world3dComputeStatusV1(undefined).computing, false);

  console.log(
    `generation3d preview-status surfaces=${contract.surfaces.map((row) => `${row.surface}:${row.windowKindId}`).join(" ")} states=${contract.states.map((state) => state.id).join(",")} cancelAction=${fixture.cancelAction}`,
  );
}

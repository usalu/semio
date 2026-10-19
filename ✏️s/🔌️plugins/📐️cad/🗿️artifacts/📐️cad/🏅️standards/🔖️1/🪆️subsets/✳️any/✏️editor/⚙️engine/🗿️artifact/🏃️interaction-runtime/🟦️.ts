/** 🏃️ CAD editor artifact: the pointer context helpers and the interaction runtime state machine with its factory. */
import type { Vec3 } from "@semio-tech/framework/3d";
import { ModelEntityKind } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { InteractionLengthEntrySpec, InteractionScalarEntrySpec, InteractionSpec, StateDefSpec, compileInteraction, getActiveSelectionSpec, isFinalInteractionState, mergeInteractionCallOutputs } from "@semio-tech/s-spatial-kernel-ts/geometry/interaction-spec";
import { Expr, ExprEnv, evalExpr, evalGuard, readPathTarget, writePathTarget } from "@semio-tech/s-spatial-kernel-ts/geometry/expressions";
import { InteractionEvent, SelectionEvent, SelectionSpec, SelectionTarget, expandSelectionTargetsForAccept, selectionEventMatches } from "@semio-tech/s-spatial-kernel-ts/geometry/selection";
import { ensureTypologyObjectFromCreateDiff, typologyConstructCommitActionForMode, typologyConstructKitByInteraction, typologyIdForInteractionCommit } from "../../🧬️typology/🟦️.ts";
import { EMPTY_MODEL_DIFF, ModelDiff, SpatialKernel, SpatialPreviewKernel, applyModelDiff, isEmptyModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { resolveDisplay } from "../../🎬️actions/🟦️.ts";
import { ActionContextPatch, ActionRegistry } from "../../🎬️actions/🧬️action-contract/🟦️.ts";
import { ConstructQueryResult, modelDefinitionActionRegistry, runRegisteredAction } from "../../🎬️actions/🏃️action-execution/🟦️.ts";
import { interactionRecordsDocumentHistory } from "../../🎬️actions/🧲️selection-targets/🟦️.ts";
import { InteractionChildCallSpec, StateEngine, StateEngineSendResult, interactionLengthEntryForState, interactionScalarEntryForState, mergeInteractionSpatial, pureTsStateEngineProvider, readInteractionContextVec3, scalarEntryAxisBase, writeInteractionContextVec3 } from "../../🎬️actions/🎰️interaction-state/🟦️.ts";
import type { Diagnostic, InteractionMessage, InteractionResponse, InteractionRuntimeOptions, InteractionSnapshot, SpatialComputeMode } from "../📨️interaction-model/🟦️.ts";
import { type InteractionRegistry, loadSpatialInteraction } from "../📚️interaction-catalog/🟦️.ts";

// #region 📦️📄️document
// #region 📜️Interaction
/** 📞️ Resolves an interaction spec for `interaction.call` (registry first, then shipped assets). */
export function resolveInteractionSpecForCall(interactionId: string, registry?: InteractionRegistry): InteractionSpec | null {
  return registry?.get(interactionId) ?? loadSpatialInteraction(interactionId);
}

export function isInteractionSessionActive(spec: InteractionSpec, state: string): boolean {
  return !isFinalInteractionState(spec, state);
}

// #region 📏️PointerContext
const CURSOR_RAW_CTX = "__cursorRaw";
const LENGTH_LOCK_CTX = "__lengthLock";
const HEIGHT_LOCK_CTX = "__heightLock";
const SCALAR_AXIS_T_CTX = "__scalarAxisT";
const DEFAULT_SCALAR_AXIS: Vec3 = [0, 0, 1];

function lookupGuard(spec: InteractionSpec, name: string): Expr | undefined {
  return spec.guards?.find((g) => g.name === name)?.expr;
}

function findState(spec: InteractionSpec, name: string): StateDefSpec | undefined {
  return spec.machine.states.find((s) => s.name === name);
}

function applyActionPatchToContext(ctx: Record<string, unknown>, patch: ActionContextPatch | undefined): void {
  if (!patch) return;
  if (patch.set) Object.assign(ctx, patch.set);
  if (patch.del) for (const k of patch.del) delete ctx[k];
}

function positiveLengthLock(ctx: Record<string, unknown>): number | null {
  const lock = ctx[LENGTH_LOCK_CTX];
  return typeof lock === "number" && Number.isFinite(lock) && lock > 0 ? lock : null;
}

function positiveHeightLock(ctx: Record<string, unknown>): number | null {
  const lock = ctx[HEIGHT_LOCK_CTX];
  return typeof lock === "number" && Number.isFinite(lock) && lock > 0 ? lock : null;
}

function scalarEntryAxis(entry: InteractionScalarEntrySpec): Vec3 {
  const a = entry.axis;
  if (a && a.length === 3) return [a[0], a[1], a[2]];
  return DEFAULT_SCALAR_AXIS;
}

function scalarHeightFromAxisT(t: number): number {
  return Math.max(0.01, Math.abs(t));
}

function lengthEntryRawPoint(ctx: Record<string, unknown>, entry: InteractionLengthEntrySpec): Vec3 | null {
  const raw = readInteractionContextVec3(ctx, CURSOR_RAW_CTX) ?? readInteractionContextVec3(ctx, entry.field);
  if (raw) return raw;
  const anchor = readInteractionContextVec3(ctx, entry.anchor);
  if (!anchor) return null;
  return [anchor[0] + 1, anchor[1], anchor[2]];
}

function applyLengthEntryToContext(ctx: Record<string, unknown>, entry: InteractionLengthEntrySpec, raw: Vec3, lock: number, preview: SpatialPreviewKernel): void {
  const anchor = readInteractionContextVec3(ctx, entry.anchor);
  if (!anchor) return;
  const clamped = preview.clampPointAlongDirection(anchor, raw, lock);
  writeInteractionContextVec3(ctx, entry.field, clamped);
  if (entry.field === "cursor" && "prevPoint" in ctx) ctx.prevPoint = anchor;
}

function clearInteractionLengthEntryFields(ctx: Record<string, unknown>): void {
  delete ctx[LENGTH_LOCK_CTX];
  delete ctx[HEIGHT_LOCK_CTX];
  delete ctx[SCALAR_AXIS_T_CTX];
  delete ctx[CURSOR_RAW_CTX];
}

// #endregion 📏️PointerContext
/** 📜️ Headless + interactive interaction controller (`send`, `commit`, `undo`). */
export class InteractionRuntime {
  private readonly sm: StateEngine;
  private readonly actions: ActionRegistry;
  private revision = 0;
  private readonly listeners = new Set<() => void>();
  private readonly snapUndoStack: { state: string; context: string }[] = [];
  private readonly snapRedoStack: { state: string; context: string }[] = [];
  private snapshotCache: InteractionSnapshot | null = null;
  private lastResponse: InteractionResponse | null = null;
  private readonly pendingSnapshotInfos: InteractionMessage[] = [];
  private child: InteractionRuntime | null = null;
  private pausedHost: InteractionChildCallSpec | null = null;

  constructor(
    private readonly spec: InteractionSpec,
    private readonly opts: InteractionRuntimeOptions,
  ) {
    this.sm = (opts.stateEngine ?? pureTsStateEngineProvider).create(spec);
    this.actions = opts.actions ?? modelDefinitionActionRegistry();
  }

  /** 🔌️ Precise BREP kernel wired into this runtime (tessellation, commit, derived views). */
  kernel(): SpatialKernel {
    return this.opts.kernel;
  }

  /** ⚡️ `fast` uses `previewKernel`; `precise` uses the BREP kernel for preview math too. */
  computeMode(): SpatialComputeMode {
    return this.opts.mode ?? "precise";
  }

  /** ⚡️ Active preview kernel for the current `mode` (fast renderer vs precise brep). */
  previewKernel(): SpatialPreviewKernel {
    const mode = this.computeMode();
    if (mode === "fast") {
      const pk = this.opts.previewKernel;
      if (!pk) throw new Error("InteractionRuntimeOptions.previewKernel is required when mode is fast");
      return pk;
    }
    return this.opts.kernel;
  }

  private exprEnv(extra?: Partial<ExprEnv>): ExprEnv {
    return {
      context: this.sm.getContext(),
      preview: this.previewKernel(),
      ...extra,
    };
  }

  private cloneCtx(c: Record<string, unknown>): Record<string, unknown> {
    return JSON.parse(JSON.stringify(c)) as Record<string, unknown>;
  }

  private inActiveInteraction(): boolean {
    if (this.child) return true;
    return isInteractionSessionActive(this.spec, this.sm.getState());
  }

  private seedChildContext(child: InteractionRuntime, inputs?: Record<string, Expr>): void {
    if (!inputs) return;
    const ctx = child.sm.getContext();
    const env: ExprEnv = this.exprEnv({ context: ctx, event: { kind: "start" } });
    for (const [key, ex] of Object.entries(inputs)) {
      ctx[key] = evalExpr(ex, env);
    }
  }

  private async startChildCall(call: InteractionChildCallSpec): Promise<void> {
    const childSpec = resolveInteractionSpecForCall(call.interactionId, this.opts.interactions);
    if (!childSpec) {
      this.pendingSnapshotInfos.push({
        code: "interaction.callMissing",
        message: `Nested interaction not found: ${call.interactionId}`,
      });
      return;
    }
    this.pausedHost = call;
    this.child = createInteractionRuntime(childSpec, {
      ...this.opts,
      interactions: this.opts.interactions,
    });
    this.seedChildContext(this.child, call.inputs);
    await this.child.send({ kind: "start" });
    await this.maybeCompleteChild();
  }

  private resumeHostAfterChild(success: boolean): void {
    const call = this.pausedHost;
    const child = this.child;
    this.child = null;
    this.pausedHost = null;
    if (!call || !child) return;
    if (success) {
      mergeInteractionCallOutputs(this.sm.getContext(), child.sm.getContext(), call.outputs);
      this.sm.restore(call.resumeTarget, this.cloneCtx(this.sm.getContext()));
      return;
    }
    this.sm.restore(call.rollback.state, this.cloneCtx(call.rollback.context));
  }

  private abortChildCall(): void {
    if (!this.child || !this.pausedHost) return;
    this.child.cancel();
    const rb = this.pausedHost.rollback;
    this.child = null;
    this.pausedHost = null;
    this.sm.restore(rb.state, this.cloneCtx(rb.context));
  }

  private async settleChildSession(): Promise<boolean> {
    const child = this.child;
    if (!child) return false;
    const snap = child.getSnapshot();
    if (!isFinalInteractionState(child.spec, snap.state)) return false;
    if (snap.capabilities.canCommit && snap.lastResponse?.ok !== true) {
      await child.commit();
    }
    return true;
  }

  private restoreUndoSnapshotAfterFailedFinalCommit(): void {
    const snap = this.snapUndoStack.pop();
    if (!snap) return;
    this.sm.restore(snap.state, JSON.parse(snap.context) as Record<string, unknown>);
    this.snapRedoStack.length = 0;
  }

  private async continueHostSessionAfterEngineSend(): Promise<void> {
    if (isFinalInteractionState(this.spec, this.sm.getState())) {
      const res = await this.runCommit(false);
      if (!res.ok) this.restoreUndoSnapshotAfterFailedFinalCommit();
      return;
    }
    if (this.canCommit()) {
      await this.runCommit(true);
      return;
    }
    this.emit();
  }

  private async maybeCompleteChild(): Promise<void> {
    if (!this.child || !this.pausedHost) return;
    if (!(await this.settleChildSession())) return;
    const ok = this.child.getSnapshot().lastResponse?.ok !== false;
    this.resumeHostAfterChild(ok);
    if (!ok) {
      this.emit();
      return;
    }
    await this.continueHostSessionAfterEngineSend();
  }

  private async handleEngineSendResult(r: StateEngineSendResult, beforeState: string, beforeCtx: Record<string, unknown>): Promise<void> {
    if (!r.ok) return;
    if (r.childCall) {
      if (!r.transient) {
        this.snapUndoStack.push({ state: beforeState, context: JSON.stringify(beforeCtx) });
        this.snapRedoStack.length = 0;
        clearInteractionLengthEntryFields(this.sm.getContext());
      }
      await this.startChildCall(r.childCall);
      this.emit();
      return;
    }
    if (!r.transient) {
      this.snapUndoStack.push({ state: beforeState, context: JSON.stringify(beforeCtx) });
      this.snapRedoStack.length = 0;
      if (this.sm.getState() !== beforeState) clearInteractionLengthEntryFields(this.sm.getContext());
    }
    await this.continueHostSessionAfterEngineSend();
  }

  private preprocessLengthEntryEvent(event: InteractionEvent): InteractionEvent {
    if (event.kind !== "pointer.move" && event.kind !== "pointer.down") return event;
    const entry = interactionLengthEntryForState(this.spec, this.sm.getState());
    if (!entry) return event;
    const point = event.point;
    if (!Array.isArray(point) || point.length < 3) return event;
    const raw: Vec3 = [Number(point[0]), Number(point[1]), Number(point[2])];
    const ctx = this.sm.getContext();
    ctx[CURSOR_RAW_CTX] = raw;
    const lock = positiveLengthLock(ctx);
    if (lock == null) return event;
    const anchor = readInteractionContextVec3(ctx, entry.anchor);
    if (!anchor) return event;
    return { ...event, point: this.previewKernel().clampPointAlongDirection(anchor, raw, lock) };
  }

  private handleSetLength(event: InteractionEvent): void {
    const entry = interactionLengthEntryForState(this.spec, this.sm.getState());
    const ctx = this.sm.getContext();
    const rawVal = event.value;
    const lock = typeof rawVal === "number" && Number.isFinite(rawVal) && rawVal > 0 ? rawVal : null;
    if (lock == null) {
      ctx[LENGTH_LOCK_CTX] = null;
      this.emit();
      return;
    }
    ctx[LENGTH_LOCK_CTX] = lock;
    if (!entry) {
      this.emit();
      return;
    }
    const raw = lengthEntryRawPoint(ctx, entry);
    if (raw) {
      ctx[CURSOR_RAW_CTX] = raw;
      applyLengthEntryToContext(ctx, entry, raw, lock, this.previewKernel());
    }
    this.emit();
  }

  private applyScalarAxisPointer(event: InteractionEvent): boolean {
    const entry = interactionScalarEntryForState(this.spec, this.sm.getState());
    if (!entry?.axisAnchor || (event.kind !== "pointer.move" && event.kind !== "pointer.down")) return false;
    const point = event.point;
    if (!Array.isArray(point) || point.length < 3) return false;
    const raw: Vec3 = [Number(point[0]), Number(point[1]), Number(point[2])];
    const ctx = this.sm.getContext();
    ctx[CURSOR_RAW_CTX] = raw;
    const base = scalarEntryAxisBase(ctx, entry);
    if (!base) return false;
    const axis = scalarEntryAxis(entry);
    const { t } = this.previewKernel().projectPointOnScalarAxis(base, axis, raw);
    ctx[SCALAR_AXIS_T_CTX] = t;
    const lock = positiveHeightLock(ctx);
    ctx[entry.field] = lock ?? scalarHeightFromAxisT(t);
    return true;
  }

  private handleScalarEntry(event: InteractionEvent): void {
    const entry = interactionScalarEntryForState(this.spec, this.sm.getState());
    if (!entry || entry.event !== event.kind) return;
    const ctx = this.sm.getContext();
    const rawVal = event.value;
    if (typeof rawVal === "number" && Number.isFinite(rawVal) && rawVal > 0) {
      ctx[HEIGHT_LOCK_CTX] = rawVal;
      ctx[entry.field] = rawVal;
    } else if (rawVal == null) {
      delete ctx[HEIGHT_LOCK_CTX];
      delete ctx[entry.field];
    }
    this.emit();
  }

  private canCommit(): boolean {
    const st = this.sm.getState();
    if (isFinalInteractionState(this.spec, st)) return false;
    return this.canCommitFromState(st);
  }

  private canCommitFromState(st: string): boolean {
    const allowed = this.spec.commit.fromStates ?? ["ready"];
    if (!allowed.includes(st)) return false;
    const w = this.spec.commit.when;
    if (w) {
      const g = lookupGuard(this.spec, w);
      if (!g) return false;
      return evalGuard(g, this.exprEnv());
    }
    return true;
  }

  private selectionEventFromStart(event: InteractionEvent, spec: SelectionSpec): SelectionEvent | null {
    const rawTargets = Array.isArray(event.targets) ? (event.targets as readonly SelectionTarget[]) : [];
    const selected = expandSelectionTargetsForAccept(this.opts.document.model, spec, rawTargets);
    if (selected.length === 0) return null;
    return { kind: "selection.changed", targets: spec.multiple ? selected : selected.slice(0, 1) };
  }

  private stateHasEvent(state: string, eventKind: string): boolean {
    return Boolean(findState(this.spec, state)?.on?.some((handler) => handler.event === eventKind));
  }

  private async consumeStartSelection(event: InteractionEvent): Promise<void> {
    const stateBeforeSelection = this.sm.getState();
    const sel = getActiveSelectionSpec(this.spec, stateBeforeSelection);
    if (!sel) return;
    const selectionEvent = this.selectionEventFromStart(event, sel);
    if (!selectionEvent || !selectionEventMatches(sel, selectionEvent)) return;
    const beforeCtx = this.cloneCtx(this.sm.getContext());
    const r = await this.sm.send(selectionEvent, this.opts.kernel, this.opts.document.model, this.actions, this.previewKernel(), this.opts.activeModelDefinitionId ?? null);
    if (!r.ok) return;
    if (!r.transient) this.snapUndoStack.push({ state: stateBeforeSelection, context: JSON.stringify(beforeCtx) });
    const stateAfterSelection = this.sm.getState();
    if (stateAfterSelection === stateBeforeSelection && this.stateHasEvent(stateAfterSelection, "confirm")) {
      const beforeConfirmCtx = this.cloneCtx(this.sm.getContext());
      const cr = await this.sm.send({ kind: "confirm" }, this.opts.kernel, this.opts.document.model, this.actions, this.previewKernel(), this.opts.activeModelDefinitionId ?? null);
      if (cr.ok && !cr.transient) this.snapUndoStack.push({ state: stateAfterSelection, context: JSON.stringify(beforeConfirmCtx) });
    }
  }

  /** 🧭️ Accepted geometry entity kinds for the active machine state (`[]` when none). */
  listActiveSelectionAccept(): readonly ModelEntityKind[] {
    if (this.child) return this.child.listActiveSelectionAccept();
    return getActiveSelectionSpec(this.spec, this.sm.getState())?.accept ?? [];
  }

  /** 🔍️ Executes a `construct` script via `opts.query` (host registers `@semio-tech/cad-js/query`). */
  async query(text: string): Promise<ConstructQueryResult> {
    if (this.child) return this.child.query(text);
    const runner = this.opts.query;
    if (!runner) throw new Error("InteractionRuntime.query requires InteractionRuntimeOptions.query");
    return runner(text, {
      model: this.opts.document.model,
      kernel: this.opts.kernel,
      actions: this.actions,
      activeModelDefinitionId: this.opts.activeModelDefinitionId ?? null,
    });
  }

  getSnapshot(): InteractionSnapshot {
    if (this.child) {
      const childSnap = this.child.getSnapshot();
      if (childSnap.nested) return childSnap;
      return {
        ...childSnap,
        nested: {
          hostInteractionId: this.spec.id,
          hostState: this.sm.getState(),
          hostContext: this.cloneCtx(this.sm.getContext()),
        },
      };
    }
    if (this.snapshotCache) return this.snapshotCache;
    const ctx = this.sm.getContext();
    const st = this.sm.getState();
    const display = resolveDisplay(this.spec, st, ctx, this.previewKernel(), this.opts.document.model);
    const spatialInteraction = mergeInteractionSpatial(this.spec);
    const flushed = this.pendingSnapshotInfos.splice(0, this.pendingSnapshotInfos.length);
    const infoDiags: Diagnostic[] = flushed.map((m) => ({ severity: "info" as const, code: m.code, message: m.message }));
    const hist = this.opts.history;
    const active = this.inActiveInteraction();
    const pristineInitial = st === this.spec.machine.initial && this.snapUndoStack.length === 0;
    const canUndo = this.snapUndoStack.length > 0 || ((!active || pristineInitial) && Boolean(hist?.peekUndo()));
    const canRedo = this.snapRedoStack.length > 0 || ((!active || pristineInitial) && Boolean(hist?.peekRedo()));
    this.snapshotCache = {
      interactionId: this.spec.id,
      state: st,
      revision: this.revision,
      context: this.cloneCtx(ctx),
      display,
      spatialInteraction,
      capabilities: {
        canCommit: this.canCommit(),
        canCancel: !isFinalInteractionState(this.spec, this.sm.getState()) && (this.sm.getState() !== this.spec.machine.initial || this.snapUndoStack.length > 0),
        canUndo,
        canRedo,
      },
      diagnostics: infoDiags,
      lastResponse: this.lastResponse,
    };
    return this.snapshotCache;
  }

  subscribe(fn: () => void): () => void {
    this.listeners.add(fn);
    return () => this.listeners.delete(fn);
  }

  private emit(): void {
    this.revision += 1;
    this.snapshotCache = null;
    for (const l of this.listeners) l();
  }

  /** 📜️ Merges `start.targets` when `compileInteraction` began in the post-start state without running transition effects. */
  private applyInstantStartPayload(event: InteractionEvent): void {
    const raw = event.targets;
    if (!Array.isArray(raw) || raw.length === 0) return;
    this.sm.getContext().seedTargets = raw;
  }

  /** 📜️ Dispatches a typed interaction event through the statechart + optional kernel queries. */
  async send(event: InteractionEvent): Promise<void> {
    if (this.child) {
      await this.child.send(event);
      await this.maybeCompleteChild();
      this.emit();
      return;
    }
    if (event.kind === "start") {
      if (this.stateHasEvent(this.sm.getState(), "start")) {
        const beforeState = this.sm.getState();
        const beforeCtx = this.cloneCtx(this.sm.getContext());
        const r = await this.sm.send(event, this.opts.kernel, this.opts.document.model, this.actions, this.previewKernel(), this.opts.activeModelDefinitionId ?? null);
        if (!r.ok) return;
        if (r.childCall) {
          await this.handleEngineSendResult(r, beforeState, beforeCtx);
          return;
        }
        if (!r.transient) {
          this.snapUndoStack.push({ state: beforeState, context: JSON.stringify(beforeCtx) });
          this.snapRedoStack.length = 0;
        }
      }
      await this.consumeStartSelection(event);
      if (isFinalInteractionState(this.spec, this.sm.getState())) {
        this.applyInstantStartPayload(event);
        await this.runCommit(false);
        return;
      }
      if (this.canCommit()) {
        this.applyInstantStartPayload(event);
        await this.runCommit(true);
        return;
      }
      this.emit();
      return;
    }
    if (event.kind === "set.length") {
      this.handleSetLength(event);
      return;
    }
    const scalarEntry = interactionScalarEntryForState(this.spec, this.sm.getState());
    if (scalarEntry && event.kind === scalarEntry.event) {
      this.handleScalarEntry(event);
      return;
    }
    if (this.applyScalarAxisPointer(event)) {
      this.emit();
      return;
    }
    if (event.kind === "selection.changed") {
      const sel = getActiveSelectionSpec(this.spec, this.sm.getState());
      const sev = event as SelectionEvent;
      if (sel && !selectionEventMatches(sel, sev)) {
        this.pendingSnapshotInfos.push({
          code: "selection.filtered",
          message: "Selection did not match this state's accept list.",
        });
        this.emit();
        return;
      }
    }
    const beforeState = this.sm.getState();
    const beforeCtx = this.cloneCtx(this.sm.getContext());
    const routed = this.preprocessLengthEntryEvent(event);
    const r = await this.sm.send(routed, this.opts.kernel, this.opts.document.model, this.actions, this.previewKernel(), this.opts.activeModelDefinitionId ?? null);
    await this.handleEngineSendResult(r, beforeState, beforeCtx);
  }

  undo(): void {
    if (this.child) {
      this.child.undo();
      this.emit();
      return;
    }
    if (this.inActiveInteraction() && this.snapUndoStack.length > 0) {
      const snap = this.snapUndoStack.pop();
      if (!snap) return;
      const curState = this.sm.getState();
      const curCtx = JSON.stringify(this.cloneCtx(this.sm.getContext()));
      this.snapRedoStack.push({ state: curState, context: curCtx });
      const o = JSON.parse(snap.context) as Record<string, unknown>;
      this.sm.restore(snap.state, o);
      this.emit();
      return;
    }
    if (this.snapUndoStack.length > 0) {
      const snap = this.snapUndoStack.pop();
      if (!snap) return;
      const curState = this.sm.getState();
      const curCtx = JSON.stringify(this.cloneCtx(this.sm.getContext()));
      this.snapRedoStack.push({ state: curState, context: curCtx });
      const o = JSON.parse(snap.context) as Record<string, unknown>;
      this.sm.restore(snap.state, o);
      this.emit();
      return;
    }
    const h = this.opts.history;
    if (h) h.undo(this.opts.document);
    this.emit();
  }

  redo(): void {
    if (this.child) {
      this.child.redo();
      this.emit();
      return;
    }
    if (this.inActiveInteraction() && this.snapRedoStack.length > 0) {
      const snap = this.snapRedoStack.pop();
      if (!snap) return;
      const curState = this.sm.getState();
      const curCtx = JSON.stringify(this.cloneCtx(this.sm.getContext()));
      this.snapUndoStack.push({ state: curState, context: curCtx });
      const o = JSON.parse(snap.context) as Record<string, unknown>;
      this.sm.restore(snap.state, o);
      this.emit();
      return;
    }
    if (this.snapRedoStack.length > 0) {
      const snap = this.snapRedoStack.pop();
      if (!snap) return;
      const curState = this.sm.getState();
      const curCtx = JSON.stringify(this.cloneCtx(this.sm.getContext()));
      this.snapUndoStack.push({ state: curState, context: curCtx });
      const o = JSON.parse(snap.context) as Record<string, unknown>;
      this.sm.restore(snap.state, o);
      this.emit();
      return;
    }
    const h = this.opts.history;
    if (h) h.redo(this.opts.document);
    this.emit();
  }

  cancel(): void {
    if (this.child) {
      this.abortChildCall();
      this.emit();
      return;
    }
    this.snapUndoStack.length = 0;
    this.snapRedoStack.length = 0;
    this.sm.reset();
    this.emit();
  }

  private async runCommit(advanceToFinalState: boolean): Promise<InteractionResponse> {
    const fail = (code: string, message: string): InteractionResponse => {
      const res: InteractionResponse = {
        ok: false,
        errors: [{ code, message }],
        warnings: [],
        infos: [],
        diff: EMPTY_MODEL_DIFF,
        data: null,
        archiveContext: null,
      };
      this.lastResponse = res;
      this.emit();
      return res;
    };
    const st = this.sm.getState();
    if (advanceToFinalState && isFinalInteractionState(this.spec, st)) {
      return fail("interaction.alreadyCommitted", "Interaction already finalized.");
    }
    if (!this.canCommitFromState(st)) {
      return fail("interaction.cannotCommit", "Commit guard or fromStates rejected this commit.");
    }
    const ctx = this.sm.getContext();
    const operation = this.spec.commit.operation;
    const env: ExprEnv = { context: ctx, preview: this.previewKernel() };
    const k = this.opts.kernel;
    const model = this.opts.document.model;
    let diff: ModelDiff = EMPTY_MODEL_DIFF;
    let data: unknown = null;
    try {
      const paramBag: Record<string, unknown> = { __context: ctx, __event: { kind: "commit" } };
      for (const [key, ex] of Object.entries(operation.params ?? {})) {
        paramBag[key] = evalExpr(ex, env);
      }
      const kit = typologyConstructKitByInteraction().get(this.spec.id);
      const actionId = kit ? typologyConstructCommitActionForMode(kit, String(paramBag.constructMode ?? ctx.constructMode ?? "")) : operation.action;
      const ar = await runRegisteredAction(this.actions, actionId, paramBag, {
        kernel: k,
        preview: this.previewKernel(),
        model: model,
        activeModelDefinitionId: this.opts.activeModelDefinitionId ?? null,
      });
      if (ar.patch) applyActionPatchToContext(this.sm.getContext(), ar.patch);
      diff = ar.diff ?? EMPTY_MODEL_DIFF;
      data = ar.data ?? null;
      if (isEmptyModelDiff(diff) && operation.action === "command.finish") {
        return fail("interaction.emptyCommit", "Command produced no geometry; add more points and finish again.");
      }
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      return fail("interaction.commitFailed", msg);
    }
    const outPath = this.spec.commit.outputDataPath;
    if (outPath) {
      const ctx2 = this.sm.getContext();
      writePathTarget(outPath, this.exprEnv({ context: ctx2 }), data);
      data = readPathTarget(outPath, this.exprEnv({ context: ctx2 })) ?? data;
    }
    const inverse = applyModelDiff(model, diff);
    const typologyId = typologyIdForInteractionCommit(this.spec.id);
    if (typologyId && !isEmptyModelDiff(diff)) ensureTypologyObjectFromCreateDiff(model, typologyId, diff);
    const archiveContext = this.cloneCtx(this.sm.getContext());
    if (advanceToFinalState) await this.sm.send({ kind: "confirm" }, k, model, this.actions, this.previewKernel(), this.opts.activeModelDefinitionId ?? null);
    const res: InteractionResponse = { ok: true, errors: [], warnings: [], infos: [], diff, data, archiveContext };
    this.lastResponse = res;
    this.snapUndoStack.length = 0;
    this.snapRedoStack.length = 0;
    const hist = this.opts.history;
    if (hist && interactionRecordsDocumentHistory(this.spec.id) && !isEmptyModelDiff(diff)) {
      hist.record({
        id: `cmd-${this.spec.id}-${this.revision}`,
        interactionId: this.spec.id,
        label: this.spec.label ?? this.spec.id,
        result: res,
        backwardsDiff: inverse,
      });
    }
    this.emit();
    return res;
  }

  /** 📜️ Executes `commit.operation` against `kernel`, applies `diff` to `document.model`, records history. */
  async commit(): Promise<InteractionResponse> {
    if (this.child) return this.child.commit();
    return this.runCommit(true);
  }
}

/** 📜️ Constructs a `InteractionRuntime` from a compiled `InteractionSpec`. */
export function createInteractionRuntime(spec: InteractionSpec, opts: InteractionRuntimeOptions): InteractionRuntime {
  return new InteractionRuntime(compileInteraction(spec), opts);
}

/** ⎋️ Hard-aborts the active interaction session when `capabilities.canCancel`. */
export function abortActiveInteractionSession(rt: InteractionRuntime): boolean {
  if (!rt.getSnapshot().capabilities.canCancel) return false;
  rt.cancel();
  return true;
}
// #endregion 📜️Interaction
// #endregion 📦️📄️document

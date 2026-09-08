// #region 🧲️Header
/** @emoji 🧭️ `@semio-tech/cad-js` — CAD domain module facet. See `cad/AGENTS.md`. */
import { ephemeralBox, ephemeralMap, ephemeralWeakMap } from "@semio-tech/framework";
import type { ArcPlaneFrame, EdgeCurve, EdgeGroup, EdgeInfo, FaceGroup, FaceInfo, MeshTransfer, Vec3 } from "@semio-tech/s-3d-js";
import { emptyMeshTransfer, kernelGeometry, solidRef } from "@semio-tech/s-3d-js";
// #endregion 🧲️Header


import { Expr, ExprEnv, InteractionEvent, InteractionLengthEntrySpec, InteractionScalarEntrySpec, InteractionSpec, Model, ModelEntityKind, SelectionEvent, SelectionSpec, SelectionTarget, SolidRef, SpatialInteraction, StateDefSpec, assertActionAvailableInModelDefinition, compileInteraction, defaultModelDefinitionId, evalExpr, evalGuard, expandSelectionTargetsForAccept, getActiveSelectionSpec, isFinalInteractionState, listActionsForModelDefinition, listSpatialInteractionsForModelDefinition, mergeInteractionCallOutputs, parseInteractionSpec, readPathTarget, selectionEventMatches, writePathTarget } from "../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts";
import { ensureTypologyObjectFromCreateDiff, typologyConstructCommitActionForMode, typologyConstructKitByInteraction, typologyIdForInteractionCommit } from "../🧬️typology/🟦️.ts";
import { interactionCompileCacheClear, modelDefinitionInteractionCatalog } from "../📔️registry/🟦️.ts";
import { EMPTY_MODEL_DIFF, ModelDiff, SpatialKernel, SpatialPreviewKernel, applyModelDiff, isEmptyModelDiff } from "../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️.ts";
import { ActionContextPatch, ActionRegistry, ConstructQueryResult, ConstructRunner, DisplayModel, InteractionChildCallSpec, InteractionSpatialResolved, SelectionApplyOperation, SelectionApplyParams, SelectionOperationInteractionDef, StateEngine, StateEngineProvider, StateEngineSendResult, clampPointAlongDirection, interactionLengthEntryForState, interactionRecordsDocumentHistory, interactionScalarEntryForState, mergeInteractionSpatial, modelDefinitionActionRegistry, projectPointOnScalarAxis, pureTsStateEngineProvider, readInteractionContextVec3, resolveDisplay, runRegisteredAction, scalarEntryAxisBase, selectionTargetsFromActionResult, writeInteractionContextVec3 } from "../🎬️actions/🟦️.ts";



// #region 📦️📄️document
// #region 📄️Document
/** @emoji 📄️ Single committed modeling operation node. */
export interface ShapeNode {
  readonly id: string;
  readonly operationKind: string;
  readonly solidRef?: SolidRef;
}

/** @emoji 📄️ Working document: model + committed shape nodes + command stack. */
export interface ModelDocument {
  readonly model: Model;
  nodes: ShapeNode[];
}

// #endregion 📄️Document

// #region 📨️Response
/** @emoji 📨️ Portable command outcome envelope (`diff` + `data` + messages). */
export interface InteractionMessage {
  readonly code: string;
  readonly message: string;
  readonly path?: string;
}

/** @emoji 📨️ Result returned by `InteractionRuntime.commit` — modeling output is always `diff` (model geometry); `data` is auxiliary. */
export interface InteractionResponse<TData = unknown> {
  readonly ok: boolean;
  readonly errors: readonly InteractionMessage[];
  readonly warnings: readonly InteractionMessage[];
  readonly infos: readonly InteractionMessage[];
  readonly diff: ModelDiff;
  readonly data: TData | null;
  /** @emoji 📦️ Context clone immediately before the post-commit `confirm` transition; null when commit aborted before confirm. */
  readonly archiveContext: Record<string, unknown> | null;
}

/** @emoji 📨️ Default empty success payload for guards and early returns. */
export const EMPTY_INTERACTION_RESPONSE: InteractionResponse<null> = {
  ok: true,
  errors: [],
  warnings: [],
  infos: [],
  diff: EMPTY_MODEL_DIFF,
  data: null,
  archiveContext: null,
};

/** @emoji 📄️ One committed model change plus inverse diff for document-level undo/redo. */
export interface Modification {
  readonly id: string;
  readonly interactionId: string;
  readonly label: string;
  readonly result: InteractionResponse;
  readonly backwardsDiff: ModelDiff;
}

/** @emoji 📄️ Two-stack modification history (undo / redo) keyed by model diffs. */
export class DocumentHistory {
  private undoStack: Modification[] = [];
  private redoStack: Modification[] = [];

  record(mod: Modification): void {
    if (isEmptyModelDiff(mod.result.diff)) return;
    this.undoStack.push(mod);
    this.redoStack = [];
  }

  peekUndo(): Modification | null {
    const n = this.undoStack.length;
    return n ? this.undoStack[n - 1]! : null;
  }

  peekRedo(): Modification | null {
    const n = this.redoStack.length;
    return n ? this.redoStack[n - 1]! : null;
  }

  /** @emoji 📚️ Committed undo stack in document order for renderer views. */
  entries(): readonly Modification[] {
    return [...this.undoStack];
  }

  /** @emoji 🧹️ Drops undo and redo stacks when the host swaps the base document. */
  clear(): void {
    this.undoStack = [];
    this.redoStack = [];
  }

  undo(doc: ModelDocument): Modification | null {
    const mod = this.undoStack.pop();
    if (!mod) return null;
    applyModelDiff(doc.model, mod.backwardsDiff);
    this.redoStack.push(mod);
    return mod;
  }

  redo(doc: ModelDocument): Modification | null {
    const mod = this.redoStack.pop();
    if (!mod) return null;
    applyModelDiff(doc.model, mod.result.diff);
    this.undoStack.push(mod);
    return mod;
  }
}
// #endregion 📨️Response

// #region 📜️Interaction
/** @emoji 🩺️ Non-fatal runtime diagnostic surfaced in snapshots. */
export interface Diagnostic {
  readonly severity: "info" | "warning" | "error";
  readonly code: string;
  readonly message: string;
}

/** @emoji 📜️ Host frame while a nested interaction session is active (chain via `outer`). */
export interface InteractionNestedHostFrame {
  readonly hostInteractionId: string;
  readonly hostState: string;
  readonly hostContext: Record<string, unknown>;
  readonly outer?: InteractionNestedHostFrame;
}

/** @emoji 📜️ Serializable interaction snapshot for hosts and renderers. */
export interface InteractionSnapshot {
  readonly interactionId: string;
  readonly state: string;
  readonly revision: number;
  readonly context: Record<string, unknown>;
  readonly display: DisplayModel;
  readonly spatialInteraction: InteractionSpatialResolved;
  readonly capabilities: { readonly canCommit: boolean; readonly canCancel: boolean; readonly canUndo: boolean; readonly canRedo: boolean };
  readonly diagnostics: readonly Diagnostic[];
  readonly lastResponse: InteractionResponse | null;
  readonly nested?: InteractionNestedHostFrame;
}

export type SpatialComputeMode = "fast" | "precise";

export interface InteractionRuntimeOptions {
  readonly kernel: SpatialKernel;
  readonly previewKernel?: SpatialPreviewKernel;
  readonly mode?: SpatialComputeMode;
  readonly document: ModelDocument;
  readonly history?: DocumentHistory;
  readonly stateEngine?: StateEngineProvider;
  readonly actions?: ActionRegistry;
  readonly interactions?: InteractionRegistry;
  readonly query?: ConstructRunner;
  readonly activeModelDefinitionId?: string | null;
}

/** @emoji 📞️ Resolves an interaction spec for `interaction.call` (registry first, then shipped assets). */
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

/** @emoji 📜️ Headless + interactive interaction controller (`send`, `commit`, `undo`). */
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

  /** @emoji 🔌️ Precise BREP kernel wired into this runtime (tessellation, commit, derived views). */
  kernel(): SpatialKernel {
    return this.opts.kernel;
  }

  /** @emoji ⚡️ `fast` uses `previewKernel`; `precise` uses the BREP kernel for preview math too. */
  computeMode(): SpatialComputeMode {
    return this.opts.mode ?? "precise";
  }

  /** @emoji ⚡️ Active preview kernel for the current `mode` (fast renderer vs precise brep). */
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

  /** @emoji 🧭️ Accepted geometry entity kinds for the active machine state (`[]` when none). */
  listActiveSelectionAccept(): readonly ModelEntityKind[] {
    if (this.child) return this.child.listActiveSelectionAccept();
    return getActiveSelectionSpec(this.spec, this.sm.getState())?.accept ?? [];
  }

  /** @emoji 🔍️ Executes a `construct` script via `opts.query` (host registers `@semio-tech/cad-js/query`). */
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

  /** @emoji 📜️ Merges `start.targets` when `compileInteraction` began in the post-start state without running transition effects. */
  private applyInstantStartPayload(event: InteractionEvent): void {
    const raw = event.targets;
    if (!Array.isArray(raw) || raw.length === 0) return;
    this.sm.getContext().seedTargets = raw;
  }

  /** @emoji 📜️ Dispatches a typed interaction event through the statechart + optional kernel queries. */
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

  /** @emoji 📜️ Executes `commit.operation` against `kernel`, applies `diff` to `document.model`, records history. */
  async commit(): Promise<InteractionResponse> {
    if (this.child) return this.child.commit();
    return this.runCommit(true);
  }
}

/** @emoji 📜️ Constructs a `InteractionRuntime` from a compiled `InteractionSpec`. */
export function createInteractionRuntime(spec: InteractionSpec, opts: InteractionRuntimeOptions): InteractionRuntime {
  return new InteractionRuntime(compileInteraction(spec), opts);
}

/** @emoji 🪪️ Runs a `selection.*` action (declarative headless command, no interaction session). */
export async function runSelectionOperationInteraction(
  interactionId: string,
  opts: InteractionRuntimeOptions & { readonly seedTargets?: readonly SelectionTarget[] },
): Promise<{ readonly response: InteractionResponse; readonly targets: readonly SelectionTarget[] }> {
  const mdId = opts.activeModelDefinitionId ?? defaultModelDefinitionId();
  const defn = selectionOperationsForModelDefinitionFromActions(mdId).find((row) => row.id === interactionId);
  if (!defn) throw new Error(`Not a selection operation: ${interactionId}`);
  assertActionAvailableInModelDefinition(interactionId, mdId);
  const seedTargets = opts.seedTargets ?? [];
  const result = await runRegisteredAction(
    opts.actions ?? modelDefinitionActionRegistry(),
    interactionId,
    { seedTargets, __context: {}, __event: { kind: "commit" } },
    { kernel: opts.kernel, preview: opts.previewKernel ?? (opts.kernel as unknown as SpatialPreviewKernel), model: opts.document.model, activeModelDefinitionId: mdId },
  );
  const targets = selectionTargetsFromActionResult(result);
  return {
    response: {
      ok: true,
      errors: [],
      warnings: [],
      infos: [],
      diff: result.diff ?? EMPTY_MODEL_DIFF,
      data: result.data ?? null,
      archiveContext: { targets },
    },
    targets,
  };
}
// #endregion 📜️Interaction

// #region 📦️Interactions
type ModelDefinitionInteractionFixture = InteractionSpec & { readonly key?: string };

const SELECTION_INTERACTION_KEYS: Readonly<Record<string, string>> = {
  "selection.selectAll": "sa",
  "selection.deselectAll": "ds",
  "selection.invert": "iv",
  "selection.selectAnchors": "xa",
  "selection.selectVertices": "xv",
  "selection.selectEdges": "xe",
  "selection.selectWires": "xw",
  "selection.selectFaces": "xf",
  "selection.selectSolids": "xc",
  "selection.selectGeometries": "xg",
  "selection.selectObjects": "xo",
};

const SELECTION_ACTION_META: Readonly<Record<string, { readonly operation: SelectionApplyOperation; readonly kinds?: readonly ModelEntityKind[] }>> = {
  "selection.selectAll": { operation: "selectAll" },
  "selection.deselectAll": { operation: "deselectAll" },
  "selection.invert": { operation: "invert" },
  "selection.selectAnchors": { operation: "selectKinds", kinds: ["anchor"] },
  "selection.selectVertices": { operation: "selectKinds", kinds: ["vertex"] },
  "selection.selectEdges": { operation: "selectKinds", kinds: ["edge"] },
  "selection.selectWires": { operation: "selectKinds", kinds: ["wire"] },
  "selection.selectFaces": { operation: "selectKinds", kinds: ["face"] },
  "selection.selectSolids": { operation: "selectKinds", kinds: ["solid"] },
  "selection.selectGeometries": { operation: "selectKinds", kinds: ["geometry"] },
  "selection.selectObjects": { operation: "selectKinds", kinds: ["object"] },
};

function selectionOperationDefForActionId(actionId: string, label?: string): SelectionOperationInteractionDef | null {
  if (!actionId.startsWith("selection.") || actionId === "selection.apply") return null;
  const meta = SELECTION_ACTION_META[actionId];
  const key = SELECTION_INTERACTION_KEYS[actionId];
  if (!meta || !key) return null;
  return {
    id: actionId,
    label: label ?? actionId.slice("selection.".length),
    key,
    operation: meta.operation,
    ...(meta.kinds ? { kinds: [...meta.kinds] } : {}),
  };
}

function selectionOperationsForModelDefinitionFromActions(modelDefinitionId: string): readonly SelectionOperationInteractionDef[] {
  const out: SelectionOperationInteractionDef[] = [];
  for (const actionId of listActionsForModelDefinition(modelDefinitionId)) {
    const defn = selectionOperationDefForActionId(actionId);
    if (defn) out.push(defn);
  }
  return out.sort((a, b) => a.id.localeCompare(b.id));
}

/** @emoji 🪪️ Maps a `selection.*` interaction id to headless `SelectionApplyParams`. */
export function selectionApplyParamsForInteraction(defn: SelectionOperationInteractionDef, seedTargets: readonly SelectionTarget[] = []): SelectionApplyParams {
  return {
    operation: defn.operation,
    seedTargets,
    ...(defn.kinds ? { kinds: defn.kinds } : {}),
  };
}

/** @emoji 🪪️ True when a selection command targets authored `object` rows on the model. */
export function selectionOperationUsesModelObjects(defn: Pick<SelectionOperationInteractionDef, "kinds">): boolean {
  return defn.kinds?.includes("object") ?? false;
}

/** @emoji 🪪️ Default seed targets for invert/deselectAll (otherwise empty). */
export function selectionSeedTargetsForOperation(operation: SelectionApplyOperation, seedCell: SelectionTarget = { kind: "solid", id: "e2e-box", editable: true }): readonly SelectionTarget[] {
  return operation === "invert" || operation === "deselectAll" ? [seedCell] : [];
}

function shippedInteractionJsons(): readonly ModelDefinitionInteractionFixture[] {
  return modelDefinitionInteractionCatalog() as readonly ModelDefinitionInteractionFixture[];
}

function interactionFixtureRow(spec: ModelDefinitionInteractionFixture): SpatialInteraction {
  return { id: spec.id, label: spec.label ?? spec.id, key: typeof spec.key === "string" ? spec.key : (spec.id[0] ?? "?") };
}

function shippedSpatialInteractionCatalog(): readonly SpatialInteraction[] {
  return shippedInteractionJsons().map(interactionFixtureRow);
}

/** @emoji 🧭️ Resolves a typed token to an interaction in one model definition (`key`, `id`, or compact `label`). */
export function resolveSpatialInteractionKeyForModelDefinition(modelDefinitionId: string, token: string): SpatialInteraction | null {
  const t = token.trim().toLowerCase();
  if (!t) return null;
  for (const p of listSpatialInteractionsForModelDefinition(modelDefinitionId)) {
    if (p.key.toLowerCase() === t) return p;
    if (p.id.toLowerCase() === t) return p;
    const slug = p.label.toLowerCase().replace(/\s+/g, "");
    if (slug === t) return p;
  }
  return null;
}

/** @emoji 🧭️ model-definition `InteractionSpec` map (fixtures + host `registerInteraction`). */
export type InteractionRegistry = ReadonlyMap<string, InteractionSpec>;

/** @emoji 🧭️ Registers one compiled interaction; returns a new registry (immutable update). */
export function registerInteractionSpec(registry: InteractionRegistry, spec: InteractionSpec): InteractionRegistry {
  const next = new Map(registry);
  next.set(spec.id, spec);
  return next;
}

/** @emoji 🧭️ Lists registered interaction specs in stable id order. */
export function listInteractionSpecs(registry: InteractionRegistry): readonly InteractionSpec[] {
  return [...registry.values()].sort((a, b) => a.id.localeCompare(b.id));
}

/** @emoji 🧭️ Shipped model-definition interactions compiled from fixtures. */
export function modelDefinitionInteractionRegistry(): InteractionRegistry {
  const map = new Map<string, InteractionSpec>();
  for (const raw of shippedInteractionJsons()) {
    const spec = parseInteractionSpec(raw);
    if (spec) map.set(spec.id, compileInteraction(spec));
  }
  return map;
}

const COMPILED_INTERACTION_BY_ID = ephemeralMap<string, InteractionSpec>("s.plugins.cad.modules.core.component.ts.COMPILED_INTERACTION_BY_ID");
/** Clears the derived interaction compile cache when model-definition assets change. */
interactionCompileCacheClear.current = () => COMPILED_INTERACTION_BY_ID.clear();

/** @emoji 📚️ Loads a model-definition interaction by stable `id` (compiled once per id for stable React runtime identity). */
export function loadSpatialInteraction(interactionId: string): InteractionSpec | null {
  const cached = COMPILED_INTERACTION_BY_ID.get(interactionId);
  if (cached) return cached;
  const raw = shippedInteractionJsons().find((spec) => spec.id === interactionId);
  const spec = raw ? parseInteractionSpec(raw) : null;
  if (!spec) return null;
  const compiled = compileInteraction(spec);
  COMPILED_INTERACTION_BY_ID.set(interactionId, compiled);
  return compiled;
}

function requireSpatialInteraction(interactionId: string): InteractionSpec {
  const spec = loadSpatialInteraction(interactionId);
  if (!spec) throw new Error(`${interactionId} interaction missing from modelDefinition assets`);
  return spec;
}

/** @emoji 📦️ Compiled `primitive.box` interaction from model-definition assets. */
export function buildBoxInteractionSpec(): InteractionSpec {
  return requireSpatialInteraction("primitive.box");
}

/** @emoji 📦️ Compiled `feature.extrudeWire` interaction from model-definition assets. */
export function buildExtrudeInteractionSpec(): InteractionSpec {
  return requireSpatialInteraction("feature.extrudeWire");
}

/** @emoji 📦️ Compiled `feature.offsetSurface` interaction from model-definition assets. */
export function buildOffsetSurfaceInteractionSpec(): InteractionSpec {
  return requireSpatialInteraction("feature.offsetSurface");
}

/** @emoji 📦️ Compiled `measure.distance` interaction from model-definition assets. */
export function buildDistanceInteractionSpec(): InteractionSpec {
  return requireSpatialInteraction("measure.distance");
}

/** @emoji 📦️ Compiled `measure.area` interaction from model-definition assets. */
export function buildAreaInteractionSpec(): InteractionSpec {
  return requireSpatialInteraction("measure.area");
}

// #endregion 📦️Interactions

// #endregion 📦️📄️document

// #region 🧪️Tests
import { EdgeRef, FaceRef, ModelSpace, ModelSpaceJson, ObjectRef, TypologyRef, VertexRef, WireRef, actionOwnedByModelDefinition, isCallableOnlyInteraction, isShapeModelDefinition, listModelDefinitionManifests, listModelDefinitionTypologies, listModelObjectsForModelDefinition, listSelectionOperationsForModelDefinition, listTypologiesForModelDefinition, loadTypology, modelDefinitionIdForInteraction, parseModelJson } from "../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts";
import { listConstructableTypologiesForModelDefinition, typologyConstructAssetIds, typologyConstructModeActionIds, typologyHasNativeConstructKit } from "../🧬️typology/🟦️.ts";
import { collectGeometrySelectionTargets, executeSelectionApply, interactionControlForState, interactionLengthEntryLiveDistance, interactionNumericEntryCommitEvent, interactionNumericEntryExplicitLockValue, interactionNumericEntryLockedValue, interactionStepFinalizeEvent, listActionDefs, listModelDefinitionActionSpecs, parseActionSpec, registerActionDef, runSelectionApply, selectionTargetsFromContext, selectionTargetsPointTransformDiff } from "../🎬️actions/🟦️.ts";

const __artifactTestRuntime = import.meta.vitest ? await import("../🏃️runtime/🟦️.ts") : null;
const __artifactTestKernel = import.meta.vitest ? await import("../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️.ts") : null;
const CAD_E2E_LOOM_MODEL_SPACE_JSON =
  '{"schema":"spatial.modelspace","revision":1,"models":[{"id":"spatial.shape","model":{"schema":"spatial.model","revision":2,"objects":[{"id":"object-edge-e11","typology":"spatial.shape.kernel.edge","primitives":[{"kind":"vertex","id":"v4","position":[1,1,1.2]},{"kind":"vertex","id":"v9","position":[2.5,0.3,2.4]},{"kind":"curve","slot":"edge","id":"e11","vertexIds":["v4","v9"]}]},{"id":"object-void-cap","typology":"spatial.shape.primitive.box","primitives":[{"kind":"vertex","id":"v0","position":[0,0,0]},{"kind":"vertex","id":"v1","position":[5,0,0]},{"kind":"vertex","id":"v2","position":[5,4,0]},{"kind":"vertex","id":"v3","position":[0,4,0]},{"kind":"curve","id":"e0","vertexIds":["v0","v1"]},{"kind":"curve","id":"e1","vertexIds":["v1","v2"]},{"kind":"curve","id":"e2","vertexIds":["v2","v3"]},{"kind":"curve","id":"e3","vertexIds":["v3","v0"]},{"kind":"curve","id":"w-deck","edgeIds":["e0","e1","e2","e3"]},{"kind":"surface","id":"stub-surface","wireIds":["w-deck"]},{"kind":"shell","id":"shell-deck","faceIds":["stub-surface"]},{"kind":"solid","slot":"solid","id":"void-cap","shellIds":["shell-deck"]}]},{"id":"object-wire-rail-upper","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"v10","position":[4.5,1.8,2.4]},{"kind":"vertex","id":"v11","position":[3.8,3.5,2.4]},{"kind":"vertex","id":"v9","position":[2.5,0.3,2.4]},{"kind":"curve","id":"e10","vertexIds":["v10","v11"]},{"kind":"curve","id":"e9","vertexIds":["v9","v10"]},{"kind":"curve","slot":"wire","id":"rail-upper","edgeIds":["e9","e10"]}]},{"id":"object-wire-stub-wire","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"v4","position":[1,1,1.2]},{"kind":"vertex","id":"v5","position":[3,2.2,1.2]},{"kind":"vertex","id":"v6","position":[4.2,0.8,1.2]},{"kind":"vertex","id":"v7","position":[2.5,3.1,1.2]},{"kind":"vertex","id":"v8","position":[0.8,2.4,1.2]},{"kind":"curve","id":"e4","vertexIds":["v4","v5"]},{"kind":"curve","id":"e5","vertexIds":["v5","v6"]},{"kind":"curve","id":"e6","vertexIds":["v6","v7"]},{"kind":"curve","id":"e7","vertexIds":["v7","v8"]},{"kind":"curve","id":"e8","vertexIds":["v8","v4"]},{"kind":"curve","slot":"wire","id":"stub-wire","edgeIds":["e4","e5","e6","e7","e8"]}]}]}}]}';
const CAD_E2E_ROUTES_MODEL_SPACE_JSON =
  '{"schema":"spatial.modelspace","revision":1,"models":[{"id":"spatial.shape","model":{"schema":"spatial.model","revision":1,"objects":[{"id":"object-wire-orbit-a","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"r10","position":[6,0,0.8]},{"kind":"vertex","id":"r11","position":[7.4,0.6,0.8]},{"kind":"vertex","id":"r12","position":[8.8,0.2,0.8]},{"kind":"vertex","id":"r13","position":[9.9,1.1,0.8]},{"kind":"vertex","id":"r14","position":[10.2,2.6,0.8]},{"kind":"vertex","id":"r15","position":[9.4,3.9,0.8]},{"kind":"vertex","id":"r16","position":[7.8,4.4,0.8]},{"kind":"vertex","id":"r17","position":[6.2,4.1,0.8]},{"kind":"curve","id":"re10","vertexIds":["r10","r11"]},{"kind":"curve","id":"re11","vertexIds":["r11","r12"]},{"kind":"curve","id":"re12","vertexIds":["r12","r13"]},{"kind":"curve","id":"re13","vertexIds":["r13","r14"]},{"kind":"curve","id":"re14","vertexIds":["r14","r15"]},{"kind":"curve","id":"re15","vertexIds":["r15","r16"]},{"kind":"curve","id":"re16","vertexIds":["r16","r17"]},{"kind":"curve","id":"re17","vertexIds":["r17","r10"]},{"kind":"curve","slot":"wire","id":"orbit-a","edgeIds":["re10","re11","re12","re13","re14","re15","re16","re17"]}]},{"id":"object-wire-spine-b","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"r18","position":[2,6,1.6]},{"kind":"vertex","id":"r19","position":[3.5,6.8,1.6]},{"kind":"vertex","id":"r20","position":[5.2,6.5,1.6]},{"kind":"vertex","id":"r21","position":[6.8,7.2,1.6]},{"kind":"vertex","id":"r22","position":[7.5,8.4,1.6]},{"kind":"vertex","id":"r23","position":[6.1,9.1,1.6]},{"kind":"curve","id":"re18","vertexIds":["r18","r19"]},{"kind":"curve","id":"re19","vertexIds":["r19","r20"]},{"kind":"curve","id":"re20","vertexIds":["r20","r21"]},{"kind":"curve","id":"re21","vertexIds":["r21","r22"]},{"kind":"curve","id":"re22","vertexIds":["r22","r23"]},{"kind":"curve","slot":"wire","id":"spine-b","edgeIds":["re18","re19","re20","re21","re22"]}]},{"id":"object-wire-stub-wire","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"r0","position":[0,0,0]},{"kind":"vertex","id":"r1","position":[1.2,0.4,0]},{"kind":"vertex","id":"r2","position":[2.6,0.1,0]},{"kind":"vertex","id":"r3","position":[3.8,0.9,0]},{"kind":"vertex","id":"r4","position":[4.5,2.1,0]},{"kind":"vertex","id":"r5","position":[4.2,3.5,0]},{"kind":"vertex","id":"r6","position":[3.1,4.2,0]},{"kind":"vertex","id":"r7","position":[1.5,4.5,0]},{"kind":"vertex","id":"r8","position":[0.2,3.8,0]},{"kind":"vertex","id":"r9","position":[-0.4,2.2,0]},{"kind":"curve","id":"re0","vertexIds":["r0","r1"]},{"kind":"curve","id":"re1","vertexIds":["r1","r2"]},{"kind":"curve","id":"re2","vertexIds":["r2","r3"]},{"kind":"curve","id":"re3","vertexIds":["r3","r4"]},{"kind":"curve","id":"re4","vertexIds":["r4","r5"]},{"kind":"curve","id":"re5","vertexIds":["r5","r6"]},{"kind":"curve","id":"re6","vertexIds":["r6","r7"]},{"kind":"curve","id":"re7","vertexIds":["r7","r8"]},{"kind":"curve","id":"re8","vertexIds":["r8","r9"]},{"kind":"curve","id":"re9","vertexIds":["r9","r0"]},{"kind":"curve","slot":"wire","id":"stub-wire","edgeIds":["re0","re1","re2","re3","re4","re5","re6","re7","re8","re9"]}]}]}}]}';
const CAD_E2E_BUILDING_BOOLEAN_MODEL_SPACE_JSON =
  '{"schema":"spatial.modelspace","revision":1,"models":[{"id":"spatial.shape","model":{"schema":"spatial.model","revision":1,"objects":[{"id":"object-small-building-cell-123052045","typology":"spatial.shape.primitive.box","primitives":[{"kind":"vertex","id":"small-building-vertex-1027259450","position":[-64.506666,41.273333,-9.407222]},{"kind":"vertex","id":"small-building-vertex-1412665337","position":[-18.586667,41.273333,-43.847222]},{"kind":"vertex","id":"small-building-vertex-200778251","position":[-18.586667,3.553333,-9.407222]},{"kind":"vertex","id":"small-building-vertex-354458280","position":[-18.586667,3.553333,-43.847222]},{"kind":"vertex","id":"small-building-vertex-560960189","position":[-64.506666,3.553333,-9.407222]},{"kind":"vertex","id":"small-building-vertex-566043311","position":[-64.506666,41.273333,-43.847222]},{"kind":"vertex","id":"small-building-vertex-584100920","position":[-64.506666,3.553333,-43.847222]},{"kind":"vertex","id":"small-building-vertex-834828749","position":[-18.586667,41.273333,-9.407222]},{"kind":"curve","id":"small-building-edge-1660152326","vertexIds":["small-building-vertex-354458280","small-building-vertex-1412665337"]},{"kind":"curve","id":"small-building-edge-1943812986","vertexIds":["small-building-vertex-566043311","small-building-vertex-584100920"]},{"kind":"curve","id":"small-building-edge-2004107109","vertexIds":["small-building-vertex-200778251","small-building-vertex-560960189"]},{"kind":"curve","id":"small-building-edge-2075229525","vertexIds":["small-building-vertex-1027259450","small-building-vertex-566043311"]},{"kind":"curve","id":"small-building-edge-224238197","vertexIds":["small-building-vertex-584100920","small-building-vertex-560960189"]},{"kind":"curve","id":"small-building-edge-229106015","vertexIds":["small-building-vertex-1027259450","small-building-vertex-834828749"]},{"kind":"curve","id":"small-building-edge-278123677","vertexIds":["small-building-vertex-584100920","small-building-vertex-354458280"]},{"kind":"curve","id":"small-building-edge-278867947","vertexIds":["small-building-vertex-834828749","small-building-vertex-200778251"]},{"kind":"curve","id":"small-building-edge-332998341","vertexIds":["small-building-vertex-200778251","small-building-vertex-354458280"]},{"kind":"curve","id":"small-building-edge-757634469","vertexIds":["small-building-vertex-1412665337","small-building-vertex-834828749"]},{"kind":"curve","id":"small-building-edge-779379499","vertexIds":["small-building-vertex-1412665337","small-building-vertex-566043311"]},{"kind":"curve","id":"small-building-edge-951546977","vertexIds":["small-building-vertex-560960189","small-building-vertex-1027259450"]},{"kind":"curve","id":"small-building-wire-1366152152","edgeIds":["small-building-edge-278123677","small-building-edge-1943812986","small-building-edge-779379499","small-building-edge-1660152326"]},{"kind":"curve","id":"small-building-wire-1559546061","edgeIds":["small-building-edge-2075229525","small-building-edge-229106015","small-building-edge-757634469","small-building-edge-779379499"]},{"kind":"curve","id":"small-building-wire-1742634236","edgeIds":["small-building-edge-278867947","small-building-edge-332998341","small-building-edge-1660152326","small-building-edge-757634469"]},{"kind":"curve","id":"small-building-wire-456551683","edgeIds":["small-building-edge-1943812986","small-building-edge-224238197","small-building-edge-951546977","small-building-edge-2075229525"]},{"kind":"curve","id":"small-building-wire-515231130","edgeIds":["small-building-edge-224238197","small-building-edge-278123677","small-building-edge-332998341","small-building-edge-2004107109"]},{"kind":"curve","id":"small-building-wire-978200956","edgeIds":["small-building-edge-951546977","small-building-edge-2004107109","small-building-edge-278867947","small-building-edge-229106015"]},{"kind":"surface","id":"small-building-face-1071813579","wireIds":["small-building-wire-1559546061"]},{"kind":"surface","id":"small-building-face-1198070201","wireIds":["small-building-wire-456551683"]},{"kind":"surface","id":"small-building-face-1321487947","wireIds":["small-building-wire-1742634236"]},{"kind":"surface","id":"small-building-face-1833451572","wireIds":["small-building-wire-1366152152"]},{"kind":"surface","id":"small-building-face-383803774","wireIds":["small-building-wire-515231130"]},{"kind":"surface","id":"small-building-face-624717229","wireIds":["small-building-wire-978200956"]},{"kind":"shell","id":"small-building-shell-319815043","faceIds":["small-building-face-1198070201","small-building-face-1833451572","small-building-face-383803774","small-building-face-624717229","small-building-face-1071813579","small-building-face-1321487947"]},{"kind":"solid","slot":"solid","id":"small-building-cell-123052045","shellIds":["small-building-shell-319815043"]}]},{"id":"object-small-building-cell-1278694563","typology":"spatial.shape.primitive.box","primitives":[{"kind":"vertex","id":"small-building-vertex-1052483923","position":[-18.586667,-57.126666,-9.407222]},{"kind":"vertex","id":"small-building-vertex-1078954806","position":[-64.506666,-57.126666,-9.407222]},{"kind":"vertex","id":"small-building-vertex-1417750768","position":[-18.586667,3.553333,54.142777]},{"kind":"vertex","id":"small-building-vertex-1487235108","position":[-64.506666,-57.126666,25.852777]},{"kind":"vertex","id":"small-building-vertex-1653716766","position":[-64.506666,3.553333,54.142777]},{"kind":"vertex","id":"small-building-vertex-1928378833","position":[-64.506666,-7.926666,62.752777]},{"kind":"vertex","id":"small-building-vertex-200778251","position":[-18.586667,3.553333,-9.407222]},{"kind":"vertex","id":"small-building-vertex-551332595","position":[-18.586667,-7.926666,62.752777]},{"kind":"vertex","id":"small-building-vertex-560960189","position":[-64.506666,3.553333,-9.407222]},{"kind":"vertex","id":"small-building-vertex-945778871","position":[-18.586667,-57.126666,25.852777]},{"kind":"curve","id":"small-building-edge-1070453701","vertexIds":["small-building-vertex-945778871","small-building-vertex-1052483923"]},{"kind":"curve","id":"small-building-edge-1108121009","vertexIds":["small-building-vertex-945778871","small-building-vertex-1487235108"]},{"kind":"curve","id":"small-building-edge-1261178177","vertexIds":["small-building-vertex-560960189","small-building-vertex-1653716766"]},{"kind":"curve","id":"small-building-edge-1422159112","vertexIds":["small-building-vertex-1078954806","small-building-vertex-1487235108"]},{"kind":"curve","id":"small-building-edge-1567786765","vertexIds":["small-building-vertex-551332595","small-building-vertex-945778871"]},{"kind":"curve","id":"small-building-edge-1705326756","vertexIds":["small-building-vertex-1928378833","small-building-vertex-551332595"]},{"kind":"curve","id":"small-building-edge-2004107109","vertexIds":["small-building-vertex-200778251","small-building-vertex-560960189"]},{"kind":"curve","id":"small-building-edge-2102926252","vertexIds":["small-building-vertex-1928378833","small-building-vertex-1653716766"]},{"kind":"curve","id":"small-building-edge-349583852","vertexIds":["small-building-vertex-1052483923","small-building-vertex-1078954806"]},{"kind":"curve","id":"small-building-edge-354613623","vertexIds":["small-building-vertex-1487235108","small-building-vertex-1928378833"]},{"kind":"curve","id":"small-building-edge-432705901","vertexIds":["small-building-vertex-200778251","small-building-vertex-1052483923"]},{"kind":"curve","id":"small-building-edge-432807106","vertexIds":["small-building-vertex-1417750768","small-building-vertex-200778251"]},{"kind":"curve","id":"small-building-edge-467629952","vertexIds":["small-building-vertex-1078954806","small-building-vertex-560960189"]},{"kind":"curve","id":"small-building-edge-49481349","vertexIds":["small-building-vertex-1417750768","small-building-vertex-551332595"]},{"kind":"curve","id":"small-building-edge-508083477","vertexIds":["small-building-vertex-1417750768","small-building-vertex-1653716766"]},{"kind":"curve","id":"small-building-wire-1095357825","edgeIds":["small-building-edge-467629952","small-building-edge-1422159112","small-building-edge-354613623","small-building-edge-2102926252","small-building-edge-1261178177"]},{"kind":"curve","id":"small-building-wire-1399618711","edgeIds":["small-building-edge-349583852","small-building-edge-432705901","small-building-edge-2004107109","small-building-edge-467629952"]},{"kind":"curve","id":"small-building-wire-1676387167","edgeIds":["small-building-edge-1070453701","small-building-edge-432705901","small-building-edge-432807106","small-building-edge-49481349","small-building-edge-1567786765"]},{"kind":"curve","id":"small-building-wire-1755121565","edgeIds":["small-building-edge-2004107109","small-building-edge-432807106","small-building-edge-508083477","small-building-edge-1261178177"]},{"kind":"curve","id":"small-building-wire-285005499","edgeIds":["small-building-edge-349583852","small-building-edge-1070453701","small-building-edge-1108121009","small-building-edge-1422159112"]},{"kind":"curve","id":"small-building-wire-311748032","edgeIds":["small-building-edge-354613623","small-building-edge-1108121009","small-building-edge-1567786765","small-building-edge-1705326756"]},{"kind":"curve","id":"small-building-wire-924227310","edgeIds":["small-building-edge-49481349","small-building-edge-508083477","small-building-edge-2102926252","small-building-edge-1705326756"]},{"kind":"surface","id":"small-building-face-1144073303","wireIds":["small-building-wire-1755121565"]},{"kind":"surface","id":"small-building-face-1382526041","wireIds":["small-building-wire-1095357825"]},{"kind":"surface","id":"small-building-face-1891411219","wireIds":["small-building-wire-1399618711"]},{"kind":"surface","id":"small-building-face-2019874201","wireIds":["small-building-wire-311748032"]},{"kind":"surface","id":"small-building-face-2129815768","wireIds":["small-building-wire-924227310"]},{"kind":"surface","id":"small-building-face-512606747","wireIds":["small-building-wire-285005499"]},{"kind":"surface","id":"small-building-face-816291467","wireIds":["small-building-wire-1676387167"]},{"kind":"shell","id":"small-building-shell-115098816","faceIds":["small-building-face-816291467","small-building-face-512606747","small-building-face-1891411219","small-building-face-1144073303","small-building-face-2129815768","small-building-face-2019874201","small-building-face-1382526041"]},{"kind":"solid","slot":"solid","id":"small-building-cell-1278694563","shellIds":["small-building-shell-115098816"]}]}]}}]}';

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️semio-tech-cad-js-core-interactions/🟦️.ts");
  await registerTests1(import.meta.vitest, { CAD_E2E_BUILDING_BOOLEAN_MODEL_SPACE_JSON, CAD_E2E_LOOM_MODEL_SPACE_JSON, CAD_E2E_ROUTES_MODEL_SPACE_JSON, DocumentHistory, EMPTY_MODEL_DIFF, EdgeRef, FaceRef, InteractionEvent, InteractionRuntime, InteractionSpec, Model, ModelDiff, ModelEntityKind, ModelSpace, ModelSpaceJson, ObjectRef, SelectionOperationInteractionDef, SelectionTarget, SolidRef, SpatialKernel, SpatialPreviewKernel, TypologyRef, VertexRef, WireRef, __artifactTestKernel, __artifactTestRuntime, actionOwnedByModelDefinition, applyModelDiff, buildAreaInteractionSpec, buildBoxInteractionSpec, buildDistanceInteractionSpec, clampPointAlongDirection, collectGeometrySelectionTargets, compileInteraction, createInteractionRuntime, defaultModelDefinitionId, emptyMeshTransfer, ensureTypologyObjectFromCreateDiff, executeSelectionApply, interactionControlForState, interactionLengthEntryForState, interactionLengthEntryLiveDistance, interactionNumericEntryCommitEvent, interactionNumericEntryExplicitLockValue, interactionNumericEntryLockedValue, interactionRecordsDocumentHistory, interactionStepFinalizeEvent, isCallableOnlyInteraction, isEmptyModelDiff, isFinalInteractionState, isShapeModelDefinition, listActionDefs, listConstructableTypologiesForModelDefinition, listInteractionSpecs, listModelDefinitionActionSpecs, listModelDefinitionManifests, listModelDefinitionTypologies, listModelObjectsForModelDefinition, listSelectionOperationsForModelDefinition, listSpatialInteractionsForModelDefinition, listTypologiesForModelDefinition, loadSpatialInteraction, loadTypology, mergeInteractionCallOutputs, modelDefinitionActionRegistry, modelDefinitionIdForInteraction, modelDefinitionInteractionRegistry, parseActionSpec, parseInteractionSpec, parseModelJson, pureTsStateEngineProvider, readInteractionContextVec3, registerActionDef, registerInteractionSpec, requireSpatialInteraction, resolveDisplay, runRegisteredAction, runSelectionApply, runSelectionOperationInteraction, selectionApplyParamsForInteraction, selectionSeedTargetsForOperation, selectionTargetsFromActionResult, selectionTargetsFromContext, selectionTargetsPointTransformDiff, solidRef, typologyConstructAssetIds, typologyConstructCommitActionForMode, typologyConstructKitByInteraction, typologyConstructModeActionIds, typologyHasNativeConstructKit, typologyIdForInteractionCommit }, { directory: import.meta.dir, url: import.meta.url });
}
// #endregion 🧪️Tests

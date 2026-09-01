// #region 🧲️Header
/** @emoji 🧭️ `@semio-tech/cad-js` — CAD domain module facet. See `cad/AGENTS.md`. */
import { ephemeralBox, ephemeralMap, ephemeralWeakMap } from "@semio-tech/framework";
import type { ArcPlaneFrame, EdgeCurve, EdgeGroup, EdgeInfo, FaceGroup, FaceInfo, MeshTransfer, Vec3 } from "@semio-tech/s-3d-js";
import { emptyMeshTransfer, kernelGeometry, solidRef } from "@semio-tech/s-3d-js";
// #endregion 🧲️Header


import { Expr, ExprEnv, InteractionEvent, InteractionLengthEntrySpec, InteractionScalarEntrySpec, InteractionSpec, Model, ModelEntityKind, SelectionEvent, SelectionSpec, SelectionTarget, SolidRef, SpatialInteraction, StateDefSpec, assertActionAvailableInModelDefinition, compileInteraction, defaultModelDefinitionId, evalExpr, evalGuard, expandSelectionTargetsForAccept, getActiveSelectionSpec, isFinalInteractionState, listActionsForModelDefinition, listSpatialInteractionsForModelDefinition, mergeInteractionCallOutputs, parseInteractionSpec, readPathTarget, selectionEventMatches, writePathTarget } from "../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts";
import { ensureTypologyObjectFromCreateDiff, typologyConstructCommitActionForMode, typologyConstructKitByInteraction, typologyIdForInteractionCommit } from "../🧬️typology/🟦️component.ts";
import { interactionCompileCacheClear, modelDefinitionInteractionCatalog } from "../📔️registry/🟦️component.ts";
import { EMPTY_MODEL_DIFF, ModelDiff, SpatialKernel, SpatialPreviewKernel, applyModelDiff, isEmptyModelDiff } from "../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️component.ts";
import { ActionContextPatch, ActionRegistry, ConstructQueryResult, ConstructRunner, DisplayModel, InteractionChildCallSpec, InteractionSpatialResolved, SelectionApplyOperation, SelectionApplyParams, SelectionOperationInteractionDef, StateEngine, StateEngineProvider, StateEngineSendResult, clampPointAlongDirection, interactionLengthEntryForState, interactionRecordsDocumentHistory, interactionScalarEntryForState, mergeInteractionSpatial, modelDefinitionActionRegistry, projectPointOnScalarAxis, pureTsStateEngineProvider, readInteractionContextVec3, resolveDisplay, runRegisteredAction, scalarEntryAxisBase, selectionTargetsFromActionResult, writeInteractionContextVec3 } from "../🎬️actions/🟦️component.ts";



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
import { EdgeRef, FaceRef, ModelSpace, ModelSpaceJson, ObjectRef, TypologyRef, VertexRef, WireRef, actionOwnedByModelDefinition, isCallableOnlyInteraction, isShapeModelDefinition, listModelDefinitionManifests, listModelDefinitionTypologies, listModelObjectsForModelDefinition, listSelectionOperationsForModelDefinition, listTypologiesForModelDefinition, loadTypology, modelDefinitionIdForInteraction, parseModelJson } from "../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts";
import { listConstructableTypologiesForModelDefinition, typologyConstructAssetIds, typologyConstructModeActionIds, typologyHasNativeConstructKit } from "../🧬️typology/🟦️component.ts";
import { collectGeometrySelectionTargets, executeSelectionApply, interactionControlForState, interactionLengthEntryLiveDistance, interactionNumericEntryCommitEvent, interactionNumericEntryExplicitLockValue, interactionNumericEntryLockedValue, interactionStepFinalizeEvent, listActionDefs, listModelDefinitionActionSpecs, parseActionSpec, registerActionDef, runSelectionApply, selectionTargetsFromContext, selectionTargetsPointTransformDiff } from "../🎬️actions/🟦️component.ts";

const __artifactTestRuntime = import.meta.vitest ? await import("../🏃️runtime/🟦️component.ts") : null;
const __artifactTestKernel = import.meta.vitest ? await import("../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️component.ts") : null;
const CAD_E2E_LOOM_MODEL_SPACE_JSON =
  '{"schema":"spatial.modelspace","revision":1,"models":[{"id":"spatial.shape","model":{"schema":"spatial.model","revision":2,"objects":[{"id":"object-edge-e11","typology":"spatial.shape.kernel.edge","primitives":[{"kind":"vertex","id":"v4","position":[1,1,1.2]},{"kind":"vertex","id":"v9","position":[2.5,0.3,2.4]},{"kind":"curve","slot":"edge","id":"e11","vertexIds":["v4","v9"]}]},{"id":"object-void-cap","typology":"spatial.shape.primitive.box","primitives":[{"kind":"vertex","id":"v0","position":[0,0,0]},{"kind":"vertex","id":"v1","position":[5,0,0]},{"kind":"vertex","id":"v2","position":[5,4,0]},{"kind":"vertex","id":"v3","position":[0,4,0]},{"kind":"curve","id":"e0","vertexIds":["v0","v1"]},{"kind":"curve","id":"e1","vertexIds":["v1","v2"]},{"kind":"curve","id":"e2","vertexIds":["v2","v3"]},{"kind":"curve","id":"e3","vertexIds":["v3","v0"]},{"kind":"curve","id":"w-deck","edgeIds":["e0","e1","e2","e3"]},{"kind":"surface","id":"stub-surface","wireIds":["w-deck"]},{"kind":"shell","id":"shell-deck","faceIds":["stub-surface"]},{"kind":"solid","slot":"solid","id":"void-cap","shellIds":["shell-deck"]}]},{"id":"object-wire-rail-upper","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"v10","position":[4.5,1.8,2.4]},{"kind":"vertex","id":"v11","position":[3.8,3.5,2.4]},{"kind":"vertex","id":"v9","position":[2.5,0.3,2.4]},{"kind":"curve","id":"e10","vertexIds":["v10","v11"]},{"kind":"curve","id":"e9","vertexIds":["v9","v10"]},{"kind":"curve","slot":"wire","id":"rail-upper","edgeIds":["e9","e10"]}]},{"id":"object-wire-stub-wire","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"v4","position":[1,1,1.2]},{"kind":"vertex","id":"v5","position":[3,2.2,1.2]},{"kind":"vertex","id":"v6","position":[4.2,0.8,1.2]},{"kind":"vertex","id":"v7","position":[2.5,3.1,1.2]},{"kind":"vertex","id":"v8","position":[0.8,2.4,1.2]},{"kind":"curve","id":"e4","vertexIds":["v4","v5"]},{"kind":"curve","id":"e5","vertexIds":["v5","v6"]},{"kind":"curve","id":"e6","vertexIds":["v6","v7"]},{"kind":"curve","id":"e7","vertexIds":["v7","v8"]},{"kind":"curve","id":"e8","vertexIds":["v8","v4"]},{"kind":"curve","slot":"wire","id":"stub-wire","edgeIds":["e4","e5","e6","e7","e8"]}]}]}}]}';
const CAD_E2E_ROUTES_MODEL_SPACE_JSON =
  '{"schema":"spatial.modelspace","revision":1,"models":[{"id":"spatial.shape","model":{"schema":"spatial.model","revision":1,"objects":[{"id":"object-wire-orbit-a","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"r10","position":[6,0,0.8]},{"kind":"vertex","id":"r11","position":[7.4,0.6,0.8]},{"kind":"vertex","id":"r12","position":[8.8,0.2,0.8]},{"kind":"vertex","id":"r13","position":[9.9,1.1,0.8]},{"kind":"vertex","id":"r14","position":[10.2,2.6,0.8]},{"kind":"vertex","id":"r15","position":[9.4,3.9,0.8]},{"kind":"vertex","id":"r16","position":[7.8,4.4,0.8]},{"kind":"vertex","id":"r17","position":[6.2,4.1,0.8]},{"kind":"curve","id":"re10","vertexIds":["r10","r11"]},{"kind":"curve","id":"re11","vertexIds":["r11","r12"]},{"kind":"curve","id":"re12","vertexIds":["r12","r13"]},{"kind":"curve","id":"re13","vertexIds":["r13","r14"]},{"kind":"curve","id":"re14","vertexIds":["r14","r15"]},{"kind":"curve","id":"re15","vertexIds":["r15","r16"]},{"kind":"curve","id":"re16","vertexIds":["r16","r17"]},{"kind":"curve","id":"re17","vertexIds":["r17","r10"]},{"kind":"curve","slot":"wire","id":"orbit-a","edgeIds":["re10","re11","re12","re13","re14","re15","re16","re17"]}]},{"id":"object-wire-spine-b","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"r18","position":[2,6,1.6]},{"kind":"vertex","id":"r19","position":[3.5,6.8,1.6]},{"kind":"vertex","id":"r20","position":[5.2,6.5,1.6]},{"kind":"vertex","id":"r21","position":[6.8,7.2,1.6]},{"kind":"vertex","id":"r22","position":[7.5,8.4,1.6]},{"kind":"vertex","id":"r23","position":[6.1,9.1,1.6]},{"kind":"curve","id":"re18","vertexIds":["r18","r19"]},{"kind":"curve","id":"re19","vertexIds":["r19","r20"]},{"kind":"curve","id":"re20","vertexIds":["r20","r21"]},{"kind":"curve","id":"re21","vertexIds":["r21","r22"]},{"kind":"curve","id":"re22","vertexIds":["r22","r23"]},{"kind":"curve","slot":"wire","id":"spine-b","edgeIds":["re18","re19","re20","re21","re22"]}]},{"id":"object-wire-stub-wire","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"r0","position":[0,0,0]},{"kind":"vertex","id":"r1","position":[1.2,0.4,0]},{"kind":"vertex","id":"r2","position":[2.6,0.1,0]},{"kind":"vertex","id":"r3","position":[3.8,0.9,0]},{"kind":"vertex","id":"r4","position":[4.5,2.1,0]},{"kind":"vertex","id":"r5","position":[4.2,3.5,0]},{"kind":"vertex","id":"r6","position":[3.1,4.2,0]},{"kind":"vertex","id":"r7","position":[1.5,4.5,0]},{"kind":"vertex","id":"r8","position":[0.2,3.8,0]},{"kind":"vertex","id":"r9","position":[-0.4,2.2,0]},{"kind":"curve","id":"re0","vertexIds":["r0","r1"]},{"kind":"curve","id":"re1","vertexIds":["r1","r2"]},{"kind":"curve","id":"re2","vertexIds":["r2","r3"]},{"kind":"curve","id":"re3","vertexIds":["r3","r4"]},{"kind":"curve","id":"re4","vertexIds":["r4","r5"]},{"kind":"curve","id":"re5","vertexIds":["r5","r6"]},{"kind":"curve","id":"re6","vertexIds":["r6","r7"]},{"kind":"curve","id":"re7","vertexIds":["r7","r8"]},{"kind":"curve","id":"re8","vertexIds":["r8","r9"]},{"kind":"curve","id":"re9","vertexIds":["r9","r0"]},{"kind":"curve","slot":"wire","id":"stub-wire","edgeIds":["re0","re1","re2","re3","re4","re5","re6","re7","re8","re9"]}]}]}}]}';
const CAD_E2E_BUILDING_BOOLEAN_MODEL_SPACE_JSON =
  '{"schema":"spatial.modelspace","revision":1,"models":[{"id":"spatial.shape","model":{"schema":"spatial.model","revision":1,"objects":[{"id":"object-small-building-cell-123052045","typology":"spatial.shape.primitive.box","primitives":[{"kind":"vertex","id":"small-building-vertex-1027259450","position":[-64.506666,41.273333,-9.407222]},{"kind":"vertex","id":"small-building-vertex-1412665337","position":[-18.586667,41.273333,-43.847222]},{"kind":"vertex","id":"small-building-vertex-200778251","position":[-18.586667,3.553333,-9.407222]},{"kind":"vertex","id":"small-building-vertex-354458280","position":[-18.586667,3.553333,-43.847222]},{"kind":"vertex","id":"small-building-vertex-560960189","position":[-64.506666,3.553333,-9.407222]},{"kind":"vertex","id":"small-building-vertex-566043311","position":[-64.506666,41.273333,-43.847222]},{"kind":"vertex","id":"small-building-vertex-584100920","position":[-64.506666,3.553333,-43.847222]},{"kind":"vertex","id":"small-building-vertex-834828749","position":[-18.586667,41.273333,-9.407222]},{"kind":"curve","id":"small-building-edge-1660152326","vertexIds":["small-building-vertex-354458280","small-building-vertex-1412665337"]},{"kind":"curve","id":"small-building-edge-1943812986","vertexIds":["small-building-vertex-566043311","small-building-vertex-584100920"]},{"kind":"curve","id":"small-building-edge-2004107109","vertexIds":["small-building-vertex-200778251","small-building-vertex-560960189"]},{"kind":"curve","id":"small-building-edge-2075229525","vertexIds":["small-building-vertex-1027259450","small-building-vertex-566043311"]},{"kind":"curve","id":"small-building-edge-224238197","vertexIds":["small-building-vertex-584100920","small-building-vertex-560960189"]},{"kind":"curve","id":"small-building-edge-229106015","vertexIds":["small-building-vertex-1027259450","small-building-vertex-834828749"]},{"kind":"curve","id":"small-building-edge-278123677","vertexIds":["small-building-vertex-584100920","small-building-vertex-354458280"]},{"kind":"curve","id":"small-building-edge-278867947","vertexIds":["small-building-vertex-834828749","small-building-vertex-200778251"]},{"kind":"curve","id":"small-building-edge-332998341","vertexIds":["small-building-vertex-200778251","small-building-vertex-354458280"]},{"kind":"curve","id":"small-building-edge-757634469","vertexIds":["small-building-vertex-1412665337","small-building-vertex-834828749"]},{"kind":"curve","id":"small-building-edge-779379499","vertexIds":["small-building-vertex-1412665337","small-building-vertex-566043311"]},{"kind":"curve","id":"small-building-edge-951546977","vertexIds":["small-building-vertex-560960189","small-building-vertex-1027259450"]},{"kind":"curve","id":"small-building-wire-1366152152","edgeIds":["small-building-edge-278123677","small-building-edge-1943812986","small-building-edge-779379499","small-building-edge-1660152326"]},{"kind":"curve","id":"small-building-wire-1559546061","edgeIds":["small-building-edge-2075229525","small-building-edge-229106015","small-building-edge-757634469","small-building-edge-779379499"]},{"kind":"curve","id":"small-building-wire-1742634236","edgeIds":["small-building-edge-278867947","small-building-edge-332998341","small-building-edge-1660152326","small-building-edge-757634469"]},{"kind":"curve","id":"small-building-wire-456551683","edgeIds":["small-building-edge-1943812986","small-building-edge-224238197","small-building-edge-951546977","small-building-edge-2075229525"]},{"kind":"curve","id":"small-building-wire-515231130","edgeIds":["small-building-edge-224238197","small-building-edge-278123677","small-building-edge-332998341","small-building-edge-2004107109"]},{"kind":"curve","id":"small-building-wire-978200956","edgeIds":["small-building-edge-951546977","small-building-edge-2004107109","small-building-edge-278867947","small-building-edge-229106015"]},{"kind":"surface","id":"small-building-face-1071813579","wireIds":["small-building-wire-1559546061"]},{"kind":"surface","id":"small-building-face-1198070201","wireIds":["small-building-wire-456551683"]},{"kind":"surface","id":"small-building-face-1321487947","wireIds":["small-building-wire-1742634236"]},{"kind":"surface","id":"small-building-face-1833451572","wireIds":["small-building-wire-1366152152"]},{"kind":"surface","id":"small-building-face-383803774","wireIds":["small-building-wire-515231130"]},{"kind":"surface","id":"small-building-face-624717229","wireIds":["small-building-wire-978200956"]},{"kind":"shell","id":"small-building-shell-319815043","faceIds":["small-building-face-1198070201","small-building-face-1833451572","small-building-face-383803774","small-building-face-624717229","small-building-face-1071813579","small-building-face-1321487947"]},{"kind":"solid","slot":"solid","id":"small-building-cell-123052045","shellIds":["small-building-shell-319815043"]}]},{"id":"object-small-building-cell-1278694563","typology":"spatial.shape.primitive.box","primitives":[{"kind":"vertex","id":"small-building-vertex-1052483923","position":[-18.586667,-57.126666,-9.407222]},{"kind":"vertex","id":"small-building-vertex-1078954806","position":[-64.506666,-57.126666,-9.407222]},{"kind":"vertex","id":"small-building-vertex-1417750768","position":[-18.586667,3.553333,54.142777]},{"kind":"vertex","id":"small-building-vertex-1487235108","position":[-64.506666,-57.126666,25.852777]},{"kind":"vertex","id":"small-building-vertex-1653716766","position":[-64.506666,3.553333,54.142777]},{"kind":"vertex","id":"small-building-vertex-1928378833","position":[-64.506666,-7.926666,62.752777]},{"kind":"vertex","id":"small-building-vertex-200778251","position":[-18.586667,3.553333,-9.407222]},{"kind":"vertex","id":"small-building-vertex-551332595","position":[-18.586667,-7.926666,62.752777]},{"kind":"vertex","id":"small-building-vertex-560960189","position":[-64.506666,3.553333,-9.407222]},{"kind":"vertex","id":"small-building-vertex-945778871","position":[-18.586667,-57.126666,25.852777]},{"kind":"curve","id":"small-building-edge-1070453701","vertexIds":["small-building-vertex-945778871","small-building-vertex-1052483923"]},{"kind":"curve","id":"small-building-edge-1108121009","vertexIds":["small-building-vertex-945778871","small-building-vertex-1487235108"]},{"kind":"curve","id":"small-building-edge-1261178177","vertexIds":["small-building-vertex-560960189","small-building-vertex-1653716766"]},{"kind":"curve","id":"small-building-edge-1422159112","vertexIds":["small-building-vertex-1078954806","small-building-vertex-1487235108"]},{"kind":"curve","id":"small-building-edge-1567786765","vertexIds":["small-building-vertex-551332595","small-building-vertex-945778871"]},{"kind":"curve","id":"small-building-edge-1705326756","vertexIds":["small-building-vertex-1928378833","small-building-vertex-551332595"]},{"kind":"curve","id":"small-building-edge-2004107109","vertexIds":["small-building-vertex-200778251","small-building-vertex-560960189"]},{"kind":"curve","id":"small-building-edge-2102926252","vertexIds":["small-building-vertex-1928378833","small-building-vertex-1653716766"]},{"kind":"curve","id":"small-building-edge-349583852","vertexIds":["small-building-vertex-1052483923","small-building-vertex-1078954806"]},{"kind":"curve","id":"small-building-edge-354613623","vertexIds":["small-building-vertex-1487235108","small-building-vertex-1928378833"]},{"kind":"curve","id":"small-building-edge-432705901","vertexIds":["small-building-vertex-200778251","small-building-vertex-1052483923"]},{"kind":"curve","id":"small-building-edge-432807106","vertexIds":["small-building-vertex-1417750768","small-building-vertex-200778251"]},{"kind":"curve","id":"small-building-edge-467629952","vertexIds":["small-building-vertex-1078954806","small-building-vertex-560960189"]},{"kind":"curve","id":"small-building-edge-49481349","vertexIds":["small-building-vertex-1417750768","small-building-vertex-551332595"]},{"kind":"curve","id":"small-building-edge-508083477","vertexIds":["small-building-vertex-1417750768","small-building-vertex-1653716766"]},{"kind":"curve","id":"small-building-wire-1095357825","edgeIds":["small-building-edge-467629952","small-building-edge-1422159112","small-building-edge-354613623","small-building-edge-2102926252","small-building-edge-1261178177"]},{"kind":"curve","id":"small-building-wire-1399618711","edgeIds":["small-building-edge-349583852","small-building-edge-432705901","small-building-edge-2004107109","small-building-edge-467629952"]},{"kind":"curve","id":"small-building-wire-1676387167","edgeIds":["small-building-edge-1070453701","small-building-edge-432705901","small-building-edge-432807106","small-building-edge-49481349","small-building-edge-1567786765"]},{"kind":"curve","id":"small-building-wire-1755121565","edgeIds":["small-building-edge-2004107109","small-building-edge-432807106","small-building-edge-508083477","small-building-edge-1261178177"]},{"kind":"curve","id":"small-building-wire-285005499","edgeIds":["small-building-edge-349583852","small-building-edge-1070453701","small-building-edge-1108121009","small-building-edge-1422159112"]},{"kind":"curve","id":"small-building-wire-311748032","edgeIds":["small-building-edge-354613623","small-building-edge-1108121009","small-building-edge-1567786765","small-building-edge-1705326756"]},{"kind":"curve","id":"small-building-wire-924227310","edgeIds":["small-building-edge-49481349","small-building-edge-508083477","small-building-edge-2102926252","small-building-edge-1705326756"]},{"kind":"surface","id":"small-building-face-1144073303","wireIds":["small-building-wire-1755121565"]},{"kind":"surface","id":"small-building-face-1382526041","wireIds":["small-building-wire-1095357825"]},{"kind":"surface","id":"small-building-face-1891411219","wireIds":["small-building-wire-1399618711"]},{"kind":"surface","id":"small-building-face-2019874201","wireIds":["small-building-wire-311748032"]},{"kind":"surface","id":"small-building-face-2129815768","wireIds":["small-building-wire-924227310"]},{"kind":"surface","id":"small-building-face-512606747","wireIds":["small-building-wire-285005499"]},{"kind":"surface","id":"small-building-face-816291467","wireIds":["small-building-wire-1676387167"]},{"kind":"shell","id":"small-building-shell-115098816","faceIds":["small-building-face-816291467","small-building-face-512606747","small-building-face-1891411219","small-building-face-1144073303","small-building-face-2129815768","small-building-face-2019874201","small-building-face-1382526041"]},{"kind":"solid","slot":"solid","id":"small-building-cell-1278694563","shellIds":["small-building-shell-115098816"]}]}]}}]}';

if (import.meta.vitest) {
  __artifactTestRuntime!.bootstrapCadModules();
  const { BrepjsKernel, preciseSpatialKernelMath } = __artifactTestKernel!;
  const geometryLoomFixtureJson = JSON.parse(CAD_E2E_LOOM_MODEL_SPACE_JSON) as ModelSpaceJson;
  const geometryRoutesFixtureJson = JSON.parse(CAD_E2E_ROUTES_MODEL_SPACE_JSON) as ModelSpaceJson;
  const buildingBooleanFixtureJson = JSON.parse(CAD_E2E_BUILDING_BOOLEAN_MODEL_SPACE_JSON) as ModelSpaceJson;
  const M = preciseSpatialKernelMath;
  const { describe, expect, it } = import.meta.vitest;

  describe("@semio-tech/cad-js/core interactions", () => {
    async function bootTransformSelection(rt: InteractionRuntime, targets: readonly SelectionTarget[]): Promise<void> {
      await rt.send({ kind: "start", modifiers: {} });
      if (targets.length === 0) return;
      await rt.send({ kind: "selection.changed", targets: [...targets], modifiers: {} });
      await rt.send({ kind: "confirm", modifiers: {} });
    }

    it("auto-commits curve.arc as one arc edge between start and end", async () => {
      const model = new Model();
      class ArcKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
        async executeCommandDiff(commandId: string, ctx: Record<string, unknown>) {
          if (commandId !== "curve.arc") return { diff: EMPTY_MODEL_DIFF };
          const center = (Array.isArray(ctx.center) ? ctx.center : [0, 0, 0]) as unknown as Vec3;
          const start = (Array.isArray(ctx.start) ? ctx.start : [1, 0, 0]) as unknown as Vec3;
          const end = M.arcEndOnCircle(center, start, (Array.isArray(ctx.end) ? ctx.end : start) as unknown as Vec3);
          const v0 = "v0" as VertexRef;
          const v1 = "v1" as VertexRef;
          const e = "e0" as EdgeRef;
          const w = "w0" as WireRef;
          return {
            diff: {
              vertices: {
                added: [
                  { id: v0, position: start },
                  { id: v1, position: end },
                ],
              },
              edges: { added: [{ id: e, vertexIds: [v0, v1], curve: { kind: "arc" as const, center } }] },
              wires: { added: [{ id: w, edgeIds: [e] }] },
            },
          };
        }
      }
      const spec = loadSpatialInteraction("curve.arc")!;
      const rt = createInteractionRuntime(spec, {
        kernel: new ArcKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
      });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [2, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [0, 2, 0] as Vec3, modifiers: {} });
      const snap = rt.getSnapshot();
      expect(snap.state).toBe("committed");
      expect(snap.lastResponse?.ok).toBe(true);
      const edges = Object.values(model.edges);
      expect(edges).toHaveLength(1);
      expect(edges[0]!.curve).toEqual({ kind: "arc", center: [0, 0, 0] });
      expect(Object.keys(model.vertices)).toHaveLength(2);
    });
    it("normalizes commit fromStates to committed for scripted commands without ready", () => {
      const spec = loadSpatialInteraction("curve.arc")!;
      expect(spec.commit.fromStates).toEqual(["committed"]);
    });
    it("transform.move vertical mode changes Z only", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const v0 = Object.keys(model.vertices)[0]!;
      const p0 = model.vertices[v0]!.position;
      class CommandKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
        async executeCommandDiff() {
          return { diff: EMPTY_MODEL_DIFF };
        }
      }
      const spec = loadSpatialInteraction("transform.move")!;
      const rt = createInteractionRuntime(spec, {
        kernel: new CommandKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
      });
      await bootTransformSelection(rt, [{ kind: "vertex", id: v0, editable: true }]);
      await rt.send({ kind: "pointer.down", point: p0, modifiers: {} });
      await rt.send({ kind: "mode.vertical", modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [p0[0] + 5, p0[1] + 4, p0[2] + 2], modifiers: {} });
      expect(rt.getSnapshot().lastResponse?.ok).toBe(true);
      expect(model.vertices[v0]!.position).toEqual([p0[0], p0[1], p0[2] + 2]);
    });
    it("transform.move confirm without pick uses selection bbox center", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [2, 0, 0], height: 0 }, solidRef("box")));
      const verts = Object.values(model.vertices);
      class CommandKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
        async executeCommandDiff() {
          return { diff: EMPTY_MODEL_DIFF };
        }
      }
      const spec = loadSpatialInteraction("transform.move")!;
      const rt = createInteractionRuntime(spec, {
        kernel: new CommandKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
      });
      await bootTransformSelection(
        rt,
        verts.map((v) => ({ kind: "vertex" as const, id: v.id, editable: true })),
      );
      expect(rt.getSnapshot().state).toBe("point_to_move_from");
      await rt.send({ kind: "confirm", modifiers: {} });
      const from = rt.getSnapshot().context.from as Vec3;
      expect(from[0]).toBeCloseTo(1, 5);
      expect(from[1]).toBeCloseTo(0, 5);
    });
    it("auto-finalizes transform.move on terminal pointer down without alreadyCommitted", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const v0 = Object.keys(model.vertices)[0]!;
      const p0 = model.vertices[v0]!.position;
      class CommandKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
        async executeCommandDiff() {
          return { diff: EMPTY_MODEL_DIFF };
        }
      }
      const spec = loadSpatialInteraction("transform.move")!;
      const rt = createInteractionRuntime(spec, {
        kernel: new CommandKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
      });
      await bootTransformSelection(rt, [{ kind: "vertex", id: v0, editable: true }]);
      await rt.send({ kind: "pointer.down", point: p0, modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [p0[0] + 2, p0[1] + 1, p0[2]], modifiers: {} });
      const snap = rt.getSnapshot();
      expect(snap.state).toBe("committed");
      expect(snap.lastResponse?.ok).toBe(true);
      expect(snap.lastResponse?.errors).toEqual([]);
      expect(snap.lastResponse?.diff?.vertices?.modified?.length).toBeGreaterThan(0);
      expect(model.vertices[v0]!.position).toEqual([p0[0] + 2, p0[1] + 1, p0[2]]);
    });

    it("transform.copy action constrains vertical delta to Z only", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("e2e-box")));
      const actions = modelDefinitionActionRegistry();
      const from: Vec3 = [0, 0, 0];
      const r = await runRegisteredAction(actions,
        "transform.copy",
        {
          targets: [{ kind: "solid", id: "e2e-box", editable: true }],
          from,
          to: [5, 4, 2],
          moveMode: "vertical",
        },
        { model: model, kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M },
      );
      const added = r.diff?.vertices?.added ?? [];
      const originals = Object.values(model.vertices);
      expect(added.length).toBe(8);
      for (const v of added) {
        expect(originals.some((o) => Math.abs(v.position[0] - o.position[0]) < 1e-5 && Math.abs(v.position[1] - o.position[1]) < 1e-5 && Math.abs(v.position[2] - o.position[2] - 2) < 1e-5)).toBe(true);
        expect(originals.some((o) => Math.abs(v.position[0] - o.position[0] - 5) < 1e-5 && Math.abs(v.position[1] - o.position[1] - 4) < 1e-5 && Math.abs(v.position[2] - o.position[2] - 2) < 1e-5)).toBe(false);
      }
    });

    it("transform.copy session keeps vertical moveMode through pick workflow", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("e2e-box")));
      const before = Object.keys(model.vertices).length;
      const spec = loadSpatialInteraction("transform.copy")!;
      const rt = createInteractionRuntime(spec, {
        kernel: new BrepjsKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
      });
      const from = model.vertices[Object.keys(model.vertices)[0]!]!.position;
      await bootTransformSelection(rt, [{ kind: "solid", id: "e2e-box", editable: true }]);
      await rt.send({ kind: "mode.vertical", modifiers: {} });
      await rt.send({ kind: "pointer.down", point: from, modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [from[0] + 5, from[1] + 4, from[2] + 2], modifiers: {} });
      const snap = rt.getSnapshot();
      expect(snap.context.moveMode).toBe("vertical");
      expect(snap.state).toBe("committed");
      expect(snap.lastResponse?.ok).toBe(true);
      expect(Object.keys(model.vertices).length).toBeGreaterThan(before);
    });

    it("transform.copy confirm without from pick uses selection bbox center", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [2, 2, 0], height: 1 }, solidRef("e2e-box")));
      const spec = loadSpatialInteraction("transform.copy")!;
      const rt = createInteractionRuntime(spec, {
        kernel: new BrepjsKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
      });
      await bootTransformSelection(rt, [{ kind: "solid", id: "e2e-box", editable: true }]);
      await rt.send({ kind: "confirm", modifiers: {} });
      const from = rt.getSnapshot().context.from as Vec3;
      expect(from[0]).toBeCloseTo(1, 5);
      expect(from[1]).toBeCloseTo(1, 5);
      await rt.send({ kind: "pointer.down", point: [from[0] + 1, from[1], from[2]], modifiers: {} });
      expect(rt.getSnapshot().state).toBe("committed");
      expect(rt.getSnapshot().lastResponse?.ok).toBe(true);
    });
  });
  describe("@semio-tech/cad-js/core action and interaction registries", () => {
    it("rejects executable action document fields", () => {
      const base = {
        schema: "spatial.action",
        id: "x",
        version: "1.0.0",
        steps: [{ operation: "return", data: { kind: "const", value: 1 } }],
      };
      expect(parseActionSpec({ ...base, run: "x" })).toBeNull();
      expect(parseActionSpec({ ...base, code: "x" })).toBeNull();
      expect(parseActionSpec({ ...base, function: "x" })).toBeNull();
      expect(parseActionSpec({ ...base, steps: [{ operation: "eval", code: "x" }] })).toBeNull();
    });
    it("loads model-definition actions from data-only JSON specs", () => {
      const specs = listModelDefinitionActionSpecs();
      const registry = modelDefinitionActionRegistry();
      expect(specs.length).toBeGreaterThan(0);
      expect(specs.every((s) => registry.get(s.id)?.spec?.schema === "spatial.action")).toBe(true);
      expect(specs.every((s) => registry.get(s.id) !== null)).toBe(true);
      expect(registry.get("command.finish")?.spec?.schema).toBe("spatial.action");
      expect(registry.get("selection.selectAll")?.spec?.steps.some((s) => s.operation === "kernel.call" && s.function === "spatial.selection.apply")).toBe(true);
      expect(registry.get("command.addPoint")?.spec?.steps.some((s) => s.operation === "kernel.call" && s.function === "spatial.action.capability")).toBe(true);
      const allowedKernelFunctions = new Set(["spatial.selection.apply", "spatial.action.capability"]);
      expect(specs.every((spec) => spec.steps.every((step) => step.operation !== "kernel.call" || allowedKernelFunctions.has(step.function)))).toBe(true);
    });
    it("typology actions reference shipped declarative action specs", () => {
      const actionIds = new Set(listModelDefinitionActionSpecs().map((row) => row.id));
      for (const typology of listModelDefinitionTypologies()) {
        for (const actionId of typology.actions) {
          expect(actionIds.has(actionId), `${typology.id} → ${actionId}`).toBe(true);
        }
      }
    });
    it("every typology ships construct kit or legacy create interactions", () => {
      const actionIds = new Set(listModelDefinitionActionSpecs().map((row) => row.id));
      const interactionIds = new Set(
        listInteractionSpecs(modelDefinitionInteractionRegistry())
          .map((row) => row.id),
      );
      for (const typology of listModelDefinitionTypologies()) {
        if (typology.id.includes(".kernel.")) continue;
        const ids = typologyConstructAssetIds(typology.id, typology.label);
        if (!typology.interactions.includes(ids.construct)) {
          expect(typology.actions.length).toBeGreaterThan(0);
          expect(typology.interactions.length).toBeGreaterThan(0);
          continue;
        }
        expect(typologyHasNativeConstructKit(typology)).toBe(true);
        expect(typology.interactions).toEqual([ids.interaction]);
        expect(typology.actions).not.toContain(ids.interaction);
        for (const actionId of typologyConstructModeActionIds(typology.id, typology.label)) {
          expect(typology.actions).toContain(actionId);
          expect(actionIds.has(actionId)).toBe(true);
        }
        expect(interactionIds.has(ids.interaction)).toBe(true);
        const commitAction = loadSpatialInteraction(ids.interaction)?.commit.operation.action;
        expect(typologyConstructModeActionIds(typology.id, typology.label)).toContain(commitAction);
        expect(commitAction).not.toBe(ids.interaction);
      }
    });
    it("every model-definition typology is constructable with native assets in its folder", () => {
      for (const manifest of listModelDefinitionManifests()) {
        const mdId = manifest.id;
        if (isShapeModelDefinition(mdId)) continue;
        for (const typology of listTypologiesForModelDefinition(mdId)) {
          if (!typologyHasNativeConstructKit(typology)) continue;
          const ids = typologyConstructAssetIds(typology.id, typology.label);
          expect(modelDefinitionIdForInteraction(ids.interaction), `${mdId} → ${ids.interaction}`).toBe(mdId);
          for (const actionId of typologyConstructModeActionIds(typology.id, typology.label)) {
            expect(actionOwnedByModelDefinition(actionId, mdId), `${mdId} → ${typology.id} → ${actionId}`).toBe(true);
          }
          for (const actionId of typology.actions) {
            expect(actionOwnedByModelDefinition(actionId, mdId), `${mdId} → ${typology.id} → ${actionId}`).toBe(true);
          }
        }
        expect(listConstructableTypologiesForModelDefinition(mdId).length).toBe(listTypologiesForModelDefinition(mdId).filter(typologyHasNativeConstructKit).length);
      }
    });
    it("typology constructFrom2PointsAndHeight adds an object row for the typology", async () => {
      const model = new Model();
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const typology = "energy.energy.hull";
      const ids = typologyConstructAssetIds(typology, "Hull");
      await runRegisteredAction(modelDefinitionActionRegistry(),
        ids.constructFrom2PointsAndHeight,
        { typology, constructMode: "2PointsAndHeight", pointA: [0, 0, 0], pointB: [3, 2, 0], height: 2.5 },
        { kernel, preview: kernel as unknown as SpatialPreviewKernel, model, activeModelDefinitionId: "aec.building.energy" },
      );
      expect(model.objects[typology]?.typology).toBe(typology);
      expect(model.objects[typology]?.primitives.solid).toBeTruthy();
    });
    it("curve.interpolateCurve rolls back from committed when kernel returns empty commit diff", async () => {
      const spec = loadSpatialInteraction("curve.interpolateCurve")!;
      const model = new Model();
      const kernel = {
        executeCommandDiff: async () => ({ diff: {} }),
      } as unknown as SpatialKernel;
      const rt = createInteractionRuntime(spec, {
        kernel,
        previewKernel: kernel as unknown as SpatialPreviewKernel,
        document: { model, nodes: [] },
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0], modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [2, 1, 0], modifiers: {} });
      await rt.send({ kind: "confirm", modifiers: {} });
      expect(rt.getSnapshot().state).toBe("next_point");
      expect(rt.getSnapshot().lastResponse?.ok).toBe(false);
      expect(rt.getSnapshot().lastResponse?.errors?.[0]?.code).toBe("interaction.emptyCommit");
    });

    it("interactionStepFinalizeEvent confirms interpolate curve with two points instead of pointer.down", () => {
      const spec = loadSpatialInteraction("curve.interpolateCurve")!;
      const ctx = { points: [[0, 0, 0] as Vec3, [2, 1, 0] as Vec3], cursor: [5, 5, 0] as Vec3 };
      expect(interactionStepFinalizeEvent(spec, "next_point", ctx, M)?.kind).toBe("confirm");
      expect(interactionNumericEntryCommitEvent(spec, "next_point", ctx, M)?.kind).toBe("pointer.down");
    });

    it("interactionNumericEntryExplicitLockValue ignores live rubber-band distance", () => {
      const spec = loadSpatialInteraction("curve.interpolateCurve")!;
      const ctx = { points: [[0, 0, 0] as Vec3], cursor: [4, 0, 0] as Vec3, __lengthLock: 2 };
      expect(interactionNumericEntryExplicitLockValue(spec, "next_point", ctx)).toBe(2);
      const liveOnly = { points: [[0, 0, 0] as Vec3], cursor: [4, 0, 0] as Vec3 };
      expect(interactionNumericEntryExplicitLockValue(spec, "next_point", liveOnly)).toBeNull();
      expect(interactionNumericEntryLockedValue(spec, "next_point", liveOnly)).toBeCloseTo(4, 5);
    });

    it("curve.interpolateCurve confirm with one point stays in next_point", async () => {
      const spec = loadSpatialInteraction("curve.interpolateCurve")!;
      const model = new Model();
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const rt = createInteractionRuntime(spec, {
        kernel,
        document: { model, nodes: [] },
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0], modifiers: {} });
      await rt.send({ kind: "confirm", modifiers: {} });
      expect(rt.getSnapshot().state).toBe("next_point");
      expect(Object.keys(model.edges).length).toBe(0);
    });

    it("curve.interpolateCurve commit binds typology object rows for document", async () => {
      const typology = "spatial.shape.curve.interpolate-curve";
      expect(typologyIdForInteractionCommit("curve.interpolateCurve")).toBe(typology);
      const spec = loadSpatialInteraction("curve.interpolateCurve")!;
      const model = new Model();
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const rt = createInteractionRuntime(spec, {
        kernel,
        document: { model, nodes: [] },
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0], modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [2, 1, 0], modifiers: {} });
      await rt.send({ kind: "confirm", modifiers: {} });
      const res = rt.getSnapshot().lastResponse!;
      expect(res.ok).toBe(true);
      expect(Object.keys(model.edges).length).toBeGreaterThan(0);
      expect(listModelObjectsForModelDefinition(model, defaultModelDefinitionId())).toHaveLength(1);
      expect(model.objects[typology as ObjectRef]?.typology).toBe(typology);
      expect(model.objects[typology as ObjectRef]?.primitives.curve).toBeTruthy();
    });

    it("curve.interpolateCurve commit uses live snapped vertex positions after vertex move", async () => {
      const spec = loadSpatialInteraction("curve.interpolateCurve")!;
      const model = new Model();
      const vertexId = "v-live" as VertexRef;
      model.vertices[vertexId] = { id: vertexId, position: [0, 0, 0] };
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const rt = createInteractionRuntime(spec, {
        kernel,
        document: { model, nodes: [] },
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      await rt.send({
        kind: "pointer.down",
        point: [0, 0, 0],
        modifiers: {},
        snap: { kind: "vertex", id: vertexId, point: [0, 0, 0] },
      } as InteractionEvent);
      await rt.send({ kind: "pointer.down", point: [2, 0, 0], modifiers: {} });
      model.vertices[vertexId] = { id: vertexId, position: [0, 4, 0] };
      await rt.send({ kind: "confirm", modifiers: {} });
      const edge = Object.values(model.edges)[0];
      expect(edge?.curve?.kind).toBe("nurbs");
      if (edge?.curve?.kind === "nurbs") {
        expect(edge.curve.poles[0]).toEqual([0, 4, 0]);
      }
    });

    it("applyModelDiff syncs nurbs through-curve poles when endpoint vertices move", async () => {
      const model = new Model();
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const created = await kernel.executeCommandDiff("curve.interpolateCurve", {
        model,
        points: [
          [0, 0, 0],
          [2, 1, 0],
          [4, 0, 0],
        ],
      });
      applyModelDiff(model, created.diff);
      const edge = Object.values(model.edges)[0]!;
      const startId = edge.vertexIds[0]!;
      applyModelDiff(model, { vertices: { modified: [{ id: startId, position: [0, 3, 0] }] } });
      const updated = model.edges[edge.id]!;
      expect(updated.curve?.kind).toBe("nurbs");
      if (updated.curve?.kind === "nurbs") {
        expect(updated.curve.poles[0]).toEqual([0, 3, 0]);
      }
    });

    it("selectionTargetsPointTransformDiff moves all nurbs poles when an edge is selected", async () => {
      const model = new Model();
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const created = await kernel.executeCommandDiff("curve.controlPointCurve", {
        model,
        points: [
          [0, 0, 0],
          [1, 2, 0],
          [3, 0, 0],
        ],
      });
      applyModelDiff(model, created.diff);
      const edge = Object.values(model.edges)[0]!;
      const before: readonly Vec3[] = edge.curve?.kind === "nurbs" ? edge.curve.poles.map((pole: Vec3) => [...pole] as Vec3) : [];
      applyModelDiff(
        model,
        selectionTargetsPointTransformDiff(model, [{ kind: "edge", id: edge.id, editable: true }], (point) => [point[0] + 1, point[1], point[2]]),
      );
      const updated = model.edges[edge.id]!;
      expect(updated.curve?.kind).toBe("nurbs");
      if (updated.curve?.kind === "nurbs") {
        expect(updated.curve.poles).toEqual(before.map((pole: Vec3) => [pole[0] + 1, pole[1], pole[2]]));
      }
    });

    it("selectionTargetsPointTransformDiff leaves interior poles when only a vertex is selected", async () => {
      const model = new Model();
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const created = await kernel.executeCommandDiff("curve.controlPointCurve", {
        model,
        points: [
          [0, 0, 0],
          [1, 2, 0],
          [3, 0, 0],
        ],
      });
      applyModelDiff(model, created.diff);
      const edge = Object.values(model.edges)[0]!;
      const midPole = edge.curve?.kind === "nurbs" ? [...edge.curve.poles[1]!] : null;
      const startId = edge.vertexIds[0]!;
      applyModelDiff(
        model,
        selectionTargetsPointTransformDiff(model, [{ kind: "vertex", id: startId, editable: true }], (point) => [point[0], point[1] + 5, point[2]]),
      );
      const updated = model.edges[edge.id]!;
      expect(updated.curve?.kind).toBe("nurbs");
      if (updated.curve?.kind === "nurbs" && midPole) {
        expect(updated.curve.poles[1]).toEqual(midPole);
      }
    });

    it("primitive.box commit binds typology object rows for document", async () => {
      const typology = "spatial.shape.primitive.box";
      const spec = loadSpatialInteraction("primitive.box")!;
      const model = new Model();
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const rt = createInteractionRuntime(spec, {
        kernel,
        document: { model, nodes: [] },
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0], modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [2, 3, 0], modifiers: {} });
      await rt.send({ kind: "set.height", value: 4, modifiers: {} });
      await rt.send({ kind: "confirm", modifiers: {} });
      const res = rt.getSnapshot().lastResponse!;
      expect(res.ok).toBe(true);
      expect(listModelObjectsForModelDefinition(model, defaultModelDefinitionId())).toHaveLength(1);
      expect(model.objects[typology as ObjectRef]?.typology).toBe(typology);
      expect(model.objects[typology as ObjectRef]?.primitives.solid).toBeTruthy();
      const rt2 = createInteractionRuntime(spec, {
        kernel,
        document: { model, nodes: [] },
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      await rt2.send({ kind: "pointer.down", point: [5, 0, 0], modifiers: {} });
      await rt2.send({ kind: "pointer.down", point: [7, 2, 0], modifiers: {} });
      await rt2.send({ kind: "set.height", value: 2, modifiers: {} });
      await rt2.send({ kind: "confirm", modifiers: {} });
      expect(rt2.getSnapshot().lastResponse?.ok).toBe(true);
      expect(listModelObjectsForModelDefinition(model, defaultModelDefinitionId())).toHaveLength(2);
    });
    it("typologyConstructCommitActionForMode resolves exactly one mode construct action", () => {
      const ids = typologyConstructAssetIds("energy.energy.hull", "Hull");
      const kit = typologyConstructKitByInteraction().get(ids.interaction)!;
      expect(typologyConstructCommitActionForMode(kit, "2PointsAndHeight")).toBe(ids.constructFrom2PointsAndHeight);
      expect(typologyConstructCommitActionForMode(kit, "curveAndHeight")).toBe(ids.constructFromCurveAndHeight);
      expect(typologyConstructCommitActionForMode(kit, "surface")).toBe(ids.constructFromSurface);
    });
    it("base plate typology lists only constructFromSurface among mode actions", () => {
      const typology = loadTypology("energy.energy.baseplate")!;
      const ids = typologyConstructAssetIds(typology.id, typology.label);
      expect(typologyConstructModeActionIds(typology.id, typology.label)).toEqual([ids.constructFromSurface]);
      expect(typologyHasNativeConstructKit(typology)).toBe(true);
    });
    it("aborting nested interaction.call rolls back the calling transition", async () => {
      const pickChild = parseInteractionSpec({
        schema: "spatial.interaction",
        id: "test.nested.pick",
        version: "1",
        invocation: "callable",
        machine: {
          initial: "pick",
          states: [
            {
              name: "pick",
              selection: { accept: ["face"], multiple: false, prompt: "Pick surface" },
              on: [
                {
                  event: "selection.changed",
                  transitions: [
                    {
                      target: "committed",
                      effects: [
                        {
                          mutation: "assign",
                          target: { root: "context", segments: [{ kind: "field", name: "faceId" }] },
                          value: {
                            kind: "path",
                            root: "event",
                            segments: [
                              { kind: "field", name: "targets" },
                              { kind: "index", index: 0 },
                              { kind: "field", name: "id" },
                            ],
                          },
                        },
                      ],
                    },
                  ],
                },
              ],
            },
            { name: "committed", final: true },
          ],
        },
        commit: { fromStates: ["committed"], operation: { kind: "action", action: "command.finish", params: {} } },
      })!;
      const host = parseInteractionSpec({
        schema: "spatial.interaction",
        id: "test.nested.host",
        version: "1",
        machine: {
          initial: "choose_mode",
          states: [
            {
              name: "choose_mode",
              on: [
                {
                  event: "mode.surface",
                  transitions: [
                    {
                      target: "committed",
                      effects: [
                        { mutation: "assign", target: { root: "context", segments: [{ kind: "field", name: "constructMode" }] }, value: { kind: "const", value: "surface" } },
                        {
                          mutation: "interaction.call",
                          interaction: "test.nested.pick",
                          outputs: [
                            {
                              target: { root: "context", segments: [{ kind: "field", name: "faceId" }] },
                              value: { kind: "path", root: "context", segments: [{ kind: "field", name: "faceId" }] },
                            },
                          ],
                        },
                      ],
                    },
                  ],
                },
              ],
            },
            { name: "committed", final: true },
          ],
        },
        commit: { fromStates: ["committed"], operation: { kind: "action", action: "command.finish", params: {} } },
      })!;
      let interactions = modelDefinitionInteractionRegistry();
      interactions = registerInteractionSpec(interactions, compileInteraction(pickChild));
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const rt = createInteractionRuntime(compileInteraction(host), {
        kernel,
        document: { model: new Model(), nodes: [] },
        interactions,
      });
      await rt.send({ kind: "mode.surface" });
      expect(rt.getSnapshot().interactionId).toBe("test.nested.pick");
      rt.cancel();
      const snap = rt.getSnapshot();
      expect(snap.interactionId).toBe("test.nested.host");
      expect(snap.state).toBe("choose_mode");
      expect(snap.context.constructMode).toBeUndefined();
      expect(snap.context.faceId).toBeUndefined();
    });
    it("mergeInteractionCallOutputs maps child context through PathTarget bindings", () => {
      const host: Record<string, unknown> = {};
      mergeInteractionCallOutputs(host, { faceId: "f-42", extra: 1 }, [
        {
          target: { root: "context", segments: [{ kind: "field", name: "faceId" }] },
          value: { kind: "path", root: "context", segments: [{ kind: "field", name: "faceId" }] },
        },
      ]);
      expect(host.faceId).toBe("f-42");
    });
    it("interaction.call supports arbitrarily nested interaction sessions", async () => {
      const grandchild = parseInteractionSpec({
        schema: "spatial.interaction",
        id: "test.pick.grandchild",
        version: "1",
        invocation: "callable",
        machine: {
          initial: "go",
          states: [
            { name: "go", on: [{ event: "confirm", transitions: [{ target: "done" }] }] },
            { name: "done", final: true },
          ],
        },
        commit: {
          fromStates: ["done"],
          operation: { kind: "action", action: "command.finish", params: { commandId: { kind: "const", value: "curve.line" } } },
        },
      })!;
      const child = parseInteractionSpec({
        schema: "spatial.interaction",
        id: "test.pick.child",
        version: "1",
        invocation: "callable",
        machine: {
          initial: "call",
          states: [
            {
              name: "call",
              on: [
                {
                  event: "confirm",
                  transitions: [
                    {
                      target: "done",
                      effects: [
                        {
                          mutation: "interaction.call",
                          interaction: "test.pick.grandchild",
                          outputs: [
                            {
                              target: { root: "context", segments: [{ kind: "field", name: "token" }] },
                              value: { kind: "const", value: "ok" },
                            },
                          ],
                        },
                      ],
                    },
                  ],
                },
              ],
            },
            { name: "done", final: true },
          ],
        },
        commit: {
          fromStates: ["done"],
          operation: { kind: "action", action: "command.finish", params: { commandId: { kind: "const", value: "curve.line" } } },
        },
      })!;
      const host = parseInteractionSpec({
        schema: "spatial.interaction",
        id: "test.pick.host",
        version: "1",
        machine: {
          initial: "call",
          states: [
            {
              name: "call",
              on: [
                {
                  event: "go",
                  transitions: [
                    {
                      target: "done",
                      effects: [
                        {
                          mutation: "interaction.call",
                          interaction: "test.pick.child",
                          outputs: [
                            {
                              target: { root: "context", segments: [{ kind: "field", name: "token" }] },
                              value: { kind: "path", root: "context", segments: [{ kind: "field", name: "token" }] },
                            },
                          ],
                        },
                      ],
                    },
                  ],
                },
              ],
            },
            { name: "done", final: true },
          ],
        },
        commit: {
          fromStates: ["done"],
          operation: { kind: "action", action: "command.finish", params: { commandId: { kind: "const", value: "curve.line" } } },
        },
      })!;
      let reg = modelDefinitionInteractionRegistry();
      reg = registerInteractionSpec(reg, compileInteraction(grandchild));
      reg = registerInteractionSpec(reg, compileInteraction(child));
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const rt = createInteractionRuntime(compileInteraction(host), {
        kernel,
        document: { model: new Model(), nodes: [] },
        interactions: reg,
      });
      await rt.send({ kind: "go" });
      let snap = rt.getSnapshot();
      expect(snap.interactionId).toBe("test.pick.child");
      expect(snap.nested?.hostInteractionId).toBe("test.pick.host");
      await rt.send({ kind: "confirm" });
      snap = rt.getSnapshot();
      expect(snap.interactionId).toBe("test.pick.grandchild");
      expect(snap.nested?.hostInteractionId).toBe("test.pick.child");
      await rt.send({ kind: "confirm" });
      snap = rt.getSnapshot();
      expect(snap.interactionId).toBe("test.pick.host");
      expect(snap.state).toBe("done");
      expect(snap.context.token).toBe("ok");
    });
    it("ActionRegistry.withModelDefinitionActions registers declarative model-definition actions only", () => {
      const r = modelDefinitionActionRegistry();
      const ids = new Set(listActionDefs(r).map((d) => d.id));
      expect(ids.has("primitive.createBoxFromCorners")).toBe(true);
      expect(ids.has("box.aabbFromDiagonalCorners")).toBe(true);
      expect(ids.has("command.finish")).toBe(true);
      expect(ids.has("feature.offsetFaces")).toBe(true);
      expect(ids.has("selection.apply")).toBe(true);
      expect(ids.has("selection.selectAll")).toBe(true);
      expect(ids.has("selection.selectVertices")).toBe(true);
      expect(listActionDefs(r).every((def) => def.spec !== undefined && def.run === undefined)).toBe(true);
    });
    it("register replaces a model-definition action id", () => {
      let r = modelDefinitionActionRegistry();
      const before = r.get("measure.faceArea")?.label;
      r = registerActionDef(r, {
        id: "measure.faceArea",
        label: "override",
        run: () => ({ data: 99 }),
      });
      expect(r.get("measure.faceArea")?.label).toBe("override");
      expect(before).not.toBe("override");
    });
    it("InteractionRegistry.withModelDefinitionInteractions get matches buildBoxInteractionSpec", () => {
      const reg = modelDefinitionInteractionRegistry();
      expect(reg.get("primitive.box")).toEqual(buildBoxInteractionSpec());
    });
    it("loadSpatialInteraction resolves callable surface.construct hub from shipped assets", () => {
      const spec = loadSpatialInteraction("surface.construct");
      expect(spec?.id).toBe("surface.construct");
      expect(spec?.invocation).toBe("callable");
      expect(isCallableOnlyInteraction(spec!)).toBe(true);
      expect(loadSpatialInteraction("curve.construct")?.invocation).toBe("callable");
    });
    it("loadSpatialInteraction returns the same compiled instance per interaction id", () => {
      expect(loadSpatialInteraction("primitive.box")).toBe(loadSpatialInteraction("primitive.box"));
    });
    it("createBoxFrom3Points forwards triplet footprint to createBoxFromCorners", async () => {
      class StubKernel extends BrepjsKernel {
        lastInput: { cornerA: Vec3; cornerB: Vec3; height: number } | null = null;
        async createBoxFromCornersDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }) {
          this.lastInput = input;
          return { diff: EMPTY_MODEL_DIFF, solid: solidRef("c") };
        }
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
      }
      const k = new StubKernel();
      const model = new Model();
      const p0: Vec3 = [0, 0, 0];
      const p1: Vec3 = [2, 3, 0];
      const p2: Vec3 = [1, 1, 0];
      await runRegisteredAction(modelDefinitionActionRegistry(),"primitive.createBoxFrom3Points", { p0, p1, p2, __context: {}, __event: { kind: "x" } }, { kernel: k as unknown as SpatialKernel, preview: M, model });
      expect(k.lastInput).toEqual({ cornerA: [0, 0, 0], cornerB: [2, 3, 0], height: 3 });
    });
    it("command.addSelection applies selection modifiers", async () => {
      const actions = modelDefinitionActionRegistry();
      const base = [{ kind: "wire", id: "w0", editable: true }] as const;
      const next = [{ kind: "wire", id: "w1", editable: true }] as const;
      const additive = await runRegisteredAction(actions,
        "command.addSelection",
        { targets: next, __context: { targets: base }, __event: { kind: "selection.changed", modifiers: { shift: true } } },
        { kernel: M as unknown as SpatialKernel, preview: M, model: new Model() },
      );
      expect((additive.patch?.set as { targets?: readonly SelectionTarget[] }).targets).toEqual([...base, ...next]);
      const subtractive = await runRegisteredAction(actions,
        "command.addSelection",
        {
          targets: next,
          __context: { targets: [...base, ...next] },
          __event: { kind: "selection.changed", modifiers: { ctrl: true } },
        },
        { kernel: M as unknown as SpatialKernel, preview: M, model: new Model() },
      );
      expect((subtractive.patch?.set as { targets?: readonly SelectionTarget[] }).targets).toEqual(base);
      const invertive = await runRegisteredAction(actions,
        "command.addSelection",
        {
          targets: [
            { kind: "wire", id: "w0", editable: true },
            { kind: "wire", id: "w2", editable: true },
          ],
          __context: { targets: [...base, ...next] },
          __event: { kind: "selection.changed", modifiers: { shift: true, ctrl: true } },
        },
        { kernel: M as unknown as SpatialKernel, preview: M, model: new Model() },
      );
      expect((invertive.patch?.set as { targets?: readonly SelectionTarget[] }).targets).toEqual([
        { kind: "wire", id: "w1", editable: true },
        { kind: "wire", id: "w2", editable: true },
      ]);
    });
    it("selection.apply runs selectAll, deselectAll, invert, and selectKinds", async () => {
      const actions = modelDefinitionActionRegistry();
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const seed = [{ kind: "vertex", id: Object.keys(model.vertices)[0]!, editable: true }] as const;
      const all = await runRegisteredAction(actions,"selection.apply", { operation: "selectAll", seedTargets: [], __context: {} }, { kernel: M as unknown as SpatialKernel, preview: M, model });
      const allTargets = selectionTargetsFromActionResult(all);
      expect(allTargets.length).toBeGreaterThan(8);
      const cleared = await runRegisteredAction(actions,"selection.apply", { operation: "deselectAll", seedTargets: allTargets, __context: {} }, { kernel: M as unknown as SpatialKernel, preview: M, model });
      expect(selectionTargetsFromActionResult(cleared)).toEqual([]);
      const verts = await runRegisteredAction(actions,"selection.apply", { operation: "selectKinds", kinds: ["vertex"], seedTargets: [], __context: {} }, { kernel: M as unknown as SpatialKernel, preview: M, model });
      const vertTargets = selectionTargetsFromActionResult(verts);
      expect(vertTargets.length).toBe(8);
      expect(vertTargets.every((t) => t.kind === "vertex")).toBe(true);
      const inverted = await runRegisteredAction(actions,"selection.apply", { operation: "invert", seedTargets: vertTargets.slice(0, 1), __context: {} }, { kernel: M as unknown as SpatialKernel, preview: M, model });
      const invertedTargets = selectionTargetsFromActionResult(inverted);
      expect(invertedTargets.some((t) => t.kind === "vertex")).toBe(true);
      expect(invertedTargets.some((t) => t.kind === "face")).toBe(true);
      expect(invertedTargets.find((t) => t.id === vertTargets[0]!.id)).toBeUndefined();
    });
    it("selection.selectAll returns targets without model diff", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const result = await runRegisteredAction(modelDefinitionActionRegistry(),"selection.selectAll", { seedTargets: [], __context: {}, __event: { kind: "commit" } }, { kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M, model });
      const targets = selectionTargetsFromActionResult(result);
      expect(targets.length).toBeGreaterThan(8);
      expect(targets.some((t) => t.kind === "solid")).toBe(true);
      expect(isEmptyModelDiff(result.diff ?? EMPTY_MODEL_DIFF)).toBe(true);
    });
    it("interactionRecordsDocumentHistory skips selection commands", () => {
      expect(interactionRecordsDocumentHistory("selection.selectAll")).toBe(false);
      expect(interactionRecordsDocumentHistory("measure.distance")).toBe(false);
      expect(interactionRecordsDocumentHistory("primitive.box")).toBe(true);
    });
    it("selection.selectAll headless does not push document history entries", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const hist = new DocumentHistory();
      await runRegisteredAction(modelDefinitionActionRegistry(),"selection.selectAll", { seedTargets: [], __context: {}, __event: { kind: "commit" } }, { kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M, model });
      expect(hist.entries()).toEqual([]);
    });
    it.each(listSelectionOperationsForModelDefinition(defaultModelDefinitionId()))("registers selection command action $id", (defn) => {
      expect(modelDefinitionActionRegistry().get(defn.id)?.spec?.schema).toBe("spatial.action");
    });
    it("selection.invert honors seed targets", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("e2e-box")));
      const seed = [{ kind: "solid", id: "e2e-box", editable: true }] as const;
      const result = await runRegisteredAction(modelDefinitionActionRegistry(),"selection.invert", { seedTargets: seed, __context: {}, __event: { kind: "commit" } }, { kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M, model });
      const targets = selectionTargetsFromActionResult(result);
      expect(targets.some((t) => t.kind === "solid" && t.id === "e2e-box")).toBe(false);
      expect(targets.length).toBeGreaterThan(0);
    });
    it("ActionRegistry.run executes selection.apply headless", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const actions = modelDefinitionActionRegistry();
      const result = await runRegisteredAction(actions,
        "selection.apply",
        {
          operation: "selectKinds",
          kinds: ["face"],
          seedTargets: [],
          __context: {},
          __event: { kind: "commit" },
        },
        { kernel: M as unknown as SpatialKernel, preview: M, model },
      );
      const targets = selectionTargetsFromActionResult(result);
      expect(targets.length).toBe(6);
      expect(targets.every((t) => t.kind === "face")).toBe(true);
    });
    it("runSelectionApply matches executeSelectionApply", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("e2e-box")));
      const ctx = { kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M, model };
      const params = { operation: "selectAll" as const, seedTargets: [] };
      const direct = executeSelectionApply(params, { model });
      const headless = await runSelectionApply(params, ctx);
      expect(headless).toEqual(direct);
    });
    it.each(listSelectionOperationsForModelDefinition(defaultModelDefinitionId()))("runSelectionApply matches runSelectionOperationInteraction for $id", async (defn) => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("e2e-box")));
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const seed = selectionSeedTargetsForOperation(defn.operation);
      const params = selectionApplyParamsForInteraction(defn, seed);
      const headless = await runSelectionApply(params, { kernel, preview: M, model });
      const interactive = await runSelectionOperationInteraction(defn.id, {
        kernel,
        document: { model, nodes: [] },
        seedTargets: seed,
      });
      expect(interactive.targets).toEqual(headless);
    });
    it("selection commands chain selectAll → deselectAll → selectVertices → invert", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("e2e-box")));
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const actions = modelDefinitionActionRegistry();
      const run = async (id: string, targets: readonly SelectionTarget[]) => {
        const result = await runRegisteredAction(actions,id, { seedTargets: targets, __context: {}, __event: { kind: "commit" } }, { kernel, preview: M, model });
        return selectionTargetsFromActionResult(result);
      };
      const all = await run("selection.selectAll", []);
      expect(all.length).toBeGreaterThan(8);
      const cleared = await run("selection.deselectAll", all);
      expect(cleared).toEqual([]);
      const verts = await run("selection.selectVertices", cleared);
      expect(verts.length).toBe(8);
      expect(verts.every((t) => t.kind === "vertex")).toBe(true);
      const inverted = await run("selection.invert", verts.slice(0, 1));
      expect(inverted.some((t) => t.kind === "face")).toBe(true);
      expect(inverted.find((t) => t.id === verts[0]!.id)).toBeUndefined();
    });
  });
  describe("@semio-tech/cad-js/core interaction box", () => {
    it("tracks first-corner cursor on the grid after start", async () => {
      class StubKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("stub");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
      }
      const spec = buildBoxInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      let snap = rt.getSnapshot();
      expect(snap.state).toBe("first_corner");
      expect(snap.context.cursor).toEqual([0, 0, 0]);
      expect(snap.display.items.find((i) => i.id === "first-cursor")?.params?.position).toEqual([0, 0, 0]);
      await rt.send({ kind: "pointer.move", point: [-1, 2.5, 0] as Vec3, modifiers: {} });
      snap = rt.getSnapshot();
      expect(snap.context.cursor).toEqual([-1, 2.5, 0]);
      expect(snap.display.items.find((i) => i.id === "first-cursor")?.params?.position).toEqual([-1, 2.5, 0]);
      await rt.send({ kind: "pointer.down", point: [3, 1, 0] as Vec3, modifiers: {} });
      snap = rt.getSnapshot();
      expect(snap.state).toBe("first_corner_other_or_length");
      expect(snap.context.cursor).toBeUndefined();
    });

    it("pushes interaction-local undo snapshot on each non-transient transition", async () => {
      class StubKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("stub");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
      }
      const spec = buildBoxInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      expect(rt.getSnapshot().capabilities.canUndo).toBe(false);
      const initial = rt.getSnapshot().state;
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      expect(rt.getSnapshot().capabilities.canUndo).toBe(true);
      await rt.undo();
      expect(rt.getSnapshot().state).toBe(initial);
    });

    it("runs box workflow with a recording kernel stub (no solid modeling in core)", async () => {
      const stubMesh: MeshTransfer = {
        position: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
        normal: new Float32Array([0, 0, 1, 0, 0, 1, 0, 0, 1]),
        index: new Uint32Array([0, 1, 2]),
        edges: new Float32Array(0),
        faceGroups: [],
        edgeGroups: [],
        faceInfos: [],
        edgeInfos: [],
      };
      class RecordingStubKernel {
        readonly id = "recording-stub";
        readonly operations = ["solid.createBox", "entity.tessellate"] as const;
        lastBox: { cornerA: Vec3; cornerB: Vec3; height: number } | null = null;
        constructor() {
          Object.assign(this, M);
        }
        async createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<SolidRef> {
          this.lastBox = input;
          return solidRef("stub-solid");
        }
        async createBoxFromCornersDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }> {
          const solid = await this.createBoxFromCorners(input);
          return { diff: M.boxModelDiff(input, solid), solid };
        }
        async volume(): Promise<number> {
          return 0;
        }
        async tessellate(): Promise<MeshTransfer> {
          return stubMesh;
        }
      }
      const spec = buildBoxInteractionSpec();
      const model = new Model();
      const kernel = new RecordingStubKernel();
      const rt = createInteractionRuntime(spec, { kernel: kernel as unknown as SpatialKernel, document: { model: model, nodes: [] } });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [2, 3, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "set.height", value: 4, modifiers: {} });
      expect(rt.getSnapshot().state).toBe("first_corner_height");
      expect(rt.getSnapshot().context.height).toBe(4);
      await rt.send({ kind: "confirm", modifiers: {} });
      const snap = rt.getSnapshot();
      const res = snap.lastResponse!;
      expect(snap.state).toBe("committed");
      expect(res.ok).toBe(true);
      expect(res.data).toEqual({ solid: "stub-solid" });
      expect(res.archiveContext).not.toBeNull();
      expect(res.archiveContext!.origin).toEqual([0, 0, 0]);
      expect(res.archiveContext!.corner).toEqual([2, 3, 0]);
      expect(res.archiveContext!.height).toBe(4);
      expect(Object.keys(model.vertices).length).toBe(8);
      expect(Object.keys(model.edges).length).toBe(12);
      expect(Object.keys(model.wires).length).toBe(6);
      expect(Object.keys(model.faces).length).toBe(6);
      expect(Object.keys(model.shells).length).toBe(1);
      expect(Object.keys(model.solids)).toEqual(["stub-solid"]);
      expect(kernel.lastBox).toEqual({
        cornerA: [0, 0, 0],
        cornerB: [2, 3, 0],
        height: 4,
      });
    });

    it("set.length clamps diagonal from diagA without prior pointer move", async () => {
      class StubKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("stub");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
      }
      const spec = buildBoxInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      await rt.send({ kind: "start" });
      await rt.send({ kind: "mode.diagonal" });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      expect(rt.getSnapshot().state).toBe("diagonal_rubber");
      await rt.send({ kind: "set.length", value: 5, modifiers: {} });
      const corner = rt.getSnapshot().context.corner as Vec3;
      expect(corner[0]).toBeCloseTo(5, 5);
      expect(corner[1]).toBeCloseTo(0, 5);
    });

    it("scalar axis pointer.move sets height from projection along +Z", async () => {
      class StubKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("stub");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
      }
      const spec = buildBoxInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      await rt.send({ kind: "start" });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [2, 3, 0] as Vec3, modifiers: {} });
      expect(rt.getSnapshot().state).toBe("first_corner_height");
      await rt.send({ kind: "pointer.move", point: [9, 9, 4.2] as Vec3, modifiers: {} });
      expect(rt.getSnapshot().context.height).toBeCloseTo(4.2, 5);
      expect(rt.getSnapshot().context.__scalarAxisT).toBeCloseTo(4.2, 5);
    });

    it("resolveDisplay injects height line, cursor, and guide for scalar entry", () => {
      const spec = buildBoxInteractionSpec();
      const ctx: Record<string, unknown> = {
        origin: [0, 0, 0] as Vec3,
        corner: [2, 3, 0] as Vec3,
        height: 3,
        __scalarAxisT: 3,
        __cursorRaw: [5, 6, 4] as Vec3,
      };
      const d = resolveDisplay(spec, "first_corner_height", ctx, M);
      expect(d.items.some((i) => i.id === "first_corner_height-scalar-height")).toBe(true);
      expect(d.items.some((i) => i.id === "first_corner_height-scalar-cursor" && i.role === "cursor")).toBe(true);
      expect(d.items.some((i) => i.id === "first_corner_height-scalar-guide" && i.role === "guide")).toBe(true);
      const heightSeg = d.items.find((i) => i.id === "first_corner_height-scalar-height");
      expect(heightSeg?.kind).toBe("segment");
      if (heightSeg?.kind === "segment") {
        const to = heightSeg.params?.to;
        expect(Array.isArray(to) && typeof to[2] === "number" ? to[2] : NaN).toBeCloseTo(3, 5);
      }
    });

    it("set.height live entry keeps first_corner_height until confirm", async () => {
      class StubKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("stub");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
      }
      const spec = buildBoxInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [2, 3, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "set.height", value: 3.5, modifiers: {} });
      expect(rt.getSnapshot().state).toBe("first_corner_height");
      expect(rt.getSnapshot().context.height).toBe(3.5);
    });

    it("set.length clamps rubber-band corner from origin on first footprint edge", async () => {
      class StubKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("stub");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
      }
      const spec = buildBoxInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.move", point: [3, 4, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "set.length", value: 2.5, modifiers: {} });
      const snap = rt.getSnapshot();
      expect(snap.state).toBe("first_corner_other_or_length");
      const corner = snap.context.corner as Vec3;
      expect(corner[0]).toBeCloseTo(1.5, 5);
      expect(corner[1]).toBeCloseTo(2, 5);
      expect(snap.display.items.some((i) => i.id === "first_corner_other_or_length-length-guide")).toBe(true);
    });
  });
  describe("@semio-tech/cad-js/core interaction length entry", () => {
    it("interactionLengthEntryForState resolves shipped line rubber-band", () => {
      const spec = requireSpatialInteraction("curve.line");
      expect(interactionLengthEntryForState(spec, "end_of_line")).toEqual({
        state: "end_of_line",
        anchor: "points.start",
        field: "cursor",
        control: "stepper",
        min: 0,
        step: 0.1,
        unit: "m",
      });
    });

    it("readInteractionContextVec3 supports points.@last on arrays", () => {
      const ctx = {
        points: [
          [0, 0, 0],
          [1, 2, 3],
        ] as Vec3[],
      };
      expect(readInteractionContextVec3(ctx, "points.@last")).toEqual([1, 2, 3]);
    });

    it("clampPointAlongDirection preserves direction and length", () => {
      expect(clampPointAlongDirection([0, 0, 0], [3, 4, 0], 2.5, M)).toEqual([1.5, 2, 0]);
    });

    it("set.length clamps cursor along anchor direction", async () => {
      class StubKernel extends BrepjsKernel {
        async curveLine() {
          return { diff: EMPTY_MODEL_DIFF };
        }
      }
      const spec = requireSpatialInteraction("curve.line");
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      await rt.send({ kind: "start" });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.move", point: [3, 4, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "set.length", value: 2.5, modifiers: {} });
      const snap = rt.getSnapshot();
      const cursor = snap.context.cursor as Vec3;
      expect(cursor[0]).toBeCloseTo(1.5, 5);
      expect(cursor[1]).toBeCloseTo(2, 5);
      expect(snap.context.__lengthLock).toBe(2.5);
      expect(snap.display.items.some((i) => i.id === "end_of_line-length-guide")).toBe(true);
    });

    it("set.length null unlocks and pointer.move follows cursor again", async () => {
      class StubKernel extends BrepjsKernel {
        async curveLine() {
          return { diff: EMPTY_MODEL_DIFF };
        }
      }
      const spec = requireSpatialInteraction("curve.line");
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      await rt.send({ kind: "start" });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.move", point: [3, 4, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "set.length", value: 2.5, modifiers: {} });
      await rt.send({ kind: "set.length", value: null, modifiers: {} });
      await rt.send({ kind: "pointer.move", point: [10, 0, 0] as Vec3, modifiers: {} });
      const cursor = rt.getSnapshot().context.cursor as Vec3;
      expect(cursor).toEqual([10, 0, 0]);
      expect(rt.getSnapshot().context.__lengthLock).toBeNull();
    });

    it("interactionLengthEntryLiveDistance reads Z rod cursor offset for extrusion_distance", () => {
      const spec = loadSpatialInteraction("surface.extrudeCrv")!;
      const entry = interactionLengthEntryForState(spec, "extrusion_distance")!;
      const distance = interactionLengthEntryLiveDistance({ origin: [0, 0, 0] as Vec3, cursor: [0, 0, 1.25] as Vec3, direction: [0, 0, 1] as Vec3 }, entry);
      expect(distance).toBeCloseTo(1.25, 5);
      expect(interactionNumericEntryLockedValue(spec, "extrusion_distance", { origin: [0, 0, 0], cursor: [0, 0, 2], direction: [0, 0, 1] })).toBe(2);
    });

    it("surface.extrudeCrv confirm in extrusion_distance commits solid", async () => {
      const spec = loadSpatialInteraction("surface.extrudeCrv")!;
      const space = ModelSpace.fromJSON(geometryRoutesFixtureJson as ModelSpaceJson);
      const model = space.models["spatial.shape"]!;
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const rt = createInteractionRuntime(spec, {
        kernel,
        document: { model, nodes: [] },
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      await rt.send({ kind: "selection.changed", targets: [{ kind: "wire", id: "stub-wire" }], modifiers: {} });
      await rt.send({ kind: "confirm", modifiers: {} });
      await rt.send({ kind: "pointer.move", point: [0, 0, 0.8], modifiers: {} });
      await rt.send({ kind: "confirm", modifiers: {} });
      expect(rt.getSnapshot().state).toBe("committed");
      expect(rt.getSnapshot().lastResponse?.ok).toBe(true);
      expect(Object.keys(model.solids).length).toBeGreaterThan(0);
    });

    it("surface.extrudeCrv start seeds curves from selected interpolate object", async () => {
      const spec = loadSpatialInteraction("surface.extrudeCrv")!;
      const model = new Model();
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const created = await kernel.executeCommandDiff("curve.interpolateCurve", {
        model,
        points: [
          [0, 0, 0],
          [2, 1, 0],
          [4, 0, 0],
        ],
      });
      applyModelDiff(model, created.diff);
      const typology = "spatial.shape.curve.interpolate-curve";
      const wireId = created.diff.wires?.added?.[0]?.id ?? Object.values(model.wires)[0]?.id;
      expect(wireId).toBeTruthy();
      const objectId = String(ensureTypologyObjectFromCreateDiff(model, typology, created.diff)!);
      const rt = createInteractionRuntime(spec, {
        kernel,
        document: { model, nodes: [] },
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      await rt.send({
        kind: "start",
        targets: [{ kind: "object", id: objectId, editable: true }],
        modifiers: {},
      });
      expect(rt.getSnapshot().state).toBe("extrusion_distance");
      expect((rt.getSnapshot().context.curves as { id: string }[])[0]?.id).toBe(wireId);
    });

    it("interactionControlForState resolves stepper for box height and ring for rotate angle", () => {
      const box = loadSpatialInteraction("primitive.box")!;
      const height = interactionControlForState(box, "first_corner_height", { height: 3 });
      expect(height?.kind).toBe("stepper");
      if (height && height.kind !== "ring") {
        expect(height.value).toBe(3);
        expect(height.unit).toBe("m");
      }
      const rotate = loadSpatialInteraction("transform.rotate")!;
      const angle = interactionControlForState(rotate, "angle_or_first_reference_point", { angle: 45 });
      expect(angle?.kind).toBe("ring");
      if (angle?.kind === "ring") {
        expect(angle.options.length).toBeGreaterThan(0);
      }
    });

    it("interactionNumericEntryCommitEvent uses pointer.down for length and confirm for scalar", () => {
      const line = requireSpatialInteraction("curve.line");
      const box = buildBoxInteractionSpec();
      const ctx = { points: { start: [0, 0, 0] as Vec3 }, cursor: [3, 0, 0] as Vec3 };
      expect(interactionNumericEntryCommitEvent(line, "end_of_line", ctx, M)?.kind).toBe("pointer.down");
      expect(interactionNumericEntryCommitEvent(box, "first_corner_height", { height: 2 }, M)?.kind).toBe("confirm");
    });

    it("interactionNumericEntryCommitEvent pointer.down from length lock without field", () => {
      const box = buildBoxInteractionSpec();
      const ctx = { origin: [0, 0, 0] as Vec3, diagA: [0, 0, 0] as Vec3, __lengthLock: 4 };
      const ev = interactionNumericEntryCommitEvent(box, "diagonal_rubber", ctx, M);
      expect(ev?.kind).toBe("pointer.down");
      if (ev?.kind === "pointer.down") {
        const point = ev.point;
        expect(Array.isArray(point) && typeof point[0] === "number" ? point[0] : NaN).toBeCloseTo(4, 5);
        expect(Array.isArray(point) && typeof point[1] === "number" ? point[1] : NaN).toBeCloseTo(0, 5);
      }
    });

    it("Enter commit applies length then pointer.down on line rubber-band", async () => {
      class StubKernel extends BrepjsKernel {
        async curveLine() {
          return { diff: EMPTY_MODEL_DIFF };
        }
      }
      const spec = requireSpatialInteraction("curve.line");
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      await rt.send({ kind: "start" });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.move", point: [3, 4, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "set.length", value: 2.5, modifiers: {} });
      const commitEv = interactionNumericEntryCommitEvent(spec, rt.getSnapshot().state, rt.getSnapshot().context, M);
      expect(commitEv?.kind).toBe("pointer.down");
      await rt.send(commitEv!);
      expect(rt.getSnapshot().state).toBe("committed");
    });

    it("pointer.down while locked commits clamped point", async () => {
      class StubKernel extends BrepjsKernel {
        async curveLine() {
          return { diff: EMPTY_MODEL_DIFF };
        }
      }
      const spec = requireSpatialInteraction("curve.line");
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      await rt.send({ kind: "start" });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.move", point: [3, 4, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "set.length", value: 2.5, modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [3, 4, 0] as Vec3, modifiers: {} });
      const snap = rt.getSnapshot();
      const end = (snap.context.points as Record<string, Vec3>).end;
      expect(end[0]).toBeCloseTo(1.5, 5);
      expect(end[1]).toBeCloseTo(2, 5);
      expect(snap.context.__lengthLock).toBeUndefined();
    });
  });
  describe("@semio-tech/cad-js/core stateEngine option", () => {
    it("explicit pure-ts provider matches default interaction snapshots", async () => {
      class StubKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
      }
      const spec = buildBoxInteractionSpec();
      const rt0 = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      const rt1 = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
        stateEngine: pureTsStateEngineProvider,
      });
      expect(rt1.getSnapshot().state).toBe(rt0.getSnapshot().state);
      expect(rt1.getSnapshot().context).toEqual(rt0.getSnapshot().context);
      expect(rt1.getSnapshot().capabilities).toEqual(rt0.getSnapshot().capabilities);
    });
  });
  describe("@semio-tech/cad-js/core measure distance", () => {
    it("measure.faceArea action adds face anchor geometry", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("m-area")));
      const fid = Object.keys(model.faces)[0]! as FaceRef;
      const r = await runRegisteredAction(modelDefinitionActionRegistry(),"measure.faceArea", { faceId: fid }, { model: model, kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M });
      expect(r.data).toBeGreaterThan(0);
      expect(r.diff?.anchors?.added?.length).toBe(1);
      expect(r.diff!.anchors!.added![0]!.attachment.kind).toBe("face");
    });

    it("commit returns vertex distance in data", async () => {
      class MeasKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
        async query(name: string, params: Record<string, unknown>) {
          if (name === "surface.resolveFaces") return [String(params.surfaceId ?? "")];
          return undefined;
        }
        async vertexDistance(a: VertexRef, b: VertexRef, t: Model) {
          const pa = t.vertices[String(a)]?.position;
          const pb = t.vertices[String(b)]?.position;
          if (!pa || !pb) return 0;
          return M.vec3Distance(pa, pb);
        }
      }
      const model = new Model();
      const va = "v0" as VertexRef;
      const vb = "v1" as VertexRef;
      model.vertices[va] = { id: va, position: [0, 0, 0] };
      model.vertices[vb] = { id: vb, position: [3, 4, 0] };
      const spec = buildDistanceInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new MeasKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
      });
      await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: va, editable: true }] });
      await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: vb, editable: true }] });
      const res = rt.getSnapshot().lastResponse!;
      expect(res.ok).toBe(true);
      expect(res.data).toBe(5);
      expect(isEmptyModelDiff(res.diff)).toBe(false);
      expect(res.diff.edges?.added?.length).toBe(1);
      expect(res.diff.wires?.added?.length).toBe(1);
      const edge = res.diff.edges!.added![0]!;
      expect(edge.vertexIds).toEqual([va, vb]);
    });

    it("auto-commits when confirm reaches the final state", async () => {
      class MeasKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
        async vertexDistance(a: VertexRef, b: VertexRef, t: Model) {
          const pa = t.vertices[String(a)]?.position;
          const pb = t.vertices[String(b)]?.position;
          if (!pa || !pb) return 0;
          return M.vec3Distance(pa, pb);
        }
      }
      const model = new Model();
      const va = "v0" as VertexRef;
      const vb = "v1" as VertexRef;
      model.vertices[va] = { id: va, position: [0, 0, 0] };
      model.vertices[vb] = { id: vb, position: [3, 4, 0] };
      const rt = createInteractionRuntime(buildDistanceInteractionSpec(), {
        kernel: new MeasKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
      });
      await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: va, editable: true }] });
      await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: vb, editable: true }] });
      const snap = rt.getSnapshot();
      expect(snap.state).toBe("committed");
      expect(snap.capabilities.canCommit).toBe(false);
      expect(snap.capabilities.canCancel).toBe(false);
      expect(snap.lastResponse?.ok).toBe(true);
      expect(snap.lastResponse?.data).toBe(5);
    });
  });
  describe("@semio-tech/cad-js/core measure area", () => {
    it("resolves face picks through surface.resolveFaces before commit", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("area-box")));
      const fid = Object.keys(model.faces)[0]! as FaceRef;
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const rt = createInteractionRuntime(buildAreaInteractionSpec(), { kernel, document: { model: model, nodes: [] } });
      await rt.send({ kind: "selection.changed", targets: [{ kind: "face", id: fid, editable: true }] });
      expect(rt.getSnapshot().context.resolvedFaceIds).toEqual([fid]);
      await rt.send({ kind: "confirm", modifiers: {} });
      const snap = rt.getSnapshot();
      expect(snap.state).toBe("committed");
      expect(snap.lastResponse?.ok).toBe(true);
      expect(typeof snap.lastResponse?.data).toBe("number");
      expect(isEmptyModelDiff(snap.lastResponse!.diff)).toBe(false);
      expect(snap.lastResponse!.diff.anchors?.added?.length).toBe(1);
    });

    it("commit returns face area in data", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("area-box")));
      const fid = Object.keys(model.faces)[0]! as FaceRef;
      class AreaKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
        async query(name: string, params: Record<string, unknown>) {
          if (name === "surface.resolveFaces") {
            const sid = String(params.surfaceId ?? "");
            return model.faces[sid as FaceRef] ? [sid] : [];
          }
          return undefined;
        }
        async faceArea(_f: FaceRef, _t: Model) {
          return 2.5;
        }
      }
      const spec = buildAreaInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new AreaKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
      });
      await rt.send({ kind: "selection.changed", targets: [{ kind: "face", id: fid, editable: true }] });
      const res = rt.getSnapshot().lastResponse!;
      expect(res.ok).toBe(true);
      expect(res.data).toBe(2.5);
      expect(isEmptyModelDiff(res.diff)).toBe(false);
      expect(res.diff.anchors?.added?.length).toBe(1);
    });
  });
  describe("@semio-tech/cad-js/core document history", () => {
    it("records modifications and undo/redo applies forward and backwards diffs", () => {
      const g = new Model();
      const h = new DocumentHistory();
      const mesh: MeshTransfer = {
        position: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
        normal: new Float32Array([0, 0, 1, 0, 0, 1, 0, 0, 1]),
        index: new Uint32Array([0, 1, 2]),
        edges: new Float32Array(0),
        faceGroups: [],
        edgeGroups: [],
        faceInfos: [],
        edgeInfos: [],
      };
      const d1 = M.meshFaceModelDiff(mesh, "a");
      const inv1 = applyModelDiff(g, d1);
      const res1: InteractionResponse = { ok: true, errors: [], warnings: [], infos: [], diff: d1, data: null, archiveContext: null };
      h.record({ id: "m1", interactionId: "c", label: "A", result: res1, backwardsDiff: inv1 });
      const d2 = M.meshFaceModelDiff(mesh, "b");
      const inv2 = applyModelDiff(g, d2);
      const res2: InteractionResponse = { ok: true, errors: [], warnings: [], infos: [], diff: d2, data: null, archiveContext: null };
      h.record({ id: "m2", interactionId: "c", label: "B", result: res2, backwardsDiff: inv2 });
      expect(Object.keys(g.faces).length).toBe(2);
      expect(h.entries().map((m) => m.id)).toEqual(["m1", "m2"]);
      const doc = { model: g, nodes: [] as ShapeNode[] };
      h.undo(doc);
      expect(Object.keys(g.faces).length).toBe(1);
      expect(h.entries().map((m) => m.id)).toEqual(["m1"]);
      h.undo(doc);
      expect(Object.keys(g.faces).length).toBe(0);
      h.redo(doc);
      expect(Object.keys(g.faces).length).toBe(1);
      h.redo(doc);
      expect(Object.keys(g.faces).length).toBe(2);
      h.clear();
      expect(h.entries()).toEqual([]);
      expect(h.peekUndo()).toBe(null);
    });
  });
  describe("@semio-tech/cad-js/core measure distance history", () => {
    it("interactionRecordsDocumentHistory skips measure interactions", () => {
      expect(interactionRecordsDocumentHistory("measure.distance")).toBe(false);
      expect(interactionRecordsDocumentHistory("primitive.box")).toBe(true);
    });

    it("does not push readonly measure commits onto document history", async () => {
      class MeasKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
        async query(name: string, params: Record<string, unknown>) {
          if (name === "surface.resolveFaces") return [String(params.surfaceId ?? "")];
          return undefined;
        }
        async vertexDistance(a: VertexRef, b: VertexRef, t: Model) {
          const pa = t.vertices[String(a)]?.position;
          const pb = t.vertices[String(b)]?.position;
          if (!pa || !pb) return 0;
          return M.vec3Distance(pa, pb);
        }
      }
      const hist = new DocumentHistory();
      const model = new Model();
      const va = "v0" as VertexRef;
      const vb = "v1" as VertexRef;
      model.vertices[va] = { id: va, position: [0, 0, 0] };
      model.vertices[vb] = { id: vb, position: [3, 4, 0] };
      const spec = buildDistanceInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new MeasKernel() as unknown as SpatialKernel,
        document: { model: model, nodes: [] },
        history: hist,
      });
      await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: va, editable: true }] });
      await rt.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: vb, editable: true }] });
      expect(hist.peekUndo()).toBe(null);
    });
  });
  describe("@semio-tech/cad-js/core interaction session undo redo", () => {
    it("supports redo after undo during an active interaction and clears redo on new branch", async () => {
      class StubKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
      }
      const spec = buildBoxInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: new Model(), nodes: [] },
      });
      expect(rt.getSnapshot().state).toBe("first_corner");
      expect(rt.getSnapshot().capabilities.canRedo).toBe(false);
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      rt.undo();
      expect(rt.getSnapshot().state).toBe("first_corner");
      expect(rt.getSnapshot().capabilities.canRedo).toBe(true);
      await rt.send({ kind: "pointer.down", point: [1, 0, 0] as Vec3, modifiers: {} });
      expect(rt.getSnapshot().capabilities.canRedo).toBe(false);
    });
  });
  describe("@semio-tech/cad-js/core undo routing", () => {
    it("uses snapshot undo while active and document history when idle", async () => {
      class StubKernel extends BrepjsKernel {
        async createBoxFromCorners() {
          return solidRef("c");
        }
        async volume() {
          return 0;
        }
        async tessellate() {
          return emptyMeshTransfer();
        }
      }
      const g = new Model();
      const mesh: MeshTransfer = {
        position: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
        normal: new Float32Array([0, 0, 1, 0, 0, 1, 0, 0, 1]),
        index: new Uint32Array([0, 1, 2]),
        edges: new Float32Array(0),
        faceGroups: [],
        edgeGroups: [],
        faceInfos: [],
        edgeInfos: [],
      };
      const d0 = M.meshFaceModelDiff(mesh, "seed");
      const inv0 = applyModelDiff(g, d0);
      const hist = new DocumentHistory();
      hist.record({
        id: "seed",
        interactionId: "x",
        label: "seed",
        result: { ok: true, errors: [], warnings: [], infos: [], diff: d0, data: null, archiveContext: null },
        backwardsDiff: inv0,
      });
      expect(Object.keys(g.faces).length).toBe(1);
      const spec = buildBoxInteractionSpec();
      const rt = createInteractionRuntime(spec, {
        kernel: new StubKernel() as unknown as SpatialKernel,
        document: { model: g, nodes: [] },
        history: hist,
      });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      rt.undo();
      expect(rt.getSnapshot().state).toBe("first_corner");
      expect(Object.keys(g.faces).length).toBe(1);
      rt.undo();
      expect(Object.keys(g.faces).length).toBe(0);
    });
  });
  describe("@semio-tech/cad-js/core interaction e2e fixtures", () => {
    type InteractionE2EFixtureKind = "loom" | "routes" | "building" | "empty";

    const MOD: InteractionEvent["modifiers"] = {};

    const p = (x: number, y: number, z = 0): Vec3 => [x, y, z];

    const sel = (kind: ModelEntityKind, id: string, editable = true): SelectionTarget => ({
      kind,
      id,
      editable,
    });

    const modelFromFixture = (kind: InteractionE2EFixtureKind): Model => {
      if (kind === "empty") return new Model();
      const raw = kind === "loom" ? geometryLoomFixtureJson : kind === "routes" ? geometryRoutesFixtureJson : buildingBooleanFixtureJson;
      if (raw && typeof raw === "object" && (raw as ModelSpaceJson).schema === "spatial.modelspace") {
        const space = ModelSpace.fromJSON(raw as ModelSpaceJson);
        return space.models["spatial.shape"] ?? space.models[Object.keys(space.models)[0]!] ?? new Model();
      }
      return parseModelJson(raw) ?? new Model();
    };

    const seedBoxCell = (model: Model, tag = "e2e-box"): SelectionTarget => {
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [2, 2, 0], height: 1 }, solidRef(tag)));
      return sel("solid", tag);
    };

    const entityCounts = (model: Model) => ({
      vertices: Object.keys(model.vertices).length,
      edges: Object.keys(model.edges).length,
      wires: Object.keys(model.wires).length,
      faces: Object.keys(model.faces).length,
      solids: Object.keys(model.solids).length,
      anchors: Object.keys(model.anchors).length,
    });

    const TRANSFORM_IDS = new Set(["transform.move", "transform.copy", "transform.rotate", "transform.mirror", "transform.scale1d", "transform.scale3d"]);

    const BOX_FACE_TOP = "box-e2e-box-face-top";

    const archivedSelectionTargets = (snap: InteractionSnapshot): readonly SelectionTarget[] => selectionTargetsFromContext(snap.lastResponse?.archiveContext ?? {});

    const assertSelectionCommandArchive = (defn: SelectionOperationInteractionDef, targets: readonly SelectionTarget[], model: Model, activeModelDefinitionId?: string | null): void => {
      switch (defn.operation) {
        case "deselectAll":
          expect(targets).toEqual([]);
          return;
        case "selectAll":
          expect(targets.length).toBeGreaterThanOrEqual(8);
          expect(targets.some((t) => t.kind === "solid" && t.id === "e2e-box")).toBe(true);
          expect(targets.some((t) => t.kind === "vertex")).toBe(true);
          expect(targets.some((t) => t.kind === "face")).toBe(true);
          return;
        case "invert":
          expect(targets.some((t) => t.kind === "solid" && t.id === "e2e-box")).toBe(false);
          expect(targets.length).toBeGreaterThan(0);
          return;
        case "selectKinds": {
          const kinds = defn.kinds ?? [];
          expect(targets.every((t) => kinds.includes(t.kind))).toBe(true);
          const expected = collectGeometrySelectionTargets(model, kinds, activeModelDefinitionId ?? null);
          expect(targets).toEqual(expected);
          if (defn.id === "selection.selectAnchors") {
            expect(targets).toEqual([]);
          } else {
            expect(targets.length).toBeGreaterThan(0);
          }
          return;
        }
      }
    };

    const e2eCases: readonly {
      readonly id: string;
      readonly fixture: InteractionE2EFixtureKind;
      readonly steps: readonly InteractionEvent[];
      readonly seedBox?: boolean;
      readonly useModelObjects?: boolean;
      readonly spec?: InteractionSpec;
      readonly assert?: (ctx: { readonly snap: InteractionSnapshot; readonly model: Model; readonly before: ReturnType<typeof entityCounts>; readonly after: ReturnType<typeof entityCounts>; readonly activeModelDefinitionId?: string | null }) => void;
    }[] = [
      {
        id: "entity.createAnchor",
        fixture: "empty",
        steps: [
          {
            kind: "selection.changed",
            targets: [sel("edge", "box-e2e-box-eb0")],
            point: p(2.5, 0),
            modifiers: MOD,
          },
        ],
        seedBox: true,
        assert: ({ after }) => expect(after.anchors).toBe(1),
      },
      {
        id: "primitive.box",
        fixture: "empty",
        spec: buildBoxInteractionSpec(),
        steps: [
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(3, 2), modifiers: MOD },
          { kind: "set.height", value: 1.25, modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
        assert: ({ after }) => expect(after.solids).toBeGreaterThanOrEqual(1),
      },
      {
        id: "feature.extrudeWire",
        fixture: "loom",
        steps: [
          { kind: "selection.changed", targets: [sel("wire", "w-deck")], modifiers: MOD },
          { kind: "set.distance", value: 1.2, modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
      },
      {
        id: "feature.offsetSurface",
        fixture: "empty",
        steps: [
          { kind: "selection.changed", targets: [sel("face", BOX_FACE_TOP)], modifiers: MOD },
          { kind: "set.distance", value: 0.15, modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
        seedBox: true,
      },
      {
        id: "measure.distance",
        fixture: "loom",
        spec: buildDistanceInteractionSpec(),
        steps: [
          { kind: "selection.changed", targets: [sel("vertex", "v0")], modifiers: MOD },
          { kind: "selection.changed", targets: [sel("vertex", "v1")], modifiers: MOD },
        ],
        assert: ({ snap, after, before }) => {
          expect(typeof snap.lastResponse?.data).toBe("number");
          expect(isEmptyModelDiff(snap.lastResponse?.diff)).toBe(false);
          expect(after.edges).toBeGreaterThan(before.edges);
        },
      },
      {
        id: "measure.area",
        fixture: "empty",
        spec: buildAreaInteractionSpec(),
        steps: [
          { kind: "selection.changed", targets: [sel("face", BOX_FACE_TOP)], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
        seedBox: true,
        assert: ({ snap, after, before }) => {
          expect(typeof snap.lastResponse?.data).toBe("number");
          expect(isEmptyModelDiff(snap.lastResponse?.diff)).toBe(false);
          expect(after.anchors).toBeGreaterThan(before.anchors);
        },
      },
      {
        id: "curve.line",
        fixture: "empty",
        steps: [
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(5, 1), modifiers: MOD },
        ],
        assert: ({ after }) => expect(after.vertices).toBeGreaterThanOrEqual(2),
      },
      {
        id: "curve.polyline",
        fixture: "empty",
        steps: [
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(2, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(4, 2), modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
        assert: ({ after }) => expect(after.vertices).toBeGreaterThanOrEqual(3),
      },
      {
        id: "curve.arc",
        fixture: "empty",
        steps: [
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(2, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(0, 2), modifiers: MOD },
        ],
        assert: ({ after }) => expect(after.edges).toBeGreaterThanOrEqual(1),
      },
      {
        id: "curve.circle",
        fixture: "empty",
        steps: [
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(2, 0), modifiers: MOD },
        ],
        assert: ({ after }) => expect(after.edges).toBeGreaterThanOrEqual(1),
      },
      {
        id: "curve.controlPointCurve",
        fixture: "empty",
        steps: [
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(1, 2), modifiers: MOD },
          { kind: "pointer.down", point: p(4, 0), modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
        assert: ({ after }) => expect(after.edges).toBeGreaterThanOrEqual(1),
      },
      {
        id: "curve.interpolateCurve",
        fixture: "empty",
        steps: [
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(2, 1), modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
        assert: ({ after }) => expect(after.edges).toBeGreaterThanOrEqual(1),
      },
      {
        id: "transform.move",
        fixture: "empty",
        steps: [
          { kind: "start", targets: [sel("solid", "e2e-box")], modifiers: MOD },
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(1, 0.5), modifiers: MOD },
        ],
        assert: ({ model }) => {
          const moved = Object.values(model.vertices).some((v) => v.position[0] > 0.5);
          expect(moved).toBe(true);
        },
      },
      {
        id: "transform.copy",
        fixture: "empty",
        steps: [
          { kind: "start", targets: [sel("solid", "e2e-box")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(4, 0), modifiers: MOD },
        ],
        assert: ({ after, before }) => expect(after.vertices).toBeGreaterThan(before.vertices),
      },
      {
        id: "transform.rotate",
        fixture: "empty",
        steps: [
          { kind: "start", targets: [sel("solid", "e2e-box")], modifiers: MOD },
          { kind: "pointer.down", point: p(1, 1), modifiers: MOD },
          { kind: "pointer.down", point: p(2, 1), modifiers: MOD },
          { kind: "pointer.down", point: p(1, 2), modifiers: MOD },
        ],
      },
      {
        id: "transform.mirror",
        fixture: "empty",
        steps: [
          { kind: "start", targets: [sel("solid", "e2e-box")], modifiers: MOD },
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(3, 0), modifiers: MOD },
        ],
        assert: ({ after }) => expect(after.vertices).toBeGreaterThanOrEqual(1),
      },
      {
        id: "transform.scale1d",
        fixture: "empty",
        steps: [
          { kind: "start", targets: [sel("solid", "e2e-box")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(1, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(2, 0), modifiers: MOD },
        ],
      },
      {
        id: "transform.scale3d",
        fixture: "empty",
        steps: [
          { kind: "start", targets: [sel("solid", "e2e-box")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "pointer.down", point: p(1, 1), modifiers: MOD },
          { kind: "pointer.down", point: p(2, 1), modifiers: MOD },
          { kind: "pointer.down", point: p(1, 2), modifiers: MOD },
        ],
      },
      {
        id: "solid.sphere",
        fixture: "empty",
        steps: [
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(1.5, 0), modifiers: MOD },
        ],
        assert: ({ after }) => expect(after.solids).toBeGreaterThanOrEqual(1),
      },
      {
        id: "solid.cylinder",
        fixture: "empty",
        steps: [
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(1, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(0, 0, 2), modifiers: MOD },
        ],
        assert: ({ after }) => expect(after.solids).toBeGreaterThanOrEqual(1),
      },
      {
        id: "surface.plane",
        fixture: "empty",
        steps: [
          { kind: "pointer.down", point: p(0, 0), modifiers: MOD },
          { kind: "pointer.down", point: p(4, 0), modifiers: MOD },
        ],
      },
      {
        id: "edit.join",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("wire", "stub-wire"), sel("wire", "orbit-a")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
      },
      {
        id: "edit.explode",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("wire", "orbit-a")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
      },
      {
        id: "edit.chamfer",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("edge", "re0")], modifiers: MOD },
          { kind: "selection.changed", targets: [sel("edge", "re1")], modifiers: MOD },
        ],
      },
      {
        id: "edit.fillet",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("edge", "re0")], modifiers: MOD },
          { kind: "selection.changed", targets: [sel("edge", "re2")], modifiers: MOD },
        ],
      },
      {
        id: "edit.split",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("wire", "stub-wire")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "selection.changed", targets: [sel("edge", "re10")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
      },
      {
        id: "edit.trim",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("wire", "orbit-a")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "selection.changed", targets: [sel("edge", "re0")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
        ],
      },
      {
        id: "surface.loft",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("wire", "stub-wire")], modifiers: MOD },
          { kind: "selection.changed", targets: [sel("wire", "orbit-a")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "dialog.ok", modifiers: MOD },
        ],
      },
      {
        id: "surface.sweep1",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("wire", "stub-wire")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "selection.changed", targets: [sel("wire", "spine-b")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "dialog.ok", modifiers: MOD },
        ],
      },
      {
        id: "surface.sweep2",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("wire", "stub-wire")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "selection.changed", targets: [sel("wire", "orbit-a")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "selection.changed", targets: [sel("wire", "spine-b")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "dialog.ok", modifiers: MOD },
        ],
      },
      {
        id: "surface.networkSrf",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("wire", "stub-wire"), sel("wire", "orbit-a")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "dialog.ok", modifiers: MOD },
        ],
      },
      {
        id: "surface.extrudeCrv",
        fixture: "routes",
        steps: [
          { kind: "selection.changed", targets: [sel("wire", "stub-wire")], modifiers: MOD },
          { kind: "confirm", modifiers: MOD },
          { kind: "set.distance", value: 0.8, modifiers: MOD },
        ],
        assert: ({ after, model }) => {
          expect(after.solids).toBeGreaterThanOrEqual(1);
          expect(listModelObjectsForModelDefinition(model, defaultModelDefinitionId()).length).toBeGreaterThanOrEqual(1);
        },
      },
      {
        id: "solid.booleanUnion",
        fixture: "building",
        steps: (() => {
          const c0 = "small-building-cell-123052045";
          const c1 = "small-building-cell-1278694563";
          return [
            { kind: "selection.changed", targets: [sel("solid", c0), sel("solid", c1)], modifiers: MOD },
            { kind: "confirm", modifiers: MOD },
          ];
        })(),
      },
      {
        id: "solid.booleanDifference",
        fixture: "building",
        steps: (() => {
          const c0 = "small-building-cell-123052045";
          const c1 = "small-building-cell-1278694563";
          return [
            { kind: "selection.changed", targets: [sel("solid", c0)], modifiers: MOD },
            { kind: "confirm", modifiers: MOD },
            { kind: "selection.changed", targets: [sel("solid", c1)], modifiers: MOD },
            { kind: "confirm", modifiers: MOD },
          ];
        })(),
      },
      {
        id: "solid.booleanIntersection",
        fixture: "building",
        steps: (() => {
          const c0 = "small-building-cell-123052045";
          const c1 = "small-building-cell-1278694563";
          return [
            { kind: "selection.changed", targets: [sel("solid", c0)], modifiers: MOD },
            { kind: "confirm", modifiers: MOD },
            { kind: "selection.changed", targets: [sel("solid", c1)], modifiers: MOD },
            { kind: "confirm", modifiers: MOD },
          ];
        })(),
      },
    ];

    it("covers every shipped interaction", () => {
      const ids = listSpatialInteractionsForModelDefinition(defaultModelDefinitionId())
        .map((row) => row.id)
        .filter((id) => {
          const spec = loadSpatialInteraction(id);
          return spec && !isCallableOnlyInteraction(spec);
        })
        .sort();
      expect(e2eCases.map((c) => c.id).sort()).toEqual(ids);
    });

    it.each(listSelectionOperationsForModelDefinition(defaultModelDefinitionId()))("$id selection action completes on seeded box", async (defn) => {
      const model = modelFromFixture("empty");
      seedBoxCell(model);
      if (defn.kinds?.includes("object")) {
        const solidId = Object.keys(model.solids)[0]!;
        model.objects["e2e-object"] = {
          id: "e2e-object" as ObjectRef,
          typology: "spatial.shape.primitive.box" as TypologyRef,
          primitives: { solid: solidId },
        };
      }
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const seed = selectionSeedTargetsForOperation(defn.operation);
      const result = await runSelectionOperationInteraction(defn.id, {
        kernel,
        document: { model, nodes: [] },
        seedTargets: seed,
      });
      expect(isEmptyModelDiff(result.response.diff ?? EMPTY_MODEL_DIFF)).toBe(true);
      assertSelectionCommandArchive(defn, result.targets, model);
    });

    it.each(e2eCases)("$id completes end-to-end on $fixture fixture", async (row) => {
      const spec = row.spec ?? loadSpatialInteraction(row.id);
      expect(spec).not.toBeNull();
      const model = modelFromFixture(row.fixture);
      if (row.seedBox || TRANSFORM_IDS.has(row.id)) seedBoxCell(model);
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const before = entityCounts(model);
      const rt = createInteractionRuntime(spec!, {
        kernel,
        document: { model: model, nodes: [] },
      });
      for (const step of row.steps) {
        await rt.send(step);
        if (isFinalInteractionState(spec!, rt.getSnapshot().state)) break;
      }
      const st = rt.getSnapshot().state;
      if (st === "ready") await rt.send({ kind: "confirm", modifiers: MOD });
      const snap = rt.getSnapshot();
      expect(snap.state, row.id).toBe("committed");
      expect(snap.lastResponse?.ok, row.id).toBe(true);
      expect(snap.lastResponse?.errors ?? [], row.id).toEqual([]);
      row.assert?.({ snap, model, before, after: entityCounts(model) });
    });
  });
}
// #endregion 🧪️Tests

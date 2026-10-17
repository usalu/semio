/** 🎰️ CAD editor actions: the interaction state engine, the spatial and numeric entry of an interaction, its engagement controls, keybinds and the pure state-engine provider. */
import type { Vec3 } from "@semio-tech/framework/3d";
import { FaceRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { EffectSpec, InteractionEngagementControlKind, InteractionEngagementEntryControl, InteractionLengthEntrySpec, InteractionOutputBinding, InteractionScalarEntrySpec, InteractionSpec, ResolvedInteractionEngagementControl, StateDefSpec, initialContextForSpec } from "@semio-tech/s-spatial-kernel-ts/geometry/interaction-spec";
import { PathSegment, writePathSegments } from "@semio-tech/s-spatial-kernel-ts/geometry/paths";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { assertActionAvailableInModelDefinition } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-ownership";
import { Expr, ExprEnv, clearPathTarget, evalExpr, evalGuard, readPathTarget, writePathTarget } from "@semio-tech/s-spatial-kernel-ts/geometry/expressions";
import { InteractionEvent, SelectionEvent } from "@semio-tech/s-spatial-kernel-ts/geometry/selection";
import { KernelQueryContext, SpatialKernel, SpatialPreviewKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { InteractionRuntime } from "../../🗿️artifact/🏃️interaction-runtime/🟦️.ts";
import { type ActionRegistry, applyActionPatchToContext } from "../🧬️action-contract/🟦️.ts";
import { modelDefinitionActionRegistry, runRegisteredAction } from "../🏃️action-execution/🟦️.ts";

// #region 📦️🎬️actions
// #region 🎬️Statechart
function findState(spec: InteractionSpec, name: string): StateDefSpec | undefined {
  return spec.machine.states.find((s) => s.name === name);
}

function listFinalInteractionStates(spec: InteractionSpec): string[] {
  return spec.machine.states.filter((s) => s.final).map((s) => s.name);
}

/** 📞️ Pauses host statechart until nested interaction completes or aborts. */
export interface InteractionChildCallSpec {
  readonly interactionId: string;
  readonly inputs?: Record<string, Expr>;
  readonly outputs?: readonly InteractionOutputBinding[] | Record<string, unknown>;
  readonly resumeTarget: string;
  readonly rollback: { readonly state: string; readonly context: Record<string, unknown> };
}

/** 🎭️ Result of `StateEngine.send` / `applyTransition` (`transient` skips interaction-local undo). */
export interface StateEngineSendResult {
  readonly ok: boolean;
  readonly transient?: boolean;
  readonly childCall?: InteractionChildCallSpec;
}

/** 🎭️ `applyTransition` output: next factory state + disambiguation index for XState routing. */
export interface ApplyTransitionResult extends StateEngineSendResult {
  readonly nextState: string;
  readonly branchIndex: number;
}

/** 🎭️ Pluggable state backend for `InteractionRuntime` (pure TS, XState, …). */
export interface StateEngine {
  getState(): string;
  getContext(): Record<string, unknown>;
  reset(): void;
  restore(state: string, context: Record<string, unknown>): void;
  send(event: InteractionEvent, kernel?: SpatialKernel, model?: Model, actions?: ActionRegistry, preview?: SpatialPreviewKernel, activeModelDefinitionId?: string | null): Promise<StateEngineSendResult>;
}

/** 🎭️ Instantiates a `StateEngine` for a compiled `InteractionSpec`. */
export interface StateEngineProvider {
  readonly id: string;
  create(spec: InteractionSpec): StateEngine;
}

function lookupGuard(spec: InteractionSpec, name: string): Expr | undefined {
  return spec.guards?.find((g) => g.name === name)?.expr;
}

/** 🎬️ Applies one declarative transition `EffectSpec` (async kernel queries + registered `ActionRegistry` calls). */
export async function applyEffectAsync(
  a: EffectSpec,
  ctx: Record<string, unknown>,
  event: InteractionEvent,
  kernel: SpatialKernel | undefined,
  model: Model,
  actions?: ActionRegistry,
  preview?: SpatialPreviewKernel,
  activeModelDefinitionId?: string | null,
): Promise<void> {
  const math = preview ?? kernel;
  if (!math) return;
  const env: ExprEnv = { context: ctx, event, model, activeModelDefinitionId, preview: math };
  const reg = actions ?? modelDefinitionActionRegistry();
  if (a.mutation === "assign") {
    const v = evalExpr(a.value, env);
    writePathTarget(a.target, env, v);
  } else if (a.mutation === "clear") {
    clearPathTarget(a.target, env);
  } else if (a.mutation === "append") {
    const cur = readPathTarget(a.target, env);
    const v = evalExpr(a.value, env);
    if (Array.isArray(cur)) {
      const next = [...cur, v];
      writePathTarget(a.target, env, next);
    }
  } else if (a.mutation === "kernel.query") {
    const queryCtx: KernelQueryContext = { model, activeModelDefinitionId };
    if (a.query === "face.resolveIds") {
      const target = (event as SelectionEvent).targets?.[0];
      const kind = target?.kind ?? "face";
      const id = target?.id ?? "";
      const faceIds = kind === "face" && id ? [id as FaceRef] : [];
      writePathTarget(a.assignTo, env, faceIds);
    } else if (kernel?.query) {
      const params: Record<string, unknown> = {};
      const res = await kernel.query(a.query, params, queryCtx);
      writePathTarget(a.assignTo, env, res);
    }
  } else if (a.mutation === "interaction.call") {
    return;
  } else if (a.mutation === "action") {
    const def = reg.get(a.action);
    if (!def) return;
    assertActionAvailableInModelDefinition(a.action, activeModelDefinitionId);
    const paramBag: Record<string, unknown> = { __context: ctx, __event: event };
    for (const [k, ex] of Object.entries(a.params ?? {})) {
      paramBag[k] = evalExpr(ex, env);
    }
    const k = kernel ?? (null as unknown as SpatialKernel);
    const r = await runRegisteredAction(reg, a.action, paramBag, { kernel: k, preview: math, model, activeModelDefinitionId: activeModelDefinitionId ?? null });
    if (r.patch) applyActionPatchToContext(ctx, r.patch);
  }
}

/** 🎬️ First matching transition for `event` from `state`; mutates `context` in place. */
export async function applyTransition(
  spec: InteractionSpec,
  state: string,
  context: Record<string, unknown>,
  event: InteractionEvent,
  kernel?: SpatialKernel,
  actions?: ActionRegistry,
  model?: Model,
  preview?: SpatialPreviewKernel,
  activeModelDefinitionId?: string | null,
): Promise<ApplyTransitionResult> {
  const graph = model ?? new Model();
  const st = findState(spec, state);
  const handler = st?.on?.find((h) => h.event === event.kind);
  if (!handler) return { ok: false, nextState: state, branchIndex: -1 };
  const choices = handler.transitions;
  if (choices.length === 0) return { ok: false, nextState: state, branchIndex: -1 };
  for (let i = 0; i < choices.length; i++) {
    const tr = choices[i]!;
    if (tr.guard) {
      const g = lookupGuard(spec, tr.guard);
      const math = preview ?? kernel;
      if (!g || !math || !evalGuard(g, { context, event, preview: math })) continue;
    }
    const rollbackState = state;
    const rollbackContext = structuredClone(context) as Record<string, unknown>;
    let childCall: InteractionChildCallSpec | undefined;
    const resumeTarget = tr.target ?? state;
    for (const eff of tr.effects ?? []) {
      if (eff.mutation === "interaction.call") {
        childCall = {
          interactionId: eff.interaction,
          inputs: eff.inputs,
          outputs: eff.outputs,
          resumeTarget,
          rollback: { state: rollbackState, context: rollbackContext },
        };
        continue;
      }
      await applyEffectAsync(eff, context, event, kernel, graph, actions, preview, activeModelDefinitionId ?? null);
    }
    if (childCall) {
      return { ok: true, transient: Boolean(tr.transient), nextState: state, childCall, branchIndex: i };
    }
    let nextState = state;
    if (tr.target) {
      nextState = tr.target;
      if (tr.target === spec.machine.initial && tr.target !== state) {
        for (const k of Object.keys(context)) delete context[k];
      }
    }
    return { ok: true, transient: Boolean(tr.transient), nextState, branchIndex: i };
  }
  return { ok: false, nextState: state, branchIndex: -1 };
}

/** ⌨️ Resolved spatial host hints (defaults disable ground picking). */
export interface InteractionSpatialResolved {
  readonly spatialGroundPick: boolean;
  readonly pickDisabledStates: readonly string[];
  readonly groundPointerMoveStates: readonly string[];
  readonly heightDragStates: readonly string[];
  readonly verticalRodStates: readonly string[];
  readonly heightConfirmState: string | null;
  readonly lengthEntry: readonly InteractionLengthEntrySpec[];
  readonly scalarEntry: readonly InteractionScalarEntrySpec[];
}

/** ⌨️ Merges `spec.interaction` with safe defaults for hosts and `InteractionSpatialView`. */
export function mergeInteractionSpatial(spec: InteractionSpec): InteractionSpatialResolved {
  const i = spec.interaction;
  const basePickDisabled = [...new Set([spec.machine.initial, "ready", ...listFinalInteractionStates(spec)])];
  return {
    spatialGroundPick: Boolean(i?.spatialGroundPick),
    pickDisabledStates: i?.pickDisabledStates ?? [...basePickDisabled],
    groundPointerMoveStates: i?.groundPointerMoveStates ?? [],
    heightDragStates: i?.heightDragStates ?? [],
    verticalRodStates: i?.verticalRodStates ?? [],
    heightConfirmState: i?.heightConfirmState === undefined ? null : i.heightConfirmState,
    lengthEntry: i?.lengthEntry ?? [],
    scalarEntry: i?.scalarEntry ?? [],
  };
}

/** 📏️ Resolved direct-distance entry config for `state` (from `interaction.lengthEntry`). */
export function interactionLengthEntryForState(spec: InteractionSpec, state: string): InteractionLengthEntrySpec | null {
  return mergeInteractionSpatial(spec).lengthEntry.find((row) => row.state === state) ?? null;
}

/** 🔢️ Resolved live scalar entry config for `state` (from `interaction.scalarEntry`). */
export function interactionScalarEntryForState(spec: InteractionSpec, state: string): InteractionScalarEntrySpec | null {
  return mergeInteractionSpatial(spec).scalarEntry.find((row) => row.state === state) ?? null;
}

function engagementLabelForInteractionEntry(field: string, event?: string): string {
  if (event === "set.height") return "Height";
  if (event === "set.distance") return "Distance";
  if (event === "set.angle") return "Angle";
  if (event === "set.radius" || field === "radius") return "Radius";
  const trimmed = field.trim();
  if (!trimmed) return "Value";
  return trimmed.replace(/[._-]+/g, " ").replace(/\b\w/g, (character) => character.toUpperCase());
}

function interactionContextNumericField(context: Record<string, unknown>, field: string, fallback: number): number {
  const raw = context[field];
  if (typeof raw === "number" && Number.isFinite(raw)) return raw;
  return fallback;
}

function defaultEngagementControlKind(entry: InteractionEngagementEntryControl, event?: string): InteractionEngagementControlKind {
  if (entry.control) return entry.control;
  if (event === "set.angle") return "ring";
  return "stepper";
}

/** 🎛️ Resolves declarative engagement control params for `state` (length/scalar entry + context value). */
export function interactionControlForState(spec: InteractionSpec, state: string, context: Record<string, unknown> = {}): ResolvedInteractionEngagementControl | null {
  const scalar = interactionScalarEntryForState(spec, state);
  if (scalar) {
    const kind = defaultEngagementControlKind(scalar, scalar.event);
    const label = engagementLabelForInteractionEntry(scalar.field, scalar.event);
    const value = interactionContextNumericField(context, scalar.field, scalar.default ?? 1);
    const step = scalar.step ?? (scalar.event === "set.angle" ? 15 : 0.1);
    const unit = scalar.unit ?? (scalar.event === "set.angle" ? "°" : "m");
    if (kind === "ring" || scalar.event === "set.angle") {
      const min = scalar.min ?? 0;
      const max = scalar.max ?? 360;
      const segments = Math.max(4, Math.min(36, Math.round((max - min) / step) || 12));
      const options = Array.from({ length: segments }, (_, index) => {
        const numeric = min + (index * (max - min)) / segments;
        const rounded = Math.round(numeric);
        return { id: `angle-${rounded}`, label: `${rounded}°` };
      });
      const nearest = options.reduce((best, row) => {
        const rowValue = Number(row.id.slice("angle-".length));
        const bestValue = Number(best.id.slice("angle-".length));
        return Math.abs(rowValue - value) < Math.abs(bestValue - value) ? row : best;
      }, options[0]!);
      return { kind: "ring", label, value: nearest.id, options, min, max, step };
    }
    const min = scalar.min ?? 0;
    if (kind === "slider") {
      return { kind: "slider", label, value, min, max: scalar.max ?? Math.max(min + 1, value * 2, 10), step, unit };
    }
    return { kind: "stepper", label, value, min, max: scalar.max, step, unit };
  }
  const length = interactionLengthEntryForState(spec, state);
  if (!length) return null;
  const kind = defaultEngagementControlKind(length);
  const label = engagementLabelForInteractionEntry(length.field);
  const live = interactionLengthEntryLiveDistance(context, length);
  const value = live ?? length.default ?? 1;
  const step = length.step ?? 0.1;
  const unit = length.unit ?? "m";
  const min = length.min ?? 0;
  if (kind === "slider") {
    return { kind: "slider", label, value, min, max: length.max ?? Math.max(min + 1, value * 2, 10), step, unit };
  }
  if (kind === "ring") {
    const max = length.max ?? Math.max(min + step, value * 2, 10);
    const segments = Math.max(4, Math.min(24, Math.round((max - min) / step) || 8));
    const options = Array.from({ length: segments }, (_, index) => {
      const numeric = min + (index * (max - min)) / segments;
      const rounded = Math.round(numeric * 100) / 100;
      return { id: `length-${rounded}`, label: `${rounded}${unit}` };
    });
    const nearest = options.reduce((best, row) => {
      const rowValue = Number(row.id.slice("length-".length));
      const bestValue = Number(best.id.slice("length-".length));
      return Math.abs(rowValue - value) < Math.abs(bestValue - value) ? row : best;
    }, options[0]!);
    return { kind: "ring", label, value: nearest.id, options, min, max, step };
  }
  return { kind: "stepper", label, value, min, max: length.max, step, unit };
}

/** 🔢️ True when `state` accepts live REPL numeric entry (length or scalar). */
export function interactionInNumericEntryState(spec: InteractionSpec, state: string): boolean {
  return interactionLengthEntryForState(spec, state) !== null || interactionScalarEntryForState(spec, state) !== null;
}

/** 🔢️ Parses REPL `cmdLine` as a live numeric value (`null` = empty, `undefined` = invalid). */
export function parseNumericCommandLine(cmdLine: string): number | null | undefined {
  const t = cmdLine.trim();
  if (!t) return null;
  if (!/^\d*\.?\d*$/.test(t)) return undefined;
  const v = Number(t);
  if (!Number.isFinite(v) || v <= 0) return undefined;
  return v;
}

/** 📏️ Live distance along a length-entry anchor→cursor axis (extrusion rod, rubber band). */
export function interactionLengthEntryLiveDistance(ctx: Record<string, unknown>, entry: InteractionLengthEntrySpec): number | null {
  const anchor = readInteractionContextVec3(ctx, entry.anchor);
  const cursor = readInteractionContextVec3(ctx, entry.field);
  if (!anchor || !cursor) return null;
  const direction = readInteractionContextVec3(ctx, "direction");
  if (direction) {
    const len = Math.hypot(direction[0], direction[1], direction[2]) || 1;
    const dir: Vec3 = [direction[0] / len, direction[1] / len, direction[2] / len];
    const distance = Math.abs((cursor[0] - anchor[0]) * dir[0] + (cursor[1] - anchor[1]) * dir[1] + (cursor[2] - anchor[2]) * dir[2]);
    return distance > 1e-9 ? distance : null;
  }
  const distance = Math.hypot(cursor[0] - anchor[0], cursor[1] - anchor[1], cursor[2] - anchor[2]);
  return distance > 1e-9 ? distance : null;
}

/** 🔢️ Explicit length/height lock from context (`set.length` / `set.height`), not live rubber-band distance. */
export function interactionNumericEntryExplicitLockValue(spec: InteractionSpec, state: string, ctx: Record<string, unknown>): number | null {
  const lengthEntry = interactionLengthEntryForState(spec, state);
  if (lengthEntry) {
    const lock = ctx[LENGTH_LOCK_CTX];
    if (typeof lock === "number" && Number.isFinite(lock) && lock > 0) return lock;
  }
  const scalarEntry = interactionScalarEntryForState(spec, state);
  if (scalarEntry) {
    const heightLock = ctx[HEIGHT_LOCK_CTX];
    if (typeof heightLock === "number" && Number.isFinite(heightLock) && heightLock > 0) return heightLock;
  }
  return null;
}

/** 🔢️ Locked numeric value from context when live entry already applied. */
export function interactionNumericEntryLockedValue(spec: InteractionSpec, state: string, ctx: Record<string, unknown>): number | null {
  const lengthEntry = interactionLengthEntryForState(spec, state);
  if (lengthEntry) {
    const lock = interactionNumericEntryExplicitLockValue(spec, state, ctx);
    if (lock != null) return lock;
    const live = interactionLengthEntryLiveDistance(ctx, lengthEntry);
    if (live != null) return live;
  }
  const scalarEntry = interactionScalarEntryForState(spec, state);
  if (scalarEntry) {
    const heightLock = ctx[HEIGHT_LOCK_CTX];
    if (typeof heightLock === "number" && Number.isFinite(heightLock) && heightLock > 0) return heightLock;
    const v = ctx[scalarEntry.field];
    if (typeof v === "number" && Number.isFinite(v) && v > 0) return v;
  }
  return null;
}

/** 🔢️ `set.length` / `set.height` event to apply a numeric value in the active entry state. */
export function interactionNumericEntryApplyEvent(spec: InteractionSpec, state: string, value: number): InteractionEvent | null {
  const lengthEntry = interactionLengthEntryForState(spec, state);
  if (lengthEntry) return { kind: "set.length", value, modifiers: {} };
  const scalarEntry = interactionScalarEntryForState(spec, state);
  if (scalarEntry) return { kind: scalarEntry.event, value, modifiers: {} };
  return null;
}

function lengthEntryCommitPoint(ctx: Record<string, unknown>, entry: InteractionLengthEntrySpec, preview: SpatialPreviewKernel): Vec3 | null {
  const fromField = readInteractionContextVec3(ctx, entry.field);
  if (fromField) return fromField;
  const lock = positiveLengthLock(ctx);
  if (lock == null) return null;
  const raw = lengthEntryRawPoint(ctx, entry);
  const anchor = readInteractionContextVec3(ctx, entry.anchor);
  if (!raw || !anchor) return null;
  return preview.clampPointAlongDirection(anchor, raw, lock);
}

/** 🔢️ Commit event after numeric entry (Enter/Space): `pointer.down` with clamped point or `confirm`. */
export function interactionNumericEntryCommitEvent(spec: InteractionSpec, state: string, ctx: Record<string, unknown>, preview: SpatialPreviewKernel): InteractionEvent | null {
  const lengthEntry = interactionLengthEntryForState(spec, state);
  const scalarEntry = interactionScalarEntryForState(spec, state);
  if (!lengthEntry && !scalarEntry) return null;
  const st = findState(spec, state);
  if (!st?.on) return null;
  const events = new Set(st.on.map((h) => h.event));
  const commitKind = scalarEntry?.commit ?? lengthEntry?.commit ?? (scalarEntry ? "confirm" : events.has("pointer.down") ? "pointer.down" : "confirm");
  if (commitKind === "pointer.down" && events.has("pointer.down") && lengthEntry) {
    const point = lengthEntryCommitPoint(ctx, lengthEntry, preview);
    if (point) return { kind: "pointer.down", point, modifiers: {} };
    return null;
  }
  if (commitKind === "confirm" && events.has("confirm")) return { kind: "confirm", modifiers: {} };
  if (events.has("pointer.down") && lengthEntry) {
    const point = lengthEntryCommitPoint(ctx, lengthEntry, preview);
    if (point) return { kind: "pointer.down", point, modifiers: {} };
  }
  if (events.has("confirm")) return { kind: "confirm", modifiers: {} };
  return null;
}

/** ✅️ Whether `state` has a passable `confirm` transition (non-selection finalize). */
export function interactionCanFinalizeStep(spec: InteractionSpec, state: string, ctx: Record<string, unknown>, preview: SpatialPreviewKernel): boolean {
  const handler = spec.machine.states.find((s) => s.name === state)?.on?.find((h) => h.event === "confirm");
  if (!handler) return false;
  for (const tr of handler.transitions) {
    if (tr.guard) {
      const g = lookupGuard(spec, tr.guard);
      if (!g || !evalGuard(g, { context: ctx, preview })) continue;
    }
    return true;
  }
  return false;
}

/** ✅️ Enter/Space finalize: `confirm` when available, else length-entry `pointer.down`. */
export function interactionStepFinalizeEvent(spec: InteractionSpec, state: string, ctx: Record<string, unknown>, preview: SpatialPreviewKernel): InteractionEvent | null {
  if (interactionCanFinalizeStep(spec, state, ctx, preview)) return { kind: "confirm", modifiers: {} };
  return interactionNumericEntryCommitEvent(spec, state, ctx, preview);
}

const LENGTH_LOCK_CTX = "__lengthLock";
const HEIGHT_LOCK_CTX = "__heightLock";
export const SCALAR_AXIS_T_CTX = "__scalarAxisT";
export const CURSOR_RAW_CTX = "__cursorRaw";

const DEFAULT_SCALAR_AXIS: Vec3 = [0, 0, 1];

/** 📏️ Axis base for scalar rubber-band (`axisAnchor` XY + `axisFloor` Z). */
export function scalarEntryAxisBase(ctx: Record<string, unknown>, entry: InteractionScalarEntrySpec): Vec3 | null {
  if (!entry.axisAnchor) return null;
  const anchor = readInteractionContextVec3(ctx, entry.axisAnchor);
  if (!anchor) return null;
  const floorPath = entry.axisFloor ?? entry.axisAnchor;
  const floor = readInteractionContextVec3(ctx, floorPath);
  const floorZ = floor ? floor[2] : anchor[2];
  return [anchor[0], anchor[1], floorZ];
}

/** 📏️ Projects `raw` onto the scalar axis; returns axis parameter `t` and closest point. */
export function projectPointOnScalarAxis(base: Vec3, axis: Vec3, raw: Vec3, preview: SpatialPreviewKernel): { readonly projected: Vec3; readonly t: number } {
  return preview.projectPointOnScalarAxis(base, axis, raw);
}

export function scalarEntryAxis(entry: InteractionScalarEntrySpec): Vec3 {
  const a = entry.axis;
  if (a && a.length === 3) return [a[0], a[1], a[2]];
  return DEFAULT_SCALAR_AXIS;
}

export function positiveHeightLock(ctx: Record<string, unknown>): number | null {
  const lock = ctx[HEIGHT_LOCK_CTX];
  return typeof lock === "number" && Number.isFinite(lock) && lock > 0 ? lock : null;
}

function scalarHeightFromAxisT(t: number): number {
  return Math.max(0.01, Math.abs(t));
}

/** 📏️ Parses a dotted `context` path into `PathSegment`s (`points.@last` = last array element). */
export function parseInteractionContextPath(path: string): readonly PathSegment[] {
  const parts = path.split(".").filter((p) => p.length > 0);
  const segs: PathSegment[] = [];
  for (const part of parts) {
    if (part === "@last") {
      segs.push({ kind: "index", index: -1 });
    } else {
      segs.push({ kind: "field", name: part });
    }
  }
  return segs;
}

function readContextPathValue(root: Record<string, unknown>, segments: readonly PathSegment[]): unknown {
  let cur: unknown = root;
  for (const seg of segments) {
    if (cur === null || cur === undefined) return undefined;
    if (seg.kind === "field") {
      if (typeof cur !== "object" || Array.isArray(cur)) return undefined;
      cur = (cur as Record<string, unknown>)[seg.name];
    } else {
      if (!Array.isArray(cur)) return undefined;
      const idx = seg.index < 0 ? cur.length + seg.index : seg.index;
      cur = cur[idx];
    }
  }
  return cur;
}

/** 📏️ Reads a `Vec3` from `context` at dotted `path` (supports `points.@last`). */
export function readInteractionContextVec3(ctx: Record<string, unknown>, path: string): Vec3 | null {
  const raw = readContextPathValue(ctx, parseInteractionContextPath(path));
  if (!Array.isArray(raw) || raw.length < 3) return null;
  const x = Number(raw[0]);
  const y = Number(raw[1]);
  const z = Number(raw[2]);
  if (!Number.isFinite(x) || !Number.isFinite(y) || !Number.isFinite(z)) return null;
  return [x, y, z];
}

/** 📏️ Writes a `Vec3` into `context` at dotted `path`. */
export function writeInteractionContextVec3(ctx: Record<string, unknown>, path: string, value: Vec3): void {
  writePathSegments(ctx, parseInteractionContextPath(path), value);
}

/** 📏️ Clamps `target` to `length` units from `anchor` along the anchor→target ray. */
export function clampPointAlongDirection(anchor: Vec3, target: Vec3, length: number, preview: SpatialPreviewKernel): Vec3 {
  return preview.clampPointAlongDirection(anchor, target, length);
}

export function positiveLengthLock(ctx: Record<string, unknown>): number | null {
  const lock = ctx[LENGTH_LOCK_CTX];
  return typeof lock === "number" && Number.isFinite(lock) && lock > 0 ? lock : null;
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

/** ⌨️ One host-triggerable transition row for palette + command input (see `TransitionSpec.key`). */
const HOST_KEYBIND_EXCLUDED_KINDS = new Set(["pointer.move", "pointer.down", "selection.changed"]);

export interface InteractionKeybindRow {
  readonly eventKind: string;
  readonly key: string;
  readonly label: string;
}

/** ⌨️ Lists keyed transitions for the active state (excludes pointer + selection). */
export function listKeyedInteractionTransitions(spec: InteractionSpec, state: string): readonly InteractionKeybindRow[] {
  const st = findState(spec, state);
  if (!st?.on) return [];
  const out: InteractionKeybindRow[] = [];
  for (const h of st.on) {
    if (HOST_KEYBIND_EXCLUDED_KINDS.has(h.event)) continue;
    for (const tr of h.transitions) {
      if (tr.transient) continue;
      const key = tr.key;
      const label = tr.label;
      if (typeof key !== "string" || key.length === 0) continue;
      if (typeof label !== "string" || label.length === 0) continue;
      out.push({ eventKind: h.event, key, label });
    }
  }
  return out;
}

/** ⎋️ Hard-aborts the active interaction session when `capabilities.canCancel`. */
export function abortActiveInteractionSession(rt: InteractionRuntime): boolean {
  if (!rt.getSnapshot().capabilities.canCancel) return false;
  rt.cancel();
  return true;
}

/** 🎬️ Minimal async statechart runner for `InteractionSpec.machine`. */
export class StatechartRuntime implements StateEngine {
  private state: string;
  private context: Record<string, unknown>;

  constructor(private readonly spec: InteractionSpec) {
    this.state = spec.machine.initial;
    this.context = initialContextForSpec(spec);
  }

  getState(): string {
    return this.state;
  }

  getContext(): Record<string, unknown> {
    return this.context;
  }

  reset(): void {
    this.state = this.spec.machine.initial;
    this.context = initialContextForSpec(this.spec);
  }

  /** 🎬️ Restores a prior `state` + `context` snapshot (interaction-local undo). */
  restore(state: string, context: Record<string, unknown>): void {
    this.state = state;
    this.context = context;
  }

  /** 🎬️ Applies one external event; returns whether a transition fired. */
  async send(event: InteractionEvent, kernel?: SpatialKernel, model?: Model, actions?: ActionRegistry, preview?: SpatialPreviewKernel, activeModelDefinitionId?: string | null): Promise<StateEngineSendResult> {
    const r = await applyTransition(this.spec, this.state, this.context, event, kernel, actions, model, preview, activeModelDefinitionId ?? null);
    if (!r.ok) return { ok: false };
    if (r.childCall) return { ok: true, transient: r.transient, childCall: r.childCall };
    this.state = r.nextState;
    return { ok: true, transient: r.transient };
  }
}

/** 🎭️ Default in-process engine (no XState); same semantics as `applyTransition`. */
export const pureTsStateEngineProvider: StateEngineProvider = {
  id: "pure-ts",
  create(spec: InteractionSpec): StateEngine {
    return new StatechartRuntime(spec);
  },
};
// #endregion 🎬️Statechart
// #endregion 📦️🎬️actions

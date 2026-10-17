/** 🏃️ CAD editor actions: the box and transform helpers, the model-definition capability table, the declarative runtime, the action registry and the selection-apply runner. */
import type { Vec3 } from "@semio-tech/framework/3d";
import { AnchorAttachment, AnchorRecord, AnchorRef, EdgeRef, FaceRef, SolidRef, VertexRecord, VertexRef, WireRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { assertActionAvailableInModelDefinition } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-ownership";
import { Expr, ExprEnv, evalExpr } from "@semio-tech/s-spatial-kernel-ts/geometry/expressions";
import { InteractionEvent, SelectionTarget } from "@semio-tech/s-spatial-kernel-ts/geometry/selection";
import { capabilityActionSpecJson, ensureTypologyObjectFromCreateDiff } from "../../🧬️typology/🟦️.ts";
import { EMPTY_MODEL_DIFF, ModelDiff, SpatialKernel, SpatialPreviewKernel, isEmptyModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { type ActionContextPatch, type ActionDef, type ActionRegistry, type ActionResult, type ActionSpec, shippedActionCatalog } from "../🧬️action-contract/🟦️.ts";
import { collectTargetVertices, executeSelectionApply, faceAnnotationCentroid, type InteractionEventModifiers, type InteractionPointSnap, interactionPointSnapFromEvent, parseSelectionTargetsFromUnknown, type SelectionApplyParams, selectionApplyParamsFromRecord, selectionCommandActionResult, selectionTargetsCenter, selectionTargetsFromActionResult, selectionTargetsPointTransformDiff, selectionTargetsWithMode, vertexPositionsTransformDiff, withResolvedInteractionPointsContext } from "../🧲️selection-targets/🟦️.ts";

// #region 📦️🎬️actions
// #region 🧮️ActionRegistry
function evalExprRecord(record: Record<string, Expr> | undefined, env: ExprEnv): Record<string, unknown> {
  const out: Record<string, unknown> = {};
  for (const [k, v] of Object.entries(record ?? {})) out[k] = evalExpr(v, env);
  return out;
}

function vec3Param(params: Record<string, unknown>, name: string, fallback: Vec3 = [0, 0, 0]): Vec3 {
  const v = params[name];
  return Array.isArray(v) && v.length >= 3 ? [Number(v[0]), Number(v[1]), Number(v[2])] : fallback;
}

function numericParam(params: Record<string, unknown>, name: string, fallback = 0): number {
  const v = params[name];
  return typeof v === "number" && Number.isFinite(v) ? v : fallback;
}

async function runCreateBox(params: Record<string, unknown>, ctx: { readonly kernel: SpatialKernel; readonly preview: SpatialPreviewKernel }): Promise<ActionResult> {
  const cornerA = vec3Param(params, "cornerA", vec3Param(params, "p0"));
  const cornerB = vec3Param(params, "cornerB", vec3Param(params, "p1", [1, 1, 0]));
  const p2 = vec3Param(params, "p2", cornerB);
  const height = numericParam(params, "height", Math.max(Math.abs(cornerB[0] - cornerA[0]), Math.abs(cornerB[1] - cornerA[1]), Math.abs(p2[2] - cornerA[2]), 1));
  if (ctx.kernel.createBoxFromCornersDiff) {
    const result = await ctx.kernel.createBoxFromCornersDiff({ cornerA, cornerB, height });
    return { diff: result.diff, data: { solid: result.solid } };
  }
  const solid = await ctx.kernel.createBoxFromCorners({ cornerA, cornerB, height });
  return { diff: ctx.preview.boxModelDiff({ cornerA, cornerB, height }, solid), data: { solid } };
}

function transformDiff(params: Record<string, unknown>, ctx: { readonly model: Model; readonly preview: SpatialPreviewKernel }, copy: boolean): ModelDiff {
  const targets = parseSelectionTargetsFromUnknown(params.targets ?? params.selection ?? params.seedTargets);
  const from = vec3Param(params, "from", selectionTargetsCenter(ctx.model, targets, ctx.preview) ?? [0, 0, 0]);
  const rawTo = vec3Param(params, "to", vec3Param(params, "cursor", from));
  const to = ctx.preview.constrainMovePoint(from, rawTo, String(params.moveMode ?? params.mode ?? "free"), vec3Param(params, "cplaneNormal", [0, 0, 1]));
  const delta = ctx.preview.vec3Sub(to, from);
  if (copy) {
    const ids = collectTargetVertices(ctx.model, targets);
    const added: VertexRecord[] = [];
    for (const id of ids) {
      const v = ctx.model.vertices[id];
      if (!v) continue;
      added.push({ id: `${id}-copy-${Math.random().toString(36).slice(2, 8)}` as VertexRef, position: ctx.preview.vec3Add(v.position, delta) });
    }
    return added.length ? { vertices: { added } } : EMPTY_MODEL_DIFF;
  }
  return selectionTargetsPointTransformDiff(ctx.model, targets, (point) => ctx.preview.vec3Add(point, delta));
}

function anchorAction(params: Record<string, unknown>, ctx: { readonly model: Model; readonly preview: SpatialPreviewKernel }): ActionResult {
  const hostKind = String(params.hostKind ?? "vertex") as AnchorAttachment["kind"];
  const hostId = String(params.hostId ?? "");
  const hitPoint = vec3Param(params, "hitPoint");
  const placement = ctx.preview.anchorPlacementFromEntity(ctx.model, hostKind, hostId, hitPoint);
  if (!placement) return { diff: EMPTY_MODEL_DIFF };
  const anchor: AnchorRecord = { id: `anchor-${Math.random().toString(36).slice(2, 10)}` as AnchorRef, ...placement };
  return { diff: { anchors: { added: [anchor] } }, data: anchor };
}

function modelDefinitionActionCapabilityDefs(): readonly ActionDef[] {
  return [
    { id: "primitive.createBoxFromCorners", run: runCreateBox },
    { id: "primitive.createBoxFrom3Points", run: runCreateBox },
    { id: "box.aabbFromDiagonalCorners", run: runCreateBox },
    { id: "entity.createAnchor", run: anchorAction },
    {
      id: "command.addPoint",
      run: (params) => {
        const ctx = (params.__context as Record<string, unknown>) ?? {};
        const field = String(params.field ?? "points");
        const key = params.key != null ? String(params.key) : null;
        const point = vec3Param(params, "point");
        const snap = interactionPointSnapFromEvent(params.__event as InteractionEvent | undefined);
        const cur = ctx[field];
        if (key) {
          const base = cur && typeof cur === "object" && !Array.isArray(cur) ? { ...(cur as Record<string, unknown>) } : {};
          base[key] = point;
          const patch: Record<string, unknown> = { [field]: base };
          if (field === "points" && (key === "from" || key === "to" || key === "center" || key === "start" || key === "end")) patch[key] = point;
          return { diff: EMPTY_MODEL_DIFF, patch: { set: patch } };
        }
        const arr = Array.isArray(cur) ? [...(cur as Vec3[])] : [];
        arr.push(point);
        const patch: Record<string, unknown> = { [field]: arr };
        if (field === "points") {
          const snaps = Array.isArray(ctx.pointSnaps) ? [...(ctx.pointSnaps as (InteractionPointSnap | null)[])] : [];
          snaps.push(snap);
          patch.pointSnaps = snaps;
        }
        return { diff: EMPTY_MODEL_DIFF, patch: { set: patch } };
      },
    },
    {
      id: "command.assignExtrusionDistance",
      run: (params) => {
        const bag = (params.__context as Record<string, unknown>) ?? {};
        const origin = vec3Param(bag, "origin", vec3Param(bag, "prevPoint", [0, 0, 0]));
        const cursor = vec3Param(bag, "cursor", origin);
        const direction = vec3Param(bag, "direction", [0, 0, 1]);
        const len = Math.hypot(direction[0], direction[1], direction[2]) || 1;
        const dir: Vec3 = [direction[0] / len, direction[1] / len, direction[2] / len];
        const delta = [cursor[0] - origin[0], cursor[1] - origin[1], cursor[2] - origin[2]] as Vec3;
        const distance = Math.abs(delta[0] * dir[0] + delta[1] * dir[1] + delta[2] * dir[2]);
        return { diff: EMPTY_MODEL_DIFF, patch: { set: { distance } }, data: distance };
      },
    },
    {
      id: "command.assignDirectionFromPoint",
      run: (params) => {
        const bag = (params.__context as Record<string, unknown>) ?? {};
        const event = params.__event as InteractionEvent | undefined;
        const origin = vec3Param(bag, "origin", vec3Param(bag, "prevPoint", [0, 0, 0]));
        const point = vec3Param(params, "point", (event as { readonly point?: Vec3 } | undefined)?.point ?? origin);
        const delta = [point[0] - origin[0], point[1] - origin[1], point[2] - origin[2]] as Vec3;
        const len = Math.hypot(delta[0], delta[1], delta[2]);
        const direction: Vec3 = len > 1e-9 ? [delta[0] / len, delta[1] / len, delta[2] / len] : [0, 0, 1];
        return { diff: EMPTY_MODEL_DIFF, patch: { set: { direction } }, data: direction };
      },
    },
    {
      id: "command.addSelection",
      run: (params) => {
        const field = String(params.field ?? "targets");
        const ctx = (params.__context as Record<string, unknown>) ?? {};
        const event = params.__event as InteractionEvent | undefined;
        const key = params.key != null ? String(params.key) : null;
        const incoming = parseSelectionTargetsFromUnknown(params.targets ?? []);
        const modifiers = (event?.modifiers ?? params.modifiers ?? {}) as InteractionEventModifiers;
        if (key) {
          const cur = ctx[field];
          const base = cur && typeof cur === "object" && !Array.isArray(cur) ? { ...(cur as Record<string, unknown>) } : {};
          const current = parseSelectionTargetsFromUnknown(base[key] ?? []);
          const merged = selectionTargetsWithMode(current, incoming, modifiers);
          base[key] = merged;
          return { diff: EMPTY_MODEL_DIFF, patch: { set: { [field]: base } }, data: { targets: merged } };
        }
        const current = parseSelectionTargetsFromUnknown(ctx[field] ?? params.current ?? []);
        const merged = selectionTargetsWithMode(current, incoming, modifiers);
        return { diff: EMPTY_MODEL_DIFF, patch: { set: { [field]: merged } }, data: { targets: merged } };
      },
    },
    {
      id: "command.undoPick",
      run: (params) => {
        const ctx = (params.__context as Record<string, unknown>) ?? {};
        const field = String(params.field ?? "points");
        const cur = ctx[field];
        const clearKeys = Array.isArray(params.clearKeys) ? (params.clearKeys as string[]) : [];
        const patch: Record<string, unknown> = {};
        if (cur && typeof cur === "object" && !Array.isArray(cur)) {
          const base = { ...(cur as Record<string, unknown>) };
          for (const key of clearKeys) delete base[key];
          patch[field] = base;
          for (const key of clearKeys) patch[key] = undefined;
        } else if (Array.isArray(cur)) {
          patch[field] = cur.slice(0, -1);
        }
        return { diff: EMPTY_MODEL_DIFF, patch: { set: patch } };
      },
    },
    {
      id: "command.selectionBboxCenter",
      run: (params, ctx) => {
        const field = String(params.field ?? "from");
        const bag = (params.__context as Record<string, unknown>) ?? {};
        const targets = parseSelectionTargetsFromUnknown(params.targets ?? bag.targets ?? []);
        const center = selectionTargetsCenter(ctx.model, targets, ctx.preview);
        if (!center) return { diff: EMPTY_MODEL_DIFF };
        return { diff: EMPTY_MODEL_DIFF, patch: { set: { [field]: center } }, data: center };
      },
    },
    {
      id: "command.finish",
      run: async (params, ctx) => {
        const commandId = String(params.commandId ?? "");
        const bag: Record<string, unknown> = { ...withResolvedInteractionPointsContext(ctx.model, (params.__context as Record<string, unknown>) ?? {}) };
        const points = bag.points;
        if (points && typeof points === "object" && !Array.isArray(points)) Object.assign(bag, points as Record<string, unknown>);
        for (const k of ["commandId", "resultKind", "__context", "__event", "pointSnaps"]) delete bag[k];
        if (!ctx.kernel.executeCommandDiff) return { diff: EMPTY_MODEL_DIFF, data: params.resultKind ?? null };
        const { diff } = await ctx.kernel.executeCommandDiff(commandId, { ...bag, model: ctx.model });
        return { diff: diff ?? EMPTY_MODEL_DIFF, data: params.resultKind ?? null };
      },
    },
    {
      id: "command.constrainMoveCursor",
      run: (params, ctx) => {
        const bag = (params.__context as Record<string, unknown>) ?? {};
        const points = bag.points && typeof bag.points === "object" && !Array.isArray(bag.points) ? (bag.points as Record<string, unknown>) : {};
        const from = vec3Param(params, "from", vec3Param(bag, "from", vec3Param(points, "from")));
        const to = vec3Param(params, "to", vec3Param(params, "point", from));
        const cursor = ctx.preview.constrainMovePoint(from, to, String(params.moveMode ?? bag.moveMode ?? "free"), vec3Param(params, "cplaneNormal", vec3Param(bag, "cplaneNormal", [0, 0, 1])));
        return { diff: EMPTY_MODEL_DIFF, patch: { set: { cursor } }, data: cursor };
      },
    },
    { id: "transform.move", run: (params, ctx) => ({ diff: transformDiff(params, ctx, false) }) },
    { id: "transform.copy", run: (params, ctx) => ({ diff: transformDiff(params, ctx, true) }) },
    {
      id: "transform.rotate",
      run: (params, ctx) => {
        const targets = parseSelectionTargetsFromUnknown(params.targets ?? params.selection ?? params.seedTargets);
        const map = ctx.preview.transformPointsForPreviewKind("rotate-preview", params);
        return { diff: vertexPositionsTransformDiff(ctx.model, targets, map) };
      },
    },
    {
      id: "transform.scale1d",
      run: (params, ctx) => {
        const targets = parseSelectionTargetsFromUnknown(params.targets ?? params.selection ?? params.seedTargets);
        const map = ctx.preview.transformPointsForPreviewKind("scale1d-preview", params);
        return { diff: vertexPositionsTransformDiff(ctx.model, targets, map) };
      },
    },
    {
      id: "transform.scale3d",
      run: (params, ctx) => {
        const targets = parseSelectionTargetsFromUnknown(params.targets ?? params.selection ?? params.seedTargets);
        const map = ctx.preview.transformPointsForPreviewKind("scale-preview", params);
        return { diff: vertexPositionsTransformDiff(ctx.model, targets, map) };
      },
    },
    {
      id: "measure.vertexDistance",
      run: async (params, ctx) => {
        const a = String(params.a ?? params.vertexA ?? params.from) as VertexRef;
        const b = String(params.b ?? params.vertexB ?? params.to) as VertexRef;
        const data = await ctx.kernel.vertexDistance(a, b, ctx.model);
        const edgeId = `measure-${a}-${b}` as EdgeRef;
        const wireId = `measure-wire-${a}-${b}` as WireRef;
        return {
          data,
          diff: {
            edges: { added: [{ id: edgeId, vertexIds: [a, b] }] },
            wires: { added: [{ id: wireId, edgeIds: [edgeId] }] },
          },
        };
      },
    },
    {
      id: "measure.faceArea",
      run: async (params, ctx) => {
        const faceId = String(params.faceId ?? params.face ?? "");
        const data = await ctx.kernel.faceArea(faceId as FaceRef, ctx.model);
        const face = ctx.model.faces[faceId];
        const position = face ? faceAnnotationCentroid(ctx.model, face) : null;
        const diff = position ? { anchors: { added: [{ id: `area-${faceId}` as AnchorRef, position, attachment: { kind: "face", id: faceId as FaceRef, u: 0, v: 0 } }] } } : EMPTY_MODEL_DIFF;
        return { data, diff };
      },
    },
    {
      id: "measure.solidVolume",
      run: async (params, ctx) => ({ data: await ctx.kernel.solidVolume(String(params.solidId ?? params.solid) as SolidRef) }),
    },
    {
      id: "feature.extrudeWireToSolid",
      run: async (params, ctx) =>
        ctx.kernel.extrudeWireDiff
          ? ctx.kernel.extrudeWireDiff({ wireId: String(params.wireId ?? params.wire ?? ""), distance: numericParam(params, "distance", 1), direction: vec3Param(params, "direction", [0, 0, 1]), model: ctx.model })
          : { diff: EMPTY_MODEL_DIFF },
    },
    {
      id: "feature.offsetFaces",
      run: async (params, ctx) =>
        ctx.kernel.offsetFacesDiff
          ? ctx.kernel.offsetFacesDiff({ faceIds: Array.isArray(params.faceIds) ? params.faceIds.map(String) : [String(params.faceId ?? "")], distance: numericParam(params, "distance", 1), model: ctx.model })
          : { diff: EMPTY_MODEL_DIFF },
    },
  ];
}

export async function executeActionCapability(
  actionId: string,
  params: Record<string, unknown>,
  args: Record<string, unknown>,
  ctx: {
    readonly kernel: SpatialKernel;
    readonly preview: SpatialPreviewKernel;
    readonly model: Model;
    readonly activeModelDefinitionId?: string | null;
  },
): Promise<unknown> {
  const def = modelDefinitionActionCapabilityDefs().find((d) => d.id === actionId);
  if (def?.run) return def.run(params, ctx);
  if (ctx.kernel.executeCommandDiff) {
    const result = await ctx.kernel.executeCommandDiff(actionId, { ...params, model: ctx.model });
    const typology = typeof params.typology === "string" ? params.typology : null;
    if (typology && !isEmptyModelDiff(result.diff)) ensureTypologyObjectFromCreateDiff(ctx.model, typology, result.diff);
    return result;
  }
  throw new Error(`Unknown action capability: ${actionId}`);
}

async function executeKernelFunction(
  functionName: string,
  actionId: string,
  params: Record<string, unknown>,
  callArgs: Record<string, unknown>,
  ctx: {
    readonly kernel: SpatialKernel;
    readonly preview: SpatialPreviewKernel;
    readonly model: Model;
    readonly activeModelDefinitionId?: string | null;
  },
): Promise<unknown> {
  const merged = { ...params, ...callArgs };
  if (functionName === "spatial.selection.apply") {
    return selectionCommandActionResult(executeSelectionApply(selectionApplyParamsFromRecord(merged), ctx));
  }
  if (functionName === "spatial.action.capability") {
    if (ctx.kernel.executeAction) return ctx.kernel.executeAction(actionId, merged, callArgs, ctx);
    return executeActionCapability(actionId, merged, callArgs, ctx);
  }
  throw new Error(`Unknown kernel function: ${functionName}`);
}

export class DeclarativeActionRuntime {
  constructor(private readonly spec: ActionSpec) {}

  async run(
    params: Record<string, unknown>,
    ctx: {
      readonly kernel: SpatialKernel;
      readonly preview: SpatialPreviewKernel;
      readonly model: Model;
      readonly activeModelDefinitionId?: string | null;
    },
  ): Promise<ActionResult> {
    const vars: Record<string, unknown> = {};
    const env: ExprEnv = {
      context: ((params.__context ?? {}) as Record<string, unknown>) ?? {},
      event: params.__event as Record<string, unknown> | undefined,
      params,
      vars,
      model: ctx.model,
      activeModelDefinitionId: ctx.activeModelDefinitionId,
      kernel: ctx.kernel,
      actionId: this.spec.id,
      metadata: ctx.model.metadata,
      preview: ctx.preview,
    };
    for (const v of this.spec.variables ?? []) vars[v.name] = await Promise.resolve(evalExpr(v.value, env));
    for (const step of this.spec.steps) {
      if (step.operation === "let") vars[step.name] = await Promise.resolve(evalExpr(step.value, env));
      else if (step.operation === "setContext") Object.assign(env.context, evalExprRecord(step.values, env));
      else if (step.operation === "deleteContext") for (const key of step.keys) delete env.context[key];
      else if (step.operation === "guard") {
        const ok = Boolean(await Promise.resolve(evalExpr(step.condition, env)));
        if (!ok) throw new Error(step.message ?? `Action guard failed: ${this.spec.id}`);
      } else if (step.operation === "kernel.call") {
        const result = await executeKernelFunction(step.function, this.spec.id, params, evalExprRecord(step.args, env), ctx);
        if (step.assignTo) vars[step.assignTo] = result;
      } else if (step.operation === "return") {
        const result = step.result ? await Promise.resolve(evalExpr(step.result, env)) : undefined;
        if (result && typeof result === "object" && !Array.isArray(result)) return result as ActionResult;
        return {
          diff: step.diff ? ((await Promise.resolve(evalExpr(step.diff, env))) as ModelDiff) : undefined,
          data: step.data ? await Promise.resolve(evalExpr(step.data, env)) : undefined,
          patch: step.patch ? ((await Promise.resolve(evalExpr(step.patch, env))) as ActionContextPatch) : undefined,
        };
      }
    }
    return {};
  }
}

/** 🧭️ Registers one action definition; returns a new registry (immutable update). */
export function registerActionDef(registry: ActionRegistry, def: ActionDef): ActionRegistry {
  const next = new Map(registry);
  next.set(def.id, def);
  return next;
}

/** 🧭️ Lists registered action definitions in stable id order. */
export function listActionDefs(registry: ActionRegistry): readonly ActionDef[] {
  return [...registry.values()].sort((a, b) => a.id.localeCompare(b.id));
}

/** 🧩️ Runs a registered action (`selection.apply`, geometry actions, …) without an interaction session. */
export async function runRegisteredAction(
  registry: ActionRegistry,
  id: string,
  params: Record<string, unknown>,
  ctx: {
    readonly kernel: SpatialKernel;
    readonly preview: SpatialPreviewKernel;
    readonly model: Model;
    readonly activeModelDefinitionId?: string | null;
  },
): Promise<ActionResult> {
  assertActionAvailableInModelDefinition(id, ctx.activeModelDefinitionId);
  const def = registry.get(id) ?? null;
  if (def?.spec) return new DeclarativeActionRuntime(def.spec).run(params, ctx);
  if (def?.run) return Promise.resolve(def.run(params, ctx));
  const kernelResult = await executeActionCapability(id, params, {}, ctx);
  if (kernelResult && typeof kernelResult === "object" && "diff" in (kernelResult as object)) return kernelResult as ActionResult;
  if (kernelResult && typeof kernelResult === "object" && "patch" in (kernelResult as object)) return kernelResult as ActionResult;
  if (kernelResult !== undefined) return { data: kernelResult };
  throw new Error(`Unknown action: ${id}`);
}

/** 🧭️ Shipped model-definition actions plus capability-backed fallbacks. */
export function modelDefinitionActionRegistry(): ActionRegistry {
  const map = new Map<string, ActionDef>();
  for (const spec of shippedActionCatalog()) map.set(spec.id, { id: spec.id, label: spec.label, spec });
  for (const def of modelDefinitionActionCapabilityDefs()) {
    if (map.has(def.id)) continue;
    map.set(def.id, {
      id: def.id,
      label: def.label ?? def.id,
      spec: capabilityActionSpecJson(def.id, def.label ?? def.id) as ActionSpec,
    });
  }
  return map;
}

// #endregion 🧮️ActionRegistry
/** 🪪️ Runs `selection.apply` headless via `ActionRegistry` (no interaction session). */
export async function runSelectionApply(
  params: SelectionApplyParams,
  ctx: {
    readonly kernel: SpatialKernel;
    readonly preview: SpatialPreviewKernel;
    readonly model: Model;
    readonly activeModelDefinitionId?: string | null;
    readonly actions?: ActionRegistry;
  },
): Promise<readonly SelectionTarget[]> {
  const actions = ctx.actions ?? modelDefinitionActionRegistry();
  const result = await runRegisteredAction(
    actions,
    "selection.apply",
    {
      operation: params.operation,
      seedTargets: params.seedTargets ?? [],
      ...(params.kinds ? { kinds: params.kinds } : {}),
      __context: {},
      __event: { kind: "commit" },
    },
    ctx,
  );
  return selectionTargetsFromActionResult(result);
}

// #region 🔍️ConstructQuery
/** 🔍️ One named column in a `construct` result row. */
export type ConstructQueryRow = Readonly<Record<string, unknown>>;

/** 🔍️ `construct` runner output (`rows` for MATCH; CALL modeling yields `diff` geometry when present). */
export interface ConstructQueryResult {
  readonly rows: readonly ConstructQueryRow[];
  readonly data?: unknown;
  readonly diff?: ModelDiff;
}

/** 🔍️ Host wiring for `InteractionRuntime.query` (`@semio-tech/cad-js/query` supplies the default runner). */
export interface ConstructQueryContext {
  readonly model: Model;
  readonly kernel: SpatialKernel;
  readonly actions: ActionRegistry;
  readonly activeModelDefinitionId?: string | null;
  /** 🪪️ Default `seedTargets` for `CALL selection.*` when the call omits `seedTargets`. */
  readonly selectionTargets?: readonly SelectionTarget[];
}

/** 🔍️ Async bridge so core never imports `@semio-tech/cad-js/query`. */
export type ConstructRunner = (text: string, ctx: ConstructQueryContext) => Promise<ConstructQueryResult>;
// #endregion 🔍️ConstructQuery
// #endregion 📦️🎬️actions

// #region 🧲️Header
/** @emoji 🧭️ `@semio-tech/cad-js` — CAD domain module facet. See `cad/AGENTS.md`. */
import { ephemeralBox, ephemeralMap, ephemeralWeakMap } from "@semio-tech/framework";
import type { ArcPlaneFrame, EdgeCurve, EdgeGroup, EdgeInfo, FaceGroup, FaceInfo, MeshTransfer, Vec3 } from "@semio-tech/kernel-3d-js";
import { emptyMeshTransfer, kernelGeometry, solidRef } from "@semio-tech/kernel-3d-js";
// #endregion 🧲️Header


import { EdgeRef, EffectSpec, Expr, ExprEnv, FaceRef, InteractionEngagementControlKind, InteractionEngagementEntryControl, InteractionEvent, InteractionLengthEntrySpec, InteractionOutputBinding, InteractionScalarEntrySpec, InteractionSpec, Model, ModelEntityKind, PathSegment, ResolvedInteractionEngagementControl, SelectionEvent, SelectionTarget, SolidRef, VertexRef, WireRef, assertActionAvailableInModelDefinition, clearPathTarget, defaultModelDefinitionId, evalExpr, evalGuard, initialContextForSpec, listModelObjectsForModelDefinition, modelDefinitionSelectionEntityKinds, readPathTarget, writePathSegments, writePathTarget } from "../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts";
import { capabilityActionSpecJson, ensureTypologyObjectFromCreateDiff } from "../🧬️typology/🟦️component.ts";
import { EMPTY_MODEL_DIFF, EdgeRecordDiff, KernelQueryContext, ModelDiff, SpatialKernel, SpatialPreviewKernel, VertexRecordDiff, isEmptyModelDiff } from "../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️component.ts";



// #region 📦️🎬️actions
// #region 🧮️ActionRegistry
/** @emoji 🧩️ Serializable context patch applied after pure box geometry actions (`set` keys merged; `del` removes top-level context keys). */
export interface ActionContextPatch {
  readonly set?: Record<string, unknown>;
  readonly del?: readonly string[];
}

/** @emoji 🧩️ Pure action output: model `diff` is the committed geometry; optional `data` is auxiliary; `patch` updates session context only. */
export interface ActionResult<TData = unknown> {
  readonly diff?: ModelDiff;
  readonly data?: TData;
  readonly patch?: ActionContextPatch;
}

export type ActionFn<TParams = Record<string, unknown>, TData = unknown> = (
  params: TParams,
  ctx: {
    readonly kernel: SpatialKernel;
    readonly preview: SpatialPreviewKernel;
    readonly model: Model;
    readonly activeModelDefinitionId?: string | null;
  },
) => Promise<ActionResult<TData>> | ActionResult<TData>;

export interface ActionParameterSpec {
  readonly kind: "string" | "number" | "boolean" | "vec3" | "stringArray" | "unknown";
}

export type ActionStepSpec =
  | { readonly operation: "let"; readonly name: string; readonly value: Expr }
  | { readonly operation: "setContext"; readonly values: Record<string, Expr> }
  | { readonly operation: "deleteContext"; readonly keys: readonly string[] }
  | { readonly operation: "kernel.call"; readonly function: string; readonly args?: Record<string, Expr>; readonly assignTo?: string }
  | { readonly operation: "guard"; readonly condition: Expr; readonly message?: string }
  | { readonly operation: "return"; readonly diff?: Expr; readonly data?: Expr; readonly patch?: Expr; readonly result?: Expr };

export interface ActionSpec {
  readonly schema: "spatial.action";
  readonly id: string;
  readonly version: string;
  readonly label?: string;
  readonly args?: Record<string, unknown>;
  readonly parameters?: Record<string, ActionParameterSpec>;
  readonly variables?: readonly { readonly name: string; readonly value: Expr }[];
  readonly steps: readonly ActionStepSpec[];
}

/** @emoji 🧩️ Registerable spatial action spec (`id` is stable registry key). */
export interface ActionDef<TParams = Record<string, unknown>, TData = unknown> {
  readonly id: string;
  readonly label?: string;
  readonly spec?: ActionSpec;
  readonly run?: ActionFn<TParams, TData>;
}

function applyActionPatchToContext(ctx: Record<string, unknown>, patch: ActionContextPatch | undefined): void {
  if (!patch) return;
  if (patch.set) Object.assign(ctx, patch.set);
  if (patch.del) for (const k of patch.del) delete ctx[k];
}

function hasExecutableActionField(raw: Record<string, unknown>): boolean {
  for (const key of ["run", "code", "function", "handler", "script"]) {
    if (key in raw) return true;
  }
  return false;
}

function isActionStepSpec(raw: unknown): raw is ActionStepSpec {
  if (!raw || typeof raw !== "object") return false;
  const r = raw as Record<string, unknown>;
  if (typeof r.operation !== "string") return false;
  if (r.operation === "let") return typeof r.name === "string" && Boolean(r.value);
  if (r.operation === "setContext") return Boolean(r.values) && typeof r.values === "object" && !Array.isArray(r.values);
  if (r.operation === "deleteContext") return Array.isArray(r.keys) && r.keys.every((k) => typeof k === "string");
  if (r.operation === "kernel.call") return typeof r.function === "string";
  if (r.operation === "guard") return Boolean(r.condition);
  if (r.operation === "return") return true;
  return false;
}

/** @emoji 🧾️ Parses a data-only `spatial.action/v1` document. */
export function parseActionSpec(raw: unknown): ActionSpec | null {
  if (!raw || typeof raw !== "object" || Array.isArray(raw)) return null;
  const r = structuredClone(raw) as Record<string, unknown>;
  if (hasExecutableActionField(r)) return null;
  if (r.schema !== "spatial.action") return null;
  if (typeof r.id !== "string" || r.id.length === 0 || typeof r.version !== "string") return null;
  if (r.args !== undefined && (!r.args || typeof r.args !== "object" || Array.isArray(r.args))) return null;
  if (!Array.isArray(r.steps) || r.steps.length === 0 || !r.steps.every(isActionStepSpec)) return null;
  const variables = r.variables;
  if (variables !== undefined) {
    if (!Array.isArray(variables)) return null;
    for (const v of variables) {
      if (!v || typeof v !== "object") return null;
      const row = v as Record<string, unknown>;
      if (typeof row.name !== "string" || !row.value) return null;
    }
  }
  return r as unknown as ActionSpec;
}

/** @emoji 📚️ Lists data-only model-definition action assets. */
export function listModelDefinitionActionSpecs(): readonly ActionSpec[] {
  return modelDefinitionActionCatalog()
    .map((raw) => parseActionSpec(raw))
    .filter((spec): spec is ActionSpec => spec !== null);
}

function shippedActionCatalog(): readonly ActionSpec[] {
  return listModelDefinitionActionSpecs();
}

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
        const point = vec3Param(params, "point", event?.point ?? origin);
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
        const current = parseSelectionTargetsFromUnknown(ctx[field] ?? params.current ?? []);
        const incoming = parseSelectionTargetsFromUnknown(params.targets ?? []);
        const modifiers = (event?.modifiers ?? params.modifiers ?? {}) as InteractionEvent["modifiers"];
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

/** @emoji 🧭️ Immutable map of registered data-only `ActionSpec` entries (model-definitions + host overrides). */
export type ActionRegistry = ReadonlyMap<string, ActionDef>;

/** @emoji 🧭️ Registers one action definition; returns a new registry (immutable update). */
export function registerActionDef(registry: ActionRegistry, def: ActionDef): ActionRegistry {
  const next = new Map(registry);
  next.set(def.id, def);
  return next;
}

/** @emoji 🧭️ Lists registered action definitions in stable id order. */
export function listActionDefs(registry: ActionRegistry): readonly ActionDef[] {
  return [...registry.values()].sort((a, b) => a.id.localeCompare(b.id));
}

/** @emoji 🧩️ Runs a registered action (`selection.apply`, geometry actions, …) without an interaction session. */
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

/** @emoji 🧭️ Shipped model-definition actions plus capability-backed fallbacks. */
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

/** @emoji 📍️ Centroid of a face boundary for measure/annotation anchors. */
function faceAnnotationCentroid(model: Model, face: FaceRecord): Vec3 | null {
  const pts: Vec3[] = [];
  for (const wid of face.wireIds) {
    for (const eid of model.wires[wid]?.edgeIds ?? []) {
      for (const vid of model.edges[eid]?.vertexIds ?? []) {
        const p = model.vertices[vid]?.position;
        if (p) pts.push(p);
      }
    }
  }
  if (!pts.length) return null;
  let x = 0;
  let y = 0;
  let z = 0;
  for (const p of pts) {
    x += p[0];
    y += p[1];
    z += p[2];
  }
  const n = pts.length;
  return [x / n, y / n, z / n];
}

/** @emoji 📏️ Measure and selection commands do not enter document undo history. */
export function interactionRecordsDocumentHistory(interactionId: string): boolean {
  return !interactionId.startsWith("measure.") && !interactionId.startsWith("selection.");
}

/** @emoji 🎯️ Collects vertex ids reachable from transform/edit selection targets. */
export function collectTargetVertices(model: Model, targets: readonly SelectionTarget[]): Set<string> {
  const out = new Set<string>();
  const walk = (kind: ModelEntityKind, id: string) => {
    if (kind === "anchor") {
      const anchor = model.anchors[id];
      if (anchor?.attachment.kind === "vertex") out.add(anchor.attachment.id);
    } else if (kind === "vertex") {
      if (model.vertices[id]) out.add(id);
    } else if (kind === "edge") {
      const e = model.edges[id];
      if (e) for (const v of e.vertexIds) walk("vertex", v);
    } else if (kind === "wire") {
      const w = model.wires[id];
      if (w) for (const e of w.edgeIds) walk("edge", e);
    } else if (kind === "face") {
      const f = model.faces[id];
      if (f) for (const w of f.wireIds) walk("wire", w);
    } else if (kind === "shell") {
      const s = model.shells[id];
      if (s) for (const f of s.faceIds) walk("face", f);
    } else if (kind === "solid" || kind === "geometry") {
      const c = model.solids[id];
      if (c) for (const s of c.shellIds) walk("shell", s);
    }
  };
  for (const t of targets) walk(t.kind, t.id);
  return out;
}

/** @emoji 🎯️ Collects edge ids when topology (edge/wire/face/…) is selected; excludes vertex-only picks. */
export function collectTargetEdges(model: Model, targets: readonly SelectionTarget[]): Set<string> {
  const out = new Set<string>();
  const walk = (kind: ModelEntityKind, id: string) => {
    if (kind === "edge") {
      if (model.edges[id]) out.add(id);
    } else if (kind === "wire") {
      const w = model.wires[id];
      if (w) for (const e of w.edgeIds) walk("edge", e);
    } else if (kind === "face") {
      const f = model.faces[id];
      if (f) for (const w of f.wireIds) walk("wire", w);
    } else if (kind === "shell") {
      const s = model.shells[id];
      if (s) for (const f of s.faceIds) walk("face", f);
    } else if (kind === "solid" || kind === "geometry") {
      const c = model.solids[id];
      if (c) for (const s of c.shellIds) walk("shell", s);
    }
  };
  for (const t of targets) walk(t.kind, t.id);
  return out;
}

/** @emoji 📦️ Center of the axis-aligned bounds of all vertices in `targets`. */
export function selectionTargetsCenter(model: Model, targets: readonly SelectionTarget[], preview: SpatialPreviewKernel): Vec3 | null {
  const pts: Vec3[] = [];
  for (const vid of collectTargetVertices(model, targets)) {
    const v = model.vertices[vid];
    if (v) pts.push(v.position);
  }
  for (const eid of collectTargetEdges(model, targets)) {
    const curve = model.edges[eid]?.curve;
    if (curve?.kind === "nurbs") pts.push(...curve.poles);
  }
  const box = preview.aabbFromPoints(pts);
  if (!box) return null;
  return [(box.min[0] + box.max[0]) / 2, (box.min[1] + box.max[1]) / 2, (box.min[2] + box.max[2]) / 2];
}

/** @emoji 🎛️ CAD play gumball visibility groups (same shape as ui UnifiedGumball config). */
export interface CadGumballConfig {
  readonly moveAxes?: boolean;
  readonly movePlanes?: boolean;
  readonly rotate?: boolean;
  readonly scaleAxes?: boolean;
  readonly scalePlanes?: boolean;
  readonly scaleUniform?: boolean;
  /** When set, only the planar subset for this drafting plane is shown (e.g. Top → `xy`). */
  readonly plane?: "xy" | "yz" | "xz";
  readonly translationSnap?: number;
  readonly rotationSnap?: number;
  readonly scaleSnap?: number;
  readonly size?: number;
}

/** @emoji 🎛️ Toggle keys for CAD play gumball window measures. */
export type CadGumballGroupKey = keyof Pick<CadGumballConfig, "moveAxes" | "movePlanes" | "rotate" | "scaleAxes" | "scalePlanes" | "scaleUniform">;

/** @emoji 🎛️ Ordered gumball group toggles for CAD play window measures. */
export const CAD_GUMBALL_GROUPS: readonly { readonly key: CadGumballGroupKey; readonly label: string }[] = [
  { key: "moveAxes", label: "Move Axes" },
  { key: "movePlanes", label: "Move Planes" },
  { key: "rotate", label: "Rotate" },
  { key: "scaleAxes", label: "Scale Axes" },
  { key: "scalePlanes", label: "Scale Planes" },
  { key: "scaleUniform", label: "Scale Uniform" },
];

/** @emoji 🎛️ Default CAD play gumball state (hidden until a group is enabled). */
export const CAD_GUMBALL_HIDDEN: CadGumballConfig = {
  moveAxes: false,
  movePlanes: false,
  rotate: false,
  scaleAxes: false,
  scalePlanes: false,
  scaleUniform: false,
};

/** @emoji 🎛️ True when at least one gumball handle group is enabled. */
export function cadGumballConfigVisible(config: CadGumballConfig | null | undefined): boolean {
  if (!config) return false;
  return config.moveAxes !== false || config.movePlanes !== false || config.rotate !== false || config.scaleAxes !== false || config.scalePlanes !== false || config.scaleUniform !== false;
}

/** @emoji ✋️ True when `targets` resolve to at least one model vertex. */
export function selectionTargetsHaveTransformableVertices(model: Model, targets: readonly SelectionTarget[]): boolean {
  return collectTargetVertices(model, targets).size > 0;
}

function modelDiffTransformNurbsPolesOnEdges(model: Model, edgeIds: Iterable<string>, mapPoint: (point: Vec3) => Vec3): ModelDiff {
  const edgeMods: EdgeRecordDiff[] = [];
  for (const id of edgeIds) {
    const edge = model.edges[id];
    const curve = edge?.curve;
    if (curve?.kind !== "nurbs" || curve.poles.length < 2) continue;
    const poles = curve.poles.map((pole) => mapPoint(pole));
    if (poles.every((pole, index) => vec3Eq(pole, curve.poles[index]!))) continue;
    edgeMods.push({ id: edge.id, curve: { ...curve, poles } });
  }
  return edgeMods.length ? { edges: { modified: edgeMods } } : EMPTY_MODEL_DIFF;
}

/** @emoji 🎛️ Applies `mapPoint` to vertices and nurbs poles on topology-selected edges. */
export function selectionTargetsPointTransformDiff(model: Model, targets: readonly SelectionTarget[], mapPoint: (point: Vec3) => Vec3): ModelDiff {
  const vertexIds = collectTargetVertices(model, targets);
  const modified: VertexRecordDiff[] = [];
  for (const vid of vertexIds) {
    const v = model.vertices[vid];
    if (!v) continue;
    const next = mapPoint(v.position);
    if (next[0] === v.position[0] && next[1] === v.position[1] && next[2] === v.position[2]) continue;
    modified.push({ id: v.id, position: next });
  }
  const nurbsDiff = modelDiffTransformNurbsPolesOnEdges(model, collectTargetEdges(model, targets), mapPoint);
  if (!modified.length) return nurbsDiff;
  if (isEmptyModelDiff(nurbsDiff)) return { vertices: { modified } };
  return { vertices: { modified }, edges: nurbsDiff.edges };
}

function vertexPositionsTransformDiff(model: Model, targets: readonly SelectionTarget[], mapPoint: (point: Vec3) => Vec3): ModelDiff {
  return selectionTargetsPointTransformDiff(model, targets, mapPoint);
}

// #region 📍️InteractionPointBinding
/** @emoji 📍️ Optional geometry snap stored beside a committed interaction point. */
export type InteractionPointSnap = { readonly kind: string; readonly id: string };

/** @emoji 📍️ Reads parallel `pointSnaps` rows aligned with `context.points`. */
export function readInteractionPointSnaps(context: Record<string, unknown>): readonly (InteractionPointSnap | null)[] {
  const raw = context.pointSnaps;
  if (!Array.isArray(raw)) return [];
  return raw.map((row) => {
    if (!row || typeof row !== "object") return null;
    const kind = (row as { kind?: unknown }).kind;
    const id = (row as { id?: unknown }).id;
    return typeof kind === "string" && typeof id === "string" ? { kind, id } : null;
  });
}

function interactionPointSnapFromEvent(event: InteractionEvent | undefined): InteractionPointSnap | null {
  if (!event || typeof event !== "object") return null;
  const snap = (event as { snap?: { kind?: unknown; id?: unknown } }).snap;
  if (!snap || typeof snap.kind !== "string" || typeof snap.id !== "string") return null;
  return { kind: snap.kind, id: snap.id };
}

function vec3Eq(a: Vec3, b: Vec3): boolean {
  return a[0] === b[0] && a[1] === b[1] && a[2] === b[2];
}

/** @emoji 📍️ Replaces interaction points bound to moved vertices with live model positions. */
export function resolveLiveInteractionPoints(model: Model, points: readonly Vec3[], snaps: readonly (InteractionPointSnap | null)[]): readonly Vec3[] {
  if (!snaps.length) return points;
  return points.map((point, index) => {
    const snap = snaps[index];
    if (!snap || snap.kind !== "vertex") return point;
    const live = model.vertices[snap.id as VertexRef]?.position;
    return live ?? point;
  });
}

/** @emoji 📍️ Shallow context copy with `points` resolved from bound vertex snaps. */
export function withResolvedInteractionPointsContext(model: Model, context: Record<string, unknown>): Record<string, unknown> {
  const points = context.points;
  if (!Array.isArray(points)) return context;
  const snaps = readInteractionPointSnaps(context);
  if (!snaps.some((row) => row?.kind === "vertex")) return context;
  const resolved = resolveLiveInteractionPoints(model, points as Vec3[], snaps);
  if (resolved.every((point, index) => vec3Eq(point, points[index] as Vec3))) return context;
  return { ...context, points: [...resolved] };
}

function modelDiffSyncNurbsThroughEdgesForMovedVertices(model: Model, movedVertexIds: readonly VertexRef[]): ModelDiff {
  const moved = new Set(movedVertexIds.map(String));
  const edgeMods: EdgeRecordDiff[] = [];
  for (const edge of Object.values(model.edges)) {
    const curve = edge.curve;
    if (curve?.kind !== "nurbs" || !curve.through || curve.poles.length < 2) continue;
    const startId = String(edge.vertexIds[0] ?? "");
    const endId = String(edge.vertexIds[1] ?? edge.vertexIds[0] ?? "");
    let poles: Vec3[] | null = null;
    if (startId && moved.has(startId)) {
      const position = model.vertices[startId as VertexRef]?.position;
      if (position) {
        poles = [...curve.poles];
        poles[0] = [position[0], position[1], position[2]];
      }
    }
    if (endId && moved.has(endId)) {
      const position = model.vertices[endId as VertexRef]?.position;
      if (position) {
        poles = poles ? [...poles] : [...curve.poles];
        poles[poles.length - 1] = [position[0], position[1], position[2]];
      }
    }
    if (!poles) continue;
    if (poles.every((point, index) => vec3Eq(point, curve.poles[index]!))) continue;
    edgeMods.push({ id: edge.id, curve: { ...curve, poles } });
  }
  return edgeMods.length ? { edges: { modified: edgeMods } } : EMPTY_MODEL_DIFF;
}
// #endregion 📍️InteractionPointBinding

function selectionTargetKey(target: SelectionTarget): string {
  return `${target.kind}:${target.id}`;
}

function selectionTargetsWithMode(current: readonly SelectionTarget[], next: readonly SelectionTarget[], modifiers: InteractionEvent["modifiers"] = {}): SelectionTarget[] {
  const dedupedNext: SelectionTarget[] = [];
  const nextKeys = new Set<string>();
  for (const target of next) {
    const key = selectionTargetKey(target);
    if (nextKeys.has(key)) continue;
    nextKeys.add(key);
    dedupedNext.push(target);
  }
  if (modifiers.shift && modifiers.ctrl) {
    const currentKeys = new Set(current.map(selectionTargetKey));
    const kept = current.filter((target) => !nextKeys.has(selectionTargetKey(target)));
    const added = dedupedNext.filter((target) => !currentKeys.has(selectionTargetKey(target)));
    return [...kept, ...added];
  }
  if (modifiers.shift) {
    const merged = [...current];
    const seen = new Set(current.map(selectionTargetKey));
    for (const target of dedupedNext) {
      const key = selectionTargetKey(target);
      if (seen.has(key)) continue;
      seen.add(key);
      merged.push(target);
    }
    return merged;
  }
  if (modifiers.ctrl) {
    return current.filter((target) => !nextKeys.has(selectionTargetKey(target)));
  }
  return dedupedNext;
}

/** @emoji 🪪️ Kernel geometry + extension view `object` kinds used by selection commands. */
export const ALL_MODEL_SELECTION_KINDS: readonly ModelEntityKind[] = ["anchor", "vertex", "edge", "wire", "face", "solid", "object", "geometry", "attribute"];

const MODEL_SELECTION_KIND_ORDER = new Map<ModelEntityKind, number>(ALL_MODEL_SELECTION_KINDS.map((kind, index) => [kind, index]));

/** @emoji 🪪️ model-definition selection command operation id (`selection.apply` param). */
export type SelectionApplyOperation = "selectAll" | "deselectAll" | "invert" | "selectKinds";

/** @emoji 🪪️ Headless `selection.apply` / interaction commit input. */
export interface SelectionApplyParams {
  readonly operation: SelectionApplyOperation;
  readonly seedTargets?: readonly SelectionTarget[];
  readonly kinds?: readonly ModelEntityKind[];
}

/** @emoji 🪪️ model-definition selection command interaction row (`selection.*` registry). */
export type SelectionOperationInteractionDef = {
  readonly id: string;
  readonly label: string;
  readonly key: string;
  readonly operation: SelectionApplyOperation;
  readonly kinds?: readonly ModelEntityKind[];
};

function toSelectionTarget(kind: ModelEntityKind, id: string): SelectionTarget {
  return { kind, id, editable: kind !== "object" };
}

/** @emoji 🪪️ Parses `context.targets` or action patch targets into validated `SelectionTarget` rows. */
export function selectionTargetsFromContext(ctx: Record<string, unknown>): readonly SelectionTarget[] {
  return parseSelectionTargetsFromUnknown(ctx.targets);
}

/** @emoji 🪪️ Reads `targets` from an `selection.apply` action result patch. */
export function selectionTargetsFromActionResult(result: ActionResult): readonly SelectionTarget[] {
  return parseSelectionTargetsFromUnknown(result.patch?.set?.targets);
}

function parseSelectionTargetsFromUnknown(raw: unknown): SelectionTarget[] {
  if (!Array.isArray(raw)) return [];
  const out: SelectionTarget[] = [];
  for (const item of raw) {
    if (!item || typeof item !== "object") continue;
    const kind = (item as { kind?: unknown }).kind;
    const id = (item as { id?: unknown }).id;
    if (typeof kind !== "string" || !MODEL_ENTITY_KINDS.has(kind)) continue;
    if (typeof id !== "string" || id.length === 0) continue;
    const editable = (item as { editable?: unknown }).editable;
    out.push({
      kind: kind as ModelEntityKind,
      id,
      editable: typeof editable === "boolean" ? editable : kind !== "object",
    });
  }
  return out;
}

function parseModelEntityKinds(raw: unknown): ModelEntityKind[] {
  if (!Array.isArray(raw)) return [];
  const out: ModelEntityKind[] = [];
  for (const item of raw) {
    if (typeof item !== "string" || !MODEL_ENTITY_KINDS.has(item)) continue;
    out.push(item as ModelEntityKind);
  }
  return out;
}

function sortSelectionTargets(targets: readonly SelectionTarget[]): SelectionTarget[] {
  return [...targets].sort((a, b) => {
    const ka = MODEL_SELECTION_KIND_ORDER.get(a.kind) ?? 999;
    const kb = MODEL_SELECTION_KIND_ORDER.get(b.kind) ?? 999;
    if (ka !== kb) return ka - kb;
    return a.id.localeCompare(b.id);
  });
}

/** @emoji 🎯️ Primary selection row for attribute editing (primitive first, then typology object). */
export function primaryAttributeSelectionTarget(selection: readonly SelectionTarget[]): SelectionTarget | null {
  if (!selection.length) return null;
  for (const row of selection) {
    if (row.kind !== "object") return row;
  }
  return selection.find((row) => row.kind === "object") ?? selection[0] ?? null;
}

/** @emoji 🪪️ Collects stable `SelectionTarget` rows for kernel `kinds` scoped to the active model definition. */
export function collectGeometrySelectionTargets(model: Model, kinds: readonly ModelEntityKind[], activeModelDefinitionId?: string | null): SelectionTarget[] {
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  const scopeKinds = kinds.length > 0 ? kinds : modelDefinitionSelectionEntityKinds(mdId);
  const allowed = new Set(scopeKinds);
  const out: SelectionTarget[] = [];
  const seen = new Set<string>();
  const push = (kind: ModelEntityKind, id: string, editable = true) => {
    const key = selectionTargetKey({ kind, id, editable });
    if (seen.has(key)) return;
    seen.add(key);
    out.push({ kind, id, editable });
  };
  for (const kind of allowed) {
    switch (kind) {
      case "anchor":
        for (const id of Object.keys(model.anchors)) push(kind, id);
        break;
      case "vertex":
        for (const id of Object.keys(model.vertices)) push(kind, id);
        break;
      case "edge":
        for (const id of Object.keys(model.edges)) push(kind, id);
        break;
      case "wire":
        for (const id of Object.keys(model.wires)) push(kind, id);
        break;
      case "face":
        for (const id of Object.keys(model.faces)) push(kind, id);
        break;
      case "shell":
        for (const id of Object.keys(model.shells)) push(kind, id);
        break;
      case "solid":
        for (const id of Object.keys(model.solids)) push(kind, id);
        break;
      case "geometry":
        for (const id of Object.keys(model.solids)) push(kind, id);
        break;
      case "attribute":
        break;
      case "object":
        for (const row of listModelObjectsForModelDefinition(model, mdId)) push(kind, String(row.id), false);
        break;
    }
  }
  return sortSelectionTargets(out);
}

/** @emoji 🪪️ Applies `selectAll` / `deselectAll` / `invert` / `selectKinds` to `current` against `model`. */
export function applySelectionOperation(operation: SelectionApplyOperation, current: readonly SelectionTarget[], model: Model, kinds: readonly ModelEntityKind[], activeModelDefinitionId?: string | null): SelectionTarget[] {
  if (operation === "deselectAll") return [];
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  const scopeKinds = kinds.length > 0 ? kinds : modelDefinitionSelectionEntityKinds(mdId);
  const universe = collectGeometrySelectionTargets(model, scopeKinds, mdId);
  if (operation === "selectAll" || operation === "selectKinds") return universe;
  const cur = new Set(current.map(selectionTargetKey));
  return universe.filter((target) => !cur.has(selectionTargetKey(target)));
}

/** @emoji 🪪️ Shared selection command core used by `selection.apply` and headless callers. */
export function executeSelectionApply(params: SelectionApplyParams, ctx: { readonly model: Model; readonly activeModelDefinitionId?: string | null }): SelectionTarget[] {
  const seed = params.seedTargets ?? [];
  const kinds = params.operation === "selectKinds" ? [...(params.kinds ?? [])] : params.operation === "invert" || params.operation === "selectAll" ? [...ALL_MODEL_SELECTION_KINDS] : [];
  return applySelectionOperation(params.operation, seed, ctx.model, kinds, ctx.activeModelDefinitionId ?? null);
}

/** @emoji 🪪️ Runs `selection.apply` headless via `ActionRegistry` (no interaction session). */
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

/** @emoji 🪪️ Standard `selection.apply` / `selection.*` construct `CALL` result (`YIELD targets` / `data.targets`). */
export function selectionCommandActionResult(targets: readonly SelectionTarget[]): ActionResult {
  return { patch: { set: { targets: [...targets] } }, diff: EMPTY_MODEL_DIFF, data: { targets } };
}

/** @emoji 🪪️ True when `actionId` is a `selection.*` construct or action (`selection.apply`, `selection.selectAll`, …). */
export function isSelectionConstructActionId(actionId: string): boolean {
  return actionId.startsWith("selection.");
}

function selectionApplyParamsFromRecord(params: Record<string, unknown>): SelectionApplyParams {
  const bag = (params.__context ?? {}) as Record<string, unknown>;
  const operation = String(params.operation ?? bag.operation ?? "selectAll") as SelectionApplyOperation;
  const seed = parseSelectionTargetsFromUnknown(params.seedTargets ?? bag.seedTargets);
  const kinds = parseModelEntityKinds(params.kinds ?? bag.kinds);
  return { operation, seedTargets: seed, ...(operation === "selectKinds" ? { kinds } : {}) };
}

// #region 🔍️ConstructQuery
/** @emoji 🔍️ One named column in a `construct` result row. */
export type ConstructQueryRow = Readonly<Record<string, unknown>>;

/** @emoji 🔍️ `construct` runner output (`rows` for MATCH; CALL modeling yields `diff` geometry when present). */
export interface ConstructQueryResult {
  readonly rows: readonly ConstructQueryRow[];
  readonly data?: unknown;
  readonly diff?: ModelDiff;
}

/** @emoji 🔍️ Host wiring for `InteractionRuntime.query` (`@semio-tech/cad-js/query` supplies the default runner). */
export interface ConstructQueryContext {
  readonly model: Model;
  readonly kernel: SpatialKernel;
  readonly actions: ActionRegistry;
  readonly activeModelDefinitionId?: string | null;
  /** @emoji 🪪️ Default `seedTargets` for `CALL selection.*` when the call omits `seedTargets`. */
  readonly selectionTargets?: readonly SelectionTarget[];
}

/** @emoji 🔍️ Async bridge so core never imports `@semio-tech/cad-js/query`. */
export type ConstructRunner = (text: string, ctx: ConstructQueryContext) => Promise<ConstructQueryResult>;
// #endregion 🔍️ConstructQuery

// #region 🎬️Statechart
/** @emoji 📞️ Pauses host statechart until nested interaction completes or aborts. */
export interface InteractionChildCallSpec {
  readonly interactionId: string;
  readonly inputs?: Record<string, Expr>;
  readonly outputs?: readonly InteractionOutputBinding[] | Record<string, unknown>;
  readonly resumeTarget: string;
  readonly rollback: { readonly state: string; readonly context: Record<string, unknown> };
}

/** @emoji 🎭️ Result of `StateEngine.send` / `applyTransition` (`transient` skips interaction-local undo). */
export interface StateEngineSendResult {
  readonly ok: boolean;
  readonly transient?: boolean;
  readonly childCall?: InteractionChildCallSpec;
}

/** @emoji 🎭️ `applyTransition` output: next factory state + disambiguation index for XState routing. */
export interface ApplyTransitionResult extends StateEngineSendResult {
  readonly nextState: string;
  readonly branchIndex: number;
}

/** @emoji 🎭️ Pluggable state backend for `InteractionRuntime` (pure TS, XState, …). */
export interface StateEngine {
  getState(): string;
  getContext(): Record<string, unknown>;
  reset(): void;
  restore(state: string, context: Record<string, unknown>): void;
  send(event: InteractionEvent, kernel?: SpatialKernel, model?: Model, actions?: ActionRegistry, preview?: SpatialPreviewKernel, activeModelDefinitionId?: string | null): Promise<StateEngineSendResult>;
}

/** @emoji 🎭️ Instantiates a `StateEngine` for a compiled `InteractionSpec`. */
export interface StateEngineProvider {
  readonly id: string;
  create(spec: InteractionSpec): StateEngine;
}

function lookupGuard(spec: InteractionSpec, name: string): Expr | undefined {
  return spec.guards?.find((g) => g.name === name)?.expr;
}

/** @emoji 🎬️ Applies one declarative transition `EffectSpec` (async kernel queries + registered `ActionRegistry` calls). */
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
  if (a.operation === "assign") {
    const v = evalExpr(a.value, env);
    writePathTarget(a.target, env, v);
  } else if (a.operation === "clear") {
    clearPathTarget(a.target, env);
  } else if (a.operation === "append") {
    const cur = readPathTarget(a.target, env);
    const v = evalExpr(a.value, env);
    if (Array.isArray(cur)) {
      const next = [...cur, v];
      writePathTarget(a.target, env, next);
    }
  } else if (a.operation === "kernel.query") {
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
  } else if (a.operation === "interaction.call") {
    return;
  } else if (a.operation === "action") {
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

/** @emoji 🎬️ First matching transition for `event` from `state`; mutates `context` in place. */
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
      if (eff.operation === "interaction.call") {
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

/** @emoji ⌨️ Resolved spatial host hints (defaults disable ground picking). */
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

/** @emoji ⌨️ Merges `spec.interaction` with safe defaults for hosts and `InteractionSpatialView`. */
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

/** @emoji 📏️ Resolved direct-distance entry config for `state` (from `interaction.lengthEntry`). */
export function interactionLengthEntryForState(spec: InteractionSpec, state: string): InteractionLengthEntrySpec | null {
  return mergeInteractionSpatial(spec).lengthEntry.find((row) => row.state === state) ?? null;
}

/** @emoji 🔢️ Resolved live scalar entry config for `state` (from `interaction.scalarEntry`). */
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

/** @emoji 🎛️ Resolves declarative engagement control params for `state` (length/scalar entry + context value). */
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

/** @emoji 🔢️ True when `state` accepts live REPL numeric entry (length or scalar). */
export function interactionInNumericEntryState(spec: InteractionSpec, state: string): boolean {
  return interactionLengthEntryForState(spec, state) !== null || interactionScalarEntryForState(spec, state) !== null;
}

/** @emoji 🔢️ Parses REPL `cmdLine` as a live numeric value (`null` = empty, `undefined` = invalid). */
export function parseNumericCommandLine(cmdLine: string): number | null | undefined {
  const t = cmdLine.trim();
  if (!t) return null;
  if (!/^\d*\.?\d*$/.test(t)) return undefined;
  const v = Number(t);
  if (!Number.isFinite(v) || v <= 0) return undefined;
  return v;
}

/** @emoji 📏️ Live distance along a length-entry anchor→cursor axis (extrusion rod, rubber band). */
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

/** @emoji 🔢️ Explicit length/height lock from context (`set.length` / `set.height`), not live rubber-band distance. */
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

/** @emoji 🔢️ Locked numeric value from context when live entry already applied. */
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

/** @emoji 🔢️ `set.length` / `set.height` event to apply a numeric value in the active entry state. */
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

/** @emoji 🔢️ Commit event after numeric entry (Enter/Space): `pointer.down` with clamped point or `confirm`. */
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

/** @emoji ✅️ Whether `state` has a passable `confirm` transition (non-selection finalize). */
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

/** @emoji ✅️ Enter/Space finalize: `confirm` when available, else length-entry `pointer.down`. */
export function interactionStepFinalizeEvent(spec: InteractionSpec, state: string, ctx: Record<string, unknown>, preview: SpatialPreviewKernel): InteractionEvent | null {
  if (interactionCanFinalizeStep(spec, state, ctx, preview)) return { kind: "confirm", modifiers: {} };
  return interactionNumericEntryCommitEvent(spec, state, ctx, preview);
}

const LENGTH_LOCK_CTX = "__lengthLock";
const HEIGHT_LOCK_CTX = "__heightLock";
const SCALAR_AXIS_T_CTX = "__scalarAxisT";
const CURSOR_RAW_CTX = "__cursorRaw";

const DEFAULT_SCALAR_AXIS: Vec3 = [0, 0, 1];

/** @emoji 📏️ Axis base for scalar rubber-band (`axisAnchor` XY + `axisFloor` Z). */
export function scalarEntryAxisBase(ctx: Record<string, unknown>, entry: InteractionScalarEntrySpec): Vec3 | null {
  if (!entry.axisAnchor) return null;
  const anchor = readInteractionContextVec3(ctx, entry.axisAnchor);
  if (!anchor) return null;
  const floorPath = entry.axisFloor ?? entry.axisAnchor;
  const floor = readInteractionContextVec3(ctx, floorPath);
  const floorZ = floor ? floor[2] : anchor[2];
  return [anchor[0], anchor[1], floorZ];
}

/** @emoji 📏️ Projects `raw` onto the scalar axis; returns axis parameter `t` and closest point. */
export function projectPointOnScalarAxis(base: Vec3, axis: Vec3, raw: Vec3, preview: SpatialPreviewKernel): { readonly projected: Vec3; readonly t: number } {
  return preview.projectPointOnScalarAxis(base, axis, raw);
}

function scalarEntryAxis(entry: InteractionScalarEntrySpec): Vec3 {
  const a = entry.axis;
  if (a && a.length === 3) return [a[0], a[1], a[2]];
  return DEFAULT_SCALAR_AXIS;
}

function positiveHeightLock(ctx: Record<string, unknown>): number | null {
  const lock = ctx[HEIGHT_LOCK_CTX];
  return typeof lock === "number" && Number.isFinite(lock) && lock > 0 ? lock : null;
}

function scalarHeightFromAxisT(t: number): number {
  return Math.max(0.01, Math.abs(t));
}

/** @emoji 📏️ Parses a dotted `context` path into `PathSegment`s (`points.@last` = last array element). */
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

/** @emoji 📏️ Reads a `Vec3` from `context` at dotted `path` (supports `points.@last`). */
export function readInteractionContextVec3(ctx: Record<string, unknown>, path: string): Vec3 | null {
  const raw = readContextPathValue(ctx, parseInteractionContextPath(path));
  if (!Array.isArray(raw) || raw.length < 3) return null;
  const x = Number(raw[0]);
  const y = Number(raw[1]);
  const z = Number(raw[2]);
  if (!Number.isFinite(x) || !Number.isFinite(y) || !Number.isFinite(z)) return null;
  return [x, y, z];
}

/** @emoji 📏️ Writes a `Vec3` into `context` at dotted `path`. */
export function writeInteractionContextVec3(ctx: Record<string, unknown>, path: string, value: Vec3): void {
  writePathSegments(ctx, parseInteractionContextPath(path), value);
}

/** @emoji 📏️ Clamps `target` to `length` units from `anchor` along the anchor→target ray. */
export function clampPointAlongDirection(anchor: Vec3, target: Vec3, length: number, preview: SpatialPreviewKernel): Vec3 {
  return preview.clampPointAlongDirection(anchor, target, length);
}

function positiveLengthLock(ctx: Record<string, unknown>): number | null {
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

/** @emoji ⌨️ One host-triggerable transition row for palette + command input (see `TransitionSpec.key`). */
const HOST_KEYBIND_EXCLUDED_KINDS = new Set(["pointer.move", "pointer.down", "selection.changed"]);

export interface InteractionKeybindRow {
  readonly eventKind: string;
  readonly key: string;
  readonly label: string;
}

/** @emoji ⌨️ Lists keyed transitions for the active state (excludes pointer + selection). */
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

/** @emoji ⎋️ Hard-aborts the active interaction session when `capabilities.canCancel`. */
export function abortActiveInteractionSession(rt: InteractionRuntime): boolean {
  if (!rt.getSnapshot().capabilities.canCancel) return false;
  rt.cancel();
  return true;
}

/** @emoji 🎬️ Minimal async statechart runner for `InteractionSpec.machine`. */
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

  /** @emoji 🎬️ Restores a prior `state` + `context` snapshot (interaction-local undo). */
  restore(state: string, context: Record<string, unknown>): void {
    this.state = state;
    this.context = context;
  }

  /** @emoji 🎬️ Applies one external event; returns whether a transition fired. */
  async send(event: InteractionEvent, kernel?: SpatialKernel, model?: Model, actions?: ActionRegistry, preview?: SpatialPreviewKernel, activeModelDefinitionId?: string | null): Promise<StateEngineSendResult> {
    const r = await applyTransition(this.spec, this.state, this.context, event, kernel, actions, model, preview, activeModelDefinitionId ?? null);
    if (!r.ok) return { ok: false };
    if (r.childCall) return { ok: true, transient: r.transient, childCall: r.childCall };
    this.state = r.nextState;
    return { ok: true, transient: r.transient };
  }
}

/** @emoji 🎭️ Default in-process engine (no XState); same semantics as `applyTransition`. */
export const pureTsStateEngineProvider: StateEngineProvider = {
  id: "pure-ts",
  create(spec: InteractionSpec): StateEngine {
    return new StatechartRuntime(spec);
  },
};
// #endregion 🎬️Statechart

// #region 🖼️Display
/** @emoji 🖼️ Resolved display primitive for renderer adapters. */
export interface DisplayItem {
  readonly kind: string;
  readonly id: string;
  readonly role?: string;
  readonly params?: Record<string, unknown>;
}

/** @emoji 🖼️ Renderer-neutral snapshot slice consumed by `@semio-tech/cad-js/renderer`. */
export interface DisplayModel {
  readonly prompt?: string;
  readonly items: readonly DisplayItem[];
}

/** @emoji 🖼️ Instantiates `display.states[state]` templates using current `context`. */
export function resolveDisplay(spec: InteractionSpec, state: string, context: Record<string, unknown>, preview: SpatialPreviewKernel, model?: Model): DisplayModel {
  const env: ExprEnv = { context: model ? withResolvedInteractionPointsContext(model, context) : context, preview };
  const section = spec.display?.states?.find((s) => s.state === state);
  const raw = section?.items ?? [];
  const items: DisplayItem[] = [];
  for (const it of raw) {
    switch (it.kind) {
      case "point":
        items.push({
          kind: "point",
          id: it.id,
          ...(it.role ? { role: it.role } : {}),
          params: { position: evalExpr(it.position, env) },
        });
        break;
      case "label":
        items.push({
          kind: "label",
          id: it.id,
          ...(it.role ? { role: it.role } : {}),
          params: { text: it.text, position: evalExpr(it.position, env) },
        });
        break;
      case "segment":
        items.push({
          kind: "segment",
          id: it.id,
          ...(it.role ? { role: it.role } : {}),
          params: { from: evalExpr(it.from, env), to: evalExpr(it.to, env) },
        });
        break;
      case "linear-handle":
        items.push({
          kind: "linear-handle",
          id: it.id,
          ...(it.role ? { role: it.role } : {}),
          params: { axis: [...it.axis], origin: evalExpr(it.origin, env) },
        });
        break;
      case "box-preview":
        items.push({
          kind: "box-preview",
          id: it.id,
          ...(it.role ? { role: it.role } : {}),
          params: {
            cornerA: evalExpr(it.cornerA, env),
            cornerB: evalExpr(it.cornerB, env),
            height: evalExpr(it.height, env),
          },
        });
        break;
      case "preview": {
        const params: Record<string, unknown> = {};
        for (const [k, v] of Object.entries(it.params ?? {})) params[k] = evalExpr(v, env);
        items.push({
          kind: "preview",
          id: it.id,
          ...(it.role ? { role: it.role } : {}),
          params: { previewKind: it.previewKind, ...params },
        });
        break;
      }
      case "entity-highlight": {
        const idVal = evalExpr(it.entityId, env);
        items.push({
          kind: "entity-highlight",
          id: it.id,
          ...(it.role ? { role: it.role } : {}),
          params: { entity: { kind: it.geometryEntityKind, id: String(idVal ?? "") } },
        });
        break;
      }
      case "curve":
        items.push({ kind: "curve", id: it.id, ...(it.role ? { role: it.role } : {}) });
        break;
      case "mesh":
        items.push({ kind: "mesh", id: it.id, ...(it.role ? { role: it.role } : {}) });
        break;
      default:
        break;
    }
  }
  const entry = interactionLengthEntryForState(spec, state);
  const lock = positiveLengthLock(context);
  const cursorRaw = readInteractionContextVec3(context, CURSOR_RAW_CTX);
  if (entry && lock != null && cursorRaw) {
    const anchor = readInteractionContextVec3(context, entry.anchor);
    if (anchor) {
      items.push({
        kind: "point",
        id: `${state}-length-cursor`,
        role: "cursor",
        params: { position: cursorRaw },
      });
      items.push({
        kind: "segment",
        id: `${state}-length-guide`,
        role: "guide",
        params: { from: anchor, to: cursorRaw },
      });
      const mid: Vec3 = [(anchor[0] + cursorRaw[0]) / 2, (anchor[1] + cursorRaw[1]) / 2, (anchor[2] + cursorRaw[2]) / 2];
      items.push({
        kind: "label",
        id: `${state}-length-label`,
        role: "prompt",
        params: { text: String(lock), position: mid },
      });
    }
  }
  const scalarEntry = interactionScalarEntryForState(spec, state);
  if (scalarEntry?.axisAnchor) {
    const base = scalarEntryAxisBase(context, scalarEntry);
    const axis = scalarEntryAxis(scalarEntry);
    const heightLock = positiveHeightLock(context);
    const fieldVal = context[scalarEntry.field];
    const height = heightLock ?? (typeof fieldVal === "number" && Number.isFinite(fieldVal) && fieldVal > 0 ? fieldVal : null);
    if (base && height != null) {
      const raw = readInteractionContextVec3(context, CURSOR_RAW_CTX);
      const signedT = typeof context[SCALAR_AXIS_T_CTX] === "number" && Number.isFinite(context[SCALAR_AXIS_T_CTX]) ? (context[SCALAR_AXIS_T_CTX] as number) : height;
      const top = preview.scalarTopOnAxis(base, axis, height, signedT);
      items.push({
        kind: "segment",
        id: `${state}-scalar-height`,
        role: "height",
        params: { from: base, to: top },
      });
      if (raw) {
        const projected = heightLock != null ? top : preview.projectPointOnScalarAxis(base, axis, raw).projected;
        items.push({
          kind: "point",
          id: `${state}-scalar-cursor`,
          role: "cursor",
          params: { position: heightLock != null ? raw : projected },
        });
        if (heightLock != null) {
          items.push({
            kind: "segment",
            id: `${state}-scalar-guide`,
            role: "guide",
            params: { from: top, to: raw },
          });
          const mid: Vec3 = [(top[0] + raw[0]) / 2, (top[1] + raw[1]) / 2, (top[2] + raw[2]) / 2];
          items.push({
            kind: "label",
            id: `${state}-scalar-label`,
            role: "prompt",
            params: { text: String(heightLock), position: mid },
          });
        } else {
          items.push({
            kind: "segment",
            id: `${state}-scalar-guide`,
            role: "guide",
            params: { from: projected, to: raw },
          });
        }
      }
    }
  }
  return { items };
}
// #endregion 🖼️Display

// #endregion 📦️🎬️actions

// #region 🧪️Tests
import { buildBoxInteractionSpec } from "../📄️artifact/🟦️component.ts";

const __actionsTestRuntime = import.meta.vitest ? await import("../🏃️runtime/🟦️component.ts") : null;
const __actionsTestKernel = import.meta.vitest ? await import("../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️component.ts") : null;

if (import.meta.vitest) {
  __actionsTestRuntime!.bootstrapCadModules();
  const { preciseSpatialKernelMath } = __actionsTestKernel!;
  const M = preciseSpatialKernelMath;
  const { describe, expect, it } = import.meta.vitest;

  describe("@semio-tech/cad-js/core box display committed", () => {
    it("keeps box-preview visible for committed state", () => {
      const spec = buildBoxInteractionSpec();
      const ctx: Record<string, unknown> = {
        origin: [0, 0, 0] as Vec3,
        corner: [2, 3, 0] as Vec3,
        height: 4,
      };
      const d = resolveDisplay(spec, "committed", ctx, M);
      const prev = d.items.find((i) => i.kind === "box-preview" && i.id === "preview-committed");
      expect(prev?.params?.cornerA).toEqual([0, 0, 0]);
      expect(prev?.params?.cornerB).toEqual([2, 3, 0]);
      expect(prev?.params?.height).toBe(4);
    });
  });
}
// #endregion 🧪️Tests

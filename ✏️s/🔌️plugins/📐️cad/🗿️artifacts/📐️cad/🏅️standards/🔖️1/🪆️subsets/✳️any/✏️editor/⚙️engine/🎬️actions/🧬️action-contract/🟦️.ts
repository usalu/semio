/** 🧬️ CAD editor actions: the serializable action specs, definitions and registry type, and the parsing of the shipped action catalog. */
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { Expr } from "@semio-tech/s-spatial-kernel-ts/geometry/expressions";
import { ModelDiff, SpatialKernel, SpatialPreviewKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { modelDefinitionActionCatalog } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";

// #region 📦️🎬️actions
// #region 🧮️ActionRegistry
/** 🧩️ Serializable context patch applied after pure box geometry actions (`set` keys merged; `del` removes top-level context keys). */
export interface ActionContextPatch {
  readonly set?: Record<string, unknown>;
  readonly del?: readonly string[];
}

/** 🧩️ Pure action output: model `diff` is the committed geometry; optional `data` is auxiliary; `patch` updates session context only. */
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

/** 🧩️ Registerable spatial action spec (`id` is stable registry key). */
export interface ActionDef<TParams = Record<string, unknown>, TData = unknown> {
  readonly id: string;
  readonly label?: string;
  readonly spec?: ActionSpec;
  readonly run?: ActionFn<TParams, TData>;
}

export function applyActionPatchToContext(ctx: Record<string, unknown>, patch: ActionContextPatch | undefined): void {
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

/** 🧾️ Parses a data-only `spatial.action/v1` document. */
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

/** 📚️ Lists data-only model-definition action assets. */
export function listModelDefinitionActionSpecs(): readonly ActionSpec[] {
  return modelDefinitionActionCatalog()
    .map((raw) => parseActionSpec(raw))
    .filter((spec): spec is ActionSpec => spec !== null);
}

export function shippedActionCatalog(): readonly ActionSpec[] {
  return listModelDefinitionActionSpecs();
}

/** 🧭️ Immutable map of registered data-only `ActionSpec` entries (model-definitions + host overrides). */
export type ActionRegistry = ReadonlyMap<string, ActionDef>;
// #endregion 🧮️ActionRegistry
// #endregion 📦️🎬️actions

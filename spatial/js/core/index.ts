// #region 🧲Header
/** @emoji 🧭 `@spatial/js-core` — model-definition runtime: `Model`, typology/action/interaction catalogs, `ActionRegistry`, `InteractionRegistry`, `StateEngine`, `SpatialKernel`. See `spatial/AGENTS.md` and `spatial/assets/modelDefinition`. */
// #endregion 🧲Header

// #region 📥ModelDefinitionAssets
import geometryLoomFixtureJson from "../../fixtures/geometry-loom.json";
import geometryRoutesFixtureJson from "../../fixtures/geometry-routes.json";
import smallBuildingModelFixtureJson from "../../fixtures/small-building.model.json";

const modelDefinitionTypologyModules = import.meta.glob(
  ["../../assets/modelDefinition/**/typology.json", "../../assets/modelDefinition/**/typology/*.json"],
  {
    eager: true,
    import: "default",
  },
) as Record<string, unknown>;

const modelDefinitionActionModules = import.meta.glob("../../assets/modelDefinition/**/action/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const modelDefinitionInteractionModules = import.meta.glob("../../assets/modelDefinition/**/interaction/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const modelDefinitionManifestModules = import.meta.glob("../../assets/modelDefinition/**/modelDefinition.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const geometryModelDefinitionManifestModule = import.meta.glob("../../assets/modelDefinition/geometry/extension.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const modelDefinitionAttributeModules = import.meta.glob("../../assets/modelDefinition/**/attributeDefinition/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const modelDefinitionPropertyDefinitionModules = import.meta.glob("../../assets/modelDefinition/**/propertyDefinition/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const modelDefinitionPropertyModules = import.meta.glob("../../assets/modelDefinition/**/property/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

function modelDefinitionTypologyCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionTypologyModules);
}

function modelDefinitionActionCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionActionModules);
}

function modelDefinitionInteractionCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionInteractionModules);
}

function modelDefinitionManifestCatalog(): readonly unknown[] {
  return [...Object.values(modelDefinitionManifestModules), ...Object.values(geometryModelDefinitionManifestModule)];
}

function modelDefinitionAttributeCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionAttributeModules);
}

function modelDefinitionPropertyCatalog(): readonly unknown[] {
  return [...Object.values(modelDefinitionPropertyDefinitionModules), ...Object.values(modelDefinitionPropertyModules)];
}
// #endregion 📥ModelDefinitionAssets

// #region 🧮Vec
/** @emoji 📐 Column vector `[x,y,z]` used by spatial factories. */
export type Vec3 = readonly [number, number, number];
// #endregion 🧮Vec

// #region 🌀EdgeGeometry
/** @emoji 🌀 OCCT-style edge curve kinds (`Geom_Curve` under a topologic `Edge`). */
export type EdgeCurve =
  | { readonly kind: "line" }
  | { readonly kind: "arc"; readonly center: Vec3 }
  | { readonly kind: "circle"; readonly center: Vec3; readonly normal: Vec3; readonly radius: number }
  | {
      readonly kind: "ellipse";
      readonly center: Vec3;
      readonly normal: Vec3;
      readonly majorAxis: Vec3;
      readonly majorRadius: number;
      readonly minorRadius: number;
    }
  | {
      readonly kind: "nurbs";
      readonly poles: readonly Vec3[];
      readonly degree: number;
      readonly weights?: readonly number[];
      readonly knots?: readonly number[];
      readonly multiplicities?: readonly number[];
      readonly periodic?: boolean;
      readonly rational?: boolean;
    };

/** @emoji 🔵 Plane frame for a circular arc through `start` and `end` about `center` (CCW in `u×v`). */
export interface ArcPlaneFrame {
  readonly center: Vec3;
  readonly radius: number;
  readonly normal: Vec3;
  readonly u: Vec3;
  readonly v: Vec3;
}
// #endregion 🌀EdgeGeometry

// #region 🧱kernelGeometry

// #region 🎮InteractionEvent
/** @emoji 🧭 Interaction input envelope; `kind` selects `machine.states[*].on` keys. */
export type InteractionEvent = { readonly kind: string; readonly [k: string]: unknown };
// #endregion 🎮InteractionEvent

// #region 🪪Selection
const MODEL_ENTITY_KINDS = new Set<string>(["anchor", "vertex", "edge", "wire", "face", "shell", "solid", "object", "geometry", "attribute"]);

/** @emoji 🪪 One picked geometry or derived view target for `selection.changed`. */
export interface SelectionTarget {
  readonly kind: ModelEntityKind;
  readonly id: string;
  readonly editable: boolean;
  readonly derivedFrom?: readonly { kind: EditableEntityKind; id: string }[];
}

/** @emoji 🪪 Host selection payload; `targets` filtered by `SelectionSpec.accept`. */
export interface SelectionEvent extends InteractionEvent {
  readonly kind: "selection.changed";
  readonly targets: readonly SelectionTarget[];
  readonly point?: Vec3;
}

/** @emoji 🪪 Per-state declarative filter for model vs extension-view picking. */
export interface SelectionSpec {
  readonly accept: readonly ModelEntityKind[];
  readonly multiple?: boolean;
  readonly prompt?: string;
}

/** @emoji 🧭 Returns `targets` whose `kind` is listed in `spec.accept`. */
export function filterSelectionTargets(spec: SelectionSpec, targets: readonly SelectionTarget[]): SelectionTarget[] {
  return targets.filter((t) => spec.accept.includes(t.kind));
}

/** @emoji 🧭 True when every target is accepted (and at least one target exists). */
export function selectionEventMatches(spec: SelectionSpec, ev: SelectionEvent): boolean {
  if (!ev.targets || ev.targets.length === 0) return false;
  const xs = filterSelectionTargets(spec, ev.targets);
  if (xs.length !== ev.targets.length) return false;
  if (!spec.multiple && xs.length > 1) return false;
  return true;
}

/** @emoji 🧭 Active `selection` block for `state`, or `null` when unrestricted. */
export function getActiveSelectionSpec(
  spec: {
    readonly machine: {
      readonly states: readonly { readonly name: string; readonly selection?: SelectionSpec }[];
    };
  },
  state: string,
): SelectionSpec | null {
  const st = spec.machine.states.find((s) => s.name === state);
  const s = st?.selection;
  return s ?? null;
}

/** @emoji ✅ Whether Enter/Space can fire a guarded `confirm` transition in `state`. */
export function interactionCanConfirmSelection(spec: InteractionSpec, state: string, ctx: Record<string, unknown>, preview: SpatialPreviewKernel): boolean {
  if (!getActiveSelectionSpec(spec, state)) return false;
  const handler = spec.machine.states.find((s) => s.name === state)?.on?.find((h) => h.event === "confirm");
  if (!handler) return false;
  for (const tr of handler.transitions) {
    if (typeof tr.key !== "string" || tr.key.length === 0) continue;
    if (tr.guard) {
      const g = lookupGuard(spec, tr.guard);
      if (!g || !evalGuard(g, { context: ctx, preview })) continue;
    }
    return true;
  }
  return false;
}
// #endregion 🪪Selection

// #region 🗺️Paths
/** @emoji 🧭 Root object for segmented path reads (`context` vs `event`). */
export type PathRoot = "context" | "event";

/** @emoji 🧭 One navigation step: object field or array index (no dynamic JSON keys). */
export type PathSegment = { readonly kind: "field"; readonly name: string } | { readonly kind: "index"; readonly index: number };

/** @emoji 🧭 Absolute path into `context` or `event` payloads. */
export interface PathTarget {
  readonly root: PathRoot;
  readonly segments: readonly PathSegment[];
}

/** @emoji 🧭 Reads `segments` from `root` (object/array chain). */
export function readPathSegments(root: unknown, segments: readonly PathSegment[]): unknown {
  let cur: unknown = root;
  for (const seg of segments) {
    if (cur === null || cur === undefined) return undefined;
    if (seg.kind === "field") {
      if (typeof cur !== "object" || Array.isArray(cur)) return undefined;
      cur = (cur as Record<string, unknown>)[seg.name];
    } else {
      if (!Array.isArray(cur)) return undefined;
      cur = cur[seg.index];
    }
  }
  return cur;
}

/** @emoji 🧭 Resolves a `PathTarget` against `ExprEnv`. */
export function readPathTarget(t: PathTarget, env: ExprEnv): unknown {
  const root = t.root === "context" ? env.context : env.event;
  return readPathSegments(root, t.segments);
}

/** @emoji 🧭 Writes `value` at `segments` under `root` (creates object/array shells). */
export function writePathSegments(root: Record<string, unknown>, segments: readonly PathSegment[], value: unknown): void {
  if (segments.length === 0) return;
  let cur: Record<string, unknown> | unknown[] = root;
  for (let i = 0; i < segments.length - 1; i++) {
    const seg = segments[i]!;
    const next = segments[i + 1]!;
    const nextIsIndex = next.kind === "index";
    if (seg.kind === "field") {
      const o = cur as Record<string, unknown>;
      let child = o[seg.name];
      if (child === undefined || child === null || typeof child !== "object") {
        o[seg.name] = nextIsIndex ? ([] as unknown[]) : {};
        child = o[seg.name];
      }
      cur = child as Record<string, unknown> | unknown[];
    } else {
      const arr = cur as unknown[];
      let child = arr[seg.index];
      if (child === undefined || child === null || typeof child !== "object") {
        arr[seg.index] = nextIsIndex ? ([] as unknown[]) : {};
        child = arr[seg.index];
      }
      cur = child as Record<string, unknown> | unknown[];
    }
  }
  const last = segments[segments.length - 1]!;
  if (last.kind === "field") {
    (cur as Record<string, unknown>)[last.name] = value;
  } else {
    (cur as unknown[])[last.index] = value;
  }
}

/** @emoji 🧭 Writes into `env.context` using a context-rooted path. */
export function writePathTarget(t: PathTarget, env: ExprEnv, value: unknown): void {
  if (t.root !== "context") return;
  writePathSegments(env.context, t.segments, value);
}

/** @emoji 🧭 Clears the value at `segments` (deletes final field or sets array slot to `undefined`). */
export function clearPathSegments(root: Record<string, unknown>, segments: readonly PathSegment[]): void {
  if (segments.length === 0) return;
  if (segments.length === 1 && segments[0]!.kind === "field") {
    delete root[segments[0]!.name];
    return;
  }
  let cur: unknown = root;
  for (let i = 0; i < segments.length - 1; i++) {
    const seg = segments[i]!;
    if (cur === null || cur === undefined) return;
    if (seg.kind === "field") cur = (cur as Record<string, unknown>)[seg.name];
    else cur = (cur as unknown[])[seg.index];
  }
  const last = segments[segments.length - 1]!;
  const parent = cur as Record<string, unknown> | unknown[];
  if (last.kind === "field") delete (parent as Record<string, unknown>)[last.name];
  else (parent as unknown[])[last.index] = undefined;
}

/** @emoji 🧭 Clears `target` on `env.context`. */
export function clearPathTarget(t: PathTarget, env: ExprEnv): void {
  if (t.root !== "context") return;
  clearPathSegments(env.context, t.segments);
}
// #endregion 🗺️Paths

// #region 🏷️Metadata
/** @emoji 🏷️ Sidecar semantic fields keyed by geometry or derived entity id (`FaceRef`, `EdgeRef`, …); never stored on brepjs shapes. */
export class AttributeStore {
  private readonly byId = new Map<string, Record<string, unknown>>();

  constructor(private readonly bumpRevision: () => void) {}

  get(id: string): Readonly<Record<string, unknown>> | undefined {
    const r = this.byId.get(id);
    return r ? r : undefined;
  }

  setField(id: string, key: string, value: unknown): void {
    let r = this.byId.get(id);
    if (!r) {
      r = {};
      this.byId.set(id, r);
    }
    r[key] = value;
    this.bumpRevision();
  }

  deleteEntity(id: string): void {
    if (this.byId.delete(id)) this.bumpRevision();
  }
}

/** @emoji 🪪 `evalExpr` `field` target: a bound geometry row entity (`kind` + `id`). */
export interface ModelEntityRef {
  readonly kind: ModelEntityKind;
  readonly id: string;
}
// #endregion 🏷️Metadata

// #region 🗺️Expr
/** @emoji 🗺️ Tagged declarative expression evaluated by `evalExpr` (`spatial/schema/json/expression.json`). */
export type Expr = ExprPath | ExprConst | ExprVar | ExprField | ExprLet | ExprExists | ExprNotEmpty | ExprAll | ExprAny | ExprNot | ExprAbs | ExprDistance | ExprKernelCall | ExprBinop | ExprFold;

export interface ExprPath {
  readonly kind: "path";
  readonly root: PathRoot;
  readonly segments: readonly PathSegment[];
}
export interface ExprConst {
  readonly kind: "const";
  readonly value: unknown;
}
export interface ExprVar {
  readonly kind: "var";
  readonly name: string;
}
export interface ExprField {
  readonly kind: "field";
  readonly object: Expr;
  readonly name: string;
}
export interface ExprLet {
  readonly kind: "let";
  readonly bindings: readonly { readonly name: string; readonly value: Expr }[];
  readonly in: Expr;
}
export interface ExprExists {
  readonly kind: "exists";
  readonly target: PathTarget;
}
export interface ExprNotEmpty {
  readonly kind: "notEmpty";
  readonly target: PathTarget;
}
export interface ExprAll {
  readonly kind: "all";
  readonly args: readonly Expr[];
}
export interface ExprAny {
  readonly kind: "any";
  readonly args: readonly Expr[];
}
export interface ExprNot {
  readonly kind: "not";
  readonly arg: Expr;
}
export interface ExprAbs {
  readonly kind: "abs";
  readonly arg: Expr;
}
export interface ExprDistance {
  readonly kind: "distance";
  readonly a: Expr;
  readonly b: Expr;
}
export interface ExprKernelCall {
  readonly kind: "kernel.call";
  readonly function: string;
  readonly args?: Record<string, Expr>;
}
export interface ExprBinop {
  readonly kind: "binop";
  readonly op: "==" | "!=" | ">" | "<" | ">=" | "<=" | "+" | "-" | "*" | "/";
  readonly left: Expr;
  readonly right: Expr;
}
export interface ExprFold {
  readonly kind: "fold";
  readonly op: "min" | "max";
  readonly args: readonly [Expr, Expr];
}

export interface ExprEnv {
  readonly context: Record<string, unknown>;
  readonly event?: Record<string, unknown>;
  readonly params?: Record<string, unknown>;
  readonly vars?: Record<string, unknown>;
  readonly model?: Model;
  readonly metadata?: AttributeStore;
  readonly views?: null;
  readonly activeViewId?: string | null;
  readonly kernel?: SpatialKernel;
  readonly actionId?: string;
  readonly preview: SpatialPreviewKernel;
}

function envWithVars(base: ExprEnv, vars: Record<string, unknown>): ExprEnv {
  return {
    context: base.context,
    event: base.event,
    params: base.params,
    vars: { ...base.vars, ...vars },
    model: base.model,
    views: base.views,
    activeViewId: base.activeViewId,
    kernel: base.kernel,
    actionId: base.actionId,
    metadata: base.metadata,
    preview: base.preview,
  };
}

function isVec3(v: unknown): v is Vec3 {
  return Array.isArray(v) && v.length === 3 && v.every((n) => typeof n === "number");
}

function isModelEntityRef(v: unknown): v is ModelEntityRef {
  if (!v || typeof v !== "object") return false;
  const o = v as Record<string, unknown>;
  return typeof o.kind === "string" && typeof o.id === "string";
}

/** @emoji 🧮 Evaluates a tagged `Expr` against `ExprEnv` (guards + action values). */
export function evalExpr(expr: Expr, env: ExprEnv): unknown {
  switch (expr.kind) {
    case "const":
      return expr.value;
    case "path":
      return readPathTarget({ root: expr.root, segments: expr.segments }, env);
    case "var":
      return env.vars ? env.vars[expr.name] : undefined;
    case "field": {
      const o = evalExpr(expr.object, env);
      const model = env.model;
      if (model && isModelEntityRef(o)) {
        return readModelEntityProperty(model, env.metadata, o.kind, o.id, expr.name, {
          views: env.views,
          activeViewId: env.activeViewId,
          preview: env.preview,
        });
      }
      if (o !== null && o !== undefined && typeof o === "object" && !Array.isArray(o)) {
        return (o as Record<string, unknown>)[expr.name];
      }
      return undefined;
    }
    case "let": {
      const next: Record<string, unknown> = {};
      for (const b of expr.bindings) {
        next[b.name] = evalExpr(b.value, env);
      }
      return evalExpr(expr.in, envWithVars(env, next));
    }
    case "exists": {
      const v = readPathTarget(expr.target, env);
      return v !== undefined && v !== null;
    }
    case "notEmpty": {
      const v = readPathTarget(expr.target, env);
      if (v === undefined || v === null) return false;
      if (Array.isArray(v)) return v.length > 0;
      if (typeof v === "string") return v.length > 0;
      return true;
    }
    case "all":
      return expr.args.every((x) => Boolean(evalExpr(x, env)));
    case "any":
      return expr.args.some((x) => Boolean(evalExpr(x, env)));
    case "not":
      return !evalExpr(expr.arg, env);
    case "abs": {
      const v = evalExpr(expr.arg, env);
      return typeof v === "number" ? env.preview.abs(v) : undefined;
    }
    case "distance": {
      const va = evalExpr(expr.a, env);
      const vb = evalExpr(expr.b, env);
      if (!isVec3(va) || !isVec3(vb)) return undefined;
      return env.preview.vec3Distance(va, vb);
    }
    case "fold":
      return expr.op === "min" ? env.preview.min2(Number(evalExpr(expr.args[0], env)), Number(evalExpr(expr.args[1], env))) : env.preview.max2(Number(evalExpr(expr.args[0], env)), Number(evalExpr(expr.args[1], env)));
    case "binop": {
      const left = evalExpr(expr.left, env);
      const right = evalExpr(expr.right, env);
      switch (expr.op) {
        case "==":
          return left === right;
        case "!=":
          return left !== right;
        case ">":
          return Number(left) > Number(right);
        case "<":
          return Number(left) < Number(right);
        case ">=":
          return Number(left) >= Number(right);
        case "<=":
          return Number(left) <= Number(right);
        case "+":
          return Number(left) + Number(right);
        case "-":
          return Number(left) - Number(right);
        case "*":
          return Number(left) * Number(right);
        case "/":
          return Number(right) === 0 ? undefined : Number(left) / Number(right);
        default:
          return undefined;
      }
    }
    default:
      return undefined;
  }
}

/** @emoji 🧭 Coerces `evalExpr` output to strict boolean guard result. */
export function evalGuard(expr: Expr, env: ExprEnv): boolean {
  return Boolean(evalExpr(expr, env));
}
// #endregion 🗺️Expr

// #region 📜Spec
/** @emoji 📜 Declared interaction-local context slots (`spatial.interaction/v1` `context`). */
export interface ContextFieldDecl {
  readonly name: string;
  readonly kind: "string" | "number" | "boolean" | "vec3" | "stringArray" | "unknown";
  readonly enumValues?: readonly string[];
}

/** @emoji 📜 Named guard binding (`guards[]`). */
export interface NamedGuard {
  readonly name: string;
  readonly expr: Expr;
}

export interface TransitionSpec {
  readonly target?: string;
  readonly guard?: string;
  readonly transient?: boolean;
  readonly effects?: readonly EffectSpec[];
  readonly key?: string;
  readonly label?: string;
}

export type EffectSpec =
  | { readonly op: "assign"; readonly target: PathTarget; readonly value: Expr }
  | { readonly op: "clear"; readonly target: PathTarget }
  | { readonly op: "append"; readonly target: PathTarget; readonly value: Expr }
  | { readonly op: "emit"; readonly event: { readonly kind: string } }
  | { readonly op: "raise"; readonly event: string }
  | { readonly op: "openTransaction" }
  | { readonly op: "commitTransaction" }
  | { readonly op: "rollbackTransaction" }
  | { readonly op: "requestPreview" }
  | { readonly op: "kernel.query"; readonly query: string; readonly assignTo: PathTarget; readonly params?: Record<string, Expr> }
  | { readonly op: "resolveEditable" }
  | { readonly op: "setDiagnostic"; readonly severity: "info" | "warning" | "error"; readonly code: string; readonly message: string }
  | { readonly op: "clearDiagnostic"; readonly code: string }
  | { readonly op: "action"; readonly action: string; readonly params?: Record<string, Expr> };

export interface EventHandlerSpec {
  readonly event: string;
  readonly transitions: readonly TransitionSpec[];
}

export interface StateDefSpec {
  readonly name: string;
  readonly final?: boolean;
  readonly selection?: SelectionSpec;
  readonly on?: readonly EventHandlerSpec[];
}

export type DisplayItemSpec =
  | { readonly kind: "point"; readonly id: string; readonly role?: string; readonly position: Expr }
  | { readonly kind: "label"; readonly id: string; readonly role?: string; readonly text: string; readonly position: Expr }
  | { readonly kind: "segment"; readonly id: string; readonly role?: string; readonly from: Expr; readonly to: Expr }
  | { readonly kind: "linear-handle"; readonly id: string; readonly role?: string; readonly axis: Vec3; readonly origin: Expr }
  | { readonly kind: "box-preview"; readonly id: string; readonly role?: string; readonly cornerA: Expr; readonly cornerB: Expr; readonly height: Expr }
  | {
      readonly kind: "preview";
      readonly id: string;
      readonly role?: string;
      readonly previewKind: string;
      readonly params?: Record<string, Expr>;
    }
  | {
      readonly kind: "entity-highlight";
      readonly id: string;
      readonly role?: string;
      readonly geometryEntityKind: ModelEntityKind;
      readonly entityId: Expr;
    }
  | { readonly kind: "curve"; readonly id: string; readonly role?: string }
  | { readonly kind: "mesh"; readonly id: string; readonly role?: string };

export type CommitOperationSpec = {
  readonly kind: "action";
  readonly action: string;
  readonly params?: Record<string, Expr>;
};

/** @emoji 📜 Parsed static interaction document (`spatial.interaction/v1`). */
export interface InteractionSpec {
  readonly schema: "spatial.interaction/v1";
  readonly id: string;
  readonly version: string;
  readonly label?: string;
  readonly context?: { readonly fields: readonly ContextFieldDecl[] };
  readonly requires?: Record<string, unknown>;
  readonly guards?: readonly NamedGuard[];
  readonly machine: {
    readonly initial: string;
    readonly states: readonly StateDefSpec[];
  };
  readonly display?: {
    readonly states?: readonly { readonly state: string; readonly items: readonly DisplayItemSpec[] }[];
  };
  readonly interaction?: InteractionSpatialConfig;
  readonly commit: {
    readonly when?: string;
    readonly fromStates?: readonly string[];
    readonly outputDataPath?: PathTarget;
    readonly operation: CommitOperationSpec;
  };
}

/** @emoji 🎮 Host + viewport hints for spatial picking (declared per interaction). */
export interface InteractionSpatialConfig {
  readonly spatialGroundPick?: boolean;
  readonly pickDisabledStates?: readonly string[];
  readonly groundPointerMoveStates?: readonly string[];
  readonly heightDragStates?: readonly string[];
  readonly verticalRodStates?: readonly string[];
  readonly heightConfirmState?: string | null;
}

function guardNames(spec: InteractionSpec): Set<string> {
  return new Set((spec.guards ?? []).map((g) => g.name));
}

function findState(spec: InteractionSpec, name: string): StateDefSpec | undefined {
  return spec.machine.states.find((s) => s.name === name);
}

function isFinalInteractionState(spec: InteractionSpec, state: string): boolean {
  return Boolean(findState(spec, state)?.final);
}

function listFinalInteractionStates(spec: InteractionSpec): string[] {
  return spec.machine.states.filter((s) => s.final).map((s) => s.name);
}

/** @emoji 🧾 Validates and returns an `InteractionSpec` or `null` when malformed. */
export function parseInteractionSpec(raw: unknown): InteractionSpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = structuredClone(raw) as Record<string, unknown>;
  if (r.schema !== "spatial.interaction/v1") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string") return null;
  const machine = r.machine;
  if (!machine || typeof machine !== "object") return null;
  const m = machine as Record<string, unknown>;
  if (typeof m.initial !== "string" || !Array.isArray(m.states)) return null;
  const states = m.states as unknown[];
  const names = new Set<string>();
  for (const st of states) {
    if (!st || typeof st !== "object") return null;
    const s = st as Record<string, unknown>;
    if (typeof s.name !== "string" || s.name.length === 0) return null;
    if (names.has(s.name)) return null;
    names.add(s.name);
    const sel = s.selection;
    if (sel) {
      if (typeof sel !== "object") return null;
      const acc = (sel as Record<string, unknown>).accept;
      if (!Array.isArray(acc) || acc.length === 0) return null;
      for (const k of acc) {
        if (typeof k !== "string" || !MODEL_ENTITY_KINDS.has(k)) return null;
      }
    }
    const on = s.on;
    if (on !== undefined) {
      if (!Array.isArray(on)) return null;
      for (const h of on) {
        if (!h || typeof h !== "object") return null;
        const he = h as Record<string, unknown>;
        if (typeof he.event !== "string" || !Array.isArray(he.transitions)) return null;
        for (const tr of he.transitions as unknown[]) {
          if (!tr || typeof tr !== "object") return null;
          const t = tr as Record<string, unknown>;
          if (t.effects !== undefined && !Array.isArray(t.effects)) return null;
          if (t.actions !== undefined) return null;
        }
      }
    }
  }
  if (!names.has(m.initial as string)) return null;
  const commit = r.commit;
  if (!commit || typeof commit !== "object") return null;
  const c = commit as Record<string, unknown>;
  const op = c.operation;
  if (!op || typeof op !== "object") return null;
  const o = op as Record<string, unknown>;
  if (o.kind !== "action" || typeof o.action !== "string") return null;
  const spec = r as unknown as InteractionSpec;
  const gn = guardNames(spec);
  if (c.when !== undefined && typeof c.when === "string" && !gn.has(c.when)) return null;
  for (const st of spec.machine.states) {
    for (const h of st.on ?? []) {
      for (const tr of h.transitions) {
        if (tr.guard !== undefined && typeof tr.guard === "string" && !gn.has(tr.guard)) return null;
      }
    }
  }
  return spec;
}

const COMPILED_INITIAL_CONTEXTS = new WeakMap<InteractionSpec, Record<string, unknown>>();

function initialStartTransition(spec: InteractionSpec): TransitionSpec | null {
  const initial = findState(spec, spec.machine.initial);
  const handler = initial?.on?.find((h) => h.event === "start");
  const transition = handler?.transitions[0];
  return transition?.target ? transition : null;
}

function staticInitialContext(spec: InteractionSpec, transition: TransitionSpec | null): Record<string, unknown> {
  const context: Record<string, unknown> = {};
  if (!transition?.effects) return context;
  const env: ExprEnv = { context, event: { kind: "start" } };
  for (const effect of transition.effects) {
    if (effect.op === "assign") writePathTarget(effect.target, env, evalExpr(effect.value, env));
    else if (effect.op === "clear") clearPathTarget(effect.target, env);
    else if (effect.op === "append") {
      const cur = readPathTarget(effect.target, env);
      const next = Array.isArray(cur) ? [...cur, evalExpr(effect.value, env)] : [evalExpr(effect.value, env)];
      writePathTarget(effect.target, env, next);
    }
  }
  return context;
}

function compileInitialState(spec: InteractionSpec, transition: TransitionSpec | null): InteractionSpec {
  if (!transition?.target) return spec;
  return {
    ...spec,
    machine: {
      ...spec.machine,
      initial: transition.target,
    },
  };
}

/** @emoji 📜 Scripted commands that end in `committed` should commit from that state, not missing `ready`. */
function normalizeCommitFromStates(spec: InteractionSpec): InteractionSpec {
  const finals = listFinalInteractionStates(spec);
  const hasReady = spec.machine.states.some((s) => s.name === "ready");
  if (hasReady || finals.length === 0) return spec;
  const from = spec.commit.fromStates;
  const onlyReady = !from || (from.length === 1 && from[0] === "ready");
  if (!onlyReady) return spec;
  const fromStates = finals.includes("committed") ? ["committed"] : finals;
  return { ...spec, commit: { ...spec.commit, fromStates } };
}

export function initialContextForSpec(spec: InteractionSpec): Record<string, unknown> {
  return structuredClone(COMPILED_INITIAL_CONTEXTS.get(spec) ?? {});
}

/** @emoji 🧭 Normalizes a parsed interaction so runtime sessions begin in the first active state. */
export function compileInteraction(spec: InteractionSpec): InteractionSpec {
  const start = initialStartTransition(spec);
  if (!start) {
    const normalized = normalizeCommitFromStates(spec);
    if (!COMPILED_INITIAL_CONTEXTS.has(normalized)) COMPILED_INITIAL_CONTEXTS.set(normalized, {});
    return normalized;
  }
  const compiled = normalizeCommitFromStates(compileInitialState(spec, start));
  COMPILED_INITIAL_CONTEXTS.set(compiled, staticInitialContext(spec, start));
  return compiled;
}
// #endregion 📜Spec

export namespace kernelGeometry {
  export type AnchorRef = string & { readonly __brand: "AnchorRef" };
  export type VertexRef = string & { readonly __brand: "VertexRef" };
  export type EdgeRef = string & { readonly __brand: "EdgeRef" };
  export type WireRef = string & { readonly __brand: "WireRef" };
  export type FaceRef = string & { readonly __brand: "FaceRef" };
  export type ShellRef = string & { readonly __brand: "ShellRef" };
  export type SolidRef = string & { readonly __brand: "SolidRef" };
  export type GeometryEntityKind = "anchor" | "vertex" | "edge" | "wire" | "face" | "shell" | "solid";
  export type EditableEntityKind = GeometryEntityKind;
  export function solidRef(id: string): SolidRef {
    return id as SolidRef;
  }

  // #region 🧱ModelGeometry
  /** @emoji 🧱 Kernel-private vertex payload (brepjs persistence; prefer `Object` at framework level). */
  export interface VertexRecord {
    readonly id: VertexRef;
    readonly position: Vec3;
  }

  export type AnchorAttachment =
    | { readonly kind: "vertex"; readonly id: VertexRef }
    | { readonly kind: "edge"; readonly id: EdgeRef; readonly t: number }
    | { readonly kind: "wire"; readonly id: WireRef; readonly t: number }
    | { readonly kind: "face"; readonly id: FaceRef; readonly u: number; readonly v: number }
    | { readonly kind: "solid"; readonly id: SolidRef; readonly u: number; readonly v: number; readonly w: number };

  /** @emoji 🧱 Anchor payload: parametric point attached to kernel geometry. */
  export interface AnchorRecord {
    readonly id: AnchorRef;
    readonly position: Vec3;
    readonly attachment: AnchorAttachment;
  }

  /** @emoji 🧱 Edge payload: two boundary vertices; optional `curve` (`Geom_Curve` analog). */
  export interface EdgeRecord {
    readonly id: EdgeRef;
    readonly vertexIds: readonly VertexRef[];
    readonly curve?: EdgeCurve;
  }

  /** @emoji 🧱 Wire payload: ordered boundary edges. */
  export interface WireRecord {
    readonly id: WireRef;
    readonly edgeIds: readonly EdgeRef[];
  }

  /** @emoji 🌊 Face-support geometry (`Geom_Surface` under a topologic `Face`). */
  export type FaceSurface =
    | { readonly kind: "plane"; readonly origin: Vec3; readonly normal: Vec3 }
    | { readonly kind: "cylinder"; readonly origin: Vec3; readonly axis: Vec3; readonly radius: number }
    | { readonly kind: "sphere"; readonly center: Vec3; readonly radius: number }
    | { readonly kind: "cone"; readonly apex: Vec3; readonly axis: Vec3; readonly radius: number; readonly semiAngle: number }
    | {
        readonly kind: "nurbs";
        readonly poles: readonly (readonly Vec3[])[];
        readonly uDegree: number;
        readonly vDegree: number;
        readonly uKnots?: readonly number[];
        readonly vKnots?: readonly number[];
      };

  /** @emoji 🧱 Face payload: trimming wires + optional underlying surface. */
  export interface FaceRecord {
    readonly id: FaceRef;
    readonly wireIds: readonly WireRef[];
    readonly surface?: FaceSurface;
  }

  /** @emoji 🧱 Shell payload: connected faces. */
  export interface ShellRecord {
    readonly id: ShellRef;
    readonly faceIds: readonly FaceRef[];
  }

  /** @emoji 🧊 Analytic brepjs solid primitive (`box`, `sphere`, `cylinder`, `cone`). */
  export type SolidPrimitive =
    | { readonly kind: "box"; readonly cornerA: Vec3; readonly cornerB: Vec3; readonly height: number }
    | { readonly kind: "sphere"; readonly center: Vec3; readonly radius: number }
    | { readonly kind: "cylinder"; readonly base: Vec3; readonly axis: Vec3; readonly radius: number; readonly height: number }
    | { readonly kind: "cone"; readonly base: Vec3; readonly axis: Vec3; readonly radius: number; readonly height: number; readonly radiusTop?: number };

  /** @emoji 🧱 Solid payload: closed shells and/or analytic primitive. */
  export interface SolidRecord {
    readonly id: SolidRef;
    readonly shellIds: readonly ShellRef[];
    readonly solid?: SolidPrimitive;
  }

  export interface KernelGeometryJson {
    readonly anchors: readonly AnchorRecord[];
    readonly vertices: readonly VertexRecord[];
    readonly edges: readonly EdgeRecord[];
    readonly wires: readonly WireRecord[];
    readonly faces: readonly FaceRecord[];
    readonly shells: readonly ShellRecord[];
    readonly solids: readonly SolidRecord[];
  }
}

type AnchorRef = kernelGeometry.AnchorRef;
export type VertexRef = kernelGeometry.VertexRef;
export type EdgeRef = kernelGeometry.EdgeRef;
export type WireRef = kernelGeometry.WireRef;
export type FaceRef = kernelGeometry.FaceRef;
export type ShellRef = kernelGeometry.ShellRef;
export type SolidRef = kernelGeometry.SolidRef;
type GeometryEntityKind = kernelGeometry.GeometryEntityKind;
type EditableEntityKind = kernelGeometry.EditableEntityKind;

export const solidRef = kernelGeometry.solidRef;

/** @emoji 🧭 Framework + brepjs sub-element selection kinds. */
export type ModelEntityKind = EditableEntityKind | "object" | "geometry" | "attribute";
// #endregion 🧱kernelGeometry

type VertexRecord = kernelGeometry.VertexRecord;
type AnchorAttachment = kernelGeometry.AnchorAttachment;
type AnchorRecord = kernelGeometry.AnchorRecord;
type EdgeRecord = kernelGeometry.EdgeRecord;
type WireRecord = kernelGeometry.WireRecord;
type FaceSurface = kernelGeometry.FaceSurface;
type FaceRecord = kernelGeometry.FaceRecord;
type ShellRecord = kernelGeometry.ShellRecord;
type SolidPrimitive = kernelGeometry.SolidPrimitive;
type SolidRecord = kernelGeometry.SolidRecord;
type KernelGeometryJson = kernelGeometry.KernelGeometryJson;

/** @emoji 🪪 Opaque object id in a model. */
export type ObjectRef = string & { readonly __brand: "ObjectRef" };

/** @emoji 🪪 Typology id referenced by objects and extension assets. */
export type TypologyRef = string & { readonly __brand: "TypologyRef" };

/** @emoji 📦 Object instance row in a model (`typologyId` + kernel `geometryRef`). */
export interface SpatialObjectRecord {
  readonly id: ObjectRef;
  readonly typologyId: TypologyRef;
  readonly geometryRef: string;
  readonly attributes?: Readonly<Record<string, unknown>>;
}

/** @emoji 🗺️ Serializable model (`spatial.model/v1`). */
export interface ModelJson {
  readonly schema: "spatial.model/v1";
  readonly revision: number;
  readonly objects: readonly SpatialObjectRecord[];
  readonly geometry: KernelGeometryJson;
}

function recordsById<T extends { id: string }>(xs: readonly T[]): Record<string, T> {
  const o: Record<string, T> = {};
  for (const x of xs) o[x.id] = x;
  return o;
}

function sortedRecordValues<T extends { id: string }>(bucket: Record<string, T>): T[] {
  return Object.keys(bucket)
    .sort()
    .map((k) => bucket[k]!);
}

/** @emoji 🧱 Mutable in-memory model: objects + kernel-private geometry + attribute store. */
export class Model {
  revision = 0;
  objects: Record<string, SpatialObjectRecord> = {};
  anchors: Record<string, AnchorRecord> = {};
  vertices: Record<string, VertexRecord> = {};
  edges: Record<string, EdgeRecord> = {};
  wires: Record<string, WireRecord> = {};
  faces: Record<string, FaceRecord> = {};
  shells: Record<string, ShellRecord> = {};
  solids: Record<string, SolidRecord> = {};
  readonly metadata: AttributeStore = new AttributeStore(() => this.bump());

  /** @emoji 🧭 Serializes to `ModelJson` (stable id-sorted arrays). */
  toJSON(): ModelJson {
    return {
      schema: "spatial.model/v1",
      revision: this.revision,
      objects: sortedRecordValues(this.objects),
      geometry: {
        anchors: sortedRecordValues(this.anchors),
        vertices: sortedRecordValues(this.vertices),
        edges: sortedRecordValues(this.edges),
        wires: sortedRecordValues(this.wires),
        faces: sortedRecordValues(this.faces),
        shells: sortedRecordValues(this.shells),
        solids: sortedRecordValues(this.solids),
      },
    };
  }

  /** @emoji 🧭 Hydrates from `ModelJson`. */
  static fromJSON(j: ModelJson): Model {
    const g = new Model();
    g.revision = j.revision;
    g.objects = recordsById(j.objects ?? []);
    const geo = j.geometry ?? (j as unknown as KernelGeometryJson);
    g.anchors = recordsById(geo.anchors ?? []);
    g.vertices = recordsById(geo.vertices ?? []);
    g.edges = recordsById(geo.edges ?? []);
    g.wires = recordsById(geo.wires ?? []);
    g.faces = recordsById(geo.faces ?? []);
    g.shells = recordsById(geo.shells ?? []);
    g.solids = recordsById(geo.solids ?? []);
    return g;
  }

  bump(): void {
    this.revision += 1;
  }
}

/** @emoji #️⃣ Stable FNV-1a digest for canonical geometry fingerprints. */
export function fnv1aHex(input: string): string {
  let h = 0x811c9dc5;
  for (let i = 0; i < input.length; i++) {
    h ^= input.charCodeAt(i);
    h = Math.imul(h, 0x01000193);
  }
  return (h >>> 0).toString(16).padStart(8, "0");
}

/** @emoji #️⃣ Opaque content hash for a hashed primitive (vertex position fingerprint). */
export type GeometryPrimitiveHash = string & { readonly __brand: "GeometryPrimitiveHash" };

/** @emoji #️⃣ Quantizes a coordinate for stable hashing. */
export function quantizeCoord(value: number, decimals = 9): number {
  const factor = 10 ** decimals;
  return Math.round(value * factor) / factor;
}

/** @emoji #️⃣ Hashes a vertex position (`spatial/AGENTS.md` primitive hashing). */
export function hashVertexPosition(position: Vec3): GeometryPrimitiveHash {
  const q = position.map((c) => quantizeCoord(c)) as Vec3;
  return `v:${fnv1aHex(`${q[0]},${q[1]},${q[2]}`)}` as GeometryPrimitiveHash;
}

/** @emoji #️⃣ Maps every model vertex id to its position hash. */
export function hashModelVertices(model: Model): Readonly<Record<string, GeometryPrimitiveHash>> {
  const out: Record<string, GeometryPrimitiveHash> = {};
  for (const [id, vertex] of Object.entries(model.vertices)) out[id] = hashVertexPosition(vertex.position);
  return out;
}

/** @emoji 🗺️ Serializable model space (`spatial.modelspace/v1`). */
export interface ModelSpaceJson {
  readonly schema: "spatial.modelspace/v1";
  readonly revision: number;
  readonly models: readonly { readonly id: string; readonly model: ModelJson }[];
}

/** @emoji 🌌 Container for linked models; geometry vertices are hashed per model. */
export class ModelSpace {
  revision = 0;
  models: Record<string, Model> = {};

  /** @emoji 🔗 Registers or replaces a linked model. */
  link(modelId: string, model: Model): void {
    this.models[modelId] = model;
    this.bump();
  }

  /** @emoji ✂️ Removes a linked model. */
  unlink(modelId: string): void {
    if (!(modelId in this.models)) return;
    delete this.models[modelId];
    this.bump();
  }

  /** @emoji 🔍 Returns a linked model or `null`. */
  get(modelId: string): Model | null {
    return this.models[modelId] ?? null;
  }

  /** @emoji #️⃣ Vertex position hashes keyed by linked model id. */
  vertexHashesByModel(): Readonly<Record<string, Readonly<Record<string, GeometryPrimitiveHash>>>> {
    const out: Record<string, Readonly<Record<string, GeometryPrimitiveHash>>> = {};
    for (const [modelId, model] of Object.entries(this.models)) out[modelId] = hashModelVertices(model);
    return out;
  }

  /** @emoji 🧭 Serializes linked models (stable id order). */
  toJSON(): ModelSpaceJson {
    const models = Object.keys(this.models)
      .sort()
      .map((id) => ({ id, model: this.models[id]!.toJSON() }));
    return { schema: "spatial.modelspace/v1", revision: this.revision, models };
  }

  /** @emoji 🧭 Hydrates from `ModelSpaceJson`. */
  static fromJSON(json: ModelSpaceJson): ModelSpace {
    const space = new ModelSpace();
    space.revision = json.revision;
    for (const row of json.models ?? []) space.models[row.id] = Model.fromJSON(row.model);
    return space;
  }

  bump(): void {
    this.revision += 1;
  }
}

/** @emoji 🧭 Reads `name` from metadata, geometry records, or model objects. */
export function readModelEntityProperty(
  model: Model,
  meta: AttributeStore | undefined,
  kind: ModelEntityKind,
  id: string,
  name: string,
  opts?: {
    readonly views?: null;
    readonly activeViewId?: string | null;
    readonly preview?: SpatialPreviewKernel;
  },
): unknown {
  const bag = meta?.get(id);
  if (bag && name in bag) return (bag as Record<string, unknown>)[name];
  switch (kind) {
    case "anchor": {
      const anchor = model.anchors[id];
      if (!anchor) return undefined;
      if (name === "position") return opts?.preview?.evaluateAnchorPosition(model, anchor) ?? anchor.position;
      return (anchor as unknown as Record<string, unknown>)[name];
    }
    case "vertex":
      return (model.vertices[id] as unknown as Record<string, unknown> | undefined)?.[name];
    case "edge":
      return (model.edges[id] as unknown as Record<string, unknown> | undefined)?.[name];
    case "wire":
      return (model.wires[id] as unknown as Record<string, unknown> | undefined)?.[name];
    case "face":
      return (model.faces[id] as unknown as Record<string, unknown> | undefined)?.[name];
    case "shell":
      return (model.shells[id] as unknown as Record<string, unknown> | undefined)?.[name];
    case "solid":
      return (model.solids[id] as unknown as Record<string, unknown> | undefined)?.[name];
    case "object": {
      const hit = model.objects[id];
      if (!hit) return undefined;
      if (name === "id") return id;
      if (name === "typologyId") return hit.typologyId;
      if (name === "geometryRef") return hit.geometryRef;
      return (hit.attributes as Record<string, unknown> | undefined)?.[name];
    }
    case "geometry":
      return model.solids[id] ? id : undefined;
    case "attribute":
      return meta?.get(id)?.[name];
    default:
      return undefined;
  }
}

/** @emoji 🧾 Parses `spatial.model/v1` JSON into a model or returns `null`. */
export function parseModelJson(raw: unknown): Model | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.model/v1") return null;
  const geoKeys = ["anchors", "vertices", "edges", "wires", "faces", "shells", "solids"] as const;
  const geometry: Record<string, unknown> = r.geometry && typeof r.geometry === "object" ? { ...(r.geometry as Record<string, unknown>) } : {};
  if (!Array.isArray(geometry.solids) && Array.isArray((geometry as { cells?: unknown }).cells)) geometry.solids = (geometry as { cells: unknown[] }).cells;
  for (const k of geoKeys) {
    if (!Array.isArray(geometry[k]) && Array.isArray(r[k])) geometry[k] = r[k];
    if (!Array.isArray(geometry[k])) geometry[k] = [];
  }
  const json: ModelJson = {
    schema: "spatial.model/v1",
    revision: typeof r.revision === "number" ? r.revision : 0,
    objects: Array.isArray(r.objects) ? (r.objects as SpatialObjectRecord[]) : [],
    geometry: geometry as KernelGeometryJson,
  };
  return Model.fromJSON(json);
}

/** @emoji 🏷️ Parsed model-definition manifest (`spatial.extension/v1` envelope on disk). */
export interface ModelDefinitionManifest {
  readonly schema: "spatial.extension/v1";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly kinds: readonly string[];
}

/** @emoji 🧾 Parses a model-definition manifest JSON or returns `null`. */
export function parseModelDefinitionManifest(raw: unknown): ModelDefinitionManifest | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.extension/v1") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string" || typeof r.label !== "string") return null;
  if (!Array.isArray(r.kinds) || r.kinds.length === 0) return null;
  return {
    schema: "spatial.extension/v1",
    id: r.id,
    version: r.version,
    label: r.label,
    description: typeof r.description === "string" ? r.description : undefined,
    kinds: r.kinds as string[],
  };
}

/** @emoji 📚 Lists model-definition manifests under spatial/assets/modelDefinition. */
export function listModelDefinitionManifests(): readonly ModelDefinitionManifest[] {
  return modelDefinitionManifestCatalog()
    .map((raw) => parseModelDefinitionManifest(raw))
    .filter((m): m is ModelDefinitionManifest => m !== null);
}

/** @emoji 🧱 Topology primitive kind allowed on typology objects (`spatial/AGENTS.md`). */
export type TypologyPrimitiveKind = "anchor" | "vertex" | "edge" | "wire" | "face" | "shell" | "solid";

/** @emoji 🏷️ Parsed typology asset (`spatial.typology/v1`). */
export interface TypologySpec {
  readonly schema: "spatial.typology/v1";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly primitiveKinds: readonly TypologyPrimitiveKind[];
  readonly actions: readonly string[];
  readonly interactions: readonly string[];
  readonly properties?: readonly string[];
  readonly attributes?: readonly string[];
}

/** @emoji 🧭 Infers default `primitiveKinds` from a shipped typology id when the asset omits the field. */
export function inferTypologyPrimitiveKinds(typologyId: string): readonly TypologyPrimitiveKind[] {
  const id = typologyId.toLowerCase();
  if (id.includes(".selection.") || id.includes(".command.")) return [];
  if (id.includes(".measure.") && id.includes("volume")) return [];
  if (id.includes(".entity.") || id.includes("create-anchor")) return ["anchor"];
  if (id.includes(".measure.")) return ["anchor"];
  if (id.includes("energy.energy.") || id.includes("structure.structure.")) return ["solid"];
  if (id.includes("lineelement") || id.includes("surfaceelement") || id.includes("solidelement")) return ["solid"];
  if (id.includes(".curve.")) return ["edge", "wire"];
  if (id.includes(".surface.")) return ["face"];
  if (id.includes(".primitive.") || id.includes(".solid.")) return ["solid"];
  if (id.includes(".feature.extrude")) return ["solid"];
  if (id.includes(".feature.offset")) return ["face", "solid"];
  if (id.includes(".transform.") || id.includes(".edit.")) return ["vertex", "edge", "wire", "face", "solid"];
  return ["solid"];
}

function parseTypologyPrimitiveKinds(raw: unknown, typologyId: string): readonly TypologyPrimitiveKind[] {
  if (!Array.isArray(raw) || raw.length === 0) return inferTypologyPrimitiveKinds(typologyId);
  const allowed = new Set<TypologyPrimitiveKind>(["anchor", "vertex", "edge", "wire", "face", "shell", "solid"]);
  const kinds: TypologyPrimitiveKind[] = [];
  for (const entry of raw) {
    if (typeof entry !== "string") continue;
    const k = entry as TypologyPrimitiveKind;
    if (allowed.has(k)) kinds.push(k);
  }
  return kinds.length ? kinds : inferTypologyPrimitiveKinds(typologyId);
}

/** @emoji 🧾 Parses `spatial.typology/v1` JSON or returns `null`. */
export function parseTypologySpec(raw: unknown): TypologySpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.typology/v1") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string" || typeof r.label !== "string") return null;
  if (!Array.isArray(r.actions) || !Array.isArray(r.interactions)) return null;
  return {
    schema: "spatial.typology/v1",
    id: r.id,
    version: r.version,
    label: r.label,
    description: typeof r.description === "string" ? r.description : undefined,
    primitiveKinds: parseTypologyPrimitiveKinds(r.primitiveKinds, r.id),
    actions: r.actions as string[],
    interactions: r.interactions as string[],
    properties: Array.isArray(r.properties) ? (r.properties as string[]) : undefined,
    attributes: Array.isArray(r.attributes) ? (r.attributes as string[]) : undefined,
  };
}

function shippedTypologyCatalog(): readonly TypologySpec[] {
  return dedupeDefinitionCatalog(
    modelDefinitionTypologyCatalog()
      .map((raw) => parseTypologySpec(raw))
      .filter((spec): spec is TypologySpec => spec !== null),
  );
}

/** @emoji 📚 Geometry model-definition manifest (`spatial/assets/modelDefinition/geometry/extension.json`). */
export function geometryModelDefinitionManifest(): ModelDefinitionManifest | null {
  const raw = Object.values(geometryModelDefinitionManifestModule)[0];
  return raw ? parseModelDefinitionManifest(raw) : null;
}

/** @emoji 📚 Lists typologies from shipped spatial/assets/modelDefinition assets. */
export function listModelDefinitionTypologies(): readonly TypologySpec[] {
  return shippedTypologyCatalog();
}

/** @emoji 📚 Lists typology assets shipped under geometry model definition (alias). */
export function listBuiltinTypologies(): readonly TypologySpec[] {
  return listModelDefinitionTypologies();
}

/** @emoji 📚 Loads a built-in typology by stable `id`. */
export function loadTypology(typologyId: string): TypologySpec | null {
  return shippedTypologyCatalog().find((t) => t.id === typologyId) ?? null;
}

/** @emoji 📚 Resolves the typology whose `interactions` list includes `interactionId`. */
export function typologyForInteraction(interactionId: string): TypologySpec | null {
  return shippedTypologyCatalog().find((t) => t.interactions.some((id) => id === interactionId)) ?? null;
}

/** @emoji 🏷️ Parsed attribute definition (`spatial.attribute/v1`). */
export interface AttributeDefinitionSpec {
  readonly schema: "spatial.attribute/v1";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly field: string;
  readonly targets: readonly string[];
  readonly value: unknown;
  readonly geometrySelector?: { readonly kinds: readonly string[] };
}

/** @emoji 🧾 Parses `spatial.attribute/v1` JSON or returns `null`. */
export function parseAttributeDefinitionSpec(raw: unknown): AttributeDefinitionSpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.attribute/v1") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string" || typeof r.label !== "string") return null;
  if (typeof r.field !== "string" || !Array.isArray(r.targets) || r.targets.length === 0) return null;
  if (!("value" in r)) return null;
  const selector = r.geometrySelector;
  const geometrySelector =
    selector && typeof selector === "object" && Array.isArray((selector as { kinds?: unknown }).kinds)
      ? { kinds: (selector as { kinds: string[] }).kinds }
      : undefined;
  return {
    schema: "spatial.attribute/v1",
    id: r.id,
    version: r.version,
    label: r.label,
    description: typeof r.description === "string" ? r.description : undefined,
    field: r.field,
    targets: r.targets as string[],
    value: r.value,
    geometrySelector,
  };
}

/** @emoji 🏷️ Parsed property definition (`spatial.property/v1`). */
export interface PropertyDefinitionSpec {
  readonly schema: "spatial.property/v1";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly unit?: string;
  readonly sources?: Readonly<Record<string, unknown>>;
  readonly output?: Readonly<Record<string, unknown>>;
}

/** @emoji 🧾 Parses `spatial.property/v1` JSON or returns `null`. */
export function parsePropertyDefinitionSpec(raw: unknown): PropertyDefinitionSpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.property/v1") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string" || typeof r.label !== "string") return null;
  return {
    schema: "spatial.property/v1",
    id: r.id,
    version: r.version,
    label: r.label,
    description: typeof r.description === "string" ? r.description : undefined,
    unit: typeof r.unit === "string" ? r.unit : undefined,
    sources: r.sources && typeof r.sources === "object" ? (r.sources as Record<string, unknown>) : undefined,
    output: r.output && typeof r.output === "object" ? (r.output as Record<string, unknown>) : undefined,
  };
}

function dedupeDefinitionCatalog<T extends { readonly id: string }>(rows: readonly T[]): readonly T[] {
  const seen = new Set<string>();
  const out: T[] = [];
  for (const row of rows) {
    if (seen.has(row.id)) continue;
    seen.add(row.id);
    out.push(row);
  }
  return out;
}

function shippedAttributeDefinitionCatalog(): readonly AttributeDefinitionSpec[] {
  return dedupeDefinitionCatalog(
    modelDefinitionAttributeCatalog()
      .map((raw) => parseAttributeDefinitionSpec(raw))
      .filter((spec): spec is AttributeDefinitionSpec => spec !== null),
  );
}

function shippedPropertyDefinitionCatalog(): readonly PropertyDefinitionSpec[] {
  return dedupeDefinitionCatalog(
    modelDefinitionPropertyCatalog()
      .map((raw) => parsePropertyDefinitionSpec(raw))
      .filter((spec): spec is PropertyDefinitionSpec => spec !== null),
  );
}

/** @emoji 📚 Lists attribute definitions from model-definition assets. */
export function listModelDefinitionAttributeDefinitions(): readonly AttributeDefinitionSpec[] {
  return shippedAttributeDefinitionCatalog();
}

/** @emoji 📚 Lists property definitions from model-definition assets. */
export function listModelDefinitionPropertyDefinitions(): readonly PropertyDefinitionSpec[] {
  return shippedPropertyDefinitionCatalog();
}

/** @emoji 📚 Loads an attribute definition by stable `id`. */
export function loadAttributeDefinition(attributeId: string): AttributeDefinitionSpec | null {
  return shippedAttributeDefinitionCatalog().find((row) => row.id === attributeId) ?? null;
}

/** @emoji 📚 Loads a property definition by stable `id`. */
export function loadPropertyDefinition(propertyId: string): PropertyDefinitionSpec | null {
  return shippedPropertyDefinitionCatalog().find((row) => row.id === propertyId) ?? null;
}

/** @emoji 🧭 Resolves the primary topology kind referenced by an object's `geometryRef`. */
export function resolveGeometryRefPrimitiveKind(model: Model, geometryRef: string): TypologyPrimitiveKind | null {
  if (model.anchors[geometryRef]) return "anchor";
  if (model.vertices[geometryRef]) return "vertex";
  if (model.edges[geometryRef]) return "edge";
  if (model.wires[geometryRef]) return "wire";
  if (model.faces[geometryRef]) return "face";
  if (model.shells[geometryRef]) return "shell";
  if (model.solids[geometryRef]) return "solid";
  return null;
}

/** @emoji ✅ Whether `typology` allows objects whose geometry resolves to `primitiveKind`. */
export function typologyAllowsPrimitiveKind(typology: TypologySpec, primitiveKind: TypologyPrimitiveKind): boolean {
  return typology.primitiveKinds.includes(primitiveKind);
}

/** @emoji ✅ Whether `object` on `model` satisfies its typology `primitiveKinds`. */
export function objectMatchesTypologyPrimitives(model: Model, object: SpatialObjectRecord): boolean {
  const typology = loadTypology(object.typologyId);
  if (!typology || typology.primitiveKinds.length === 0) return false;
  const kind = resolveGeometryRefPrimitiveKind(model, object.geometryRef);
  return kind ? typologyAllowsPrimitiveKind(typology, kind) : false;
}

/** @emoji 🪪 Kernel topology typology ids used by construct `MATCH` on geometry rows. */
export const KERNEL_TOPOLOGY_TYPOLOGY_IDS: Readonly<Record<TypologyPrimitiveKind, string>> = {
  anchor: "builtin.kernel.anchor",
  vertex: "builtin.kernel.vertex",
  edge: "builtin.kernel.edge",
  wire: "builtin.kernel.wire",
  face: "builtin.kernel.face",
  shell: "builtin.kernel.shell",
  solid: "builtin.kernel.solid",
};

/** @emoji 🧭 Maps typology id → `ModelEntityKind` for construct `Object {typology:…}` patterns. */
export function buildTypologyToEntityKindMap(): Readonly<Record<string, ModelEntityKind>> {
  const out: Record<string, ModelEntityKind> = {};
  for (const [kind, id] of Object.entries(KERNEL_TOPOLOGY_TYPOLOGY_IDS)) out[id] = kind as ModelEntityKind;
  for (const spec of shippedTypologyCatalog()) {
    if (spec.primitiveKinds.length !== 1) continue;
    const kind = spec.primitiveKinds[0]!;
    if (kind === "anchor" && !spec.id.includes("entity") && !spec.id.includes("measure")) continue;
    out[spec.id] = kind;
  }
  return out;
}

/** @emoji ✅ Whether a property definition applies to `object` on `model`. */
export function propertyDefinitionAppliesToObject(defn: PropertyDefinitionSpec, object: SpatialObjectRecord): boolean {
  const typologies = defn.sources?.typologies;
  if (Array.isArray(typologies) && typologies.length > 0) return typologies.includes(object.typologyId);
  const views = defn.sources?.views;
  if (Array.isArray(views) && views.length > 0) return false;
  return true;
}

/** @emoji 📐 Derives property output for one model object from a property definition. */
export async function derivePropertyValue(
  defn: PropertyDefinitionSpec,
  ctx: { readonly model: Model; readonly kernel: SpatialKernel; readonly object: SpatialObjectRecord },
): Promise<Record<string, unknown>> {
  if (!propertyDefinitionAppliesToObject(defn, ctx.object)) return {};
  if (defn.id === "builtin.volume") {
    const kind = resolveGeometryRefPrimitiveKind(ctx.model, ctx.object.geometryRef);
    if (kind !== "solid") return { volume: 0 };
    const volume = await ctx.kernel.solidVolume(ctx.object.geometryRef as SolidRef);
    return { volume };
  }
  if (defn.id === "energy.heatedvolume") {
    const kind = resolveGeometryRefPrimitiveKind(ctx.model, ctx.object.geometryRef);
    if (kind !== "solid") return { heatedvolume: 0 };
    const heatedvolume = await ctx.kernel.solidVolume(ctx.object.geometryRef as SolidRef);
    return { heatedvolume };
  }
  const output = defn.output ?? {};
  return { ...output };
}

/** @emoji 📚 Property definitions that apply to `object` on `model`. */
export function listApplicablePropertyDefinitions(model: Model, object: SpatialObjectRecord): readonly PropertyDefinitionSpec[] {
  return shippedPropertyDefinitionCatalog().filter((defn) => propertyDefinitionAppliesToObject(defn, object));
}

/** @emoji 📚 Attribute definitions whose `targets` include `targetKind`. */
export function listAttributeDefinitionsForTarget(targetKind: string): readonly AttributeDefinitionSpec[] {
  return shippedAttributeDefinitionCatalog().filter((defn) => defn.targets.includes(targetKind));
}

// #endregion 🧱Model

// #region 🧮Diff
export type AnchorRecordDiff = { readonly id: AnchorRef } & Partial<Pick<AnchorRecord, "position" | "attachment">>;
export type VertexRecordDiff = { readonly id: VertexRef } & Partial<Pick<VertexRecord, "position">>;
export type EdgeRecordDiff = { readonly id: EdgeRef } & Partial<Pick<EdgeRecord, "vertexIds" | "curve">>;
export type WireRecordDiff = { readonly id: WireRef } & Partial<Pick<WireRecord, "edgeIds">>;
export type FaceRecordDiff = { readonly id: FaceRef } & Partial<Pick<FaceRecord, "wireIds" | "surface">>;
export type ShellRecordDiff = { readonly id: ShellRef } & Partial<Pick<ShellRecord, "faceIds">>;
export type SolidRecordDiff = { readonly id: SolidRef } & Partial<Pick<SolidRecord, "shellIds" | "solid">>;
/** @emoji 🧮 Forward patch bucket for one geometry table (`added` / `modified` / `removed` arrays). */
export interface EntityDiff<TRec, TDiff, TId extends string> {
  readonly added?: readonly TRec[];
  readonly modified?: readonly TDiff[];
  readonly removed?: readonly TId[];
}

/** @emoji 🧮 Serializable model diff applied by `applyModelDiff`. */
export interface ModelDiff {
  readonly anchors?: EntityDiff<AnchorRecord, AnchorRecordDiff, AnchorRef>;
  readonly vertices?: EntityDiff<VertexRecord, VertexRecordDiff, VertexRef>;
  readonly edges?: EntityDiff<EdgeRecord, EdgeRecordDiff, EdgeRef>;
  readonly wires?: EntityDiff<WireRecord, WireRecordDiff, WireRef>;
  readonly faces?: EntityDiff<FaceRecord, FaceRecordDiff, FaceRef>;
  readonly shells?: EntityDiff<ShellRecord, ShellRecordDiff, ShellRef>;
  readonly solids?: EntityDiff<SolidRecord, SolidRecordDiff, SolidRef>;
}

export const EMPTY_MODEL_DIFF: ModelDiff = {};

function isEntityDiffEmpty<TRec, TDiff, TId extends string>(e: EntityDiff<TRec, TDiff, TId> | undefined): boolean {
  if (!e) return true;
  const a = e.added?.length ?? 0;
  const m = e.modified?.length ?? 0;
  const r = e.removed?.length ?? 0;
  return a === 0 && m === 0 && r === 0;
}

/** @emoji 🧮 True when `diff` has no geometry mutations. */
export function isEmptyModelDiff(d: ModelDiff | undefined): boolean {
  if (!d) return true;
  return (
    isEntityDiffEmpty(d.anchors) &&
    isEntityDiffEmpty(d.vertices) &&
    isEntityDiffEmpty(d.edges) &&
    isEntityDiffEmpty(d.wires) &&
    isEntityDiffEmpty(d.faces) &&
    isEntityDiffEmpty(d.shells) &&
    isEntityDiffEmpty(d.solids)
  );
}

function cloneRec<T>(r: T): T {
  return JSON.parse(JSON.stringify(r)) as T;
}

function applyEntityDiff<T extends { id: string }, TDiff extends { id: string }>(bucket: Record<string, T>, section: EntityDiff<T, TDiff, string> | undefined, inverse: EntityDiff<T, TDiff, string>): void {
  if (!section) return;
  if (section.removed) {
    for (const id of section.removed) {
      const cur = bucket[id];
      if (!cur) continue;
      if (!inverse.added) inverse.added = [];
      (inverse.added as T[]).push(cloneRec(cur));
      delete bucket[id];
    }
  }
  if (section.added) {
    for (const rec of section.added) {
      const id = rec.id;
      bucket[id] = cloneRec(rec as T);
      if (!inverse.removed) inverse.removed = [];
      (inverse.removed as string[]).push(id);
    }
  }
  if (section.modified) {
    for (const md of section.modified) {
      const id = md.id;
      const cur = bucket[id];
      if (!cur) continue;
      const back: Record<string, unknown> = { id };
      const curO = cur as Record<string, unknown>;
      const mdO = md as Record<string, unknown>;
      for (const fk of Object.keys(mdO)) {
        if (fk === "id") continue;
        back[fk] = curO[fk];
        curO[fk] = mdO[fk];
      }
      if (!inverse.modified) inverse.modified = [];
      (inverse.modified as TDiff[]).push(back as TDiff);
    }
  }
}

/** @emoji 🧮 Applies `diff` to `model` in place; returns an inverse `ModelDiff` for `applyModelDiff` again. */
export function applyModelDiff(model: Model, diff: ModelDiff): ModelDiff {
  const inv: ModelDiff = {};
  const aInv: EntityDiff<AnchorRecord, AnchorRecordDiff, AnchorRef> = {};
  const vInv: EntityDiff<VertexRecord, VertexRecordDiff, VertexRef> = {};
  const eInv: EntityDiff<EdgeRecord, EdgeRecordDiff, EdgeRef> = {};
  const wInv: EntityDiff<WireRecord, WireRecordDiff, WireRef> = {};
  const fInv: EntityDiff<FaceRecord, FaceRecordDiff, FaceRef> = {};
  const sInv: EntityDiff<ShellRecord, ShellRecordDiff, ShellRef> = {};
  const cInv: EntityDiff<SolidRecord, SolidRecordDiff, SolidRef> = {};
  applyEntityDiff(model.anchors as Record<string, AnchorRecord>, diff.anchors, aInv);
  applyEntityDiff(model.vertices as Record<string, VertexRecord>, diff.vertices, vInv);
  applyEntityDiff(model.edges as Record<string, EdgeRecord>, diff.edges, eInv);
  applyEntityDiff(model.wires as Record<string, WireRecord>, diff.wires, wInv);
  applyEntityDiff(model.faces as Record<string, FaceRecord>, diff.faces, fInv);
  applyEntityDiff(model.shells as Record<string, ShellRecord>, diff.shells, sInv);
  applyEntityDiff(model.solids as Record<string, SolidRecord>, diff.solids, cInv);
  if (!isEntityDiffEmpty(aInv)) inv.anchors = aInv;
  if (!isEntityDiffEmpty(vInv)) inv.vertices = vInv;
  if (!isEntityDiffEmpty(eInv)) inv.edges = eInv;
  if (!isEntityDiffEmpty(wInv)) inv.wires = wInv;
  if (!isEntityDiffEmpty(fInv)) inv.faces = fInv;
  if (!isEntityDiffEmpty(sInv)) inv.shells = sInv;
  if (!isEntityDiffEmpty(cInv)) inv.solids = cInv;
  if (!isEmptyModelDiff(diff)) model.bump();
  return inv;
}

// #region 🔌SpatialKernelInterface
export type Aabb = { readonly min: Vec3; readonly max: Vec3 };

/** @emoji ⚡ Fast approximate preview math (sync); subset of `SpatialKernel`. */
export interface SpatialPreviewKernel {
  vec3Add(a: Vec3, b: Vec3): Vec3;
  vec3Sub(a: Vec3, b: Vec3): Vec3;
  vec3Scale(a: Vec3, s: number): Vec3;
  vec3Dot(a: Vec3, b: Vec3): number;
  vec3Cross(a: Vec3, b: Vec3): Vec3;
  vec3Length(a: Vec3): number;
  vec3Distance(a: Vec3, b: Vec3): number;
  vec3Normalize(a: Vec3): Vec3;
  arcPlaneFrame(center: Vec3, start: Vec3, end: Vec3): ArcPlaneFrame | null;
  arcSweepRadians(frame: ArcPlaneFrame, end: Vec3): number;
  arcSamplePoints(center: Vec3, start: Vec3, end: Vec3, segments?: number): readonly Vec3[];
  arcFrameFromRadiusPoint(center: Vec3, onCircle: Vec3): ArcPlaneFrame | null;
  arcEndOnCircle(center: Vec3, start: Vec3, pick: Vec3): Vec3;
  arcEndFromAngle(center: Vec3, start: Vec3, angleDeg: number): Vec3 | null;
  circleSamplePoints(center: Vec3, normal: Vec3, radius: number, segments?: number): readonly Vec3[];
  ellipseSamplePoints(center: Vec3, normal: Vec3, majorAxis: Vec3, majorRadius: number, minorRadius: number, segments?: number): readonly Vec3[];
  nurbsDisplaySamplePoints(poles: readonly Vec3[], segmentsPerSpan?: number): readonly Vec3[];
  polylineLength(points: readonly Vec3[]): number;
  edgeCurveLength(curve: EdgeCurve | undefined, ends: readonly Vec3[]): number;
  edgeSamplePoints(vertices: Readonly<Record<string, VertexRecord>>, edge: EdgeRecord, segments?: number): readonly Vec3[];
  circleFromCenterRadiusPoint(center: Vec3, radiusPoint: Vec3): { readonly center: Vec3; readonly normal: Vec3; readonly radius: number } | null;
  nurbsCurveFromPoles(poles: readonly Vec3[]): EdgeCurve | null;
  aabbFromPoints(points: readonly Vec3[]): Aabb | null;
  aabbCornerPoints(min: Vec3, max: Vec3): readonly Vec3[];
  aabbIntersect(a: Aabb, b: Aabb): Aabb | null;
  solidPrimitiveAabb(solid: SolidPrimitive): Aabb;
  modelObjectAabb(model: Model, solid: SolidRecord): Aabb | null;
  boxModelDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }, solid: SolidRef): ModelDiff;
  meshFaceModelDiff(mesh: MeshTransfer, idTag: string): ModelDiff;
  evaluateAnchorPosition(model: Model, anchor: AnchorRecord): Vec3;
  anchorPlacementFromEntity(model: Model, kind: AnchorAttachment["kind"], id: string, point: Vec3): { readonly position: Vec3; readonly attachment: AnchorAttachment } | null;
  computeBoxPreviewLayout(cornerA: Vec3, cornerB: Vec3, height: number): { readonly position: Vec3; readonly scale: Vec3 };
  transformPointsForPreviewKind(previewKind: string, params: Record<string, unknown>): (point: Vec3) => Vec3;
  constrainMovePoint(from: Vec3, to: Vec3, mode: string, cplaneNormal?: Vec3): Vec3;
  abs(x: number): number;
  min2(a: number, b: number): number;
  max2(a: number, b: number): number;
  minN(nums: readonly number[]): number;
  maxN(nums: readonly number[]): number;
  hypot3(x: number, y: number, z: number): number;
  atan2(y: number, x: number): number;
  cos(a: number): number;
  sin(a: number): number;
  randomTag(prefix: string): string;
}

/** @emoji 🔌 Precise BREP kernel: preview math + construction, tessellation, derived views. */
export interface SpatialKernel extends SpatialPreviewKernel {
  readonly id: string;
  readonly operations: readonly string[];
  createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<SolidRef>;
  volume(solid: SolidRef): Promise<number>;
  tessellate(solid: SolidRef, tolerance: number, model?: Model): Promise<MeshTransfer>;
  query?(name: string, params: Record<string, unknown>, ctx?: KernelQueryContext): Promise<unknown>;
  executeAction?(
    actionId: string,
    params: Record<string, unknown>,
    args: Record<string, unknown>,
    ctx: {
      readonly model: Model;
      readonly preview: SpatialPreviewKernel;
      readonly views?: null;
      readonly activeViewId?: string | null;
    },
  ): Promise<ActionResult> | ActionResult;
  executeCommandDiff(commandId: string, params: Record<string, unknown>): Promise<{ readonly diff: ModelDiff }>;
  extrudeWire(input: { wireId: string; distance: number; direction: Vec3; model: Model }): Promise<SolidRef>;
  offsetFaces(input: { faceIds: readonly string[]; distance: number; model: Model }): Promise<void>;
  createBoxFromCornersDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }>;
  extrudeWireDiff(input: { wireId: string; distance: number; direction: Vec3; model: Model }): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }>;
  offsetFacesDiff(input: { faceIds: readonly string[]; distance: number; model: Model }): Promise<{ readonly diff: ModelDiff }>;
  vertexDistance(a: VertexRef, b: VertexRef, model: Model): Promise<number>;
  edgeLength(e: EdgeRef, model: Model): Promise<number>;
  faceArea(f: FaceRef, model: Model): Promise<number>;
  solidVolume(c: SolidRef): Promise<number>;
  adjacentSolids(solid: SolidRef, model: Model): Promise<readonly SolidRef[]>;
  sharedFacesBetween(a: SolidRef, b: SolidRef, model: Model): Promise<readonly FaceRef[]>;
}

/** @emoji 🧩 Triangle index range for one B-Rep face (Three.js `addGroup`). */
export interface FaceGroup {
  readonly start: number;
  readonly count: number;
  readonly entityId: FaceRef;
}

/** @emoji 🧩 Line index range for one B-Rep edge (Three.js edge pick). */
export interface EdgeGroup {
  readonly start: number;
  readonly count: number;
  readonly entityId: EdgeRef;
}

/** @emoji 🧩 Face metadata for kernel→renderer picking and tooltips. */
export interface FaceInfo {
  readonly entityId: FaceRef;
  readonly surfaceType: string;
  readonly area: number;
  readonly normal: readonly [number, number, number];
}

/** @emoji 🧩 Edge metadata for kernel→renderer picking and tooltips. */
export interface EdgeInfo {
  readonly entityId: EdgeRef;
  readonly curveType: string;
  readonly length: number;
}

/** @emoji 🖼️ Zero-copy tessellation payload (grouped buffers + B-Rep edge polylines). */
export interface MeshTransfer {
  readonly position: Float32Array;
  readonly normal: Float32Array;
  readonly index: Uint32Array;
  readonly edges: Float32Array;
  readonly faceGroups: readonly FaceGroup[];
  readonly edgeGroups: readonly EdgeGroup[];
  readonly faceInfos: readonly FaceInfo[];
  readonly edgeInfos: readonly EdgeInfo[];
  readonly color?: string;
}

/** @emoji 🖼️ Empty mesh transfer for stubs and missing solids. */
export function emptyMeshTransfer(): MeshTransfer {
  return {
    position: new Float32Array(0),
    normal: new Float32Array(0),
    index: new Uint32Array(0),
    edges: new Float32Array(0),
    faceGroups: [],
    edgeGroups: [],
    faceInfos: [],
    edgeInfos: [],
  };
}

/** @emoji 🧱 Appends a tessellated commit as one mesh `face` on `Model` (in-memory scene growth). */
export function appendCommittedMeshFaceToModel(model: Model, mesh: MeshTransfer, idTag: string, math: SpatialPreviewKernel): void {
  applyModelDiff(model, math.meshFaceModelDiff(mesh, idTag));
}

/** @emoji 🔌 Optional query context for derived-view resolution in kernel adapters. */
export interface KernelQueryContext {
  readonly model: Model;
  readonly views?: null;
  readonly activeViewId?: string | null;
}
// #endregion 🔌SpatialKernelInterface

// #region 🧮ActionRegistry
/** @emoji 🧩 Serializable context patch applied after pure box geometry actions (`set` keys merged; `del` removes top-level context keys). */
export interface ActionContextPatch {
  readonly set?: Record<string, unknown>;
  readonly del?: readonly string[];
}

/** @emoji 🧩 Pure action output: model `diff` is the committed geometry; optional `data` is auxiliary; `patch` updates session context only. */
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
    readonly views?: null;
    readonly activeViewId?: string | null;
  },
) => Promise<ActionResult<TData>> | ActionResult<TData>;

export interface ActionParameterSpec {
  readonly kind: "string" | "number" | "boolean" | "vec3" | "stringArray" | "unknown";
}

export type ActionStepSpec =
  | { readonly op: "let"; readonly name: string; readonly value: Expr }
  | { readonly op: "setContext"; readonly values: Record<string, Expr> }
  | { readonly op: "deleteContext"; readonly keys: readonly string[] }
  | { readonly op: "kernel.call"; readonly function: string; readonly args?: Record<string, Expr>; readonly assignTo?: string }
  | { readonly op: "guard"; readonly condition: Expr; readonly message?: string }
  | { readonly op: "return"; readonly diff?: Expr; readonly data?: Expr; readonly patch?: Expr; readonly result?: Expr };

export interface ActionSpec {
  readonly schema: "spatial.action/v1";
  readonly id: string;
  readonly version: string;
  readonly label?: string;
  readonly args?: Record<string, unknown>;
  readonly parameters?: Record<string, ActionParameterSpec>;
  readonly variables?: readonly { readonly name: string; readonly value: Expr }[];
  readonly steps: readonly ActionStepSpec[];
}

/** @emoji 🧩 Registerable spatial action spec (`id` is stable registry key). */
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
  if (typeof r.op !== "string") return false;
  if (r.op === "let") return typeof r.name === "string" && Boolean(r.value);
  if (r.op === "setContext") return Boolean(r.values) && typeof r.values === "object" && !Array.isArray(r.values);
  if (r.op === "deleteContext") return Array.isArray(r.keys) && r.keys.every((k) => typeof k === "string");
  if (r.op === "kernel.call") return typeof r.function === "string";
  if (r.op === "guard") return Boolean(r.condition);
  if (r.op === "return") return true;
  return false;
}

/** @emoji 🧾 Parses a data-only `spatial.action/v1` document. */
export function parseActionSpec(raw: unknown): ActionSpec | null {
  if (!raw || typeof raw !== "object" || Array.isArray(raw)) return null;
  const r = structuredClone(raw) as Record<string, unknown>;
  if (hasExecutableActionField(r)) return null;
  if (r.schema !== "spatial.action/v1") return null;
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

/** @emoji 📚 Lists data-only built-in action assets. */
export function listModelDefinitionActionSpecs(): readonly ActionSpec[] {
  return modelDefinitionActionCatalog()
    .map((raw) => parseActionSpec(raw))
    .filter((spec): spec is ActionSpec => spec !== null);
}

/** @emoji 📚 Lists declarative actions from model-definition assets (alias). */
export function listBuiltinActionSpecs(): readonly ActionSpec[] {
  return listModelDefinitionActionSpecs();
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
  const height = numericParam(
    params,
    "height",
    Math.max(Math.abs(cornerB[0] - cornerA[0]), Math.abs(cornerB[1] - cornerA[1]), Math.abs(p2[2] - cornerA[2]), 1),
  );
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
  const ids = collectTargetVertices(ctx.model, targets);
  const added: VertexRecord[] = [];
  const modified: VertexRecordDiff[] = [];
  for (const id of ids) {
    const v = ctx.model.vertices[id];
    if (!v) continue;
    const moved = { id: (copy ? `${id}-copy-${Math.random().toString(36).slice(2, 8)}` : id) as VertexRef, position: ctx.preview.vec3Add(v.position, delta) };
    if (copy) added.push(moved);
    else modified.push(moved);
  }
  return { ...(added.length ? { vertices: { added } } : {}), ...(modified.length ? { vertices: { modified } } : {}) };
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

function builtinActionCapabilityDefs(): readonly ActionDef[] {
  return [
    selectionApplyActionDef(),
    ...SELECTION_OPERATION_INTERACTION_DEFS.map(selectionCommandActionForDef),
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
        return { diff: EMPTY_MODEL_DIFF, patch: { set: { [field]: arr } } };
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
        const bag: Record<string, unknown> = { ...((params.__context as Record<string, unknown>) ?? {}) };
        const points = bag.points;
        if (points && typeof points === "object" && !Array.isArray(points)) Object.assign(bag, points as Record<string, unknown>);
        for (const k of ["commandId", "resultKind", "__context", "__event"]) delete bag[k];
        if (!ctx.kernel.executeCommandDiff) return { diff: EMPTY_MODEL_DIFF, data: params.resultKind ?? null };
        const { diff } = await ctx.kernel.executeCommandDiff(commandId, bag);
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
    { id: "transform.rotate", run: () => ({ diff: EMPTY_MODEL_DIFF }) },
    { id: "transform.scale1d", run: () => ({ diff: EMPTY_MODEL_DIFF }) },
    { id: "transform.scale3d", run: () => ({ diff: EMPTY_MODEL_DIFF }) },
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
    { id: "feature.extrudeWireToSolid", run: async (params, ctx) => ctx.kernel.extrudeWireDiff ? ctx.kernel.extrudeWireDiff({ wireId: String(params.wireId ?? params.wire ?? ""), distance: numericParam(params, "distance", 1), direction: vec3Param(params, "direction", [0, 0, 1]), model: ctx.model }) : ({ diff: EMPTY_MODEL_DIFF }) },
    { id: "feature.offsetFaces", run: async (params, ctx) => ctx.kernel.offsetFacesDiff ? ctx.kernel.offsetFacesDiff({ faceIds: Array.isArray(params.faceIds) ? params.faceIds.map(String) : [String(params.faceId ?? "")], distance: numericParam(params, "distance", 1), model: ctx.model }) : ({ diff: EMPTY_MODEL_DIFF }) },
  ];
}

export async function executeBuiltinActionCapability(
  actionId: string,
  params: Record<string, unknown>,
  args: Record<string, unknown>,
  ctx: {
    readonly kernel: SpatialKernel;
    readonly preview: SpatialPreviewKernel;
    readonly model: Model;
    readonly views?: null;
    readonly activeViewId?: string | null;
  },
): Promise<unknown> {
  const def = builtinActionCapabilityDefs().find((d) => d.id === actionId);
  if (def?.run) return def.run(params, ctx);
  if (ctx.kernel.executeCommandDiff) return ctx.kernel.executeCommandDiff(actionId, params);
  throw new Error(`Unknown action capability: ${actionId}`);
}

async function executeKernelFunction(
  functionName: string,
  actionId: string,
  params: Record<string, unknown>,
  args: Record<string, unknown>,
  ctx: {
    readonly kernel: SpatialKernel;
    readonly preview: SpatialPreviewKernel;
    readonly model: Model;
    readonly views?: null;
    readonly activeViewId?: string | null;
  },
): Promise<unknown> {
  if (functionName !== "spatial.action.execute") throw new Error(`Unknown kernel function: ${functionName}`);
  if (ctx.kernel.executeAction) return ctx.kernel.executeAction(actionId, params, args, ctx);
  return executeBuiltinActionCapability(actionId, params, args, ctx);
}

export class DeclarativeActionRuntime {
  constructor(private readonly spec: ActionSpec) {}

  async run(
    params: Record<string, unknown>,
    ctx: {
      readonly kernel: SpatialKernel;
      readonly preview: SpatialPreviewKernel;
      readonly model: Model;
      readonly views?: null;
      readonly activeViewId?: string | null;
    },
  ): Promise<ActionResult> {
    const vars: Record<string, unknown> = {};
    const env: ExprEnv = {
      context: ((params.__context ?? {}) as Record<string, unknown>) ?? {},
      event: params.__event as Record<string, unknown> | undefined,
      params,
      vars,
      model: ctx.model,
      views: ctx.views,
      activeViewId: ctx.activeViewId,
      kernel: ctx.kernel,
      actionId: this.spec.id,
      metadata: ctx.model.metadata,
      preview: ctx.preview,
    };
    for (const v of this.spec.variables ?? []) vars[v.name] = await Promise.resolve(evalExpr(v.value, env));
    for (const step of this.spec.steps) {
      if (step.op === "let") vars[step.name] = await Promise.resolve(evalExpr(step.value, env));
      else if (step.op === "setContext") Object.assign(env.context, evalExprRecord(step.values, env));
      else if (step.op === "deleteContext") for (const key of step.keys) delete env.context[key];
      else if (step.op === "guard") {
        const ok = Boolean(await Promise.resolve(evalExpr(step.condition, env)));
        if (!ok) throw new Error(step.message ?? `Action guard failed: ${this.spec.id}`);
      } else if (step.op === "kernel.call") {
        const result = await executeKernelFunction(step.function, this.spec.id, params, evalExprRecord(step.args, env), ctx);
        if (step.assignTo) vars[step.assignTo] = result;
      } else if (step.op === "return") {
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

/** @emoji 🧭 Runtime registry for data-only `ActionSpec` entries (built-ins + host overrides). */
export class ActionRegistry {
  private readonly defs = new Map<string, ActionDef>();

  register(def: ActionDef): void {
    this.defs.set(def.id, def);
  }

  get(id: string): ActionDef | null {
    return this.defs.get(id) ?? null;
  }

  list(): readonly ActionDef[] {
    return [...this.defs.values()];
  }

  /** @emoji 🧩 Runs a registered action (`selection.apply`, geometry actions, …) without an interaction session. */
  async run(
    id: string,
    params: Record<string, unknown>,
    ctx: {
      readonly kernel: SpatialKernel;
      readonly preview: SpatialPreviewKernel;
      readonly model: Model;
      readonly views?: null;
      readonly activeViewId?: string | null;
    },
  ): Promise<ActionResult> {
    const def = this.get(id);
    if (def?.spec) return new DeclarativeActionRuntime(def.spec).run(params, ctx);
    if (def?.run) return Promise.resolve(def.run(params, ctx));
    const kernelResult = await executeBuiltinActionCapability(id, params, {}, ctx);
    if (kernelResult && typeof kernelResult === "object" && "diff" in (kernelResult as object)) return kernelResult as ActionResult;
    if (kernelResult && typeof kernelResult === "object" && "patch" in (kernelResult as object)) return kernelResult as ActionResult;
    if (kernelResult !== undefined) return { data: kernelResult };
    throw new Error(`Unknown action: ${id}`);
  }

  static withBuiltins(): ActionRegistry {
    const r = new ActionRegistry();
    for (const spec of shippedActionCatalog()) r.register({ id: spec.id, label: spec.label, spec });
    for (const cap of builtinActionCapabilityDefs()) {
      if (!r.get(cap.id)) r.register(cap);
    }
    return r;
  }
}

/** @emoji 📍 Centroid of a face boundary for measure/annotation anchors. */
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

/** @emoji 📏 Measure and selection commands do not enter document undo history. */
export function interactionRecordsDocumentHistory(interactionId: string): boolean {
  return !interactionId.startsWith("measure.") && !interactionId.startsWith("selection.");
}

/** @emoji 🎯 Collects vertex ids reachable from transform/edit selection targets. */
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

/** @emoji 📦 Center of the axis-aligned bounds of all vertices in `targets`. */
export function selectionTargetsCenter(model: Model, targets: readonly SelectionTarget[], preview: SpatialPreviewKernel): Vec3 | null {
  const pts: Vec3[] = [];
  for (const vid of collectTargetVertices(model, targets)) {
    const v = model.vertices[vid];
    if (v) pts.push(v.position);
  }
  const box = preview.aabbFromPoints(pts);
  if (!box) return null;
  return [(box.min[0] + box.max[0]) / 2, (box.min[1] + box.max[1]) / 2, (box.min[2] + box.max[2]) / 2];
}

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

/** @emoji 🪪 Kernel geometry + extension view `object` kinds used by selection commands. */
export const ALL_MODEL_SELECTION_KINDS: readonly ModelEntityKind[] = ["anchor", "vertex", "edge", "wire", "face", "solid", "object", "geometry", "attribute"];

const MODEL_SELECTION_KIND_ORDER = new Map<ModelEntityKind, number>(ALL_MODEL_SELECTION_KINDS.map((kind, index) => [kind, index]));

/** @emoji 🪪 Built-in selection command operation id (`selection.apply` param). */
export type SelectionApplyOperation = "selectAll" | "deselectAll" | "invert" | "selectKinds";

/** @emoji 🪪 Headless `selection.apply` / interaction commit input. */
export interface SelectionApplyParams {
  readonly operation: SelectionApplyOperation;
  readonly seedTargets?: readonly SelectionTarget[];
  readonly kinds?: readonly ModelEntityKind[];
}

/** @emoji 🪪 Built-in selection command interaction row (`selection.*` registry). */
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

/** @emoji 🪪 Parses `context.targets` or action patch targets into validated `SelectionTarget` rows. */
export function selectionTargetsFromContext(ctx: Record<string, unknown>): readonly SelectionTarget[] {
  return parseSelectionTargetsFromUnknown(ctx.targets);
}

/** @emoji 🪪 Reads `targets` from an `selection.apply` action result patch. */
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

/** @emoji 🪪 Collects stable `SelectionTarget` rows for kernel `kinds` from `model` (+ derived views when provided). */
export function collectGeometrySelectionTargets(model: Model, kinds: readonly ModelEntityKind[], views?: null, activeViewId?: string | null): SelectionTarget[] {
  const out: SelectionTarget[] = [];
  const seen = new Set<string>();
  const push = (kind: ModelEntityKind, id: string, editable = true) => {
    const key = selectionTargetKey({ kind, id, editable });
    if (seen.has(key)) return;
    seen.add(key);
    out.push({ kind, id, editable });
  };
  for (const kind of kinds) {
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
      case "solid":
        for (const id of Object.keys(model.solids)) push(kind, id);
        break;
      case "geometry":
        for (const id of Object.keys(model.solids)) push(kind, id);
        break;
      case "attribute":
        break;
      case "object":
        for (const id of Object.keys(model.objects)) push(kind, id, false);
        break;
    }
  }
  return sortSelectionTargets(out);
}

/** @emoji 🪪 Applies `selectAll` / `deselectAll` / `invert` / `selectKinds` to `current` against `topo`. */
export function applySelectionOperation(operation: SelectionApplyOperation, current: readonly SelectionTarget[], model: Model, kinds: readonly ModelEntityKind[], views?: null, activeViewId?: string | null): SelectionTarget[] {
  if (operation === "deselectAll") return [];
  const scopeKinds = kinds.length > 0 ? kinds : [...ALL_MODEL_SELECTION_KINDS];
  const universe = collectGeometrySelectionTargets(model, scopeKinds, views, activeViewId);
  if (operation === "selectAll" || operation === "selectKinds") return universe;
  const cur = new Set(current.map(selectionTargetKey));
  return universe.filter((target) => !cur.has(selectionTargetKey(target)));
}

/** @emoji 🪪 Shared selection command core used by `selection.apply` and headless callers. */
export function executeSelectionApply(params: SelectionApplyParams, ctx: { readonly model: Model; readonly views?: null; readonly activeViewId?: string | null }): SelectionTarget[] {
  const seed = params.seedTargets ?? [];
  const kinds = params.operation === "selectKinds" ? [...(params.kinds ?? [])] : params.operation === "invert" || params.operation === "selectAll" ? [...ALL_MODEL_SELECTION_KINDS] : [];
  return applySelectionOperation(params.operation, seed, ctx.model, kinds, ctx.views ?? null, ctx.activeViewId ?? null);
}

/** @emoji 🪪 Runs `selection.apply` headless via `ActionRegistry` (no interaction session). */
export async function runSelectionApply(
  params: SelectionApplyParams,
  ctx: {
    readonly kernel: SpatialKernel;
    readonly preview: SpatialPreviewKernel;
    readonly model: Model;
    readonly views?: null;
    readonly activeViewId?: string | null;
    readonly actions?: ActionRegistry;
  },
): Promise<readonly SelectionTarget[]> {
  const actions = ctx.actions ?? ActionRegistry.withBuiltins();
  const result = await actions.run(
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

/** @emoji 🪪 Standard `selection.apply` / `selection.*` construct `CALL` result (`YIELD targets` / `data.targets`). */
export function selectionCommandActionResult(targets: readonly SelectionTarget[]): ActionResult {
  return { patch: { set: { targets: [...targets] } }, diff: EMPTY_MODEL_DIFF, data: { targets } };
}

/** @emoji 🪪 True when `actionId` is a `selection.*` construct or action (`selection.apply`, `selection.selectAll`, …). */
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

function selectionApplyActionDef(): ActionDef {
  return {
    id: "selection.apply",
    run: (params, ctx) => {
      const targets = executeSelectionApply(selectionApplyParamsFromRecord(params as Record<string, unknown>), ctx);
      return selectionCommandActionResult(targets);
    },
  };
}

function selectionCommandActionForDef(defn: SelectionOperationInteractionDef): ActionDef {
  return {
    id: defn.id,
    label: defn.label,
    run: (params, ctx) => {
      const seed = parseSelectionTargetsFromUnknown(params.seedTargets);
      const targets = executeSelectionApply(selectionApplyParamsForInteraction(defn, seed), ctx);
      return selectionCommandActionResult(targets);
    },
  };
}

// #region 🔍ConstructQuery
/** @emoji 🔍 One named column in a `construct` result row. */
export type ConstructQueryRow = Readonly<Record<string, unknown>>;

/** @emoji 🔍 `construct` runner output (`rows` for MATCH; CALL modeling yields `diff` geometry when present). */
export interface ConstructQueryResult {
  readonly rows: readonly ConstructQueryRow[];
  readonly data?: unknown;
  readonly diff?: ModelDiff;
}

/** @emoji 🔍 Host wiring for `InteractionRuntime.query` (`@spatial/js-query` supplies the default runner). */
export interface ConstructQueryContext {
  readonly model: Model;
  readonly kernel: SpatialKernel;
  readonly actions: ActionRegistry;
  readonly views?: null;
  readonly activeViewId?: string | null;
  /** @emoji 🪪 Default `seedTargets` for `CALL selection.*` when the call omits `seedTargets`. */
  readonly selectionTargets?: readonly SelectionTarget[];
}

/** @emoji 🔍 Async bridge so core never imports `@spatial/js-query`. */
export type ConstructRunner = (text: string, ctx: ConstructQueryContext) => Promise<ConstructQueryResult>;
// #endregion 🔍ConstructQuery

// #region 🎬Statechart
/** @emoji 🎭 Result of `StateEngine.send` / `applyTransition` (`transient` skips interaction-local undo). */
export interface StateEngineSendResult {
  readonly ok: boolean;
  readonly transient?: boolean;
}

/** @emoji 🎭 `applyTransition` output: next factory state + disambiguation index for XState routing. */
export interface ApplyTransitionResult extends StateEngineSendResult {
  readonly nextState: string;
  readonly branchIndex: number;
}

/** @emoji 🎭 Pluggable state backend for `InteractionRuntime` (pure TS, XState, …). */
export interface StateEngine {
  getState(): string;
  getContext(): Record<string, unknown>;
  reset(): void;
  restore(state: string, context: Record<string, unknown>): void;
  send(event: InteractionEvent, kernel?: SpatialKernel, model?: Model, actions?: ActionRegistry, views?: null, preview?: SpatialPreviewKernel): Promise<StateEngineSendResult>;
}

/** @emoji 🎭 Instantiates a `StateEngine` for a compiled `InteractionSpec`. */
export interface StateEngineProvider {
  readonly id: string;
  create(spec: InteractionSpec): StateEngine;
}

function lookupGuard(spec: InteractionSpec, name: string): Expr | undefined {
  return spec.guards?.find((g) => g.name === name)?.expr;
}

/** @emoji 🎬 Applies one declarative transition `EffectSpec` (async kernel queries + registered `ActionRegistry` calls). */
export async function applyEffectAsync(
  a: EffectSpec,
  ctx: Record<string, unknown>,
  event: InteractionEvent,
  kernel: SpatialKernel | undefined,
  model: Model,
  actions?: ActionRegistry,
  views?: null,
  preview?: SpatialPreviewKernel,
  activeViewId?: string | null,
): Promise<void> {
  const math = preview ?? kernel;
  if (!math) return;
  const env: ExprEnv = { context: ctx, event, model, views, activeViewId, preview: math };
  const reg = actions ?? ActionRegistry.withBuiltins();
  if (a.op === "assign") {
    const v = evalExpr(a.value, env);
    writePathTarget(a.target, env, v);
  } else if (a.op === "clear") {
    clearPathTarget(a.target, env);
  } else if (a.op === "append") {
    const cur = readPathTarget(a.target, env);
    const v = evalExpr(a.value, env);
    if (Array.isArray(cur)) {
      const next = [...cur, v];
      writePathTarget(a.target, env, next);
    }
  } else if (a.op === "kernel.query") {
    const queryCtx: KernelQueryContext = { model, views: env.views };
    if (a.query === "face.resolveIds") {
      const target = (event as SelectionEvent).targets?.[0];
      const kind = target?.kind ?? "face";
      const id = target?.id ?? "";
      const faceIds = views?.resolveFaceIds(model, kind, id) ?? (kind === "face" && id ? [id as FaceRef] : []);
      writePathTarget(a.assignTo, env, faceIds);
    } else if (kernel?.query) {
      const params: Record<string, unknown> = {};
      const res = await kernel.query(a.query, params, queryCtx);
      writePathTarget(a.assignTo, env, res);
    }
  } else if (a.op === "action") {
    const def = reg.get(a.action);
    if (!def) return;
    const paramBag: Record<string, unknown> = { __context: ctx, __event: event };
    for (const [k, ex] of Object.entries(a.params ?? {})) {
      paramBag[k] = evalExpr(ex, env);
    }
    const k = kernel ?? (null as unknown as SpatialKernel);
    const r = await reg.run(a.action, paramBag, { kernel: k, preview: math, model });
    if (r.patch) applyActionPatchToContext(ctx, r.patch);
  }
}

/** @emoji 🎬 First matching transition for `event` from `state`; mutates `context` in place. */
export async function applyTransition(
  spec: InteractionSpec,
  state: string,
  context: Record<string, unknown>,
  event: InteractionEvent,
  kernel?: SpatialKernel,
  actions?: ActionRegistry,
  model?: Model,
  views?: null,
  preview?: SpatialPreviewKernel,
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
    for (const eff of tr.effects ?? []) {
      await applyEffectAsync(eff, context, event, kernel, graph, actions, views, preview, views?.activeViewId ?? null);
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
  };
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

/** @emoji ⎋ Hard-aborts the active interaction session when `capabilities.canCancel`. */
export function abortActiveInteractionSession(rt: InteractionRuntime): boolean {
  if (!rt.getSnapshot().capabilities.canCancel) return false;
  rt.cancel();
  return true;
}

/** @emoji 🎬 Minimal async statechart runner for `InteractionSpec.machine`. */
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

  /** @emoji 🎬 Restores a prior `state` + `context` snapshot (interaction-local undo). */
  restore(state: string, context: Record<string, unknown>): void {
    this.state = state;
    this.context = context;
  }

  /** @emoji 🎬 Applies one external event; returns whether a transition fired. */
  async send(event: InteractionEvent, kernel?: SpatialKernel, model?: Model, actions?: ActionRegistry, views?: null, preview?: SpatialPreviewKernel): Promise<StateEngineSendResult> {
    const r = await applyTransition(this.spec, this.state, this.context, event, kernel, actions, model, views, preview);
    if (r.ok) this.state = r.nextState;
    return { ok: r.ok, transient: r.transient };
  }
}

/** @emoji 🎭 Default in-process engine (no XState); same semantics as `applyTransition`. */
export const pureTsStateEngineProvider: StateEngineProvider = {
  id: "pure-ts",
  create(spec: InteractionSpec): StateEngine {
    return new StatechartRuntime(spec);
  },
};
// #endregion 🎬Statechart

// #region 🖼️Display
/** @emoji 🖼️ Resolved display primitive for renderer adapters. */
export interface DisplayItem {
  readonly kind: string;
  readonly id: string;
  readonly role?: string;
  readonly params?: Record<string, unknown>;
}

/** @emoji 🖼️ Renderer-neutral snapshot slice consumed by `@spatial/js-renderer-r3f`. */
export interface DisplayModel {
  readonly prompt?: string;
  readonly items: readonly DisplayItem[];
}

/** @emoji 🖼️ Instantiates `display.states[state]` templates using current `context`. */
export function resolveDisplay(spec: InteractionSpec, state: string, context: Record<string, unknown>): DisplayModel {
  const env: ExprEnv = { context };
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
  return { items };
}
// #endregion 🖼️Display

// #region 📄Document
/** @emoji 📄 Single committed modeling operation node. */
export interface ShapeNode {
  readonly id: string;
  readonly operationKind: string;
  readonly solidRef?: SolidRef;
}

/** @emoji 📄 Working document: model + committed shape nodes + command stack. */
export interface ModelDocument {
  readonly model: Model;
  nodes: ShapeNode[];
}

// #endregion 📄Document

// #region 📨Response
/** @emoji 📨 Portable command outcome envelope (`diff` + `data` + messages). */
export interface InteractionMessage {
  readonly code: string;
  readonly message: string;
  readonly path?: string;
}

/** @emoji 📨 Result returned by `InteractionRuntime.commit` — modeling output is always `diff` (model geometry); `data` is auxiliary. */
export interface InteractionResponse<TData = unknown> {
  readonly ok: boolean;
  readonly errors: readonly InteractionMessage[];
  readonly warnings: readonly InteractionMessage[];
  readonly infos: readonly InteractionMessage[];
  readonly diff: ModelDiff;
  readonly data: TData | null;
  /** @emoji 📦 Context clone immediately before the post-commit `confirm` transition; null when commit aborted before confirm. */
  readonly archiveContext: Record<string, unknown> | null;
}

/** @emoji 📨 Default empty success payload for guards and early returns. */
export const EMPTY_INTERACTION_RESPONSE: InteractionResponse<null> = {
  ok: true,
  errors: [],
  warnings: [],
  infos: [],
  diff: EMPTY_MODEL_DIFF,
  data: null,
  archiveContext: null,
};

/** @emoji 📄 One committed model change plus inverse diff for document-level undo/redo. */
export interface Modification {
  readonly id: string;
  readonly interactionId: string;
  readonly label: string;
  readonly result: InteractionResponse;
  readonly backwardsDiff: ModelDiff;
}

/** @emoji 📄 Two-stack modification history (undo / redo) keyed by model diffs. */
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

  /** @emoji 📚 Committed undo stack in document order for renderer views. */
  entries(): readonly Modification[] {
    return [...this.undoStack];
  }

  /** @emoji 🧹 Drops undo and redo stacks when the host swaps the base document. */
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
// #endregion 📨Response

// #region 📜Interaction
/** @emoji 🩺 Non-fatal runtime diagnostic surfaced in snapshots. */
export interface Diagnostic {
  readonly severity: "info" | "warning" | "error";
  readonly code: string;
  readonly message: string;
}

/** @emoji 📜 Serializable interaction snapshot for hosts and renderers. */
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
  readonly query?: ConstructRunner;
  readonly views?: null;
  readonly activeViewId?: string | null;
}

export function isInteractionSessionActive(spec: InteractionSpec, state: string): boolean {
  return !isFinalInteractionState(spec, state);
}

/** @emoji 📜 Headless + interactive interaction controller (`send`, `commit`, `undo`). */
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

  constructor(
    private readonly spec: InteractionSpec,
    private readonly opts: InteractionRuntimeOptions,
  ) {
    this.sm = (opts.stateEngine ?? pureTsStateEngineProvider).create(spec);
    this.actions = opts.actions ?? ActionRegistry.withBuiltins();
  }

  /** @emoji 🔌 Precise BREP kernel wired into this runtime (tessellation, commit, derived views). */
  kernel(): SpatialKernel {
    return this.opts.kernel;
  }

  /** @emoji ⚡ `fast` uses `previewKernel`; `precise` uses the BREP kernel for preview math too. */
  computeMode(): SpatialComputeMode {
    return this.opts.mode ?? "precise";
  }

  /** @emoji ⚡ Active preview kernel for the current `mode` (fast renderer vs precise brep). */
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
    return isInteractionSessionActive(this.spec, this.sm.getState());
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
    const selected = filterSelectionTargets(spec, rawTargets);
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
    const r = await this.sm.send(selectionEvent, this.opts.kernel, this.opts.document.model, this.actions, this.opts.views, this.previewKernel());
    if (!r.ok) return;
    if (!r.transient) this.snapUndoStack.push({ state: stateBeforeSelection, context: JSON.stringify(beforeCtx) });
    const stateAfterSelection = this.sm.getState();
    if (stateAfterSelection === stateBeforeSelection && this.stateHasEvent(stateAfterSelection, "confirm")) {
      const beforeConfirmCtx = this.cloneCtx(this.sm.getContext());
      const cr = await this.sm.send({ kind: "confirm" }, this.opts.kernel, this.opts.document.model, this.actions, this.opts.views, this.previewKernel());
      if (cr.ok && !cr.transient) this.snapUndoStack.push({ state: stateAfterSelection, context: JSON.stringify(beforeConfirmCtx) });
    }
  }

  /** @emoji 🧭 Accepted geometry entity kinds for the active machine state (`[]` when none). */
  listActiveSelectionAccept(): readonly ModelEntityKind[] {
    return getActiveSelectionSpec(this.spec, this.sm.getState())?.accept ?? [];
  }

  /** @emoji 🔍 Executes a `construct` script via `opts.query` (host registers `@spatial/js-query`). */
  async query(text: string): Promise<ConstructQueryResult> {
    const runner = this.opts.query;
    if (!runner) throw new Error("InteractionRuntime.query requires InteractionRuntimeOptions.query");
    return runner(text, {
      model: this.opts.document.model,
      kernel: this.opts.kernel,
      actions: this.actions,
      views: this.opts.views,
      activeViewId: this.opts.activeViewId ?? null,
    });
  }

  getSnapshot(): InteractionSnapshot {
    if (this.snapshotCache) return this.snapshotCache;
    const ctx = this.sm.getContext();
    const st = this.sm.getState();
    const display = resolveDisplay(this.spec, st, ctx);
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

  /** @emoji 📜 Merges `start.targets` when `compileInteraction` began in the post-start state without running transition effects. */
  private applyInstantStartPayload(event: InteractionEvent): void {
    const raw = event.targets;
    if (!Array.isArray(raw) || raw.length === 0) return;
    this.sm.getContext().seedTargets = raw;
  }

  /** @emoji 📜 Dispatches a typed interaction event through the statechart + optional kernel queries. */
  async send(event: InteractionEvent): Promise<void> {
    if (event.kind === "start") {
      await this.consumeStartSelection(event);
      if (this.stateHasEvent(this.sm.getState(), "start")) {
        const beforeState = this.sm.getState();
        const beforeCtx = this.cloneCtx(this.sm.getContext());
        const r = await this.sm.send(event, this.opts.kernel, this.opts.document.model, this.actions, this.opts.views, this.previewKernel());
        if (!r.ok) return;
        if (!r.transient) {
          this.snapUndoStack.push({ state: beforeState, context: JSON.stringify(beforeCtx) });
          this.snapRedoStack.length = 0;
        }
      }
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
    const r = await this.sm.send(event, this.opts.kernel, this.opts.document.model, this.actions, this.opts.views, this.previewKernel());
    if (!r.ok) return;
    if (!r.transient) {
      this.snapUndoStack.push({ state: beforeState, context: JSON.stringify(beforeCtx) });
      this.snapRedoStack.length = 0;
    }
    if (isFinalInteractionState(this.spec, this.sm.getState())) {
      await this.runCommit(false);
      return;
    }
    if (this.canCommit()) {
      await this.runCommit(true);
      return;
    }
    this.emit();
  }

  undo(): void {
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
    if (advanceToFinalState && !this.canCommit()) return fail("interaction.cannotCommit", "Commit guard or fromStates rejected this commit.");
    const ctx = this.sm.getContext();
    const op = this.spec.commit.operation;
    const env: ExprEnv = { context: ctx, preview: this.previewKernel() };
    const k = this.opts.kernel;
    const model = this.opts.document.model;
    let diff: ModelDiff = EMPTY_MODEL_DIFF;
    let data: unknown = null;
    try {
      const paramBag: Record<string, unknown> = { __context: ctx, __event: { kind: "commit" } };
      for (const [key, ex] of Object.entries(op.params ?? {})) {
        paramBag[key] = evalExpr(ex, env);
      }
      const ar = await this.actions.run(op.action, paramBag, {
        kernel: k,
        preview: this.previewKernel(),
        model: model,
        views: this.opts.views,
        activeViewId: this.opts.activeViewId ?? null,
      });
      if (ar.patch) applyActionPatchToContext(this.sm.getContext(), ar.patch);
      diff = ar.diff ?? EMPTY_MODEL_DIFF;
      data = ar.data ?? null;
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      return fail("interaction.commitFailed", msg);
    }
    const outPath = this.spec.commit.outputDataPath;
    if (outPath) {
      const ctx2 = this.sm.getContext();
      writePathTarget(outPath, { context: ctx2, event: undefined }, data);
      data = readPathTarget(outPath, { context: ctx2, event: undefined }) ?? data;
    }
    const inverse = applyModelDiff(model, diff);
    const archiveContext = this.cloneCtx(this.sm.getContext());
    if (advanceToFinalState) await this.sm.send({ kind: "confirm" }, k, model, this.actions, this.opts.views, this.previewKernel());
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

  /** @emoji 📜 Executes `commit.operation` against `kernel`, applies `diff` to `document.model`, records history. */
  async commit(): Promise<InteractionResponse> {
    return this.runCommit(true);
  }
}

/** @emoji 📜 Constructs a `InteractionRuntime` from a compiled `InteractionSpec`. */
export function createInteractionRuntime(spec: InteractionSpec, opts: InteractionRuntimeOptions): InteractionRuntime {
  return new InteractionRuntime(compileInteraction(spec), opts);
}

/** @emoji 🪪 Runs a `selection.*` action (declarative headless command, no interaction session). */
export async function runSelectionOperationInteraction(
  interactionId: string,
  opts: InteractionRuntimeOptions & { readonly seedTargets?: readonly SelectionTarget[] },
): Promise<{ readonly response: InteractionResponse; readonly targets: readonly SelectionTarget[] }> {
  const defn = resolveSelectionOperationInteraction(interactionId);
  if (!defn) throw new Error(`Not a selection operation: ${interactionId}`);
  const seedTargets = opts.seedTargets ?? [];
  const result = await (opts.actions ?? ActionRegistry.withBuiltins()).run(
    interactionId,
    { seedTargets, __context: {}, __event: { kind: "commit" } },
    { kernel: opts.kernel, preview: opts.previewKernel ?? (opts.kernel as unknown as SpatialPreviewKernel), model: opts.document.model },
  );
  const targets = selectionTargetsFromActionResult(result);
  return {
    response: {
      ok: true,
      diff: result.diff ?? EMPTY_MODEL_DIFF,
      archiveContext: { targets },
    },
    targets,
  };
}
// #endregion 📜Interaction

// #region 📦Interactions
type BuiltinInteractionFixture = InteractionSpec & { readonly key?: string };

const SELECTION_OPERATION_INTERACTION_DEFS = [
  { id: "selection.selectAll", label: "SelectAll", key: "sa", operation: "selectAll" },
  { id: "selection.deselectAll", label: "DeselectAll", key: "ds", operation: "deselectAll" },
  { id: "selection.invert", label: "InvertSelection", key: "iv", operation: "invert" },
  { id: "selection.selectAnchors", label: "SelectAnchors", key: "xa", operation: "selectKinds", kinds: ["anchor"] },
  { id: "selection.selectVertices", label: "SelectVertices", key: "xv", operation: "selectKinds", kinds: ["vertex"] },
  { id: "selection.selectEdges", label: "SelectEdges", key: "xe", operation: "selectKinds", kinds: ["edge"] },
  { id: "selection.selectWires", label: "SelectWires", key: "xw", operation: "selectKinds", kinds: ["wire"] },
  { id: "selection.selectFaces", label: "SelectFaces", key: "xf", operation: "selectKinds", kinds: ["face"] },
  { id: "selection.selectSolids", label: "SelectSolids", key: "xc", operation: "selectKinds", kinds: ["solid"] },
  { id: "selection.selectGeometries", label: "SelectGeometries", key: "xg", operation: "selectKinds", kinds: ["geometry"] },
  { id: "selection.selectObjects", label: "SelectObjects", key: "xo", operation: "selectKinds", kinds: ["object"] },
] as const satisfies readonly SelectionOperationInteractionDef[];

/** @emoji 🪪 Built-in instant selection command fixtures (`selection.*`). */
export function listSelectionOperationInteractionDefs(): readonly SelectionOperationInteractionDef[] {
  return SELECTION_OPERATION_INTERACTION_DEFS;
}

/** @emoji 🪪 Resolves a built-in `selection.*` interaction row by stable id. */
export function resolveSelectionOperationInteraction(interactionId: string): SelectionOperationInteractionDef | null {
  return SELECTION_OPERATION_INTERACTION_DEFS.find((defn) => defn.id === interactionId) ?? null;
}

/** @emoji 🪪 Maps a `selection.*` interaction id to headless `SelectionApplyParams`. */
export function selectionApplyParamsForInteraction(defn: SelectionOperationInteractionDef, seedTargets: readonly SelectionTarget[] = []): SelectionApplyParams {
  return {
    operation: defn.operation,
    seedTargets,
    ...(defn.kinds ? { kinds: defn.kinds } : {}),
  };
}

/** @emoji 🪪 True when a selection command targets authored `object` rows on the model. */
export function selectionOperationUsesViewObjects(defn: Pick<SelectionOperationInteractionDef, "kinds">): boolean {
  return defn.kinds?.includes("object") ?? false;
}

/** @emoji 🪪 Default seed targets for invert/deselectAll (otherwise empty). */
export function selectionSeedTargetsForOperation(operation: SelectionApplyOperation, seedCell: SelectionTarget = { kind: "solid", id: "e2e-box", editable: true }): readonly SelectionTarget[] {
  return operation === "invert" || operation === "deselectAll" ? [seedCell] : [];
}

const shippedInteractionJsons = modelDefinitionInteractionCatalog() as readonly BuiltinInteractionFixture[];

function interactionFixtureRow(spec: BuiltinInteractionFixture): SpatialInteraction {
  return { id: spec.id, label: spec.label ?? spec.id, key: typeof spec.key === "string" ? spec.key : (spec.id[0] ?? "?") };
}

/** @emoji 🧭 Built-in `InteractionSpec` registry (fixtures + host `register`). */
export class InteractionRegistry {
  private readonly specs = new Map<string, InteractionSpec>();

  register(spec: InteractionSpec): void {
    this.specs.set(spec.id, spec);
  }

  get(id: string): InteractionSpec | null {
    return this.specs.get(id) ?? null;
  }

  list(): readonly InteractionSpec[] {
    return [...this.specs.values()];
  }

  static withBuiltins(): InteractionRegistry {
    const r = new InteractionRegistry();
    const xs = shippedInteractionJsons.map((raw) => {
      const spec = parseInteractionSpec(raw);
      return spec ? compileInteraction(spec) : null;
    });
    for (const s of xs) {
      if (s) r.register(s);
    }
    return r;
  }
}

/** @emoji 📦 Parses primitive.box interaction from model-definition assets. */
export function buildBoxInteractionSpec(): InteractionSpec {
  const raw = shippedInteractionJsons.find((row) => row.id === "primitive.box");
  const spec = raw ? parseInteractionSpec(raw) : null;
  if (!spec) throw new Error("primitive.box interaction missing from modelDefinition assets");
  return compileInteraction(spec);
}

/** @emoji 📦 Parses feature.extrude-wire interaction from model-definition assets. */
export function buildExtrudeInteractionSpec(): InteractionSpec {
  const raw = shippedInteractionJsons.find((row) => row.id === "feature.extrude-wire");
  const spec = raw ? parseInteractionSpec(raw) : null;
  if (!spec) throw new Error("feature.extrude-wire interaction missing");
  return compileInteraction(spec);
}

/** @emoji 📦 Parses feature.offset-surface interaction from model-definition assets. */
export function buildOffsetSurfaceInteractionSpec(): InteractionSpec {
  const raw = shippedInteractionJsons.find((row) => row.id === "feature.offset-surface");
  const spec = raw ? parseInteractionSpec(raw) : null;
  if (!spec) throw new Error("feature.offset-surface interaction missing");
  return compileInteraction(spec);
}

/** @emoji 📦 Parses measure.length interaction from model-definition assets. */
export function buildDistanceInteractionSpec(): InteractionSpec {
  const raw = shippedInteractionJsons.find((row) => row.id === "measure.distance" || row.id === "measure.length");
  const spec = raw ? parseInteractionSpec(raw) : null;
  if (!spec) throw new Error("measure.length interaction missing");
  return compileInteraction(spec);
}

/** @emoji 📦 Parses measure.area interaction from model-definition assets. */
export function buildAreaInteractionSpec(): InteractionSpec {
  const raw = shippedInteractionJsons.find((row) => row.id === "measure.area");
  const spec = raw ? parseInteractionSpec(raw) : null;
  if (!spec) throw new Error("measure.area interaction missing");
  return compileInteraction(spec);
}



/** @emoji 📚 Host-facing interaction row from spatial/assets/modelDefinition interaction JSON. */
export interface SpatialInteraction {
  readonly id: string;
  readonly label: string;
  /** @emoji ⌨️ Host interaction key; must stay unique and appear in `label` (see `resolveSpatialInteractionKey`). */
  readonly key: string;
}

/** @emoji 📚 Interaction ids from shipped model-definition assets. */
export function listSpatialInteractions(): readonly SpatialInteraction[] {
  return shippedInteractionJsons.map(interactionFixtureRow);
}

/** @emoji 🧭 Resolves a typed token to an interaction (`key`, `id`, or compact `label`). */
export function resolveSpatialInteractionKey(token: string): SpatialInteraction | null {
  const t = token.trim().toLowerCase();
  if (!t) return null;
  for (const p of listSpatialInteractions()) {
    if (p.key.toLowerCase() === t) return p;
    if (p.id.toLowerCase() === t) return p;
    const slug = p.label.toLowerCase().replace(/\s+/g, "");
    if (slug === t) return p;
  }
  return null;
}

/** @emoji 📚 Loads a built-in interaction by stable `id` (see `listSpatialInteractions`). */
export function loadSpatialInteraction(interactionId: string): InteractionSpec | null {
  const raw = shippedInteractionJsons.find((spec) => spec.id === interactionId);
  const spec = raw ? parseInteractionSpec(raw) : null;
  return spec ? compileInteraction(spec) : null;
}
// #endregion 📦Interactions

// #region 🧪Tests
const __spatialCoreTestKernel = import.meta.vitest ? await import("@spatial/js-kernel-brepjs") : null;

if (import.meta.vitest) {
  const { BrepjsKernel, preciseSpatialKernelMath } = __spatialCoreTestKernel!;
  const M = preciseSpatialKernelMath;
  const { describe, expect, it } = import.meta.vitest;

  describe("@spatial/js-core vec", () => {
    it("adds and distances", () => {
      expect(M.vec3Distance([0, 0, 0], [3, 4, 0])).toBe(5);
    });
  });

  describe("@spatial/js-core model definition catalogs", () => {
    it("loads attribute and property definition assets", () => {
      const attributes = listModelDefinitionAttributeDefinitions();
      const properties = listModelDefinitionPropertyDefinitions();
      expect(attributes.length).toBeGreaterThanOrEqual(6);
      expect(properties.some((row) => row.id === "builtin.volume")).toBe(true);
      expect(loadAttributeDefinition("builtin.material")?.field).toBe("material");
      expect(loadPropertyDefinition("builtin.volume")?.unit).toBe("volume");
    });
    it("loads geometry and AEC typology assets", () => {
      const typologies = listModelDefinitionTypologies();
      expect(typologies.length).toBeGreaterThanOrEqual(67);
      expect(loadTypology("energy.energy.hull")?.properties).toContain("builtin.volume");
      expect(loadTypology("builtin.measure.volume")?.primitiveKinds).toEqual([]);
    });
    it("assigns primitiveKinds to geometry typologies", () => {
      const box = loadTypology("builtin.primitive.box");
      const line = loadTypology("builtin.curve.line");
      const selectAll = loadTypology("builtin.selection.select-all");
      expect(box?.primitiveKinds).toEqual(["solid"]);
      expect(line?.primitiveKinds).toEqual(["edge", "wire"]);
      expect(selectAll?.primitiveKinds).toEqual([]);
    });
    it("derives builtin.volume for solid-backed objects", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      const solidId = Object.keys(model.solids)[0]!;
      const object = {
        id: "obj-a" as ObjectRef,
        typologyId: "builtin.primitive.box" as TypologyRef,
        geometryRef: solidId,
      };
      const defn = loadPropertyDefinition("builtin.volume")!;
      const kernel = {
        solidVolume: async () => 42,
      } as unknown as SpatialKernel;
      const out = await derivePropertyValue(defn, { model, kernel, object });
      expect(out.volume).toBe(42);
      expect(listApplicablePropertyDefinitions(model, object).map((row) => row.id)).toContain("builtin.volume");
    });
    it("validates object geometry against typology primitiveKinds", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      const solidId = Object.keys(model.solids)[0]!;
      model.objects["obj-a"] = {
        id: "obj-a" as ObjectRef,
        typologyId: "builtin.primitive.box" as TypologyRef,
        geometryRef: solidId,
      };
      expect(objectMatchesTypologyPrimitives(model, model.objects["obj-a"]!)).toBe(true);
      model.objects["obj-b"] = {
        id: "obj-b" as ObjectRef,
        typologyId: "builtin.selection.select-all" as TypologyRef,
        geometryRef: solidId,
      };
      expect(objectMatchesTypologyPrimitives(model, model.objects["obj-b"]!)).toBe(false);
    });
  });

  describe("@spatial/js-core model space and hashing", () => {
    it("hashes vertex positions stably", () => {
      const a = hashVertexPosition([1, 2, 3]);
      const b = hashVertexPosition([1.0000000004, 2, 3]);
      expect(a).toBe(b);
      expect(hashVertexPosition([1, 2, 4])).not.toBe(a);
    });
    it("links models in a model space", () => {
      const space = new ModelSpace();
      const m = new Model();
      applyModelDiff(m, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      space.link("primary", m);
      expect(space.get("primary")).toBe(m);
      const hashes = space.vertexHashesByModel();
      expect(Object.keys(hashes.primary ?? {}).length).toBe(8);
      const roundTrip = ModelSpace.fromJSON(space.toJSON());
      expect(roundTrip.get("primary")?.revision).toBe(m.revision);
      expect(hashModelVertices(roundTrip.get("primary")!)).toEqual(hashes.primary);
    });
  });

  describe("@spatial/js-core edge and solid geometry", () => {
    it("arcEndOnCircle projects off-circle pick onto arc", () => {
      const end = M.arcEndOnCircle([0, 0, 0], [2, 0, 0], [0, 3, 0]);
      expect(end[0]).toBeCloseTo(0, 5);
      expect(end[1]).toBeCloseTo(2, 5);
      expect(M.vec3Distance([0, 0, 0], end)).toBeCloseTo(2, 5);
    });
    it("arcSamplePoints quarter arc from center start end", () => {
      const pts = M.arcSamplePoints([0, 0, 0], [2, 0, 0], [0, 2, 0], 4);
      expect(pts[0]).toEqual([2, 0, 0]);
      expect(pts[pts.length - 1]![0]).toBeCloseTo(0, 5);
      expect(pts[pts.length - 1]![1]).toBeCloseTo(2, 5);
    });
    it("circleSamplePoints and edgeCurveLength for Geom_Circle", () => {
      const pts = M.circleSamplePoints([0, 0, 0], [0, 0, 1], 2, 64);
      expect(pts.length).toBeGreaterThan(8);
      expect(
        M.edgeCurveLength({ kind: "circle", center: [0, 0, 0], normal: [0, 0, 1], radius: 2 }, [
          [2, 0, 0],
          [2, 0, 0],
        ]),
      ).toBeCloseTo(M.cos(0) * 0 + M.sin(0) * 0 + 4 * 3.141592653589793, 3);
    });
    it("nurbs edge samples through control poles", () => {
      const curve = M.nurbsCurveFromPoles([
        [0, 0, 0],
        [1, 2, 0],
        [3, 0, 0],
      ] as Vec3[])!;
      const v0 = "v0" as VertexRef;
      const v1 = "v1" as VertexRef;
      const verts = {
        [v0]: { id: v0, position: [0, 0, 0] as Vec3 },
        [v1]: { id: v1, position: [3, 0, 0] as Vec3 },
      };
      const edge: EdgeRecord = { id: "e" as EdgeRef, vertexIds: [v0, v1], curve };
      const pts = M.edgeSamplePoints(verts, edge, 24);
      expect(pts.length).toBeGreaterThan(4);
    });
    it("solidPrimitiveAabb sphere bounds", () => {
      const b = M.solidPrimitiveAabb({ kind: "sphere", center: [1, 2, 3], radius: 5 });
      expect(b.min).toEqual([-4, -3, -2]);
      expect(b.max).toEqual([6, 7, 8]);
    });
  });

  describe("@spatial/js-core expr", () => {
    it("evaluates numeric fold min expr", () => {
      const e: Expr = {
        kind: "fold",
        op: "min",
        args: [
          { kind: "const", value: 3 },
          { kind: "const", value: 7 },
        ],
      };
      expect(evalExpr(e, { context: {}, preview: M })).toBe(3);
    });
    it("evaluates guards used by box factory", () => {
      const g: Expr = {
        kind: "all",
        args: [
          { kind: "exists", target: { root: "context", segments: [{ kind: "field", name: "origin" }] } },
          {
            kind: "binop",
            op: ">",
            left: { kind: "path", root: "context", segments: [{ kind: "field", name: "height" }] },
            right: { kind: "const", value: 0 },
          },
        ],
      };
      expect(evalGuard(g, { context: { origin: [0, 0, 0], height: 2 } })).toBe(true);
    });
  });

  describe("@spatial/js-core model json", () => {
    it("parseModelJson fills missing entity arrays with empty lists", () => {
      const model = parseModelJson({
        schema: "spatial.model/v1",
        revision: 1,
        objects: [],
        geometry: {
          vertices: [{ id: "v0", position: [0, 0, 0] }],
          edges: [{ id: "e0", vertexIds: ["v0", "v0"] }],
        },
      });
      expect(model).not.toBeNull();
      expect(Object.keys(model!.anchors).length).toBe(0);
      expect(Object.keys(model!.vertices).length).toBe(1);
      expect(Object.keys(model!.edges).length).toBe(1);
    });
  });

  describe("@spatial/js-core model commit mesh", () => {
    it("appendCommittedMeshFaceToModel adds one mesh face from a triangle mesh", () => {
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
      appendCommittedMeshFaceToModel(g, mesh, "t0", M);
      expect(Object.keys(g.faces).length).toBe(1);
      expect(g.revision).toBeGreaterThan(0);
    });
  });

  describe("@spatial/js-core metadata", () => {
    it("AttributeStore setField bumps model revision", () => {
      const g = new Model();
      const r0 = g.revision;
      g.metadata.setField("e1", "exposure", "external");
      expect(g.revision).toBeGreaterThan(r0);
      expect(g.metadata.get("e1")?.exposure).toBe("external");
    });
  });

  describe("@spatial/js-core interactions", () => {
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
      await rt.send({ kind: "start", targets: [{ kind: "vertex", id: v0, editable: true }], modifiers: {} });
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
      await rt.send({
        kind: "start",
        targets: verts.map((v) => ({ kind: "vertex" as const, id: v.id, editable: true })),
        modifiers: {},
      });
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
      await rt.send({ kind: "start", targets: [{ kind: "vertex", id: v0, editable: true }], modifiers: {} });
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
      const actions = ActionRegistry.withBuiltins();
      const from: Vec3 = [0, 0, 0];
      const r = await actions.run(
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
      await rt.send({ kind: "start", targets: [{ kind: "solid", id: "e2e-box", editable: true }], modifiers: {} });
      await rt.send({ kind: "confirm", modifiers: {} });
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
      await rt.send({ kind: "start", targets: [{ kind: "solid", id: "e2e-box", editable: true }], modifiers: {} });
      await rt.send({ kind: "confirm", modifiers: {} });
      await rt.send({ kind: "confirm", modifiers: {} });
      const from = rt.getSnapshot().context.from as Vec3;
      expect(from[0]).toBeCloseTo(1, 5);
      expect(from[1]).toBeCloseTo(1, 5);
      await rt.send({ kind: "pointer.down", point: [from[0] + 1, from[1], from[2]], modifiers: {} });
      expect(rt.getSnapshot().state).toBe("committed");
      expect(rt.getSnapshot().lastResponse?.ok).toBe(true);
    });
  });

  describe("@spatial/js-core action and interaction registries", () => {
    it("rejects executable action document fields", () => {
      const base = {
        schema: "spatial.action/v1",
        id: "x",
        version: "1.0.0",
        steps: [{ op: "return", data: { kind: "const", value: 1 } }],
      };
      expect(parseActionSpec({ ...base, run: "x" })).toBeNull();
      expect(parseActionSpec({ ...base, code: "x" })).toBeNull();
      expect(parseActionSpec({ ...base, function: "x" })).toBeNull();
      expect(parseActionSpec({ ...base, steps: [{ op: "eval", code: "x" }] })).toBeNull();
    });
    it("loads built-in actions from data-only JSON specs", () => {
      const specs = listBuiltinActionSpecs();
      const registry = ActionRegistry.withBuiltins();
      expect(specs.length).toBeGreaterThan(0);
      expect(specs.every((s) => registry.get(s.id)?.spec?.schema === "spatial.action/v1")).toBe(true);
      expect(specs.every((s) => registry.get(s.id) !== null)).toBe(true);
      expect(registry.get("command.finish")?.spec?.schema).toBe("spatial.action/v1");
      expect(registry.get("transform.move")?.run).toBeTypeOf("function");
    });
    it("ActionRegistry.withBuiltins registers known geometry actions", () => {
      const r = ActionRegistry.withBuiltins();
      const ids = new Set(r.list().map((d) => d.id));
      expect(ids.has("primitive.createBoxFromCorners")).toBe(true);
      expect(ids.has("box.aabbFromDiagonalCorners")).toBe(true);
      expect(ids.has("command.finish")).toBe(true);
      expect(ids.has("transform.scale1d")).toBe(true);
      expect(ids.has("transform.copy")).toBe(true);
      expect(ids.has("feature.offsetFaces")).toBe(true);
      expect(ids.has("selection.apply")).toBe(true);
      expect(ids.has("selection.selectAll")).toBe(true);
      expect(ids.has("selection.selectVertices")).toBe(true);
    });
    it("register replaces a built-in action id", () => {
      const r = ActionRegistry.withBuiltins();
      const before = r.get("measure.faceArea")?.label;
      r.register({
        id: "measure.faceArea",
        label: "override",
        run: () => ({ data: 99 }),
      });
      expect(r.get("measure.faceArea")?.label).toBe("override");
      expect(before).not.toBe("override");
    });
    it("InteractionRegistry.withBuiltins get matches buildBoxInteractionSpec", () => {
      const reg = InteractionRegistry.withBuiltins();
      expect(reg.get("primitive.box")).toEqual(buildBoxInteractionSpec());
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
      await ActionRegistry.withBuiltins().run("primitive.createBoxFrom3Points", { p0, p1, p2, __context: {}, __event: { kind: "x" } }, { kernel: k as unknown as SpatialKernel, preview: M, model });
      expect(k.lastInput).toEqual({ cornerA: [0, 0, 0], cornerB: [2, 3, 0], height: 3 });
    });
    it("command.addSelection applies selection modifiers", async () => {
      const actions = ActionRegistry.withBuiltins();
      const base = [{ kind: "wire", id: "w0", editable: true }] as const;
      const next = [{ kind: "wire", id: "w1", editable: true }] as const;
      const additive = await actions.run(
        "command.addSelection",
        { targets: next, __context: { targets: base }, __event: { kind: "selection.changed", modifiers: { shift: true } } },
        { kernel: M as unknown as SpatialKernel, preview: M, model: new Model() },
      );
      expect((additive.patch?.set as { targets?: readonly SelectionTarget[] }).targets).toEqual([...base, ...next]);
      const subtractive = await actions.run(
        "command.addSelection",
        {
          targets: next,
          __context: { targets: [...base, ...next] },
          __event: { kind: "selection.changed", modifiers: { ctrl: true } },
        },
        { kernel: M as unknown as SpatialKernel, preview: M, model: new Model() },
      );
      expect((subtractive.patch?.set as { targets?: readonly SelectionTarget[] }).targets).toEqual(base);
      const invertive = await actions.run(
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
      const actions = ActionRegistry.withBuiltins();
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const seed = [{ kind: "vertex", id: Object.keys(model.vertices)[0]!, editable: true }] as const;
      const all = await actions.run("selection.apply", { operation: "selectAll", seedTargets: [], __context: {} }, { kernel: M as unknown as SpatialKernel, preview: M, model });
      const allTargets = selectionTargetsFromActionResult(all);
      expect(allTargets.length).toBeGreaterThan(8);
      expect(allTargets.every((t) => t.kind !== "surface")).toBe(true);
      const cleared = await actions.run("selection.apply", { operation: "deselectAll", seedTargets: allTargets, __context: {} }, { kernel: M as unknown as SpatialKernel, preview: M, model });
      expect(selectionTargetsFromActionResult(cleared)).toEqual([]);
      const verts = await actions.run("selection.apply", { operation: "selectKinds", kinds: ["vertex"], seedTargets: [], __context: {} }, { kernel: M as unknown as SpatialKernel, preview: M, model });
      const vertTargets = selectionTargetsFromActionResult(verts);
      expect(vertTargets.length).toBe(8);
      expect(vertTargets.every((t) => t.kind === "vertex")).toBe(true);
      const inverted = await actions.run("selection.apply", { operation: "invert", seedTargets: vertTargets.slice(0, 1), __context: {} }, { kernel: M as unknown as SpatialKernel, preview: M, model });
      const invertedTargets = selectionTargetsFromActionResult(inverted);
      expect(invertedTargets.some((t) => t.kind === "vertex")).toBe(true);
      expect(invertedTargets.some((t) => t.kind === "face")).toBe(true);
      expect(invertedTargets.find((t) => t.id === vertTargets[0]!.id)).toBeUndefined();
    });
    it("selection.selectAll returns targets without model diff", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const result = await ActionRegistry.withBuiltins().run(
        "selection.selectAll",
        { seedTargets: [], __context: {}, __event: { kind: "commit" } },
        { kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M, model },
      );
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
      await ActionRegistry.withBuiltins().run(
        "selection.selectAll",
        { seedTargets: [], __context: {}, __event: { kind: "commit" } },
        { kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M, model },
      );
      expect(hist.entries()).toEqual([]);
    });
    it.each(listSelectionOperationInteractionDefs())("registers selection command action $id", (defn) => {
      expect(ActionRegistry.withBuiltins().get(defn.id)?.spec?.schema).toBe("spatial.action/v1");
    });
    it("selection.invert honors seed targets", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("e2e-box")));
      const seed = [{ kind: "solid", id: "e2e-box", editable: true }] as const;
      const result = await ActionRegistry.withBuiltins().run(
        "selection.invert",
        { seedTargets: seed, __context: {}, __event: { kind: "commit" } },
        { kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M, model },
      );
      const targets = selectionTargetsFromActionResult(result);
      expect(targets.some((t) => t.kind === "solid" && t.id === "e2e-box")).toBe(false);
      expect(targets.length).toBeGreaterThan(0);
    });
    it("ActionRegistry.run executes selection.apply headless", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const actions = ActionRegistry.withBuiltins();
      const result = await actions.run(
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
    it.each(listSelectionOperationInteractionDefs())("runSelectionApply matches runSelectionOperationInteraction for $id", async (defn) => {
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
      const actions = ActionRegistry.withBuiltins();
      const run = async (id: string, targets: readonly SelectionTarget[]) => {
        const result = await actions.run(id, { seedTargets: targets, __context: {}, __event: { kind: "commit" } }, { kernel, preview: M, model });
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
  describe("@spatial/js-core model diff", () => {
    it("applyModelDiff then inverse restores counts", () => {
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
      const d = M.meshFaceModelDiff(mesh, "x");
      const inv = applyModelDiff(g, d);
      expect(Object.keys(g.faces).length).toBe(1);
      applyModelDiff(g, inv);
      expect(Object.keys(g.faces).length).toBe(0);
    });

    it("boxModelDiff creates selectable boundary and volume records", () => {
      const g = new Model();
      applyModelDiff(g, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [2, 3, 0], height: 4 }, solidRef("box-solid")));
      expect(Object.keys(g.vertices).length).toBe(8);
      expect(Object.keys(g.edges).length).toBe(12);
      expect(Object.keys(g.wires).length).toBe(6);
      expect(Object.keys(g.faces).length).toBe(6);
      expect(Object.keys(g.shells).length).toBe(1);
      expect(Object.keys(g.solids)).toEqual(["box-solid"]);
    });
  });
  describe("@spatial/js-core selection filter", () => {
    it("selectionEventMatches rejects kinds outside accept", () => {
      const spec: SelectionSpec = { accept: ["face"], multiple: false };
      const ok: SelectionEvent = {
        kind: "selection.changed",
        targets: [{ kind: "face", id: "f1", editable: true }],
      };
      const bad: SelectionEvent = {
        kind: "selection.changed",
        targets: [{ kind: "surface", id: "s1", editable: false }],
      };
      expect(selectionEventMatches(spec, ok)).toBe(true);
      expect(selectionEventMatches(spec, bad)).toBe(false);
    });
  });
  describe("@spatial/js-core interaction box", () => {
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
  });

  describe("@spatial/js-core stateEngine option", () => {
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

  describe("@spatial/js-core measure distance", () => {
    it("measure.faceArea action adds face anchor geometry", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("m-area")));
      const fid = Object.keys(model.faces)[0]! as FaceRef;
      const r = await ActionRegistry.withBuiltins().run("measure.faceArea", { faceId: fid }, { model: model, kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M });
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

  describe("@spatial/js-core measure area", () => {
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

  describe("@spatial/js-core document history", () => {
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

  describe("@spatial/js-core measure distance history", () => {
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

  describe("@spatial/js-core interaction session undo redo", () => {
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

  describe("@spatial/js-core undo routing", () => {
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

  describe("@spatial/js-core box display committed", () => {
    it("keeps box-preview visible for committed state", () => {
      const spec = buildBoxInteractionSpec();
      const ctx: Record<string, unknown> = {
        origin: [0, 0, 0] as Vec3,
        corner: [2, 3, 0] as Vec3,
        height: 4,
      };
      const d = resolveDisplay(spec, "committed", ctx);
      const prev = d.items.find((i) => i.kind === "box-preview" && i.id === "preview-committed");
      expect(prev?.params?.cornerA).toEqual([0, 0, 0]);
      expect(prev?.params?.cornerB).toEqual([2, 3, 0]);
      expect(prev?.params?.height).toBe(4);
    });
  });

  describe("@spatial/js-core interaction e2e fixtures", () => {
    type InteractionE2EFixtureKind = "loom" | "routes" | "building" | "empty";

    const MOD: InteractionEvent["modifiers"] = {};

    const p = (x: number, y: number, z = 0): Vec3 => [x, y, z];

    const sel = (kind: ModelEntityKind, id: string, editable = true): SelectionTarget => ({
      kind,
      id,
      editable: kind === "surface" || kind === "part" ? false : editable,
    });

    const topoFromFixture = (kind: InteractionE2EFixtureKind): Model => {
      if (kind === "empty") return new Model();
      const raw = kind === "loom" ? geometryLoomFixtureJson : kind === "routes" ? geometryRoutesFixtureJson : smallBuildingModelFixtureJson;
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

    const assertSelectionCommandArchive = (defn: SelectionOperationInteractionDef, targets: readonly SelectionTarget[], model: Model, views?: null, activeViewId?: string | null): void => {
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
          const expected = collectGeometrySelectionTargets(model, kinds, views ?? null, activeViewId ?? null);
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
      readonly useView?: boolean;
      readonly spec?: InteractionSpec;
      readonly assert?: (ctx: {
        readonly snap: InteractionSnapshot;
        readonly model: Model;
        readonly before: ReturnType<typeof entityCounts>;
        readonly after: ReturnType<typeof entityCounts>;
        readonly views?: null;
        readonly activeViewId?: string | null;
      }) => void;
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
      const ids = listSpatialInteractions()
        .map((row) => row.id)
        .sort();
      expect(e2eCases.map((c) => c.id).sort()).toEqual(ids);
    });

    it.each(SELECTION_OPERATION_INTERACTION_DEFS)("$id selection action completes on seeded box", async (defn) => {
      const model = topoFromFixture("empty");
      seedBoxCell(model);
      if (defn.kinds?.includes("object")) {
        const solidId = Object.keys(model.solids)[0]!;
        model.objects["e2e-object"] = {
          id: "e2e-object" as ObjectRef,
          typologyId: "builtin.primitive.box",
          geometryRef: solidId,
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
      const model = topoFromFixture(row.fixture);
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
// #endregion 🧪Tests

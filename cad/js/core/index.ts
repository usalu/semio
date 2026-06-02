// #region 🧲Header
/** @emoji 🧭 `@cad/js/core` — model-definition runtime: `Model`, typology/action/interaction catalogs, `ActionRegistry`, `InteractionRegistry`, `StateEngine`, `SpatialKernel`. See `cad/AGENTS.md` and `cad/assets/modelDefinition`. */
// #endregion 🧲Header

// #region 📥ModelDefinitionRegistry
/** @emoji 📥 Registered model-definition asset modules (populated by `ModelDefinitionAssets` region). */
export interface ModelDefinitionAssetModules {
  readonly typologies: Readonly<Record<string, unknown>>;
  readonly actions: Readonly<Record<string, unknown>>;
  readonly interactions: Readonly<Record<string, unknown>>;
  readonly manifests: Readonly<Record<string, unknown>>;
  readonly extensions: Readonly<Record<string, unknown>>;
  readonly attributes: Readonly<Record<string, unknown>>;
  readonly propertyDefinitions: Readonly<Record<string, unknown>>;
  readonly properties: Readonly<Record<string, unknown>>;
  readonly transformations: Readonly<Record<string, unknown>>;
}

const emptyModelDefinitionAssetModules = (): ModelDefinitionAssetModules => ({
  typologies: {},
  actions: {},
  interactions: {},
  manifests: {},
  extensions: {},
  attributes: {},
  propertyDefinitions: {},
  properties: {},
  transformations: {},
});

let modelDefinitionAssetModules: ModelDefinitionAssetModules = emptyModelDefinitionAssetModules();

let modelDefinitionFolderIdMapCache: ReadonlyMap<string, string> | null = null;
let typologyOwnerByIdCache: ReadonlyMap<string, string> | null = null;
let actionOwnerByIdCache: ReadonlyMap<string, string> | null = null;
let interactionOwnerByIdCache: ReadonlyMap<string, string> | null = null;
let attributeOwnerByIdCache: ReadonlyMap<string, string> | null = null;
let propertyOwnerByIdCache: ReadonlyMap<string, string> | null = null;
let defaultModelDefinitionIdCache: string | null = null;

function resetModelDefinitionCaches(): void {
  modelDefinitionFolderIdMapCache = null;
  typologyOwnerByIdCache = null;
  actionOwnerByIdCache = null;
  interactionOwnerByIdCache = null;
  attributeOwnerByIdCache = null;
  propertyOwnerByIdCache = null;
  defaultModelDefinitionIdCache = null;
}

/** @emoji 📥 Replaces shipped model-definition catalogs (tests or host injection). */
export function registerModelDefinitionAssets(modules: ModelDefinitionAssetModules): void {
  modelDefinitionAssetModules = modules;
  resetModelDefinitionCaches();
}

function modelDefinitionTypologyCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionAssetModules.typologies);
}

function modelDefinitionActionCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionAssetModules.actions);
}

function modelDefinitionInteractionCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionAssetModules.interactions);
}

function modelDefinitionManifestCatalog(): readonly unknown[] {
  return [...Object.values(modelDefinitionAssetModules.manifests), ...Object.values(modelDefinitionAssetModules.extensions)];
}

function modelDefinitionAttributeCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionAssetModules.attributes);
}

function modelDefinitionPropertyCatalog(): readonly unknown[] {
  return [...Object.values(modelDefinitionAssetModules.propertyDefinitions), ...Object.values(modelDefinitionAssetModules.properties)];
}

function modelDefinitionTransformationModules(): Readonly<Record<string, unknown>> {
  return modelDefinitionAssetModules.transformations;
}
// #endregion 📥ModelDefinitionRegistry

// #region 📥ModelDefinitionAssets
const __modelDefinitionTypologyModules = import.meta.glob(
  ["../../assets/modelDefinition/**/typology.json", "../../assets/modelDefinition/**/typology/*.json"],
  { eager: true, import: "default" },
) as Record<string, unknown>;

const __modelDefinitionActionModules = import.meta.glob("../../assets/modelDefinition/**/action/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const __modelDefinitionInteractionModules = import.meta.glob("../../assets/modelDefinition/**/interaction/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const __modelDefinitionManifestModules = import.meta.glob("../../assets/modelDefinition/**/modelDefinition.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const __modelDefinitionAttributeModules = import.meta.glob("../../assets/modelDefinition/**/attributeDefinition/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const __modelDefinitionPropertyDefinitionModules = import.meta.glob(
  ["../../assets/modelDefinition/**/propertyDefinition/*.json", "../../assets/modelDefinition/**/propertyKind/*.json"],
  { eager: true, import: "default" },
) as Record<string, unknown>;

const __modelDefinitionPropertyModules = import.meta.glob("../../assets/modelDefinition/**/property/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const __modelDefinitionTransformationModules = import.meta.glob("../../assets/modelDefinition/**/transformation/**/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const __modelDefinitionExtensionManifestModules = import.meta.glob("../../assets/modelDefinition/**/extension.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

registerModelDefinitionAssets({
  typologies: __modelDefinitionTypologyModules,
  actions: __modelDefinitionActionModules,
  interactions: __modelDefinitionInteractionModules,
  manifests: __modelDefinitionManifestModules,
  extensions: __modelDefinitionExtensionManifestModules,
  attributes: __modelDefinitionAttributeModules,
  propertyDefinitions: __modelDefinitionPropertyDefinitionModules,
  properties: __modelDefinitionPropertyModules,
  transformations: __modelDefinitionTransformationModules,
});
// #endregion 📥ModelDefinitionAssets

// #region 🧮Vec
/** @emoji 📐 Column vector `[x,y,z]` used by spatial factories. */
export type Vec3 = readonly [number, number, number];
// #endregion 🧮Vec

// #region 🌀EdgeGeometry
/** @emoji 🌀 Edge curve geometry kinds (`line`, `arc`, `circle`, `ellipse`, `nurbs`). */
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
      readonly through?: boolean;
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

/** @emoji 🧭 Maps object picks to wire/edge primitives when `spec.accept` lists curve geometry kinds. */
export function expandSelectionTargetsForAccept(model: Model, spec: SelectionSpec, targets: readonly SelectionTarget[]): SelectionTarget[] {
  const accept = new Set(spec.accept);
  const out: SelectionTarget[] = [];
  const seen = new Set<string>();
  const push = (kind: ModelEntityKind, id: string): void => {
    const key = `${kind}:${id}`;
    if (seen.has(key)) return;
    seen.add(key);
    out.push({ kind, id, editable: true });
  };
  for (const target of targets) {
    if (accept.has(target.kind)) {
      push(target.kind, target.id);
      continue;
    }
    if (target.kind !== "object") continue;
    const row = model.objects[target.id as ObjectRef];
    if (!row) continue;
    const wireRef = row.primitives.wire;
    const edgeRef = row.primitives.edge;
    if (accept.has("wire") && wireRef) push("wire", String(wireRef));
    else if (accept.has("edge") && edgeRef) push("edge", String(edgeRef));
  }
  return out;
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
/** @emoji 🧭 Root object for segmented path reads (`context`, `event`, or action `params`). */
export type PathRoot = "context" | "event" | "params";

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

function pathRootRecord(root: PathRoot, env: ExprEnv): unknown {
  if (root === "context") return env.context;
  if (root === "params") return env.params ?? {};
  return env.event;
}

/** @emoji 🧭 Resolves a `PathTarget` against `ExprEnv`. */
export function readPathTarget(t: PathTarget, env: ExprEnv): unknown {
  return readPathSegments(pathRootRecord(t.root, env), t.segments);
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

  deleteField(id: string, key: string): void {
    const r = this.byId.get(id);
    if (!r || !Object.hasOwn(r, key)) return;
    delete r[key];
    if (Object.keys(r).length === 0) this.byId.delete(id);
    this.bumpRevision();
  }

  deleteEntity(id: string): void {
    if (this.byId.delete(id)) this.bumpRevision();
  }

  /** @emoji 🧭 Iterates `(entityId, fields)` pairs in stable id order. */
  entries(): Iterable<[string, Readonly<Record<string, unknown>>]> {
    return [...this.byId.entries()].sort(([a], [b]) => a.localeCompare(b));
  }

  /** @emoji 🧭 Serializes sidecar attribute rows for STEP / JSON. */
  toJSON(): readonly { readonly id: string; readonly fields: Readonly<Record<string, unknown>> }[] {
    return [...this.entries()].map(([id, fields]) => ({ id, fields }));
  }

  /** @emoji 🧭 Hydrates sidecar attributes from JSON rows. */
  static fromJSON(rows: readonly { readonly id: string; readonly fields: Readonly<Record<string, unknown>> }[]): AttributeStore {
    const store = new AttributeStore(() => {});
    for (const row of rows ?? []) store.byId.set(row.id, { ...row.fields });
    return store;
  }

  /** @emoji 🧭 Replaces all attribute rows; bumps parent revision when `bumpRevision` is true. */
  loadSnapshot(rows: readonly { readonly id: string; readonly fields: Readonly<Record<string, unknown>> }[], bumpRevision = true): void {
    this.byId.clear();
    for (const row of rows ?? []) this.byId.set(row.id, { ...row.fields });
    if (bumpRevision && rows.length > 0) this.bumpRevision();
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
  readonly activeModelDefinitionId?: string | null;
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
    activeModelDefinitionId: base.activeModelDefinitionId,
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
          activeModelDefinitionId: env.activeModelDefinitionId,
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
  | { readonly op: "action"; readonly action: string; readonly params?: Record<string, Expr> }
  | {
      readonly op: "interaction.call";
      readonly interaction: string;
      readonly inputs?: Record<string, Expr>;
      readonly outputs?: readonly InteractionOutputBinding[] | Record<string, unknown>;
    };

/** @emoji 📞 Maps host context paths from expressions evaluated against the child session context. */
export interface InteractionOutputBinding {
  readonly target: PathTarget;
  readonly value: Expr;
}

function interactionOutputBindings(
  outputs: readonly InteractionOutputBinding[] | Record<string, unknown> | undefined,
): readonly InteractionOutputBinding[] | undefined {
  if (!outputs) return undefined;
  if (Array.isArray(outputs)) return outputs;
  return Object.entries(outputs).map(([hostKey, childKey]) => ({
    target: { root: "context", segments: [{ kind: "field", name: hostKey }] },
    value: { kind: "path", root: "context", segments: [{ kind: "field", name: String(childKey) }] },
  }));
}

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
  /** @emoji 📞 `standalone` (default) for hosts; `callable` only via `interaction.call`. */
  readonly invocation?: InteractionInvocation;
  /** @emoji 🏷️ Typology object created when this interaction commits geometry. */
  readonly produces?: { readonly typology: string };
}

/** @emoji 📞 How hosts may start an interaction (`standalone` vs nested-only `callable`). */
export type InteractionInvocation = "standalone" | "callable";

/** @emoji 📏 One rubber-band state where REPL digits clamp distance along the cursor ray. */
export interface InteractionLengthEntrySpec {
  readonly state: string;
  readonly anchor: string;
  readonly field: string;
  /** @emoji ✅ Host commit on Enter/Space (`pointer.down` default, `confirm` for scalar-like steps). */
  readonly commit?: "pointer.down" | "confirm";
}

/** @emoji 🔢 One state where REPL digits set a scalar context field live (`set.height`, `set.radius`, …). */
export interface InteractionScalarEntrySpec {
  readonly state: string;
  readonly event: string;
  readonly field: string;
  /** @emoji ✅ Host commit on Enter/Space (defaults to `confirm`). */
  readonly commit?: "pointer.down" | "confirm";
  /** @emoji 📍 Context path to Vec3 for axis XY (Z from `axisFloor` when set). */
  readonly axisAnchor?: string;
  /** @emoji 📍 Context path to Vec3 whose Z is the axis floor (defaults to `axisAnchor`). */
  readonly axisFloor?: string;
  readonly axis?: readonly [number, number, number];
}

/** @emoji 🎮 Host + viewport hints for spatial picking (declared per interaction). */
export interface InteractionSpatialConfig {
  readonly spatialGroundPick?: boolean;
  readonly pickDisabledStates?: readonly string[];
  readonly groundPointerMoveStates?: readonly string[];
  readonly heightDragStates?: readonly string[];
  readonly verticalRodStates?: readonly string[];
  readonly heightConfirmState?: string | null;
  readonly lengthEntry?: readonly InteractionLengthEntrySpec[];
  readonly scalarEntry?: readonly InteractionScalarEntrySpec[];
}

/** @emoji 📞 Resolved invocation for an interaction document. */
export function interactionInvocation(spec: InteractionSpec): InteractionInvocation {
  if (spec.invocation === "callable" || spec.invocation === "standalone") return spec.invocation;
  return "standalone";
}

/** @emoji 📞 True when an interaction must not be started standalone by hosts. */
export function isCallableOnlyInteraction(spec: InteractionSpec): boolean {
  return interactionInvocation(spec) === "callable";
}

function guardNames(spec: InteractionSpec): Set<string> {
  return new Set((spec.guards ?? []).map((g) => g.name));
}

function findState(spec: InteractionSpec, name: string): StateDefSpec | undefined {
  return spec.machine.states.find((s) => s.name === name);
}

/** @emoji 🏁 True when `state` is marked `final` on the interaction machine. */
export function isFinalInteractionState(spec: InteractionSpec, state: string): boolean {
  return Boolean(findState(spec, state)?.final);
}

function listFinalInteractionStates(spec: InteractionSpec): string[] {
  return spec.machine.states.filter((s) => s.final).map((s) => s.name);
}

function normalizeInteractionCallEffectRaw(fx: Record<string, unknown>): boolean {
  if (fx.op !== "interaction.call" || typeof fx.interaction !== "string") return false;
  const outputs = fx.outputs;
  if (outputs === undefined) return true;
  if (Array.isArray(outputs)) return outputs.every((row) => row && typeof row === "object" && "target" in (row as object) && "value" in (row as object));
  if (outputs && typeof outputs === "object") {
    fx.outputs = Object.entries(outputs as Record<string, unknown>).map(([hostKey, childKey]) => ({
      target: { root: "context", segments: [{ kind: "field", name: hostKey }] },
      value: { kind: "path", root: "context", segments: [{ kind: "field", name: String(childKey) }] },
    }));
    return true;
  }
  return false;
}

function normalizeInteractionDocumentRaw(r: Record<string, unknown>): void {
  const spatial = r.interaction;
  if (spatial && typeof spatial === "object" && (spatial as Record<string, unknown>).callableOnly === true) {
    r.invocation = "callable";
    delete (spatial as Record<string, unknown>).callableOnly;
  }
  const machine = r.machine;
  if (!machine || typeof machine !== "object") return;
  const states = (machine as Record<string, unknown>).states;
  if (!Array.isArray(states)) return;
  for (const st of states) {
    if (!st || typeof st !== "object") continue;
    const on = (st as Record<string, unknown>).on;
    if (!Array.isArray(on)) continue;
    for (const h of on) {
      if (!h || typeof h !== "object") continue;
      const transitions = (h as Record<string, unknown>).transitions;
      if (!Array.isArray(transitions)) continue;
      for (const tr of transitions) {
        if (!tr || typeof tr !== "object") continue;
        const effects = (tr as Record<string, unknown>).effects;
        if (!Array.isArray(effects)) continue;
        for (const eff of effects) {
          if (!eff || typeof eff !== "object") continue;
          if (!normalizeInteractionCallEffectRaw(eff as Record<string, unknown>)) return;
        }
      }
    }
  }
}

/** @emoji 📞 Writes child session values onto host context using declarative output bindings. */
export function mergeInteractionCallOutputs(
  hostContext: Record<string, unknown>,
  childContext: Record<string, unknown>,
  outputs: readonly InteractionOutputBinding[] | Record<string, unknown> | undefined,
): void {
  const bindings = interactionOutputBindings(outputs);
  if (!bindings?.length) return;
  const childEnv: ExprEnv = { context: childContext, event: { kind: "interaction.return" } };
  const hostEnv: ExprEnv = { ...childEnv, context: hostContext };
  for (const row of bindings) {
    writePathTarget(row.target, hostEnv, evalExpr(row.value, childEnv));
  }
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
  if (r.invocation !== undefined && r.invocation !== "standalone" && r.invocation !== "callable") return null;
  normalizeInteractionDocumentRaw(r);
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
    if (effect.op === "interaction.call") continue;
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

  /** @emoji 🧱 Edge payload: two boundary vertices; optional `curve`. */
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

  /** @emoji 🌊 Face-support geometry (`plane`, `cylinder`, `cone`, `sphere`, `torus`, `nurbs`). */
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

/** @emoji 📦 Primitive refs owned by one object row. */
export type SpatialObjectPrimitives = Readonly<Record<string, string>>;

/** @emoji 📦 Object instance row in a model (`typology` + kernel `primitives`). */
export interface SpatialObjectRecord {
  readonly id: ObjectRef;
  readonly typology: TypologyRef;
  readonly primitives: SpatialObjectPrimitives;
  readonly attributes?: Readonly<Record<string, unknown>>;
}

/** @emoji 🗺️ Serializable model (`spatial.model/v1`). */
export interface ModelJson {
  readonly schema: "spatial.model/v1";
  readonly revision: number;
  readonly objects: readonly SpatialObjectRecord[];
  readonly geometry: KernelGeometryJson;
  readonly metadata?: readonly { readonly id: string; readonly fields: Readonly<Record<string, unknown>> }[];
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

function sortedPrimitiveEntries(primitives: Record<string, unknown>): readonly (readonly [string, string])[] {
  return Object.entries(primitives)
    .filter((entry): entry is [string, string] => typeof entry[0] === "string" && typeof entry[1] === "string" && entry[0].length > 0)
    .sort(([left], [right]) => left.localeCompare(right));
}

function normalizeSpatialObjectPrimitives(primitives: unknown): SpatialObjectPrimitives {
  return primitives && typeof primitives === "object" ? Object.fromEntries(sortedPrimitiveEntries(primitives as Record<string, unknown>)) : {};
}

function normalizeSpatialObjectRecord(row: SpatialObjectRecord | Record<string, unknown>): SpatialObjectRecord {
  return {
    id: String(row.id) as ObjectRef,
    typology: String(row.typology ?? "spatial.shape.object") as TypologyRef,
    primitives: Array.isArray(row.primitives) ? {} : normalizeSpatialObjectPrimitives(row.primitives),
    ...(row.attributes && typeof row.attributes === "object" ? { attributes: row.attributes as Readonly<Record<string, unknown>> } : {}),
  };
}

function readVec3FromUnknown(value: unknown): Vec3 | null {
  if (!Array.isArray(value) || value.length < 3) return null;
  const x = Number(value[0]);
  const y = Number(value[1]);
  const z = Number(value[2]);
  if (![x, y, z].every((n) => Number.isFinite(n))) return null;
  return [x, y, z];
}

/** @emoji 🧱 Promotes inline `objects[].primitives[]` rows into kernel geometry tables and slot refs. */
export function materializeInlineObjectPrimitives(model: Model, rawObjects: readonly unknown[] = []): void {
  let changed = false;
  const rawById = new Map<string, Record<string, unknown>>();
  for (const entry of rawObjects) {
    if (!entry || typeof entry !== "object") continue;
    const row = entry as Record<string, unknown>;
    if (typeof row.id === "string") rawById.set(row.id, row);
  }
  for (const [objectId, object] of Object.entries(model.objects)) {
    const raw = rawById.get(objectId)?.primitives;
    if (!Array.isArray(raw)) continue;
    const slots: Record<string, string> = {};
    for (const entry of raw) {
      if (!entry || typeof entry !== "object") continue;
      const row = entry as Record<string, unknown>;
      const kind = typeof row.kind === "string" ? row.kind : "";
      const id = typeof row.id === "string" ? row.id : "";
      if (!kind || !id) continue;
      const slot = typeof row.slot === "string" && row.slot.length > 0 ? row.slot : kind;
      slots[slot] = id;
      if (kind === "vertex") {
        const position = readVec3FromUnknown(row.position);
        if (position) {
          model.vertices[id as VertexRef] = { id: id as VertexRef, position };
          changed = true;
        }
        continue;
      }
      if (kind === "edge" && Array.isArray(row.vertexIds) && row.vertexIds.length >= 2) {
        model.edges[id as EdgeRef] = {
          id: id as EdgeRef,
          vertexIds: [String(row.vertexIds[0]), String(row.vertexIds[1])] as [VertexRef, VertexRef],
          ...(row.curve && typeof row.curve === "object" ? { curve: row.curve as EdgeCurve } : {}),
        };
        changed = true;
        continue;
      }
      if (kind === "wire" && Array.isArray(row.edgeIds)) {
        model.wires[id as WireRef] = { id: id as WireRef, edgeIds: row.edgeIds.map(String) as EdgeRef[] };
        changed = true;
        continue;
      }
      if (kind === "face" && Array.isArray(row.wireIds)) {
        model.faces[id as FaceRef] = {
          id: id as FaceRef,
          wireIds: row.wireIds.map(String) as WireRef[],
          ...(row.surface && typeof row.surface === "object" ? { surface: row.surface as FaceSurface } : {}),
        };
        changed = true;
        continue;
      }
      if (kind === "shell" && Array.isArray(row.faceIds)) {
        model.shells[id as ShellRef] = { id: id as ShellRef, faceIds: row.faceIds.map(String) as FaceRef[] };
        changed = true;
        continue;
      }
      if (kind === "solid") {
        model.solids[id as SolidRef] = {
          id: id as SolidRef,
          shellIds: Array.isArray(row.shellIds) ? (row.shellIds.map(String) as ShellRef[]) : [],
          ...(row.solid && typeof row.solid === "object" ? { solid: row.solid as SolidPrimitive } : {}),
        };
        changed = true;
      }
    }
    model.objects[objectId] = { ...object, primitives: normalizeSpatialObjectPrimitives(slots) };
    changed = true;
  }
  if (changed) model.bump();
}

export function objectPrimitiveEntries(object: SpatialObjectRecord): readonly (readonly [string, string])[] {
  return sortedPrimitiveEntries(object.primitives);
}

export function objectPrimitiveRefs(object: SpatialObjectRecord): readonly string[] {
  return objectPrimitiveEntries(object).map(([, primitiveRef]) => primitiveRef);
}

export function objectPrimaryPrimitiveRef(object: SpatialObjectRecord): string | null {
  return objectPrimitiveEntries(object)[0]?.[1] ?? null;
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
    const meta = this.metadata.toJSON();
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
      ...(meta.length > 0 ? { metadata: meta } : {}),
    };
  }

  /** @emoji 🧭 Hydrates from `ModelJson`. */
  static fromJSON(j: ModelJson): Model {
    const g = new Model();
    g.revision = j.revision;
    const rawObjects = j.objects ?? [];
    g.objects = recordsById(rawObjects.map((row) => normalizeSpatialObjectRecord(row as SpatialObjectRecord | Record<string, unknown>)));
    const geo = j.geometry ?? (j as unknown as KernelGeometryJson);
    g.anchors = recordsById(geo.anchors ?? []);
    g.vertices = recordsById(geo.vertices ?? []);
    g.edges = recordsById(geo.edges ?? []);
    g.wires = recordsById(geo.wires ?? []);
    g.faces = recordsById(geo.faces ?? []);
    g.shells = recordsById(geo.shells ?? []);
    g.solids = recordsById(geo.solids ?? []);
    if (j.metadata?.length) g.metadata.loadSnapshot(j.metadata, false);
    materializeInlineObjectPrimitives(g, rawObjects);
    return g;
  }

  bump(): void {
    this.revision += 1;
  }
}

/** @emoji 🗑️ Removes object rows by id; geometry primitives stay intact so linked topology remains valid. */
export function deleteObjectsFromModel(model: Model, objectIds: readonly string[]): readonly string[] {
  const removed: string[] = [];
  for (const id of objectIds) {
    if (!model.objects[id]) continue;
    delete model.objects[id];
    removed.push(id);
  }
  if (removed.length > 0) model.bump();
  return removed;
}

/** @emoji 🪪 Object ids from selection eligible for deletion (excludes geometry primitives). */
export function deletableObjectIdsFromSelection(selection: readonly SelectionTarget[]): readonly string[] {
  const ids: string[] = [];
  const seen = new Set<string>();
  for (const target of selection) {
    if (target.kind !== "object" || seen.has(target.id)) continue;
    seen.add(target.id);
    ids.push(target.id);
  }
  return ids;
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

/** @emoji #️⃣ Hashes canonical primitive payload bytes. */
export function hashPrimitivePayload(kind: string, payload: string): GeometryPrimitiveHash {
  return `${kind[0]}:${fnv1aHex(payload)}` as GeometryPrimitiveHash;
}

/** @emoji #️⃣ Hashes a vertex position (`cad/AGENTS.md` primitive hashing). */
export function hashVertexPosition(position: Vec3): GeometryPrimitiveHash {
  const q = position.map((c) => quantizeCoord(c)) as Vec3;
  return hashPrimitivePayload("vertex", `${q[0]},${q[1]},${q[2]}`);
}

/** @emoji #️⃣ Hashes an edge by vertex ids and optional curve. */
export function hashEdgeRecord(edge: EdgeRecord, vertices: Record<string, VertexRecord>): GeometryPrimitiveHash {
  const positions = edge.vertexIds.map((vid) => {
    const p = vertices[vid]?.position ?? ([0, 0, 0] as Vec3);
    return p.map((c) => quantizeCoord(c)).join(",");
  });
  const curveKey = edge.curve ? JSON.stringify(edge.curve) : "line";
  return hashPrimitivePayload("edge", `${edge.vertexIds.join(",")}|${curveKey}|${positions.join(";")}`);
}

/** @emoji #️⃣ Hashes a wire by sorted edge ids. */
export function hashWireRecord(wire: WireRecord): GeometryPrimitiveHash {
  return hashPrimitivePayload("wire", [...wire.edgeIds].sort().join(","));
}

/** @emoji #️⃣ Hashes a face by sorted wire ids and surface kind. */
export function hashFaceRecord(face: FaceRecord): GeometryPrimitiveHash {
  const surfaceKey = face.surface ? JSON.stringify(face.surface) : "none";
  return hashPrimitivePayload("face", `${[...face.wireIds].sort().join(",")}|${surfaceKey}`);
}

/** @emoji #️⃣ Hashes a shell by sorted face ids. */
export function hashShellRecord(shell: ShellRecord): GeometryPrimitiveHash {
  return hashPrimitivePayload("shell", [...shell.faceIds].sort().join(","));
}

/** @emoji #️⃣ Hashes a solid by sorted shell ids and solid primitive. */
export function hashSolidRecord(solid: SolidRecord): GeometryPrimitiveHash {
  const primitiveKey = solid.solid ? JSON.stringify(solid.solid) : "none";
  return hashPrimitivePayload("solid", `${[...solid.shellIds].sort().join(",")}|${primitiveKey}`);
}

/** @emoji #️⃣ Hashes an anchor position and attachment. */
export function hashAnchorRecord(anchor: AnchorRecord): GeometryPrimitiveHash {
  return hashPrimitivePayload("anchor", `${anchor.position.map((c) => quantizeCoord(c)).join(",")}|${JSON.stringify(anchor.attachment)}`);
}

/** @emoji #️⃣ Per-primitive hashes for one model (`ModelSpace` geometry fingerprint). */
export type ModelPrimitiveHashes = Readonly<Partial<Record<TypologyPrimitiveKind, Readonly<Record<string, GeometryPrimitiveHash>>>>>;

/** @emoji #️⃣ Maps primitive tables on `model` to content hashes (every vertex and primitive). */
export function hashModelPrimitives(model: Model): ModelPrimitiveHashes {
  const out: Partial<Record<TypologyPrimitiveKind, Record<string, GeometryPrimitiveHash>>> = {};
  const put = (kind: TypologyPrimitiveKind, id: string, hash: GeometryPrimitiveHash): void => {
    out[kind] ??= {};
    out[kind]![id] = hash;
  };
  for (const [id, row] of Object.entries(model.anchors)) put("anchor", id, hashAnchorRecord(row));
  for (const [id, row] of Object.entries(model.vertices)) put("vertex", id, hashVertexPosition(row.position));
  for (const [id, row] of Object.entries(model.edges)) put("edge", id, hashEdgeRecord(row, model.vertices));
  for (const [id, row] of Object.entries(model.wires)) put("wire", id, hashWireRecord(row));
  for (const [id, row] of Object.entries(model.faces)) put("face", id, hashFaceRecord(row));
  for (const [id, row] of Object.entries(model.shells)) put("shell", id, hashShellRecord(row));
  for (const [id, row] of Object.entries(model.solids)) put("solid", id, hashSolidRecord(row));
  return out;
}

/** @emoji #️⃣ Maps every model vertex id to its position hash. */
export function hashModelVertices(model: Model): Readonly<Record<string, GeometryPrimitiveHash>> {
  return hashModelPrimitives(model).vertex ?? {};
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

  /** @emoji #️⃣ Full primitive hashes keyed by linked model id. */
  geometryHashesByModel(): Readonly<Record<string, ModelPrimitiveHashes>> {
    const out: Record<string, ModelPrimitiveHashes> = {};
    for (const [modelId, model] of Object.entries(this.models)) out[modelId] = hashModelPrimitives(model);
    return out;
  }

  /** @emoji 🔄 Transfers a transformation from a linked source model into a new linked target model. */
  transfer(linkedSourceId: string, linkedTargetId: string, spec: TransformationSpec, preview: SpatialPreviewKernel): Model {
    const source = this.models[linkedSourceId];
    if (!source) throw new Error(`ModelSpace: unknown source model ${linkedSourceId}`);
    const target = applyTransformation(spec, source, preview);
    this.link(linkedTargetId, target);
    return target;
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

// #region 🪜StepRoundtrip
const STEP_AP242_SCHEMA = "AP242_MANAGED_MODEL_BASED_3D_ENGINEERING_MIM_LF";

/** @emoji 🪜 Escapes a string for STEP string literals. */
export function stepEscape(value: string): string {
  return `'${value.replace(/'/g, "''")}'`;
}

/** @emoji 🪜 Formats a number for STEP (rejects non-finite). */
export function stepNumber(value: number): string {
  if (!Number.isFinite(value)) throw new Error(`stepNumber: non-finite ${value}`);
  const s = Object.is(value, -0) ? "0." : String(value);
  return s.includes(".") || s.includes("e") || s.includes("E") ? s : `${s}.`;
}

/** @emoji 🪜 Parsed STEP entity map (`#id` → assignment body after `=`). */
export type StepEntityMap = ReadonlyMap<number, string>;

/** @emoji 🪜 Incremental AP242 entity writer with stable `#` numbering. */
export class StepEntityWriter {
  private nextId = 10;
  private readonly lines: string[] = [];

  /** @emoji 🪜 Allocates the next entity id. */
  alloc(): number {
    const id = this.nextId;
    this.nextId += 1;
    return id;
  }

  /** @emoji 🪜 Emits `#id = body;` (body must not include leading `#`). */
  emit(id: number, body: string): number {
    const trimmed = body.trim().replace(/;\s*$/, "");
    this.lines.push(`#${id} = ${trimmed};`);
    return id;
  }

  /** @emoji 🪜 Emits with a freshly allocated id. */
  emitNew(body: string): number {
    return this.emit(this.alloc(), body);
  }

  /** @emoji 🪜 DATA section entity lines in emission order. */
  entityLines(): readonly string[] {
    return this.lines;
  }

  /** @emoji 🪜 Resets the writer for another file. */
  reset(): void {
    this.nextId = 10;
    this.lines.length = 0;
  }
}

/** @emoji 🪜 Parses STEP `DATA` entities into an id → body map. */
export function parseStepEntityMap(stepText: string): StepEntityMap {
  const out = new Map<number, string>();
  const data = stepText.match(/DATA;\s*([\s\S]*?)ENDSEC;/i)?.[1] ?? stepText;
  const re = /^#(\d+)\s*=\s*(.+?);\s*$/gm;
  for (const match of data.matchAll(re)) {
    out.set(Number(match[1]), match[2]!.trim());
  }
  return out;
}

/** @emoji 🪜 Extracts quoted STEP string literal contents (first argument). */
export function stepParseFirstString(entityBody: string): string | null {
  const m = entityBody.match(/'((?:''|[^'])*)'/);
  if (!m) return null;
  return m[1]!.replace(/''/g, "'");
}

/** @emoji 🪜 Extracts the value string from `DESCRIPTIVE_REPRESENTATION_ITEM('Value', 'payload')`. */
export function stepParseDescriptivePayload(entityBody: string): string | null {
  const matches = [...entityBody.matchAll(/'((?:''|[^'])*)'/g)];
  if (matches.length >= 2) return matches[1]![1]!.replace(/''/g, "'");
  return stepParseFirstString(entityBody);
}

/** @emoji 🪜 Reads `spatial.*` UDA JSON payloads keyed by property name from STEP entities. */
export function parseSpatialUdaPayloads(entities: StepEntityMap): Readonly<Record<string, string>> {
  const out: Record<string, string> = {};
  const propNames = new Map<number, string>();
  for (const [id, body] of entities) {
    if (!body.startsWith("PROPERTY_DEFINITION(")) continue;
    const name = stepParseFirstString(body);
    if (name?.startsWith("spatial.")) propNames.set(id, name);
  }
  for (const body of entities.values()) {
    if (!body.startsWith("PROPERTY_DEFINITION_REPRESENTATION(")) continue;
    const refs = [...body.matchAll(/#(\d+)/g)].map((m) => Number(m[1]));
    if (refs.length < 2) continue;
    const propName = propNames.get(refs[0]!);
    if (!propName) continue;
    const reprBody = entities.get(refs[1]!);
    const itemRef = reprBody?.match(/#(\d+)/g)?.map((m) => Number(m.slice(1)))?.[0];
    const itemBody = itemRef !== undefined ? entities.get(itemRef) : undefined;
    const payload = itemBody ? stepParseDescriptivePayload(itemBody) : reprBody ? stepParseDescriptivePayload(reprBody) : null;
    if (payload) out[propName] = payload;
  }
  return out;
}

/** @emoji 🪜 Renumber brepjs STEP `DATA` lines into a shared writer (returns old→new id map). */
export function mergeStepDataChunk(chunk: string, writer: StepEntityWriter): ReadonlyMap<number, number> {
  const idMap = new Map<number, number>();
  const data = chunk.match(/DATA;\s*([\s\S]*?)ENDSEC;/i)?.[1] ?? "";
  const bodies: { readonly oldId: number; readonly body: string }[] = [];
  const re = /^#(\d+)\s*=\s*(.+?);\s*$/gm;
  for (const match of data.matchAll(re)) {
    const oldId = Number(match[1]);
    bodies.push({ oldId, body: match[2]!.trim() });
    idMap.set(oldId, writer.alloc());
  }
  const rewriteRefs = (body: string): string =>
    body.replace(/#(\d+)\b/g, (_, digits: string) => {
      const mapped = idMap.get(Number(digits));
      return mapped !== undefined ? `#${mapped}` : `#${digits}`;
    });
  for (const row of bodies) {
    const newId = idMap.get(row.oldId)!;
    writer.emit(newId, rewriteRefs(row.body));
  }
  return idMap;
}

/** @emoji 🪜 Builds AP242 STEP header for spatial six-pillar exports. */
export function stepSpatialFileHeader(fileName: string, timestampIso: string): string {
  const ts = stepEscape(timestampIso);
  const name = stepEscape(fileName);
  return [
    "ISO-10303-21;",
    "HEADER;",
    "FILE_DESCRIPTION(('Pure Spatial State Export'), '2;1');",
    `FILE_NAME(${name}, ${ts}, ('Spatial'), ('Spatial'), 'spatial-kernel', 'spatial', '');`,
    `FILE_SCHEMA(('${STEP_AP242_SCHEMA}'));`,
    "ENDSEC;",
  ].join("\n");
}

/** @emoji 🪜 Assembles header + DATA + ENDSEC footer. */
export function assembleStepFile(header: string, writer: StepEntityWriter): string {
  return `${header}\nDATA;\n${writer.entityLines().join("\n")}\nENDSEC;\nEND-ISO-10303-21;\n`;
}

/** @emoji 🪜 Property-definition + descriptive value UDA pair on a STEP definition context. */
export function emitSpatialUdaProperty(
  writer: StepEntityWriter,
  contextId: number,
  propertyName: string,
  payloadJson: string,
  role: "Authored Attribute" | "Derived Property" | "System_Generated",
): void {
  const propId = writer.emitNew(`PROPERTY_DEFINITION(${stepEscape(propertyName)}, ${stepEscape(role)}, #${contextId})`);
  const itemId = writer.emitNew(`DESCRIPTIVE_REPRESENTATION_ITEM('Value', ${stepEscape(payloadJson)})`);
  const reprId = writer.emitNew(`REPRESENTATION('Spatial_Uda', (#${itemId}), #10)`);
  writer.emitNew(`PROPERTY_DEFINITION_REPRESENTATION(#${propId}, #${reprId})`);
}

/** @emoji 🪜 Restores `AttributeStore` fields from parsed `spatial.attribute.*` UDA keys. */
export function applySpatialAttributesFromUda(model: Model, uda: Readonly<Record<string, string>>): void {
  for (const [key, json] of Object.entries(uda)) {
    if (!key.startsWith("spatial.attribute.")) continue;
    const entityId = key.slice("spatial.attribute.".length);
    const fields = JSON.parse(json) as Record<string, unknown>;
    for (const [field, value] of Object.entries(fields)) model.metadata.setField(entityId, field, value);
  }
}

/** @emoji 🪜 Restores models from `spatial.model` / `spatial.geometry` UDA payloads. */
export function modelSpaceFromSpatialUda(uda: Readonly<Record<string, string>>, modelIds: readonly string[]): ModelSpace {
  const space = new ModelSpace();
  const ms = JSON.parse(uda["spatial.modelspace"] ?? "{}") as { revision?: number };
  space.revision = ms.revision ?? 0;
  for (const modelId of modelIds) {
    const modelJson = JSON.parse(uda[`spatial.model.${modelId}`] ?? "{}") as ModelJson;
    const geometryJson = uda[`spatial.geometry.${modelId}`];
    const full: ModelJson = {
      ...modelJson,
      geometry: geometryJson ? (JSON.parse(geometryJson) as KernelGeometryJson) : modelJson.geometry,
    };
    space.models[modelId] = Model.fromJSON(full);
  }
  return space;
}
// #endregion 🪜StepRoundtrip

/** @emoji 🧭 Reads `name` from metadata, geometry records, or model objects. */
export function readModelEntityProperty(
  model: Model,
  meta: AttributeStore | undefined,
  kind: ModelEntityKind,
  id: string,
  name: string,
  opts?: {
    readonly activeModelDefinitionId?: string | null;
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
      if (name === "typology") return hit.typology;
      if (name === "primitives") return hit.primitives;
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

/** @emoji 🧱 Primitive kind allowed on typology objects (`cad/AGENTS.md`). */
export type TypologyPrimitiveKind = "anchor" | "vertex" | "edge" | "wire" | "face" | "shell" | "solid";

/** @emoji 🏷️ Parsed model-definition manifest (`spatial.modelDefinition/v1` on disk). */
export interface ModelDefinitionManifest {
  readonly schema: "spatial.modelDefinition/v1";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly kinds: readonly string[];
  readonly default?: boolean;
  readonly kernelTypologies?: Readonly<Partial<Record<TypologyPrimitiveKind, string>>>;
}

/** @emoji 🧭 Default geometry-edit model definition id (manifest `default: true`). */
export function defaultModelDefinitionId(): string {
  if (defaultModelDefinitionIdCache) return defaultModelDefinitionIdCache;
  const manifests = listModelDefinitionManifests();
  const row = manifests.find((m) => m.default) ?? manifests[0];
  defaultModelDefinitionIdCache = row?.id ?? "";
  return defaultModelDefinitionIdCache;
}

/** @emoji 🧭 True when the active definition is geometry edit (`ModelDefinition`) rather than typology objects. */
export function isShapeModelDefinition(modelDefinitionId: string | null | undefined): boolean {
  if (modelDefinitionId == null) return true;
  return kernelTypologyIds(modelDefinitionId) !== null;
}

/** @emoji 🪪 Kernel typology ids per primitive kind on a model-definition manifest. */
export function kernelTypologyIds(modelDefinitionId: string): Readonly<Partial<Record<TypologyPrimitiveKind, string>>> | null {
  const manifest = listModelDefinitionManifests().find((row) => row.id === modelDefinitionId);
  const map = manifest?.kernelTypologies;
  if (!map || Object.keys(map).length === 0) return null;
  return map;
}

/** @emoji 🧾 Parses a model-definition manifest JSON or returns `null`. */
export function parseModelDefinitionManifest(raw: unknown): ModelDefinitionManifest | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.modelDefinition/v1" && r.schema !== "spatial.extension/v1") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string" || typeof r.label !== "string") return null;
  if (!Array.isArray(r.kinds) || r.kinds.length === 0) return null;
  const kernelTypologies = parseKernelTypologies(r.kernelTypologies);
  return {
    schema: "spatial.modelDefinition/v1",
    id: r.id,
    version: r.version,
    label: r.label,
    description: typeof r.description === "string" ? r.description : undefined,
    kinds: r.kinds as string[],
    ...(r.default === true ? { default: true } : {}),
    ...(kernelTypologies ? { kernelTypologies } : {}),
  };
}

function parseKernelTypologies(raw: unknown): Readonly<Partial<Record<TypologyPrimitiveKind, string>>> | undefined {
  if (!raw || typeof raw !== "object" || Array.isArray(raw)) return undefined;
  const allowed = new Set<TypologyPrimitiveKind>(["anchor", "vertex", "edge", "wire", "face", "shell", "solid"]);
  const out: Partial<Record<TypologyPrimitiveKind, string>> = {};
  for (const [key, value] of Object.entries(raw as Record<string, unknown>)) {
    if (!allowed.has(key as TypologyPrimitiveKind) || typeof value !== "string" || value.length === 0) continue;
    out[key as TypologyPrimitiveKind] = value;
  }
  return Object.keys(out).length > 0 ? out : undefined;
}

/** @emoji 📚 Lists model-definition manifests under spatial/assets/modelDefinition. */
export function listModelDefinitionManifests(): readonly ModelDefinitionManifest[] {
  return modelDefinitionManifestCatalog()
    .map((raw) => parseModelDefinitionManifest(raw))
    .filter((m): m is ModelDefinitionManifest => m !== null);
}

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
export function inferTypologyPrimitiveKinds(typology: string): readonly TypologyPrimitiveKind[] {
  const id = typology.toLowerCase();
  if (id.includes(".selection.") || id.includes(".command.")) return [];
  if (id.includes(".measure.") && id.includes("volume")) return [];
  if (id.includes(".entity.") || id.includes("create-anchor")) return ["anchor"];
  if (id.includes(".measure.")) return ["anchor"];
  if (id.includes(".curve.")) return ["edge", "wire"];
  if (id.includes(".surface.")) return ["face"];
  if (id.includes(".primitive.") || id.includes(".solid.")) return ["solid"];
  if (id.includes(".feature.extrude")) return ["solid"];
  if (id.includes(".feature.offset")) return ["face", "solid"];
  if (id.includes(".transform.") || id.includes(".edit.")) return ["vertex", "edge", "wire", "face", "solid"];
  return ["solid"];
}

function parseTypologyPrimitiveKinds(raw: unknown, typology: string): readonly TypologyPrimitiveKind[] {
  if (!Array.isArray(raw) || raw.length === 0) return inferTypologyPrimitiveKinds(typology);
  const allowed = new Set<TypologyPrimitiveKind>(["anchor", "vertex", "edge", "wire", "face", "shell", "solid"]);
  const kinds: TypologyPrimitiveKind[] = [];
  for (const entry of raw) {
    if (typeof entry !== "string") continue;
    const k = entry as TypologyPrimitiveKind;
    if (allowed.has(k)) kinds.push(k);
  }
  return kinds.length ? kinds : inferTypologyPrimitiveKinds(typology);
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

/** @emoji 📚 Lists typologies from shipped spatial/assets/modelDefinition assets. */
export function listModelDefinitionTypologies(): readonly TypologySpec[] {
  return shippedTypologyCatalog();
}

/** @emoji 📚 Loads a model-definition typology by stable `id`. */
export function loadTypology(typology: string): TypologySpec | null {
  return shippedTypologyCatalog().find((t) => t.id === typology) ?? null;
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

/** @emoji 🧭 Resolves the primitive kind referenced by one primitive ref. */
export function resolvePrimitiveRefKind(model: Model, primitiveRef: string): TypologyPrimitiveKind | null {
  if (model.anchors[primitiveRef]) return "anchor";
  if (model.vertices[primitiveRef]) return "vertex";
  if (model.edges[primitiveRef]) return "edge";
  if (model.wires[primitiveRef]) return "wire";
  if (model.faces[primitiveRef]) return "face";
  if (model.shells[primitiveRef]) return "shell";
  if (model.solids[primitiveRef]) return "solid";
  return null;
}

/** @emoji 🌳 Nested primitive node under an object primitive (`solid` → `shell` → `face` → `wire` → `edge` → `vertex`). */
export interface ModelPrimitiveHierarchyNode {
  readonly kind: TypologyPrimitiveKind;
  readonly id: string;
  readonly children: readonly ModelPrimitiveHierarchyNode[];
}

function sortedPrimitiveChildIds(ids: readonly string[]): string[] {
  return [...ids].sort((a, b) => a.localeCompare(b));
}

function buildModelPrimitiveHierarchyNode(model: Model, kind: TypologyPrimitiveKind, id: string): ModelPrimitiveHierarchyNode | null {
  const children: ModelPrimitiveHierarchyNode[] = [];
  switch (kind) {
    case "solid": {
      const solid = model.solids[id];
      if (!solid) return null;
      for (const shellId of sortedPrimitiveChildIds(solid.shellIds)) {
        const child = buildModelPrimitiveHierarchyNode(model, "shell", shellId);
        if (child) children.push(child);
      }
      break;
    }
    case "shell": {
      const shell = model.shells[id];
      if (!shell) return null;
      for (const faceId of sortedPrimitiveChildIds(shell.faceIds)) {
        const child = buildModelPrimitiveHierarchyNode(model, "face", faceId);
        if (child) children.push(child);
      }
      break;
    }
    case "face": {
      const face = model.faces[id];
      if (!face) return null;
      for (const wireId of sortedPrimitiveChildIds(face.wireIds)) {
        const child = buildModelPrimitiveHierarchyNode(model, "wire", wireId);
        if (child) children.push(child);
      }
      break;
    }
    case "wire": {
      const wire = model.wires[id];
      if (!wire) return null;
      for (const edgeId of sortedPrimitiveChildIds(wire.edgeIds)) {
        const child = buildModelPrimitiveHierarchyNode(model, "edge", edgeId);
        if (child) children.push(child);
      }
      break;
    }
    case "edge": {
      const edge = model.edges[id];
      if (!edge) return null;
      for (const vertexId of sortedPrimitiveChildIds(edge.vertexIds)) {
        const child = buildModelPrimitiveHierarchyNode(model, "vertex", vertexId);
        if (child) children.push(child);
      }
      break;
    }
    case "vertex":
      if (!model.vertices[id]) return null;
      break;
    case "anchor":
      if (!model.anchors[id]) return null;
      break;
  }
  return { kind, id, children };
}

/** @emoji 🌳 Builds nested primitive hierarchy under one object primitive ref in `model`. */
export function buildModelPrimitiveHierarchy(model: Model, primitiveRef: string): ModelPrimitiveHierarchyNode | null {
  const kind = resolvePrimitiveRefKind(model, primitiveRef);
  if (!kind) return null;
  return buildModelPrimitiveHierarchyNode(model, kind, primitiveRef);
}

/** @emoji ✅ Whether `typology` allows objects whose geometry resolves to `primitiveKind`. */
export function typologyAllowsPrimitiveKind(typology: TypologySpec, primitiveKind: TypologyPrimitiveKind): boolean {
  return typology.primitiveKinds.includes(primitiveKind);
}

/** @emoji ✅ Whether `object` on `model` satisfies its typology `primitiveKinds`. */
export function objectMatchesTypologyPrimitives(model: Model, object: SpatialObjectRecord): boolean {
  const typology = loadTypology(object.typology);
  if (!typology || typology.primitiveKinds.length === 0) return false;
  const primitiveKinds = objectPrimitiveRefs(object)
    .map((primitiveRef) => resolvePrimitiveRefKind(model, primitiveRef))
    .filter((kind): kind is TypologyPrimitiveKind => kind !== null);
  return primitiveKinds.length > 0 && primitiveKinds.every((kind) => typologyAllowsPrimitiveKind(typology, kind));
}

/** @emoji 🧭 Typology → entity kind map for one model definition (`ModelDefinition` includes kernel typology ids; AEC typologies map to `object`). */
export function buildTypologyToEntityKindMapForModelDefinition(modelDefinitionId: string): Readonly<Record<string, ModelEntityKind>> {
  const out: Record<string, ModelEntityKind> = {};
  const kernelTypologies = kernelTypologyIds(modelDefinitionId);
  if (kernelTypologies) {
    for (const [kind, id] of Object.entries(kernelTypologies)) {
      if (typeof id === "string" && id.length > 0) out[id] = kind as ModelEntityKind;
    }
    for (const spec of listTypologiesForModelDefinition(modelDefinitionId)) {
      if (spec.primitiveKinds.length !== 1) continue;
      const kind = spec.primitiveKinds[0]!;
      if (kind === "anchor" && !spec.id.includes("entity") && !spec.id.includes("measure")) continue;
      out[spec.id] = kind;
    }
    return out;
  }
  for (const spec of listTypologiesForModelDefinition(modelDefinitionId)) out[spec.id] = "object";
  return out;
}

/** @emoji ✅ Whether a property definition applies to `object` on `model`. */
export function propertyDefinitionAppliesToObject(defn: PropertyDefinitionSpec, object: SpatialObjectRecord): boolean {
  const typologies = defn.sources?.typologies;
  if (Array.isArray(typologies) && typologies.length > 0) return typologies.includes(object.typology);
  return true;
}

/** @emoji 📐 Derives property output for one model object from a property definition. */
export async function derivePropertyValue(
  defn: PropertyDefinitionSpec,
  ctx: { readonly model: Model; readonly kernel: SpatialKernel; readonly object: SpatialObjectRecord },
): Promise<Record<string, unknown>> {
  if (!propertyDefinitionAppliesToObject(defn, ctx.object)) return {};
  const primaryPrimitiveRef = objectPrimaryPrimitiveRef(ctx.object);
  if (defn.id === "spatial.shape.volume" || defn.id === "energy.heatedvolume") {
    const kind = primaryPrimitiveRef ? resolvePrimitiveRefKind(ctx.model, primaryPrimitiveRef) : null;
    if (kind !== "solid") return defn.id === "energy.heatedvolume" ? { heatedvolume: 0 } : { volume: 0 };
    await ctx.kernel.syncSolidsFromModel(ctx.model);
    const amount = await ctx.kernel.solidVolume(primaryPrimitiveRef as SolidRef);
    return defn.id === "energy.heatedvolume" ? { heatedvolume: amount } : { volume: amount };
  }
  const output = defn.output ?? {};
  return { ...output };
}

/** @emoji 📚 Property definitions for one model definition that apply to `object` on `model`. */
export function listApplicablePropertyDefinitionsForModelDefinition(
  modelDefinitionId: string,
  model: Model,
  object: SpatialObjectRecord,
): readonly PropertyDefinitionSpec[] {
  const scoped = new Set(listPropertyDefinitionsForModelDefinition(modelDefinitionId).map((row) => row.id));
  return shippedPropertyDefinitionCatalog().filter((defn) => scoped.has(defn.id) && propertyDefinitionAppliesToObject(defn, object));
}

/** @emoji 🧭 Throws when `actionId` is outside the active model definition catalog. */
export function assertActionAvailableInModelDefinition(actionId: string, activeModelDefinitionId?: string | null): void {
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  if (!actionAvailableInModelDefinition(actionId, mdId)) {
    throw new Error(`action ${actionId} is not available in model definition ${mdId}`);
  }
}

/** @emoji 🧱 Primitive entity kinds selectable on factory geometry (excludes typology `object` rows). */
export const PRIMITIVE_MODEL_ENTITY_KINDS: readonly ModelEntityKind[] = ["anchor", "vertex", "edge", "wire", "face", "shell", "solid"];

/** @emoji ✅ True when `defn` applies to a model entity kind under the active model definition. */
export function attributeDefinitionAppliesToEntity(defn: AttributeDefinitionSpec, entityKind: ModelEntityKind): boolean {
  if (!defn.targets.includes(entityKind)) return false;
  const selector = defn.geometrySelector?.kinds;
  if (selector && selector.length > 0 && !selector.includes(entityKind)) return false;
  return true;
}

/** @emoji 📚 Attribute definitions for one model definition and entity kind. */
export function listAttributeDefinitionsForModelDefinitionEntity(
  modelDefinitionId: string,
  entityKind: ModelEntityKind,
): readonly AttributeDefinitionSpec[] {
  return listAttributeDefinitionsForModelDefinition(modelDefinitionId).filter((defn) => attributeDefinitionAppliesToEntity(defn, entityKind));
}

/** @emoji 🧲 True when the active model definition exposes factory-geometry pick targets (all definitions). */
export function modelDefinitionUsesGeometryPicking(_modelDefinitionId: string): boolean {
  return true;
}

/** @emoji 📋 String/number/boolean/record options from an attribute value schema. */
export function attributeDefinitionValueOptions(defn: AttributeDefinitionSpec): readonly string[] | null {
  const schema = defn.value;
  if (!schema || typeof schema !== "object") return null;
  const row = schema as Record<string, unknown>;
  if (row.kind === "string" && Array.isArray(row.options)) {
    return row.options.filter((option): option is string => typeof option === "string");
  }
  if (row.kind === "oneOf" && Array.isArray(row.variants)) {
    for (const variant of row.variants) {
      if (!variant || typeof variant !== "object") continue;
      const v = variant as Record<string, unknown>;
      if (v.kind === "string" && Array.isArray(v.options)) {
        return v.options.filter((option): option is string => typeof option === "string");
      }
    }
  }
  return null;
}

/** @emoji 🧾 Value editor kind inferred from an attribute definition schema. */
export function attributeDefinitionEditorKind(defn: AttributeDefinitionSpec): "string" | "enum" | "number" | "boolean" | "text" {
  if (attributeDefinitionValueOptions(defn)) return "enum";
  const schema = defn.value;
  if (!schema || typeof schema !== "object") return "text";
  const row = schema as Record<string, unknown>;
  if (row.kind === "number") return "number";
  if (row.kind === "boolean") return "boolean";
  if (row.kind === "string") return "string";
  if (row.kind === "oneOf") return "text";
  return "text";
}

/** @emoji ✅ Validates a value against an attribute definition schema. */
export function validateAttributeValue(defn: AttributeDefinitionSpec, value: unknown): boolean {
  const schema = defn.value;
  if (!schema || typeof schema !== "object") return false;
  const row = schema as Record<string, unknown>;
  const kind = row.kind;
  if (kind === "string") {
    if (typeof value !== "string") return false;
    const options = row.options;
    if (Array.isArray(options) && options.length > 0) return options.includes(value);
    return true;
  }
  if (kind === "number") return typeof value === "number" && Number.isFinite(value);
  if (kind === "boolean") return typeof value === "boolean";
  if (kind === "record") {
    if (!value || typeof value !== "object" || Array.isArray(value)) return false;
    return true;
  }
  if (kind === "oneOf" && Array.isArray(row.variants)) {
    return row.variants.some((variant) => validateAttributeValue({ ...defn, value: variant }, value));
  }
  return false;
}

/** @emoji 🪪 Qualified transformation id (`modelDefinitionId.transformationId`). */
export function qualifiedTransformationId(modelDefinitionId: string, transformationId: string): string {
  return `${modelDefinitionId}.${transformationId}`;
}

/** @emoji 🔄 Z-band selector for surface classification rules. */
export type TransformationDeriveZBand = "min" | "max" | "mid";

/** @emoji 🔄 One surface-classification rule in a transformation `derive` block. */
export interface TransformationDeriveClassifyRule {
  readonly role: string;
  readonly typology: string;
  readonly dominantAxis?: "x" | "y" | "z";
  readonly minDominantNormal?: number;
  readonly minAxisNormal?: number;
  readonly zBand?: TransformationDeriveZBand;
  readonly fallback?: boolean;
}

/** @emoji 🔄 Opening metadata → typology mapping in a transformation `derive` block. */
export interface TransformationDeriveOpening {
  readonly fields: readonly string[];
  readonly values: readonly (string | boolean)[];
  readonly typology: string;
  readonly role: string;
}

/** @emoji 🔄 Solid-fuse options for a transformation `derive` block. */
export interface TransformationDeriveFuse {
  readonly hullSolidId?: string;
  readonly contactPairs?: readonly (readonly [string, string])[];
  readonly maxSeparation?: number;
}

/** @emoji 🔄 Source primitive collection for a transformation `derive` block. */
export interface TransformationDeriveCollect {
  readonly sourceModelDefinition: string;
  readonly primitiveKind: TypologyPrimitiveKind;
}

/** @emoji 🔄 Hull object row for a transformation `derive` block. */
export interface TransformationDeriveHull {
  readonly typology: string;
  readonly primitiveKind: string;
}

/** @emoji 🔄 Ensures typology rows exist after derive. */
export interface TransformationDeriveEnsure {
  readonly typology: string;
  readonly empty?: boolean;
}

/** @emoji 🔄 Declarative surface-classification derive spec on `spatial.transformation/v1`. */
export interface TransformationDeriveSpec {
  readonly collect: TransformationDeriveCollect;
  readonly fuse?: TransformationDeriveFuse;
  readonly hull: TransformationDeriveHull;
  readonly classify: {
    readonly zTolRatio?: number;
    readonly zTolMin?: number;
    readonly rules: readonly TransformationDeriveClassifyRule[];
    readonly opening?: TransformationDeriveOpening;
    readonly mergeRoles?: readonly string[];
    readonly mergeByPlane?: boolean;
  };
  readonly ensure?: readonly TransformationDeriveEnsure[];
}

/** @emoji 🔄 Parsed transformation (`spatial.transformation/v1`). */
export interface TransformationSpec {
  readonly schema: "spatial.transformation/v1";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly modelDefinitionId: string;
  readonly source: { readonly modelDefinition: string };
  readonly target: { readonly modelDefinition: string };
  readonly typologies: readonly string[];
  readonly derive?: TransformationDeriveSpec;
}

function parseTransformationDeriveClassifyRule(raw: unknown): TransformationDeriveClassifyRule | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (typeof r.role !== "string" || typeof r.typology !== "string") return null;
  const dominantAxis = r.dominantAxis === "x" || r.dominantAxis === "y" || r.dominantAxis === "z" ? r.dominantAxis : undefined;
  const zBand = r.zBand === "min" || r.zBand === "max" || r.zBand === "mid" ? r.zBand : undefined;
  return {
    role: r.role,
    typology: r.typology,
    ...(dominantAxis ? { dominantAxis } : {}),
    ...(typeof r.minDominantNormal === "number" ? { minDominantNormal: r.minDominantNormal } : {}),
    ...(typeof r.minAxisNormal === "number" ? { minAxisNormal: r.minAxisNormal } : {}),
    ...(zBand ? { zBand } : {}),
    ...(r.fallback === true ? { fallback: true } : {}),
  };
}

function parseTransformationDeriveSpec(raw: unknown): TransformationDeriveSpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  const collect = r.collect as Record<string, unknown> | undefined;
  const hull = r.hull as Record<string, unknown> | undefined;
  const classify = r.classify as Record<string, unknown> | undefined;
  if (typeof collect?.sourceModelDefinition !== "string") return null;
  const primitiveKind = collect.primitiveKind;
  const allowed = new Set<TypologyPrimitiveKind>(["anchor", "vertex", "edge", "wire", "face", "shell", "solid"]);
  if (typeof primitiveKind !== "string" || !allowed.has(primitiveKind as TypologyPrimitiveKind)) return null;
  if (typeof hull?.typology !== "string" || typeof hull?.primitiveKind !== "string") return null;
  if (!classify || !Array.isArray(classify.rules) || classify.rules.length === 0) return null;
  const rules = classify.rules.map(parseTransformationDeriveClassifyRule).filter((row): row is TransformationDeriveClassifyRule => row !== null);
  if (rules.length !== classify.rules.length) return null;
  const fuseRaw = r.fuse as Record<string, unknown> | undefined;
  const fuse: TransformationDeriveFuse | undefined =
    fuseRaw && typeof fuseRaw === "object"
      ? {
          ...(typeof fuseRaw.hullSolidId === "string" ? { hullSolidId: fuseRaw.hullSolidId } : {}),
          ...(Array.isArray(fuseRaw.contactPairs)
            ? {
                contactPairs: fuseRaw.contactPairs
                  .filter((pair): pair is [string, string] => Array.isArray(pair) && pair.length === 2 && typeof pair[0] === "string" && typeof pair[1] === "string")
                  .map((pair) => [pair[0], pair[1]] as const),
              }
            : {}),
          ...(typeof fuseRaw.maxSeparation === "number" ? { maxSeparation: fuseRaw.maxSeparation } : {}),
        }
      : undefined;
  const openingRaw = classify.opening as Record<string, unknown> | undefined;
  const opening: TransformationDeriveOpening | undefined =
    openingRaw &&
    Array.isArray(openingRaw.fields) &&
    openingRaw.fields.every((f) => typeof f === "string") &&
    Array.isArray(openingRaw.values) &&
    typeof openingRaw.typology === "string" &&
    typeof openingRaw.role === "string"
      ? {
          fields: openingRaw.fields as string[],
          values: openingRaw.values as (string | boolean)[],
          typology: openingRaw.typology,
          role: openingRaw.role,
        }
      : undefined;
  const ensure: TransformationDeriveEnsure[] | undefined = Array.isArray(r.ensure)
    ? r.ensure
        .map((row) => {
          if (!row || typeof row !== "object") return null;
          const e = row as Record<string, unknown>;
          if (typeof e.typology !== "string") return null;
          return { typology: e.typology, ...(e.empty === true ? { empty: true } : {}) };
        })
        .filter((row): row is TransformationDeriveEnsure => row !== null)
    : undefined;
  return {
    collect: { sourceModelDefinition: collect.sourceModelDefinition, primitiveKind: primitiveKind as TypologyPrimitiveKind },
    ...(fuse ? { fuse } : {}),
    hull: { typology: hull.typology, primitiveKind: hull.primitiveKind },
    classify: {
      rules,
      ...(typeof classify.zTolRatio === "number" ? { zTolRatio: classify.zTolRatio } : {}),
      ...(typeof classify.zTolMin === "number" ? { zTolMin: classify.zTolMin } : {}),
      ...(opening ? { opening } : {}),
      ...(Array.isArray(classify.mergeRoles) ? { mergeRoles: classify.mergeRoles.filter((x): x is string => typeof x === "string") } : {}),
      ...(classify.mergeByPlane === true ? { mergeByPlane: true } : {}),
    },
    ...(ensure?.length ? { ensure } : {}),
  };
}

/** @emoji 🧾 Parses `spatial.transformation/v1` JSON; `modelDefinitionId` comes from the asset folder. */
export function parseTransformationSpec(raw: unknown, modelDefinitionId: string): TransformationSpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.transformation/v1") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string" || typeof r.label !== "string") return null;
  const source = r.source as { modelDefinition?: unknown } | undefined;
  const target = r.target as { modelDefinition?: unknown } | undefined;
  if (typeof source?.modelDefinition !== "string" || typeof target?.modelDefinition !== "string") return null;
  if (!Array.isArray(r.typologies) || r.typologies.length === 0) return null;
  const typologies = r.typologies.filter((row): row is string => typeof row === "string");
  if (typologies.length !== r.typologies.length) return null;
  const derive = r.derive !== undefined ? parseTransformationDeriveSpec(r.derive) : undefined;
  if (r.derive !== undefined && !derive) return null;
  return {
    schema: "spatial.transformation/v1",
    id: r.id,
    version: r.version,
    label: r.label,
    description: typeof r.description === "string" ? r.description : undefined,
    modelDefinitionId,
    source: { modelDefinition: source.modelDefinition },
    target: { modelDefinition: target.modelDefinition },
    typologies,
    ...(derive ? { derive } : {}),
  };
}

function modelDefinitionIdFromTransformationAssetPath(assetPath: string): string | null {
  const normalized = assetPath.replace(/\\/g, "/");
  const marker = "/assets/modelDefinition/";
  const idx = normalized.indexOf(marker);
  if (idx < 0) return null;
  const rest = normalized.slice(idx + marker.length);
  const parts = rest.split("/");
  const tIdx = parts.indexOf("transformation");
  if (tIdx <= 0) return null;
  return parts.slice(0, tIdx).join(".");
}

function shippedTransformationCatalog(): readonly TransformationSpec[] {
  const seen = new Set<string>();
  const out: TransformationSpec[] = [];
  for (const [path, raw] of Object.entries(modelDefinitionTransformationModules())) {
    const modelDefinitionId = modelDefinitionIdFromTransformationAssetPath(path);
    if (!modelDefinitionId) continue;
    const spec = parseTransformationSpec(raw, modelDefinitionId);
    if (!spec) continue;
    const qid = qualifiedTransformationId(spec.modelDefinitionId, spec.id);
    if (seen.has(qid)) continue;
    seen.add(qid);
    out.push(spec);
  }
  return out;
}

/** @emoji 📚 Lists transformation assets under spatial/assets/modelDefinition. */
export function listModelDefinitionTransformations(): readonly TransformationSpec[] {
  return shippedTransformationCatalog();
}

/** @emoji 📚 Loads a transformation by qualified id (`aec.building.energy.from_geometry`). */
export function loadTransformation(qualifiedId: string): TransformationSpec | null {
  return shippedTransformationCatalog().find((row) => qualifiedTransformationId(row.modelDefinitionId, row.id) === qualifiedId) ?? null;
}

/** @emoji 🔄 Lists transformations whose target is `modelDefinitionId` (derive current definition from source). */
export function listTransformationsIntoModelDefinition(modelDefinitionId: string): readonly TransformationSpec[] {
  return listModelDefinitionTransformations().filter((row) => row.target.modelDefinition === modelDefinitionId);
}

/** @emoji 🔄 Lists transformations whose source is `modelDefinitionId` (derive another definition from current). */
export function listTransformationsFromModelDefinition(modelDefinitionId: string): readonly TransformationSpec[] {
  return listModelDefinitionTransformations().filter((row) => row.source.modelDefinition === modelDefinitionId);
}

// #region 🧭ModelDefinitionScope
function modelDefinitionFolderFromAssetPath(assetPath: string): string | null {
  const normalized = assetPath.replace(/\\/g, "/");
  const marker = "/assets/modelDefinition/";
  const idx = normalized.indexOf(marker);
  if (idx < 0) return null;
  const rest = normalized.slice(idx + marker.length);
  const folder = rest.split("/")[0];
  return folder || null;
}

function modelDefinitionFolderIdMap(): ReadonlyMap<string, string> {
  if (modelDefinitionFolderIdMapCache) return modelDefinitionFolderIdMapCache;
  const map = new Map<string, string>();
  const modules = {
    ...modelDefinitionAssetModules.manifests,
    ...modelDefinitionAssetModules.extensions,
  };
  for (const [path, raw] of Object.entries(modules)) {
    const folder = modelDefinitionFolderFromAssetPath(path);
    const manifest = parseModelDefinitionManifest(raw);
    if (!folder || !manifest) continue;
    map.set(folder, manifest.id);
  }
  modelDefinitionFolderIdMapCache = map;
  return map;
}

/** @emoji 🧭 Resolves manifest `id` from an asset path under `spatial/assets/modelDefinition`. */
export function modelDefinitionIdFromAssetPath(assetPath: string): string | null {
  const folder = modelDefinitionFolderFromAssetPath(assetPath);
  if (!folder) return null;
  return modelDefinitionFolderIdMap().get(folder) ?? null;
}

function typologyOwnerById(): ReadonlyMap<string, string> {
  if (typologyOwnerByIdCache) return typologyOwnerByIdCache;
  const map = new Map<string, string>();
  for (const [path, raw] of Object.entries(modelDefinitionAssetModules.typologies)) {
    const owner = modelDefinitionIdFromAssetPath(path);
    const spec = parseTypologySpec(raw);
    if (!owner || !spec) continue;
    map.set(spec.id, owner);
  }
  typologyOwnerByIdCache = map;
  return map;
}

/** @emoji 🧭 Typologies owned by a model-definition folder manifest. */
export function listTypologiesForModelDefinition(modelDefinitionId: string): readonly TypologySpec[] {
  const owners = typologyOwnerById();
  return shippedTypologyCatalog().filter((row) => owners.get(row.id) === modelDefinitionId);
}

function actionOwnerById(): ReadonlyMap<string, string> {
  if (actionOwnerByIdCache) return actionOwnerByIdCache;
  const map = new Map<string, string>();
  for (const [path, raw] of Object.entries(modelDefinitionAssetModules.actions)) {
    const owner = modelDefinitionIdFromAssetPath(path);
    const spec = parseActionSpec(raw);
    if (!owner || !spec) continue;
    map.set(spec.id, owner);
  }
  actionOwnerByIdCache = map;
  return map;
}

/** @emoji 🧭 True when an action asset file lives under `modelDefinitionId`. */
export function actionOwnedByModelDefinition(actionId: string, modelDefinitionId: string): boolean {
  return actionOwnerById().get(actionId) === modelDefinitionId;
}

/** @emoji 🧭 Model definition that owns a typology asset. */
export function modelDefinitionIdForTypology(typologyId: string): string | null {
  return typologyOwnerById().get(typologyId) ?? null;
}

/** @emoji 📚 Host-facing interaction row from model-definition interaction JSON. */
export interface SpatialInteraction {
  readonly id: string;
  readonly label: string;
  /** @emoji ⌨️ Host interaction key; must stay unique and appear in `label`. */
  readonly key: string;
}

function interactionOwnerById(): ReadonlyMap<string, string> {
  if (interactionOwnerByIdCache) return interactionOwnerByIdCache;
  const map = new Map<string, string>();
  for (const [path, raw] of Object.entries(modelDefinitionAssetModules.interactions)) {
    const owner = modelDefinitionIdFromAssetPath(path);
    const spec = parseInteractionSpec(raw);
    if (!owner || !spec) continue;
    map.set(spec.id, owner);
  }
  interactionOwnerByIdCache = map;
  return map;
}

/** @emoji 🧭 Model definition that owns an interaction asset. */
export function modelDefinitionIdForInteraction(interactionId: string): string | null {
  return interactionOwnerById().get(interactionId) ?? null;
}

/** @emoji 🧭 Interactions shipped for a model definition (folder assets + typology references). */
export function listSpatialInteractionsForModelDefinition(
  modelDefinitionId: string,
  options?: { readonly includeCallable?: boolean },
): readonly SpatialInteraction[] {
  const ids = new Set<string>();
  for (const [id, owner] of interactionOwnerById()) {
    if (owner === modelDefinitionId) ids.add(id);
  }
  for (const typology of listTypologiesForModelDefinition(modelDefinitionId)) {
    for (const interactionId of typology.interactions) ids.add(interactionId);
  }
  const catalog = new Map(shippedSpatialInteractionCatalog().map((row) => [row.id, row] as const));
  return [...ids]
    .map((id) => catalog.get(id))
    .filter((row): row is SpatialInteraction => row !== undefined)
    .filter((row) => {
      if (options?.includeCallable) return true;
      const spec = loadSpatialInteraction(row.id);
      return spec ? !isCallableOnlyInteraction(spec) : true;
    })
    .sort((a, b) => a.id.localeCompare(b.id));
}

function attributeOwnerById(): ReadonlyMap<string, string> {
  if (attributeOwnerByIdCache) return attributeOwnerByIdCache;
  const map = new Map<string, string>();
  for (const [path, raw] of Object.entries(modelDefinitionAssetModules.attributes)) {
    const owner = modelDefinitionIdFromAssetPath(path);
    const spec = parseAttributeDefinitionSpec(raw);
    if (!owner || !spec) continue;
    map.set(spec.id, owner);
  }
  attributeOwnerByIdCache = map;
  return map;
}

/** @emoji 🧭 Attribute definitions owned by a model definition. */
export function listAttributeDefinitionsForModelDefinition(modelDefinitionId: string): readonly AttributeDefinitionSpec[] {
  const owners = attributeOwnerById();
  return shippedAttributeDefinitionCatalog().filter((row) => owners.get(row.id) === modelDefinitionId);
}

function propertyOwnerById(): ReadonlyMap<string, string> {
  if (propertyOwnerByIdCache) return propertyOwnerByIdCache;
  const map = new Map<string, string>();
  for (const [path, raw] of Object.entries({
    ...modelDefinitionAssetModules.propertyDefinitions,
    ...modelDefinitionAssetModules.properties,
  })) {
    const owner = modelDefinitionIdFromAssetPath(path);
    const spec = parsePropertyDefinitionSpec(raw);
    if (!owner || !spec) continue;
    map.set(spec.id, owner);
  }
  propertyOwnerByIdCache = map;
  return map;
}

/** @emoji 🧭 Property definitions referenced by typologies in a model definition. */
export function listPropertyDefinitionsForModelDefinition(modelDefinitionId: string): readonly PropertyDefinitionSpec[] {
  const ids = new Set<string>();
  for (const row of shippedPropertyDefinitionCatalog()) {
    if (propertyOwnerById().get(row.id) === modelDefinitionId) ids.add(row.id);
  }
  for (const typology of listTypologiesForModelDefinition(modelDefinitionId)) {
    for (const propertyId of typology.properties ?? []) ids.add(propertyId);
  }
  return [...ids]
    .map((id) => loadPropertyDefinition(id))
    .filter((row): row is PropertyDefinitionSpec => row !== null);
}

/** @emoji 🧭 Interaction ids invoked via `interaction.call` in one spec. */
export function interactionIdsReferencedByInteractionSpec(spec: InteractionSpec): readonly string[] {
  const ids = new Set<string>();
  for (const st of spec.machine.states) {
    for (const h of st.on ?? []) {
      for (const tr of h.transitions) {
        for (const fx of tr.effects ?? []) {
          if (fx.op === "interaction.call") ids.add(fx.interaction);
        }
      }
    }
  }
  return [...ids];
}

/** @emoji 🧭 Action ids referenced by one interaction spec (transition effects + commit + nested interactions). */
export function actionIdsReferencedByInteractionSpec(spec: InteractionSpec): readonly string[] {
  const ids = new Set<string>();
  if (spec.commit.operation.kind === "action") {
    const kit = typologyConstructKitByInteraction().get(spec.id);
    if (kit) {
      ids.add(kit.constructFrom2PointsAndHeight);
      ids.add(kit.constructFromCurveAndHeight);
      ids.add(kit.constructFromSurface);
    } else {
      ids.add(spec.commit.operation.action);
    }
  }
  for (const st of spec.machine.states) {
    for (const h of st.on ?? []) {
      for (const tr of h.transitions) {
        for (const fx of tr.effects ?? []) {
          if (fx.op === "action") ids.add(fx.action);
        }
      }
    }
  }
  for (const nestedId of interactionIdsReferencedByInteractionSpec(spec)) {
    const nested = loadSpatialInteraction(nestedId);
    if (nested) {
      for (const actionId of actionIdsReferencedByInteractionSpec(nested)) ids.add(actionId);
    }
  }
  return [...ids];
}

/** @emoji 🧭 Action ids declared on typologies, action assets, or owned interactions. */
export function listActionsForModelDefinition(modelDefinitionId: string): readonly string[] {
  const ids = new Set<string>();
  for (const typology of listTypologiesForModelDefinition(modelDefinitionId)) {
    for (const actionId of typology.actions) {
      if (actionOwnedByModelDefinition(actionId, modelDefinitionId)) ids.add(actionId);
    }
  }
  for (const [path, raw] of Object.entries(modelDefinitionAssetModules.actions)) {
    if (modelDefinitionIdFromAssetPath(path) !== modelDefinitionId) continue;
    const spec = parseActionSpec(raw);
    if (spec) ids.add(spec.id);
  }
  for (const row of listSpatialInteractionsForModelDefinition(modelDefinitionId)) {
    const interaction = loadSpatialInteraction(row.id);
    if (interaction) {
      for (const actionId of actionIdsReferencedByInteractionSpec(interaction)) ids.add(actionId);
    }
  }
  return [...ids].sort((a, b) => a.localeCompare(b));
}

/** @emoji 🧭 True when `actionId` is declared in the active model definition (or is `selection.apply`). */
export function actionAvailableInModelDefinition(actionId: string, modelDefinitionId: string): boolean {
  if (actionId === "selection.apply") return true;
  if (actionId.startsWith("command.")) return true;
  const transformation = loadTransformation(actionId);
  if (transformation) {
    return transformation.source.modelDefinition === modelDefinitionId || transformation.target.modelDefinition === modelDefinitionId;
  }
  return listActionsForModelDefinition(modelDefinitionId).includes(actionId);
}

/** @emoji 🧭 Selection command fixtures whose action assets belong to a model definition. */
export function listSelectionOperationsForModelDefinition(modelDefinitionId: string): readonly SelectionOperationInteractionDef[] {
  return selectionOperationsForModelDefinitionFromActions(modelDefinitionId);
}

/** @emoji 🧭 Selection entity kinds available while a model definition is active (factory primitives + objects). */
export function modelDefinitionSelectionEntityKinds(modelDefinitionId: string): readonly ModelEntityKind[] {
  const entityKindIds = new Set<string>([...PRIMITIVE_MODEL_ENTITY_KINDS, "object", "geometry", "attribute"]);
  const kinds = new Set<ModelEntityKind>([...PRIMITIVE_MODEL_ENTITY_KINDS, "object"]);
  for (const defn of listAttributeDefinitionsForModelDefinition(modelDefinitionId)) {
    for (const kind of defn.targets) {
      if (entityKindIds.has(kind)) kinds.add(kind as ModelEntityKind);
    }
    for (const kind of defn.geometrySelector?.kinds ?? []) {
      if (entityKindIds.has(kind)) kinds.add(kind as ModelEntityKind);
    }
  }
  const ordered: ModelEntityKind[] = [];
  for (const kind of PRIMITIVE_MODEL_ENTITY_KINDS) {
    if (kinds.has(kind)) ordered.push(kind);
  }
  for (const kind of kinds) {
    if (!(PRIMITIVE_MODEL_ENTITY_KINDS as readonly string[]).includes(kind)) ordered.push(kind);
  }
  return ordered;
}

/** @emoji 🧭 Object rows owned by typologies declared under a model definition. */
export function listModelObjectsForModelDefinition(model: Model, modelDefinitionId: string): readonly SpatialObjectRecord[] {
  const typologyIds = new Set(listTypologiesForModelDefinition(modelDefinitionId).map((row) => row.id));
  return Object.values(model.objects).filter((row) => typologyIds.has(row.typology));
}

/** @emoji 🧭 Counts in-view typology objects for a model definition (renderer scope). */
export function countViewObjectsForModelDefinition(model: Model, modelDefinitionId: string): number {
  return listModelObjectsForModelDefinition(model, modelDefinitionId).length;
}

/** @emoji 🧭 Summarizes scoped catalogs for the active model definition (hosts + REPL). */
export interface ModelDefinitionScope {
  readonly modelDefinitionId: string;
  readonly typologies: readonly TypologySpec[];
  readonly interactions: readonly SpatialInteraction[];
  readonly selectionOperations: readonly SelectionOperationInteractionDef[];
  readonly attributeDefinitions: readonly AttributeDefinitionSpec[];
  readonly propertyDefinitions: readonly PropertyDefinitionSpec[];
  readonly actions: readonly string[];
  readonly selectionEntityKinds: readonly ModelEntityKind[];
}

/** @emoji 🧭 Resolves everything available under one model definition manifest id. */
export function resolveModelDefinitionScope(modelDefinitionId: string): ModelDefinitionScope {
  return {
    modelDefinitionId,
    typologies: listTypologiesForModelDefinition(modelDefinitionId),
    interactions: listSpatialInteractionsForModelDefinition(modelDefinitionId),
    selectionOperations: listSelectionOperationsForModelDefinition(modelDefinitionId),
    attributeDefinitions: listAttributeDefinitionsForModelDefinition(modelDefinitionId),
    propertyDefinitions: listPropertyDefinitionsForModelDefinition(modelDefinitionId),
    actions: listActionsForModelDefinition(modelDefinitionId),
    selectionEntityKinds: modelDefinitionSelectionEntityKinds(modelDefinitionId),
  };
}
// #endregion 🧭ModelDefinitionScope

// #region 🔄TransformationGeometry
type TransformationApplier = (spec: TransformationSpec, source: Model) => Model;

const transformationAppliers = new Map<string, TransformationApplier>();

/** @emoji 🔄 Registers a model-definition-specific transformation implementation. */
export function registerTransformationApplier(qualifiedTransformationId: string, applier: TransformationApplier): void {
  transformationAppliers.set(qualifiedTransformationId, applier);
}

function collectTransformationPrimitiveRefs(
  model: Model,
  sourceModelDefinition: string,
  primitiveKind: TypologyPrimitiveKind,
): readonly SolidRef[] {
  const refs = new Set<string>();
  for (const obj of listModelObjectsForModelDefinition(model, sourceModelDefinition)) {
    for (const [kind, primitiveRef] of objectPrimitiveEntries(obj)) {
      if (kind === primitiveKind && model.solids[primitiveRef]) refs.add(primitiveRef);
    }
  }
  if (!refs.size) {
    for (const id of Object.keys(model.solids)) refs.add(id);
  }
  return [...refs].sort().map((id) => id as SolidRef);
}

function transformationObjectId(typology: string, index: number): ObjectRef {
  return (index === 0 ? typology : `${typology}#${index}`) as ObjectRef;
}

function deriveOpeningMatch(model: Model, faceId: FaceRef, opening: TransformationDeriveOpening): boolean {
  const fields = model.metadata.get(String(faceId));
  if (!fields) return false;
  for (const field of opening.fields) {
    const value = fields[field];
    if (opening.values.some((candidate) => candidate === value)) return true;
  }
  return false;
}

function deriveClassifyRuleMatches(
  rule: TransformationDeriveClassifyRule,
  normal: Vec3,
  centroid: Vec3,
  zMin: number,
  zMax: number,
  zTol: number,
): boolean {
  if (rule.fallback) return true;
  const ax = Math.abs(normal[0]);
  const ay = Math.abs(normal[1]);
  const az = Math.abs(normal[2]);
  if (rule.dominantAxis === 'z' && rule.minDominantNormal != null) {
    if (!(az >= ax && az >= ay && az >= rule.minDominantNormal)) return false;
    if (rule.zBand === 'max') return centroid[2] >= zMax - zTol;
    if (rule.zBand === 'min') return centroid[2] <= zMin + zTol;
    return true;
  }
  if (rule.minAxisNormal != null) return ax >= rule.minAxisNormal || ay >= rule.minAxisNormal;
  return false;
}

function classifyFaceFromDeriveRules(
  derive: TransformationDeriveSpec,
  normal: Vec3,
  centroid: Vec3,
  zMin: number,
  zMax: number,
  zTol: number,
): TransformationDeriveClassifyRule {
  for (const rule of derive.classify.rules) {
    if (deriveClassifyRuleMatches(rule, normal, centroid, zMin, zMax, zTol)) return rule;
  }
  return derive.classify.rules[derive.classify.rules.length - 1]!;
}

function cloneModelGeometryShell(source: Model): Model {
  const target = new Model();
  target.revision = source.revision;
  target.anchors = source.anchors;
  target.vertices = source.vertices;
  target.edges = source.edges;
  target.wires = source.wires;
  target.faces = source.faces;
  target.shells = { ...source.shells };
  target.solids = { ...source.solids };
  return target;
}

/** @emoji 🔄 Copies geometry and keeps only object rows whose typology is listed on the transformation spec. */
function applyTransformationFallback(spec: TransformationSpec, source: Model): Model {
  const target = cloneModelGeometryShell(source);
  const allowedTypologies = new Set(spec.typologies);
  const objects: Model["objects"] = {};
  for (const row of Object.values(source.objects)) {
    if (!allowedTypologies.has(row.typology)) continue;
    objects[row.id] = row;
  }
  target.objects = objects;
  target.bump();
  return target;
}

function runDeriveTransformation(spec: TransformationSpec, source: Model, preview: SpatialPreviewKernel): Model {
  const derive = spec.derive;
  if (!derive) throw new Error(`transformation ${qualifiedTransformationId(spec.modelDefinitionId, spec.id)} is missing derive`);
  const target = cloneModelGeometryShell(source);
  const solidRefs = collectTransformationPrimitiveRefs(source, derive.collect.sourceModelDefinition, derive.collect.primitiveKind);
  if (!solidRefs.length) {
    target.bump();
    return target;
  }
  const sourceObjectIds = sortedRecordValues(source.objects).map((row) => String(row.id));
  const { hullSolid, externalFaces } = preview.fuseSolidsToExternalFaces(target, solidRefs, {
    hullSolidId: derive.fuse?.hullSolidId,
    contactPairs: derive.fuse?.contactPairs,
    maxSeparation: derive.fuse?.maxSeparation,
  });
  if (solidRefs.length > 1) {
    target.metadata.setField(String(hullSolid), 'fuseSourceSolidIds', solidRefs.map(String));
    target.solids[hullSolid] = { id: hullSolid, shellIds: [] };
  }
  const centroids: Vec3[] = [];
  const faceMeta: { readonly face: FaceRef; readonly normal: Vec3; readonly centroid: Vec3 }[] = [];
  for (const faceId of externalFaces) {
    const face = target.faces[faceId];
    if (!face) continue;
    const normal = preview.faceNormal(target, face);
    const centroid = preview.faceCentroid(target, face);
    if (!normal || !centroid) continue;
    faceMeta.push({ face: faceId, normal, centroid });
    centroids.push(centroid);
  }
  let zMin = Infinity;
  let zMax = -Infinity;
  for (const c of centroids) {
    zMin = Math.min(zMin, c[2]);
    zMax = Math.max(zMax, c[2]);
  }
  if (!Number.isFinite(zMin)) {
    zMin = 0;
    zMax = 0;
  }
  const zTolRatio = derive.classify.zTolRatio ?? 0.02;
  const zTolMin = derive.classify.zTolMin ?? 1e-3;
  const zTol = Math.max((zMax - zMin) * zTolRatio, zTolMin);
  const roleCounts = new Map<string, number>();
  const hullTypology = derive.hull.typology as TypologyRef;
  target.objects[transformationObjectId(hullTypology, 0)] = {
    id: transformationObjectId(hullTypology, 0),
    typology: hullTypology,
    primitives: { [derive.hull.primitiveKind]: String(hullSolid) },
    attributes: { sourceObjectIds, fusedSolidIds: solidRefs.map(String) },
  };
  const mergeRoles = new Set(derive.classify.mergeRoles ?? []);
  const mergeByPlane = derive.classify.mergeByPlane === true;
  const groupedFaces = new Map<string, { readonly role: string; readonly typology: TypologyRef; readonly faces: FaceRef[] }>();
  for (const row of faceMeta) {
    const opening = derive.classify.opening;
    const classified =
      opening && deriveOpeningMatch(source, row.face, opening)
        ? { role: opening.role, typology: opening.typology as TypologyRef }
        : (() => {
            const rule = classifyFaceFromDeriveRules(derive, row.normal, row.centroid, zMin, zMax, zTol);
            return { role: rule.role, typology: rule.typology as TypologyRef };
          })();
    const mergeGroup = mergeRoles.has(classified.role);
    const groupKey = mergeGroup && mergeByPlane
      ? `${classified.typology}:${preview.facePlaneGroupKey(row.normal, row.centroid)}`
      : `${classified.typology}:${String(row.face)}`;
    const existing = groupedFaces.get(groupKey);
    if (existing) existing.faces.push(row.face);
    else groupedFaces.set(groupKey, { role: classified.role, typology: classified.typology, faces: [row.face] });
  }
  for (const group of groupedFaces.values()) {
    const index = roleCounts.get(group.typology) ?? 0;
    roleCounts.set(group.typology, index + 1);
    const objectId = transformationObjectId(group.typology, index);
    target.objects[objectId] = {
      id: objectId,
      typology: group.typology,
      primitives: { face: String(group.faces[0]!) },
      attributes: { sourceObjectIds, surfaceRole: group.role, faceIds: group.faces.map(String) },
    };
  }
  for (const ensure of derive.ensure ?? []) {
    if (!roleCounts.has(ensure.typology)) {
      const objectId = transformationObjectId(ensure.typology, 0);
      target.objects[objectId] = {
        id: objectId,
        typology: ensure.typology as TypologyRef,
        primitives: ensure.empty ? {} : { face: String(externalFaces[0] ?? '') },
        attributes: { sourceObjectIds },
      };
    }
  }
  target.bump();
  return target;
}

// #endregion 🔄TransformationGeometry

/** @emoji 🔄 Derives a target-definition model from a source model (shared geometry, new object rows). */
export function applyTransformation(spec: TransformationSpec, source: Model, preview: SpatialPreviewKernel): Model {
  const qualified = qualifiedTransformationId(spec.modelDefinitionId, spec.id);
  const applier = transformationAppliers.get(qualified);
  if (applier) return applier(spec, source);
  if (spec.derive) return runDeriveTransformation(spec, source, preview);
  return applyTransformationFallback(spec, source);
}

// #endregion 🧱Model

// #region 🏗️TypologyConstruct
/** @emoji 🏷️ PascalCase object name from a typology label (`External Wall` → `ExternalWall`). */
export function typologyObjectPascalFromLabel(label: string): string {
  return label
    .replace(/[^a-zA-Z0-9]+/g, " ")
    .trim()
    .split(/\s+/)
    .filter(Boolean)
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join("");
}

export type TypologyConstructMode = "2PointsAndHeight" | "curveAndHeight" | "surface";

/** @emoji 🧭 Per-typology construct kit: three mode actions + one interaction id. */
export type TypologyConstructKit = {
  readonly typology: string;
  readonly interaction: string;
  readonly constructFrom2PointsAndHeight: string;
  readonly constructFromCurveAndHeight: string;
  readonly constructFromSurface: string;
};

/** @emoji 🧭 Stable ids: three `construct*From*` actions and one `construct*` interaction. */
export function typologyConstructAssetIds(typology: string, label: string): TypologyConstructKit & { readonly construct: string } {
  const parts = typology.split(".");
  const prefix = parts.length > 1 ? `${parts.slice(0, -1).join(".")}.` : "";
  const pascal = typologyObjectPascalFromLabel(label);
  const interaction = `${prefix}construct${pascal}`;
  return {
    typology,
    interaction,
    construct: interaction,
    constructFrom2PointsAndHeight: `${prefix}construct${pascal}From2PointsAndHeight`,
    constructFromCurveAndHeight: `${prefix}construct${pascal}FromCurveAndHeight`,
    constructFromSurface: `${prefix}construct${pascal}FromSurface`,
  };
}

/** @emoji 🏷️ True when typology construct exposes surface-only workflow (e.g. base plate). */
function typologyConstructIsSurfacePrimary(typologyId: string): boolean {
  return typologyId.endsWith(".baseplate");
}

/** @emoji 🧭 `construct*` action ids declared on a typology (`surface`-primary typologies ship surface only). */
export function typologyConstructModeActionIds(typologyId: string, label: string): readonly string[] {
  const ids = typologyConstructAssetIds(typologyId, label);
  if (typologyConstructIsSurfacePrimary(typologyId)) return [ids.constructFromSurface];
  return [ids.constructFrom2PointsAndHeight, ids.constructFromCurveAndHeight, ids.constructFromSurface];
}

/** @emoji 🎯 Resolves the single mode action an interaction commit must run for `constructMode`. */
export function typologyConstructCommitActionForMode(kit: TypologyConstructKit, mode: string): string {
  switch (mode as TypologyConstructMode) {
    case "2PointsAndHeight":
      return kit.constructFrom2PointsAndHeight;
    case "curveAndHeight":
      return kit.constructFromCurveAndHeight;
    case "surface":
      return kit.constructFromSurface;
    default:
      throw new Error(`Unknown constructMode ${mode} for ${kit.interaction}`);
  }
}

/** @emoji 📄 Declarative capability action JSON for typology construction steps. */
export function capabilityActionSpecJson(id: string, label: string): ActionSpec {
  return {
    schema: "spatial.action/v1",
    id,
    version: "1.0.0",
    label,
    steps: [
      { op: "kernel.call", function: "spatial.action.capability", assignTo: "result" },
      { op: "return", result: { kind: "var", name: "result" } },
    ],
  } as ActionSpec;
}

let typologyConstructKitByInteractionCache: ReadonlyMap<string, TypologyConstructKit> | null = null;

/** @emoji 🧭 Maps each typology construct interaction id to its mode actions (not the interaction id). */
export function typologyConstructKitByInteraction(): ReadonlyMap<string, TypologyConstructKit> {
  if (typologyConstructKitByInteractionCache) return typologyConstructKitByInteractionCache;
  const map = new Map<string, TypologyConstructKit>();
  for (const typology of listModelDefinitionTypologies()) {
    const ids = typologyConstructAssetIds(typology.id, typology.label);
    map.set(ids.interaction, {
      typology: ids.typology,
      interaction: ids.interaction,
      constructFrom2PointsAndHeight: ids.constructFrom2PointsAndHeight,
      constructFromCurveAndHeight: ids.constructFromCurveAndHeight,
      constructFromSurface: ids.constructFromSurface,
    });
  }
  typologyConstructKitByInteractionCache = map;
  return map;
}

/** @emoji 🏗️ True when a typology ships exactly one construct interaction and its mode `construct*` actions. */
export function typologyHasNativeConstructKit(typology: TypologySpec): boolean {
  const ids = typologyConstructAssetIds(typology.id, typology.label);
  const expectedActions = [...typologyConstructModeActionIds(typology.id, typology.label)].sort();
  const actualActions = [...typology.actions].sort();
  return typology.interactions.length === 1 && typology.interactions[0] === ids.interaction && actualActions.join() === expectedActions.join();
}

/** @emoji 🏗️ Typologies in a model definition that expose the native construct interaction. */
export function listConstructableTypologiesForModelDefinition(modelDefinitionId: string): readonly TypologySpec[] {
  return listTypologiesForModelDefinition(modelDefinitionId).filter(typologyHasNativeConstructKit);
}

/** @emoji 🧭 Typology id for an interaction commit (`construct` kit or typology `interactions` list). */
export function typologyIdForInteractionCommit(interactionId: string): string | null {
  const fromKit = typologyConstructKitByInteraction().get(interactionId)?.typology;
  if (fromKit) return fromKit;
  const fromTypology = typologyForInteraction(interactionId)?.id;
  if (fromTypology) return fromTypology;
  const produces = loadSpatialInteraction(interactionId)?.produces?.typology;
  return typeof produces === "string" && produces.length > 0 ? produces : null;
}

/** @emoji 📦 Binds a typology object row to the primary primitive added by a create/construct diff. */
export function ensureTypologyObjectFromCreateDiff(model: Model, typology: string, diff: ModelDiff): ObjectRef | null {
  const typologySpec = loadTypology(typology);
  if (!typologySpec) return null;
  const solidId = diff.solids?.added?.[0]?.id;
  const wireId = diff.wires?.added?.[0]?.id;
  const edgeId = diff.edges?.added?.[0]?.id;
  const primitiveRef = solidId ?? wireId ?? edgeId;
  if (!primitiveRef) return null;
  const primitiveKind =
    (solidId && typologySpec.primitiveKinds.includes("solid") && "solid") ||
    (wireId && typologySpec.primitiveKinds.includes("wire") && "wire") ||
    (edgeId && typologySpec.primitiveKinds.includes("edge") && "edge") ||
    typologySpec.primitiveKinds[0] ||
    "solid";
  const typologyObjectId = typology as ObjectRef;
  const objectId = model.objects[typologyObjectId] ? (String(primitiveRef) as ObjectRef) : typologyObjectId;
  model.objects[objectId] = {
    id: objectId,
    typology: typology as TypologyRef,
    primitives: { [primitiveKind]: String(primitiveRef) },
  };
  model.bump();
  return objectId;
}

// #endregion 🏗️TypologyConstruct

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
  const movedVertexIds = diff.vertices?.modified?.map((row) => row.id) ?? [];
  let nurbsEdgeSyncApplied = false;
  if (movedVertexIds.length > 0) {
    const nurbsSync = modelDiffSyncNurbsThroughEdgesForMovedVertices(model, movedVertexIds);
    if (!isEmptyModelDiff(nurbsSync)) {
      const eInvSync: EntityDiff<EdgeRecord, EdgeRecordDiff, EdgeRef> = {};
      applyEntityDiff(model.edges as Record<string, EdgeRecord>, nurbsSync.edges, eInvSync);
      if (!isEntityDiffEmpty(eInvSync)) {
        nurbsEdgeSyncApplied = true;
        inv.edges = inv.edges
          ? {
              added: [...(inv.edges.added ?? []), ...(eInvSync.added ?? [])],
              modified: [...(inv.edges.modified ?? []), ...(eInvSync.modified ?? [])],
              removed: [...(inv.edges.removed ?? []), ...(eInvSync.removed ?? [])],
            }
          : eInvSync;
      }
    }
  }
  if (!isEntityDiffEmpty(aInv)) inv.anchors = aInv;
  if (!isEntityDiffEmpty(vInv)) inv.vertices = vInv;
  if (!isEntityDiffEmpty(eInv)) inv.edges = eInv;
  if (!isEntityDiffEmpty(wInv)) inv.wires = wInv;
  if (!isEntityDiffEmpty(fInv)) inv.faces = fInv;
  if (!isEntityDiffEmpty(sInv)) inv.shells = sInv;
  if (!isEntityDiffEmpty(cInv)) inv.solids = cInv;
  if (!isEmptyModelDiff(diff) || nurbsEdgeSyncApplied) model.bump();
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
  nurbsCurveFromPoles(poles: readonly Vec3[], through?: boolean): EdgeCurve | null;
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
  facePoints(model: Model, face: FaceRecord): readonly Vec3[];
  faceCentroid(model: Model, face: FaceRecord): Vec3 | null;
  faceNormal(model: Model, face: FaceRecord): Vec3 | null;
  solidFaceIds(model: Model, solidId: string): readonly FaceRef[];
  fuseSolidsToExternalFaces(
    model: Model,
    solidRefs: readonly SolidRef[],
    options?: { readonly hullSolidId?: string; readonly contactPairs?: readonly (readonly [string, string])[]; readonly maxSeparation?: number },
  ): { readonly hullSolid: SolidRef; readonly externalFaces: readonly FaceRef[] };
  facePlaneGroupKey(normal: Vec3, centroid: Vec3): string;
  projectPointOnScalarAxis(base: Vec3, axis: Vec3, raw: Vec3): { readonly projected: Vec3; readonly t: number };
  scalarTopOnAxis(base: Vec3, axis: Vec3, height: number, signedT: number): Vec3;
  clampPointAlongDirection(anchor: Vec3, target: Vec3, length: number): Vec3;
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
      readonly activeModelDefinitionId?: string | null;
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
  syncSolidsFromModel(model: Model): Promise<void>;
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

/** @emoji 🔌 Optional query context for kernel adapters. */
export interface KernelQueryContext {
  readonly model: Model;
  readonly activeModelDefinitionId?: string | null;
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
    readonly activeModelDefinitionId?: string | null;
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

/** @emoji 📚 Lists data-only model-definition action assets. */
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
    { id: "feature.extrudeWireToSolid", run: async (params, ctx) => ctx.kernel.extrudeWireDiff ? ctx.kernel.extrudeWireDiff({ wireId: String(params.wireId ?? params.wire ?? ""), distance: numericParam(params, "distance", 1), direction: vec3Param(params, "direction", [0, 0, 1]), model: ctx.model }) : ({ diff: EMPTY_MODEL_DIFF }) },
    { id: "feature.offsetFaces", run: async (params, ctx) => ctx.kernel.offsetFacesDiff ? ctx.kernel.offsetFacesDiff({ faceIds: Array.isArray(params.faceIds) ? params.faceIds.map(String) : [String(params.faceId ?? "")], distance: numericParam(params, "distance", 1), model: ctx.model }) : ({ diff: EMPTY_MODEL_DIFF }) },
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

/** @emoji 🧭 Runtime registry for data-only `ActionSpec` entries (model-definitions + host overrides). */
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
      readonly activeModelDefinitionId?: string | null;
    },
  ): Promise<ActionResult> {
    assertActionAvailableInModelDefinition(id, ctx.activeModelDefinitionId);
    const def = this.get(id);
    if (def?.spec) return new DeclarativeActionRuntime(def.spec).run(params, ctx);
    if (def?.run) return Promise.resolve(def.run(params, ctx));
    const kernelResult = await executeActionCapability(id, params, {}, ctx);
    if (kernelResult && typeof kernelResult === "object" && "diff" in (kernelResult as object)) return kernelResult as ActionResult;
    if (kernelResult && typeof kernelResult === "object" && "patch" in (kernelResult as object)) return kernelResult as ActionResult;
    if (kernelResult !== undefined) return { data: kernelResult };
    throw new Error(`Unknown action: ${id}`);
  }

  static withModelDefinitionActions(): ActionRegistry {
    const r = new ActionRegistry();
    for (const spec of shippedActionCatalog()) r.register({ id: spec.id, label: spec.label, spec });
    for (const def of modelDefinitionActionCapabilityDefs()) {
      if (r.get(def.id)) continue;
      r.register({
        id: def.id,
        label: def.label ?? def.id,
        spec: capabilityActionSpecJson(def.id, def.label ?? def.id) as ActionSpec,
      });
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

/** @emoji 🎯 Collects edge ids when topology (edge/wire/face/…) is selected; excludes vertex-only picks. */
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

/** @emoji 📦 Center of the axis-aligned bounds of all vertices in `targets`. */
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

/** @emoji 🎛 CAD play gumball modes (move / rotate / scale). */
export type CadTransformGumballMode = "move" | "rotate" | "scale";

/** @emoji 🪟 Per-window transform combobox value (`none` disables the gumball). */
export type CadPlayTransformWindowMode = "none" | CadTransformGumballMode;

/** @emoji 🪟 Ordered transform combobox entries for CAD play window measures. */
export const CAD_PLAY_TRANSFORM_WINDOW_MODES: readonly CadPlayTransformWindowMode[] = ["none", "move", "rotate", "scale"];

/** @emoji 🪟 Type guard for CAD play transform window measure values. */
export function isCadPlayTransformWindowMode(value: string): value is CadPlayTransformWindowMode {
  return (CAD_PLAY_TRANSFORM_WINDOW_MODES as readonly string[]).includes(value);
}

/** @emoji 🪟 Maps window combobox value to active gumball mode or `null` when disabled. */
export function cadTransformGumballModeFromWindowMode(mode: CadPlayTransformWindowMode): CadTransformGumballMode | null {
  return mode === "none" ? null : mode;
}

/** @emoji 🪟 Human label for a CAD play transform window measure item. */
export function cadPlayTransformWindowModeLabel(mode: CadPlayTransformWindowMode): string {
  if (mode === "none") return "None";
  if (mode === "move") return "Move";
  if (mode === "rotate") return "Rotate";
  return "Scale";
}

/** @emoji ✋ True when `targets` resolve to at least one model vertex. */
export function selectionTargetsHaveTransformableVertices(model: Model, targets: readonly SelectionTarget[]): boolean {
  return collectTargetVertices(model, targets).size > 0;
}

/** @emoji 🎛 Maps toolbar gumball mode to TransformControls mode. */
export function cadTransformGumballModeToControlsMode(mode: CadTransformGumballMode): "translate" | "rotate" | "scale" {
  if (mode === "rotate") return "rotate";
  if (mode === "scale") return "scale";
  return "translate";
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

/** @emoji 🎛 Applies `mapPoint` to vertices and nurbs poles on topology-selected edges. */
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

// #region 📍InteractionPointBinding
/** @emoji 📍 Optional geometry snap stored beside a committed interaction point. */
export type InteractionPointSnap = { readonly kind: string; readonly id: string };

/** @emoji 📍 Reads parallel `pointSnaps` rows aligned with `context.points`. */
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

/** @emoji 📍 Replaces interaction points bound to moved vertices with live model positions. */
export function resolveLiveInteractionPoints(
  model: Model,
  points: readonly Vec3[],
  snaps: readonly (InteractionPointSnap | null)[],
): readonly Vec3[] {
  if (!snaps.length) return points;
  return points.map((point, index) => {
    const snap = snaps[index];
    if (!snap || snap.kind !== "vertex") return point;
    const live = model.vertices[snap.id as VertexRef]?.position;
    return live ?? point;
  });
}

/** @emoji 📍 Shallow context copy with `points` resolved from bound vertex snaps. */
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
// #endregion 📍InteractionPointBinding

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

/** @emoji 🪪 model-definition selection command operation id (`selection.apply` param). */
export type SelectionApplyOperation = "selectAll" | "deselectAll" | "invert" | "selectKinds";

/** @emoji 🪪 Headless `selection.apply` / interaction commit input. */
export interface SelectionApplyParams {
  readonly operation: SelectionApplyOperation;
  readonly seedTargets?: readonly SelectionTarget[];
  readonly kinds?: readonly ModelEntityKind[];
}

/** @emoji 🪪 model-definition selection command interaction row (`selection.*` registry). */
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

/** @emoji 🎯 Primary selection row for attribute editing (primitive first, then typology object). */
export function primaryAttributeSelectionTarget(selection: readonly SelectionTarget[]): SelectionTarget | null {
  if (!selection.length) return null;
  for (const row of selection) {
    if (row.kind !== "object") return row;
  }
  return selection.find((row) => row.kind === "object") ?? selection[0] ?? null;
}

/** @emoji 🪪 Collects stable `SelectionTarget` rows for kernel `kinds` scoped to the active model definition. */
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

/** @emoji 🪪 Applies `selectAll` / `deselectAll` / `invert` / `selectKinds` to `current` against `model`. */
export function applySelectionOperation(operation: SelectionApplyOperation, current: readonly SelectionTarget[], model: Model, kinds: readonly ModelEntityKind[], activeModelDefinitionId?: string | null): SelectionTarget[] {
  if (operation === "deselectAll") return [];
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  const scopeKinds = kinds.length > 0 ? kinds : modelDefinitionSelectionEntityKinds(mdId);
  const universe = collectGeometrySelectionTargets(model, scopeKinds, mdId);
  if (operation === "selectAll" || operation === "selectKinds") return universe;
  const cur = new Set(current.map(selectionTargetKey));
  return universe.filter((target) => !cur.has(selectionTargetKey(target)));
}

/** @emoji 🪪 Shared selection command core used by `selection.apply` and headless callers. */
export function executeSelectionApply(params: SelectionApplyParams, ctx: { readonly model: Model; readonly activeModelDefinitionId?: string | null }): SelectionTarget[] {
  const seed = params.seedTargets ?? [];
  const kinds = params.operation === "selectKinds" ? [...(params.kinds ?? [])] : params.operation === "invert" || params.operation === "selectAll" ? [...ALL_MODEL_SELECTION_KINDS] : [];
  return applySelectionOperation(params.operation, seed, ctx.model, kinds, ctx.activeModelDefinitionId ?? null);
}

/** @emoji 🪪 Runs `selection.apply` headless via `ActionRegistry` (no interaction session). */
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
  const actions = ctx.actions ?? ActionRegistry.withModelDefinitionActions();
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

// #region 🔍ConstructQuery
/** @emoji 🔍 One named column in a `construct` result row. */
export type ConstructQueryRow = Readonly<Record<string, unknown>>;

/** @emoji 🔍 `construct` runner output (`rows` for MATCH; CALL modeling yields `diff` geometry when present). */
export interface ConstructQueryResult {
  readonly rows: readonly ConstructQueryRow[];
  readonly data?: unknown;
  readonly diff?: ModelDiff;
}

/** @emoji 🔍 Host wiring for `InteractionRuntime.query` (`@cad/js/query` supplies the default runner). */
export interface ConstructQueryContext {
  readonly model: Model;
  readonly kernel: SpatialKernel;
  readonly actions: ActionRegistry;
  readonly activeModelDefinitionId?: string | null;
  /** @emoji 🪪 Default `seedTargets` for `CALL selection.*` when the call omits `seedTargets`. */
  readonly selectionTargets?: readonly SelectionTarget[];
}

/** @emoji 🔍 Async bridge so core never imports `@cad/js/query`. */
export type ConstructRunner = (text: string, ctx: ConstructQueryContext) => Promise<ConstructQueryResult>;
// #endregion 🔍ConstructQuery

// #region 🎬Statechart
/** @emoji 📞 Pauses host statechart until nested interaction completes or aborts. */
export interface InteractionChildCallSpec {
  readonly interactionId: string;
  readonly inputs?: Record<string, Expr>;
  readonly outputs?: readonly InteractionOutputBinding[] | Record<string, unknown>;
  readonly resumeTarget: string;
  readonly rollback: { readonly state: string; readonly context: Record<string, unknown> };
}

/** @emoji 🎭 Result of `StateEngine.send` / `applyTransition` (`transient` skips interaction-local undo). */
export interface StateEngineSendResult {
  readonly ok: boolean;
  readonly transient?: boolean;
  readonly childCall?: InteractionChildCallSpec;
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
  send(event: InteractionEvent, kernel?: SpatialKernel, model?: Model, actions?: ActionRegistry, preview?: SpatialPreviewKernel, activeModelDefinitionId?: string | null): Promise<StateEngineSendResult>;
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
  preview?: SpatialPreviewKernel,
  activeModelDefinitionId?: string | null,
): Promise<void> {
  const math = preview ?? kernel;
  if (!math) return;
  const env: ExprEnv = { context: ctx, event, model, activeModelDefinitionId, preview: math };
  const reg = actions ?? ActionRegistry.withModelDefinitionActions();
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
  } else if (a.op === "interaction.call") {
    return;
  } else if (a.op === "action") {
    const def = reg.get(a.action);
    if (!def) return;
    assertActionAvailableInModelDefinition(a.action, activeModelDefinitionId);
    const paramBag: Record<string, unknown> = { __context: ctx, __event: event };
    for (const [k, ex] of Object.entries(a.params ?? {})) {
      paramBag[k] = evalExpr(ex, env);
    }
    const k = kernel ?? (null as unknown as SpatialKernel);
    const r = await reg.run(a.action, paramBag, { kernel: k, preview: math, model, activeModelDefinitionId: activeModelDefinitionId ?? null });
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
      if (eff.op === "interaction.call") {
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

/** @emoji 📏 Resolved direct-distance entry config for `state` (from `interaction.lengthEntry`). */
export function interactionLengthEntryForState(spec: InteractionSpec, state: string): InteractionLengthEntrySpec | null {
  return mergeInteractionSpatial(spec).lengthEntry.find((row) => row.state === state) ?? null;
}

/** @emoji 🔢 Resolved live scalar entry config for `state` (from `interaction.scalarEntry`). */
export function interactionScalarEntryForState(spec: InteractionSpec, state: string): InteractionScalarEntrySpec | null {
  return mergeInteractionSpatial(spec).scalarEntry.find((row) => row.state === state) ?? null;
}

/** @emoji 🔢 True when `state` accepts live REPL numeric entry (length or scalar). */
export function interactionInNumericEntryState(spec: InteractionSpec, state: string): boolean {
  return interactionLengthEntryForState(spec, state) !== null || interactionScalarEntryForState(spec, state) !== null;
}

/** @emoji 🔢 Parses REPL `cmdLine` as a live numeric value (`null` = empty, `undefined` = invalid). */
export function parseNumericCommandLine(cmdLine: string): number | null | undefined {
  const t = cmdLine.trim();
  if (!t) return null;
  if (!/^\d*\.?\d*$/.test(t)) return undefined;
  const v = Number(t);
  if (!Number.isFinite(v) || v <= 0) return undefined;
  return v;
}

/** @emoji 📏 Live distance along a length-entry anchor→cursor axis (extrusion rod, rubber band). */
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

/** @emoji 🔢 Explicit length/height lock from context (`set.length` / `set.height`), not live rubber-band distance. */
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

/** @emoji 🔢 Locked numeric value from context when live entry already applied. */
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

/** @emoji 🔢 `set.length` / `set.height` event to apply a numeric value in the active entry state. */
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

/** @emoji 🔢 Commit event after numeric entry (Enter/Space): `pointer.down` with clamped point or `confirm`. */
export function interactionNumericEntryCommitEvent(
  spec: InteractionSpec,
  state: string,
  ctx: Record<string, unknown>,
  preview: SpatialPreviewKernel,
): InteractionEvent | null {
  const lengthEntry = interactionLengthEntryForState(spec, state);
  const scalarEntry = interactionScalarEntryForState(spec, state);
  if (!lengthEntry && !scalarEntry) return null;
  const st = findState(spec, state);
  if (!st?.on) return null;
  const events = new Set(st.on.map((h) => h.event));
  const commitKind =
    scalarEntry?.commit ?? lengthEntry?.commit ?? (scalarEntry ? "confirm" : events.has("pointer.down") ? "pointer.down" : "confirm");
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

/** @emoji ✅ Whether `state` has a passable `confirm` transition (non-selection finalize). */
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

/** @emoji ✅ Enter/Space finalize: `confirm` when available, else length-entry `pointer.down`. */
export function interactionStepFinalizeEvent(
  spec: InteractionSpec,
  state: string,
  ctx: Record<string, unknown>,
  preview: SpatialPreviewKernel,
): InteractionEvent | null {
  if (interactionCanFinalizeStep(spec, state, ctx, preview)) return { kind: "confirm", modifiers: {} };
  return interactionNumericEntryCommitEvent(spec, state, ctx, preview);
}

const LENGTH_LOCK_CTX = "__lengthLock";
const HEIGHT_LOCK_CTX = "__heightLock";
const SCALAR_AXIS_T_CTX = "__scalarAxisT";
const CURSOR_RAW_CTX = "__cursorRaw";

const DEFAULT_SCALAR_AXIS: Vec3 = [0, 0, 1];

/** @emoji 📏 Axis base for scalar rubber-band (`axisAnchor` XY + `axisFloor` Z). */
export function scalarEntryAxisBase(ctx: Record<string, unknown>, entry: InteractionScalarEntrySpec): Vec3 | null {
  if (!entry.axisAnchor) return null;
  const anchor = readInteractionContextVec3(ctx, entry.axisAnchor);
  if (!anchor) return null;
  const floorPath = entry.axisFloor ?? entry.axisAnchor;
  const floor = readInteractionContextVec3(ctx, floorPath);
  const floorZ = floor ? floor[2] : anchor[2];
  return [anchor[0], anchor[1], floorZ];
}

/** @emoji 📏 Projects `raw` onto the scalar axis; returns axis parameter `t` and closest point. */
export function projectPointOnScalarAxis(
  base: Vec3,
  axis: Vec3,
  raw: Vec3,
  preview: SpatialPreviewKernel,
): { readonly projected: Vec3; readonly t: number } {
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

/** @emoji 📏 Parses a dotted `context` path into `PathSegment`s (`points.@last` = last array element). */
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

/** @emoji 📏 Reads a `Vec3` from `context` at dotted `path` (supports `points.@last`). */
export function readInteractionContextVec3(ctx: Record<string, unknown>, path: string): Vec3 | null {
  const raw = readContextPathValue(ctx, parseInteractionContextPath(path));
  if (!Array.isArray(raw) || raw.length < 3) return null;
  const x = Number(raw[0]);
  const y = Number(raw[1]);
  const z = Number(raw[2]);
  if (!Number.isFinite(x) || !Number.isFinite(y) || !Number.isFinite(z)) return null;
  return [x, y, z];
}

/** @emoji 📏 Writes a `Vec3` into `context` at dotted `path`. */
export function writeInteractionContextVec3(ctx: Record<string, unknown>, path: string, value: Vec3): void {
  writePathSegments(ctx, parseInteractionContextPath(path), value);
}

/** @emoji 📏 Clamps `target` to `length` units from `anchor` along the anchor→target ray. */
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

function applyLengthEntryToContext(
  ctx: Record<string, unknown>,
  entry: InteractionLengthEntrySpec,
  raw: Vec3,
  lock: number,
  preview: SpatialPreviewKernel,
): void {
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
  async send(event: InteractionEvent, kernel?: SpatialKernel, model?: Model, actions?: ActionRegistry, preview?: SpatialPreviewKernel, activeModelDefinitionId?: string | null): Promise<StateEngineSendResult> {
    const r = await applyTransition(this.spec, this.state, this.context, event, kernel, actions, model, preview, activeModelDefinitionId ?? null);
    if (!r.ok) return { ok: false };
    if (r.childCall) return { ok: true, transient: r.transient, childCall: r.childCall };
    this.state = r.nextState;
    return { ok: true, transient: r.transient };
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

/** @emoji 🖼️ Renderer-neutral snapshot slice consumed by `@cad/js/renderer`. */
export interface DisplayModel {
  readonly prompt?: string;
  readonly items: readonly DisplayItem[];
}

/** @emoji 🖼️ Instantiates `display.states[state]` templates using current `context`. */
export function resolveDisplay(
  spec: InteractionSpec,
  state: string,
  context: Record<string, unknown>,
  preview: SpatialPreviewKernel,
  model?: Model,
): DisplayModel {
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
    const height =
      heightLock ??
      (typeof fieldVal === "number" && Number.isFinite(fieldVal) && fieldVal > 0 ? fieldVal : null);
    if (base && height != null) {
      const raw = readInteractionContextVec3(context, CURSOR_RAW_CTX);
      const signedT =
        typeof context[SCALAR_AXIS_T_CTX] === "number" && Number.isFinite(context[SCALAR_AXIS_T_CTX])
          ? (context[SCALAR_AXIS_T_CTX] as number)
          : height;
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

/** @emoji 📜 Host frame while a nested interaction session is active (chain via `outer`). */
export interface InteractionNestedHostFrame {
  readonly hostInteractionId: string;
  readonly hostState: string;
  readonly hostContext: Record<string, unknown>;
  readonly outer?: InteractionNestedHostFrame;
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

/** @emoji 📞 Resolves an interaction spec for `interaction.call` (registry first, then shipped assets). */
export function resolveInteractionSpecForCall(interactionId: string, registry?: InteractionRegistry): InteractionSpec | null {
  return registry?.get(interactionId) ?? loadSpatialInteraction(interactionId);
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
  private child: InteractionRuntime | null = null;
  private pausedHost: InteractionChildCallSpec | null = null;

  constructor(
    private readonly spec: InteractionSpec,
    private readonly opts: InteractionRuntimeOptions,
  ) {
    this.sm = (opts.stateEngine ?? pureTsStateEngineProvider).create(spec);
    this.actions = opts.actions ?? ActionRegistry.withModelDefinitionActions();
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
    if (this.child) return true;
    return isInteractionSessionActive(this.spec, this.sm.getState());
  }

  private seedChildContext(child: InteractionRuntime, inputs?: Record<string, Expr>): void {
    if (!inputs) return;
    const ctx = child.sm.getContext();
    const env: ExprEnv = { context: ctx, event: { kind: "start" } };
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

  private async handleEngineSendResult(
    r: StateEngineSendResult,
    beforeState: string,
    beforeCtx: Record<string, unknown>,
  ): Promise<void> {
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

  /** @emoji 🧭 Accepted geometry entity kinds for the active machine state (`[]` when none). */
  listActiveSelectionAccept(): readonly ModelEntityKind[] {
    if (this.child) return this.child.listActiveSelectionAccept();
    return getActiveSelectionSpec(this.spec, this.sm.getState())?.accept ?? [];
  }

  /** @emoji 🔍 Executes a `construct` script via `opts.query` (host registers `@cad/js/query`). */
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

  /** @emoji 📜 Merges `start.targets` when `compileInteraction` began in the post-start state without running transition effects. */
  private applyInstantStartPayload(event: InteractionEvent): void {
    const raw = event.targets;
    if (!Array.isArray(raw) || raw.length === 0) return;
    this.sm.getContext().seedTargets = raw;
  }

  /** @emoji 📜 Dispatches a typed interaction event through the statechart + optional kernel queries. */
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
      const kit = typologyConstructKitByInteraction().get(this.spec.id);
      const actionId = kit ? typologyConstructCommitActionForMode(kit, String(paramBag.constructMode ?? ctx.constructMode ?? "")) : op.action;
      const ar = await this.actions.run(actionId, paramBag, {
        kernel: k,
        preview: this.previewKernel(),
        model: model,
        activeModelDefinitionId: this.opts.activeModelDefinitionId ?? null,
      });
      if (ar.patch) applyActionPatchToContext(this.sm.getContext(), ar.patch);
      diff = ar.diff ?? EMPTY_MODEL_DIFF;
      data = ar.data ?? null;
      if (isEmptyModelDiff(diff) && op.action === "command.finish") {
        return fail("interaction.emptyCommit", "Command produced no geometry; add more points and finish again.");
      }
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

  /** @emoji 📜 Executes `commit.operation` against `kernel`, applies `diff` to `document.model`, records history. */
  async commit(): Promise<InteractionResponse> {
    if (this.child) return this.child.commit();
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
  const mdId = opts.activeModelDefinitionId ?? defaultModelDefinitionId();
  const defn = selectionOperationsForModelDefinitionFromActions(mdId).find((row) => row.id === interactionId);
  if (!defn) throw new Error(`Not a selection operation: ${interactionId}`);
  assertActionAvailableInModelDefinition(interactionId, mdId);
  const seedTargets = opts.seedTargets ?? [];
  const result = await (opts.actions ?? ActionRegistry.withModelDefinitionActions()).run(
    interactionId,
    { seedTargets, __context: {}, __event: { kind: "commit" } },
    { kernel: opts.kernel, preview: opts.previewKernel ?? (opts.kernel as unknown as SpatialPreviewKernel), model: opts.document.model, activeModelDefinitionId: mdId },
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

const SELECTION_ACTION_META: Readonly<
  Record<string, { readonly operation: SelectionApplyOperation; readonly kinds?: readonly ModelEntityKind[] }>
> = {
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

/** @emoji 🪪 Maps a `selection.*` interaction id to headless `SelectionApplyParams`. */
export function selectionApplyParamsForInteraction(defn: SelectionOperationInteractionDef, seedTargets: readonly SelectionTarget[] = []): SelectionApplyParams {
  return {
    operation: defn.operation,
    seedTargets,
    ...(defn.kinds ? { kinds: defn.kinds } : {}),
  };
}

/** @emoji 🪪 True when a selection command targets authored `object` rows on the model. */
export function selectionOperationUsesModelObjects(defn: Pick<SelectionOperationInteractionDef, "kinds">): boolean {
  return defn.kinds?.includes("object") ?? false;
}

/** @emoji 🪪 Default seed targets for invert/deselectAll (otherwise empty). */
export function selectionSeedTargetsForOperation(operation: SelectionApplyOperation, seedCell: SelectionTarget = { kind: "solid", id: "e2e-box", editable: true }): readonly SelectionTarget[] {
  return operation === "invert" || operation === "deselectAll" ? [seedCell] : [];
}

const shippedInteractionJsons = modelDefinitionInteractionCatalog() as readonly ModelDefinitionInteractionFixture[];

function interactionFixtureRow(spec: ModelDefinitionInteractionFixture): SpatialInteraction {
  return { id: spec.id, label: spec.label ?? spec.id, key: typeof spec.key === "string" ? spec.key : (spec.id[0] ?? "?") };
}

function shippedSpatialInteractionCatalog(): readonly SpatialInteraction[] {
  return shippedInteractionJsons.map(interactionFixtureRow);
}

/** @emoji 🧭 Resolves a typed token to an interaction in one model definition (`key`, `id`, or compact `label`). */
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

/** @emoji 🧭 model-definition `InteractionSpec` registry (fixtures + host `register`). */
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

  static withModelDefinitionInteractions(): InteractionRegistry {
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

const COMPILED_INTERACTION_BY_ID = new Map<string, InteractionSpec>();

/** @emoji 📚 Loads a model-definition interaction by stable `id` (compiled once per id for stable React runtime identity). */
export function loadSpatialInteraction(interactionId: string): InteractionSpec | null {
  const cached = COMPILED_INTERACTION_BY_ID.get(interactionId);
  if (cached) return cached;
  const raw = shippedInteractionJsons.find((spec) => spec.id === interactionId);
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

/** @emoji 📦 Compiled `primitive.box` interaction from model-definition assets. */
export function buildBoxInteractionSpec(): InteractionSpec {
  return requireSpatialInteraction("primitive.box");
}

/** @emoji 📦 Compiled `feature.extrudeWire` interaction from model-definition assets. */
export function buildExtrudeInteractionSpec(): InteractionSpec {
  return requireSpatialInteraction("feature.extrudeWire");
}

/** @emoji 📦 Compiled `feature.offsetSurface` interaction from model-definition assets. */
export function buildOffsetSurfaceInteractionSpec(): InteractionSpec {
  return requireSpatialInteraction("feature.offsetSurface");
}

/** @emoji 📦 Compiled `measure.distance` interaction from model-definition assets. */
export function buildDistanceInteractionSpec(): InteractionSpec {
  return requireSpatialInteraction("measure.distance");
}

/** @emoji 📦 Compiled `measure.area` interaction from model-definition assets. */
export function buildAreaInteractionSpec(): InteractionSpec {
  return requireSpatialInteraction("measure.area");
}

// #endregion 📦Interactions

// #region 🧪Tests
const __spatialCoreTestKernel = import.meta.vitest ? await import("@cad/js/kernel/brepjs") : null;
const __cadInteractionE2EFixtureModules = import.meta.vitest
  ? await Promise.all([
      import("../../assets/play/geometry-loom.json"),
      import("../../assets/play/geometry-routes.json"),
      import("../../assets/play/small-building.model.json"),
    ])
  : null;

if (import.meta.vitest) {
  const { BrepjsKernel, preciseSpatialKernelMath } = __spatialCoreTestKernel!;
  const geometryLoomFixtureJson = __cadInteractionE2EFixtureModules![0].default;
  const geometryRoutesFixtureJson = __cadInteractionE2EFixtureModules![1].default;
  const smallBuildingModelFixtureJson = __cadInteractionE2EFixtureModules![2].default;
  const M = preciseSpatialKernelMath;
  const { describe, expect, it } = import.meta.vitest;

  describe("@cad/js/core vec", () => {
    it("adds and distances", () => {
      expect(M.vec3Distance([0, 0, 0], [3, 4, 0])).toBe(5);
    });
  });

  describe("@cad/js/core model definition catalogs", () => {
    it("loads attribute and property definition assets", () => {
      const attributes = listModelDefinitionAttributeDefinitions();
      const properties = listModelDefinitionPropertyDefinitions();
      expect(attributes.length).toBeGreaterThanOrEqual(6);
      expect(properties.some((row) => row.id === "spatial.shape.volume")).toBe(true);
      expect(loadAttributeDefinition("spatial.shape.material")?.field).toBe("material");
      expect(loadPropertyDefinition("spatial.shape.volume")?.unit).toBe("volume");
    });
    it("loads geometry and AEC typology assets", () => {
      const typologies = listModelDefinitionTypologies();
      expect(typologies.length).toBe(27);
      expect(loadTypology("energy.energy.hull")?.properties).toContain("energy.heatedvolume");
      expect(loadTypology("energy.energy.hull")?.properties).toContain("spatial.shape.volume");
    });
    it("assigns primitiveKinds to geometry typologies", () => {
      const box = loadTypology("spatial.shape.primitive.box");
      const line = loadTypology("spatial.shape.curve.line");
      expect(box?.primitiveKinds).toEqual(["solid"]);
      expect(line?.primitiveKinds).toEqual(["edge", "wire"]);
    });
    it("derives spatial.shape.volume for solid-backed objects", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      const solidId = Object.keys(model.solids)[0]!;
      const object = {
        id: "obj-a" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: solidId },
      };
      const defn = loadPropertyDefinition("spatial.shape.volume")!;
      const kernel = {
        syncSolidsFromModel: async () => {},
        solidVolume: async () => 42,
      } as unknown as SpatialKernel;
      const out = await derivePropertyValue(defn, { model, kernel, object });
      expect(out.volume).toBe(42);
      expect(listApplicablePropertyDefinitionsForModelDefinition(defaultModelDefinitionId(), model, object).map((row) => row.id)).toContain("spatial.shape.volume");
    });
    it("validates object geometry against typology primitiveKinds", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      const solidId = Object.keys(model.solids)[0]!;
      model.objects["obj-a"] = {
        id: "obj-a" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: solidId },
      };
      expect(objectMatchesTypologyPrimitives(model, model.objects["obj-a"]!)).toBe(true);
      model.objects["obj-b"] = {
        id: "obj-b" as ObjectRef,
        typology: "spatial.shape.selection.select-all" as TypologyRef,
        primitives: { solid: solidId },
      };
      expect(objectMatchesTypologyPrimitives(model, model.objects["obj-b"]!)).toBe(false);
    });
  });

  describe("@cad/js/core model space and hashing", () => {
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
      const geo = space.geometryHashesByModel().primary;
      expect(Object.keys(geo?.solid ?? {}).length).toBeGreaterThanOrEqual(1);
      const roundTrip = ModelSpace.fromJSON(space.toJSON());
      expect(roundTrip.get("primary")?.revision).toBe(m.revision);
      expect(hashModelVertices(roundTrip.get("primary")!)).toEqual(hashes.primary);
    });
    it("hashes edges and solids deterministically", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      const hashes = hashModelPrimitives(model);
      const solidId = Object.keys(model.solids)[0]!;
      const edgeId = Object.keys(model.edges)[0]!;
      expect(hashes.solid?.[solidId]).toBeTruthy();
      expect(hashes.edge?.[edgeId]).toBeTruthy();
      expect(hashes.solid?.[solidId]).toBe(hashSolidRecord(model.solids[solidId]!));
    });
  });

  describe("@cad/js/core transformations", () => {
    it("maps CAD play transform window combobox values to gumball modes", () => {
      expect(cadTransformGumballModeFromWindowMode("none")).toBeNull();
      expect(cadTransformGumballModeFromWindowMode("rotate")).toBe("rotate");
      expect(isCadPlayTransformWindowMode("scale")).toBe(true);
      expect(isCadPlayTransformWindowMode("nope")).toBe(false);
      expect(cadPlayTransformWindowModeLabel("none")).toBe("None");
      expect(cadPlayTransformWindowModeLabel("move")).toBe("Move");
    });

    it("lists model definition manifests and transformation directions", () => {
      const manifests = listModelDefinitionManifests();
      expect(manifests.some((row) => row.id === "spatial.shape")).toBe(true);
      expect(manifests.some((row) => row.id === "aec.building.energy")).toBe(true);
      expect(listTransformationsIntoModelDefinition("aec.building.energy").some((row) => row.id === "from_geometry")).toBe(true);
      expect(listTransformationsFromModelDefinition("spatial.shape").some((row) => row.target.modelDefinition === "aec.building.energy")).toBe(true);
    });
    it("scopes catalogs to active model definition", () => {
      const shape = resolveModelDefinitionScope(defaultModelDefinitionId());
      const energy = resolveModelDefinitionScope("aec.building.energy");
      expect(shape.interactions.some((row) => row.id === "primitive.box")).toBe(true);
      expect(energy.interactions.length).toBe(5);
      expect(energy.interactions.every((row) => row.id.startsWith("energy.energy.construct"))).toBe(true);
      expect(energy.interactions.some((row) => row.id === "primitive.box")).toBe(false);
      expect(energy.typologies.some((row) => row.id === "energy.energy.hull")).toBe(true);
      expect(energy.selectionEntityKinds).toContain("object");
      expect(energy.selectionEntityKinds).toContain("solid");
      expect(shape.selectionEntityKinds).toContain("vertex");
      expect(shape.selectionEntityKinds).toContain("face");
      const structure = resolveModelDefinitionScope("aec.building.structure");
      expect(structure.selectionEntityKinds).toContain("face");
      expect(structure.selectionEntityKinds).toContain("object");
      expect(listAttributeDefinitionsForModelDefinitionEntity("aec.building.structure", "face").some((row) => row.field === "exposure")).toBe(
        true,
      );
      expect(listPropertyDefinitionsForModelDefinition("aec.building.energy").some((row) => row.id === "energy.heatedvolume")).toBe(true);
      expect(actionAvailableInModelDefinition("primitive.createBoxFromCorners", defaultModelDefinitionId())).toBe(true);
      expect(actionAvailableInModelDefinition("primitive.createBoxFromCorners", "aec.building.energy")).toBe(false);
      expect(listSelectionOperationsForModelDefinition(defaultModelDefinitionId()).some((row) => row.id === "selection.selectVertices")).toBe(true);
      expect(listSelectionOperationsForModelDefinition("aec.building.energy").length).toBe(0);
    });
    it("buildModelPrimitiveHierarchy nests shell through vertex under solid", () => {
      const model = new Model();
      const solid = solidRef("box-solid");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solid));
      const tree = buildModelPrimitiveHierarchy(model, String(solid));
      expect(tree?.kind).toBe("solid");
      expect(tree?.children).toHaveLength(1);
      const shell = tree!.children[0]!;
      expect(shell.kind).toBe("shell");
      expect(shell.children.length).toBeGreaterThan(0);
      const face = shell.children[0]!;
      expect(face.kind).toBe("face");
      expect(face.children.some((row) => row.kind === "wire")).toBe(true);
      const wire = face.children.find((row) => row.kind === "wire")!;
      expect(wire.children.some((row) => row.kind === "edge")).toBe(true);
      const edge = wire.children.find((row) => row.kind === "edge")!;
      expect(edge.children.every((row) => row.kind === "vertex")).toBe(true);
      expect(edge.children.length).toBe(2);
    });

    it("listModelObjectsForModelDefinition filters objects by typology ownership", () => {
      const model = new Model();
      const cell = solidRef("c0");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
      model.objects["hull"] = { id: "hull" as ObjectRef, typology: "energy.energy.hull", primitives: { solid: String(cell) } };
      model.objects["other"] = { id: "other" as ObjectRef, typology: "spatial.shape.primitive.box", primitives: { solid: String(cell) } };
      expect(listModelObjectsForModelDefinition(model, "aec.building.energy").map((row) => String(row.id))).toEqual(["hull"]);
      expect(countViewObjectsForModelDefinition(model, "aec.building.energy")).toBe(1);
    });
    it("collectGeometrySelectionTargets scopes object rows to model definition typologies", () => {
      const model = new Model();
      const cell = solidRef("c0");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
      model.objects["hull"] = { id: "hull" as ObjectRef, typology: "energy.energy.hull", primitives: { solid: String(cell) } };
      model.objects["other"] = { id: "other" as ObjectRef, typology: "spatial.shape.primitive.box", primitives: { solid: String(cell) } };
      const all = collectGeometrySelectionTargets(model, ["object"], "aec.building.energy");
      expect(all.map((row) => row.id)).toEqual(["hull"]);
    });
    it("ActionRegistry rejects out-of-scope actions for active model definition", async () => {
      const actions = ActionRegistry.withModelDefinitionActions();
      const model = new Model();
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      await expect(
        actions.run(
          "primitive.createBoxFromCorners",
          { cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1, __context: {}, __event: { kind: "test" } },
          { kernel, preview: kernel as unknown as SpatialPreviewKernel, model, activeModelDefinitionId: "aec.building.energy" },
        ),
      ).rejects.toThrow(/not available in model definition aec\.building\.energy/);
    });
    it("loads and applies from_geometry transformation", () => {
      const spec = loadTransformation("aec.building.energy.from_geometry");
      expect(spec?.source.modelDefinition).toBe("spatial.shape");
      expect(spec?.target.modelDefinition).toBe("aec.building.energy");
      const source = new Model();
      applyModelDiff(source, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      source.objects["geom"] = {
        id: "geom" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: "box" },
      };
      const target = applyTransformation(spec!, source, M);
      expect(target.objects["energy.energy.hull"]?.typology).toBe("energy.energy.hull");
      expect(target.objects["energy.energy.hull"]?.primitives).toEqual({ solid: "box" });
      expect(target.solids.box).toBe(source.solids.box);
      expect(listModelObjectsForModelDefinition(target, "aec.building.energy").filter((row) => row.typology === "energy.energy.roof")).toHaveLength(1);
      expect(listModelObjectsForModelDefinition(target, "aec.building.energy").filter((row) => row.typology === "energy.energy.baseplate")).toHaveLength(1);
      expect(listModelObjectsForModelDefinition(target, "aec.building.energy").filter((row) => row.typology === "energy.energy.externalwall")).toHaveLength(4);
      expect(target.objects["energy.energy.roof"]?.primitives.face).toBe("box-box-face-top");
      expect(target.objects["energy.energy.baseplate"]?.primitives.face).toBe("box-box-face-bottom");
      const space = new ModelSpace();
      space.link("geometry", source);
      space.transfer("geometry", "energy", spec!, M);
      expect(space.get("energy")?.objects["energy.energy.windows"]).toBeTruthy();
    });
    it("from_geometry fuses touching shape solids and drops internal faces", () => {
      const spec = loadTransformation("aec.building.energy.from_geometry")!;
      const source = new Model();
      applyModelDiff(source, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("lower")));
      applyModelDiff(source, M.boxModelDiff({ cornerA: [0, 0, 1], cornerB: [1, 1, 1], height: 1 }, solidRef("upper")));
      source.objects["lower"] = {
        id: "lower" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: "lower" },
      };
      source.objects["upper"] = {
        id: "upper" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: "upper" },
      };
      const target = applyTransformation(spec, source, M);
      expect(target.solids["from_geometry-hull"]?.shellIds).toEqual([]);
      expect(target.metadata.get("from_geometry-hull")?.fuseSourceSolidIds).toEqual(["lower", "upper"]);
      expect(target.objects["energy.energy.hull"]?.primitives.solid).toBe("from_geometry-hull");
      expect(target.objects["energy.energy.roof"]?.primitives.face).toBe("box-upper-face-top");
      expect(target.objects["energy.energy.baseplate"]?.primitives.face).toBe("box-lower-face-bottom");
      expect(
        listModelObjectsForModelDefinition(target, "aec.building.energy").filter((row) => row.typology === "energy.energy.externalwall"),
      ).toHaveLength(4);
      expect(
        listModelObjectsForModelDefinition(target, "aec.building.energy").filter(
          (row) => row.typology === "energy.energy.hull" && row.id !== "energy.energy.hull",
        ),
      ).toHaveLength(0);
      expect(target.objects["box-lower-face-top"]).toBeUndefined();
      expect(target.objects["box-upper-face-bottom"]).toBeUndefined();
    });
    it("from_geometry hull heated volume is boolean union not vertex AABB", async () => {
      const spec = loadTransformation("aec.building.energy.from_geometry")!;
      const source = new Model();
      applyModelDiff(source, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("west")));
      applyModelDiff(source, M.boxModelDiff({ cornerA: [3, 0, 0], cornerB: [4, 1, 0], height: 1 }, solidRef("east")));
      source.objects["west"] = {
        id: "west" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: "west" },
      };
      source.objects["east"] = {
        id: "east" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: "east" },
      };
      const target = applyTransformation(spec, source, M);
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const hull = target.objects["energy.energy.hull"]!;
      const heated = await derivePropertyValue(loadPropertyDefinition("energy.heatedvolume")!, { model: target, kernel, object: hull });
      expect(heated.heatedvolume).toBeCloseTo(2, 3);
    });
  });

  describe("@cad/js/core attribute validation", () => {
    it("validates opening attribute options", () => {
      const defn = loadAttributeDefinition("spatial.shape.opening")!;
      expect(validateAttributeValue(defn, "window")).toBe(true);
      expect(validateAttributeValue(defn, "invalid")).toBe(false);
    });
  });

  describe("@cad/js/core edge and solid geometry", () => {
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
    it("circleSamplePoints and edgeCurveLength for circle curves", () => {
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

  describe("@cad/js/core expr", () => {
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

  describe("@cad/js/core model json", () => {
    it("materializeInlineObjectPrimitives promotes play fixture wires into model geometry", () => {
      const space = ModelSpace.fromJSON(geometryRoutesFixtureJson as ModelSpaceJson);
      const model = space.models["spatial.shape"]!;
      expect(model.wires["stub-wire"]?.edgeIds.length).toBeGreaterThan(0);
      expect(model.objects["object-wire-stub-wire"]?.primitives.wire).toBe("stub-wire");
    });

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

  describe("@cad/js/core model commit mesh", () => {
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

  describe("@cad/js/core metadata", () => {
    it("AttributeStore setField bumps model revision", () => {
      const g = new Model();
      const r0 = g.revision;
      g.metadata.setField("e1", "exposure", "external");
      expect(g.revision).toBeGreaterThan(r0);
      expect(g.metadata.get("e1")?.exposure).toBe("external");
    });

    it("AttributeStore entries and JSON roundtrip", () => {
      const store = new AttributeStore(() => {});
      store.setField("face-1", "exposure", "external");
      store.setField("face-2", "uValue", 0.25);
      const json = store.toJSON();
      expect(json).toHaveLength(2);
      const restored = AttributeStore.fromJSON(json);
      expect(restored.get("face-1")?.exposure).toBe("external");
      expect(restored.get("face-2")?.uValue).toBe(0.25);
    });

    it("ModelJson metadata roundtrip", () => {
      const g = new Model();
      g.metadata.setField("solid-a", "tag", "roof");
      const back = Model.fromJSON(g.toJSON());
      expect(back.metadata.get("solid-a")?.tag).toBe("roof");
    });
  });

  describe("@cad/js/core step roundtrip helpers", () => {
    it("stepEscape quotes apostrophes", () => {
      expect(stepEscape("a'b")).toBe("'a''b'");
    });

    it("stepNumber formats integers with trailing dot", () => {
      expect(stepNumber(3)).toBe("3.");
      expect(stepNumber(0.25)).toBe("0.25");
    });

    it("parseStepEntityMap reads DATA entities", () => {
      const text = "DATA;\n#10 = CARTESIAN_POINT('O', (0.,0.,0.));\nENDSEC;";
      const map = parseStepEntityMap(text);
      expect(map.get(10)).toContain("CARTESIAN_POINT");
    });

    it("emitSpatialUdaProperty roundtrips through parseSpatialUdaPayloads", () => {
      const writer = new StepEntityWriter();
      writer.emit(10, "GEOMETRIC_REPRESENTATION_CONTEXT(3)");
      emitSpatialUdaProperty(writer, 12, "spatial.modelspace", '{"revision":1}', "System_Generated");
      const file = assembleStepFile(stepSpatialFileHeader("t.stp", "2026-05-28T00:00:00Z"), writer);
      const uda = parseSpatialUdaPayloads(parseStepEntityMap(file));
      expect(JSON.parse(uda["spatial.modelspace"]!).revision).toBe(1);
    });
  });

  describe("@cad/js/core interactions", () => {
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
      const actions = ActionRegistry.withModelDefinitionActions();
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

  describe("@cad/js/core action and interaction registries", () => {
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
    it("loads model-definition actions from data-only JSON specs", () => {
      const specs = listModelDefinitionActionSpecs();
      const registry = ActionRegistry.withModelDefinitionActions();
      expect(specs.length).toBeGreaterThan(0);
      expect(specs.every((s) => registry.get(s.id)?.spec?.schema === "spatial.action/v1")).toBe(true);
      expect(specs.every((s) => registry.get(s.id) !== null)).toBe(true);
      expect(registry.get("command.finish")?.spec?.schema).toBe("spatial.action/v1");
      expect(registry.get("selection.selectAll")?.spec?.steps.some((s) => s.op === "kernel.call" && s.function === "spatial.selection.apply")).toBe(true);
      expect(registry.get("command.addPoint")?.spec?.steps.some((s) => s.op === "kernel.call" && s.function === "spatial.action.capability")).toBe(true);
      const allowedKernelFunctions = new Set(["spatial.selection.apply", "spatial.action.capability"]);
      expect(
        specs.every((spec) => spec.steps.every((step) => step.op !== "kernel.call" || allowedKernelFunctions.has(step.function))),
      ).toBe(true);
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
      const interactionIds = new Set(InteractionRegistry.withModelDefinitionInteractions().list().map((row) => row.id));
      for (const typology of listModelDefinitionTypologies()) {
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
          expect(typologyHasNativeConstructKit(typology), `${mdId} → ${typology.id}`).toBe(true);
          const ids = typologyConstructAssetIds(typology.id, typology.label);
          expect(modelDefinitionIdForInteraction(ids.interaction), `${mdId} → ${ids.interaction}`).toBe(mdId);
          for (const actionId of typologyConstructModeActionIds(typology.id, typology.label)) {
            expect(actionOwnedByModelDefinition(actionId, mdId), `${mdId} → ${typology.id} → ${actionId}`).toBe(true);
          }
          for (const actionId of typology.actions) {
            expect(actionOwnedByModelDefinition(actionId, mdId), `${mdId} → ${typology.id} → ${actionId}`).toBe(true);
          }
        }
        expect(listConstructableTypologiesForModelDefinition(mdId).length).toBe(listTypologiesForModelDefinition(mdId).length);
      }
    });
    it("typology constructFrom2PointsAndHeight adds an object row for the typology", async () => {
      const model = new Model();
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const typology = "energy.energy.hull";
      const ids = typologyConstructAssetIds(typology, "Hull");
      await ActionRegistry.withModelDefinitionActions().run(
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

    it("curve.interpolateCurve commit binds typology object rows for hierarchy", async () => {
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
      expect(model.objects[typology as ObjectRef]?.primitives.wire).toBeTruthy();
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
      const before = edge.curve?.kind === "nurbs" ? edge.curve.poles.map((pole) => [...pole] as Vec3) : [];
      applyModelDiff(model, selectionTargetsPointTransformDiff(model, [{ kind: "edge", id: edge.id }], (point) => [point[0] + 1, point[1], point[2]]));
      const updated = model.edges[edge.id]!;
      expect(updated.curve?.kind).toBe("nurbs");
      if (updated.curve?.kind === "nurbs") {
        expect(updated.curve.poles).toEqual(before.map((pole) => [pole[0] + 1, pole[1], pole[2]]));
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
      applyModelDiff(model, selectionTargetsPointTransformDiff(model, [{ kind: "vertex", id: startId }], (point) => [point[0], point[1] + 5, point[2]]));
      const updated = model.edges[edge.id]!;
      expect(updated.curve?.kind).toBe("nurbs");
      if (updated.curve?.kind === "nurbs" && midPole) {
        expect(updated.curve.poles[1]).toEqual(midPole);
      }
    });

    it("primitive.box commit binds typology object rows for hierarchy", async () => {
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
        schema: "spatial.interaction/v1",
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
                          op: "assign",
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
        schema: "spatial.interaction/v1",
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
                        { op: "assign", target: { root: "context", segments: [{ kind: "field", name: "constructMode" }] }, value: { kind: "const", value: "surface" } },
                        {
                          op: "interaction.call",
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
      const interactions = InteractionRegistry.withModelDefinitionInteractions();
      interactions.register(compileInteraction(pickChild));
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
        schema: "spatial.interaction/v1",
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
          operation: { kind: "action", action: "command.finish", params: {} },
        },
      })!;
      const child = parseInteractionSpec({
        schema: "spatial.interaction/v1",
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
                          op: "interaction.call",
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
          operation: { kind: "action", action: "command.finish", params: {} },
        },
      })!;
      const host = parseInteractionSpec({
        schema: "spatial.interaction/v1",
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
                          op: "interaction.call",
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
          operation: { kind: "action", action: "command.finish", params: {} },
        },
      })!;
      const reg = InteractionRegistry.withModelDefinitionInteractions();
      reg.register(compileInteraction(grandchild));
      reg.register(compileInteraction(child));
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
      const r = ActionRegistry.withModelDefinitionActions();
      const ids = new Set(r.list().map((d) => d.id));
      expect(ids.has("primitive.createBoxFromCorners")).toBe(true);
      expect(ids.has("box.aabbFromDiagonalCorners")).toBe(true);
      expect(ids.has("command.finish")).toBe(true);
      expect(ids.has("feature.offsetFaces")).toBe(true);
      expect(ids.has("selection.apply")).toBe(true);
      expect(ids.has("selection.selectAll")).toBe(true);
      expect(ids.has("selection.selectVertices")).toBe(true);
      expect(r.list().every((def) => def.spec !== undefined && def.run === undefined)).toBe(true);
    });
    it("register replaces a model-definition action id", () => {
      const r = ActionRegistry.withModelDefinitionActions();
      const before = r.get("measure.faceArea")?.label;
      r.register({
        id: "measure.faceArea",
        label: "override",
        run: () => ({ data: 99 }),
      });
      expect(r.get("measure.faceArea")?.label).toBe("override");
      expect(before).not.toBe("override");
    });
    it("InteractionRegistry.withModelDefinitionInteractions get matches buildBoxInteractionSpec", () => {
      const reg = InteractionRegistry.withModelDefinitionInteractions();
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
      await ActionRegistry.withModelDefinitionActions().run("primitive.createBoxFrom3Points", { p0, p1, p2, __context: {}, __event: { kind: "x" } }, { kernel: k as unknown as SpatialKernel, preview: M, model });
      expect(k.lastInput).toEqual({ cornerA: [0, 0, 0], cornerB: [2, 3, 0], height: 3 });
    });
    it("command.addSelection applies selection modifiers", async () => {
      const actions = ActionRegistry.withModelDefinitionActions();
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
      const actions = ActionRegistry.withModelDefinitionActions();
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
      const result = await ActionRegistry.withModelDefinitionActions().run(
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
      await ActionRegistry.withModelDefinitionActions().run(
        "selection.selectAll",
        { seedTargets: [], __context: {}, __event: { kind: "commit" } },
        { kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M, model },
      );
      expect(hist.entries()).toEqual([]);
    });
    it.each(listSelectionOperationsForModelDefinition(defaultModelDefinitionId()))("registers selection command action $id", (defn) => {
      expect(ActionRegistry.withModelDefinitionActions().get(defn.id)?.spec?.schema).toBe("spatial.action/v1");
    });
    it("selection.invert honors seed targets", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("e2e-box")));
      const seed = [{ kind: "solid", id: "e2e-box", editable: true }] as const;
      const result = await ActionRegistry.withModelDefinitionActions().run(
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
      const actions = ActionRegistry.withModelDefinitionActions();
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
      const actions = ActionRegistry.withModelDefinitionActions();
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
  describe("@cad/js/core model diff", () => {
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

    it("deleteObjectsFromModel removes object rows but keeps geometry primitives", () => {
      const g = new Model();
      applyModelDiff(g, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-solid")));
      g.objects["box-a"] = { id: "box-a" as ObjectRef, typology: "spatial.shape.primitive.box", primitives: { solid: "box-solid" } };
      g.objects["box-b"] = { id: "box-b" as ObjectRef, typology: "spatial.shape.primitive.box", primitives: { solid: "box-solid" } };
      const removed = deleteObjectsFromModel(g, ["box-a", "missing"]);
      expect(removed).toEqual(["box-a"]);
      expect(g.objects["box-a"]).toBeUndefined();
      expect(g.objects["box-b"]).toBeTruthy();
      expect(Object.keys(g.solids)).toEqual(["box-solid"]);
    });

    it("deletableObjectIdsFromSelection keeps only object targets", () => {
      const selection: SelectionTarget[] = [
        { kind: "object", id: "box-a", editable: true },
        { kind: "solid", id: "box-solid", editable: true },
        { kind: "object", id: "box-a", editable: true },
      ];
      expect(deletableObjectIdsFromSelection(selection)).toEqual(["box-a"]);
    });
  });
  describe("@cad/js/core selection filter", () => {
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

    it("expandSelectionTargetsForAccept maps object picks to wire primitives", () => {
      const model = new Model();
      const typology = "spatial.shape.curve.interpolate-curve";
      model.objects[typology as ObjectRef] = {
        id: typology as ObjectRef,
        typology: typology as TypologyRef,
        primitives: { wire: "w-interp" },
      };
      const spec: SelectionSpec = { accept: ["wire", "edge"], multiple: true };
      const expanded = expandSelectionTargetsForAccept(model, spec, [{ kind: "object", id: typology, editable: true }]);
      expect(expanded).toEqual([{ kind: "wire", id: "w-interp", editable: true }]);
    });
  });
  describe("@cad/js/core interaction box", () => {
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
        expect(heightSeg.params.to[2]).toBeCloseTo(3, 5);
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

  describe("@cad/js/core interaction length entry", () => {
    it("interactionLengthEntryForState resolves shipped line rubber-band", () => {
      const spec = requireSpatialInteraction("curve.line");
      expect(interactionLengthEntryForState(spec, "end_of_line")).toEqual({
        state: "end_of_line",
        anchor: "points.start",
        field: "cursor",
      });
    });

    it("readInteractionContextVec3 supports points.@last on arrays", () => {
      const ctx = { points: [[0, 0, 0], [1, 2, 3]] as Vec3[] };
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
      const distance = interactionLengthEntryLiveDistance(
        { origin: [0, 0, 0] as Vec3, cursor: [0, 0, 1.25] as Vec3, direction: [0, 0, 1] as Vec3 },
        entry,
      );
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
        expect(ev.point[0]).toBeCloseTo(4, 5);
        expect(ev.point[1]).toBeCloseTo(0, 5);
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

  describe("@cad/js/core stateEngine option", () => {
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

  describe("@cad/js/core measure distance", () => {
    it("measure.faceArea action adds face anchor geometry", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("m-area")));
      const fid = Object.keys(model.faces)[0]! as FaceRef;
      const r = await ActionRegistry.withModelDefinitionActions().run("measure.faceArea", { faceId: fid }, { model: model, kernel: new BrepjsKernel() as unknown as SpatialKernel, preview: M });
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

  describe("@cad/js/core measure area", () => {
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

  describe("@cad/js/core document history", () => {
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

  describe("@cad/js/core measure distance history", () => {
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

  describe("@cad/js/core interaction session undo redo", () => {
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

  describe("@cad/js/core undo routing", () => {
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

  describe("@cad/js/core box display committed", () => {
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

  describe("@cad/js/core interaction e2e fixtures", () => {
    type InteractionE2EFixtureKind = "loom" | "routes" | "building" | "empty";

    const MOD: InteractionEvent["modifiers"] = {};

    const p = (x: number, y: number, z = 0): Vec3 => [x, y, z];

    const sel = (kind: ModelEntityKind, id: string, editable = true): SelectionTarget => ({
      kind,
      id,
      editable: kind === "surface" || kind === "part" ? false : editable,
    });

    const modelFromFixture = (kind: InteractionE2EFixtureKind): Model => {
      if (kind === "empty") return new Model();
      const raw = kind === "loom" ? geometryLoomFixtureJson : kind === "routes" ? geometryRoutesFixtureJson : smallBuildingModelFixtureJson;
      if (raw && typeof raw === "object" && (raw as ModelSpaceJson).schema === "spatial.modelspace/v1") {
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
      readonly assert?: (ctx: {
        readonly snap: InteractionSnapshot;
        readonly model: Model;
        readonly before: ReturnType<typeof entityCounts>;
        readonly after: ReturnType<typeof entityCounts>;
        readonly activeModelDefinitionId?: string | null;
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
          typology: "spatial.shape.primitive.box",
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
// #endregion 🧪Tests

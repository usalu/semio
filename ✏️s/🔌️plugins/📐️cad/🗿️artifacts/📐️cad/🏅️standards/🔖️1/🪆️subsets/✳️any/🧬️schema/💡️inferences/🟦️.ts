/** 💡️ Cad inference schema — object/vertex counts + 3d bounding box across every pane. */

export interface CadBounds {
  min: [number, number, number];
  max: [number, number, number];
}

export interface CadInference {
  /** @derived */
  objectCount: number;
  /** @derived */
  vertexCount: number;
  /** @derived */
  bounds: CadBounds | null;
}

// #region 🔍️ConstructQueryLanguage
/** 🔍️ Promoted from the dissolved artifact ⚙️engine (D4 — derived compute over a Model snapshot, no mutable state): Cypher-inspired `construct` query/transformation DSL. */
// #region 🧲️Header
/** 🔍️ `@semio-tech/cad-js/query` — Cypher-inspired `construct` language: `MATCH (Object {typology: '…'})`, `KernelIndex` on `Model`, typed execution of `ConstructAst` (view `CALL` removed). */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { kernelGeometry, type Vec3 } from "@semio-tech/framework-3d-js";
import {
  Model,
  buildTypologyToEntityKindMapForModelDefinition,
  applyTransformation,
  loadTransformation,
  actionAvailableInModelDefinition,
  defaultModelDefinitionId,
  type Expr,
  type ExprBinop,
  type ExprField,
  type ExprVar,
  type ModelEntityKind,
  type ModelEntityRef,
  type ObjectRef,
  type AttributeTable,
  type TypologyRef,
  type SelectionTarget,
  evalExpr,
  type ExprEnv,
} from "../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts";
import { applyModelDiff, EMPTY_MODEL_DIFF, type SpatialKernel, type ModelDiff } from "../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️.ts";
import {
  type ActionRegistry,
  runRegisteredAction,
  type ActionResult,
  isSelectionConstructActionId,
  type ConstructQueryContext,
  type ConstructQueryResult,
  type ConstructQueryRow,
} from "../../✏️editor/⚙️engine/🎬️actions/🟦️.ts";

type SolidRef = kernelGeometry.SolidRef;
type FaceRef = kernelGeometry.FaceRef;
type ShellRef = kernelGeometry.ShellRef;
type WireRef = kernelGeometry.WireRef;
type EdgeRef = kernelGeometry.EdgeRef;
type VertexRef = kernelGeometry.VertexRef;
// #endregion 🔌️Adapters

// #region Ast
export interface NodePatternAst {
  readonly kind: "node";
  readonly var?: string;
  readonly label?: string;
  readonly props?: Record<string, unknown>;
}

export interface RelPatternAst {
  readonly kind: "rel";
  readonly types: readonly string[];
  readonly direction: "->" | "<-" | "--";
}

export type PatternElementAst = NodePatternAst | RelPatternAst;

export interface PatternAst {
  readonly elements: readonly PatternElementAst[];
}

export interface MatchClauseAst {
  readonly kind: "match";
  readonly patterns: readonly PatternAst[];
  readonly where?: Expr;
}

export interface WithClauseAst {
  readonly kind: "with";
  readonly projections: readonly { readonly expr: Expr; readonly alias?: string }[];
  readonly where?: Expr;
}

export interface YieldItemAst {
  readonly key: string;
  readonly alias?: string;
}

export interface CallClauseAst {
  readonly kind: "call";
  readonly actionId: string;
  readonly args: Readonly<Record<string, unknown>>;
  readonly yieldItems: readonly YieldItemAst[];
}

export interface UnwindClauseAst {
  readonly kind: "unwind";
  readonly source: Expr;
  readonly alias: string;
  readonly where?: Expr;
}

export interface ReturnClauseAst {
  readonly kind: "return";
  readonly projections: readonly { readonly expr: Expr; readonly alias?: string }[];
  readonly orderBy?: Expr;
  readonly limit?: number;
}

export type ConstructClauseAst = MatchClauseAst | WithClauseAst | CallClauseAst | UnwindClauseAst;

/** 🧭️ Checks model typology meaning on a first-party Construct AST. */
export function assertConstructAst(ast: ConstructAst, activeModelDefinitionId?: string | null): void {
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  const typologyKinds = buildTypologyToEntityKindMapForModelDefinition(mdId);
  for (const cl of ast.clauses) {
    if (cl.kind !== "match") continue;
    for (const pat of cl.patterns) {
      for (const el of pat.elements) {
        if (el.kind !== "node") continue;
        if (el.label && el.label !== "Object") {
          throw new Error(`unknown node label ${el.label}; use Object {typology: '…'}`);
        }
        const typology = el.props?.typology;
        if (typeof typology === "string" && (typology.includes(".derived.") || typology.startsWith("view."))) {
          throw new Error(`${typology} is not a shipped model typology`);
        }
        if (typeof typology === "string" && !(typology in typologyKinds)) {
          throw new Error(`unknown typology ${typology} for model definition ${mdId}; see listTypologiesForModelDefinition()`);
        }
      }
    }
  }
}

/** 🔍️ Resolves one `YIELD` key (supports dot paths into `data`) from an `ActionResult`. */
export function resolveActionYield(result: ActionResult, key: string): unknown {
  if (!key.includes(".")) {
    if (key === "diff") return result.diff;
    if (key === "data") return result.data;
    if (key === "patch") return result.patch;
    if (key === "targets") {
      const patchSet = result.patch?.set as { targets?: unknown } | undefined;
      if (patchSet?.targets !== undefined) return patchSet.targets;
      const data = result.data as { targets?: unknown } | undefined;
      return data?.targets;
    }
    return (result as Record<string, unknown>)[key];
  }
  const [head, ...rest] = key.split(".");
  let cur: unknown = head === "data" ? result.data : head === "diff" ? result.diff : head === "patch" ? result.patch : (result as Record<string, unknown>)[head];
  for (const seg of rest) {
    if (cur === null || cur === undefined || typeof cur !== "object") return undefined;
    cur = (cur as Record<string, unknown>)[seg];
  }
  return cur;
}

export interface ConstructAst {
  readonly clauses: readonly ConstructClauseAst[];
  readonly returnClause?: ReturnClauseAst;
}

// #endregion Ast

// #region Index
function typologyToEntityKindForConstruct(activeModelDefinitionId?: string | null): Readonly<Record<string, ModelEntityKind>> {
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  return buildTypologyToEntityKindMapForModelDefinition(mdId);
}

function patternEntityKind(node: NodePatternAst, activeModelDefinitionId?: string | null): ModelEntityKind | undefined {
  if (node.label && node.label !== "Object") return undefined;
  const typology = node.props?.typology;
  if (typeof typology !== "string") return undefined;
  return typologyToEntityKindForConstruct(activeModelDefinitionId)[typology];
}

function objectHandlesForTypology(model: Model, typology: string): EntityHandle[] {
  return Object.values(model.objects)
    .filter((row) => row.typology === typology)
    .map((row) => ({ kind: "object" as ModelEntityKind, id: String(row.id) }));
}

export class KernelIndex {
  private revisionAt = -1;
  private readonly byKind = new Map<ModelEntityKind, Set<string>>();
  private readonly faceToSolids = new Map<string, Set<string>>();
  private readonly edgeToFaces = new Map<string, Set<string>>();

  constructor(private readonly model: Model) {}

  private rebuild(): void {
    this.byKind.clear();
    this.faceToSolids.clear();
    this.edgeToFaces.clear();
    const add = (k: ModelEntityKind, id: string) => {
      let s = this.byKind.get(k);
      if (!s) {
        s = new Set();
        this.byKind.set(k, s);
      }
      s.add(id);
    };
    for (const id of Object.keys(this.model.anchors)) add("anchor", id);
    for (const id of Object.keys(this.model.vertices)) add("vertex", id);
    for (const id of Object.keys(this.model.edges)) add("edge", id);
    for (const id of Object.keys(this.model.wires)) add("wire", id);
    for (const id of Object.keys(this.model.faces)) add("face", id);
    for (const id of Object.keys(this.model.shells)) add("shell", id);
    for (const id of Object.keys(this.model.solids)) add("solid", id);
    for (const id of Object.keys(this.model.objects)) add("object", id);
    for (const [sid, solid] of Object.entries(this.model.solids)) {
      for (const shellId of solid.shellIds) {
        const sh = this.model.shells[shellId];
        if (!sh) continue;
        for (const fid of sh.faceIds) {
          let xs = this.faceToSolids.get(fid);
          if (!xs) {
            xs = new Set();
            this.faceToSolids.set(fid, xs);
          }
          xs.add(sid);
        }
      }
    }
    for (const [fid, face] of Object.entries(this.model.faces)) {
      for (const wid of face.wireIds) {
        const w = this.model.wires[wid];
        if (!w) continue;
        for (const eid of w.edgeIds) {
          let fs = this.edgeToFaces.get(eid);
          if (!fs) {
            fs = new Set();
            this.edgeToFaces.set(eid, fs);
          }
          fs.add(fid);
        }
      }
    }
    this.revisionAt = this.model.revision;
  }

  ensure(): void {
    if (this.revisionAt !== this.model.revision) this.rebuild();
  }

  idsForKind(k: ModelEntityKind): readonly string[] {
    this.ensure();
    return [...(this.byKind.get(k) ?? [])];
  }

  lookupById(id: string): { kind: ModelEntityKind; id: string } | null {
    this.ensure();
    for (const [k, s] of this.byKind) {
      if (s.has(id)) return { kind: k, id };
    }
    return null;
  }

  selectivityScore(node: NodePatternAst): number {
    if (node.props?.id !== undefined) return 0;
    if (patternEntityKind(node)) return 1;
    return 2;
  }

  adjacentSolidIds(solidId: string): Set<string> {
    this.ensure();
    const out = new Set<string>();
    const solid = this.model.solids[solidId];
    if (!solid) return out;
    const faces = new Set<string>();
    for (const shellId of solid.shellIds) {
      const sh = this.model.shells[shellId];
      if (!sh) continue;
      for (const f of sh.faceIds) faces.add(f);
    }
    for (const f of faces) {
      for (const other of this.faceToSolids.get(f) ?? []) {
        if (other !== solidId) out.add(other);
      }
    }
    return out;
  }

  edgeIncidentFaceCount(edgeId: string): number {
    this.ensure();
    return this.edgeToFaces.get(edgeId)?.size ?? 0;
  }

  facesForEdge(edgeId: string): readonly string[] {
    this.ensure();
    return [...(this.edgeToFaces.get(edgeId) ?? [])];
  }
}
// #endregion Index

// #region Traversers
export type EntityHandle = ModelEntityRef;

function* iterateBoundedBy(model: Model, from: EntityHandle): Generator<EntityHandle> {
  if (from.kind === "face") {
    const f = model.faces[from.id];
    if (!f) return;
    for (const w of f.wireIds) yield { kind: "wire", id: w };
  } else if (from.kind === "solid") {
    const c = model.solids[from.id];
    if (!c) return;
    for (const s of c.shellIds) yield { kind: "shell", id: s };
  } else if (from.kind === "shell") {
    const s = model.shells[from.id];
    if (!s) return;
    for (const f of s.faceIds) yield { kind: "face", id: f };
  } else if (from.kind === "wire") {
    const w = model.wires[from.id];
    if (!w) return;
    for (const e of w.edgeIds) yield { kind: "edge", id: e };
  } else if (from.kind === "edge") {
    const e = model.edges[from.id];
    if (!e) return;
    for (const v of e.vertexIds) yield { kind: "vertex", id: v };
  }
}

function* iterateContainsInverse(model: Model, from: EntityHandle): Generator<EntityHandle> {
  if (from.kind === "face") {
    for (const [sid, sh] of Object.entries(model.shells)) {
      if (sh.faceIds.includes(from.id as FaceRef)) yield { kind: "shell", id: sid };
    }
  } else if (from.kind === "shell") {
    for (const [cid, c] of Object.entries(model.solids)) {
      if (c.shellIds.includes(from.id as ShellRef)) yield { kind: "solid", id: cid };
    }
  } else if (from.kind === "wire") {
    for (const [fid, fa] of Object.entries(model.faces)) {
      if (fa.wireIds.includes(from.id as WireRef)) yield { kind: "face", id: fid };
    }
  } else if (from.kind === "edge") {
    for (const [wid, w] of Object.entries(model.wires)) {
      if (w.edgeIds.includes(from.id as EdgeRef)) yield { kind: "wire", id: wid };
    }
  } else if (from.kind === "vertex") {
    for (const [eid, e] of Object.entries(model.edges)) {
      if (e.vertexIds.includes(from.id as VertexRef)) yield { kind: "edge", id: eid };
    }
  }
}

function* iterateContainsForward(model: Model, from: EntityHandle): Generator<EntityHandle> {
  if (from.kind === "solid") {
    const c = model.solids[from.id];
    if (!c) return;
    for (const shellId of c.shellIds) yield { kind: "shell", id: shellId };
  } else if (from.kind === "shell") {
    const s = model.shells[from.id];
    if (!s) return;
    for (const fid of s.faceIds) yield { kind: "face", id: fid };
  } else if (from.kind === "face") {
    const f = model.faces[from.id];
    if (!f) return;
    for (const wid of f.wireIds) yield { kind: "wire", id: wid };
  } else if (from.kind === "wire") {
    const w = model.wires[from.id];
    if (!w) return;
    for (const eid of w.edgeIds) yield { kind: "edge", id: eid };
  }
}

function lookupAnyEntity(model: Model, id: string): EntityHandle | null {
  if (model.anchors[id]) return { kind: "anchor", id };
  if (model.vertices[id]) return { kind: "vertex", id };
  if (model.edges[id]) return { kind: "edge", id };
  if (model.wires[id]) return { kind: "wire", id };
  if (model.faces[id]) return { kind: "face", id };
  if (model.shells[id]) return { kind: "shell", id };
  if (model.solids[id]) return { kind: "solid", id };
  return null;
}

function* iterateShares(model: Model, index: KernelIndex, from: EntityHandle): Generator<EntityHandle> {
  if (from.kind === "edge") {
    for (const fid of index.facesForEdge(from.id)) yield { kind: "face", id: fid };
  } else if (from.kind === "vertex") {
    for (const e of iterateContainsInverse(model, from)) yield e;
  }
}

function* traverseRel(model: Model, _kernel: SpatialKernel, index: KernelIndex, from: EntityHandle, relTypes: readonly string[], direction: RelPatternAst["direction"]): Generator<EntityHandle> {
  const rel = relTypes[0] ?? "BOUNDED_BY";
  const both = direction === "--";
  const out = direction === "->" || both;
  const inn = direction === "<-" || both;
  const forward =
    rel === "BOUNDED_BY"
      ? iterateBoundedBy(model, from)
      : rel === "CONTAINS"
        ? out
          ? iterateContainsForward(model, from)
          : iterateContainsInverse(model, from)
        : rel === "SHARES"
          ? iterateShares(model, index, from)
          : rel === "ADJACENT_TO" && from.kind === "solid"
            ? (function* (): Generator<EntityHandle> {
                for (const id of index.adjacentSolidIds(from.id)) yield { kind: "solid", id };
              })()
            : rel === "HAS_VERTEX"
              ? (function* (): Generator<EntityHandle> {
                  let frontier: EntityHandle[] = [from];
                  for (let depth = 0; depth < 8; depth++) {
                    const next: EntityHandle[] = [];
                    for (const x of frontier) {
                      for (const y of iterateBoundedBy(model, x)) {
                        if (y.kind === "vertex") yield y;
                        else next.push(y);
                      }
                    }
                    frontier = next;
                  }
                })()
              : (function* (): Generator<EntityHandle> {})();
  if (out && !inn) return yield* forward;
  if (inn && !out) {
    if (rel === "CONTAINS") return yield* iterateContainsInverse(model, from);
    return yield* forward;
  }
  yield* forward;
}
// #endregion Traversers

// #region Planner
export type PlanStepAst =
  | { readonly kind: "match"; readonly pattern: PatternAst; readonly where?: Expr }
  | { readonly kind: "with"; readonly projections: readonly { readonly expr: Expr; readonly alias?: string }[]; readonly where?: Expr }
  | { readonly kind: "call"; readonly actionId: string; readonly args: Readonly<Record<string, unknown>>; readonly yieldItems: readonly YieldItemAst[] }
  | { readonly kind: "unwind"; readonly source: Expr; readonly alias: string; readonly where?: Expr };

export interface ExecutionPlan {
  readonly steps: readonly PlanStepAst[];
  readonly returnClause?: ReturnClauseAst;
}

/** 🧭️ Flattens `MATCH` comma patterns into sequential steps (cartesian product handled in executor). */
export function planConstruct(ast: ConstructAst): ExecutionPlan {
  const steps: PlanStepAst[] = [];
  for (const cl of ast.clauses) {
    if (cl.kind === "match") {
      for (const p of cl.patterns) steps.push({ kind: "match", pattern: p, ...(cl.where ? { where: cl.where } : {}) });
    } else if (cl.kind === "with") steps.push({ kind: "with", projections: cl.projections, ...(cl.where ? { where: cl.where } : {}) });
    else if (cl.kind === "call") steps.push({ kind: "call", actionId: cl.actionId, args: cl.args, yieldItems: cl.yieldItems });
    else if (cl.kind === "unwind") steps.push({ kind: "unwind", source: cl.source, alias: cl.alias, ...(cl.where ? { where: cl.where } : {}) });
  }
  return { steps, ...(ast.returnClause ? { returnClause: ast.returnClause } : {}) };
}
// #endregion Planner

// #region Executor
type Row = Record<string, ModelEntityRef | unknown>;

function rowVarsToEnv(row: Row, model: Model, meta: AttributeTable, preview: SpatialKernel, activeModelDefinitionId?: string | null): ExprEnv {
  const vars: Record<string, unknown> = {};
  for (const [k, v] of Object.entries(row)) {
    if (v && typeof v === "object" && "kind" in (v as object) && "id" in (v as object)) vars[k] = v;
    else vars[k] = v;
  }
  return { context: {}, vars, model, metadata: meta, activeModelDefinitionId, preview };
}

/** 🔄️ Runs a shipped transformation against the construct context model. */
function runTransformationCall(actionId: string, ctx: ConstructQueryContext): ActionResult {
  const spec = loadTransformation(actionId);
  if (!spec) throw new Error(`unknown transformation ${actionId}`);
  const model = applyTransformation(spec, ctx.model, ctx.kernel);
  const objects = Object.keys(model.objects)
    .sort()
    .map((key) => model.objects[key]!);
  return { diff: EMPTY_MODEL_DIFF, data: { model, objects } };
}

function* expandPattern(model: Model, kernel: SpatialKernel, index: KernelIndex, pat: PatternAst, activeModelDefinitionId?: string | null): Generator<Row> {
  const els = pat.elements;
  if (!els.length) return;
  const first = els[0] as NodePatternAst;
  const startVar = first.var ?? "__n0";
  const startKind = patternEntityKind(first, activeModelDefinitionId);
  const startTypology = first.props?.typology;
  const idProp = first.props?.id;
  index.ensure();
  let seeds: EntityHandle[] = [];
  if (typeof idProp === "string" && startKind) seeds = [{ kind: startKind, id: idProp }];
  else if (typeof idProp === "string") {
    const lk = index.lookupById(idProp);
    if (lk) seeds = [lk];
  } else if (typeof startTypology === "string" && startKind === "object") {
    seeds = objectHandlesForTypology(model, startTypology);
  } else if (startKind) seeds = index.idsForKind(startKind).map((id) => ({ kind: startKind, id }));
  else seeds = [];

  function* expandFrom(j: number, row: Row): Generator<Row> {
    if (j >= els.length) {
      yield { ...row };
      return;
    }
    const el = els[j]!;
    if (el.kind === "node") {
      const nm = el.var ?? `__n${j}`;
      const kind = patternEntityKind(el, activeModelDefinitionId);
      const pid = el.props?.id;
      const typology = el.props?.typology;
      if (j === 0) {
        for (const s of seeds) {
          if (kind && s.kind !== kind) continue;
          if (typeof pid === "string" && s.id !== pid) continue;
          if (typeof typology === "string" && s.kind === "object") {
            const obj = model.objects[s.id];
            if (!obj || obj.typology !== typology) continue;
          }
          yield* expandFrom(j + 1, { ...row, [nm]: s });
        }
      }
      return;
    }
    const rel = el as RelPatternAst;
    const prevNode = els[j - 1] as NodePatternAst;
    const prevName = prevNode.var ?? `__n${j - 1}`;
    const from = row[prevName] as EntityHandle;
    const nextNode = els[j + 1] as NodePatternAst;
    const nm = nextNode.var ?? `__n${j + 1}`;
    const kind = patternEntityKind(nextNode, activeModelDefinitionId);
    const pid = nextNode.props?.id;
    const typology = nextNode.props?.typology;
    for (const x of traverseRel(model, kernel, index, from, rel.types, rel.direction)) {
      if (kind && x.kind !== kind) continue;
      if (typeof pid === "string" && x.id !== pid) continue;
      if (typeof typology === "string" && x.kind === "object") {
        const obj = model.objects[x.id];
        if (!obj || obj.typology !== typology) continue;
      }
      yield* expandFrom(j + 2, { ...row, [nm]: x });
    }
  }
  yield* expandFrom(0, {});
}

async function* executeConstruct(plan: ExecutionPlan, ctx: ConstructQueryContext): AsyncIterable<ConstructQueryRow> {
  const index = new KernelIndex(ctx.model);
  let rows: Row[] = [{}];
  for (const st of plan.steps) {
    if (st.kind === "match") {
      const next: Row[] = [];
      for (const r of rows) {
        for (const row of expandPattern(ctx.model, ctx.kernel, index, st.pattern, ctx.activeModelDefinitionId)) {
          const merged = { ...r, ...row };
          if (st.where) {
            const ok = evalExpr(st.where, rowVarsToEnv(merged, ctx.model, ctx.model.metadata, ctx.kernel, ctx.activeModelDefinitionId));
            if (!ok) continue;
          }
          next.push(merged);
        }
      }
      rows = next;
    } else if (st.kind === "with") {
      const next: Row[] = [];
      for (const r of rows) {
        const env = rowVarsToEnv(r, ctx.model, ctx.model.metadata, ctx.kernel, ctx.activeModelDefinitionId);
        const out: Row = { ...r };
        for (const p of st.projections) {
          const v = evalExpr(p.expr, env);
          if (p.alias) out[p.alias] = v;
        }
        if (st.where) {
          const ok = evalExpr(st.where, rowVarsToEnv(out, ctx.model, ctx.model.metadata, ctx.kernel, ctx.activeModelDefinitionId));
          if (!ok) continue;
        }
        next.push(out);
      }
      rows = next;
    } else if (st.kind === "call") {
      const transformation = loadTransformation(st.actionId);
      const modelDefinitionId = ctx.activeModelDefinitionId ?? defaultModelDefinitionId();
      if (!transformation && !ctx.actions.get(st.actionId)) throw new Error(`unknown action ${st.actionId}`);
      if (!transformation && !actionAvailableInModelDefinition(st.actionId, modelDefinitionId)) {
        throw new Error(`action ${st.actionId} is not available in model definition ${modelDefinitionId}`);
      }
      const next: Row[] = [];
      for (const r of rows) {
        const paramBag: Record<string, unknown> = { __context: {}, __event: { kind: "construct.call" }, ...st.args };
        if (isSelectionConstructActionId(st.actionId) && paramBag.seedTargets === undefined && ctx.selectionTargets && ctx.selectionTargets.length > 0) {
          paramBag.seedTargets = ctx.selectionTargets;
        }
        const res = transformation
          ? runTransformationCall(st.actionId, ctx)
          : await runRegisteredAction(ctx.actions, st.actionId, paramBag, {
              kernel: ctx.kernel,
              preview: ctx.kernel,
              model: ctx.model,
              activeModelDefinitionId: ctx.activeModelDefinitionId ?? null,
            });
        const nr = { ...r };
        for (const y of st.yieldItems) {
          const v = resolveActionYield(res, y.key);
          if (v !== undefined) nr[y.alias ?? y.key] = v;
        }
        next.push(nr);
      }
      rows = next;
    } else if (st.kind === "unwind") {
      const next: Row[] = [];
      for (const r of rows) {
        const env = rowVarsToEnv(r, ctx.model, ctx.model.metadata, ctx.kernel, ctx.activeModelDefinitionId);
        const src = evalExpr(st.source, env);
        if (!Array.isArray(src)) continue;
        for (const item of src) {
          const merged: Row = { ...r, [st.alias]: item };
          if (st.where) {
            const ok = evalExpr(st.where, rowVarsToEnv(merged, ctx.model, ctx.model.metadata, ctx.kernel, ctx.activeModelDefinitionId));
            if (!ok) continue;
          }
          next.push(merged);
        }
      }
      rows = next;
    }
  }
  const ret = plan.returnClause;
  if (!ret) {
    for (const r of rows) yield { ...r };
    return;
  }
  let out = rows;
  if (ret.limit !== undefined) out = out.slice(0, ret.limit);
  for (const r of out) {
    const env = rowVarsToEnv(r, ctx.model, ctx.model.metadata, ctx.kernel, ctx.activeModelDefinitionId);
    const o: Record<string, unknown> = {};
    for (let i = 0; i < ret.projections.length; i++) {
      const p = ret.projections[i]!;
      const k = p.alias ?? `c${i}`;
      o[k] = evalExpr(p.expr, env);
    }
    yield o;
  }
}
// #endregion Executor

// #region Api
/** 🔍️ Materializes `executeConstruct` into `ConstructQueryResult`. */
export async function runConstructAst(ast: ConstructAst, ctx: ConstructQueryContext): Promise<ConstructQueryResult> {
  assertConstructAst(ast, ctx.activeModelDefinitionId);
  const plan = planConstruct(ast);
  const rows: ConstructQueryRow[] = [];
  let data: unknown;
  let diff: ModelDiff | undefined;
  for await (const row of executeConstruct(plan, ctx)) {
    rows.push(row);
    if (row.data !== undefined) data = row.data;
    if (row.diff !== undefined) diff = row.diff as ModelDiff;
  }
  return { rows, ...(data !== undefined ? { data } : {}), ...(diff !== undefined ? { diff } : {}) };
}


/** 🔍️ Cached `KernelIndex` wrapper for repeated `construct` scripts on one document revision. */
export class ConstructEngine {
  private index: KernelIndex | null = null;
  private rev = -1;

  constructor(private readonly model: Model) {}

  private ix(): KernelIndex {
    if (!this.index || this.rev !== this.model.revision) {
      this.index = new KernelIndex(this.model);
      this.rev = this.model.revision;
    }
    return this.index;
  }

  /** 🧭️ Ensures `KernelIndex` matches current `model.revision` (side-effect on cache). */
  warmIndex(): void {
    this.ix().ensure();
  }
}
// #endregion Api

// #endregion 🔍️ConstructQueryLanguage

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class cadCadInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const cadCadInferenceGuardReject = (at: string, why: string): never => {
  throw new cadCadInferenceGuardRefusal(at, why);
};

type cadCadInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type cadCadInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type cadCadInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const cadCadInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : cadCadInferenceGuardReject(at, "value is not an object");
export const cadCadInferenceGuardArray = (value: unknown, at: string, bounds: cadCadInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return cadCadInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) cadCadInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) cadCadInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const cadCadInferenceGuardString = (value: unknown, at: string, bounds: cadCadInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return cadCadInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) cadCadInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) cadCadInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) cadCadInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const cadCadInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : cadCadInferenceGuardReject(at, "value is not a boolean"));
export const cadCadInferenceGuardNumber = (value: unknown, at: string, bounds: cadCadInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return cadCadInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) cadCadInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) cadCadInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const cadCadInferenceGuardInteger = (value: unknown, at: string, bounds: cadCadInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? cadCadInferenceGuardNumber(value, at, bounds) : cadCadInferenceGuardReject(at, "value is not an integer");
export const cadCadInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : cadCadInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const cadCadInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : cadCadInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

/** 📐️ One `[x, y, z]` corner the schema admits as exactly three finite numbers. */
const parseCadBoundsCorner = (value: unknown, at: string): [number, number, number] => {
  const [x, y, z] = cadCadInferenceGuardArray(value, at, {"minItems": 3, "maxItems": 3}).map((item, index) => cadCadInferenceGuardNumber(item, `${at}[${index}]`));
  return [x, y, z];
};

export function parseCadBounds(value: unknown, at = "$"): CadBounds {
  const row = cadCadInferenceGuardObject(value, at);
  return {
    min: parseCadBoundsCorner(row["min"], `${at}.min`),
    max: parseCadBoundsCorner(row["max"], `${at}.max`),
  };
}

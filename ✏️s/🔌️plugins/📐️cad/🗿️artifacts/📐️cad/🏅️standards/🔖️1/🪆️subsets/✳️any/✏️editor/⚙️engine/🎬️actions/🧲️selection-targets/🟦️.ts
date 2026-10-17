/** 🧲️ CAD editor actions: the geometry targets of a selection, the gumball configuration, the interaction point snaps and the selection operations. */
import type { Vec3 } from "@semio-tech/framework/3d";
import { FaceRecord, ModelEntityKind, VertexRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { defaultModelDefinitionId } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { PRIMITIVE_MODEL_ENTITY_KINDS, listModelObjectsForModelDefinition } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-ownership";
import { modelDefinitionSelectionEntityKinds } from "@semio-tech/s-spatial-kernel-ts/geometry/selection-operations";
import { InteractionEvent, SelectionTarget } from "@semio-tech/s-spatial-kernel-ts/geometry/selection";
import { EMPTY_MODEL_DIFF, EdgeRecordDiff, ModelDiff, SpatialPreviewKernel, VertexRecordDiff, isEmptyModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { ActionResult } from "../🧬️action-contract/🟦️.ts";

// #region 📦️🎬️actions
// #region 🧮️ActionRegistry
/** 📍️ Centroid of a face boundary for measure/annotation anchors. */
export function faceAnnotationCentroid(model: Model, face: FaceRecord): Vec3 | null {
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

/** 📏️ Measure and selection commands do not enter document undo history. */
export function interactionRecordsDocumentHistory(interactionId: string): boolean {
  return !interactionId.startsWith("measure.") && !interactionId.startsWith("selection.");
}

/** 🎯️ Collects vertex ids reachable from transform/edit selection targets. */
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

/** 🎯️ Collects edge ids when topology (edge/wire/face/…) is selected; excludes vertex-only picks. */
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

/** 📦️ Center of the axis-aligned bounds of all vertices in `targets`. */
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

/** 🎛️ CAD play gumball visibility groups (same shape as ui UnifiedGumball config). */
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

/** 🎛️ Toggle keys for CAD play gumball window measures. */
export type CadGumballGroupKey = keyof Pick<CadGumballConfig, "moveAxes" | "movePlanes" | "rotate" | "scaleAxes" | "scalePlanes" | "scaleUniform">;

/** 🎛️ Ordered gumball group toggles for CAD play window measures. */
export const CAD_GUMBALL_GROUPS: readonly { readonly key: CadGumballGroupKey; readonly label: string }[] = [
  { key: "moveAxes", label: "Move Axes" },
  { key: "movePlanes", label: "Move Planes" },
  { key: "rotate", label: "Rotate" },
  { key: "scaleAxes", label: "Scale Axes" },
  { key: "scalePlanes", label: "Scale Planes" },
  { key: "scaleUniform", label: "Scale Uniform" },
];

/** 🎛️ Default CAD play gumball state (hidden until a group is enabled). */
export const CAD_GUMBALL_HIDDEN: CadGumballConfig = {
  moveAxes: false,
  movePlanes: false,
  rotate: false,
  scaleAxes: false,
  scalePlanes: false,
  scaleUniform: false,
};

/** 🎛️ True when at least one gumball handle group is enabled. */
export function cadGumballConfigVisible(config: CadGumballConfig | null | undefined): boolean {
  if (!config) return false;
  return config.moveAxes !== false || config.movePlanes !== false || config.rotate !== false || config.scaleAxes !== false || config.scalePlanes !== false || config.scaleUniform !== false;
}

/** ✋️ True when `targets` resolve to at least one model vertex. */
export function selectionTargetsHaveTransformableVertices(model: Model, targets: readonly SelectionTarget[]): boolean {
  return collectTargetVertices(model, targets).size > 0;
}

function modelDiffTransformNurbsPolesOnEdges(model: Model, edgeIds: Iterable<string>, mapPoint: (point: Vec3) => Vec3): ModelDiff {
  const edgeMods: EdgeRecordDiff[] = [];
  for (const id of edgeIds) {
    const edge = model.edges[id];
    const curve = edge?.curve;
    if (curve?.kind !== "nurbs" || curve.poles.length < 2) continue;
    const poles = curve.poles.map((pole: Vec3) => mapPoint(pole));
    if (poles.every((pole: Vec3, index: number) => vec3Eq(pole, curve.poles[index]!))) continue;
    edgeMods.push({ id: edge.id, curve: { ...curve, poles } });
  }
  return edgeMods.length ? { edges: { modified: edgeMods } } : EMPTY_MODEL_DIFF;
}

/** 🎛️ Applies `mapPoint` to vertices and nurbs poles on topology-selected edges. */
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

export function vertexPositionsTransformDiff(model: Model, targets: readonly SelectionTarget[], mapPoint: (point: Vec3) => Vec3): ModelDiff {
  return selectionTargetsPointTransformDiff(model, targets, mapPoint);
}

// #endregion 🧮️ActionRegistry
// #region 📍️InteractionPointBinding
/** 📍️ Optional geometry snap stored beside a committed interaction point. */
export type InteractionPointSnap = { readonly kind: string; readonly id: string };

/** 📍️ Reads parallel `pointSnaps` rows aligned with `context.points`. */
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

export function interactionPointSnapFromEvent(event: InteractionEvent | undefined): InteractionPointSnap | null {
  if (!event || typeof event !== "object") return null;
  const snap = (event as { snap?: { kind?: unknown; id?: unknown } }).snap;
  if (!snap || typeof snap.kind !== "string" || typeof snap.id !== "string") return null;
  return { kind: snap.kind, id: snap.id };
}

function vec3Eq(a: Vec3, b: Vec3): boolean {
  return a[0] === b[0] && a[1] === b[1] && a[2] === b[2];
}

/** 📍️ Replaces interaction points bound to moved vertices with live model positions. */
export function resolveLiveInteractionPoints(model: Model, points: readonly Vec3[], snaps: readonly (InteractionPointSnap | null)[]): readonly Vec3[] {
  if (!snaps.length) return points;
  return points.map((point, index) => {
    const snap = snaps[index];
    if (!snap || snap.kind !== "vertex") return point;
    const live = model.vertices[snap.id as VertexRef]?.position;
    return live ?? point;
  });
}

/** 📍️ Shallow context copy with `points` resolved from bound vertex snaps. */
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

/** ⌨️ Keyboard modifiers carried on pointer/selection `InteractionEvent`s. */
export interface InteractionEventModifiers {
  readonly shift?: boolean;
  readonly ctrl?: boolean;
  readonly alt?: boolean;
  readonly meta?: boolean;
}

export function selectionTargetsWithMode(current: readonly SelectionTarget[], next: readonly SelectionTarget[], modifiers: InteractionEventModifiers = {}): SelectionTarget[] {
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

/** 🪪️ Kernel geometry + extension view `object` kinds used by selection commands. */
export const ALL_MODEL_SELECTION_KINDS: readonly ModelEntityKind[] = ["anchor", "vertex", "edge", "wire", "face", "solid", "object", "geometry", "attribute"];

const MODEL_SELECTION_KIND_ORDER = new Map<ModelEntityKind, number>(ALL_MODEL_SELECTION_KINDS.map((kind, index) => [kind, index]));

/** 🪪️ model-definition selection command operation id (`selection.apply` param). */
export type SelectionApplyOperation = "selectAll" | "deselectAll" | "invert" | "selectKinds";

/** 🪪️ Headless `selection.apply` / interaction commit input. */
export interface SelectionApplyParams {
  readonly operation: SelectionApplyOperation;
  readonly seedTargets?: readonly SelectionTarget[];
  readonly kinds?: readonly ModelEntityKind[];
}

/** 🪪️ model-definition selection command interaction row (`selection.*` registry). */
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

/** 🪪️ Parses `context.targets` or action patch targets into validated `SelectionTarget` rows. */
export function selectionTargetsFromContext(ctx: Record<string, unknown>): readonly SelectionTarget[] {
  return parseSelectionTargetsFromUnknown(ctx.targets);
}

/** 🪪️ Reads `targets` from an `selection.apply` action result patch. */
export function selectionTargetsFromActionResult(result: ActionResult): readonly SelectionTarget[] {
  return parseSelectionTargetsFromUnknown(result.patch?.set?.targets);
}

let modelEntityKindSetCache: ReadonlySet<string> | null = null;
/** 🪪️ Selectable entity kinds, resolved lazily so the geometry import cycle is settled before the first read. */
function modelEntityKindSet(): ReadonlySet<string> {
  return (modelEntityKindSetCache ??= new Set<string>([...PRIMITIVE_MODEL_ENTITY_KINDS, "object", "geometry", "attribute"]));
}

export function parseSelectionTargetsFromUnknown(raw: unknown): SelectionTarget[] {
  if (!Array.isArray(raw)) return [];
  const out: SelectionTarget[] = [];
  for (const item of raw) {
    if (!item || typeof item !== "object") continue;
    const kind = (item as { kind?: unknown }).kind;
    const id = (item as { id?: unknown }).id;
    if (typeof kind !== "string" || !modelEntityKindSet().has(kind)) continue;
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
    if (typeof item !== "string" || !modelEntityKindSet().has(item)) continue;
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

/** 🎯️ Primary selection row for attribute editing (primitive first, then typology object). */
export function primaryAttributeSelectionTarget(selection: readonly SelectionTarget[]): SelectionTarget | null {
  if (!selection.length) return null;
  for (const row of selection) {
    if (row.kind !== "object") return row;
  }
  return selection.find((row) => row.kind === "object") ?? selection[0] ?? null;
}

/** 🪪️ Collects stable `SelectionTarget` rows for kernel `kinds` scoped to the active model definition. */
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

/** 🪪️ Applies `selectAll` / `deselectAll` / `invert` / `selectKinds` to `current` against `model`. */
export function applySelectionOperation(operation: SelectionApplyOperation, current: readonly SelectionTarget[], model: Model, kinds: readonly ModelEntityKind[], activeModelDefinitionId?: string | null): SelectionTarget[] {
  if (operation === "deselectAll") return [];
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  const scopeKinds = kinds.length > 0 ? kinds : modelDefinitionSelectionEntityKinds(mdId);
  const universe = collectGeometrySelectionTargets(model, scopeKinds, mdId);
  if (operation === "selectAll" || operation === "selectKinds") return universe;
  const cur = new Set(current.map(selectionTargetKey));
  return universe.filter((target) => !cur.has(selectionTargetKey(target)));
}

/** 🪪️ Shared selection command core used by `selection.apply` and headless callers. */
export function executeSelectionApply(params: SelectionApplyParams, ctx: { readonly model: Model; readonly activeModelDefinitionId?: string | null }): SelectionTarget[] {
  const seed = params.seedTargets ?? [];
  const kinds = params.operation === "selectKinds" ? [...(params.kinds ?? [])] : params.operation === "invert" || params.operation === "selectAll" ? [...ALL_MODEL_SELECTION_KINDS] : [];
  return applySelectionOperation(params.operation, seed, ctx.model, kinds, ctx.activeModelDefinitionId ?? null);
}

/** 🪪️ Standard `selection.apply` / `selection.*` construct `CALL` result (`YIELD targets` / `data.targets`). */
export function selectionCommandActionResult(targets: readonly SelectionTarget[]): ActionResult {
  return { patch: { set: { targets: [...targets] } }, diff: EMPTY_MODEL_DIFF, data: { targets } };
}

/** 🪪️ True when `actionId` is a `selection.*` construct or action (`selection.apply`, `selection.selectAll`, …). */
export function isSelectionConstructActionId(actionId: string): boolean {
  return actionId.startsWith("selection.");
}

export function selectionApplyParamsFromRecord(params: Record<string, unknown>): SelectionApplyParams {
  const bag = (params.__context ?? {}) as Record<string, unknown>;
  const operation = String(params.operation ?? bag.operation ?? "selectAll") as SelectionApplyOperation;
  const seed = parseSelectionTargetsFromUnknown(params.seedTargets ?? bag.seedTargets);
  const kinds = parseModelEntityKinds(params.kinds ?? bag.kinds);
  return { operation, seedTargets: seed, ...(operation === "selectKinds" ? { kinds } : {}) };
}
// #endregion 📦️🎬️actions

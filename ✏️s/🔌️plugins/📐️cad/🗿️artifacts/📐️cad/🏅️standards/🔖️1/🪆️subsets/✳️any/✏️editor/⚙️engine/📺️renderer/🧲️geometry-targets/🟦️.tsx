/** 🧲️ Mesh transfer helpers, archived footprints, layout math, pick-target kinds and toggles, geometry bucket queries, pick-target creation, typology pattern materials and the committed mesh layers.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | renderer}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import {
  reactHostPort,
  sortCanvasPickTargetsGeneralFirst,
  type CanvasPickRequest,
  type CanvasPickTarget,
  type CheckboxState,
  type ThreeEvent,
  type ReactNode,
  type BufferGeometry as ThreeBufferGeometry,
  type MeshStandardMaterial as ThreeMeshStandardMaterial,
  type Object3D as ThreeObject3D
} from "@semio-tech/ui-react";
// #endregion 🔌️Adapters
import { ephemeralMap } from "@semio-tech/framework";
import { createTemplatePool, ViewRadiusLayer, WORLD_LOCKED_OPACITY_SCALE, worldEntitySelectable } from "@semio-tech/infinite-world-r3f";
import {
  kernelGeometry,
  solidRef,
  type FaceGroup,
  type FaceInfo,
  type MeshTransfer,
  type Vec3
} from "../../../../../../../../../../⚙️engine/📔️registry/🟦️.ts";
import {
  isShapeModelDefinition,
  defaultModelDefinitionId,
  modelDefinitionSelectionEntityKinds,
  modelDefinitionUsesGeometryPicking,
  PRIMITIVE_MODEL_ENTITY_KINDS,
  listModelObjectsForModelDefinition,
  objectPrimaryPrimitiveRef,
  listTypologiesForModelDefinition,
  objectPrimitiveEntries,
  resolvePrimitiveRefKind,
  parseModelJson,
  Model,
  type InteractionEvent,
  type SelectionTarget,
  type SpatialObjectRecord,
  type ModelEntityKind,
  type ModelJson,
  resolveTypologyStyle,
  typologyStyleCacheKey,
  type ResolvedTypologyStyle,
  type SpatialEntityFlags
} from "../../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts";
import { typologyObjectPascalFromLabel } from "../../🧬️typology/🟦️.ts";
import { isEmptyModelDiff, type SpatialKernel, type SpatialPreviewKernel } from "../../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️.ts";
import { type DisplayItem, type DisplayModel } from "../../🎬️actions/🟦️.ts";
import { DocumentHistory, type InteractionSnapshot } from "../../🗿️artifact/🟦️.ts";
import { faceNormal } from "../../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🧮️preview/🟦️.ts";
import {
  type AnchorRecord,
  type EdgeRecord,
  type EdgeRef,
  type FaceRecord,
  scenePreview,
  type ShellRecord,
  type ShellRef,
  type SolidRecord,
  type SolidRef,
  THREE,
  type VertexRecord,
  type WireRecord,
  type WireRef
} from "../🔌️ports/🟦️.ts";
import { spatialSceneColors } from "../🎨️style/🟦️.ts";

// #region 🎬️WorkerClient
/** 🧩️ Binary search `faceGroups` by triangle index (playground `ShapeRenderer` pattern). */
export function findFaceGroupAt(groups: readonly FaceGroup[], triangleIndex: number): FaceGroup | null {
  const indexBufferOffset = triangleIndex * 3;
  let lo = 0;
  let hi = groups.length - 1;
  while (lo <= hi) {
    const mid = (lo + hi) >>> 1;
    const group = groups[mid]!;
    if (indexBufferOffset < group.start) hi = mid - 1;
    else if (indexBufferOffset >= group.start + group.count) lo = mid + 1;
    else return group;
  }
  return null;
}

/** 🎞️ Debounced `SpatialKernel.tessellate` for R3F hosts (worker-backed brepjs). */
export function useTessellation(kernel: SpatialKernel | null, solid: ReturnType<typeof solidRef> | null, tolerance: number): MeshTransfer | null {
  const [mesh, setMesh] = reactHostPort.useState<MeshTransfer | null>(null);
  const rafRef = reactHostPort.useRef(0);
  reactHostPort.useEffect(() => {
    if (!kernel || !solid) {
      setMesh(null);
      return;
    }
    cancelAnimationFrame(rafRef.current);
    rafRef.current = requestAnimationFrame(() => {
      void kernel.tessellate(solid, tolerance).then((next) => setMesh(isRenderableMeshTransfer(next) ? next : null));
    });
    return () => cancelAnimationFrame(rafRef.current);
  }, [kernel, solid, tolerance]);
  return mesh;
}

/** 📦️ Lists `SolidRef` ids present on a model graph (document solids for tessellation). */
export function listModelSolidRefs(model: Model | ModelJson | null): readonly SolidRef[] {
  if (!model) return [];
  const graph = model instanceof Model ? model : parseModelJson(model);
  if (!graph) return [];
  return Object.keys(graph.solids).map((id) => solidRef(id));
}

/** 🔑️ Stable React key from mesh buffer fingerprints (avoids stale geometry reuse). */
export function meshTransferContentKey(mesh: MeshTransfer, fallback = 0): string {
  const p = mesh.position;
  if (p.length === 0) return `empty-${fallback}`;
  const mid = ((p.length / 6) | 0) * 3;
  return `${p.length}-${p[0]}-${p[mid] ?? 0}-${p[p.length - 1] ?? 0}-${mesh.faceGroups.length}`;
}

export function isRenderableMeshTransfer(mesh: MeshTransfer): boolean {
  if (mesh.position.length === 0) return false;
  if (mesh.position.length % 3 !== 0) return false;
  if (mesh.normal.length !== mesh.position.length) return false;
  if (mesh.edges.length % 3 !== 0) return false;
  for (const value of mesh.position) {
    if (!Number.isFinite(value)) return false;
  }
  for (const value of mesh.normal) {
    if (!Number.isFinite(value)) return false;
  }
  for (const value of mesh.edges) {
    if (!Number.isFinite(value)) return false;
  }
  const vertexCount = mesh.position.length / 3;
  for (const value of mesh.index) {
    if (!Number.isFinite(value) || value < 0 || value >= vertexCount) return false;
  }
  return true;
}

/** 🎞️ Tessellates every model solid through `SpatialKernel.tessellate` (worker-backed). */
export function useDocumentMeshes(kernel: SpatialKernel | null, model: Model, tolerance: number, keepPreviousWhileLoading = false): readonly { readonly solid: SolidRef; readonly mesh: MeshTransfer }[] {
  const [meshes, setMeshes] = reactHostPort.useState<readonly { readonly solid: SolidRef; readonly mesh: MeshTransfer }[]>([]);
  const revision = model.revision;
  const revisionRef = reactHostPort.useRef(revision);
  revisionRef.current = revision;
  reactHostPort.useEffect(() => {
    if (!kernel) {
      setMeshes([]);
      return;
    }
    const modelAtStart = model;
    const revisionAtStart = revision;
    const solids = listModelSolidRefs(modelAtStart);
    if (solids.length === 0) {
      setMeshes([]);
      return;
    }
    if (!keepPreviousWhileLoading) setMeshes([]);
    let cancelled = false;
    void (async () => {
      const rows = await Promise.all(
        solids.map(async (solid) => {
          try {
            const mesh = await kernel.tessellate(solid, tolerance, modelAtStart);
            return isRenderableMeshTransfer(mesh) ? { solid, mesh } : null;
          } catch {
            return null;
          }
        }),
      );
      if (cancelled || revisionAtStart !== revisionRef.current) return;
      setMeshes(rows.filter((row): row is { readonly solid: SolidRef; readonly mesh: MeshTransfer } => row !== null));
    })();
    return () => {
      cancelled = true;
    };
  }, [kernel, model, revision, tolerance, keepPreviousWhileLoading]);
  return meshes;
}

/** 📐️ Axis-aligned bounds of all mesh positions (for camera auto-fit). */
export function boundsFromMeshTransfers(meshes: readonly MeshTransfer[]): { readonly center: Vec3; readonly radius: number } | null {
  if (meshes.length === 0) return null;
  let minX = Infinity;
  let minY = Infinity;
  let minZ = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;
  let maxZ = -Infinity;
  let hasFinitePoint = false;
  for (const mesh of meshes) {
    const pos = mesh.position;
    for (let i = 0; i < pos.length; i += 3) {
      const x = pos[i]!;
      const y = pos[i + 1]!;
      const z = pos[i + 2]!;
      if (!Number.isFinite(x) || !Number.isFinite(y) || !Number.isFinite(z)) continue;
      hasFinitePoint = true;
      if (x < minX) minX = x;
      if (y < minY) minY = y;
      if (z < minZ) minZ = z;
      if (x > maxX) maxX = x;
      if (y > maxY) maxY = y;
      if (z > maxZ) maxZ = z;
    }
  }
  if (!hasFinitePoint) return null;
  const cx = (minX + maxX) / 2;
  const cy = (minY + maxY) / 2;
  const cz = (minZ + maxZ) / 2;
  const dx = maxX - minX;
  const dy = maxY - minY;
  const dz = maxZ - minZ;
  const radius = Math.sqrt(dx * dx + dy * dy + dz * dz) / 2;
  return { center: [cx, cy, cz], radius: Math.max(radius, 0.5) };
}

/** 📐️ Axis-aligned bounds of geometry vertex positions (factory / REPL geometry auto-fit). */
export function boundsFromSpatialPickGeometry(geometry: SpatialPickGeometry | null | undefined): { readonly center: Vec3; readonly radius: number } | null {
  if (!geometry) return null;
  const buckets = geometryBuckets(geometry);
  const verts = geometryRecords(buckets.vertices);
  if (!verts.length) return null;
  let minX = Infinity;
  let minY = Infinity;
  let minZ = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;
  let maxZ = -Infinity;
  for (const vertex of verts) {
    const [x, y, z] = vertex.position;
    if (x < minX) minX = x;
    if (y < minY) minY = y;
    if (z < minZ) minZ = z;
    if (x > maxX) maxX = x;
    if (y > maxY) maxY = y;
    if (z > maxZ) maxZ = z;
  }
  const cx = (minX + maxX) / 2;
  const cy = (minY + maxY) / 2;
  const cz = (minZ + maxZ) / 2;
  const dx = maxX - minX;
  const dy = maxY - minY;
  const dz = maxZ - minZ;
  const radius = Math.sqrt(dx * dx + dy * dy + dz * dz) / 2;
  return { center: [cx, cy, cz], radius: Math.max(radius, 0.5) };
}

export function mergeSpatialSceneBounds(a: { readonly center: Vec3; readonly radius: number } | null, b: { readonly center: Vec3; readonly radius: number } | null): { readonly center: Vec3; readonly radius: number } | null {
  if (!a) return b;
  if (!b) return a;
  const min: Vec3 = [Math.min(a.center[0] - a.radius, b.center[0] - b.radius), Math.min(a.center[1] - a.radius, b.center[1] - b.radius), Math.min(a.center[2] - a.radius, b.center[2] - b.radius)];
  const max: Vec3 = [Math.max(a.center[0] + a.radius, b.center[0] + b.radius), Math.max(a.center[1] + a.radius, b.center[1] + b.radius), Math.max(a.center[2] + a.radius, b.center[2] + b.radius)];
  const center: Vec3 = [(min[0] + max[0]) / 2, (min[1] + max[1]) / 2, (min[2] + max[2]) / 2];
  const radius = Math.max(Math.sqrt((max[0] - min[0]) ** 2 + (max[1] - min[1]) ** 2 + (max[2] - min[2]) ** 2) / 2, 0.5);
  return { center, radius };
}
// #endregion 🎬️WorkerClient

// #region 🪩️ArchivedFootprints
/** 📦️ Footprint of a finished axis-aligned box for persistent REPL overlays. */
export interface ArchivedBoxLayout {
  readonly cornerA: Vec3;
  readonly cornerB: Vec3;
  readonly height: number;
}

function isVec3Record(v: unknown): v is Vec3 {
  return Array.isArray(v) && v.length === 3 && v.every((x) => typeof x === "number");
}

/** 📦️ Reads `origin`/`corner`/`height` from post-commit interaction context when present. */
export function tryArchivedBoxFromContext(ctx: Record<string, unknown>): ArchivedBoxLayout | null {
  const o = ctx.origin;
  const c = ctx.corner;
  const h = ctx.height;
  if (!isVec3Record(o) || !isVec3Record(c)) return null;
  const hz = typeof h === "number" && Number.isFinite(h) && h > 0 ? h : null;
  if (hz === null) return null;
  return { cornerA: o, cornerB: c, height: hz };
}

/** 🧊️ True when committed kernel solids own the scene (footprint box previews would duplicate meshes). */
export function modelHasCommittedSolidsForDisplay(model: Model | null | undefined): boolean {
  return listModelSolidRefs(model ?? null).length > 0;
}

/** 🧊️ True when typology objects expose standalone surface primitives for factory face shading. */
export function modelHasFactoryFaceDisplay(model: Model | null | undefined, modelDefinitionId: string): boolean {
  if (!model) return false;
  return visibleFaceRefsForModelDefinition(model, modelDefinitionId).size > 0;
}

/** 📦️ Drops axis-aligned footprint `box-preview` display items when kernel solids are present. */
export function filterFootprintBoxPreviewDisplayItems(display: DisplayModel, model: Model | null | undefined): DisplayModel {
  if (!modelHasCommittedSolidsForDisplay(model)) return display;
  const items = display.items.filter((item) => item.kind !== "box-preview");
  return items.length === display.items.length ? display : { ...display, items };
}

export function mergeDisplayWithArchivedBoxes(base: DisplayModel, archived: readonly ArchivedBoxLayout[], model: Model | null | undefined): DisplayModel {
  if (archived.length === 0 || modelHasCommittedSolidsForDisplay(model)) return base;
  const extra: DisplayItem[] = archived.map((b, i) => ({
    kind: "box-preview",
    id: `archived-box-${i}`,
    role: "archived",
    params: { cornerA: b.cornerA, cornerB: b.cornerB, height: b.height },
  }));
  return { ...base, items: [...extra, ...base.items] };
}

/** 📦️ True when a history entry should leave a persistent box footprint overlay (not transforms). */
export function historyEntryArchivesBoxFootprint(interactionId: string): boolean {
  if (interactionId.startsWith("transform.")) return false;
  if (interactionId.startsWith("selection.")) return false;
  if (interactionId.startsWith("measure.")) return false;
  if (interactionId === "primitive.box") return true;
  if (interactionId.includes("construct") && /box/i.test(interactionId)) return true;
  return false;
}

export function archivedBoxesFromHistory(history: DocumentHistory): readonly ArchivedBoxLayout[] {
  return history
    .entries()
    .filter((mod) => historyEntryArchivesBoxFootprint(mod.interactionId))
    .map((mod) => (mod.result.archiveContext ? tryArchivedBoxFromContext(mod.result.archiveContext) : null))
    .filter((box): box is ArchivedBoxLayout => box !== null);
}

export function replBaseDisplayForHistory(snapshot: InteractionSnapshot): DisplayModel {
  if (snapshot.state !== "committed") return snapshot.display;
  const diff = snapshot.lastResponse?.diff;
  const committedGeometry = snapshot.lastResponse?.ok === true && diff !== undefined && !isEmptyModelDiff(diff);
  if (!committedGeometry) return snapshot.display;
  return { ...snapshot.display, items: snapshot.display.items.filter((item) => item.role !== "preview") };
}
// #endregion 🪩️ArchivedFootprints

// #region 📐️Layout
/** 📐️ Center and axis-aligned scale for a unit `BoxGeometry` from two XY footprint corners and height. */
export function computeBoxPreviewLayout(cornerA: Vec3, cornerB: Vec3, height: number, preview: SpatialPreviewKernel = scenePreview()): { readonly position: Vec3; readonly scale: Vec3 } {
  return preview.computeBoxPreviewLayout(cornerA, cornerB, height);
}

/** 🟦️ Center and radius for the live sphere preview while the radius point is moving. */
export function computeSpherePreviewLayout(center: Vec3 | null, cursor: Vec3 | null): { readonly position: Vec3; readonly radius: number } | null {
  if (!center || !cursor) return null;
  const radius = Math.hypot(cursor[0] - center[0], cursor[1] - center[1], cursor[2] - center[2]);
  return radius > 1e-9 ? { position: center, radius } : null;
}

export function readVec3(v: unknown): Vec3 | null {
  if (Array.isArray(v) && v.length === 3 && v.every((x) => typeof x === "number")) return v as unknown as Vec3;
  return null;
}

export function readNumber(v: unknown): number | null {
  return typeof v === "number" && Number.isFinite(v) ? v : null;
}

export function readVec3Array(v: unknown): readonly Vec3[] {
  if (!Array.isArray(v)) return [];
  return v.filter(isVec3Record) as readonly Vec3[];
}

/** 📦️ Axis-aligned bounds for geometry highlight wireframes. */
export function bboxFromPoints(points: readonly Vec3[], preview: SpatialPreviewKernel = scenePreview()): { readonly min: Vec3; readonly max: Vec3 } | null {
  return preview.aabbFromPoints(points);
}

/** 📦️ Twelve edges of an axis-aligned box for preview line rendering. */
export function bboxWireSegments(min: Vec3, max: Vec3): readonly (readonly [Vec3, Vec3])[] {
  const [x0, y0, z0] = min;
  const [x1, y1, z1] = max;
  const c: readonly Vec3[] = [
    [x0, y0, z0],
    [x1, y0, z0],
    [x1, y1, z0],
    [x0, y1, z0],
    [x0, y0, z1],
    [x1, y0, z1],
    [x1, y1, z1],
    [x0, y1, z1],
  ];
  const idx: readonly (readonly [number, number])[] = [
    [0, 1],
    [1, 2],
    [2, 3],
    [3, 0],
    [4, 5],
    [5, 6],
    [6, 7],
    [7, 4],
    [0, 4],
    [1, 5],
    [2, 6],
    [3, 7],
  ];
  return idx.map(([a, b]) => [c[a]!, c[b]!] as const);
}

export function parseDisplaySelectionTargets(v: unknown): readonly { readonly kind: ModelEntityKind; readonly id: string }[] {
  if (!Array.isArray(v)) return [];
  const out: { kind: ModelEntityKind; id: string }[] = [];
  for (const raw of v) {
    if (!raw || typeof raw !== "object") continue;
    const o = raw as Record<string, unknown>;
    const kind = o.kind;
    const id = o.id;
    if (typeof kind === "string" && typeof id === "string") out.push({ kind: kind as ModelEntityKind, id });
  }
  return out;
}

/** 🖼️ Maps declarative `previewKind` + params to a point transform for geometry wireframes. */
export function transformPointsForPreviewKind(previewKind: string, params: Record<string, unknown>, preview: SpatialPreviewKernel = scenePreview()): (point: Vec3) => Vec3 {
  return preview.transformPointsForPreviewKind(previewKind, params);
}

/** 🖼️ Active geometry point transform from move/copy/mirror/rotate/scale preview display items. */
export function geometryPreviewTransformFromDisplay(model: DisplayModel): ((point: Vec3) => Vec3) | null {
  for (const item of model.items) {
    if (item.kind !== "preview" || !item.params) continue;
    const previewKind = typeof item.params.previewKind === "string" ? item.params.previewKind : "";
    if (!previewKindUsesGeometryWireframe(previewKind)) continue;
    if (previewKind === "move-preview" || previewKind === "copy-preview" || previewKind === "mirror-preview" || previewKind === "rotate-preview" || previewKind === "scale-preview" || previewKind === "scale1d-preview") {
      return transformPointsForPreviewKind(previewKind, item.params);
    }
  }
  return null;
}

export function previewKindUsesGeometryWireframe(previewKind: string): boolean {
  return (
    previewKind === "selected-objects" ||
    previewKind === "move-preview" ||
    previewKind === "copy-preview" ||
    previewKind === "mirror-preview" ||
    previewKind === "rotate-preview" ||
    previewKind === "scale-preview" ||
    previewKind === "scale1d-preview" ||
    previewKind.endsWith("-selection") ||
    previewKind.startsWith("boolean-") ||
    previewKind === "highlight-curves" ||
    previewKind === "cutters" ||
    previewKind === "split-objects" ||
    previewKind === "trim-preview" ||
    previewKind === "extrusion" ||
    previewKind === "network-curves"
  );
}

export const raycastNone: ThreeObject3D["raycast"] = () => undefined;
// #endregion 📐️Layout

// #region 🧲️GeometryTargets
export type SpatialPickKind = "pointer.down" | "pointer.move";

/** 🎯️ Primitive and object pick kinds for renderer feedback (maps to kernel geometry via {@link SpatialPickTarget.geometryKind}). */
export type SpatialGeometryPickTargetKind = "object" | "face" | "edge" | "vertex";

export type SpatialPickTargetKind = SpatialGeometryPickTargetKind;

export const SPATIAL_PICK_TARGET_KINDS: readonly SpatialPickTargetKind[] = ["object", "face", "edge", "vertex"];

const CAD_PICK_GENERALITY: Readonly<Record<SpatialPickTargetKind, number>> = {
  object: 0,
  face: 1,
  edge: 2,
  vertex: 3,
};

function spatialPickTargetToCanvas(target: SpatialPickTarget): CanvasPickTarget {
  return { domain: target.kind, id: target.id, generality: CAD_PICK_GENERALITY[target.kind], label: target.id };
}

function canvasPickModifiersFromEvent(modifiers: InteractionEvent["modifiers"]): Readonly<Record<string, boolean>> | undefined {
  if (!modifiers || typeof modifiers !== "object") return undefined;
  const out: Record<string, boolean> = {};
  for (const [key, value] of Object.entries(modifiers)) if (typeof value === "boolean") out[key] = value;
  return out;
}

export function spatialPickCanvasRequest(request: SpatialSelectionRequest | null): CanvasPickRequest | null {
  if (!request) return null;
  return {
    targets: sortCanvasPickTargetsGeneralFirst(request.targets.map(spatialPickTargetToCanvas)),
    client: request.client,
    modifiers: canvasPickModifiersFromEvent(request.modifiers),
  };
}

export const GEOMETRY_KIND_TO_OBJECT_PICK: Partial<Record<ModelEntityKind, SpatialGeometryPickTargetKind>> = {
  vertex: "vertex",
  edge: "edge",
  wire: "edge",
  face: "face",
  shell: "face",
  solid: "object",
  anchor: "vertex",
};

function spatialPickKindsForSelectionAccept(accept: readonly ModelEntityKind[]): ReadonlySet<SpatialPickTargetKind> | null {
  if (!accept.length) return null;
  const out = new Set<SpatialPickTargetKind>();
  for (const kind of accept) {
    if (kind === "object" || kind === "face" || kind === "edge" || kind === "vertex") {
      out.add(kind);
      continue;
    }
    const mapped = GEOMETRY_KIND_TO_OBJECT_PICK[kind];
    if (mapped) out.add(mapped);
  }
  return out;
}

export function kernelGeometryKindForObjectPick(kind: SpatialGeometryPickTargetKind, geometryKind?: ModelEntityKind): ModelEntityKind {
  if (geometryKind) return geometryKind;
  if (kind === "vertex") return "vertex";
  if (kind === "edge") return "edge";
  if (kind === "face") return "face";
  return "solid";
}

/** 👁️ Per-kind on/off map for visibility filters or selection/hover gates (`false` disables). */
export type SpatialPickKindToggles = Partial<Record<SpatialPickTargetKind, boolean>>;

/** 👁️ Per-typology on/off map for play chrome (`false` disables show or selection). */
export type SpatialTypologyToggles = Partial<Record<string, boolean>>;

export interface SpatialPickTarget {
  readonly kind: SpatialPickTargetKind;
  readonly id: string;
  readonly point: Vec3;
  readonly points?: readonly Vec3[];
  /** 🧭️ Kernel-private geometry entity kind for primitive picks (e.g. `wire` vs `edge`). */
  readonly geometryKind?: ModelEntityKind;
  /** 🏷️ Typology id when the target belongs to a model-definition object row. */
  readonly typologyId?: string;
}

export interface SpatialSelectionRequest {
  readonly targets: readonly SpatialPickTarget[];
  readonly point: Vec3;
  readonly client: { readonly x: number; readonly y: number };
  readonly modifiers: InteractionEvent["modifiers"];
}

export type SpatialSelectionMethod = "rectangle" | "lasso";
export type SpatialSelectionCoverage = "partial" | "full";
export type SpatialSelectionMode = "default" | "additive" | "subtractive" | "invertive";

export interface SpatialDragSelectionState {
  readonly method: SpatialSelectionMethod;
  readonly coverage: SpatialSelectionCoverage;
  readonly startClient: { readonly x: number; readonly y: number };
  readonly currentClient: { readonly x: number; readonly y: number };
  readonly path: readonly { readonly x: number; readonly y: number }[];
  readonly modifiers: InteractionEvent["modifiers"];
}

export type SpatialPickGeometry = Model | ModelJson;

export function spatialPickTargetKey(target: SpatialPickTarget): string {
  return `${target.kind}:${target.id}`;
}

/** 🪪️ Stable hover/selection key for a {@link SelectionTarget} (primitive kinds use entity kind + id). */
export function selectionTargetHoverKey(target: SelectionTarget): string {
  return `${target.kind}:${target.id}`;
}

export function spatialSelectionTargetKey(target: SelectionTarget): string {
  return selectionTargetHoverKey(target);
}

/** 👁️ Default all geometry pick kinds enabled (visibility + selection). */
export function defaultSpatialPickKindToggles(): Record<SpatialPickTargetKind, boolean> {
  return Object.fromEntries(SPATIAL_PICK_TARGET_KINDS.map((kind) => [kind, true])) as Record<SpatialPickTargetKind, boolean>;
}

/** 👁️ Filters pick targets by visibility (show/hide highlights); does not affect ray pick or selection. */
export function filterSpatialPickTargetsForVisibility(targets: readonly SpatialPickTarget[], filterKindToggles: SpatialPickKindToggles = {}): SpatialPickTarget[] {
  return targets.filter((target) => filterKindToggles[target.kind] !== false);
}

/** 👁️ Reads persisted hide/lock flags from a model document for a pick-target entity id. */
export function spatialEntityFlagsForModelEntity(model: Model | null | undefined, entityId: string): SpatialEntityFlags {
  return model?.metadata.getEntityFlags(entityId) ?? {};
}

/** 👁️ Excludes hidden/locked entities from canvas pick and selection. */
export function filterSpatialPickTargetsForEntityFlags(targets: readonly SpatialPickTarget[], flagsForId: (entityId: string) => SpatialEntityFlags): SpatialPickTarget[] {
  return targets.filter((target) => worldEntitySelectable(flagsForId(target.id)));
}

/** 👁️ Drops locked/hidden entities from committed selection targets. */
export function pruneSelectionTargetsForEntityFlags(targets: readonly SelectionTarget[], flagsForId: (entityId: string) => SpatialEntityFlags): SelectionTarget[] {
  return targets.filter((target) => worldEntitySelectable(flagsForId(target.id)));
}

/** 👁️ Effective pick kinds must be both visible and enabled for selection/hover. */
export function intersectSpatialPickKindToggles(visibleKindToggles: SpatialPickKindToggles = {}, selectionKindToggles: SpatialPickKindToggles = {}): SpatialPickKindToggles {
  const merged: SpatialPickKindToggles = {};
  for (const kind of SPATIAL_PICK_TARGET_KINDS) {
    if (visibleKindToggles[kind] === false || selectionKindToggles[kind] === false) merged[kind] = false;
  }
  return merged;
}

/** 👁️ Maps active model-definition entity kinds to renderer pick-kind toggles. */
export function modelDefinitionPickTargetKinds(modelDefinitionId: string | null): readonly SpatialPickTargetKind[] {
  const entityKinds = modelDefinitionSelectionEntityKinds(modelDefinitionId ?? defaultModelDefinitionId());
  const out = new Set<SpatialPickTargetKind>();
  for (const kind of entityKinds) {
    if (kind === "vertex" || kind === "anchor") out.add("vertex");
    else if (kind === "edge" || kind === "wire") out.add("edge");
    else if (kind === "face" || kind === "shell") out.add("face");
    else if (kind === "solid" || kind === "geometry" || kind === "object") out.add("object");
  }
  if (out.size > 0) return [...out];
  return isShapeModelDefinition(modelDefinitionId) ? SPATIAL_PICK_TARGET_KINDS : ["object"];
}

/** 👁️ Default visibility/selection toggles for kinds allowed by the active model definition. */
export function defaultSpatialPickKindTogglesForModelDefinition(modelDefinitionId: string | null): Record<SpatialPickTargetKind, boolean> {
  const allowed = new Set(modelDefinitionPickTargetKinds(modelDefinitionId));
  return Object.fromEntries(SPATIAL_PICK_TARGET_KINDS.map((kind) => [kind, allowed.has(kind)])) as Record<SpatialPickTargetKind, boolean>;
}

/** 👁️ Typology ids declared on the active model definition (sorted). */
export function modelDefinitionTypologyIds(modelDefinitionId: string | null): readonly string[] {
  return listTypologiesForModelDefinition(modelDefinitionId ?? defaultModelDefinitionId())
    .map((row) => row.id)
    .sort((a, b) => a.localeCompare(b));
}

/** 👁️ Default all typologies on the active model definition enabled for show/selection. */
export function defaultSpatialTypologyTogglesForModelDefinition(modelDefinitionId: string | null): Record<string, boolean> {
  return Object.fromEntries(modelDefinitionTypologyIds(modelDefinitionId).map((id) => [id, true]));
}

/** 🏷️ Short typology label for play chrome (`Base Plate` → `BasePlate`). */
export function spatialTypologyToggleLabel(typologyId: string, label?: string): string {
  if (label?.trim()) return typologyObjectPascalFromLabel(label);
  const tail = typologyId.split(".").pop() ?? typologyId;
  return typologyObjectPascalFromLabel(tail.replace(/[._-]+/g, " "));
}

function typologyToggleAllowsTarget(target: SpatialPickTarget, toggles: SpatialTypologyToggles, typologyIds: readonly string[]): boolean {
  if (target.typologyId) return toggles[target.typologyId] !== false;
  return typologyIds.some((id) => toggles[id] !== false);
}

/** 👁️ Filters pick targets by typology show/selection toggles. */
export function filterSpatialPickTargetsForTypologyToggles(targets: readonly SpatialPickTarget[], toggles: SpatialTypologyToggles, typologyIds: readonly string[]): SpatialPickTarget[] {
  return targets.filter((target) => typologyToggleAllowsTarget(target, toggles, typologyIds));
}

/** 👁️ Derives per-kind toggles from typology-filtered targets (scene layers + legacy gates). */
export function spatialPickKindTogglesFromTypologyFilteredTargets(modelDefinitionId: string | null, visibleTargets: readonly SpatialPickTarget[]): SpatialPickKindToggles {
  const allowed = new Set(modelDefinitionPickTargetKinds(modelDefinitionId));
  const merged: SpatialPickKindToggles = {};
  for (const kind of SPATIAL_PICK_TARGET_KINDS) {
    merged[kind] = allowed.has(kind) && visibleTargets.some((target) => target.kind === kind);
  }
  return merged;
}

/** 👁️ Scene-layer pick-kind toggles from model definition + primitive show toggles (not typology pick targets). */
export function spatialSceneKindTogglesForModelDefinition(modelDefinitionId: string | null, primitiveToggles: SpatialPrimitiveToggles = defaultSpatialPrimitiveToggles()): SpatialPickKindToggles {
  const toggles = defaultSpatialPickKindTogglesForModelDefinition(modelDefinitionId);
  if (primitiveToggles.vertex === false && primitiveToggles.anchor === false) toggles.vertex = false;
  if (primitiveToggles.edge === false && primitiveToggles.wire === false) toggles.edge = false;
  if (primitiveToggles.face === false && primitiveToggles.shell === false) toggles.face = false;
  if (primitiveToggles.solid === false) toggles.object = false;
  return toggles;
}

/** 👁️ Per-primitive on/off map for play chrome (`false` disables show or filter). */
export type SpatialPrimitiveToggles = Partial<Record<ModelEntityKind, boolean>>;

/** 🧱️ Factory primitive kinds toggled in play (anchor → solid). */
export const SPATIAL_PRIMITIVE_KINDS: readonly ModelEntityKind[] = PRIMITIVE_MODEL_ENTITY_KINDS;

/** 👁️ Default all factory primitive kinds enabled for show/filter. */
export function defaultSpatialPrimitiveToggles(): Record<ModelEntityKind, boolean> {
  return Object.fromEntries(SPATIAL_PRIMITIVE_KINDS.map((kind) => [kind, true])) as Record<ModelEntityKind, boolean>;
}

/** ☑️ Aggregate enabled state for a fixed-key boolean toggle map (`false` = off). */
export type SpatialToggleGroupState = "all" | "none" | "partial";

/** ☑️ Returns whether every key is on, every key is off, or the group is mixed. */
export function spatialToggleGroupState(keys: readonly string[], toggles: Readonly<Record<string, boolean | undefined>>): SpatialToggleGroupState {
  if (keys.length === 0) return "none";
  let on = 0;
  for (const key of keys) {
    if (toggles[key] !== false) on += 1;
  }
  if (on === 0) return "none";
  if (on === keys.length) return "all";
  return "partial";
}

/** ☑️ Sets every key in a chrome toggle group on or off. */
export function spatialToggleGroupFill<T extends string>(keys: readonly T[], enabled: boolean): Record<T, boolean> {
  return Object.fromEntries(keys.map((key) => [key, enabled])) as Record<T, boolean>;
}

/** ☑️ Maps a chrome group aggregate onto the owned native checkbox state. */
export function spatialToggleCheckboxState(state: SpatialToggleGroupState): CheckboxState {
  if (state === "partial") return "indeterminate";
  return state === "all";
}

/** 🧭️ Resolves the primitive entity kind for a pick target (typology object rows → `null`). */
export function pickTargetPrimitiveKind(target: SpatialPickTarget): ModelEntityKind | null {
  if (target.kind === "object" && !target.geometryKind) return null;
  return target.geometryKind ?? kernelGeometryKindForObjectPick(target.kind as SpatialGeometryPickTargetKind, target.geometryKind);
}

/** 👁️ Filters pick targets by primitive show/filter toggles (typology object rows pass through). */
export function filterSpatialPickTargetsForPrimitiveToggles(targets: readonly SpatialPickTarget[], toggles: SpatialPrimitiveToggles): SpatialPickTarget[] {
  return targets.filter((target) => {
    const primitive = pickTargetPrimitiveKind(target);
    if (!primitive) return true;
    return toggles[primitive] !== false;
  });
}

/** 👁️ Resolves which scene layers stay visible for geometry edit vs typology object picking. */
export function resolveSpatialSceneVisibility(
  activeModelDefinitionId: string | null,
  filterKindToggles: SpatialPickKindToggles = {},
): {
  readonly showFactoryWireframe: boolean;
  readonly showCommittedFaces: boolean;
  readonly showCommittedEdges: boolean;
} {
  const visible = (kind: SpatialPickTargetKind) => filterKindToggles[kind] !== false;
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  if (modelDefinitionUsesGeometryPicking(mdId)) {
    return {
      showFactoryWireframe: visible("edge"),
      showCommittedFaces: visible("face") || visible("object"),
      showCommittedEdges: visible("edge"),
    };
  }
  return {
    showFactoryWireframe: false,
    showCommittedFaces: false,
    showCommittedEdges: false,
  };
}

export function spatialPickKindsForActiveView(activeModelDefinitionId: string | null): ReadonlySet<SpatialPickTargetKind> {
  return new Set(modelDefinitionPickTargetKinds(activeModelDefinitionId));
}

/** 👁️ Keeps pick targets allowed by the active model definition (primitives + typology objects). */
export function filterSpatialPickTargetsForActiveView(targets: readonly SpatialPickTarget[], activeModelDefinitionId: string | null): SpatialPickTarget[] {
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  const allowedPickKinds = spatialPickKindsForActiveView(mdId);
  const entityKinds = new Set(modelDefinitionSelectionEntityKinds(mdId));
  return targets.filter((target) => {
    if (!allowedPickKinds.has(target.kind)) return false;
    if (target.kind === "object" && !target.geometryKind) {
      if (!entityKinds.has("object")) return false;
      return !isShapeModelDefinition(mdId);
    }
    const geometryKind = target.geometryKind ?? kernelGeometryKindForObjectPick(target.kind, undefined);
    return entityKinds.has(geometryKind);
  });
}

function recordsById<T extends { id: string }>(xs: readonly T[]): Record<string, T> {
  const o: Record<string, T> = {};
  for (const x of xs) o[x.id] = x;
  return o;
}

function asRecordBucket<T extends { id: string }>(x: readonly T[] | Record<string, T> | undefined): Record<string, T> {
  if (!x) return {};
  return Array.isArray(x) ? recordsById(x) : (x as Record<string, T>);
}

/** 🧲️ Normalizes `ModelJson` array buckets to the record shape used by interaction math. */
export function geometryBuckets(g: SpatialPickGeometry): {
  readonly anchors: Record<string, AnchorRecord>;
  readonly vertices: Record<string, VertexRecord>;
  readonly edges: Record<string, EdgeRecord>;
  readonly wires: Record<string, WireRecord>;
  readonly faces: Record<string, FaceRecord>;
  readonly shells: Record<string, ShellRecord>;
  readonly solids: Record<string, SolidRecord>;
} {
  if (g instanceof Model) {
    return {
      anchors: g.anchors,
      vertices: g.vertices,
      edges: g.edges,
      wires: g.wires,
      faces: g.faces,
      shells: g.shells,
      solids: g.solids,
    };
  }
  return {
    anchors: asRecordBucket(g.geometry.anchors),
    vertices: asRecordBucket(g.geometry.vertices),
    edges: asRecordBucket(g.geometry.edges),
    wires: asRecordBucket(g.geometry.wires),
    faces: asRecordBucket(g.geometry.faces),
    shells: asRecordBucket(g.geometry.shells),
    solids: asRecordBucket(g.geometry.solids),
  };
}

function geometryRecords<T>(records: Record<string, T> | undefined): readonly T[] {
  return records ? Object.values(records) : [];
}

function geometryPointCentroid(points: readonly Vec3[]): Vec3 | null {
  if (points.length === 0) return null;
  const sum = points.reduce((acc, p) => [acc[0] + p[0], acc[1] + p[1], acc[2] + p[2]] as unknown as Vec3, [0, 0, 0] as unknown as Vec3);
  return [sum[0] / points.length, sum[1] / points.length, sum[2] / points.length] as unknown as Vec3;
}

function geometryEdgePoints(vertices: Record<string, VertexRecord>, edge: EdgeRecord): readonly Vec3[] {
  return scenePreview().edgeSamplePoints(vertices, edge, 32);
}

/** 📍️ NURBS poles on an edge (control points or through-points per `curve.through`). */
export function nurbsPolesFromEdge(edge: EdgeRecord): readonly Vec3[] | null {
  const curve = edge.curve;
  if (curve?.kind !== "nurbs" || curve.poles.length < 2) return null;
  return curve.poles;
}

/** 📍️ NURBS poles for edge/wire pick highlights when selected or hovered. */
export function geometryEntityNurbsPoles(buckets: ReturnType<typeof geometryBuckets>, kind: ModelEntityKind, id: string): readonly Vec3[] {
  if (kind === "edge" && buckets.edges[id]) {
    return nurbsPolesFromEdge(buckets.edges[id]!) ?? [];
  }
  if (kind === "wire" && buckets.wires[id]) {
    const poles = buckets.wires[id]!.edgeIds.flatMap((edgeId: EdgeRef) => {
      const edge = buckets.edges[edgeId];
      return edge ? (nurbsPolesFromEdge(edge) ?? []) : [];
    });
    return uniqueGeometryPoints(poles);
  }
  return [];
}

function geometryFacePoints(vertices: Record<string, VertexRecord>, edges: Record<string, EdgeRecord>, wires: Record<string, WireRecord>, face: FaceRecord): readonly Vec3[] {
  const ids = face.wireIds.flatMap((wireId: WireRef) => wires[wireId]?.edgeIds ?? []);
  const points = ids.flatMap((id: EdgeRef) => {
    const edge = edges[id];
    return edge ? geometryEdgePoints(vertices, edge) : [];
  });
  const unique = new Map(points.map((p: Vec3) => [p.join(","), p]));
  return [...unique.values()];
}

function uniqueGeometryPoints(points: readonly Vec3[]): readonly Vec3[] {
  return [...new Map(points.map((p: Vec3) => [p.join(","), p])).values()];
}

function geometryWirePoints(vertices: Record<string, VertexRecord>, edges: Record<string, EdgeRecord>, wire: WireRecord): readonly Vec3[] {
  return uniqueGeometryPoints(wire.edgeIds.flatMap((id: EdgeRef) => (edges[id] ? geometryEdgePoints(vertices, edges[id]!) : [])));
}

function geometryShellPoints(vertices: Record<string, VertexRecord>, edges: Record<string, EdgeRecord>, wires: Record<string, WireRecord>, faces: Record<string, FaceRecord>, shell: ShellRecord): readonly Vec3[] {
  return uniqueGeometryPoints(shell.faceIds.flatMap((id: kernelGeometry.FaceRef) => (faces[id] ? geometryFacePoints(vertices, edges, wires, faces[id]!) : [])));
}

function geometrySolidPoints(vertices: Record<string, VertexRecord>, edges: Record<string, EdgeRecord>, wires: Record<string, WireRecord>, faces: Record<string, FaceRecord>, shells: Record<string, ShellRecord>, solid: SolidRecord): readonly Vec3[] {
  return uniqueGeometryPoints(solid.shellIds.flatMap((id: ShellRef) => (shells[id] ? geometryShellPoints(vertices, edges, wires, faces, shells[id]!) : [])));
}

function geometryAllVertexPoints(vertices: Record<string, VertexRecord>): readonly Vec3[] {
  return geometryRecords(vertices).map((vertex) => vertex.position);
}

function geometryEntityPoints(buckets: ReturnType<typeof geometryBuckets>, kind: ModelEntityKind, id: string): readonly Vec3[] {
  if (kind === "anchor") {
    const anchor = buckets.anchors[id];
    return anchor ? [anchor.position] : [];
  }
  if (kind === "vertex") return buckets.vertices[id]?.position ? [buckets.vertices[id]!.position] : [];
  if (kind === "edge" && buckets.edges[id]) return geometryEdgePoints(buckets.vertices, buckets.edges[id]!);
  if (kind === "wire" && buckets.wires[id]) return geometryWirePoints(buckets.vertices, buckets.edges, buckets.wires[id]!);
  if (kind === "face" && buckets.faces[id]) return geometryFacePoints(buckets.vertices, buckets.edges, buckets.wires, buckets.faces[id]!);
  if (kind === "shell" && buckets.shells[id]) return geometryShellPoints(buckets.vertices, buckets.edges, buckets.wires, buckets.faces, buckets.shells[id]!);
  if (kind === "solid" && buckets.solids[id]) return geometrySolidPoints(buckets.vertices, buckets.edges, buckets.wires, buckets.faces, buckets.shells, buckets.solids[id]!);
  return [];
}

function geometryEntityPointsForPickTarget(buckets: ReturnType<typeof geometryBuckets>, target: SpatialPickTarget): readonly Vec3[] {
  const geometryKind = kernelGeometryKindForObjectPick(target.kind as SpatialGeometryPickTargetKind, target.geometryKind);
  return geometryEntityPoints(buckets, geometryKind, target.id);
}

/** 📐️ Consecutive segment pairs along a sampled edge polyline. */
export function polylineWireSegments(points: readonly Vec3[]): readonly (readonly [Vec3, Vec3])[] {
  if (points.length < 2) return [];
  const out: (readonly [Vec3, Vec3])[] = [];
  for (let i = 1; i < points.length; i++) out.push([points[i - 1]!, points[i]!]);
  return out;
}

function geometryWireEdgeSegments(vertices: Record<string, VertexRecord>, edges: Record<string, EdgeRecord>, wire: WireRecord): readonly (readonly [Vec3, Vec3])[] {
  const out: (readonly [Vec3, Vec3])[] = [];
  for (const edgeId of wire.edgeIds) {
    const edge = edges[edgeId];
    if (!edge) continue;
    out.push(...polylineWireSegments(geometryEdgePoints(vertices, edge)));
  }
  return out;
}

/** 📐️ Geometry wire segments for previews (edges/wires/faces), bbox fallback for aggregates. */
export function geometryEntityWireSegments(buckets: ReturnType<typeof geometryBuckets>, kind: ModelEntityKind, id: string): readonly (readonly [Vec3, Vec3])[] {
  if (kind === "edge" && buckets.edges[id]) {
    return geometryWireEdgeSegments(buckets.vertices, buckets.edges, { id, edgeIds: [id] } as unknown as WireRecord);
  }
  if (kind === "wire" && buckets.wires[id]) return geometryWireEdgeSegments(buckets.vertices, buckets.edges, buckets.wires[id]!);
  if (kind === "face" && buckets.faces[id]) {
    const face = buckets.faces[id]!;
    return face.wireIds.flatMap((wireId: WireRef) => {
      const wire = buckets.wires[wireId];
      return wire ? geometryWireEdgeSegments(buckets.vertices, buckets.edges, wire) : [];
    });
  }
  if (kind === "shell" && buckets.shells[id]) {
    return buckets.shells[id]!.faceIds.flatMap((faceId: kernelGeometry.FaceRef) => geometryEntityWireSegments(buckets, "face", faceId));
  }
  if (kind === "solid" && buckets.solids[id]) {
    return buckets.solids[id]!.shellIds.flatMap((shellId: ShellRef) => geometryEntityWireSegments(buckets, "shell", shellId));
  }
  const pts = geometryEntityPoints(buckets, kind, id);
  const bb = bboxFromPoints(pts);
  return bb ? bboxWireSegments(bb.min, bb.max) : [];
}

/** 📐️ All B-rep edge segments for factory geometry wireframe display. */
export function collectGeometryEdgeSegments(buckets: ReturnType<typeof geometryBuckets>): readonly (readonly [Vec3, Vec3])[] {
  const out: (readonly [Vec3, Vec3])[] = [];
  for (const edge of geometryRecords(buckets.edges)) {
    out.push(...polylineWireSegments(geometryEdgePoints(buckets.vertices, edge)));
  }
  return out;
}

/** 📐️ B-rep edge segments limited to revealed factory-geometry members. */
export function collectGeometryEdgeSegmentsForMembers(buckets: ReturnType<typeof geometryBuckets>, revealedMemberKeys: ReadonlySet<string>): readonly (readonly [Vec3, Vec3])[] {
  const out: (readonly [Vec3, Vec3])[] = [];
  for (const edge of geometryRecords(buckets.edges)) {
    if (!revealedMemberKeys.has(`edge:${edge.id}`)) continue;
    out.push(...polylineWireSegments(geometryEdgePoints(buckets.vertices, edge)));
  }
  return out;
}

function modelObjectPickPoints(model: Model, row: SpatialObjectRecord): readonly Vec3[] {
  const buckets = geometryBuckets(model);
  const cellRef = Object.values(row.primitives)[0];
  const cell = cellRef ? buckets.solids[cellRef] : undefined;
  if (!cell) return [];
  return geometrySolidPoints(buckets.vertices, buckets.edges, buckets.wires, buckets.faces, buckets.shells, cell);
}

export function collectSolidPrimitiveMemberIds(buckets: ReturnType<typeof geometryBuckets>, solidId: string): ReadonlySet<string> {
  const out = new Set<string>();
  const solid = buckets.solids[solidId];
  if (!solid) return out;
  const visitShell = (shellId: string): void => {
    if (out.has(`shell:${shellId}`)) return;
    out.add(`shell:${shellId}`);
    const shell = buckets.shells[shellId];
    if (!shell) return;
    for (const faceId of shell.faceIds) visitFace(faceId);
  };
  const visitFace = (faceId: string): void => {
    if (out.has(`face:${faceId}`)) return;
    out.add(`face:${faceId}`);
    const face = buckets.faces[faceId];
    if (!face) return;
    for (const wireId of face.wireIds) visitWire(wireId);
  };
  const visitWire = (wireId: string): void => {
    if (out.has(`wire:${wireId}`)) return;
    out.add(`wire:${wireId}`);
    const wire = buckets.wires[wireId];
    if (!wire) return;
    for (const edgeId of wire.edgeIds) visitEdge(edgeId);
  };
  const visitEdge = (edgeId: string): void => {
    if (out.has(`edge:${edgeId}`)) return;
    out.add(`edge:${edgeId}`);
    const edge = buckets.edges[edgeId];
    if (!edge) return;
    for (const vertexId of edge.vertexIds) out.add(`vertex:${vertexId}`);
  };
  out.add(`solid:${solidId}`);
  for (const shellId of solid.shellIds) visitShell(shellId);
  return out;
}

export function buildGeometryTypologyIndex(model: Model, modelDefinitionId: string): ReadonlyMap<string, string> {
  const buckets = geometryBuckets(model);
  const out = new Map<string, string>();
  for (const row of listModelObjectsForModelDefinition(model, modelDefinitionId)) {
    for (const [, primitiveRef] of objectPrimitiveEntries(row)) {
      const kind = resolvePrimitiveRefKind(model, primitiveRef);
      if (kind === "solid") {
        for (const key of collectSolidPrimitiveMemberIds(buckets, primitiveRef)) out.set(key, row.typology);
        continue;
      }
      out.set(`${kind}:${primitiveRef}`, row.typology);
    }
  }
  for (const anchor of geometryRecords(buckets.anchors)) {
    const attachment = anchor.attachment;
    const mapped = out.get(`${attachment.kind}:${attachment.id}`);
    if (mapped) out.set(`anchor:${anchor.id}`, mapped);
  }
  return out;
}

/** 🧭️ Maps factory geometry member keys (`vertex:v0`, `solid:s0`, …) to owning object ids for reveal gating. */
export function buildGeometryObjectIndex(model: Model, modelDefinitionId: string): ReadonlyMap<string, string> {
  const buckets = geometryBuckets(model);
  const out = new Map<string, string>();
  const mapObjectPrimitive = (objectId: string, primitiveRef: string): void => {
    const kind = resolvePrimitiveRefKind(model, primitiveRef) ?? "solid";
    out.set(`${kind}:${primitiveRef}`, objectId);
    if (kind === "solid") {
      out.set(`object:${primitiveRef}`, objectId);
      for (const key of collectSolidPrimitiveMemberIds(buckets, String(primitiveRef))) out.set(key, objectId);
    }
  };
  for (const row of listModelObjectsForModelDefinition(model, modelDefinitionId)) {
    const objectId = String(row.id);
    out.set(`object:${objectId}`, objectId);
    for (const [, primitiveRef] of objectPrimitiveEntries(row)) mapObjectPrimitive(objectId, String(primitiveRef));
  }
  for (const solid of geometryRecords(buckets.solids)) {
    const solidId = solid.id;
    if (out.has(`solid:${solidId}`)) continue;
    out.set(`solid:${solidId}`, solidId);
    out.set(`object:${solidId}`, solidId);
    for (const key of collectSolidPrimitiveMemberIds(buckets, solidId)) {
      if (!out.has(key)) out.set(key, solidId);
    }
  }
  for (const anchor of geometryRecords(buckets.anchors)) {
    const attachment = anchor.attachment;
    const mapped = out.get(`${attachment.kind}:${attachment.id}`);
    if (mapped) out.set(`anchor:${anchor.id}`, mapped);
  }
  return out;
}

function createModelObjectSpatialPickTargets(model: Model, modelDefinitionId: string): readonly SpatialPickTarget[] {
  const targets: SpatialPickTarget[] = [];
  for (const row of listModelObjectsForModelDefinition(model, modelDefinitionId)) {
    const points = modelObjectPickPoints(model, row);
    const point = geometryPointCentroid(points);
    if (!point) continue;
    targets.push({
      kind: "object",
      id: String(row.id),
      point,
      points: points.length ? points : undefined,
      typologyId: row.typology,
    });
  }
  return targets;
}

function appendPrimitiveSpatialPickTargets(
  targets: SpatialPickTarget[],
  buckets: ReturnType<typeof geometryBuckets>,
  entityKinds: ReadonlySet<ModelEntityKind>,
  geometryTypologyIndex: ReadonlyMap<string, string>,
  skipSolidIds: ReadonlySet<string> = new Set(),
): void {
  const withTypology = (target: Omit<SpatialPickTarget, "typologyId"> & { readonly geometryKind: ModelEntityKind }): SpatialPickTarget => ({
    ...target,
    typologyId: geometryTypologyIndex.get(`${target.geometryKind}:${target.id}`),
  });
  if (entityKinds.has("anchor")) {
    for (const anchor of geometryRecords(buckets.anchors)) {
      targets.push(withTypology({ kind: "vertex", geometryKind: "anchor", id: anchor.id, point: anchor.position }));
    }
  }
  if (entityKinds.has("vertex")) {
    for (const vertex of geometryRecords(buckets.vertices)) {
      targets.push(withTypology({ kind: "vertex", geometryKind: "vertex", id: vertex.id, point: vertex.position }));
    }
  }
  if (entityKinds.has("edge")) {
    for (const edge of geometryRecords(buckets.edges)) {
      const points = geometryEdgePoints(buckets.vertices, edge);
      const point = geometryPointCentroid(points);
      if (point) targets.push(withTypology({ kind: "edge", geometryKind: "edge", id: edge.id, point, points }));
    }
  }
  if (entityKinds.has("wire")) {
    for (const wire of geometryRecords(buckets.wires)) {
      const points = geometryWirePoints(buckets.vertices, buckets.edges, wire);
      const point = geometryPointCentroid(points);
      if (point) targets.push(withTypology({ kind: "edge", geometryKind: "wire", id: wire.id, point, points }));
    }
  }
  if (entityKinds.has("face")) {
    for (const face of geometryRecords(buckets.faces)) {
      const points = geometryFacePoints(buckets.vertices, buckets.edges, buckets.wires, face);
      const point = geometryPointCentroid(points);
      if (point) targets.push(withTypology({ kind: "face", geometryKind: "face", id: face.id, point, points }));
    }
  }
  if (entityKinds.has("shell")) {
    for (const shell of geometryRecords(buckets.shells)) {
      const segments = geometryEntityWireSegments(buckets, "shell", shell.id);
      const points = segments.flatMap(([a, b]) => [a, b]);
      const point = geometryPointCentroid(points);
      if (point) targets.push(withTypology({ kind: "face", geometryKind: "shell", id: shell.id, point, points: points.length ? points : undefined }));
    }
  }
  if (entityKinds.has("solid")) {
    const all = geometryAllVertexPoints(buckets.vertices);
    const allCenter = geometryPointCentroid(all);
    for (const cell of geometryRecords(buckets.solids)) {
      if (skipSolidIds.has(cell.id)) continue;
      const points = geometrySolidPoints(buckets.vertices, buckets.edges, buckets.wires, buckets.faces, buckets.shells, cell);
      const point = geometryPointCentroid(points) ?? allCenter;
      if (point) targets.push(withTypology({ kind: "object", geometryKind: "solid", id: cell.id, point, points: points.length ? points : all }));
    }
  }
}

/** 🧲️ Builds renderer-side snap/select targets from factory geometry and typology object rows. */
export function createSpatialPickTargets(geometry: SpatialPickGeometry | null | undefined, activeModelDefinitionId?: string | null): readonly SpatialPickTarget[] {
  if (!geometry) return [];
  const buckets = geometryBuckets(geometry);
  const model = geometry instanceof Model ? geometry : parseModelJson(geometry as ModelJson);
  if (!model) return [];
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  const entityKinds = new Set(modelDefinitionSelectionEntityKinds(mdId));
  const scopedObjects = !isShapeModelDefinition(mdId) ? listModelObjectsForModelDefinition(model, mdId) : [];
  const geometryTypologyIndex = buildGeometryTypologyIndex(model, mdId);
  const targets: SpatialPickTarget[] = [];
  if (entityKinds.has("object") && scopedObjects.length > 0) targets.push(...createModelObjectSpatialPickTargets(model, mdId));
  if (modelDefinitionUsesGeometryPicking(mdId)) {
    const skipSolidIds = new Set(scopedObjects.map((row) => objectPrimaryPrimitiveRef(row)).filter((primitiveRef): primitiveRef is string => typeof primitiveRef === "string" && primitiveRef.length > 0));
    appendPrimitiveSpatialPickTargets(targets, buckets, entityKinds, geometryTypologyIndex, skipSolidIds);
  }
  return targets;
}

export function filterSpatialPickTargets(targets: readonly SpatialPickTarget[], accept: readonly ModelEntityKind[] = [], toggles: SpatialPickKindToggles = {}): SpatialPickTarget[] {
  const acceptSet = accept.length > 0 ? new Set(accept) : null;
  const acceptKinds = spatialPickKindsForSelectionAccept(accept);
  return targets.filter((target) => {
    if (toggles[target.kind] === false) return false;
    if (!acceptKinds) return true;
    if (acceptKinds.has(target.kind)) return true;
    const primitive = pickTargetPrimitiveKind(target);
    return primitive !== null && (acceptSet?.has(primitive) ?? false);
  });
}

/** 🧲️ Creates a statechart event carrying snapped point plus selected geometry metadata. */
export function createSpatialPickEvent(kind: SpatialPickKind, point: Vec3, target: SpatialPickTarget | null, modifiers: InteractionEvent["modifiers"] = {}): InteractionEvent {
  const geometryKind = target?.kind === "object" && !target.geometryKind ? "object" : target ? kernelGeometryKindForObjectPick(target.kind as SpatialGeometryPickTargetKind, target.geometryKind) : undefined;
  return target && geometryKind
    ? {
        kind,
        point,
        modifiers,
        snap: { kind: geometryKind, id: target.id, point: target.point },
        selection: { kind: geometryKind, id: target.id },
      }
    : { kind, point, modifiers };
}
// #endregion 🧲️GeometryTargets

// #region 🧲️GeometryInteraction
/** 🎨️ Maps a resolved typology style to committed-mesh material props. */
export function typologyStyleToMaterialProps(style: ResolvedTypologyStyle): { readonly color: string; readonly emissive: string; readonly opacity: number } {
  return { color: style.color, emissive: style.color, opacity: style.opacity };
}

export function targetStyle(target: SpatialPickTarget, hovered: boolean, selected: boolean, typologyStyle?: ResolvedTypologyStyle, locked = false): { color: string; emissive: string; opacity: number; lineWidth: number } {
  const palette = spatialSceneColors();
  if (selected) return { color: palette.selected, emissive: palette.selectedEmissive, opacity: target.kind === "vertex" ? 1 : 0.34, lineWidth: 9 };
  if (hovered) return { color: palette.hovered, emissive: palette.hoveredEmissive, opacity: target.kind === "vertex" ? 1 : 0.28, lineWidth: 8 };
  if (locked) {
    const dimOpacity = (target.kind === "vertex" ? 0.55 : 0.12) * WORLD_LOCKED_OPACITY_SCALE;
    if (typologyStyle) {
      return { color: typologyStyle.color, emissive: typologyStyle.color, opacity: dimOpacity, lineWidth: 4 };
    }
    if (target.kind === "vertex") return { color: palette.vertex, emissive: palette.vertexEmissive, opacity: dimOpacity, lineWidth: 4 };
    if (target.kind === "edge") return { color: palette.edge, emissive: palette.edgeEmissive, opacity: dimOpacity, lineWidth: 4 };
    if (target.kind === "object" && !target.geometryKind) return { color: palette.object, emissive: palette.objectEmissive, opacity: dimOpacity, lineWidth: 5 };
    return { color: palette.face, emissive: palette.faceEmissive, opacity: dimOpacity, lineWidth: 4 };
  }
  if (typologyStyle) {
    if (target.kind === "vertex") return { color: typologyStyle.color, emissive: typologyStyle.color, opacity: 1, lineWidth: 5 };
    if (target.kind === "edge") return { color: typologyStyle.edgeColor, emissive: typologyStyle.edgeColor, opacity: 0.85, lineWidth: 5 };
    if (target.kind === "object" && !target.geometryKind) return { color: typologyStyle.color, emissive: typologyStyle.color, opacity: typologyStyle.opacity, lineWidth: 7 };
    return { color: typologyStyle.color, emissive: typologyStyle.color, opacity: Math.min(0.42, typologyStyle.opacity), lineWidth: 5 };
  }
  if (target.kind === "vertex") return { color: palette.vertex, emissive: palette.vertexEmissive, opacity: 1, lineWidth: 5 };
  if (target.kind === "edge") return { color: palette.edge, emissive: palette.edgeEmissive, opacity: 0.8, lineWidth: 5 };
  if (target.kind === "object" && !target.geometryKind) return { color: palette.object, emissive: palette.objectEmissive, opacity: 0.28, lineWidth: 7 };
  return { color: palette.face, emissive: palette.faceEmissive, opacity: 0.16, lineWidth: 5 };
}

export function pinnedPickTargetKeys(keys: ReadonlySet<string>): ReadonlySet<string> {
  const out = new Set<string>();
  for (const key of keys) {
    out.add(key);
    const colon = key.indexOf(":");
    if (colon < 0) continue;
    const kind = key.slice(0, colon);
    const id = key.slice(colon + 1);
    const mapped = GEOMETRY_KIND_TO_OBJECT_PICK[kind as ModelEntityKind];
    if (mapped) out.add(`${mapped}:${id}`);
    if (kind === "object") out.add(`solid:${id}`);
  }
  return out;
}

/** 🪪️ Expands a hover/selection key across geometry pick aliases (`solid:foo` ↔ `object:foo`). */
export function spatialHoverKeyAliases(key: string | null | undefined): ReadonlySet<string> {
  if (!key) return new Set();
  return pinnedPickTargetKeys(new Set([key]));
}

/** 🪪️ True when a canvas pick key matches a shared hover/selection key (including aliases). */
export function spatialHoverKeysMatch(left: string | null | undefined, right: string | null | undefined): boolean {
  if (!left || !right) return false;
  if (left === right) return true;
  return spatialHoverKeyAliases(left).has(right);
}
// #endregion 🧲️GeometryInteraction

// #region 🎨️TypologyPatternMaterial
const typologyPatternMaterialCache = ephemeralMap<string, ThreeMeshStandardMaterial>("s.plugins.cad.modules.renderer.component.tsx.typologyPatternMaterialCache");

function typologyPatternKindUniform(kind: ResolvedTypologyStyle["pattern"]["kind"]): number {
  if (kind === "hatch") return 1;
  if (kind === "crosshatch") return 2;
  if (kind === "dots") return 3;
  return 0;
}

const TYPOLOGY_PATTERN_VERTEX_PREFIX = "varying vec3 vTypologyWorldPos;\n";
const TYPOLOGY_PATTERN_VERTEX_INJECT = `#include <worldpos_vertex>
vec4 typologyWorldPosition = vec4(transformed, 1.0);
#ifdef USE_BATCHING
  typologyWorldPosition = batchingMatrix * typologyWorldPosition;
#endif
#ifdef USE_INSTANCING
  typologyWorldPosition = instanceMatrix * typologyWorldPosition;
#endif
typologyWorldPosition = modelMatrix * typologyWorldPosition;
vTypologyWorldPos = typologyWorldPosition.xyz;
`;
const TYPOLOGY_PATTERN_FRAGMENT_PREFIX = "varying vec3 vTypologyWorldPos;\nuniform float uPatternKind;\nuniform float uPatternDirection;\nuniform float uPatternSpacing;\nuniform float uPatternLineWidth;\nuniform vec3 uPatternColor;\n";
const TYPOLOGY_PATTERN_FRAGMENT_BLEND = `
vec3 applyTypologyPattern(vec3 baseColor, vec3 worldPos, vec3 surfaceNormal) {
  if (uPatternKind < 0.5) return baseColor;
  vec3 n = normalize(surfaceNormal);
  vec3 ref = abs(n.z) < 0.9 ? vec3(0.0, 0.0, 1.0) : vec3(0.0, 1.0, 0.0);
  vec3 tangent = normalize(cross(n, ref));
  vec3 bitangent = cross(n, tangent);
  float angle = uPatternDirection;
  vec2 planeCoord = vec2(dot(worldPos, tangent), dot(worldPos, bitangent));
  mat2 rot = mat2(cos(angle), -sin(angle), sin(angle), cos(angle));
  planeCoord = rot * planeCoord;
  float hatch = 0.0;
  if (uPatternKind < 1.5) {
    float stripe = abs(fract(planeCoord.x / uPatternSpacing) - 0.5) * uPatternSpacing;
    hatch = 1.0 - smoothstep(0.0, uPatternLineWidth, stripe);
  } else if (uPatternKind < 2.5) {
    float stripeA = abs(fract(planeCoord.x / uPatternSpacing) - 0.5) * uPatternSpacing;
    float stripeB = abs(fract(planeCoord.y / uPatternSpacing) - 0.5) * uPatternSpacing;
    float hatchA = 1.0 - smoothstep(0.0, uPatternLineWidth, stripeA);
    float hatchB = 1.0 - smoothstep(0.0, uPatternLineWidth, stripeB);
    hatch = max(hatchA, hatchB);
  } else {
    vec2 cell = planeCoord / uPatternSpacing;
    vec2 grid = fract(cell) - 0.5;
    hatch = 1.0 - smoothstep(uPatternLineWidth * 0.5, uPatternLineWidth, length(grid));
  }
  return mix(baseColor, uPatternColor, hatch * 0.88);
}
`;

/** 🎨️ Builds or reuses a shaded material with optional procedural typology pattern. */
export function createTypologyStyledMaterial(style: ResolvedTypologyStyle): ThreeMeshStandardMaterial {
  const key = typologyStyleCacheKey(style);
  const cached = typologyPatternMaterialCache.get(key);
  if (cached) return cached;
  const props = typologyStyleToMaterialProps(style);
  const material = new THREE.MeshStandardMaterial({
    color: props.color,
    emissive: props.emissive,
    emissiveIntensity: 0.08,
    metalness: 0,
    roughness: 0.45,
    side: THREE.DoubleSide,
    transparent: true,
    opacity: props.opacity,
    depthWrite: false,
    polygonOffset: true,
    polygonOffsetFactor: 1,
    polygonOffsetUnits: 1,
  });
  if (style.pattern.kind !== "none") {
    const pattern = style.pattern;
    material.onBeforeCompile = (shader) => {
      shader.uniforms.uPatternKind = { value: typologyPatternKindUniform(pattern.kind) };
      shader.uniforms.uPatternDirection = { value: (pattern.direction * Math.PI) / 180 };
      shader.uniforms.uPatternSpacing = { value: pattern.spacing };
      shader.uniforms.uPatternLineWidth = { value: pattern.lineWidth };
      shader.uniforms.uPatternColor = { value: new THREE.Color(pattern.color) };
      shader.vertexShader = shader.vertexShader.replace("#include <common>", `#include <common>\n${TYPOLOGY_PATTERN_VERTEX_PREFIX}`);
      shader.vertexShader = shader.vertexShader.replace("#include <worldpos_vertex>", TYPOLOGY_PATTERN_VERTEX_INJECT);
      shader.fragmentShader = shader.fragmentShader.replace("#include <common>", `#include <common>\n${TYPOLOGY_PATTERN_FRAGMENT_PREFIX}`);
      shader.fragmentShader = shader.fragmentShader.replace("#include <output_fragment>", `${TYPOLOGY_PATTERN_FRAGMENT_BLEND}outgoingLight = applyTypologyPattern(outgoingLight, vTypologyWorldPos, normal);\n#include <output_fragment>`);
    };
    material.customProgramCacheKey = () => key;
  }
  typologyPatternMaterialCache.set(key, material);
  return material;
}
// #endregion 🎨️TypologyPatternMaterial

// #region 🧊️CommittedMesh
const CAD_WORLD_CHUNK_SIZE = 256;
const CAD_WORLD_MAX_DISTANCE = 8000;
const cadMeshGeometryPool = createTemplatePool<string, ThreeBufferGeometry>();

/** 📍️ Chunk anchor at mesh bounds center for view-radius streaming. */
export function meshTransferOrigin(mesh: MeshTransfer): Vec3 {
  return boundsFromMeshTransfers([mesh])?.center ?? [0, 0, 0];
}

/** 👁️ Options for object-scoped committed mesh visibility. */
export interface CommittedMeshVisibilityOptions {
  readonly flagsForId?: (entityId: string) => SpatialEntityFlags;
  readonly typologyToggles?: SpatialTypologyToggles;
  readonly filterKindToggles?: SpatialPickKindToggles;
}

function collectVisibleSolidRefsForObject(model: Model, row: SpatialObjectRecord, flagsForId: (entityId: string) => SpatialEntityFlags, out: Set<string>): void {
  for (const [, primitiveRef] of objectPrimitiveEntries(row)) {
    if (resolvePrimitiveRefKind(model, primitiveRef) !== "solid") continue;
    const solidId = String(primitiveRef);
    if (flagsForId(solidId).hidden === true) continue;
    out.add(solidId);
  }
}

function collectVisibleFaceRefsForObject(model: Model, row: SpatialObjectRecord, flagsForId: (entityId: string) => SpatialEntityFlags, out: Set<string>): void {
  for (const [, primitiveRef] of objectPrimitiveEntries(row)) {
    if (resolvePrimitiveRefKind(model, primitiveRef) !== "face") continue;
    const faceId = String(primitiveRef);
    if (flagsForId(faceId).hidden === true) continue;
    out.add(faceId);
  }
}

function orderedWireBoundaryPoints(vertices: Readonly<Record<string, VertexRecord>>, edges: Readonly<Record<string, EdgeRecord>>, wire: WireRecord): readonly Vec3[] {
  const points: Vec3[] = [];
  for (const edgeId of wire.edgeIds) {
    const edge = edges[edgeId];
    if (!edge) continue;
    for (const sample of scenePreview().edgeSamplePoints(vertices, edge, 8)) {
      const prev = points[points.length - 1];
      if (!prev || Math.hypot(prev[0] - sample[0], prev[1] - sample[1], prev[2] - sample[2]) > 1e-9) points.push(sample);
    }
  }
  if (points.length >= 2) {
    const first = points[0]!;
    const last = points[points.length - 1]!;
    if (Math.hypot(first[0] - last[0], first[1] - last[1], first[2] - last[2]) < 1e-9) points.pop();
  }
  return points;
}

/** 🧊️ Builds a shaded planar face mesh from factory topology (energy/structure surface primitives). */
export function buildPlanarFaceMeshTransfer(model: Model, faceId: string): MeshTransfer | null {
  const buckets = geometryBuckets(model);
  const face = buckets.faces[faceId];
  if (!face?.wireIds.length) return null;
  const wire = buckets.wires[face.wireIds[0]!];
  if (!wire) return null;
  const boundary = orderedWireBoundaryPoints(buckets.vertices, buckets.edges, wire);
  if (boundary.length < 3) return null;
  const normal = faceNormal(model, face);
  if (!normal) return null;
  const position = new Float32Array(boundary.length * 3);
  const normals = new Float32Array(boundary.length * 3);
  for (let i = 0; i < boundary.length; i++) {
    const p = boundary[i]!;
    position[i * 3] = p[0];
    position[i * 3 + 1] = p[1];
    position[i * 3 + 2] = p[2];
    normals[i * 3] = normal[0];
    normals[i * 3 + 1] = normal[1];
    normals[i * 3 + 2] = normal[2];
  }
  const triangleCount = boundary.length - 2;
  const index = new Uint32Array(triangleCount * 3);
  for (let i = 0; i < triangleCount; i++) {
    index[i * 3] = 0;
    index[i * 3 + 1] = i + 1;
    index[i * 3 + 2] = i + 2;
  }
  return {
    position,
    normal: normals,
    index,
    edges: new Float32Array(0),
    faceGroups: [{ start: 0, count: triangleCount * 3, entityId: faceId as kernelGeometry.FaceRef }],
    edgeGroups: [],
    faceInfos: [{ entityId: faceId as kernelGeometry.FaceRef, surfaceType: face.surface?.kind ?? "plane", area: 0, normal: [normal[0], normal[1], normal[2]] }],
    edgeInfos: [],
  };
}

/** 👁️ Solid ids eligible for committed mesh draw under a model definition (object-scoped). */
export function visibleSolidRefsForModelDefinition(model: Model, modelDefinitionId: string, options: CommittedMeshVisibilityOptions = {}): ReadonlySet<string> {
  const flagsForId = options.flagsForId ?? ((): SpatialEntityFlags => ({}));
  const typologyToggles = options.typologyToggles ?? {};
  const objectVisible = options.filterKindToggles?.object !== false;
  const scoped = listModelObjectsForModelDefinition(model, modelDefinitionId);
  const out = new Set<string>();
  if (scoped.length > 0) {
    if (!objectVisible) return out;
    for (const row of scoped) {
      const objectId = String(row.id);
      if (flagsForId(objectId).hidden === true) continue;
      if (typologyToggles[row.typology] === false) continue;
      collectVisibleSolidRefsForObject(model, row, flagsForId, out);
    }
    return out;
  }
  if (!isShapeModelDefinition(modelDefinitionId)) return out;
  if (!objectVisible) return out;
  for (const solidId of Object.keys(model.solids)) {
    if (flagsForId(solidId).hidden === true) continue;
    out.add(solidId);
  }
  return out;
}

/** 👁️ Face ids eligible for factory surface shading under a model definition (typology surface primitives only). */
export function visibleFaceRefsForModelDefinition(model: Model, modelDefinitionId: string, options: CommittedMeshVisibilityOptions = {}): ReadonlySet<string> {
  const flagsForId = options.flagsForId ?? ((): SpatialEntityFlags => ({}));
  const typologyToggles = options.typologyToggles ?? {};
  const objectVisible = options.filterKindToggles?.object !== false;
  const faceVisible = options.filterKindToggles?.face !== false;
  const scoped = listModelObjectsForModelDefinition(model, modelDefinitionId);
  const out = new Set<string>();
  if (scoped.length === 0 || !objectVisible || !faceVisible) return out;
  for (const row of scoped) {
    const objectId = String(row.id);
    if (flagsForId(objectId).hidden === true) continue;
    if (typologyToggles[row.typology] === false) continue;
    collectVisibleFaceRefsForObject(model, row, flagsForId, out);
  }
  return out;
}

export interface FactoryFaceMeshRow {
  readonly faceId: string;
  readonly mesh: MeshTransfer;
  readonly style?: ResolvedTypologyStyle;
}

/** 🧊️ Lists planar face meshes for typology-owned surface primitives (energy/structure panes). */
export function listFactoryFaceMeshesForModelDefinition(model: Model, modelDefinitionId: string, options: CommittedMeshVisibilityOptions = {}): readonly FactoryFaceMeshRow[] {
  const allowed = visibleFaceRefsForModelDefinition(model, modelDefinitionId, options);
  if (allowed.size === 0) return [];
  const typologyIndex = buildGeometryTypologyIndex(model, modelDefinitionId);
  const rows: FactoryFaceMeshRow[] = [];
  for (const faceId of allowed) {
    const mesh = buildPlanarFaceMeshTransfer(model, faceId);
    if (!mesh || !isRenderableMeshTransfer(mesh)) continue;
    const typology = typologyIndex.get(`face:${faceId}`);
    rows.push({ faceId, mesh, style: typology ? resolveTypologyStyle(typology) : undefined });
  }
  return rows;
}

/** 👁️ Filters tessellated committed meshes to visible object-owned solids. */
export function filterCommittedMeshesForModelDefinition(
  model: Model,
  modelDefinitionId: string,
  meshes: readonly { readonly solid: SolidRef; readonly mesh: MeshTransfer }[],
  options: CommittedMeshVisibilityOptions = {},
): readonly { readonly solid: SolidRef; readonly mesh: MeshTransfer }[] {
  const allowed = visibleSolidRefsForModelDefinition(model, modelDefinitionId, options);
  if (allowed.size === 0) return [];
  return meshes.filter((row) => allowed.has(String(row.solid)));
}

/** 👁️ Resolves hide/lock flags including object ownership for factory geometry members. */
export function resolveSpatialEntityFlags(model: Model, modelDefinitionId: string, entityId: string): SpatialEntityFlags {
  const direct = model.getEntityFlags(entityId);
  if (direct.hidden === true || direct.locked === true) return direct;
  const objectIndex = buildGeometryObjectIndex(model, modelDefinitionId);
  const ownerId =
    objectIndex.get(`object:${entityId}`) ?? objectIndex.get(`solid:${entityId}`) ?? objectIndex.get(`face:${entityId}`) ?? objectIndex.get(`edge:${entityId}`) ?? objectIndex.get(`vertex:${entityId}`) ?? objectIndex.get(`anchor:${entityId}`);
  if (!ownerId) return direct;
  const ownerFlags = model.getEntityFlags(ownerId);
  if (ownerFlags.hidden !== true && ownerFlags.locked !== true) return direct;
  return { ...direct, ...(ownerFlags.hidden === true ? { hidden: true } : {}), ...(ownerFlags.locked === true ? { locked: true } : {}) };
}

/** 🧊️ Builds a Three.js `BufferGeometry` from a kernel `MeshTransfer` (face groups preserved). */
export function buildBufferGeometryFromMeshTransfer(data: MeshTransfer): ThreeBufferGeometry {
  const geo = new THREE.BufferGeometry();
  if (!isRenderableMeshTransfer(data)) return geo;
  geo.setAttribute("position", new THREE.Float32BufferAttribute(data.position, 3));
  geo.setAttribute("normal", new THREE.Float32BufferAttribute(data.normal, 3));
  geo.setIndex(new THREE.BufferAttribute(data.index, 1));
  for (const g of data.faceGroups) geo.addGroup(g.start, g.count, 0);
  return geo;
}

/** 🎯️ Maps a picked triangle index to B-Rep `FaceInfo` via grouped buffer ranges. */
export function resolveFaceInfoFromTriangleIndex(mesh: MeshTransfer, triangleIndex: number | null | undefined): FaceInfo | null {
  if (triangleIndex === null || triangleIndex === undefined) return null;
  const group = findFaceGroupAt(mesh.faceGroups, triangleIndex);
  if (!group) return null;
  return mesh.faceInfos.find((info: FaceInfo) => info.entityId === group.entityId) ?? null;
}

/** ➖️ B-Rep edge overlay from `MeshTransfer.edges` (kernel `meshEdges`, not triangle edges). */
function CommittedEdgeOverlay({ data, visible = true, edgeColor }: { readonly data: MeshTransfer; readonly visible?: boolean; readonly edgeColor?: string }): ReactNode {
  const geometry = reactHostPort.useMemo(() => {
    const geo = new THREE.BufferGeometry();
    geo.setAttribute("position", new THREE.BufferAttribute(data.edges, 3));
    return geo;
  }, [data.edges]);
  reactHostPort.useEffect(() => () => geometry.dispose(), [geometry]);
  if (!visible) return null;
  return (
    <lineSegments geometry={geometry} raycast={raycastNone}>
      <lineBasicMaterial color={edgeColor ?? spatialSceneColors().foreground} depthTest />
    </lineSegments>
  );
}

/** 🎨️ Resolves face material properties (color, emissive, intensity, opacity, transparent) for committed solid meshes, handling selection and hover states. */
export function resolveCommittedMeshMaterialProps(
  style: ResolvedTypologyStyle | undefined,
  defaultColor: string | undefined,
  solidId: SolidRef | undefined,
  hoveredTargetKey: string | null | undefined,
  selectedTargetKey: string | null | undefined,
  selectedTargetKeys: ReadonlySet<string> | null | undefined,
): {
  readonly color: string;
  readonly emissive: string;
  readonly emissiveIntensity: number;
  readonly opacity: number;
  readonly transparent: boolean;
} {
  const targetKey = solidId ? `solid:${solidId}` : null;
  const hovered = targetKey ? spatialHoverKeysMatch(hoveredTargetKey, targetKey) : false;
  const selected = targetKey ? (selectedTargetKeys ? [...selectedTargetKeys].some((key) => spatialHoverKeysMatch(key, targetKey)) : spatialHoverKeysMatch(selectedTargetKey, targetKey)) : false;

  const palette = spatialSceneColors();
  if (selected) {
    return {
      color: palette.selected,
      emissive: palette.selectedEmissive,
      emissiveIntensity: 0.35,
      opacity: 0.34,
      transparent: true,
    };
  }
  if (hovered) {
    return {
      color: palette.hovered,
      emissive: palette.hoveredEmissive,
      emissiveIntensity: 0.35,
      opacity: 0.28,
      transparent: true,
    };
  }
  if (style) {
    return {
      color: style.color,
      emissive: style.color,
      emissiveIntensity: 0.08,
      opacity: style.opacity ?? COMMITTED_MESH_FACE_OPACITY,
      transparent: style.opacity !== undefined && style.opacity < 1,
    };
  }
  return {
    color: defaultColor ?? palette.committed,
    emissive: defaultColor ?? palette.committedEmissive,
    emissiveIntensity: 0.08,
    opacity: COMMITTED_MESH_FACE_OPACITY,
    transparent: true,
  };
}

export interface TessellatedCommitMeshProps {
  readonly mesh: MeshTransfer;
  readonly style?: ResolvedTypologyStyle;
  readonly pickable?: boolean;
  readonly showFaces?: boolean;
  readonly showEdges?: boolean;
  readonly onFacePointerMove?: (info: FaceInfo, event: ThreeEvent<PointerEvent>) => void;
  readonly onFacePointerDown?: (info: FaceInfo, event: ThreeEvent<PointerEvent>) => void;
  readonly solidId?: SolidRef;
  readonly hoveredTargetKey?: string | null;
  readonly selectedTargetKey?: string | null;
  readonly selectedTargetKeys?: ReadonlySet<string> | null;
}

export const COMMITTED_MESH_FACE_OPACITY = 0.72;

/** 🧊️ Shaded B-Rep mesh + edge overlay; optional face picking via `faceIndex`. */
export function TessellatedCommitMesh({
  mesh: data,
  style,
  pickable = false,
  showFaces = true,
  showEdges = true,
  onFacePointerMove,
  onFacePointerDown,
  solidId,
  hoveredTargetKey,
  selectedTargetKey,
  selectedTargetKeys,
}: TessellatedCommitMeshProps): ReactNode {
  const targetKey = solidId ? `solid:${solidId}` : null;
  const hovered = targetKey ? spatialHoverKeysMatch(hoveredTargetKey, targetKey) : false;
  const selected = targetKey ? (selectedTargetKeys ? [...selectedTargetKeys].some((key) => spatialHoverKeysMatch(key, targetKey)) : spatialHoverKeysMatch(selectedTargetKey, targetKey)) : false;

  const geometryKey = meshTransferContentKey(data);
  const geometry = reactHostPort.useMemo(() => {
    cadMeshGeometryPool.acquire(geometryKey);
    const template = cadMeshGeometryPool.getOrCreate(geometryKey, () => buildBufferGeometryFromMeshTransfer(data));
    return template.clone();
  }, [geometryKey, data.position, data.normal, data.index, data.faceGroups]);
  reactHostPort.useEffect(
    () => () => {
      geometry.dispose();
      cadMeshGeometryPool.release(geometryKey);
    },
    [geometry, geometryKey],
  );
  const faceMaterial = reactHostPort.useMemo(() => {
    const props = resolveCommittedMeshMaterialProps(style, data.color, solidId, hoveredTargetKey, selectedTargetKey, selectedTargetKeys);
    if (!selected && !hovered && style) return createTypologyStyledMaterial(style);
    return new THREE.MeshStandardMaterial({
      color: props.color,
      metalness: 0,
      roughness: 0.45,
      emissive: props.emissive,
      emissiveIntensity: props.emissiveIntensity,
      side: THREE.DoubleSide,
      polygonOffset: true,
      polygonOffsetFactor: 1,
      polygonOffsetUnits: 1,
      transparent: props.transparent,
      opacity: props.opacity,
      depthWrite: false,
    });
  }, [style, data.color, solidId, hoveredTargetKey, selectedTargetKey, selectedTargetKeys, selected, hovered]);
  const edgeColor = style?.edgeColor ?? data.color ?? spatialSceneColors().foreground;
  const faceInfoById = reactHostPort.useMemo(() => {
    const map = new Map<string, FaceInfo>();
    for (const info of data.faceInfos) map.set(String(info.entityId), info);
    return map;
  }, [data.faceInfos]);
  const resolveFace = reactHostPort.useCallback(
    (event: ThreeEvent<PointerEvent>) => {
      const group = findFaceGroupAt(data.faceGroups, event.faceIndex ?? -1);
      if (!group) return null;
      return faceInfoById.get(String(group.entityId)) ?? null;
    },
    [data.faceGroups, faceInfoById],
  );
  if (!showFaces && !showEdges) return null;
  const meshRaycast = pickable ? undefined : raycastNone;
  return (
    <group>
      {showFaces ? (
        <mesh
          geometry={geometry}
          raycast={meshRaycast}
          onPointerMove={
            pickable && onFacePointerMove
              ? (e) => {
                  const info = resolveFace(e);
                  if (info) onFacePointerMove(info, e);
                }
              : undefined
          }
          onPointerDown={
            pickable && onFacePointerDown
              ? (e) => {
                  const info = resolveFace(e);
                  if (!info) return;
                  e.stopPropagation();
                  onFacePointerDown(info, e);
                }
              : undefined
          }
          material={faceMaterial}
        />
      ) : null}
      {data.edges.length > 0 ? <CommittedEdgeOverlay data={data} visible={showEdges} edgeColor={edgeColor} /> : null}
    </group>
  );
}

function ChunkedCommitMeshRow(
  props: TessellatedCommitMeshProps & {
    readonly origin: Vec3;
    readonly rowKey: string;
  },
): ReactNode {
  const { origin: _origin, rowKey: _rowKey, ...meshProps } = props;
  return <TessellatedCommitMesh {...meshProps} />;
}

/** 🧊️ Renders typology-owned planar face surfaces (energy/structure surface primitives). */
export function FactoryFaceSurfaceLayer({ faces, modelRevision, visible = true }: { readonly faces: readonly FactoryFaceMeshRow[]; readonly modelRevision?: number; readonly visible?: boolean }): ReactNode {
  if (!visible || faces.length === 0) return null;
  const rev = modelRevision ?? 0;
  return (
    <ViewRadiusLayer chunkSize={CAD_WORLD_CHUNK_SIZE} maxDistance={CAD_WORLD_MAX_DISTANCE}>
      {faces.map((row, i) => (
        <ChunkedCommitMeshRow
          key={`face:${row.faceId}:r${rev}:${meshTransferContentKey(row.mesh, i)}`}
          rowKey={`face:${row.faceId}:r${rev}:${meshTransferContentKey(row.mesh, i)}`}
          origin={meshTransferOrigin(row.mesh)}
          mesh={row.mesh}
          style={row.style}
          showFaces
          showEdges={false}
        />
      ))}
    </ViewRadiusLayer>
  );
}

/** 🧊️ Renders all committed document solids tessellated by the active kernel. */
export function CommittedMeshLayer({
  meshes,
  modelRevision,
  styleForSolid,
  pickable = false,
  showFaces = true,
  showEdges = true,
  onFacePointerMove,
  onFacePointerDown,
  hoveredTargetKey,
  selectedTargetKey,
  selectedTargetKeys,
}: {
  readonly meshes: readonly { readonly solid: SolidRef; readonly mesh: MeshTransfer }[];
  readonly modelRevision?: number;
  readonly styleForSolid?: (solid: SolidRef) => ResolvedTypologyStyle | undefined;
  readonly pickable?: boolean;
  readonly showFaces?: boolean;
  readonly showEdges?: boolean;
  readonly onFacePointerMove?: (info: FaceInfo, event: ThreeEvent<PointerEvent>) => void;
  readonly onFacePointerDown?: (info: FaceInfo, event: ThreeEvent<PointerEvent>) => void;
  readonly hoveredTargetKey?: string | null;
  readonly selectedTargetKey?: string | null;
  readonly selectedTargetKeys?: ReadonlySet<string> | null;
}): ReactNode {
  if (meshes.length === 0 || (!showFaces && !showEdges)) return null;
  const rev = modelRevision ?? 0;
  return (
    <ViewRadiusLayer chunkSize={CAD_WORLD_CHUNK_SIZE} maxDistance={CAD_WORLD_MAX_DISTANCE}>
      {meshes.map((row, i) => (
        <ChunkedCommitMeshRow
          key={`${row.solid}:r${rev}:${meshTransferContentKey(row.mesh, i)}`}
          rowKey={`${row.solid}:r${rev}:${meshTransferContentKey(row.mesh, i)}`}
          origin={meshTransferOrigin(row.mesh)}
          mesh={row.mesh}
          style={styleForSolid?.(row.solid)}
          pickable={pickable}
          showFaces={showFaces}
          showEdges={showEdges}
          onFacePointerMove={onFacePointerMove}
          onFacePointerDown={onFacePointerDown}
          solidId={row.solid}
          hoveredTargetKey={hoveredTargetKey}
          selectedTargetKey={selectedTargetKey}
          selectedTargetKeys={selectedTargetKeys}
        />
      ))}
    </ViewRadiusLayer>
  );
}
// #endregion 🧊️CommittedMesh

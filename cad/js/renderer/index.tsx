/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
// #region 🧲Header
/** @emoji 🎬 `@cad/js/renderer` — CAD renderer (R3F) with {@link InteractionRepl} host props/`on*` callbacks, {@link InteractionCanvas}, and {@link InteractionSpatialView}. See `cad/assets/modelDefinition/spatial.shape/interaction/box.json`. */
// #endregion 🧲Header

// #region 🔌Adapters
import { Button, cn, Input, Label, reactHostPort, sceneHostPort, type EngagementSpec, type ThreeEvent } from "@ui/react";
import { type CSSProperties, type KeyboardEvent, type ReactNode } from "react";
// #endregion 🔌Adapters

// #region 🔌PortWiring
const Canvas = sceneHostPort.fiber.canvas;
const useFrame = sceneHostPort.fiber.useFrame;
const useThree = sceneHostPort.fiber.useThree;
const { Line, OrbitControls, Text, TransformControls } = sceneHostPort.drei;
const createPortal = sceneHostPort.fiber.createPortal;
const THREE = sceneHostPort.three;
const MOUSE = THREE.MOUSE;
THREE.Object3D.DEFAULT_UP.set(0, 0, 1);
// #endregion 🔌PortWiring

import {
  abortActiveInteractionSession,
  applyModelDiff,
  solidRef,
  createInteractionRuntime,
  emptyMeshTransfer,
  DocumentHistory,
  EMPTY_MODEL_DIFF,
  interactionCanConfirmSelection,
  InteractionRegistry,
  isEmptyModelDiff,
  isInteractionSessionActive,
  isShapeModelDefinition,
  defaultModelDefinitionId,
  listModelDefinitionManifests,
  listTransformationsFromModelDefinition,
  listTransformationsIntoModelDefinition,
  listSpatialInteractionsForModelDefinition,
  resolveSpatialInteractionKeyForModelDefinition,
  modelDefinitionSelectionEntityKinds,
  modelDefinitionUsesGeometryPicking,
  PRIMITIVE_MODEL_ENTITY_KINDS,
  countViewObjectsForModelDefinition,
  primaryAttributeSelectionTarget,
  listAttributeDefinitionsForModelDefinitionEntity,
  attributeDefinitionEditorKind,
  attributeDefinitionValueOptions,
  derivePropertyValue,
  listApplicablePropertyDefinitionsForModelDefinition,
  listTypologiesForModelDefinition,
  typologyObjectPascalFromLabel,
  objectPrimitiveEntries,
  resolvePrimitiveRefKind,
  validateAttributeValue,
  resolveModelDefinitionScope,
  type AttributeDefinitionSpec,
  applyTransformation,
  qualifiedTransformationId,
  selectionOperationUsesModelObjects,
  selectionSeedTargetsForOperation,
  loadSpatialInteraction,
  parseModelJson,
  listKeyedInteractionTransitions,
  interactionLengthEntryForState,
  interactionScalarEntryForState,
  interactionInNumericEntryState,
  interactionNumericEntryApplyEvent,
  interactionNumericEntryCommitEvent,
  interactionNumericEntryLockedValue,
  parseNumericCommandLine,
  isFinalInteractionState,
  mergeInteractionSpatial,
  Model,
  type InteractionEvent,
  type InteractionKeybindRow,
  type InteractionRuntime,
  type InteractionRuntimeOptions,
  type InteractionSnapshot,
  type InteractionSpec,
  type DisplayItem,
  type DisplayModel,
  type TransformationSpec,
  kernelGeometry,
  type SpatialKernel,
  type SpatialPreviewKernel,
  type ModelDocument,
  type SelectionTarget,
  type SpatialObjectRecord,
  type ShellRecord,
  type FaceGroup,
  type FaceInfo,
  type MeshTransfer,
  type SpatialInteraction,
  type ModelEntityKind,
  type ModelJson,
  type ObjectRef,
  type Vec3,
  type SpatialComputeMode,
  cadTransformGumballModeToControlsMode,
  selectionTargetsCenter,
  collectTargetVertices,
  selectionTargetsHaveTransformableVertices,
  type CadTransformGumballMode,
  type ModelDiff,
  ensureTypologyObjectFromCreateDiff,
} from "@cad/js/core";

type AnchorRecord = kernelGeometry.AnchorRecord;
type AnchorRef = kernelGeometry.AnchorRef;
type VertexRef = kernelGeometry.VertexRef;
type ShellRef = kernelGeometry.ShellRef;
type SolidRef = kernelGeometry.SolidRef;
type SolidRecord = kernelGeometry.SolidRecord;
type EdgeRecord = kernelGeometry.EdgeRecord;
type FaceRecord = kernelGeometry.FaceRecord;
type ShellRecord = kernelGeometry.ShellRecord;
type VertexRecord = kernelGeometry.VertexRecord;
type WireRecord = kernelGeometry.WireRecord;
type EdgeRef = kernelGeometry.EdgeRef;

export type { SpatialComputeMode };
import { PreciseSpatialKernelMath, preciseSpatialKernelMath } from "@cad/js/kernel/brepjs";

// #region ⚡R3FPreviewKernel
/** @emoji ⚡ Fast approximate `SpatialPreviewKernel` for live R3F previews (lower tessellation). */
export class R3FPreviewKernel extends PreciseSpatialKernelMath {
  override arcSamplePoints = (center: Vec3, start: Vec3, end: Vec3, segments = 12): readonly Vec3[] => preciseSpatialKernelMath.arcSamplePoints(center, start, end, segments);

  override edgeSamplePoints = (vertices: Readonly<Record<string, VertexRecord>>, edge: EdgeRecord, segments = 12): readonly Vec3[] => preciseSpatialKernelMath.edgeSamplePoints(vertices, edge, segments);

  override circleSamplePoints = (center: Vec3, normal: Vec3, radius: number, segments = 24): readonly Vec3[] => preciseSpatialKernelMath.circleSamplePoints(center, normal, radius, segments);

  override nurbsDisplaySamplePoints = (poles: readonly Vec3[], segmentsPerSpan = 6): readonly Vec3[] => preciseSpatialKernelMath.nurbsDisplaySamplePoints(poles, segmentsPerSpan);
}

/** @emoji ⚡ Default fast preview kernel for play and R3F hosts. */
export const r3fPreviewKernel = new R3FPreviewKernel();

const scenePreviewKernelRef: { current: SpatialPreviewKernel } = { current: r3fPreviewKernel };

/** @emoji ⚡ Binds the active scene preview kernel (fast vs precise) for R3F wireframe helpers. */
export function bindScenePreviewKernel(kernel: SpatialPreviewKernel): void {
  scenePreviewKernelRef.current = kernel;
}

function scenePreview(): SpatialPreviewKernel {
  return scenePreviewKernelRef.current;
}
// #endregion ⚡R3FPreviewKernel

// #region 🎬WorkerClient
/** @emoji 🧩 Binary search `faceGroups` by triangle index (playground `ShapeRenderer` pattern). */
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

/** @emoji 🎞️ Debounced `SpatialKernel.tessellate` for R3F hosts (worker-backed brepjs). */
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

/** @emoji 📦 Lists `SolidRef` ids present on a model graph (document solids for tessellation). */
export function listModelSolidRefs(model: Model | ModelJson | null): readonly SolidRef[] {
  if (!model) return [];
  const graph = model instanceof Model ? model : parseModelJson(model);
  if (!graph) return [];
  return Object.keys(graph.solids).map((id) => solidRef(id));
}

/** @emoji 🔑 Stable React key from mesh buffer fingerprints (avoids stale geometry reuse). */
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

/** @emoji 🎞️ Tessellates every model solid through `SpatialKernel.tessellate` (worker-backed). */
export function useDocumentMeshes(kernel: SpatialKernel | null, model: Model, tolerance: number): readonly { readonly solid: SolidRef; readonly mesh: MeshTransfer }[] {
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
    setMeshes([]);
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
  }, [kernel, model, revision, tolerance]);
  return meshes;
}

/** @emoji 📐 Axis-aligned bounds of all mesh positions (for camera auto-fit). */
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

/** @emoji 📐 Axis-aligned bounds of geometry vertex positions (factory / REPL geometry auto-fit). */
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

function mergeSpatialSceneBounds(a: { readonly center: Vec3; readonly radius: number } | null, b: { readonly center: Vec3; readonly radius: number } | null): { readonly center: Vec3; readonly radius: number } | null {
  if (!a) return b;
  if (!b) return a;
  const min: Vec3 = [Math.min(a.center[0] - a.radius, b.center[0] - b.radius), Math.min(a.center[1] - a.radius, b.center[1] - b.radius), Math.min(a.center[2] - a.radius, b.center[2] - b.radius)];
  const max: Vec3 = [Math.max(a.center[0] + a.radius, b.center[0] + b.radius), Math.max(a.center[1] + a.radius, b.center[1] + b.radius), Math.max(a.center[2] + a.radius, b.center[2] + b.radius)];
  const center: Vec3 = [(min[0] + max[0]) / 2, (min[1] + max[1]) / 2, (min[2] + max[2]) / 2];
  const radius = Math.max(Math.sqrt((max[0] - min[0]) ** 2 + (max[1] - min[1]) ** 2 + (max[2] - min[2]) ** 2) / 2, 0.5);
  return { center, radius };
}
// #endregion 🎬WorkerClient

// #region 🪩ArchivedFootprints
/** @emoji 📦 Footprint of a finished axis-aligned box for persistent REPL overlays. */
export interface ArchivedBoxLayout {
  readonly cornerA: Vec3;
  readonly cornerB: Vec3;
  readonly height: number;
}

function isVec3Record(v: unknown): v is Vec3 {
  return Array.isArray(v) && v.length === 3 && v.every((x) => typeof x === "number");
}

/** @emoji 📦 Reads `origin`/`corner`/`height` from post-commit interaction context when present. */
export function tryArchivedBoxFromContext(ctx: Record<string, unknown>): ArchivedBoxLayout | null {
  const o = ctx.origin;
  const c = ctx.corner;
  const h = ctx.height;
  if (!isVec3Record(o) || !isVec3Record(c)) return null;
  const hz = typeof h === "number" && Number.isFinite(h) && h > 0 ? h : null;
  if (hz === null) return null;
  return { cornerA: o, cornerB: c, height: hz };
}

/** @emoji 🧊 True when committed kernel solids own the scene (footprint box previews would duplicate meshes). */
export function modelHasCommittedSolidsForDisplay(model: Model | null | undefined): boolean {
  return listModelSolidRefs(model).length > 0;
}

/** @emoji 📦 Drops axis-aligned footprint `box-preview` display items when kernel solids are present. */
export function filterFootprintBoxPreviewDisplayItems(display: DisplayModel, model: Model | null | undefined): DisplayModel {
  if (!modelHasCommittedSolidsForDisplay(model)) return display;
  const items = display.items.filter((item) => item.kind !== "box-preview");
  return items.length === display.items.length ? display : { ...display, items };
}

function mergeDisplayWithArchivedBoxes(base: DisplayModel, archived: readonly ArchivedBoxLayout[], model: Model | null | undefined): DisplayModel {
  if (archived.length === 0 || modelHasCommittedSolidsForDisplay(model)) return base;
  const extra: DisplayItem[] = archived.map((b, i) => ({
    kind: "box-preview",
    id: `archived-box-${i}`,
    role: "archived",
    params: { cornerA: b.cornerA, cornerB: b.cornerB, height: b.height },
  }));
  return { ...base, items: [...extra, ...base.items] };
}

/** @emoji 📦 True when a history entry should leave a persistent box footprint overlay (not transforms). */
export function historyEntryArchivesBoxFootprint(interactionId: string): boolean {
  if (interactionId.startsWith("transform.")) return false;
  if (interactionId.startsWith("selection.")) return false;
  if (interactionId.startsWith("measure.")) return false;
  if (interactionId === "primitive.box") return true;
  if (interactionId.includes("construct") && /box/i.test(interactionId)) return true;
  return false;
}

function archivedBoxesFromHistory(history: DocumentHistory): readonly ArchivedBoxLayout[] {
  return history
    .entries()
    .filter((mod) => historyEntryArchivesBoxFootprint(mod.interactionId))
    .map((mod) => (mod.result.archiveContext ? tryArchivedBoxFromContext(mod.result.archiveContext) : null))
    .filter((box): box is ArchivedBoxLayout => box !== null);
}

function replBaseDisplayForHistory(snapshot: InteractionSnapshot): DisplayModel {
  if (snapshot.state !== "committed") return snapshot.display;
  return { ...snapshot.display, items: snapshot.display.items.filter((item) => item.role !== "preview") };
}
// #endregion 🪩ArchivedFootprints

// #region 📐Layout
/** @emoji 📐 Center and axis-aligned scale for a unit `BoxGeometry` from two XY footprint corners and height. */
export function computeBoxPreviewLayout(cornerA: Vec3, cornerB: Vec3, height: number, preview: SpatialPreviewKernel = scenePreview()): { readonly position: Vec3; readonly scale: Vec3 } {
  return preview.computeBoxPreviewLayout(cornerA, cornerB, height);
}

/** @emoji 🟦 Center and radius for the live sphere preview while the radius point is moving. */
export function computeSpherePreviewLayout(center: Vec3 | null, cursor: Vec3 | null): { readonly position: Vec3; readonly radius: number } | null {
  if (!center || !cursor) return null;
  const radius = Math.hypot(cursor[0] - center[0], cursor[1] - center[1], cursor[2] - center[2]);
  return radius > 1e-9 ? { position: center, radius } : null;
}

function readVec3(v: unknown): Vec3 | null {
  if (Array.isArray(v) && v.length === 3 && v.every((x) => typeof x === "number")) return v as unknown as Vec3;
  return null;
}

function readNumber(v: unknown): number | null {
  return typeof v === "number" && Number.isFinite(v) ? v : null;
}

function readVec3Array(v: unknown): readonly Vec3[] {
  if (!Array.isArray(v)) return [];
  return v.filter(isVec3Record) as readonly Vec3[];
}

/** @emoji 📦 Axis-aligned bounds for geometry highlight wireframes. */
export function bboxFromPoints(points: readonly Vec3[], preview: SpatialPreviewKernel = scenePreview()): { readonly min: Vec3; readonly max: Vec3 } | null {
  return preview.aabbFromPoints(points);
}

/** @emoji 📦 Twelve edges of an axis-aligned box for preview line rendering. */
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

function parseDisplaySelectionTargets(v: unknown): readonly { readonly kind: ModelEntityKind; readonly id: string }[] {
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

/** @emoji 🖼️ Maps declarative `previewKind` + params to a point transform for geometry wireframes. */
export function transformPointsForPreviewKind(previewKind: string, params: Record<string, unknown>, preview: SpatialPreviewKernel = scenePreview()): (point: Vec3) => Vec3 {
  return preview.transformPointsForPreviewKind(previewKind, params);
}

/** @emoji 🖼️ Active geometry point transform from move/copy/mirror/rotate/scale preview display items. */
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

function previewKindUsesGeometryWireframe(previewKind: string): boolean {
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

const raycastNone: THREE.Object3D["raycast"] = () => undefined;
// #endregion 📐Layout

// #region 🧲GeometryTargets
export type SpatialPickKind = "pointer.down" | "pointer.move";

/** @emoji 🎯 Primitive and object pick kinds for renderer feedback (maps to kernel geometry via {@link SpatialPickTarget.geometryKind}). */
export type SpatialGeometryPickTargetKind = "object" | "face" | "edge" | "vertex";

export type SpatialPickTargetKind = SpatialGeometryPickTargetKind;

export const SPATIAL_PICK_TARGET_KINDS: readonly SpatialPickTargetKind[] = ["object", "face", "edge", "vertex"];

const GEOMETRY_KIND_TO_OBJECT_PICK: Partial<Record<ModelEntityKind, SpatialGeometryPickTargetKind>> = {
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
    if (kind === "object") out.add("object");
  }
  return out;
}

function kernelGeometryKindForObjectPick(kind: SpatialGeometryPickTargetKind, geometryKind?: ModelEntityKind): ModelEntityKind {
  if (geometryKind) return geometryKind;
  if (kind === "vertex") return "vertex";
  if (kind === "edge") return "edge";
  if (kind === "face") return "face";
  return "solid";
}

/** @emoji 👁️ Per-kind on/off map for visibility filters or selection/hover gates (`false` disables). */
export type SpatialPickKindToggles = Partial<Record<SpatialPickTargetKind, boolean>>;

/** @emoji 👁️ Per-typology on/off map for play chrome (`false` disables show or selection). */
export type SpatialTypologyToggles = Partial<Record<string, boolean>>;

export interface SpatialPickTarget {
  readonly kind: SpatialPickTargetKind;
  readonly id: string;
  readonly point: Vec3;
  readonly points?: readonly Vec3[];
  /** @emoji 🧭 Kernel-private geometry entity kind for primitive picks (e.g. `wire` vs `edge`). */
  readonly geometryKind?: ModelEntityKind;
  /** @emoji 🏷️ Typology id when the target belongs to a model-definition object row. */
  readonly typologyId?: string;
}

export interface SpatialSelectionRequest {
  readonly targets: readonly SpatialPickTarget[];
  readonly point: Vec3;
  readonly client: { readonly x: number; readonly y: number };
  readonly modifiers: InteractionEvent["modifiers"];
}

export type SpatialSelectionMethod = "rectangle" | "lasso";
type SpatialSelectionCoverage = "partial" | "full";
type SpatialSelectionMode = "default" | "additive" | "subtractive" | "invertive";

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

/** @emoji 🪪 Stable hover/selection key for a {@link SelectionTarget} (primitive kinds use entity kind + id). */
export function selectionTargetHoverKey(target: SelectionTarget): string {
  return `${target.kind}:${target.id}`;
}

function spatialSelectionTargetKey(target: SelectionTarget): string {
  return selectionTargetHoverKey(target);
}

/** @emoji 👁️ Default all geometry pick kinds enabled (visibility + selection). */
export function defaultSpatialPickKindToggles(): Record<SpatialPickTargetKind, boolean> {
  return Object.fromEntries(SPATIAL_PICK_TARGET_KINDS.map((kind) => [kind, true])) as Record<SpatialPickTargetKind, boolean>;
}

/** @emoji 👁️ Filters pick targets by visibility (show/hide highlights); does not affect ray pick or selection. */
export function filterSpatialPickTargetsForVisibility(targets: readonly SpatialPickTarget[], filterKindToggles: SpatialPickKindToggles = {}): SpatialPickTarget[] {
  return targets.filter((target) => filterKindToggles[target.kind] !== false);
}

/** @emoji 👁️ Effective pick kinds must be both visible and enabled for selection/hover. */
export function intersectSpatialPickKindToggles(visibleKindToggles: SpatialPickKindToggles = {}, selectionKindToggles: SpatialPickKindToggles = {}): SpatialPickKindToggles {
  const merged: SpatialPickKindToggles = {};
  for (const kind of SPATIAL_PICK_TARGET_KINDS) {
    if (visibleKindToggles[kind] === false || selectionKindToggles[kind] === false) merged[kind] = false;
  }
  return merged;
}

/** @emoji 👁️ Maps active model-definition entity kinds to renderer pick-kind toggles. */
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

/** @emoji 👁️ Default visibility/selection toggles for kinds allowed by the active model definition. */
export function defaultSpatialPickKindTogglesForModelDefinition(modelDefinitionId: string | null): Record<SpatialPickTargetKind, boolean> {
  const allowed = new Set(modelDefinitionPickTargetKinds(modelDefinitionId));
  return Object.fromEntries(SPATIAL_PICK_TARGET_KINDS.map((kind) => [kind, allowed.has(kind)])) as Record<SpatialPickTargetKind, boolean>;
}

/** @emoji 👁️ Typology ids declared on the active model definition (sorted). */
export function modelDefinitionTypologyIds(modelDefinitionId: string | null): readonly string[] {
  return listTypologiesForModelDefinition(modelDefinitionId ?? defaultModelDefinitionId())
    .map((row) => row.id)
    .sort((a, b) => a.localeCompare(b));
}

/** @emoji 👁️ Default all typologies on the active model definition enabled for show/selection. */
export function defaultSpatialTypologyTogglesForModelDefinition(modelDefinitionId: string | null): Record<string, boolean> {
  return Object.fromEntries(modelDefinitionTypologyIds(modelDefinitionId).map((id) => [id, true]));
}

/** @emoji 🏷️ Short typology label for play chrome (`Base Plate` → `BasePlate`). */
export function spatialTypologyToggleLabel(typologyId: string, label?: string): string {
  if (label?.trim()) return typologyObjectPascalFromLabel(label);
  const tail = typologyId.split(".").pop() ?? typologyId;
  return typologyObjectPascalFromLabel(tail.replace(/[._-]+/g, " "));
}

function typologyToggleAllowsTarget(target: SpatialPickTarget, toggles: SpatialTypologyToggles, typologyIds: readonly string[]): boolean {
  if (target.typologyId) return toggles[target.typologyId] !== false;
  return typologyIds.some((id) => toggles[id] !== false);
}

/** @emoji 👁️ Filters pick targets by typology show/selection toggles. */
export function filterSpatialPickTargetsForTypologyToggles(targets: readonly SpatialPickTarget[], toggles: SpatialTypologyToggles, typologyIds: readonly string[]): SpatialPickTarget[] {
  return targets.filter((target) => typologyToggleAllowsTarget(target, toggles, typologyIds));
}

/** @emoji 👁️ Derives per-kind toggles from typology-filtered targets (scene layers + legacy gates). */
export function spatialPickKindTogglesFromTypologyFilteredTargets(modelDefinitionId: string | null, visibleTargets: readonly SpatialPickTarget[]): SpatialPickKindToggles {
  const allowed = new Set(modelDefinitionPickTargetKinds(modelDefinitionId));
  const merged: SpatialPickKindToggles = {};
  for (const kind of SPATIAL_PICK_TARGET_KINDS) {
    merged[kind] = allowed.has(kind) && visibleTargets.some((target) => target.kind === kind);
  }
  return merged;
}

/** @emoji 👁️ Per-primitive on/off map for play chrome (`false` disables show or filter). */
export type SpatialPrimitiveToggles = Partial<Record<ModelEntityKind, boolean>>;

/** @emoji 🧱 Factory primitive kinds toggled in play (anchor → solid). */
export const SPATIAL_PRIMITIVE_KINDS: readonly ModelEntityKind[] = PRIMITIVE_MODEL_ENTITY_KINDS;

/** @emoji 👁️ Default all factory primitive kinds enabled for show/filter. */
export function defaultSpatialPrimitiveToggles(): Record<ModelEntityKind, boolean> {
  return Object.fromEntries(SPATIAL_PRIMITIVE_KINDS.map((kind) => [kind, true])) as Record<ModelEntityKind, boolean>;
}

/** @emoji ☑️ Aggregate enabled state for a fixed-key boolean toggle map (`false` = off). */
export type SpatialToggleGroupState = "all" | "none" | "partial";

/** @emoji ☑️ Returns whether every key is on, every key is off, or the group is mixed. */
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

/** @emoji ☑️ Sets every key in a chrome toggle group on or off. */
export function spatialToggleGroupFill<T extends string>(keys: readonly T[], enabled: boolean): Record<T, boolean> {
  return Object.fromEntries(keys.map((key) => [key, enabled])) as Record<T, boolean>;
}

/** @emoji 🧭 Resolves the primitive entity kind for a pick target (typology object rows → `null`). */
export function pickTargetPrimitiveKind(target: SpatialPickTarget): ModelEntityKind | null {
  if (target.kind === "object" && !target.geometryKind) return null;
  return target.geometryKind ?? kernelGeometryKindForObjectPick(target.kind as SpatialGeometryPickTargetKind, target.geometryKind);
}

/** @emoji 👁️ Filters pick targets by primitive show/filter toggles (typology object rows pass through). */
export function filterSpatialPickTargetsForPrimitiveToggles(targets: readonly SpatialPickTarget[], toggles: SpatialPrimitiveToggles): SpatialPickTarget[] {
  return targets.filter((target) => {
    const primitive = pickTargetPrimitiveKind(target);
    if (!primitive) return true;
    return toggles[primitive] !== false;
  });
}

/** @emoji 👁️ Resolves which scene layers stay visible for geometry edit vs typology object picking. */
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

function spatialPickKindsForActiveView(activeModelDefinitionId: string | null): ReadonlySet<SpatialPickTargetKind> {
  return new Set(modelDefinitionPickTargetKinds(activeModelDefinitionId));
}

/** @emoji 👁️ Keeps pick targets allowed by the active model definition (primitives + typology objects). */
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

/** @emoji 🧲 Normalizes `ModelJson` array buckets to the record shape used by interaction math. */
function geometryBuckets(g: SpatialPickGeometry): {
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
    anchors: asRecordBucket((g as ModelJson & { readonly anchors?: readonly AnchorRecord[] }).anchors),
    vertices: asRecordBucket(g.vertices),
    edges: asRecordBucket(g.edges),
    wires: asRecordBucket(g.wires),
    faces: asRecordBucket(g.faces),
    shells: asRecordBucket(g.shells),
    solids: asRecordBucket(g.solids),
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

/** @emoji 📍 NURBS through-points stored on an edge (`through` interpolation curves). */
export function nurbsThroughPointsFromEdge(edge: EdgeRecord): readonly Vec3[] | null {
  const curve = edge.curve;
  if (curve?.kind !== "nurbs" || !curve.through || curve.poles.length < 2) return null;
  return curve.poles;
}

/** @emoji 📍 Through-points for edge/wire pick highlights (interpolate curves). */
export function geometryEntityThroughPoints(buckets: ReturnType<typeof geometryBuckets>, kind: ModelEntityKind, id: string): readonly Vec3[] {
  if (kind === "edge" && buckets.edges[id]) {
    return nurbsThroughPointsFromEdge(buckets.edges[id]!) ?? [];
  }
  if (kind === "wire" && buckets.wires[id]) {
    const poles = buckets.wires[id]!.edgeIds.flatMap((edgeId) => {
      const edge = buckets.edges[edgeId];
      return edge ? (nurbsThroughPointsFromEdge(edge) ?? []) : [];
    });
    return uniqueGeometryPoints(poles);
  }
  return [];
}

function geometryFacePoints(vertices: Record<string, VertexRecord>, edges: Record<string, EdgeRecord>, wires: Record<string, WireRecord>, face: FaceRecord): readonly Vec3[] {
  const ids = face.wireIds.flatMap((wireId) => wires[wireId]?.edgeIds ?? []);
  const points = ids.flatMap((id) => {
    const edge = edges[id];
    return edge ? geometryEdgePoints(vertices, edge) : [];
  });
  const unique = new Map(points.map((p) => [p.join(","), p]));
  return [...unique.values()];
}

function uniqueGeometryPoints(points: readonly Vec3[]): readonly Vec3[] {
  return [...new Map(points.map((p) => [p.join(","), p])).values()];
}

function geometryWirePoints(vertices: Record<string, VertexRecord>, edges: Record<string, EdgeRecord>, wire: WireRecord): readonly Vec3[] {
  return uniqueGeometryPoints(wire.edgeIds.flatMap((id) => (edges[id] ? geometryEdgePoints(vertices, edges[id]!) : [])));
}

function geometryShellPoints(vertices: Record<string, VertexRecord>, edges: Record<string, EdgeRecord>, wires: Record<string, WireRecord>, faces: Record<string, FaceRecord>, shell: ShellRecord): readonly Vec3[] {
  return uniqueGeometryPoints(shell.faceIds.flatMap((id) => (faces[id] ? geometryFacePoints(vertices, edges, wires, faces[id]!) : [])));
}

function geometrySolidPoints(vertices: Record<string, VertexRecord>, edges: Record<string, EdgeRecord>, wires: Record<string, WireRecord>, faces: Record<string, FaceRecord>, shells: Record<string, ShellRecord>, solid: SolidRecord): readonly Vec3[] {
  return uniqueGeometryPoints(solid.shellIds.flatMap((id) => (shells[id] ? geometryShellPoints(vertices, edges, wires, faces, shells[id]!) : [])));
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

/** @emoji 📐 Consecutive segment pairs along a sampled edge polyline. */
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

/** @emoji 📐 Geometry wire segments for previews (edges/wires/faces), bbox fallback for aggregates. */
export function geometryEntityWireSegments(buckets: ReturnType<typeof geometryBuckets>, kind: ModelEntityKind, id: string): readonly (readonly [Vec3, Vec3])[] {
  if (kind === "edge" && buckets.edges[id]) {
    return geometryWireEdgeSegments(buckets.vertices, buckets.edges, { id, edgeIds: [id] } as unknown as WireRecord);
  }
  if (kind === "wire" && buckets.wires[id]) return geometryWireEdgeSegments(buckets.vertices, buckets.edges, buckets.wires[id]!);
  if (kind === "face" && buckets.faces[id]) {
    const face = buckets.faces[id]!;
    return face.wireIds.flatMap((wireId) => {
      const wire = buckets.wires[wireId];
      return wire ? geometryWireEdgeSegments(buckets.vertices, buckets.edges, wire) : [];
    });
  }
  if (kind === "shell" && buckets.shells[id]) {
    return buckets.shells[id]!.faceIds.flatMap((faceId) => geometryEntityWireSegments(buckets, "face", faceId));
  }
  if (kind === "solid" && buckets.solids[id]) {
    return buckets.solids[id]!.shellIds.flatMap((shellId) => geometryEntityWireSegments(buckets, "shell", shellId));
  }
  const pts = geometryEntityPoints(buckets, kind, id);
  const bb = bboxFromPoints(pts);
  return bb ? bboxWireSegments(bb.min, bb.max) : [];
}

/** @emoji 📐 All B-rep edge segments for factory geometry wireframe display. */
export function collectGeometryEdgeSegments(buckets: ReturnType<typeof geometryBuckets>): readonly (readonly [Vec3, Vec3])[] {
  const out: (readonly [Vec3, Vec3])[] = [];
  for (const edge of geometryRecords(buckets.edges)) {
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

function collectSolidPrimitiveMemberIds(buckets: ReturnType<typeof geometryBuckets>, solidId: string): ReadonlySet<string> {
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

function buildGeometryTypologyIndex(model: Model, modelDefinitionId: string): ReadonlyMap<string, string> {
  const typologyIds = new Set(listTypologiesForModelDefinition(modelDefinitionId).map((row) => row.id));
  const buckets = geometryBuckets(model);
  const out = new Map<string, string>();
  for (const row of Object.values(model.objects)) {
    if (!typologyIds.has(row.typology)) continue;
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

function createModelObjectSpatialPickTargets(model: Model, modelDefinitionId: string): readonly SpatialPickTarget[] {
  const typologyIds = new Set(listTypologiesForModelDefinition(modelDefinitionId).map((row) => row.id));
  const targets: SpatialPickTarget[] = [];
  for (const row of Object.values(model.objects)) {
    if (!typologyIds.has(row.typology)) continue;
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

function appendPrimitiveSpatialPickTargets(targets: SpatialPickTarget[], buckets: ReturnType<typeof geometryBuckets>, entityKinds: ReadonlySet<ModelEntityKind>, geometryTypologyIndex: ReadonlyMap<string, string>): void {
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
      const points = geometrySolidPoints(buckets.vertices, buckets.edges, buckets.wires, buckets.faces, buckets.shells, cell);
      const point = geometryPointCentroid(points) ?? allCenter;
      if (point) targets.push(withTypology({ kind: "object", geometryKind: "solid", id: cell.id, point, points: points.length ? points : all }));
    }
  }
}

/** @emoji 🧲 Builds renderer-side snap/select targets from factory geometry and typology object rows. */
export function createSpatialPickTargets(geometry: SpatialPickGeometry | null | undefined, activeModelDefinitionId?: string | null): readonly SpatialPickTarget[] {
  if (!geometry) return [];
  const buckets = geometryBuckets(geometry);
  const model = geometry instanceof Model ? geometry : parseModelJson(geometry as ModelJson);
  if (!model) return [];
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  const entityKinds = new Set(modelDefinitionSelectionEntityKinds(mdId));
  const geometryTypologyIndex = buildGeometryTypologyIndex(model, mdId);
  const targets: SpatialPickTarget[] = [];
  if (modelDefinitionUsesGeometryPicking(mdId)) appendPrimitiveSpatialPickTargets(targets, buckets, entityKinds, geometryTypologyIndex);
  if (entityKinds.has("object") && !isShapeModelDefinition(mdId)) targets.push(...createModelObjectSpatialPickTargets(model, mdId));
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

/** @emoji 🧲 Creates a statechart event carrying snapped point plus selected geometry metadata. */
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
// #endregion 🧲GeometryTargets

// #region 🖼️DisplayPrimitives
function BoxPreviewItem({ item }: { readonly item: DisplayItem }): ReactNode {
  const palette = spatialSceneColors();
  const p = item.params;
  const edgeGeo = reactHostPort.useMemo(() => new THREE.EdgesGeometry(new THREE.BoxGeometry(1, 1, 1)), []);
  if (!p) return null;
  const a = readVec3(p.cornerA);
  const b = readVec3(p.cornerB);
  const hRaw = readNumber(p.height);
  if (!a || !b) return null;
  const h = hRaw === null || hRaw <= 0 ? 0.06 : hRaw;
  const { position, scale } = computeBoxPreviewLayout(a, b, h);
  const archived = item.role === "archived";
  return (
    <group position={position} scale={scale}>
      <mesh raycast={raycastNone}>
        <boxGeometry args={[1, 1, 1]} />
        <meshStandardMaterial
          color={archived ? palette.archived : palette.committed}
          emissive={archived ? palette.archivedEmissive : palette.committedEmissive}
          emissiveIntensity={archived ? 0.22 : 0.35}
          transparent
          opacity={archived ? 0.38 : 0.52}
          depthWrite={false}
        />
      </mesh>
      <lineSegments raycast={raycastNone} geometry={edgeGeo}>
        <lineBasicMaterial color={archived ? palette.archived : palette.committedWire} transparent opacity={archived ? 0.55 : 0.85} />
      </lineSegments>
    </group>
  );
}

function PointItem({ item }: { readonly item: DisplayItem }): ReactNode {
  const palette = spatialSceneColors();
  const pos = readVec3(item.params?.position);
  if (!pos) return null;
  const cursor = item.role === "cursor";
  const r = cursor ? 0.045 : 0.06;
  return (
    <mesh position={pos} raycast={raycastNone}>
      <sphereGeometry args={[r, 16, 16]} />
      <meshStandardMaterial
        color={cursor ? palette.hovered : palette.construction}
        emissive={cursor ? palette.hoveredEmissive : palette.constructionEmissive}
        emissiveIntensity={cursor ? 0.45 : 0.35}
      />
    </mesh>
  );
}

function LinearHandleItem({ item }: { readonly item: DisplayItem }): ReactNode {
  const p = item.params;
  if (!p) return null;
  const origin = readVec3(p.origin);
  const axis = readVec3(p.axis);
  if (!origin || !axis) return null;
  const ax = axis[0];
  const ay = axis[1];
  const az = axis[2];
  const len = Math.hypot(ax, ay, az) || 1;
  const ux = ax / len;
  const uy = ay / len;
  const uz = az / len;
  const span = 5;
  const x1 = origin[0] + ux * span;
  const y1 = origin[1] + uy * span;
  const z1 = origin[2] + uz * span;
  return (
    <Line
      raycast={raycastNone}
      points={[
        [origin[0], origin[1], origin[2]],
        [x1, y1, z1],
      ]}
      color={spatialSceneColors().dimension}
      lineWidth={2}
      dashed={false}
    />
  );
}

function SegmentItem({ item }: { readonly item: DisplayItem }): ReactNode {
  const palette = spatialSceneColors();
  const p = item.params;
  if (!p) return null;
  const a = readVec3(p.from);
  const b = readVec3(p.to);
  if (!a || !b) return null;
  const guide = item.role === "guide";
  const heightLine = item.role === "height";
  return (
    <Line
      raycast={raycastNone}
      points={[
        [a[0], a[1], a[2]],
        [b[0], b[1], b[2]],
      ]}
      color={guide ? palette.guide : heightLine ? palette.hovered : palette.accent}
      lineWidth={guide ? 1 : heightLine ? 2.5 : 2}
      dashed={guide}
      {...(guide ? { dashSize: 0.12, gapSize: 0.08 } : {})}
    />
  );
}

function LabelItem({ item }: { readonly item: DisplayItem }): ReactNode {
  const p = item.params;
  if (!p) return null;
  const pos = readVec3(p.position);
  const text = p.text;
  if (!pos || typeof text !== "string") return null;
  return (
    <reactHostPort.Suspense fallback={null}>
      <Text position={pos} fontSize={0.22} color={spatialSceneColors().foreground} anchorX="left" anchorY="bottom" raycast={raycastNone}>
        {text}
      </Text>
    </reactHostPort.Suspense>
  );
}

function GeometryTargetWireframes({
  geometry,
  targets,
  transform,
  color,
  opacity,
}: {
  readonly geometry: SpatialPickGeometry;
  readonly targets: readonly { readonly kind: ModelEntityKind; readonly id: string }[];
  readonly transform: (point: Vec3) => Vec3;
  readonly color: string;
  readonly opacity: number;
}): ReactNode {
  const buckets = reactHostPort.useMemo(() => geometryBuckets(geometry), [geometry]);
  const segments = reactHostPort.useMemo(() => {
    const out: (readonly [Vec3, Vec3])[] = [];
    for (const target of targets) {
      for (const [a, b] of geometryEntityWireSegments(buckets, target.kind, target.id)) {
        out.push([transform(a), transform(b)]);
      }
    }
    return out;
  }, [buckets, targets, transform]);
  if (!segments.length) return null;
  return (
    <group>
      {segments.map(([a, b], i) => (
        <Line
          key={`${a[0]}-${a[1]}-${a[2]}-${b[0]}-${b[1]}-${b[2]}-${i}`}
          raycast={raycastNone}
          points={[
            [a[0], a[1], a[2]],
            [b[0], b[1], b[2]],
          ]}
          color={color}
          lineWidth={2}
          transparent
          opacity={opacity}
        />
      ))}
    </group>
  );
}

function PreviewItem({ item, geometry }: { readonly item: DisplayItem; readonly geometry?: SpatialPickGeometry | null }): ReactNode {
  const palette = spatialSceneColors();
  const p = item.params;
  if (!p) return null;
  const previewKind = typeof p.previewKind === "string" ? p.previewKind : "preview";
  const targets = parseDisplaySelectionTargets(p.targets);
  const transform = reactHostPort.useMemo(() => transformPointsForPreviewKind(previewKind, p), [previewKind, p]);
  const points = readVec3Array(p.points);
  const cursor = readVec3(p.cursor);
  const prevPoint = readVec3(p.prevPoint);
  const from = readVec3(p.from) ?? prevPoint;
  const linePoints = points.length ? [...points, ...(cursor ? [cursor] : [])] : from && cursor ? [from, cursor] : [];
  const ghost = previewKind === "move-preview" || previewKind === "copy-preview" || previewKind === "mirror-preview";
  const wireColor = previewKind === "selected-objects" || previewKind.endsWith("-selection") ? palette.construction : ghost ? palette.committed : palette.accent;
  const wireOpacity = ghost ? 0.92 : 0.78;
  if (geometry && targets.length && previewKindUsesGeometryWireframe(previewKind)) {
    return (
      <group>
        {ghost ? <GeometryTargetWireframes geometry={geometry} targets={targets} transform={(pt) => pt} color={palette.ghost} opacity={0.35} /> : null}
        <GeometryTargetWireframes geometry={geometry} targets={targets} transform={transform} color={wireColor} opacity={wireOpacity} />
        {from ? (
          <mesh position={from} raycast={raycastNone}>
            <sphereGeometry args={[0.05, 12, 12]} />
            <meshStandardMaterial color={palette.construction} emissive={palette.constructionEmissive} emissiveIntensity={0.4} />
          </mesh>
        ) : null}
        {linePoints.length >= 2 ? <Line raycast={raycastNone} points={linePoints.map((pt) => [pt[0], pt[1], pt[2]])} color={palette.dimension} lineWidth={2} /> : null}
      </group>
    );
  }
  if (previewKind === "sphere" && points.length >= 1 && cursor) {
    const sphere = computeSpherePreviewLayout(points[0]!, cursor);
    if (sphere) {
      return (
        <group>
          <mesh position={sphere.position} raycast={raycastNone}>
            <sphereGeometry args={[sphere.radius, 32, 16]} />
            <meshStandardMaterial color={palette.committed} emissive={palette.committedEmissive} emissiveIntensity={0.28} transparent opacity={0.34} depthWrite={false} side={THREE.DoubleSide} />
          </mesh>
          <mesh position={sphere.position} raycast={raycastNone}>
            <sphereGeometry args={[sphere.radius, 32, 16]} />
            <meshBasicMaterial color={palette.committedWire} wireframe transparent opacity={0.55} depthWrite={false} />
          </mesh>
          <Line
            raycast={raycastNone}
            points={[
              [sphere.position[0], sphere.position[1], sphere.position[2]],
              [cursor[0], cursor[1], cursor[2]],
            ]}
            color={palette.dimension}
            lineWidth={1.5}
            dashed
            dashSize={0.08}
            gapSize={0.06}
          />
          <mesh position={sphere.position} raycast={raycastNone}>
            <sphereGeometry args={[0.04, 10, 10]} />
            <meshStandardMaterial color={palette.construction} emissive={palette.constructionEmissive} emissiveIntensity={0.35} />
          </mesh>
        </group>
      );
    }
  }
  // #region 🔵CircleArcPreview
  if ((previewKind === "circle-outline" || previewKind === "circle") && points.length >= 1 && cursor) {
    const center = points[0]!;
    const radius = Math.hypot(cursor[0] - center[0], cursor[1] - center[1], cursor[2] - center[2]);
    if (radius > 1e-9) {
      const segments = 64;
      const circlePts: [number, number, number][] = [];
      for (let i = 0; i <= segments; i++) {
        const a = (i / segments) * Math.PI * 2;
        circlePts.push([center[0] + Math.cos(a) * radius, center[1] + Math.sin(a) * radius, center[2]]);
      }
      return (
        <group>
          <Line raycast={raycastNone} points={circlePts} color={palette.accent} lineWidth={2} />
          <Line
            raycast={raycastNone}
            points={[
              [center[0], center[1], center[2]],
              [cursor[0], cursor[1], cursor[2]],
            ]}
            color={palette.dimension}
            lineWidth={1.5}
            dashed
            dashSize={0.08}
            gapSize={0.06}
          />
          <mesh position={center} raycast={raycastNone}>
            <sphereGeometry args={[0.04, 10, 10]} />
            <meshStandardMaterial color={palette.construction} emissive={palette.constructionEmissive} emissiveIntensity={0.35} />
          </mesh>
        </group>
      );
    }
  }
  if (previewKind === "arc" && points.length >= 2 && cursor) {
    const center = points[0]!;
    const start = points[1]!;
    const arcEnd = scenePreview().arcEndOnCircle(center, start, cursor);
    const arcPts = scenePreview().arcSamplePoints(center, start, arcEnd, 64);
    if (arcPts.length >= 2) {
      return (
        <group>
          <Line raycast={raycastNone} points={arcPts.map((pt) => [pt[0], pt[1], pt[2]])} color={palette.accent} lineWidth={2} />
          <Line
            raycast={raycastNone}
            points={[
              [center[0], center[1], center[2]],
              [start[0], start[1], start[2]],
            ]}
            color={palette.dimension}
            lineWidth={1.5}
            dashed
            dashSize={0.08}
            gapSize={0.06}
          />
          <Line
            raycast={raycastNone}
            points={[
              [center[0], center[1], center[2]],
              [arcEnd[0], arcEnd[1], arcEnd[2]],
            ]}
            color={palette.dimension}
            lineWidth={1.5}
            dashed
            dashSize={0.08}
            gapSize={0.06}
          />
          <mesh position={center} raycast={raycastNone}>
            <sphereGeometry args={[0.04, 10, 10]} />
            <meshStandardMaterial color={palette.construction} emissive={palette.constructionEmissive} emissiveIntensity={0.35} />
          </mesh>
          <mesh position={start} raycast={raycastNone}>
            <sphereGeometry args={[0.04, 10, 10]} />
            <meshStandardMaterial color={palette.construction} emissive={palette.constructionEmissive} emissiveIntensity={0.35} />
          </mesh>
          <mesh position={arcEnd} raycast={raycastNone}>
            <sphereGeometry args={[0.04, 10, 10]} />
            <meshStandardMaterial color={palette.accent} emissive={palette.accentEmissive} emissiveIntensity={0.35} />
          </mesh>
        </group>
      );
    }
  }
  // #endregion 🔵CircleArcPreview
  if (previewKind === "interpolated-curve" && linePoints.length >= 2) {
    const sampled = scenePreview().nurbsDisplaySamplePoints(linePoints, Math.max(12, linePoints.length * 8));
    const placedCount = cursor ? linePoints.length - 1 : linePoints.length;
    return (
      <group>
        <Line raycast={raycastNone} points={sampled.map((pt) => [pt[0], pt[1], pt[2]])} color={palette.accent} lineWidth={2} />
        {linePoints.slice(0, placedCount).map((pt, i) => (
          <mesh key={i} position={pt} raycast={raycastNone}>
            <sphereGeometry args={[0.04, 10, 10]} />
            <meshStandardMaterial color={palette.construction} emissive={palette.constructionEmissive} emissiveIntensity={0.35} />
          </mesh>
        ))}
      </group>
    );
  }
  return <group>{linePoints.length >= 2 ? <Line raycast={raycastNone} points={linePoints.map((pt) => [pt[0], pt[1], pt[2]])} color={palette.accent} lineWidth={2} /> : null}</group>;
}

function EntityHighlightItem({ item, geometry }: { readonly item: DisplayItem; readonly geometry?: SpatialPickGeometry | null }): ReactNode {
  const p = item.params;
  if (!p || !geometry) return null;
  const entity = p.entity;
  if (!entity || typeof entity !== "object") return null;
  const kind = (entity as { kind?: unknown }).kind;
  const id = (entity as { id?: unknown }).id;
  if (typeof kind !== "string" || typeof id !== "string") return null;
  return <GeometryTargetWireframes geometry={geometry} targets={[{ kind: kind as ModelEntityKind, id }]} transform={(pt) => pt} color={spatialSceneColors().construction} opacity={0.85} />;
}

function CurveItem({ item }: { readonly item: DisplayItem }): ReactNode {
  const points = readVec3Array(item.params?.points);
  if (points.length < 2) return null;
  return <Line raycast={raycastNone} points={points.map((pt) => [pt[0], pt[1], pt[2]])} color={palette.accent} lineWidth={2} />;
}

function isMeshTransferLike(v: unknown): v is MeshTransfer {
  if (!v || typeof v !== "object") return false;
  const m = v as MeshTransfer;
  return m.position instanceof Float32Array && m.index instanceof Uint32Array && Array.isArray(m.faceGroups);
}

function MeshItem({ item }: { readonly item: DisplayItem }): ReactNode {
  const raw = item.params?.mesh ?? item.params?.transfer;
  if (!isMeshTransferLike(raw)) return null;
  return <TessellatedCommitMesh mesh={raw} />;
}

function defaultDisplayItemNode(item: DisplayItem, geometry?: SpatialPickGeometry | null): ReactNode {
  switch (item.kind) {
    case "box-preview":
      return <BoxPreviewItem item={item} />;
    case "point":
      return <PointItem item={item} />;
    case "linear-handle":
      return <LinearHandleItem item={item} />;
    case "segment":
      return <SegmentItem item={item} />;
    case "label":
      return <LabelItem item={item} />;
    case "preview":
      return <PreviewItem item={item} geometry={geometry} />;
    case "entity-highlight":
      return <EntityHighlightItem item={item} geometry={geometry} />;
    case "curve":
      return <CurveItem item={item} />;
    case "mesh":
      return <MeshItem item={item} />;
    default:
      return null;
  }
}

// #region 🎨HostCustomization
/** @emoji 🖼️ Host hook that renders one resolved `DisplayItem` inside `<InteractionDisplay>`. */
export type SpatialDisplayItemRenderer = (item: DisplayItem, geometry: SpatialPickGeometry | null | undefined, defaultRender: () => ReactNode) => ReactNode;

const spatialDisplayItemRenderers = new Map<string, SpatialDisplayItemRenderer>();

/** @emoji 🖼️ Registers a custom display kind; returns unregister. Libraries extend without forking the package. */
export function registerSpatialDisplayItemKind(kind: string, render: SpatialDisplayItemRenderer): () => void {
  spatialDisplayItemRenderers.set(kind, render);
  return () => spatialDisplayItemRenderers.delete(kind);
}

/** @emoji 🖼️ Looks up a host-registered display kind renderer. */
export function getSpatialDisplayItemKindRenderer(kind: string): SpatialDisplayItemRenderer | undefined {
  return spatialDisplayItemRenderers.get(kind);
}

function renderDisplayItem(item: DisplayItem, geometry: SpatialPickGeometry | null | undefined, renderItem?: SpatialDisplayItemRenderer): ReactNode {
  const fallback = () => defaultDisplayItemNode(item, geometry);
  const custom = renderItem ?? spatialDisplayItemRenderers.get(item.kind);
  return custom ? custom(item, geometry, fallback) : fallback();
}

//#region 🔖TransformGumball
/** @emoji 📐 World-space gumball matrix snapshot (Three.js compose order). */
export interface GumballMatrixSnapshot {
  readonly position: Vec3;
  readonly quaternion: readonly [number, number, number, number];
  readonly scale: Vec3;
}

function gumballSnapshotFromObject3D(object: THREE.Object3D): GumballMatrixSnapshot {
  const position = object.position;
  const quaternion = object.quaternion;
  const scale = object.scale;
  return {
    position: [position.x, position.y, position.z],
    quaternion: [quaternion.x, quaternion.y, quaternion.z, quaternion.w],
    scale: [scale.x, scale.y, scale.z],
  };
}

/** @emoji 🎛 Applies a gumball world-matrix delta to all vertices reachable from `targets`. */
export function transformGumballMatrixDiff(model: Model, targets: readonly SelectionTarget[], before: GumballMatrixSnapshot, after: GumballMatrixSnapshot): ModelDiff {
  const vertexIds = collectTargetVertices(model, targets);
  if (vertexIds.size === 0) return EMPTY_MODEL_DIFF;
  const mBefore = new THREE.Matrix4().compose(
    new THREE.Vector3(before.position[0], before.position[1], before.position[2]),
    new THREE.Quaternion(before.quaternion[0], before.quaternion[1], before.quaternion[2], before.quaternion[3]),
    new THREE.Vector3(before.scale[0], before.scale[1], before.scale[2]),
  );
  const mAfter = new THREE.Matrix4().compose(
    new THREE.Vector3(after.position[0], after.position[1], after.position[2]),
    new THREE.Quaternion(after.quaternion[0], after.quaternion[1], after.quaternion[2], after.quaternion[3]),
    new THREE.Vector3(after.scale[0], after.scale[1], after.scale[2]),
  );
  const delta = mAfter.multiply(mBefore.clone().invert());
  const point = new THREE.Vector3();
  const modified: { id: VertexRef; position: Vec3 }[] = [];
  for (const vid of vertexIds) {
    const v = model.vertices[vid];
    if (!v) continue;
    point.set(v.position[0], v.position[1], v.position[2]);
    point.applyMatrix4(delta);
    const next: Vec3 = [point.x, point.y, point.z];
    if (next[0] === v.position[0] && next[1] === v.position[1] && next[2] === v.position[2]) continue;
    modified.push({ id: v.id, position: next });
  }
  return modified.length ? { vertices: { modified } } : EMPTY_MODEL_DIFF;
}

/** @emoji 🎛 R3F gumball for multi-target primitive transforms (pivot at selection bbox center). */
export function SpatialTransformGumball(props: {
  readonly mode: CadTransformGumballMode;
  readonly model: Model;
  readonly targets: readonly SelectionTarget[];
  readonly previewKernel?: SpatialPreviewKernel;
  readonly onCommit: (diff: ModelDiff) => void;
}): ReactNode {
  const previewKernel = props.previewKernel ?? r3fPreviewKernel;
  const pivot = reactHostPort.useMemo(() => selectionTargetsCenter(props.model, props.targets, previewKernel), [props.model, props.targets, previewKernel, props.model.revision]);
  const groupRef = reactHostPort.useRef<THREE.Group>(null);
  const [tcTarget, setTcTarget] = reactHostPort.useState<THREE.Object3D | null>(null);
  const beforeRef = reactHostPort.useRef<GumballMatrixSnapshot | null>(null);
  const scene = useThree((state) => state.scene);
  const controlsMode = cadTransformGumballModeToControlsMode(props.mode);
  const canTransform = selectionTargetsHaveTransformableVertices(props.model, props.targets);

  reactHostPort.useLayoutEffect(() => {
    const group = groupRef.current;
    if (!group || !pivot) return;
    group.position.set(pivot[0], pivot[1], pivot[2]);
    group.quaternion.set(0, 0, 0, 1);
    group.scale.set(1, 1, 1);
    group.updateMatrixWorld(true);
    setTcTarget(group);
  }, [pivot, props.mode, props.targets]);

  if (!pivot || !canTransform) return null;

  return (
    <>
      <group ref={groupRef}>
        <mesh visible={false}>
          <boxGeometry args={[0.001, 0.001, 0.001]} />
        </mesh>
      </group>
      {tcTarget
        ? createPortal(
            <TransformControls
              object={tcTarget}
              mode={controlsMode}
              onMouseDown={() => {
                if (groupRef.current) beforeRef.current = gumballSnapshotFromObject3D(groupRef.current);
              }}
              onMouseUp={() => {
                const before = beforeRef.current;
                const group = groupRef.current;
                beforeRef.current = null;
                if (!before || !group || !pivot) return;
                const after = gumballSnapshotFromObject3D(group);
                const diff = transformGumballMatrixDiff(props.model, props.targets, before, after);
                if (!isEmptyModelDiff(diff)) props.onCommit(diff);
                group.position.set(pivot[0], pivot[1], pivot[2]);
                group.quaternion.set(0, 0, 0, 1);
                group.scale.set(1, 1, 1);
                group.updateMatrixWorld(true);
              }}
            />,
            scene,
          )
        : null}
    </>
  );
}
//#endregion 🔖TransformGumball

/** @emoji 🪩 Optional scene slots for host overlays (gizmos, annotations, alternate lighting). */
export interface InteractionSpatialViewSlots {
  readonly beforeScene?: ReactNode;
  readonly afterDisplay?: ReactNode;
  readonly afterCommitted?: ReactNode;
  readonly lights?: ReactNode;
  readonly environment?: ReactNode;
}

/** @emoji 🎨 Theme tokens for default scene chrome (hosts override per product). */
export interface InteractionSpatialViewTheme {
  readonly background?: string;
  readonly ambientIntensity?: number;
  readonly directionalIntensity?: number;
  readonly directionalPosition?: Vec3;
  readonly gridDivisions?: number;
  readonly gridSize?: number;
  readonly groundPlaneColor?: string;
  readonly groundPlaneOpacity?: number;
}

export const defaultInteractionSpatialViewTheme: InteractionSpatialViewTheme = {
  ambientIntensity: 0.45,
  directionalIntensity: 1.1,
  directionalPosition: [12, 18, 10],
  gridDivisions: 40,
  gridSize: 40,
  groundPlaneOpacity: 0,
};
// #endregion 🎨HostCustomization

// #region 🎨SpatialSceneColors
/** @emoji 🎨 Resolved product palette for spatial canvas materials (no ad-hoc hex in hosts). */
export interface SpatialSceneColorPalette {
  readonly canvas: string;
  readonly accent: string;
  readonly accentEmissive: string;
  readonly accentSecondary: string;
  readonly accentSecondaryEmissive: string;
  readonly foreground: string;
  readonly muted: string;
  readonly mutedEmissive: string;
  readonly selected: string;
  readonly selectedEmissive: string;
  readonly hovered: string;
  readonly hoveredEmissive: string;
  readonly vertex: string;
  readonly vertexEmissive: string;
  readonly edge: string;
  readonly edgeEmissive: string;
  readonly object: string;
  readonly objectEmissive: string;
  readonly face: string;
  readonly faceEmissive: string;
  readonly gridMajor: string;
  readonly gridMinor: string;
  readonly groundPlane: string;
  readonly archived: string;
  readonly archivedEmissive: string;
  readonly ghost: string;
  readonly committed: string;
  readonly committedEmissive: string;
  readonly committedWire: string;
  readonly dimension: string;
  readonly guide: string;
  readonly construction: string;
  readonly constructionEmissive: string;
}

const SPATIAL_SCENE_COLOR_FALLBACK: SpatialSceneColorPalette = {
  canvas: "#e8e8e8",
  accent: "#c9a227",
  accentEmissive: "#3d3010",
  accentSecondary: "#8a8a8a",
  accentSecondaryEmissive: "#2a2a2a",
  foreground: "#1a1a1a",
  muted: "#6b6b6b",
  mutedEmissive: "#2a2a2a",
  selected: "#c9a227",
  selectedEmissive: "#3d3010",
  hovered: "#a08020",
  hoveredEmissive: "#302808",
  vertex: "#1a1a1a",
  vertexEmissive: "#404040",
  edge: "#4a4a4a",
  edgeEmissive: "#2a2a2a",
  object: "#6b6b6b",
  objectEmissive: "#2a2a2a",
  face: "#8a8a8a",
  faceEmissive: "#333333",
  gridMajor: "#b0b0b0",
  gridMinor: "#d8d8d8",
  groundPlane: "#b0b0b0",
  archived: "#6b8a72",
  archivedEmissive: "#1a2820",
  ghost: "#8a8a8a",
  committed: "#8a8a8a",
  committedEmissive: "#2a2a2a",
  committedWire: "#d0d0d0",
  dimension: "#1a1a1a",
  guide: "#6b6b6b",
  construction: "#c9a227",
  constructionEmissive: "#3d3010",
};

function readSpatialCssColor(variable: string, fallback: string): string {
  if (typeof document === "undefined") return fallback;
  const resolved = getComputedStyle(document.documentElement).getPropertyValue(variable).trim();
  return resolved || fallback;
}

let spatialSceneColorCache: SpatialSceneColorPalette | null = null;

/** @emoji 🎨 Reads `--canvas`, `--accent`, and selection tokens for Three.js materials. */
export function spatialSceneColors(): SpatialSceneColorPalette {
  if (spatialSceneColorCache) return spatialSceneColorCache;
  const accent = readSpatialCssColor("--accent", SPATIAL_SCENE_COLOR_FALLBACK.accent);
  const accentSecondary = readSpatialCssColor("--accent-secondary", SPATIAL_SCENE_COLOR_FALLBACK.accentSecondary);
  const foreground = readSpatialCssColor("--foreground", SPATIAL_SCENE_COLOR_FALLBACK.foreground);
  const muted = readSpatialCssColor("--muted-foreground", SPATIAL_SCENE_COLOR_FALLBACK.muted);
  const selected = readSpatialCssColor("--color-changed-selected", accent);
  const hovered = readSpatialCssColor("--color-changed-hovered", accentSecondary);
  spatialSceneColorCache = {
    canvas: readSpatialCssColor("--canvas", SPATIAL_SCENE_COLOR_FALLBACK.canvas),
    accent,
    accentEmissive: readSpatialCssColor("--active-base", SPATIAL_SCENE_COLOR_FALLBACK.accentEmissive),
    accentSecondary,
    accentSecondaryEmissive: readSpatialCssColor("--hover-panel", SPATIAL_SCENE_COLOR_FALLBACK.accentSecondaryEmissive),
    foreground,
    muted,
    mutedEmissive: readSpatialCssColor("--hover-base", SPATIAL_SCENE_COLOR_FALLBACK.mutedEmissive),
    selected,
    selectedEmissive: readSpatialCssColor("--active-base", SPATIAL_SCENE_COLOR_FALLBACK.selectedEmissive),
    hovered,
    hoveredEmissive: readSpatialCssColor("--hover-panel", SPATIAL_SCENE_COLOR_FALLBACK.hoveredEmissive),
    vertex: foreground,
    vertexEmissive: readSpatialCssColor("--hover-base", SPATIAL_SCENE_COLOR_FALLBACK.vertexEmissive),
    edge: readSpatialCssColor("--border-color", SPATIAL_SCENE_COLOR_FALLBACK.edge),
    edgeEmissive: readSpatialCssColor("--hover-base", SPATIAL_SCENE_COLOR_FALLBACK.edgeEmissive),
    object: muted,
    objectEmissive: readSpatialCssColor("--hover-panel", SPATIAL_SCENE_COLOR_FALLBACK.objectEmissive),
    face: accentSecondary,
    faceEmissive: readSpatialCssColor("--hover-window", SPATIAL_SCENE_COLOR_FALLBACK.faceEmissive),
    gridMajor: readSpatialCssColor("--border-color", SPATIAL_SCENE_COLOR_FALLBACK.gridMajor),
    gridMinor: readSpatialCssColor("--muted-foreground", SPATIAL_SCENE_COLOR_FALLBACK.gridMinor),
    groundPlane: readSpatialCssColor("--border-color", SPATIAL_SCENE_COLOR_FALLBACK.groundPlane),
    archived: readSpatialCssColor("--success-border", SPATIAL_SCENE_COLOR_FALLBACK.archived),
    archivedEmissive: readSpatialCssColor("--success-foreground", SPATIAL_SCENE_COLOR_FALLBACK.archivedEmissive),
    ghost: muted,
    committed: accentSecondary,
    committedEmissive: readSpatialCssColor("--hover-panel", SPATIAL_SCENE_COLOR_FALLBACK.committedEmissive),
    committedWire: readSpatialCssColor("--foreground", SPATIAL_SCENE_COLOR_FALLBACK.committedWire),
    dimension: foreground,
    guide: muted,
    construction: accent,
    constructionEmissive: readSpatialCssColor("--active-base", SPATIAL_SCENE_COLOR_FALLBACK.constructionEmissive),
  };
  return spatialSceneColorCache;
}

/** @emoji 🔄 Clears cached CSS palette (tests or theme switches). */
export function resetSpatialSceneColorCache(): void {
  spatialSceneColorCache = null;
}

function spatialSceneColorToHex(color: string): number {
  return new THREE.Color(color).getHex();
}

const cadChromePopoverClass = "border-border bg-popover text-popover-foreground shadow-md rounded-md border p-single";
const cadChromeMenuButtonClass =
  "flex w-full cursor-pointer items-center gap-single rounded-sm px-single py-half text-left text-xs text-foreground hover:bg-accent hover:text-accent-foreground";
const cadChromePanelAsideClass = "border-border bg-panel text-foreground flex shrink-0 flex-col gap-single overflow-auto border-l p-double relative z-[2]";
const cadChromeTagClass = "border-border inline-flex items-center gap-half rounded-full border px-half py-0.5 text-xs";
const cadChromeTagOnClass = "bg-accent text-accent-foreground";
const cadChromeTagOffClass = "bg-muted text-muted-foreground";
const cadFieldClass = "h-medium w-full";
// #endregion 🎨SpatialSceneColors

/** @emoji 🖼️ Maps `DisplayModel.items` to R3F nodes (must live under `<Canvas>`). */
export function InteractionDisplay({ model, geometry, renderItem }: { readonly model: DisplayModel; readonly geometry?: SpatialPickGeometry | null; readonly renderItem?: SpatialDisplayItemRenderer }): ReactNode {
  return (
    <group>
      {model.items.map((item) => (
        <group key={item.id}>{renderDisplayItem(item, geometry, renderItem)}</group>
      ))}
    </group>
  );
}
// #endregion 🖼️DisplayPrimitives

// #region 🖱️Interaction
function pointerModifiers(event: ThreeEvent<PointerEvent>) {
  return {
    alt: event.altKey,
    ctrl: event.ctrlKey,
    meta: event.metaKey,
    shift: event.shiftKey,
  };
}

/** @emoji 🖱️ Ground hit-test on the **XY** working plane at fixed world **Z** (= spatial footprint plane; factory height is world Z). */
export interface GroundPickPlaneProps {
  readonly planeZ?: number;
  readonly enabled?: boolean;
  readonly onPick?: (point: Vec3) => void;
  readonly onContextPick?: (point: Vec3) => void;
  readonly onPointerMove?: (point: Vec3) => void;
  readonly pointerMoveEnabled?: boolean;
  readonly planeColor?: string;
  readonly planeOpacity?: number;
}

export function GroundPickPlane({ planeZ = 0, enabled = true, onPick, onContextPick, onPointerMove, pointerMoveEnabled, planeColor, planeOpacity = 0.18 }: GroundPickPlaneProps): ReactNode {
  const resolvedPlaneColor = planeColor ?? spatialSceneColors().groundPlane;
  const moveOn = pointerMoveEnabled ?? Boolean(onPointerMove);
  const pickDownOn = enabled || moveOn;
  const onPointerDown = (e: ThreeEvent<PointerEvent>) => {
    if (!pickDownOn || !onPick) return;
    e.stopPropagation();
    const p = e.point;
    onPick([p.x, p.y, planeZ] as unknown as Vec3);
  };
  const onContextMenu = (e: ThreeEvent<MouseEvent>) => {
    if (!enabled || !onContextPick) return;
    e.stopPropagation();
    const p = e.point;
    onContextPick([p.x, p.y, planeZ] as unknown as Vec3);
  };
  const onPointerMoveH = (e: ThreeEvent<PointerEvent>) => {
    if (!moveOn || !onPointerMove) return;
    e.stopPropagation();
    const p = e.point;
    onPointerMove([p.x, p.y, planeZ] as unknown as Vec3);
  };
  return (
    <mesh position={[0, 0, planeZ]} onPointerDown={onPointerDown} onContextMenu={onContextMenu} onPointerMove={onPointerMoveH}>
      <planeGeometry args={[120, 120]} />
      <meshBasicMaterial transparent opacity={planeOpacity} color={resolvedPlaneColor} side={THREE.DoubleSide} />
    </mesh>
  );
}

function vec3FromSnapshotContext(ctx: Record<string, unknown>, key: string): Vec3 | null {
  return readVec3(ctx[key]);
}

/** @emoji 🖱️ YZ wall at the second corner so `pointer.move` changes world Z (factory height uses |Δz|). */
function HeightDragSurface({ origin, corner, enabled, onPointerMove }: { readonly origin: Vec3; readonly corner: Vec3; readonly enabled: boolean; readonly onPointerMove?: (point: Vec3) => void }): ReactNode {
  const z0 = origin[2];
  const zSpan = 10;
  const zMid = z0 + zSpan / 2;
  const ySpan = 6;
  const onMove = (e: ThreeEvent<PointerEvent>) => {
    if (!enabled || !onPointerMove) return;
    e.stopPropagation();
    const p = e.point;
    onPointerMove([p.x, p.y, p.z] as unknown as Vec3);
  };
  const xPlane = corner[0] + 0.06;
  return (
    <mesh position={[xPlane, corner[1], zMid]} rotation={[0, Math.PI / 2, 0]} onPointerMove={onMove} renderOrder={2}>
      <planeGeometry args={[zSpan, ySpan]} />
      <meshStandardMaterial
        transparent
        opacity={0.38}
        color={spatialSceneColors().accent}
        emissive={spatialSceneColors().accentEmissive}
        emissiveIntensity={0.25}
        roughness={0.88}
        metalness={0.08}
        depthWrite={false}
        side={THREE.DoubleSide}
      />
    </mesh>
  );
}

/** @emoji 🖱️ Z-aligned rod at `origin` so `pointer.move` drives peak height without XY drift. */
function VerticalZDragRod({ origin, enabled, onPointerMove }: { readonly origin: Vec3; readonly enabled: boolean; readonly onPointerMove?: (point: Vec3) => void }): ReactNode {
  const h = 22;
  const onMove = (e: ThreeEvent<PointerEvent>) => {
    if (!enabled || !onPointerMove) return;
    e.stopPropagation();
    const p = e.point;
    onPointerMove([p.x, p.y, p.z] as unknown as Vec3);
  };
  return (
    <mesh position={[origin[0], origin[1], origin[2] + h / 2]} rotation={[Math.PI / 2, 0, 0]} onPointerMove={onMove} renderOrder={3}>
      <cylinderGeometry args={[0.14, 0.14, h, 10]} />
      <meshStandardMaterial transparent opacity={0.14} color={spatialSceneColors().accentSecondary} depthWrite={false} side={THREE.DoubleSide} />
    </mesh>
  );
}

/** @emoji 🎮 Maps R3F pointer events to `InteractionEvent` envelopes (point + modifiers). */
export function createR3FInteractionAdapter() {
  const toPoint = (event: ThreeEvent<PointerEvent>): Vec3 => [event.point.x, event.point.y, event.point.z];
  return {
    pointerMove: (event: ThreeEvent<PointerEvent>): InteractionEvent => ({
      kind: "pointer.move",
      point: toPoint(event),
      modifiers: pointerModifiers(event),
    }),
    pointerDown: (event: ThreeEvent<PointerEvent>): InteractionEvent => ({
      kind: "pointer.down",
      point: toPoint(event),
      modifiers: pointerModifiers(event),
    }),
  };
}
// #endregion 🖱️Interaction

// #region 🧲GeometryInteraction
function targetBounds(points: readonly Vec3[]): { readonly center: Vec3; readonly size: Vec3 } | null {
  if (points.length === 0) return null;
  const min = points.reduce((acc, p) => [Math.min(acc[0], p[0]), Math.min(acc[1], p[1]), Math.min(acc[2], p[2])] as unknown as Vec3, points[0]!);
  const max = points.reduce((acc, p) => [Math.max(acc[0], p[0]), Math.max(acc[1], p[1]), Math.max(acc[2], p[2])] as unknown as Vec3, points[0]!);
  return {
    center: [(min[0] + max[0]) / 2, (min[1] + max[1]) / 2, (min[2] + max[2]) / 2] as unknown as Vec3,
    size: [Math.max(max[0] - min[0], 0.08), Math.max(max[1] - min[1], 0.08), Math.max(max[2] - min[2], 0.08)] as unknown as Vec3,
  };
}

const spatialPickPriority: Record<SpatialPickTargetKind, number> = {
  vertex: 0,
  edge: 1,
  face: 2,
  object: 3,
  surface: 4,
  part: 5,
};

function targetRayScore(ray: THREE.Ray, target: SpatialPickTarget): number | null {
  const points = target.points?.length ? target.points : [target.point];
  const box = new THREE.Box3();
  for (const point of points) box.expandByPoint(new THREE.Vector3(point[0], point[1], point[2]));
  box.expandByScalar(target.kind === "vertex" ? 0.12 : 0.08);
  const hit = ray.intersectBox(box, new THREE.Vector3());
  if (!hit) return null;
  return ray.origin.distanceTo(hit) + spatialPickPriority[target.kind] * 1e-4;
}

function pointerModifiersFromNativeEvent(event: PointerEvent): InteractionEvent["modifiers"] {
  return {
    alt: event.altKey,
    ctrl: event.ctrlKey,
    meta: event.metaKey,
    shift: event.shiftKey,
  };
}

function spatialSelectionModeFromModifiers(modifiers: { readonly alt?: boolean; readonly ctrl?: boolean; readonly meta?: boolean; readonly shift?: boolean } = {}): SpatialSelectionMode {
  if (modifiers.shift && modifiers.ctrl) return "invertive";
  if (modifiers.shift) return "additive";
  if (modifiers.ctrl) return "subtractive";
  return "default";
}

function uniqueSelectionTargets(targets: readonly SelectionTarget[]): SelectionTarget[] {
  const out: SelectionTarget[] = [];
  const seen = new Set<string>();
  for (const target of targets) {
    const key = spatialSelectionTargetKey(target);
    if (seen.has(key)) continue;
    seen.add(key);
    out.push(target);
  }
  return out;
}

function mergeSelectionTargets(current: readonly SelectionTarget[], next: readonly SelectionTarget[], mode: SpatialSelectionMode): SelectionTarget[] {
  const uniqueNext = uniqueSelectionTargets(next);
  const nextKeys = new Set(uniqueNext.map(spatialSelectionTargetKey));
  if (mode === "default") return uniqueNext;
  if (mode === "additive") {
    const seen = new Set(current.map(spatialSelectionTargetKey));
    const merged = [...current];
    for (const target of uniqueNext) {
      const key = spatialSelectionTargetKey(target);
      if (seen.has(key)) continue;
      seen.add(key);
      merged.push(target);
    }
    return merged;
  }
  if (mode === "subtractive") return current.filter((target) => !nextKeys.has(spatialSelectionTargetKey(target)));
  const currentKeys = new Set(current.map(spatialSelectionTargetKey));
  return [...current.filter((target) => !nextKeys.has(spatialSelectionTargetKey(target))), ...uniqueNext.filter((target) => !currentKeys.has(spatialSelectionTargetKey(target)))];
}

function dragDistance(a: { readonly x: number; readonly y: number }, b: { readonly x: number; readonly y: number }): number {
  return Math.hypot(b.x - a.x, b.y - a.y);
}

function spatialSelectionCoverageFromPath(path: readonly { readonly x: number; readonly y: number }[]): SpatialSelectionCoverage {
  const start = path[0];
  if (!start) return "full";
  for (const point of path.slice(1)) {
    const dx = point.x - start.x;
    if (Math.abs(dx) < 2) continue;
    return dx < 0 ? "partial" : "full";
  }
  const end = path[path.length - 1] ?? start;
  return end.x < start.x ? "partial" : "full";
}

function pointInRectangle(point: { readonly x: number; readonly y: number }, rect: { readonly left: number; readonly right: number; readonly top: number; readonly bottom: number }): boolean {
  return point.x >= rect.left && point.x <= rect.right && point.y >= rect.top && point.y <= rect.bottom;
}

function pointInPolygon(point: { readonly x: number; readonly y: number }, polygon: readonly { readonly x: number; readonly y: number }[]): boolean {
  let inside = false;
  for (let i = 0, j = polygon.length - 1; i < polygon.length; j = i++) {
    const a = polygon[i]!;
    const b = polygon[j]!;
    const intersects = a.y > point.y !== b.y > point.y && point.x < ((b.x - a.x) * (point.y - a.y)) / (b.y - a.y || 1e-9) + a.x;
    if (intersects) inside = !inside;
  }
  return inside;
}

function projectPointToClient(point: Vec3, camera: THREE.Camera, rect: DOMRect): { readonly x: number; readonly y: number } | null {
  const projected = new THREE.Vector3(point[0], point[1], point[2]).project(camera);
  if (!Number.isFinite(projected.x) || !Number.isFinite(projected.y) || !Number.isFinite(projected.z)) return null;
  if (projected.z < -1 || projected.z > 1) return null;
  return {
    x: rect.left + ((projected.x + 1) / 2) * rect.width,
    y: rect.top + ((1 - projected.y) / 2) * rect.height,
  };
}

function spatialPickTargetsFromClientPoint(
  client: { readonly x: number; readonly y: number },
  camera: THREE.Camera,
  rect: DOMRect,
  targets: readonly SpatialPickTarget[],
  selectionAccept: readonly ModelEntityKind[],
  kindToggles: SpatialPickKindToggles,
): SpatialPickTarget[] {
  const pointer = new THREE.Vector2(((client.x - rect.left) / rect.width) * 2 - 1, -(((client.y - rect.top) / rect.height) * 2 - 1));
  const raycaster = new THREE.Raycaster();
  raycaster.setFromCamera(pointer, camera);
  return spatialPickTargetsFromRay(raycaster.ray, targets, selectionAccept, kindToggles);
}

function spatialPickTargetsFromScreenSelection(
  drag: SpatialDragSelectionState,
  targets: readonly SpatialPickTarget[],
  camera: THREE.Camera,
  rect: DOMRect,
  selectionAccept: readonly ModelEntityKind[],
  kindToggles: SpatialPickKindToggles,
  geometryPreviewTransform?: ((point: Vec3) => Vec3) | null,
): SpatialPickTarget[] {
  const selectable = filterSpatialPickTargets(targets, selectionAccept, kindToggles);
  const mapPoint = geometryPreviewTransform ?? ((point: Vec3) => point);
  const rectBounds = {
    left: Math.min(drag.startClient.x, drag.currentClient.x),
    right: Math.max(drag.startClient.x, drag.currentClient.x),
    top: Math.min(drag.startClient.y, drag.currentClient.y),
    bottom: Math.max(drag.startClient.y, drag.currentClient.y),
  };
  const contains = drag.method === "rectangle" ? (point: { readonly x: number; readonly y: number }) => pointInRectangle(point, rectBounds) : (point: { readonly x: number; readonly y: number }) => pointInPolygon(point, drag.path);
  return selectable.filter((target) => {
    const points = (target.points?.length ? target.points : [target.point]).map(mapPoint);
    const projected = points.map((point) => projectPointToClient(point, camera, rect)).filter((point): point is { readonly x: number; readonly y: number } => point !== null);
    if (projected.length === 0) return false;
    return drag.coverage === "partial" ? projected.some(contains) : projected.every(contains);
  });
}

function spatialPickTargetsFromRay(ray: THREE.Ray, targets: readonly SpatialPickTarget[], selectionAccept: readonly ModelEntityKind[], kindToggles: SpatialPickKindToggles): SpatialPickTarget[] {
  return filterSpatialPickTargets(targets, selectionAccept, kindToggles)
    .map((target) => ({ target, score: targetRayScore(ray, target) }))
    .filter((hit): hit is { readonly target: SpatialPickTarget; readonly score: number } => hit.score !== null)
    .sort((a, b) => a.score - b.score)
    .map((hit) => hit.target);
}

function targetStyle(target: SpatialPickTarget, hovered: boolean, selected: boolean): { color: string; emissive: string; opacity: number; lineWidth: number } {
  const palette = spatialSceneColors();
  if (selected) return { color: palette.selected, emissive: palette.selectedEmissive, opacity: target.kind === "vertex" ? 1 : 0.34, lineWidth: 9 };
  if (hovered) return { color: palette.hovered, emissive: palette.hoveredEmissive, opacity: target.kind === "vertex" ? 1 : 0.28, lineWidth: 8 };
  if (target.kind === "vertex") return { color: palette.vertex, emissive: palette.vertexEmissive, opacity: 1, lineWidth: 5 };
  if (target.kind === "edge") return { color: palette.edge, emissive: palette.edgeEmissive, opacity: 0.8, lineWidth: 5 };
  if (target.kind === "object" && !target.geometryKind) return { color: palette.object, emissive: palette.objectEmissive, opacity: 0.28, lineWidth: 7 };
  return { color: palette.face, emissive: palette.faceEmissive, opacity: 0.16, lineWidth: 5 };
}

function selectionTargetPickKind(target: SelectionTarget): SpatialPickTargetKind | null {
  if (target.kind === "object") return "object";
  return GEOMETRY_KIND_TO_OBJECT_PICK[target.kind] ?? null;
}

function pinnedPickTargetKeys(keys: ReadonlySet<string>): ReadonlySet<string> {
  const out = new Set<string>();
  for (const key of keys) {
    out.add(key);
    const colon = key.indexOf(":");
    if (colon < 0) continue;
    const kind = key.slice(0, colon);
    const id = key.slice(colon + 1);
    const mapped = GEOMETRY_KIND_TO_OBJECT_PICK[kind as ModelEntityKind];
    if (mapped) out.add(`${mapped}:${id}`);
  }
  return out;
}

function spatialSelectionTarget(target: SpatialPickTarget): SelectionTarget {
  if (target.kind === "object" && !target.geometryKind) {
    return { kind: "object", id: target.id, editable: false };
  }
  const geometryKind = kernelGeometryKindForObjectPick(target.kind as SpatialGeometryPickTargetKind, target.geometryKind);
  return { kind: geometryKind, id: target.id, editable: true };
}

/** @emoji 🎯 Host geometry picking when browse is idle, session finished, or interaction defers picks (`pickDisabledStates`). */
export function replHostGeometryPickingEnabled(interactionId: string, spec: InteractionSpec, state: string): boolean {
  if (!interactionId) return true;
  if (!isInteractionSessionActive(spec, state)) return true;
  return mergeInteractionSpatial(spec).pickDisabledStates.includes(state);
}

/** @emoji 👁️ Pick-target overlay visible whenever the active model definition uses factory geometry picking. */
export function replGeometryPickLayerVisible(modelDefinitionId: string | null): boolean {
  return modelDefinitionUsesGeometryPicking(modelDefinitionId ?? defaultModelDefinitionId());
}

/** @emoji 🖱️ Returns the closest pick target eligible for hover highlighting along a ray. */
export function pickHoverTargetFromRay(ray: THREE.Ray, targets: readonly SpatialPickTarget[], hoverKindToggles: SpatialPickKindToggles = {}): SpatialPickTarget | null {
  return spatialPickTargetsFromRay(ray, targets, [], hoverKindToggles)[0] ?? null;
}

/** @emoji 📌 Renders visibility-enabled pick highlights plus pinned hover/selection targets. */
export function resolveSpatialPickTargetsToRender(viewTargets: readonly SpatialPickTarget[], filterKindToggles: SpatialPickKindToggles = {}, pinnedTargetKeys: ReadonlySet<string> = new Set()): SpatialPickTarget[] {
  const pinnedKeys = pinnedPickTargetKeys(pinnedTargetKeys);
  const enabledTargets = filterSpatialPickTargetsForVisibility(viewTargets, filterKindToggles);
  const seen = new Set<string>();
  const out: SpatialPickTarget[] = [];
  for (const target of enabledTargets) {
    const key = spatialPickTargetKey(target);
    if (seen.has(key)) continue;
    out.push(target);
    seen.add(key);
  }
  for (const target of viewTargets) {
    const key = spatialPickTargetKey(target);
    if (!pinnedKeys.has(key) || seen.has(key)) continue;
    out.push(target);
    seen.add(key);
  }
  return out;
}

/** @emoji 👁️ Visual-only pick-target highlight; hit-testing is handled by `SpatialPickRayCatcher`. */
function SpatialPickTargetNode({
  target,
  geometry = null,
  geometryPreviewTransform = null,
  hoveredTargetKey,
  selectedTargetKey,
  selectedTargetKeys,
}: {
  readonly target: SpatialPickTarget;
  readonly geometry?: SpatialPickGeometry | null;
  readonly geometryPreviewTransform?: ((point: Vec3) => Vec3) | null;
  readonly hoveredTargetKey?: string | null;
  readonly selectedTargetKey?: string | null;
  readonly selectedTargetKeys?: ReadonlySet<string> | null;
}): ReactNode {
  const mapPt = geometryPreviewTransform ?? ((p: Vec3) => p);
  const displayPoint = mapPt(target.point);
  const displayPoints = target.points?.map(mapPt);
  const targetKey = spatialPickTargetKey(target);
  const hovered = hoveredTargetKey === targetKey;
  const selected = selectedTargetKeys?.has(targetKey) ?? selectedTargetKey === targetKey;
  const style = targetStyle(target, hovered, selected);
  const userData = { spatialPickKey: targetKey };
  if (target.kind === "vertex") {
    return (
      <mesh position={displayPoint} userData={userData} raycast={raycastNone} renderOrder={8}>
        <sphereGeometry args={[selected || hovered ? 0.12 : 0.085, 16, 16]} />
        <meshStandardMaterial color={style.color} emissive={style.emissive} emissiveIntensity={0.45} depthTest={false} transparent />
      </mesh>
    );
  }
  const wireKind = target.geometryKind ?? (target.kind === "edge" ? "edge" : null);
  const wireSegments = reactHostPort.useMemo(() => {
    if (!geometry || !wireKind) return [] as readonly (readonly [Vec3, Vec3])[];
    return geometryEntityWireSegments(geometryBuckets(geometry), wireKind, target.id);
  }, [geometry, wireKind, target.id]);
  const throughPoints = reactHostPort.useMemo(() => {
    if (!geometry || !wireKind) return [] as readonly Vec3[];
    return geometryEntityThroughPoints(geometryBuckets(geometry), wireKind, target.id);
  }, [geometry, wireKind, target.id]);
  if (wireSegments.length > 0) {
    const palette = spatialSceneColors();
    return (
      <group userData={userData}>
        {wireSegments.map(([a, b], i) => (
          <Line
            key={`${i}-${a.join(",")}-${b.join(",")}`}
            raycast={raycastNone}
            points={[
              [mapPt(a)[0], mapPt(a)[1], mapPt(a)[2]],
              [mapPt(b)[0], mapPt(b)[1], mapPt(b)[2]],
            ]}
            color={style.color}
            lineWidth={style.lineWidth}
            transparent
            opacity={style.opacity}
          />
        ))}
        {(selected || hovered) && throughPoints.length > 0
          ? throughPoints.map((pt, i) => {
              const p = mapPt(pt);
              return (
                <mesh key={`through-${i}`} position={[p[0], p[1], p[2]]} raycast={raycastNone} renderOrder={9}>
                  <sphereGeometry args={[0.04, 10, 10]} />
                  <meshStandardMaterial
                    color={palette.construction}
                    emissive={palette.constructionEmissive}
                    emissiveIntensity={0.35}
                    depthTest={false}
                  />
                </mesh>
              );
            })
          : null}
      </group>
    );
  }
  if (displayPoints && displayPoints.length >= 2 && target.kind === "edge") {
    return <Line userData={userData} raycast={raycastNone} points={displayPoints.map((p) => [p[0], p[1], p[2]])} color={style.color} lineWidth={style.lineWidth} />;
  }
  if (!hovered && !selected) return null;
  return (
    <mesh position={displayPoint} userData={userData} raycast={raycastNone} renderOrder={8}>
      <sphereGeometry args={[0.07, 12, 12]} />
      <meshStandardMaterial color={style.color} emissive={style.emissive} emissiveIntensity={0.35} depthTest={false} transparent opacity={style.opacity} />
    </mesh>
  );
}

/** @emoji 🧵 Draws all geometry edges for imported factory geometry (one batched `lineSegments`). */
function GeometryFactoryWireframeLayer({ geometry, visible = true }: { readonly geometry?: SpatialPickGeometry | null; readonly visible?: boolean }): ReactNode {
  const segments = reactHostPort.useMemo(() => {
    if (!geometry) return [] as readonly (readonly [Vec3, Vec3])[];
    return collectGeometryEdgeSegments(geometryBuckets(geometry));
  }, [geometry]);
  const edgeGeometry = reactHostPort.useMemo(() => {
    if (!segments.length) return null;
    const pos = new Float32Array(segments.length * 6);
    for (let i = 0; i < segments.length; i++) {
      const [a, b] = segments[i]!;
      const o = i * 6;
      pos[o] = a[0];
      pos[o + 1] = a[1];
      pos[o + 2] = a[2];
      pos[o + 3] = b[0];
      pos[o + 4] = b[1];
      pos[o + 5] = b[2];
    }
    const geo = new THREE.BufferGeometry();
    geo.setAttribute("position", new THREE.BufferAttribute(pos, 3));
    return geo;
  }, [segments]);
  reactHostPort.useEffect(() => () => edgeGeometry?.dispose(), [edgeGeometry]);
  if (!visible || !edgeGeometry) return null;
  return (
    <lineSegments geometry={edgeGeometry} raycast={raycastNone} renderOrder={0}>
      <lineBasicMaterial color={spatialSceneColors().committedWire} transparent opacity={0.72} depthTest />
    </lineSegments>
  );
}

/** @emoji 🧲 Renders optional factory geometry as pickable snap/select targets. */
//#region 🧲SpatialPickGeometryLayer
export function SpatialPickGeometryLayer({
  geometry,
  activeModelDefinitionId = defaultModelDefinitionId(),
  modelDefinitionRevision = 0,
  geometryPreviewTransform = null,
  selectionAccept = [],
  selectionKindToggles = {},
  filterKindToggles = {},
  hoveredTargetKey,
  selectedTargetKey,
  selectedTargetKeys,
  hostSelectionEnabled = false,
  onSelectionRequest,
}: {
  readonly geometry?: SpatialPickGeometry | null;
  readonly activeModelDefinitionId?: string | null;
  readonly modelDefinitionRevision?: number;
  readonly geometryPreviewTransform?: ((point: Vec3) => Vec3) | null;
  readonly selectionAccept?: readonly ModelEntityKind[];
  readonly selectionKindToggles?: SpatialPickKindToggles;
  /** @emoji 👁️ Which kinds are drawn as pick-target highlights (independent of selection). */
  readonly filterKindToggles?: SpatialPickKindToggles;
  readonly hoveredTargetKey?: string | null;
  readonly selectedTargetKey?: string | null;
  readonly selectedTargetKeys?: ReadonlySet<string> | null;
  readonly hostSelectionEnabled?: boolean;
  readonly onSelectionRequest?: (request: SpatialSelectionRequest) => void;
}): ReactNode {
  const modelRevision = geometry && typeof geometry === "object" && "revision" in geometry ? Number((geometry as { revision?: unknown }).revision) : 0;
  const targets = reactHostPort.useMemo(() => createSpatialPickTargets(geometry, activeModelDefinitionId), [geometry, modelRevision, modelDefinitionRevision, activeModelDefinitionId]);
  const viewTargets = reactHostPort.useMemo(() => filterSpatialPickTargetsForActiveView(targets, activeModelDefinitionId ?? null), [targets, activeModelDefinitionId]);
  const pinnedTargetKeys = reactHostPort.useMemo(() => {
    const keys = new Set<string>();
    if (hoveredTargetKey) keys.add(hoveredTargetKey);
    if (selectedTargetKey) keys.add(selectedTargetKey);
    selectedTargetKeys?.forEach((key) => keys.add(key));
    return keys;
  }, [hoveredTargetKey, selectedTargetKey, selectedTargetKeys]);
  const renderedTargets = reactHostPort.useMemo(() => {
    return resolveSpatialPickTargetsToRender(viewTargets, filterKindToggles, pinnedTargetKeys);
  }, [viewTargets, filterKindToggles, pinnedTargetKeys]);
  const selectableTargets = reactHostPort.useMemo(() => filterSpatialPickTargets(viewTargets, selectionAccept, selectionKindToggles), [viewTargets, selectionAccept, selectionKindToggles]);
  const requestSelection = reactHostPort.useCallback(
    (target: SpatialPickTarget, event: ThreeEvent<PointerEvent>) => {
      if (!hostSelectionEnabled || !onSelectionRequest || selectionAccept.length === 0) return;
      event.stopPropagation();
      onSelectionRequest({
        targets: [target],
        point: target.point,
        client: { x: event.nativeEvent.clientX, y: event.nativeEvent.clientY },
        modifiers: pointerModifiersFromNativeEvent(event.nativeEvent),
      });
    },
    [hostSelectionEnabled, onSelectionRequest, selectionAccept.length],
  );
  return (
    <group>
      {renderedTargets.map((target) => (
        <SpatialPickTargetNode
          key={`${target.kind}:${target.id}`}
          target={target}
          geometry={geometry}
          geometryPreviewTransform={geometryPreviewTransform}
          hoveredTargetKey={hoveredTargetKey}
          selectedTargetKey={selectedTargetKey}
          selectedTargetKeys={selectedTargetKeys}
        />
      ))}
      {hostSelectionEnabled && onSelectionRequest
        ? selectableTargets.map((target) => <SpatialPickHitTarget key={`hit:${target.kind}:${target.id}`} target={target} geometryPreviewTransform={geometryPreviewTransform} onPick={requestSelection} />)
        : null}
    </group>
  );
}

/** @emoji 🖱️ Invisible pick proxy for a spatial target (visual highlight is on {@link SpatialPickTargetNode}). */
function SpatialPickHitTarget({
  target,
  geometryPreviewTransform = null,
  onPick,
}: {
  readonly target: SpatialPickTarget;
  readonly geometryPreviewTransform?: ((point: Vec3) => Vec3) | null;
  readonly onPick: (target: SpatialPickTarget, event: ThreeEvent<PointerEvent>) => void;
}): ReactNode {
  const mapPt = geometryPreviewTransform ?? ((p: Vec3) => p);
  const displayPoint = mapPt(target.point);
  const displayPoints = target.points?.map(mapPt);
  if (target.kind === "vertex") {
    return (
      <mesh
        position={displayPoint}
        visible={false}
        onPointerDown={(event) => {
          if (event.button !== 0) return;
          onPick(target, event);
        }}
      >
        <sphereGeometry args={[0.14, 8, 8]} />
        <meshBasicMaterial transparent opacity={0} depthWrite={false} />
      </mesh>
    );
  }
  if (displayPoints && displayPoints.length >= 2 && target.kind === "edge") {
    return (
      <Line
        visible={false}
        points={displayPoints.map((p) => [p[0], p[1], p[2]])}
        lineWidth={12}
        onPointerDown={(event) => {
          if (event.button !== 0) return;
          onPick(target, event);
        }}
      >
        <meshBasicMaterial transparent opacity={0} />
      </Line>
    );
  }
  const bounds = displayPoints ? targetBounds(displayPoints) : null;
  if (!bounds) return null;
  return (
    <mesh
      position={bounds.center}
      scale={bounds.size}
      visible={false}
      onPointerDown={(event) => {
        if (event.button !== 0) return;
        onPick(target, event);
      }}
    >
      <boxGeometry args={[1, 1, 1]} />
      <meshBasicMaterial transparent opacity={0} depthWrite={false} />
    </mesh>
  );
}
// #endregion 🧲GeometryInteraction

// #region 🧊CommittedMesh
/** @emoji 🧊 Builds a Three.js `BufferGeometry` from a kernel `MeshTransfer` (face groups preserved). */
export function buildBufferGeometryFromMeshTransfer(data: MeshTransfer): THREE.BufferGeometry {
  const geo = new THREE.BufferGeometry();
  if (!isRenderableMeshTransfer(data)) return geo;
  geo.setAttribute("position", new THREE.Float32BufferAttribute(data.position, 3));
  geo.setAttribute("normal", new THREE.Float32BufferAttribute(data.normal, 3));
  geo.setIndex(new THREE.BufferAttribute(data.index, 1));
  for (const g of data.faceGroups) geo.addGroup(g.start, g.count, 0);
  return geo;
}

/** @emoji 🎯 Maps a picked triangle index to B-Rep `FaceInfo` via grouped buffer ranges. */
export function resolveFaceInfoFromTriangleIndex(mesh: MeshTransfer, triangleIndex: number | null | undefined): FaceInfo | null {
  if (triangleIndex === null || triangleIndex === undefined) return null;
  const group = findFaceGroupAt(mesh.faceGroups, triangleIndex);
  if (!group) return null;
  return mesh.faceInfos.find((info) => info.entityId === group.entityId) ?? null;
}

/** @emoji ➖ B-Rep edge overlay from `MeshTransfer.edges` (kernel `meshEdges`, not triangle edges). */
function CommittedEdgeOverlay({ data, visible = true }: { readonly data: MeshTransfer; readonly visible?: boolean }): ReactNode {
  const geometry = reactHostPort.useMemo(() => {
    const geo = new THREE.BufferGeometry();
    geo.setAttribute("position", new THREE.BufferAttribute(data.edges, 3));
    return geo;
  }, [data.edges]);
  reactHostPort.useEffect(() => () => geometry.dispose(), [geometry]);
  if (!visible) return null;
  return (
    <lineSegments geometry={geometry} raycast={raycastNone}>
      <lineBasicMaterial color={spatialSceneColors().foreground} depthTest />
    </lineSegments>
  );
}

export interface TessellatedCommitMeshProps {
  readonly mesh: MeshTransfer;
  readonly pickable?: boolean;
  readonly showFaces?: boolean;
  readonly showEdges?: boolean;
  readonly onFacePointerMove?: (info: FaceInfo, event: ThreeEvent<PointerEvent>) => void;
  readonly onFacePointerDown?: (info: FaceInfo, event: ThreeEvent<PointerEvent>) => void;
}

export const COMMITTED_MESH_FACE_OPACITY = 0.72;

/** @emoji 🧊 Shaded B-Rep mesh + edge overlay; optional face picking via `faceIndex`. */
export function TessellatedCommitMesh({ mesh: data, pickable = false, showFaces = true, showEdges = true, onFacePointerMove, onFacePointerDown }: TessellatedCommitMeshProps): ReactNode {
  const geometry = reactHostPort.useMemo(() => buildBufferGeometryFromMeshTransfer(data), [data.position, data.normal, data.index, data.faceGroups]);
  reactHostPort.useEffect(() => () => geometry.dispose(), [geometry]);
  if (!showFaces && !showEdges) return null;
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
        >
          <meshStandardMaterial
            color={data.color ?? spatialSceneColors().committed}
            metalness={0}
            roughness={0.45}
            emissive={data.color ?? spatialSceneColors().committedEmissive}
            emissiveIntensity={0.08}
            side={THREE.DoubleSide}
            polygonOffset
            polygonOffsetFactor={1}
            polygonOffsetUnits={1}
            transparent
            opacity={COMMITTED_MESH_FACE_OPACITY}
            depthWrite={false}
          />
        </mesh>
      ) : null}
      {data.edges.length > 0 ? <CommittedEdgeOverlay data={data} visible={showEdges} /> : null}
    </group>
  );
}

/** @emoji 🧊 Renders all committed document solids tessellated by the active kernel. */
export function CommittedMeshLayer({
  meshes,
  modelRevision,
  pickable = false,
  showFaces = true,
  showEdges = true,
  onFacePointerMove,
  onFacePointerDown,
}: {
  readonly meshes: readonly { readonly solid: SolidRef; readonly mesh: MeshTransfer }[];
  readonly modelRevision?: number;
  readonly pickable?: boolean;
  readonly showFaces?: boolean;
  readonly showEdges?: boolean;
  readonly onFacePointerMove?: (info: FaceInfo, event: ThreeEvent<PointerEvent>) => void;
  readonly onFacePointerDown?: (info: FaceInfo, event: ThreeEvent<PointerEvent>) => void;
}): ReactNode {
  if (meshes.length === 0 || (!showFaces && !showEdges)) return null;
  const rev = modelRevision ?? 0;
  return (
    <group>
      {meshes.map((row, i) => (
        <TessellatedCommitMesh key={`${row.solid}:r${rev}:${meshTransferContentKey(row.mesh, i)}`} mesh={row.mesh} pickable={pickable} showFaces={showFaces} showEdges={showEdges} onFacePointerMove={onFacePointerMove} onFacePointerDown={onFacePointerDown} />
      ))}
    </group>
  );
}
// #endregion 🧊CommittedMesh

// #region 🪝Hooks
/** @emoji 🪝 Memoized `createInteractionRuntime` for React hosts. */
export function useInteractionRuntime(spec: InteractionSpec, opts: InteractionRuntimeOptions): InteractionRuntime {
  const specId = spec.id;
  return reactHostPort.useMemo(() => createInteractionRuntime(spec, opts), [spec, specId, opts]);
}

/** @emoji 🪝 Subscribes to `InteractionRuntime` revision updates for React hosts. */
export function useInteractionSnapshot(rt: InteractionRuntime): InteractionSnapshot {
  return reactHostPort.useSyncExternalStore(
    (cb) => rt.subscribe(cb),
    () => rt.getSnapshot(),
    () => rt.getSnapshot(),
  );
}

/** @emoji 🎛️ Resolves functional or literal host-state updates (testable without React). */
export function resolveHostStateNext<T>(value: T, next: T | ((prev: T) => T)): T {
  return typeof next === "function" ? (next as (prev: T) => T)(value) : next;
}

/** @emoji 🎛️ Controlled-or-uncontrolled state slice for embeddable spatial hosts. */
export function useHostState<T>(controlled: T | undefined, onChange: ((value: T) => void) | undefined, initial: T | (() => T)): readonly [T, (next: T | ((prev: T) => T)) => void] {
  const [internal, setInternal] = reactHostPort.useState(initial);
  const isControlled = controlled !== undefined;
  const value = isControlled ? controlled : internal;
  const setValue = reactHostPort.useCallback(
    (next: T | ((prev: T) => T)) => {
      if (isControlled) {
        const resolved = resolveHostStateNext(controlled as T, next);
        onChange?.(resolved);
        return;
      }
      setInternal((prev) => {
        const resolved = resolveHostStateNext(prev, next);
        onChange?.(resolved);
        return resolved;
      });
    },
    [controlled, isControlled, onChange],
  );
  return [value, setValue] as const;
}
// #endregion 🪝Hooks

// #region 🪩Canvas
export interface InteractionCanvasProps {
  readonly children: ReactNode;
  readonly onCanvasReady?: (binding: { readonly camera: THREE.Camera; readonly domElement: HTMLCanvasElement }) => void;
  /** @emoji 🎞️ `always` while an interaction session runs; `demand` when idle for GPU savings. */
  readonly frameloop?: "always" | "demand";
  readonly background?: string;
  readonly cameraPosition?: Vec3;
  readonly cameraFov?: number;
  readonly cameraNear?: number;
  readonly cameraFar?: number;
  readonly dpr?: number | [number, number];
  readonly shadows?: boolean | "basic" | "percentage" | "soft" | "variance";
  readonly style?: CSSProperties;
  readonly className?: string;
  readonly gl?: React.ComponentProps<typeof Canvas>["gl"];
  readonly onPointerDown?: (event: PointerEvent) => void;
  readonly onPointerMove?: (event: PointerEvent) => void;
  readonly onPointerUp?: (event: PointerEvent) => void;
  readonly onPointerLeave?: (event: PointerEvent) => void;
  readonly onPointerCancel?: (event: PointerEvent) => void;
  readonly onWheel?: (event: WheelEvent) => void;
  readonly onContextMenu?: (event: MouseEvent) => void;
  readonly onDoubleClick?: (event: MouseEvent) => void;
  readonly onLostPointerCapture?: (event: PointerEvent) => void;
}

/** @emoji 📡 Host event callbacks accepted by {@link InteractionCanvas}. */
export type InteractionCanvasHostCallbacks = Pick<
  InteractionCanvasProps,
  "onCanvasReady" | "onPointerDown" | "onPointerMove" | "onPointerUp" | "onPointerLeave" | "onPointerCancel" | "onWheel" | "onContextMenu" | "onDoubleClick" | "onLostPointerCapture"
>;

export type SpatialAutoFitBehavior = "initial" | "changes";

export function spatialAutoFitShouldRun(behavior: SpatialAutoFitBehavior, key: string, lastKey: string, hasApplied: boolean): boolean {
  if (!key || key === lastKey) return false;
  return behavior === "changes" || !hasApplied;
}

/** @emoji 🛰️ Frames the camera to fit committed meshes and/or factory geometry (playground auto-fit). */
export function SpatialAutoFit({
  meshes,
  geometry = null,
  padding = 1.25,
  behavior = "initial",
}: {
  readonly meshes: readonly MeshTransfer[];
  readonly geometry?: SpatialPickGeometry | null;
  readonly padding?: number;
  readonly behavior?: SpatialAutoFitBehavior;
}): null {
  const { camera, controls, invalidate } = useThree();
  const geometryRevision = geometry && typeof geometry === "object" && "revision" in geometry ? Number((geometry as { revision?: unknown }).revision) : 0;
  const bounds = reactHostPort.useMemo(() => mergeSpatialSceneBounds(boundsFromMeshTransfers(meshes), boundsFromSpatialPickGeometry(geometry)), [meshes, geometry, geometryRevision]);
  const lastKey = reactHostPort.useRef("");
  const hasApplied = reactHostPort.useRef(false);
  reactHostPort.useEffect(() => {
    if (!bounds) return;
    const meshKey = meshes.map((m, i) => meshTransferContentKey(m, i)).join("|");
    const key = `${geometryRevision}:${meshKey}`;
    if (!spatialAutoFitShouldRun(behavior, key, lastKey.current, hasApplied.current)) return;
    lastKey.current = key;
    hasApplied.current = true;
    applySpatialAutoFitCamera(camera, bounds, padding, controls);
    invalidate();
  }, [behavior, bounds, camera, controls, geometryRevision, invalidate, meshes, padding]);
  return null;
}

export function applySpatialAutoFitCamera(camera: THREE.Camera, bounds: { readonly center: Vec3; readonly radius: number }, padding = 1.25, controls?: unknown): void {
  const [cx, cy, cz] = bounds.center;
  const dist = Math.max(bounds.radius * padding, 2);
  camera.position.set(cx + dist, cy + dist, cz + dist * 0.85);
  const orbit = controls as { readonly target?: THREE.Vector3; update?: () => void } | undefined;
  if (orbit?.target) {
    orbit.target.set(cx, cy, cz);
    orbit.update?.();
  } else {
    camera.lookAt(cx, cy, cz);
  }
  if ("updateProjectionMatrix" in camera && typeof camera.updateProjectionMatrix === "function") {
    camera.updateProjectionMatrix();
  }
}

/** @emoji 🔄 Invalidates demand frameloop when host-driven scene visuals change. */
function InvalidateOnRevision({ revision }: { readonly revision: string | number }): null {
  const invalidate = useThree((state) => state.invalidate);
  reactHostPort.useEffect(() => {
    invalidate();
  }, [revision, invalidate]);
  return null;
}

/** @emoji 🎯 Redraws when host selection pick keys change (demand frameloop). */
function InteractionSelectionInvalidateBridge({ selectionKey }: { readonly selectionKey: string }): null {
  const invalidate = useThree((state) => state.invalidate);
  reactHostPort.useEffect(() => {
    invalidate();
  }, [selectionKey, invalidate]);
  return null;
}

/** @emoji 🔄 Keeps demand frameloop alive while the camera moves (playground `Invalidator`). */
function SpatialInvalidator(): null {
  const { controls, camera } = useThree();
  const lastPos = reactHostPort.useRef(new THREE.Vector3());
  const lastTarget = reactHostPort.useRef(new THREE.Vector3());
  useFrame(({ invalidate }) => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any -- drei OrbitControls
    const ctrl = controls as any;
    if (!ctrl) return;
    const target = ctrl.target as THREE.Vector3 | undefined;
    const moved = !camera.position.equals(lastPos.current) || (target ? !target.equals(lastTarget.current) : false);
    if (moved) {
      lastPos.current.copy(camera.position);
      if (target) lastTarget.current.copy(target);
      invalidate();
    }
  });
  return null;
}

/** @emoji 🛰️ Orbit controls that repaint on demand and never block R3F pointer routing. */
function SpatialOrbitControls({ onCameraNavigate }: { readonly onCameraNavigate?: (active: boolean) => void }): ReactNode {
  const invalidate = useThree((state) => state.invalidate);
  return (
    <OrbitControls
      makeDefault
      enableDamping={false}
      onChange={() => invalidate()}
      onStart={() => onCameraNavigate?.(true)}
      onEnd={() => onCameraNavigate?.(false)}
      mouseButtons={{
        LEFT: -1 as unknown as MOUSE,
        MIDDLE: MOUSE.DOLLY,
        RIGHT: MOUSE.ROTATE,
      }}
    />
  );
}

/** @emoji 🪩 Root `<Canvas>` configuration for factory viewports. */
export function InteractionCanvas({
  children,
  onCanvasReady,
  frameloop = "demand",
  background = defaultInteractionSpatialViewTheme.background,
  cameraPosition = [10, 10, 8],
  cameraFov = 45,
  cameraNear,
  cameraFar,
  dpr,
  shadows,
  style,
  className,
  gl,
  onPointerDown,
  onPointerMove,
  onPointerUp,
  onPointerLeave,
  onPointerCancel,
  onWheel,
  onContextMenu,
  onDoubleClick,
  onLostPointerCapture,
}: InteractionCanvasProps): ReactNode {
  return (
    <Canvas
      frameloop={frameloop}
      className={className}
      style={{ height: "100%", width: "100%", ...style }}
      dpr={dpr}
      shadows={shadows}
      camera={{
        up: [0, 0, 1],
        position: cameraPosition,
        fov: cameraFov,
        ...(cameraNear !== undefined ? { near: cameraNear } : {}),
        ...(cameraFar !== undefined ? { far: cameraFar } : {}),
      }}
      gl={gl}
      onPointerDown={(event) => onPointerDown?.(event.nativeEvent)}
      onPointerMove={(event) => onPointerMove?.(event.nativeEvent)}
      onPointerUp={(event) => onPointerUp?.(event.nativeEvent)}
      onPointerLeave={(event) => onPointerLeave?.(event.nativeEvent)}
      onPointerCancel={(event) => onPointerCancel?.(event.nativeEvent)}
      onWheel={(event) => onWheel?.(event.nativeEvent)}
      onContextMenu={(event) => onContextMenu?.(event.nativeEvent)}
      onDoubleClick={(event) => onDoubleClick?.(event.nativeEvent)}
      onLostPointerCapture={(event) => onLostPointerCapture?.(event.nativeEvent)}
      onCreated={({ camera, gl: renderer }) => onCanvasReady?.({ camera, domElement: renderer.domElement })}
    >
      {background ? <color attach="background" args={[background]} /> : null}
      {children}
    </Canvas>
  );
}

export interface InteractionSpatialViewProps {
  readonly previewKernel?: SpatialPreviewKernel;
  readonly snapshot: InteractionSnapshot;
  readonly onGroundPick?: (point: Vec3, event: InteractionEvent) => void;
  /** @emoji 🖱️ `pointer.move` hits ground (XY at fixed Z); height slab passes full 3D. */
  readonly onScenePointerMove?: (point: Vec3, event: InteractionEvent) => void;
  readonly onInteractionEvent?: (event: InteractionEvent) => void;
  readonly pickEnabled?: boolean;
  readonly committedMesh?: MeshTransfer | null;
  readonly committedMeshes?: readonly { readonly solid: SolidRef; readonly mesh: MeshTransfer }[];
  readonly geometry?: SpatialPickGeometry | null;
  /** @emoji 🧲 Pick-target source; defaults to `geometry` (use spatial.shape geometry when the active model is typology-only). */
  readonly pickGeometry?: SpatialPickGeometry | null;
  readonly activeModelDefinitionId?: string | null;
  readonly modelDefinitionRevision?: number;
  /** @emoji 🖼️ When set, drives `InteractionDisplay` instead of `snapshot.display` (e.g. merged archived footprints). */
  readonly displayModel?: DisplayModel;
  readonly renderDisplayItem?: SpatialDisplayItemRenderer;
  readonly selectionAccept?: readonly ModelEntityKind[];
  readonly filterKindToggles?: SpatialPickKindToggles;
  readonly selectionKindToggles?: SpatialPickKindToggles;
  /** @emoji 🖱️ Hover raycast kind filter; defaults to `selectionKindToggles` when omitted. */
  readonly hoverKindToggles?: SpatialPickKindToggles;
  readonly hoveredTargetKey?: string | null;
  readonly selectedTargetKey?: string | null;
  readonly selectedTargetKeys?: ReadonlySet<string> | null;
  readonly hostSelectionEnabled?: boolean;
  readonly onSelectionRequest?: (request: SpatialSelectionRequest) => void;
  readonly onCameraNavigate?: (active: boolean) => void;
  readonly onCommittedFacePointerDown?: (info: FaceInfo, event: ThreeEvent<PointerEvent>) => void;
  readonly onCommittedFacePointerMove?: (info: FaceInfo, event: ThreeEvent<PointerEvent>) => void;
  readonly onSnapshotStateChange?: (state: string) => void;
  readonly onSnapshotRevisionChange?: (revision: number) => void;
  readonly onPickEnabledChange?: (enabled: boolean) => void;
  /** @emoji 🧲 When false, skips pick-target meshes (during active interaction sessions). */
  readonly showPickLayer?: boolean;
  readonly committedMeshPickable?: boolean;
  readonly autoFitMeshes?: boolean;
  readonly autoFitBehavior?: SpatialAutoFitBehavior;
  readonly theme?: InteractionSpatialViewTheme;
  readonly slots?: InteractionSpatialViewSlots;
  /** @emoji 🎛 When set with targets, shows a gumball at the selection centroid (toolbar move/rotate/scale). */
  readonly transformGumballMode?: CadTransformGumballMode | null;
  readonly transformGumballTargets?: readonly SelectionTarget[];
  readonly onTransformGumballCommit?: (diff: ModelDiff) => void;
}

/** @emoji 📡 Host event callbacks accepted by {@link InteractionSpatialView}. */
export type InteractionSpatialViewHostCallbacks = Pick<
  InteractionSpatialViewProps,
  "onGroundPick" | "onScenePointerMove" | "onInteractionEvent" | "onSelectionRequest" | "onCameraNavigate" | "onCommittedFacePointerDown" | "onCommittedFacePointerMove" | "onSnapshotStateChange" | "onSnapshotRevisionChange" | "onPickEnabledChange"
>;

/** @emoji 🖱️ Ground-plane picking is command input and must stay independent from host geometry selection. */
export function interactionSpatialGroundPickPlaneEnabled(snapshot: Pick<InteractionSnapshot, "spatialInteraction" | "state">, pickEnabled: boolean): boolean {
  const si = snapshot.spatialInteraction;
  return pickEnabled !== false && si.spatialGroundPick && !si.pickDisabledStates.includes(snapshot.state);
}

/** @emoji 🪩 Lights, orbit controls, ground picking, factory overlays, optional committed mesh. */
export function InteractionSpatialView({
  previewKernel = r3fPreviewKernel,
  snapshot,
  onGroundPick,
  onScenePointerMove,
  onInteractionEvent,
  pickEnabled = true,
  committedMesh,
  committedMeshes,
  geometry,
  pickGeometry: pickGeometryProp,
  activeModelDefinitionId = defaultModelDefinitionId(),
  modelDefinitionRevision = 0,
  displayModel,
  renderDisplayItem,
  selectionAccept = [],
  filterKindToggles = {},
  hoveredTargetKey,
  selectedTargetKey,
  selectedTargetKeys,
  selectionKindToggles = {},
  hostSelectionEnabled = false,
  onSelectionRequest,
  onCameraNavigate,
  onCommittedFacePointerDown,
  onCommittedFacePointerMove,
  onSnapshotStateChange,
  onSnapshotRevisionChange,
  onPickEnabledChange,
  showPickLayer = true,
  committedMeshPickable = false,
  autoFitMeshes = false,
  autoFitBehavior = "initial",
  theme = defaultInteractionSpatialViewTheme,
  slots,
  transformGumballMode = null,
  transformGumballTargets = [],
  onTransformGumballCommit,
}: InteractionSpatialViewProps): ReactNode {
  reactHostPort.useEffect(() => {
    bindScenePreviewKernel(previewKernel);
  }, [previewKernel]);
  reactHostPort.useEffect(() => {
    onSnapshotStateChange?.(snapshot.state);
  }, [snapshot.state, onSnapshotStateChange]);
  reactHostPort.useEffect(() => {
    onSnapshotRevisionChange?.(snapshot.revision);
  }, [snapshot.revision, onSnapshotRevisionChange]);
  const resolvedTheme = { ...defaultInteractionSpatialViewTheme, ...theme };
  const gridDivisions = resolvedTheme.gridDivisions ?? 40;
  const gridSize = resolvedTheme.gridSize ?? 40;
  const gridHelper = reactHostPort.useMemo(() => {
    const palette = spatialSceneColors();
    const g = new THREE.GridHelper(gridSize, gridDivisions, spatialSceneColorToHex(palette.gridMajor), spatialSceneColorToHex(palette.gridMinor));
    g.rotation.x = Math.PI / 2;
    g.position.set(0, 0, 0.002);
    g.traverse((obj) => {
      obj.raycast = raycastNone;
    });
    return g;
  }, [gridDivisions, gridSize]);
  const layerMeshes = reactHostPort.useMemo(() => {
    if (committedMeshes?.length) return committedMeshes;
    if (committedMesh) return [{ solid: solidRef("committed"), mesh: committedMesh }];
    return [];
  }, [committedMeshes, committedMesh]);
  const autoFitSources = reactHostPort.useMemo(() => layerMeshes.map((row) => row.mesh), [layerMeshes]);
  const ctx = snapshot.context;
  const geometryPreviewTransform = reactHostPort.useMemo(() => geometryPreviewTransformFromDisplay(displayModel ?? snapshot.display), [displayModel, snapshot.display]);
  const origin = vec3FromSnapshotContext(ctx, "origin") ?? vec3FromSnapshotContext(ctx, "pointA");
  const corner = vec3FromSnapshotContext(ctx, "corner") ?? vec3FromSnapshotContext(ctx, "pointB");
  const si = snapshot.spatialInteraction;
  const groundMoveOn = si.spatialGroundPick && si.groundPointerMoveStates.includes(snapshot.state) && Boolean(onScenePointerMove);
  const heightMoveOn = si.spatialGroundPick && si.heightDragStates.includes(snapshot.state) && Boolean(onScenePointerMove) && origin !== null && corner !== null;
  const zRodMoveOn = si.spatialGroundPick && si.verticalRodStates.includes(snapshot.state) && Boolean(onScenePointerMove) && origin !== null;
  const pickPlaneEnabled = interactionSpatialGroundPickPlaneEnabled(snapshot, pickEnabled);
  reactHostPort.useEffect(() => {
    onPickEnabledChange?.(pickPlaneEnabled);
  }, [pickPlaneEnabled, onPickEnabledChange]);
  const onGroundPickEvent = (point: Vec3) => {
    const event = createSpatialPickEvent("pointer.down", point, null);
    onInteractionEvent?.(event);
    onGroundPick?.(point, event);
  };
  const onGroundContextEvent = (point: Vec3) => {
    onInteractionEvent?.({ kind: "contextmenu", point, modifiers: {} });
  };
  const onScenePointerMoveEvent = (point: Vec3) => {
    const event = createSpatialPickEvent("pointer.move", point, null);
    onInteractionEvent?.(event);
    onScenePointerMove?.(point, event);
  };
  const dirPos = resolvedTheme.directionalPosition ?? [12, 18, 10];
  const geometryRevision = geometry && typeof geometry === "object" && "revision" in geometry ? Number((geometry as { revision?: unknown }).revision) : 0;
  const sceneVisibility = reactHostPort.useMemo(() => resolveSpatialSceneVisibility(activeModelDefinitionId, filterKindToggles), [activeModelDefinitionId, filterKindToggles]);
  const scenePickGeometry = geometry ?? pickGeometryProp;
  const pickGeometryRevision = scenePickGeometry && typeof scenePickGeometry === "object" && "revision" in scenePickGeometry ? Number((scenePickGeometry as { revision?: unknown }).revision) : 0;
  return (
    <>
      {slots?.beforeScene}
      <InvalidateOnRevision revision={`${snapshot.revision}:${modelDefinitionRevision}:${geometryRevision}:${pickGeometryRevision}:${hoveredTargetKey ?? ""}:${selectedTargetKey ?? ""}:${selectedTargetKeys?.size ?? 0}`} />
      <SpatialInvalidator />
      {autoFitMeshes ? <SpatialAutoFit meshes={autoFitSources} geometry={geometry} behavior={autoFitBehavior} /> : null}
      {slots?.environment}
      {slots?.lights ?? (
        <>
          <ambientLight intensity={resolvedTheme.ambientIntensity ?? 0.45} />
          <directionalLight position={dirPos} intensity={resolvedTheme.directionalIntensity ?? 1.1} />
        </>
      )}
      <SpatialOrbitControls onCameraNavigate={onCameraNavigate} />
      <primitive object={gridHelper} />
      <GroundPickPlane
        enabled={pickPlaneEnabled}
        onPick={onGroundPickEvent}
        onContextPick={onGroundContextEvent}
        onPointerMove={onScenePointerMoveEvent}
        pointerMoveEnabled={groundMoveOn}
        planeColor={resolvedTheme.groundPlaneColor ?? spatialSceneColors().groundPlane}
        planeOpacity={resolvedTheme.groundPlaneOpacity}
      />
      <GeometryFactoryWireframeLayer geometry={scenePickGeometry} visible={sceneVisibility.showFactoryWireframe} />
      {showPickLayer ? (
        <SpatialPickGeometryLayer
          geometry={scenePickGeometry}
          activeModelDefinitionId={activeModelDefinitionId}
          modelDefinitionRevision={modelDefinitionRevision}
          geometryPreviewTransform={geometryPreviewTransform}
          selectionAccept={selectionAccept}
          selectionKindToggles={selectionKindToggles}
          filterKindToggles={filterKindToggles}
          hoveredTargetKey={hoveredTargetKey}
          selectedTargetKey={selectedTargetKey}
          selectedTargetKeys={selectedTargetKeys}
          hostSelectionEnabled={hostSelectionEnabled}
          onSelectionRequest={onSelectionRequest}
        />
      ) : null}
      {heightMoveOn && origin && corner ? <HeightDragSurface origin={origin} corner={corner} enabled={heightMoveOn} onPointerMove={onScenePointerMoveEvent} /> : null}
      {zRodMoveOn && origin ? <VerticalZDragRod origin={origin} enabled={zRodMoveOn} onPointerMove={onScenePointerMoveEvent} /> : null}
      <CommittedMeshLayer
        meshes={layerMeshes}
        modelRevision={geometry?.revision ?? 0}
        pickable={committedMeshPickable}
        showFaces={sceneVisibility.showCommittedFaces}
        showEdges={sceneVisibility.showCommittedEdges}
        onFacePointerDown={onCommittedFacePointerDown}
        onFacePointerMove={onCommittedFacePointerMove}
      />
      <InteractionDisplay geometry={geometry} model={displayModel ?? snapshot.display} renderItem={renderDisplayItem} />
      {slots?.afterDisplay}
      {slots?.afterCommitted}
      {transformGumballMode && geometry && onTransformGumballCommit ? (
        <SpatialTransformGumball mode={transformGumballMode} model={geometry} targets={transformGumballTargets} previewKernel={previewKernel} onCommit={onTransformGumballCommit} />
      ) : null}
    </>
  );
}
// #endregion 🪩Canvas

// #region 🪩Repl
/** @emoji ☑️ Master checkbox for a chrome toggle group (supports indeterminate partial state). */
function SpatialChromeMasterToggle({ state, onEnabledChange, ariaLabel }: { readonly state: SpatialToggleGroupState; readonly onEnabledChange: (enabled: boolean) => void; readonly ariaLabel: string }): ReactNode {
  const inputRef = reactHostPort.useRef<HTMLInputElement>(null);
  reactHostPort.useEffect(() => {
    if (inputRef.current) inputRef.current.indeterminate = state === "partial";
  }, [state]);
  return <input ref={inputRef} type="checkbox" aria-label={ariaLabel} checked={state === "all"} onChange={(e) => onEnabledChange(e.target.checked)} />;
}

type ReplSuggestKind = "interaction" | "transition" | "action" | "selection";

interface ReplSuggestion {
  readonly kind: ReplSuggestKind;
  readonly key: string;
  readonly label: string;
  readonly detail: string;
  readonly transition?: InteractionKeybindRow;
  readonly interactionId?: string;
  readonly onRun: () => void;
}

function resolveScopedSpatialInteractionKey(token: string, modelDefinitionId: string): SpatialInteraction | null {
  return resolveSpatialInteractionKeyForModelDefinition(modelDefinitionId, token);
}

function replCommandTextWithoutSpaces(text: string): string {
  return text.replace(/\s+/g, "");
}

function replFirstWireId(model: Model): string | null {
  const ks = Object.keys(model.wires);
  return ks.length ? model.wires[ks[0]!]!.id : null;
}

function replFirstFaceId(model: Model): string | null {
  const ks = Object.keys(model.faces);
  return ks.length ? model.faces[ks[0]!]!.id : null;
}

function replBuildDispatchEvent(row: InteractionKeybindRow, opts: { readonly interactionId: string; readonly model: Model }): InteractionEvent | null {
  const { interactionId, model } = opts;
  if (row.eventKind === "set.height" || row.eventKind === "set.distance" || row.eventKind === "set.footprint") return null;
  if (row.eventKind === "selection.changed") {
    if (interactionId === "feature.extrudeWire") {
      const wid = replFirstWireId(model);
      if (!wid) return null;
      return { kind: "selection.changed", targets: [{ kind: "wire", id: wid, editable: true }], modifiers: {} };
    }
    if (interactionId === "feature.offsetSurface") {
      const fid = replFirstFaceId(model);
      if (!fid) return null;
      return { kind: "selection.changed", targets: [{ kind: "face", id: fid, editable: true }], modifiers: {} };
    }
    return null;
  }
  return { kind: row.eventKind, modifiers: {} };
}

/** @emoji 📏 Parses REPL `cmdLine` as a live direct-distance value (`null` = empty, `undefined` = not numeric). */
export function replLengthEntryLiveValue(cmdLine: string): number | null | undefined {
  return parseNumericCommandLine(cmdLine);
}

function replTryParseValueInteraction(line: string, spec: InteractionSpec, state: string): InteractionEvent | null {
  const t = line.trim();
  const m = t.match(/^(\S+)\s+(.+)$/);
  if (!m) return null;
  const head = m[1]!.toLowerCase();
  const tail = m[2]!.trim();
  const rows = listKeyedInteractionTransitions(spec, state);
  for (const row of rows) {
    if (row.eventKind === "set.height") {
      if (head !== row.key.toLowerCase() && head !== "height") continue;
      const v = Number(tail);
      if (!Number.isFinite(v) || v <= 0) return null;
      return { kind: "set.height", value: v, modifiers: {} };
    }
    if (row.eventKind === "set.distance") {
      if (head !== row.key.toLowerCase() && head !== "dist" && head !== "distance") continue;
      const v = Number(tail);
      if (!Number.isFinite(v)) return null;
      return { kind: "set.distance", value: v, modifiers: {} };
    }
    if (row.eventKind === "set.footprint") {
      if (head !== row.key.toLowerCase() && head !== "footprint" && head !== "lw") continue;
      const parts = tail.split(/\s+/);
      const L = Number(parts[0]);
      const W = Number(parts[1]);
      if (!Number.isFinite(L) || !Number.isFinite(W)) return null;
      return { kind: "set.footprint", value: { length: L, width: W }, modifiers: {} };
    }
    if (row.eventKind.startsWith("set.")) {
      const alias = row.eventKind.slice("set.".length).toLowerCase();
      if (head !== row.key.toLowerCase() && head !== alias && head !== "number" && head !== "n") continue;
      const v = Number(tail);
      if (!Number.isFinite(v)) return null;
      return { kind: row.eventKind, value: v, modifiers: {} };
    }
  }
  return null;
}

function replSuggestionHaystack(s: ReplSuggestion): string {
  return `${s.key} ${s.label} ${s.detail}`.toLowerCase();
}

function replRankScore(query: string, s: ReplSuggestion): number {
  const ql = query.trim().toLowerCase();
  if (!ql) return -1;
  const key = s.key.toLowerCase();
  const label = s.label.toLowerCase();
  const detail = s.detail.toLowerCase();
  if (key.startsWith(ql)) return 4000 - key.length;
  if (label.startsWith(ql)) return 3000 - label.length;
  if (detail.startsWith(ql)) return 2000 - detail.length;
  if (replSuggestionHaystack(s).includes(ql)) return 1000;
  return -1;
}

export function replFilterSuggestions(query: string, all: readonly ReplSuggestion[]): ReplSuggestion[] {
  const q = query.trim();
  if (!q) return [];
  return all
    .map((s) => ({ s, score: replRankScore(q, s) }))
    .filter((row) => row.score >= 0)
    .sort((a, b) => b.score - a.score)
    .map((row) => row.s);
}

/** @emoji ⌨️ Inline completion suffix for the active suggestion (longest prefix match on key, label, or detail). */
export function replCompletionSuffix(query: string, suggestion: ReplSuggestion | undefined): string {
  if (!query.trim() || !suggestion) return "";
  const q = query;
  const ql = q.toLowerCase();
  let best = "";
  for (const text of [suggestion.label, suggestion.detail, suggestion.key]) {
    if (!text.toLowerCase().startsWith(ql)) continue;
    const suffix = text.slice(q.length);
    if (suffix.length > best.length) best = suffix;
  }
  return best;
}

/** @emoji ⌨️ First non-empty inline completion suffix across ranked matches. */
export function replActiveCompletionSuffix(query: string, matches: readonly ReplSuggestion[], index: number): string {
  if (!query.trim() || !matches.length) return "";
  const order = [matches[Math.min(index, matches.length - 1)]!, ...matches];
  const seen = new Set<ReplSuggestion>();
  for (const s of order) {
    if (seen.has(s)) continue;
    seen.add(s);
    const suffix = replCompletionSuffix(query, s);
    if (suffix) return suffix;
  }
  return "";
}

export function replPaletteRows(cmdLine: string, all: readonly ReplSuggestion[]): ReplSuggestion[] {
  return replFilterSuggestions(cmdLine, all);
}

function replInteractionSuggestions(query: string, all: readonly ReplSuggestion[]): ReplSuggestion[] {
  const xs = query.trim() ? replFilterSuggestions(query, all) : all;
  return xs.filter((suggestion) => suggestion.kind === "interaction");
}

function replExactInteractionSuggestion(query: string, all: readonly ReplSuggestion[]): ReplSuggestion | null {
  const raw = query.trim().toLowerCase();
  if (!raw) return null;
  for (const suggestion of all) {
    if (suggestion.kind !== "interaction") continue;
    for (const text of [suggestion.key, suggestion.label, suggestion.detail]) {
      if (text.toLowerCase() === raw) return suggestion;
    }
  }
  return null;
}

function replInteractionSuggestionOnSpace(query: string, matches: readonly ReplSuggestion[], all: readonly ReplSuggestion[]): ReplSuggestion | null {
  const exact = replExactInteractionSuggestion(query, all);
  if (exact) return exact;
  return matches.find((suggestion) => suggestion.kind === "interaction") ?? null;
}

function replInteractionIdOnSpace(query: string, matches: readonly ReplSuggestion[], all: readonly ReplSuggestion[], lastFinalizedInteractionId: string): string | null {
  if (!query.trim()) return lastFinalizedInteractionId || null;
  return replInteractionSuggestionOnSpace(query, matches, all)?.interactionId ?? null;
}

/** @emoji ⌨️ True when the event target is already a text field (skip REPL global key capture). */
export function replIsQueryTypingTarget(t: EventTarget | null): boolean {
  if (!(t instanceof HTMLElement)) return false;
  if (t instanceof HTMLTextAreaElement) return true;
  if (t instanceof HTMLInputElement) {
    const type = (t.type || "text").toLowerCase();
    return type !== "button" && type !== "checkbox" && type !== "radio" && type !== "file" && type !== "range" && type !== "color";
  }
  return Boolean(t.closest('[data-slot="engagement"] input, [data-slot="engagement"] textarea'));
}

function replShouldRepeatInteractionOnSpace(
  event: {
    readonly key: string;
    readonly ctrlKey: boolean;
    readonly metaKey: boolean;
    readonly altKey: boolean;
    readonly defaultPrevented: boolean;
    readonly isComposing: boolean;
    readonly target: EventTarget | null;
  },
  state: {
    readonly interactionActive: boolean;
    readonly cmdTarget: EventTarget | null;
  },
): boolean {
  if (event.defaultPrevented || event.isComposing || state.interactionActive) return false;
  if (event.key !== " " || event.ctrlKey || event.metaKey || event.altKey) return false;
  if (replIsQueryTypingTarget(event.target)) return false;
  return event.target !== state.cmdTarget;
}

function replEscapeAction(state: { readonly hasInteraction: boolean; readonly interactionActive: boolean; readonly cmdLine: string; readonly hasSelectionMenu: boolean }): "abort" | "dismiss" | "none" {
  if (state.hasInteraction || state.interactionActive) return "abort";
  if (state.cmdLine.trim() || state.hasSelectionMenu) return "dismiss";
  return "none";
}

function replSelectionEvent(selection: readonly SelectionTarget[], point?: Vec3): InteractionEvent {
  return point ? { kind: "selection.changed", targets: selection, point, modifiers: {} } : { kind: "selection.changed", targets: selection, modifiers: {} };
}

function replStartEvent(selection: readonly SelectionTarget[]): InteractionEvent {
  return { kind: "start", targets: selection, modifiers: {} };
}

function replSelectionAccepted(accept: readonly ModelEntityKind[], selection: readonly SelectionTarget[]): SelectionTarget[] {
  return selection.filter((target) => accept.includes(target.kind));
}

/** @emoji 🪪 Reads validated `context.targets` for interaction highlight sync. */
export function replInteractionSelectionFromContext(ctx: Record<string, unknown>): readonly SelectionTarget[] {
  const raw = ctx.targets;
  if (!Array.isArray(raw)) return [];
  return raw.filter((target): target is SelectionTarget => {
    return Boolean(target && typeof target === "object" && "kind" in target && "id" in target && typeof (target as { kind?: unknown }).kind === "string" && typeof (target as { id?: unknown }).id === "string");
  });
}

/** @emoji 🪪 Shallow equality for ordered selection target lists. */
export function replSelectionTargetsEqual(a: readonly SelectionTarget[], b: readonly SelectionTarget[]): boolean {
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) {
    const x = a[i]!;
    const y = b[i]!;
    if (x.kind !== y.kind || x.id !== y.id || x.editable !== y.editable) return false;
  }
  return true;
}

function replApplySelectionPick(current: readonly SelectionTarget[], picked: readonly SelectionTarget[], modifiers: InteractionEvent["modifiers"]): SelectionTarget[] {
  const modeModifiers = (modifiers ?? {}) as { readonly alt?: boolean; readonly ctrl?: boolean; readonly meta?: boolean; readonly shift?: boolean };
  return mergeSelectionTargets(current, picked, spatialSelectionModeFromModifiers(modeModifiers));
}

/** @emoji 🗂️ Renderer highlight targets keyed by model definition id. */
export type SpatialRendererSelectionByModel = Readonly<Record<string, readonly SelectionTarget[]>>;

/** @emoji 🗂️ Interaction pick targets keyed by interaction state id (session-local). */
export type SpatialInteractionSelectionByState = Readonly<Record<string, readonly SelectionTarget[]>>;

/** @emoji 🪪 Reads renderer selection for one model definition (empty when unset). */
export function replRendererSelectionTargets(byModel: SpatialRendererSelectionByModel, modelDefinitionId: string): readonly SelectionTarget[] {
  return byModel[modelDefinitionId] ?? [];
}

/** @emoji 🪪 Updates renderer selection for one model definition without touching other models. */
export function replWithRendererSelectionTargets(byModel: SpatialRendererSelectionByModel, modelDefinitionId: string, targets: readonly SelectionTarget[]): SpatialRendererSelectionByModel {
  const prev = byModel[modelDefinitionId] ?? [];
  if (replSelectionTargetsEqual(prev, targets)) return byModel;
  return { ...byModel, [modelDefinitionId]: [...targets] };
}

/** @emoji 🪪 Reads interaction selection for one state (empty when unset). */
export function replInteractionSelectionTargets(byState: SpatialInteractionSelectionByState, stateId: string): readonly SelectionTarget[] {
  return byState[stateId] ?? [];
}

/** @emoji 🪪 Updates interaction selection for one state without touching other states. */
export function replWithInteractionSelectionTargets(byState: SpatialInteractionSelectionByState, stateId: string, targets: readonly SelectionTarget[]): SpatialInteractionSelectionByState {
  const prev = byState[stateId] ?? [];
  if (replSelectionTargetsEqual(prev, targets)) return byState;
  return { ...byState, [stateId]: [...targets] };
}

/** @emoji 🪪 Removes in-view targets of a pick kind when its selection toggle is turned off. */
export function replPruneSelectionByKind(selection: readonly SelectionTarget[], activeModelDefinitionId: string | null, kind: SpatialPickTargetKind): SelectionTarget[] {
  if (!spatialPickKindsForActiveView(activeModelDefinitionId).has(kind)) return [...selection];
  if (kind === "object") {
    return selection.filter((target) => target.kind !== "object" || target.editable !== false);
  }
  const geometryKinds: readonly ModelEntityKind[] = kind === "vertex" ? ["vertex", "anchor"] : kind === "edge" ? ["edge", "wire"] : kind === "face" ? ["face", "shell"] : ["solid", "geometry"];
  return selection.filter((target) => !geometryKinds.includes(target.kind) && selectionTargetPickKind(target) !== kind);
}

/** @emoji 🪪 Removes in-view selection rows for a factory primitive kind when its filter toggle is turned off. */
export function replPruneSelectionByPrimitive(selection: readonly SelectionTarget[], primitiveKind: ModelEntityKind): SelectionTarget[] {
  return selection.filter((target) => {
    if (target.kind === "object" && target.editable === false) return true;
    return target.kind !== primitiveKind;
  });
}

/** @emoji 🪪 Removes in-view selection rows for a typology when its selection toggle is turned off. */
export function replPruneSelectionByTypology(selection: readonly SelectionTarget[], model: Model, activeModelDefinitionId: string | null, typologyId: string): SelectionTarget[] {
  const typologyIds = modelDefinitionTypologyIds(activeModelDefinitionId);
  if (!typologyIds.includes(typologyId)) return [...selection];
  const index = buildGeometryTypologyIndex(model, activeModelDefinitionId ?? defaultModelDefinitionId());
  return selection.filter((target) => {
    if (target.kind === "object" && target.editable === false) {
      const row = model.objects[target.id];
      return row?.typology !== typologyId;
    }
    const geometryKind = target.kind === "object" ? "solid" : target.kind;
    return index.get(`${geometryKind}:${target.id}`) !== typologyId;
  });
}

/** @emoji 🪪 Picks the highlight layer: interaction state selection while active, else renderer selection for the active model. */
export function replDisplayedSelectionTargets(
  interactionActive: boolean,
  activeModelDefinitionId: string | null,
  interactionState: string,
  rendererByModel: SpatialRendererSelectionByModel,
  interactionByState: SpatialInteractionSelectionByState,
): readonly SelectionTarget[] {
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  if (interactionActive) return replInteractionSelectionTargets(interactionByState, interactionState);
  return replRendererSelectionTargets(rendererByModel, mdId);
}

/** @emoji 🪪 Merges a pick into the active renderer model or interaction state selection slice. */
export function replMergeSelectionPickInView(
  interactionActive: boolean,
  activeModelDefinitionId: string | null,
  interactionState: string,
  rendererByModel: SpatialRendererSelectionByModel,
  interactionByState: SpatialInteractionSelectionByState,
  picked: readonly SelectionTarget[],
  modifiers: InteractionEvent["modifiers"] = {},
): SelectionTarget[] {
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  const current = interactionActive ? replInteractionSelectionTargets(interactionByState, interactionState) : replRendererSelectionTargets(rendererByModel, mdId);
  return replApplySelectionPick(current, picked, modifiers);
}

/** @emoji 🪪 Applies archived interaction result to renderer selection for the active model when `archiveContext.targets` is set (including `[]`). */
export function replFinalizeSelection(rendererByModel: SpatialRendererSelectionByModel, activeModelDefinitionId: string | null, result: InteractionSnapshot["lastResponse"]): SpatialRendererSelectionByModel {
  const ctx = result?.archiveContext;
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  if (!ctx || typeof ctx !== "object" || !Object.hasOwn(ctx, "targets")) return rendererByModel;
  const targets = replInteractionSelectionFromContext(ctx as Record<string, unknown>);
  return replWithRendererSelectionTargets(rendererByModel, mdId, targets);
}

/** @emoji 🪩 Memoized `DocumentHistory` for REPL hosts. */
export function useDocumentHistory(): DocumentHistory {
  return reactHostPort.useMemo(() => new DocumentHistory(), []);
}

/** @emoji 🪩 Labels + capability mirror for undo/redo chrome (uses `InteractionSnapshot.capabilities`). */
export function getReplHistoryPresentation(spec: InteractionSpec, snap: InteractionSnapshot, history: DocumentHistory): { readonly canUndo: boolean; readonly canRedo: boolean; readonly undoLabel: string; readonly redoLabel: string } {
  const active = isInteractionSessionActive(spec, snap.state);
  const u = history.peekUndo()?.label ?? "";
  const r = history.peekRedo()?.label ?? "";
  return {
    canUndo: snap.capabilities.canUndo,
    canRedo: snap.capabilities.canRedo,
    undoLabel: active ? "Interaction input" : u,
    redoLabel: active ? "Interaction input" : r,
  };
}

/** @emoji 🪩 Subscribes to runtime revisions and derives REPL undo/redo labels. */
export function useReplHistoryState(rt: InteractionRuntime, spec: InteractionSpec, history: DocumentHistory) {
  const snap = useInteractionSnapshot(rt);
  return reactHostPort.useMemo(() => getReplHistoryPresentation(spec, snap, history), [spec, snap, history]);
}

/** @emoji 🎛️ Optional controlled chrome state for {@link InteractionRepl}. */
export interface InteractionReplHostValues {
  readonly cmdLine?: string;
  readonly activeSuggestionIndex?: number;
  readonly filterTypologyToggles?: SpatialTypologyToggles;
  readonly selectionTypologyToggles?: SpatialTypologyToggles;
  readonly filterPrimitiveToggles?: SpatialPrimitiveToggles;
  readonly selectionPrimitiveToggles?: SpatialPrimitiveToggles;
  readonly activeModelDefinitionId?: string | null;
  readonly selectionMethod?: SpatialSelectionMethod;
  readonly modelDefinitionRevision?: number;
  readonly dragSelection?: SpatialDragSelectionState | null;
  readonly selectionMenu?: SpatialSelectionRequest | null;
  readonly hoveredPickKey?: string | null;
  readonly rendererSelectionByModel?: SpatialRendererSelectionByModel;
  readonly interactionSelectionByState?: SpatialInteractionSelectionByState;
  readonly interactionMenuOpen?: boolean;
  readonly lastFinalizedInteractionId?: string;
}

/** @emoji 📡 Optional `on*` host callbacks for {@link InteractionRepl}. */
export interface InteractionReplHostCallbacks {
  readonly onCmdLineChange?: (value: string) => void;
  readonly onActiveSuggestionIndexChange?: (index: number) => void;
  readonly onFilterTypologyTogglesChange?: (value: SpatialTypologyToggles) => void;
  readonly onSelectionTypologyTogglesChange?: (value: SpatialTypologyToggles) => void;
  readonly onFilterPrimitiveTogglesChange?: (value: SpatialPrimitiveToggles) => void;
  readonly onSelectionPrimitiveTogglesChange?: (value: SpatialPrimitiveToggles) => void;
  readonly onActiveModelDefinitionIdChange?: (value: string) => void;
  readonly onSelectionMethodChange?: (value: SpatialSelectionMethod) => void;
  readonly onModelDefinitionRevisionChange?: (revision: number) => void;
  readonly onDragSelectionChange?: (value: SpatialDragSelectionState | null) => void;
  readonly onSelectionMenuChange?: (value: SpatialSelectionRequest | null) => void;
  readonly onHoveredPickKeyChange?: (key: string | null) => void;
  readonly onRendererSelectionByModelChange?: (value: SpatialRendererSelectionByModel) => void;
  readonly onInteractionSelectionByStateChange?: (value: SpatialInteractionSelectionByState) => void;
  readonly onInteractionMenuOpenChange?: (open: boolean) => void;
  readonly onLastFinalizedInteractionIdChange?: (id: string) => void;
  readonly onCanvasReady?: InteractionCanvasProps["onCanvasReady"];
  readonly onInteractionEvent?: (event: InteractionEvent) => void;
  readonly onGroundPick?: (point: Vec3, event: InteractionEvent) => void;
  readonly onScenePointerMove?: (point: Vec3, event: InteractionEvent) => void;
  readonly onSelectionRequest?: (request: SpatialSelectionRequest) => void;
  readonly onHoverTarget?: (target: SpatialPickTarget | null) => void;
  readonly onCameraNavigate?: (active: boolean) => void;
  readonly onCommandSubmit?: (line: string) => boolean | void;
  readonly onTransitionRun?: (row: InteractionKeybindRow) => void;
  readonly onCancel?: () => void;
  readonly onUndo?: () => void;
  readonly onRedo?: () => void;
  readonly onSnapshotChange?: (snapshot: InteractionSnapshot) => void;
  /** @emoji 💬 Publishes the window engagement spec (or `null`) whenever the interaction state changes; the host renders it in the {@link Window} engagement slot. */
  readonly onEngagementChange?: (engagement: EngagementSpec | null) => void;
  readonly onEscape?: () => void;
  /** @emoji 🗑️ Delete/Backspace when deletable selection exists; return true when handled. */
  readonly onDeleteSelection?: () => boolean;
  readonly onApplyTransformation?: (spec: TransformationSpec) => void;
  /** @emoji 🧲 Geometry used for pick targets (defaults to `geometry`; use spatial.shape geometry when the active model is typology-only). */
  readonly pickGeometry?: SpatialPickGeometry | null;
  readonly onDocumentModelChange?: (model: Model) => void;
}

/** @emoji 📐 Layout and partial canvas/spatial-view overrides for {@link InteractionRepl}. */
export interface InteractionReplLayoutProps {
  readonly rootStyle?: CSSProperties;
  readonly asideStyle?: CSSProperties;
  readonly showAside?: boolean;
  /** @emoji 💬 Builds the window {@link EngagementSpec} (interaction options/input/status) and publishes it via {@link InteractionReplHostCallbacks.onEngagementChange}. */
  readonly showEngagement?: boolean;
  /** @emoji 📐 Size the REPL to its host instead of the viewport (`100vh`); stacks aside under the canvas. */
  readonly fillHost?: boolean;
  /** @emoji ⌨️ Registers document-level REPL key capture; disable on inactive hosts (e.g. CAD play quad panes). */
  readonly captureGlobalKeys?: boolean;
  /** @emoji 🙈 Hides model-definition and transformation dropdowns (e.g. play hosts them in `asideExtra`). */
  readonly hideModelDefinitionControls?: boolean;
  readonly frameloop?: InteractionCanvasProps["frameloop"];
  readonly canvas?: Omit<InteractionCanvasProps, "children">;
  /** @emoji 🖼️ Spread after REPL wiring; overrides win (use for theme/slots/face handlers, not session pick state). */
  readonly spatialView?: Omit<
    InteractionSpatialViewProps,
    | "snapshot"
    | "geometry"
    | "committedMeshes"
    | "displayModel"
    | "modelDefinitionRevision"
    | "activeModelDefinitionId"
    | "filterTypologyToggles"
    | "selectionTypologyToggles"
    | "hoveredTargetKey"
    | "selectedTargetKey"
    | "selectedTargetKeys"
    | "selectionAccept"
    | "showPickLayer"
    | "pickEnabled"
    | "onInteractionEvent"
    | "onScenePointerMove"
    | "onSelectionRequest"
    | "onCameraNavigate"
    | "onGroundPick"
  >;
}

/** @emoji 🎛️ Default uncontrolled chrome for {@link InteractionRepl}. */
export function defaultInteractionReplChromeState(): Required<
  Pick<
    InteractionReplHostValues,
    | "cmdLine"
    | "activeSuggestionIndex"
    | "filterTypologyToggles"
    | "selectionTypologyToggles"
    | "filterPrimitiveToggles"
    | "selectionPrimitiveToggles"
    | "activeModelDefinitionId"
    | "selectionMethod"
    | "modelDefinitionRevision"
    | "dragSelection"
    | "selectionMenu"
    | "hoveredPickKey"
    | "rendererSelectionByModel"
    | "interactionSelectionByState"
    | "interactionMenuOpen"
    | "lastFinalizedInteractionId"
  >
> {
  return {
    cmdLine: "",
    activeSuggestionIndex: 0,
    filterTypologyToggles: defaultSpatialTypologyTogglesForModelDefinition(defaultModelDefinitionId()),
    selectionTypologyToggles: defaultSpatialTypologyTogglesForModelDefinition(defaultModelDefinitionId()),
    filterPrimitiveToggles: defaultSpatialPrimitiveToggles(),
    selectionPrimitiveToggles: defaultSpatialPrimitiveToggles(),
    activeModelDefinitionId: defaultModelDefinitionId(),
    selectionMethod: "rectangle",
    modelDefinitionRevision: 0,
    dragSelection: null,
    selectionMenu: null,
    hoveredPickKey: null,
    rendererSelectionByModel: {},
    interactionSelectionByState: {},
    interactionMenuOpen: false,
    lastFinalizedInteractionId: "",
  };
}

export interface InteractionReplProps extends InteractionReplHostValues, InteractionReplHostCallbacks, InteractionReplLayoutProps {
  readonly interactionId: string;
  readonly spec: InteractionSpec;
  readonly onInteractionId: (id: string) => void;
  readonly runtime: InteractionRuntime;
  readonly history: DocumentHistory;
  readonly document: ModelDocument;
  readonly geometry: SpatialPickGeometry | null;
  readonly asideExtra?: ReactNode;
  readonly archivedBoxLayouts?: readonly ArchivedBoxLayout[];
  /** @emoji 🔁 When host bumps this positive counter for the same interaction, `cancel()` then `start` without remounting GL. */
  readonly sessionRestartNonce?: number;
  readonly viewTheme?: InteractionSpatialViewTheme;
  readonly viewSlots?: InteractionSpatialViewSlots;
  readonly renderDisplayItem?: SpatialDisplayItemRenderer;
  readonly autoFitMeshes?: boolean;
  readonly autoFitBehavior?: SpatialAutoFitBehavior;
  readonly tessellationTolerance?: number;
  /** @emoji 🎛 Toolbar gumball mode; hidden while an interaction session is active. */
  readonly transformGumballMode?: CadTransformGumballMode | null;
  readonly onTransformGumballCommit?: (diff: ModelDiff) => void;
}

/** @emoji 💬 One interaction a window can start from the floating panel while idle. */
export interface InteractionReplEngagementInteraction {
  readonly id: string;
  readonly key: string;
  readonly label: string;
}

/** @emoji 💬 Inputs for {@link buildInteractionReplEngagement} (interaction state + callbacks for the floating panel). */
export interface InteractionReplEngagementInputs {
  readonly showEngagement: boolean;
  readonly boundInteractionSession: boolean;
  readonly interactionId: string;
  readonly state: string;
  readonly lastResponseOk: boolean | null;
  readonly lastResponseErrorCount: number;
  readonly selectionCount: number;
  readonly cmdLine: string;
  readonly transitions: readonly InteractionKeybindRow[];
  readonly interactions: readonly InteractionReplEngagementInteraction[];
  readonly onTransition: (row: InteractionKeybindRow) => void;
  readonly onStartInteraction: (interactionId: string) => void;
  readonly onInputChange: (value: string) => void;
  readonly onInputSubmit: (value: string) => void;
}

/** @emoji 💬 Builds the compact window {@link EngagementSpec}: an active session lists its transitions as options, idle exposes a command input to start one, plus state/selection/response status. */
export function buildInteractionReplEngagement(inputs: InteractionReplEngagementInputs): EngagementSpec | null {
  if (!inputs.showEngagement) return null;
  const options = inputs.boundInteractionSession
    ? inputs.transitions.map((row) => ({
        id: `engagement-transition-${row.eventKind}-${row.key}`,
        label: `${row.key.toUpperCase()} ${row.label}`,
        onPress: () => inputs.onTransition(row),
      }))
    : [];
  const status: { id: string; content: ReactNode }[] = [];
  if (inputs.interactionId) status.push({ id: "engagement-state", content: `State: ${inputs.state}` });
  if (inputs.selectionCount > 0) status.push({ id: "engagement-selection", content: `Selected: ${inputs.selectionCount}` });
  if (inputs.lastResponseOk !== null) {
    status.push({
      id: "engagement-response",
      content: inputs.lastResponseOk ? "OK" : `Error${inputs.lastResponseErrorCount > 0 ? ` (${inputs.lastResponseErrorCount})` : ""}`,
    });
  }
  const input =
    inputs.boundInteractionSession || inputs.interactions.length > 0
      ? {
          id: "engagement-input",
          value: inputs.cmdLine,
          placeholder: inputs.boundInteractionSession ? "Type a transition or value" : "Type an interaction",
          onChange: inputs.onInputChange,
          onSubmit: inputs.onInputSubmit,
        }
      : undefined;
  const possibleEngagements = inputs.boundInteractionSession
    ? inputs.transitions.map((row) => ({
        id: `engagement-possible-transition-${row.eventKind}-${row.key}`,
        label: `${row.key.toUpperCase()} ${row.label}`,
        detail: row.eventKind,
        onSelect: () => inputs.onTransition(row),
      }))
    : inputs.interactions.map((interaction) => ({
        id: interaction.id,
        label: interaction.label,
        detail: interaction.key,
        onSelect: () => inputs.onStartInteraction(interaction.id),
      }));
  if (options.length === 0 && !input && status.length === 0 && possibleEngagements.length === 0) return null;
  return {
    options: options.length ? options : undefined,
    input,
    status: status.length ? status : undefined,
    possibleEngagements: possibleEngagements.length ? possibleEngagements : undefined,
  };
}

/** @emoji 🪩 Full spatial REPL: canvas, interaction palette, history controls, last response. */
export function InteractionRepl({
  interactionId,
  spec,
  onInteractionId,
  runtime: rt,
  history,
  document: documentModel,
  geometry,
  asideExtra,
  archivedBoxLayouts = [],
  sessionRestartNonce = 0,
  viewTheme,
  viewSlots,
  renderDisplayItem,
  autoFitMeshes = false,
  autoFitBehavior = "initial",
  tessellationTolerance,
  cmdLine: cmdLineProp,
  activeSuggestionIndex: activeSuggestionIndexProp,
  filterTypologyToggles: filterTypologyTogglesProp,
  selectionTypologyToggles: selectionTypologyTogglesProp,
  filterPrimitiveToggles: filterPrimitiveTogglesProp,
  selectionPrimitiveToggles: selectionPrimitiveTogglesProp,
  activeModelDefinitionId: activeModelDefinitionIdProp,
  selectionMethod: selectionMethodProp,
  modelDefinitionRevision: modelDefinitionRevisionProp,
  dragSelection: dragSelectionProp,
  selectionMenu: selectionMenuProp,
  hoveredPickKey: hoveredPickKeyProp,
  rendererSelectionByModel: rendererSelectionByModelProp,
  interactionSelectionByState: interactionSelectionByStateProp,
  interactionMenuOpen: interactionMenuOpenProp,
  lastFinalizedInteractionId: lastFinalizedInteractionIdProp,
  onCmdLineChange,
  onActiveSuggestionIndexChange,
  onFilterTypologyTogglesChange,
  onSelectionTypologyTogglesChange,
  onFilterPrimitiveTogglesChange,
  onSelectionPrimitiveTogglesChange,
  onActiveModelDefinitionIdChange,
  onSelectionMethodChange,
  onModelDefinitionRevisionChange,
  onDragSelectionChange,
  onSelectionMenuChange,
  onHoveredPickKeyChange,
  onRendererSelectionByModelChange,
  onInteractionSelectionByStateChange,
  onInteractionMenuOpenChange,
  onLastFinalizedInteractionIdChange,
  onApplyTransformation,
  pickGeometry: pickGeometryProp,
  onDocumentModelChange,
  onCanvasReady,
  onInteractionEvent: onInteractionEventProp,
  onGroundPick: onGroundPickProp,
  onScenePointerMove: onScenePointerMoveProp,
  onSelectionRequest: onSelectionRequestProp,
  onHoverTarget: onHoverTargetProp,
  onCameraNavigate: onCameraNavigateProp,
  onCommandSubmit,
  onTransitionRun,
  onCancel,
  onUndo,
  onRedo,
  onSnapshotChange,
  onEscape,
  onDeleteSelection,
  rootStyle,
  asideStyle,
  showAside = true,
  showEngagement = false,
  onEngagementChange,
  fillHost = false,
  captureGlobalKeys = true,
  asideHost = null,
  hideModelDefinitionControls = false,
  frameloop = "always",
  canvas: canvasOverrides,
  spatialView: spatialViewOverrides,
  transformGumballMode = null,
  onTransformGumballCommit,
}: InteractionReplProps): ReactNode {
  const snapshot = useInteractionSnapshot(rt);
  const rtRef = reactHostPort.useRef(rt);
  rtRef.current = rt;
  const tessTolerance = tessellationTolerance ?? (rt.computeMode() === "fast" ? 0.02 : 0.0008);
  const committedMeshes = useDocumentMeshes(rt.kernel(), documentModel.model, tessTolerance);
  const documentArchivedBoxLayouts = reactHostPort.useMemo(() => archivedBoxesFromHistory(history), [history, snapshot.revision]);
  const allArchivedBoxLayouts = reactHostPort.useMemo(() => [...documentArchivedBoxLayouts, ...archivedBoxLayouts], [documentArchivedBoxLayouts, archivedBoxLayouts]);
  const baseDisplay = reactHostPort.useMemo(
    () => filterFootprintBoxPreviewDisplayItems(replBaseDisplayForHistory(snapshot), documentModel.model),
    [snapshot, documentModel.model],
  );
  const mergedDisplay = reactHostPort.useMemo(
    () => mergeDisplayWithArchivedBoxes(baseDisplay, allArchivedBoxLayouts, documentModel.model),
    [baseDisplay, allArchivedBoxLayouts, documentModel.model],
  );
  const chromeDefaults = reactHostPort.useMemo(() => defaultInteractionReplChromeState(), []);
  const [cmdLine, setCmdLine] = useHostState(cmdLineProp, onCmdLineChange, () => chromeDefaults.cmdLine);
  const [activeIndex, setActiveIndex] = useHostState(activeSuggestionIndexProp, onActiveSuggestionIndexChange, () => chromeDefaults.activeSuggestionIndex);
  const [filterTypologyToggles, setFilterTypologyToggles] = useHostState(filterTypologyTogglesProp, onFilterTypologyTogglesChange, () => chromeDefaults.filterTypologyToggles);
  const [selectionTypologyToggles, setSelectionTypologyToggles] = useHostState(selectionTypologyTogglesProp, onSelectionTypologyTogglesChange, () => chromeDefaults.selectionTypologyToggles);
  const [filterPrimitiveToggles, setFilterPrimitiveToggles] = useHostState(filterPrimitiveTogglesProp, onFilterPrimitiveTogglesChange, () => chromeDefaults.filterPrimitiveToggles);
  const [selectionPrimitiveToggles, setSelectionPrimitiveToggles] = useHostState(selectionPrimitiveTogglesProp, onSelectionPrimitiveTogglesChange, () => chromeDefaults.selectionPrimitiveToggles);
  const [activeModelDefinitionId, setActiveModelDefinitionId] = useHostState(activeModelDefinitionIdProp, onActiveModelDefinitionIdChange, () => chromeDefaults.activeModelDefinitionId);
  const mdIdForView = activeModelDefinitionId ?? defaultModelDefinitionId();
  const committedMeshesForView = reactHostPort.useMemo(() => (modelDefinitionUsesGeometryPicking(mdIdForView) ? committedMeshes : []), [committedMeshes, mdIdForView]);
  const [selectionMethod, setSelectionMethod] = useHostState(selectionMethodProp, onSelectionMethodChange, () => chromeDefaults.selectionMethod);
  const [modelDefinitionRevision, setModelDefinitionRevision] = useHostState(modelDefinitionRevisionProp, onModelDefinitionRevisionChange, () => chromeDefaults.modelDefinitionRevision);
  const modelDefinitions = reactHostPort.useMemo(() => listModelDefinitionManifests(), []);
  const transfersFrom = reactHostPort.useMemo(() => listTransformationsIntoModelDefinition(activeModelDefinitionId ?? defaultModelDefinitionId()), [activeModelDefinitionId]);
  const transfersTo = reactHostPort.useMemo(() => listTransformationsFromModelDefinition(activeModelDefinitionId ?? defaultModelDefinitionId()), [activeModelDefinitionId]);
  const modelDefinitionScope = reactHostPort.useMemo(() => resolveModelDefinitionScope(activeModelDefinitionId ?? defaultModelDefinitionId()), [activeModelDefinitionId]);
  const scopedInteractions = reactHostPort.useMemo(() => listSpatialInteractionsForModelDefinition(activeModelDefinitionId ?? defaultModelDefinitionId()), [activeModelDefinitionId, modelDefinitionRevision]);
  const kernel = rt.kernel();
  const [dragSelection, setDragSelection] = useHostState(dragSelectionProp, onDragSelectionChange, () => chromeDefaults.dragSelection);
  const [selectionMenu, setSelectionMenu] = useHostState(selectionMenuProp, onSelectionMenuChange, () => chromeDefaults.selectionMenu);
  const [hoveredPickKey, setHoveredPickKey] = useHostState(hoveredPickKeyProp, onHoveredPickKeyChange, () => chromeDefaults.hoveredPickKey);
  const [rendererSelectionByModel, setRendererSelectionByModel] = useHostState(rendererSelectionByModelProp, onRendererSelectionByModelChange, () => ({ ...chromeDefaults.rendererSelectionByModel }));
  const [interactionSelectionByState, setInteractionSelectionByState] = useHostState(interactionSelectionByStateProp, onInteractionSelectionByStateChange, () => ({ ...chromeDefaults.interactionSelectionByState }));
  const [interactionMenuOpen, setInteractionMenuOpen] = useHostState(interactionMenuOpenProp, onInteractionMenuOpenChange, () => chromeDefaults.interactionMenuOpen);
  const [lastFinalizedInteractionId, setLastFinalizedInteractionId] = useHostState(lastFinalizedInteractionIdProp, onLastFinalizedInteractionIdChange, () => chromeDefaults.lastFinalizedInteractionId);
  const [canvasBinding, setCanvasBinding] = reactHostPort.useState<{ readonly camera: THREE.Camera; readonly domElement: HTMLCanvasElement } | null>(null);
  const handleCanvasReady = reactHostPort.useCallback(
    (binding: { readonly camera: THREE.Camera; readonly domElement: HTMLCanvasElement }) => {
      setCanvasBinding(binding);
      onCanvasReady?.(binding);
    },
    [onCanvasReady],
  );
  reactHostPort.useEffect(() => {
    onSnapshotChange?.(snapshot);
  }, [snapshot, onSnapshotChange]);
  const cmdRef = reactHostPort.useRef<HTMLInputElement>(null);
  const numericEntryPrevStateRef = reactHostPort.useRef(snapshot.state);
  const setCmdLineRef = reactHostPort.useRef(setCmdLine);
  const rendererSelectionByModelRef = reactHostPort.useRef(rendererSelectionByModel);
  const suppressAutoStartOnceRef = reactHostPort.useRef(false);
  const lastViewsRefreshRef = reactHostPort.useRef<{ readonly model: Model | null; readonly revision: number; readonly activeModelDefinitionId: string | null }>({
    model: null,
    revision: -1,
    activeModelDefinitionId: null,
  });
  const dragSelectionRef = reactHostPort.useRef<SpatialDragSelectionState | null>(null);
  const dragCleanupRef = reactHostPort.useRef<(() => void) | null>(null);
  const cameraNavigatingRef = reactHostPort.useRef(false);
  const interactionActive = isInteractionSessionActive(spec, snapshot.state);
  const boundInteractionSession = Boolean(interactionId) && interactionActive;
  const activeTransformGumballMode = !boundInteractionSession && transformGumballMode ? transformGumballMode : null;
  const displayedSelectionTargets = reactHostPort.useMemo(
    () => replDisplayedSelectionTargets(boundInteractionSession, activeModelDefinitionId, snapshot.state, rendererSelectionByModel, interactionSelectionByState),
    [boundInteractionSession, activeModelDefinitionId, snapshot.state, rendererSelectionByModel, interactionSelectionByState],
  );
  const selectedPickKeys = reactHostPort.useMemo(() => {
    const keys = new Set(displayedSelectionTargets.map(spatialSelectionTargetKey));
    return pinnedPickTargetKeys(keys);
  }, [displayedSelectionTargets]);
  const selectedPickKey = displayedSelectionTargets[0] ? spatialSelectionTargetKey(displayedSelectionTargets[0]) : null;
  const selectionInvalidateKey = reactHostPort.useMemo(() => [...selectedPickKeys].sort().join("\0"), [selectedPickKeys]);
  const geometryPreviewTransform = reactHostPort.useMemo(() => geometryPreviewTransformFromDisplay(mergedDisplay), [mergedDisplay]);
  const pickSourceGeometry = geometry ?? pickGeometryProp;
  const pickSourceRevision = pickSourceGeometry && typeof pickSourceGeometry === "object" && "revision" in pickSourceGeometry ? Number((pickSourceGeometry as { revision?: unknown }).revision) : 0;
  const pickTargets = reactHostPort.useMemo(() => createSpatialPickTargets(pickSourceGeometry, activeModelDefinitionId), [pickSourceGeometry, pickSourceRevision, modelDefinitionRevision, activeModelDefinitionId]);
  const activeTypologyIds = reactHostPort.useMemo(() => modelDefinitionTypologyIds(activeModelDefinitionId), [activeModelDefinitionId]);
  const scopedPickTargets = reactHostPort.useMemo(() => filterSpatialPickTargetsForActiveView(pickTargets, activeModelDefinitionId), [pickTargets, activeModelDefinitionId]);
  const visiblePickTargets = reactHostPort.useMemo(() => {
    const showPrimitives = filterSpatialPickTargetsForPrimitiveToggles(scopedPickTargets, filterPrimitiveToggles);
    return filterSpatialPickTargetsForTypologyToggles(showPrimitives, filterTypologyToggles, activeTypologyIds);
  }, [scopedPickTargets, filterPrimitiveToggles, filterTypologyToggles, activeTypologyIds]);
  const viewFilterKindToggles = reactHostPort.useMemo(() => spatialPickKindTogglesFromTypologyFilteredTargets(activeModelDefinitionId, visiblePickTargets), [activeModelDefinitionId, visiblePickTargets]);
  const selectablePickTargets = reactHostPort.useMemo(() => {
    const filterPrimitives = filterSpatialPickTargetsForPrimitiveToggles(visiblePickTargets, selectionPrimitiveToggles);
    return filterSpatialPickTargetsForTypologyToggles(filterPrimitives, selectionTypologyToggles, activeTypologyIds);
  }, [visiblePickTargets, selectionPrimitiveToggles, selectionTypologyToggles, activeTypologyIds]);
  const effectiveSelectionKindToggles = reactHostPort.useMemo(
    () => intersectSpatialPickKindToggles(viewFilterKindToggles, spatialPickKindTogglesFromTypologyFilteredTargets(activeModelDefinitionId, selectablePickTargets)),
    [activeModelDefinitionId, selectablePickTargets, viewFilterKindToggles],
  );
  const scopeTypologyIds = reactHostPort.useMemo(() => modelDefinitionScope.typologies.map((row) => row.id), [modelDefinitionScope.typologies]);
  const primitiveShowGroupState = reactHostPort.useMemo(() => spatialToggleGroupState(SPATIAL_PRIMITIVE_KINDS, filterPrimitiveToggles), [filterPrimitiveToggles]);
  const primitiveFilterGroupState = reactHostPort.useMemo(() => spatialToggleGroupState(SPATIAL_PRIMITIVE_KINDS, selectionPrimitiveToggles), [selectionPrimitiveToggles]);
  const typologyShowGroupState = reactHostPort.useMemo(() => spatialToggleGroupState(scopeTypologyIds, filterTypologyToggles), [scopeTypologyIds, filterTypologyToggles]);
  const typologySelectionGroupState = reactHostPort.useMemo(() => spatialToggleGroupState(scopeTypologyIds, selectionTypologyToggles), [scopeTypologyIds, selectionTypologyToggles]);
  reactHostPort.useEffect(() => {
    setCmdLineRef.current = setCmdLine;
  }, [setCmdLine]);

  reactHostPort.useEffect(() => {
    rendererSelectionByModelRef.current = rendererSelectionByModel;
  }, [rendererSelectionByModel]);

  const dismissReplChrome = reactHostPort.useCallback(() => {
    dragCleanupRef.current?.();
    dragCleanupRef.current = null;
    dragSelectionRef.current = null;
    setDragSelection(null);
    setCmdLine("");
    setSelectionMenu(null);
    setHoveredPickKey(null);
    setInteractionMenuOpen(false);
  }, []);

  const cancelActiveInteraction = reactHostPort.useCallback(() => {
    const aborted = abortActiveInteractionSession(rt);
    if (!aborted && !interactionId) return false;
    if (!aborted) rt.cancel();
    suppressAutoStartOnceRef.current = true;
    setInteractionSelectionByState({});
    dismissReplChrome();
    if (interactionId) onInteractionId("");
    onCancel?.();
    return true;
  }, [rt, interactionId, onInteractionId, dismissReplChrome, onCancel, setInteractionSelectionByState]);

  reactHostPort.useEffect(() => {
    if (!interactionId || !snapshot.lastResponse?.ok) return;
    setLastFinalizedInteractionId(interactionId);
    setRendererSelectionByModel((prev) => replFinalizeSelection(prev, activeModelDefinitionId, snapshot.lastResponse));
    setInteractionSelectionByState((prev) => (Object.keys(prev).length === 0 ? prev : {}));
    setCmdLine("");
  }, [interactionId, snapshot.lastResponse, activeModelDefinitionId, setInteractionSelectionByState, setLastFinalizedInteractionId, setRendererSelectionByModel, setCmdLine]);

  reactHostPort.useEffect(() => {
    if (!interactionId || !isFinalInteractionState(spec, snapshot.state)) return;
    setCmdLine("");
  }, [interactionId, spec, snapshot.state, setCmdLine]);

  const handleEscapeKey = reactHostPort.useCallback(() => {
    if (selectionMenu !== null) {
      setSelectionMenu(null);
      setHoveredPickKey(null);
      onEscape?.();
      return;
    }
    switch (replEscapeAction({ hasInteraction: Boolean(interactionId), interactionActive, cmdLine, hasSelectionMenu: selectionMenu !== null })) {
      case "abort":
        cancelActiveInteraction();
        onEscape?.();
        return;
      case "dismiss":
        dismissReplChrome();
        onEscape?.();
        return;
      default:
        return;
    }
  }, [interactionId, interactionActive, cmdLine, selectionMenu, dismissReplChrome, cancelActiveInteraction, onEscape, setSelectionMenu, setHoveredPickKey]);

  const startRuntime = reactHostPort.useCallback(async () => {
    const accept = rt.listActiveSelectionAccept() as readonly ModelEntityKind[];
    const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
    const accepted = replSelectionAccepted(accept, replRendererSelectionTargets(rendererSelectionByModelRef.current, mdId));
    setInteractionSelectionByState({ [rt.getSnapshot().state]: [...accepted] });
    await rt.send(replStartEvent(accepted));
  }, [rt, activeModelDefinitionId, setInteractionSelectionByState]);

  reactHostPort.useEffect(() => {
    if (!interactionId) return;
    if (suppressAutoStartOnceRef.current) {
      suppressAutoStartOnceRef.current = false;
      return;
    }
    void startRuntime();
  }, [interactionId, startRuntime]);

  reactHostPort.useEffect(() => {
    if (sessionRestartNonce <= 0) return;
    rt.cancel();
    void startRuntime();
  }, [sessionRestartNonce, rt, startRuntime]);

  const repeatCurrentInteraction = reactHostPort.useCallback(() => {
    rt.cancel();
    void startRuntime();
  }, [rt, startRuntime]);

  const modelRevision = documentModel.model.revision;
  const hostPickingEnabled = replHostGeometryPickingEnabled(interactionId, spec, snapshot.state);
  const showPickLayer = replGeometryPickLayerVisible(mdIdForView);

  reactHostPort.useEffect(() => {
    setSelectionMenu(null);
    setHoveredPickKey(null);
  }, [geometry, snapshot.state, modelDefinitionRevision]);

  reactHostPort.useEffect(() => {
    const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
    const typologyDefaults = defaultSpatialTypologyTogglesForModelDefinition(mdId);
    const primitiveDefaults = defaultSpatialPrimitiveToggles();
    setFilterTypologyToggles(typologyDefaults);
    setSelectionTypologyToggles(typologyDefaults);
    setFilterPrimitiveToggles(primitiveDefaults);
    setSelectionPrimitiveToggles(primitiveDefaults);
    const allowed = listSpatialInteractionsForModelDefinition(mdId);
    if (interactionId && !allowed.some((row) => row.id === interactionId)) onInteractionId("");
    setLastFinalizedInteractionId("");
  }, [activeModelDefinitionId, modelDefinitionRevision, interactionId, onInteractionId, setFilterTypologyToggles, setSelectionTypologyToggles, setFilterPrimitiveToggles, setSelectionPrimitiveToggles, setLastFinalizedInteractionId]);

  reactHostPort.useEffect(() => {
    setCmdLine("");
    setActiveIndex(0);
    setSelectionMenu(null);
    setHoveredPickKey(null);
    setInteractionMenuOpen(false);
    setInteractionSelectionByState((prev) => (Object.keys(prev).length === 0 ? prev : {}));
  }, [interactionId, rt, setInteractionSelectionByState]);

  const confirmInteractionSelection = reactHostPort.useCallback(() => {
    const snap = rt.getSnapshot();
    if (!interactionCanConfirmSelection(spec, snap.state, snap.context, scenePreview())) return false;
    void rt.send({ kind: "confirm", modifiers: {} });
    return true;
  }, [rt, spec]);

  reactHostPort.useEffect(() => {
    if (!interactionId || !interactionActive) {
      setInteractionSelectionByState((prev) => (Object.keys(prev).length === 0 ? prev : {}));
      return;
    }
    const stateId = snapshot.state;
    const machineTargets = replInteractionSelectionFromContext(snapshot.context);
    setInteractionSelectionByState((prev) => {
      const current = prev[stateId] ?? [];
      if (hostPickingEnabled && machineTargets.length === 0 && current.length > 0) {
        return prev;
      }
      if (replSelectionTargetsEqual(current, machineTargets)) return prev;
      return replWithInteractionSelectionTargets(prev, stateId, machineTargets);
    });
  }, [interactionId, interactionActive, hostPickingEnabled, snapshot.revision, snapshot.state, snapshot.context, setInteractionSelectionByState]);

  const runtimeSelectionAccept = reactHostPort.useMemo(() => rt.listActiveSelectionAccept(), [rt, snapshot.state]);
  const defaultSelectionAccept = reactHostPort.useMemo(() => modelDefinitionSelectionEntityKinds(activeModelDefinitionId ?? defaultModelDefinitionId()), [activeModelDefinitionId]);
  const activeSelectionAccept = reactHostPort.useMemo((): readonly ModelEntityKind[] => {
    if (runtimeSelectionAccept.length > 0) {
      const allowed = new Set(defaultSelectionAccept);
      return runtimeSelectionAccept.filter((kind) => allowed.has(kind));
    }
    if (boundInteractionSession && runtimeSelectionAccept.length === 0) return [];
    return defaultSelectionAccept;
  }, [runtimeSelectionAccept, boundInteractionSession, defaultSelectionAccept]);
  const viewObjectCount = reactHostPort.useMemo(() => {
    if (isShapeModelDefinition(activeModelDefinitionId)) return 0;
    return countViewObjectsForModelDefinition(documentModel.model, mdIdForView);
  }, [activeModelDefinitionId, documentModel.model, mdIdForView, modelDefinitionRevision]);

  const commitSelection = reactHostPort.useCallback(
    (selection: readonly SelectionTarget[]) => {
      setSelectionMenu(null);
      setHoveredPickKey(null);
      const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
      if (boundInteractionSession) {
        setInteractionSelectionByState((prev) => replWithInteractionSelectionTargets(prev, snapshot.state, selection));
      } else {
        setRendererSelectionByModel((prev) => replWithRendererSelectionTargets(prev, mdId, selection));
      }
    },
    [boundInteractionSession, activeModelDefinitionId, snapshot.state, setInteractionSelectionByState, setRendererSelectionByModel, setSelectionMenu, setHoveredPickKey],
  );

  const applySelectionPrune = reactHostPort.useCallback(
    (map: (selection: readonly SelectionTarget[]) => readonly SelectionTarget[]) => {
      const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
      setRendererSelectionByModel((prev) => replWithRendererSelectionTargets(prev, mdId, map(replRendererSelectionTargets(prev, mdId))));
      setInteractionSelectionByState((prev) => {
        let next: SpatialInteractionSelectionByState = prev;
        for (const stateId of Object.keys(prev)) {
          const pruned = map(prev[stateId] ?? []);
          next = replWithInteractionSelectionTargets(next, stateId, pruned);
        }
        return next;
      });
    },
    [activeModelDefinitionId, setRendererSelectionByModel, setInteractionSelectionByState],
  );

  const dispatchSelectionTargets = reactHostPort.useCallback(
    (targets: readonly SpatialPickTarget[], modifiers: InteractionEvent["modifiers"] = {}, point?: Vec3) => {
      const picked = uniqueSelectionTargets(targets.map(spatialSelectionTarget));
      const nextSelection = replMergeSelectionPickInView(boundInteractionSession, activeModelDefinitionId, snapshot.state, rendererSelectionByModel, interactionSelectionByState, picked, modifiers);
      commitSelection(nextSelection);
      if (boundInteractionSession && picked.length > 0) void rt.send({ ...replSelectionEvent(picked, point), modifiers });
    },
    [commitSelection, boundInteractionSession, interactionSelectionByState, activeModelDefinitionId, snapshot.state, rt, rendererSelectionByModel],
  );

  const onSelectionRequest = reactHostPort.useCallback(
    (request: SpatialSelectionRequest) => {
      onSelectionRequestProp?.(request);
      if (request.targets.length === 1) {
        dispatchSelectionTargets([request.targets[0]!], request.modifiers, request.point);
        return;
      }
      setSelectionMenu(request);
      setHoveredPickKey(request.targets[0] ? spatialPickTargetKey(request.targets[0]) : null);
    },
    [dispatchSelectionTargets, onSelectionRequestProp, setSelectionMenu, setHoveredPickKey],
  );

  const onHoverTarget = reactHostPort.useCallback(
    (target: SpatialPickTarget | null) => {
      const key = target ? spatialPickTargetKey(target) : null;
      setHoveredPickKey((prev) => (prev === key ? prev : key));
      onHoverTargetProp?.(target);
    },
    [onHoverTargetProp, setHoveredPickKey],
  );

  const onCameraNavigate = reactHostPort.useCallback(
    (active: boolean) => {
      cameraNavigatingRef.current = active;
      if (active) onHoverTarget(null);
      onCameraNavigateProp?.(active);
    },
    [onHoverTarget, onCameraNavigateProp],
  );

  reactHostPort.useEffect(() => {
    const canvas = canvasBinding?.domElement;
    const camera = canvasBinding?.camera;
    if (!canvas || !camera || !hostPickingEnabled) return;
    let lastHoverAt = 0;
    const onMove = (event: PointerEvent) => {
      if (cameraNavigatingRef.current || event.buttons !== 0) {
        onHoverTarget(null);
        return;
      }
      const now = performance.now();
      if (now - lastHoverAt < 32) return;
      lastHoverAt = now;
      const rect = canvas.getBoundingClientRect();
      const hits = spatialPickTargetsFromClientPoint({ x: event.clientX, y: event.clientY }, camera, rect, selectablePickTargets, [], {});
      onHoverTarget(hits[0] ?? null);
    };
    const onLeave = () => onHoverTarget(null);
    canvas.addEventListener("pointermove", onMove, { passive: true });
    canvas.addEventListener("pointerleave", onLeave, { passive: true });
    return () => {
      canvas.removeEventListener("pointermove", onMove);
      canvas.removeEventListener("pointerleave", onLeave);
    };
  }, [canvasBinding, hostPickingEnabled, selectablePickTargets, onHoverTarget]);

  const pointerMoveActive = reactHostPort.useMemo(() => {
    const si = snapshot.spatialInteraction;
    return si.spatialGroundPick && (si.groundPointerMoveStates.includes(snapshot.state) || si.heightDragStates.includes(snapshot.state) || si.verticalRodStates.includes(snapshot.state));
  }, [snapshot.state, snapshot.spatialInteraction]);

  const onSpatialInteractionEvent = reactHostPort.useCallback(
    (ev: InteractionEvent) => {
      onInteractionEventProp?.(ev);
      const activeRt = rtRef.current;
      if (ev.kind === "pointer.down") {
        const st = activeRt.getSnapshot().state;
        const hi = activeRt.getSnapshot().spatialInteraction.heightConfirmState;
        const snapEv = (ev as { snap?: { kind: string; id: string } }).snap;
        if (hi && st === hi && !snapEv) {
          void activeRt.send({ kind: "confirm", modifiers: (ev as { modifiers?: Record<string, unknown> }).modifiers ?? {} });
          return;
        }
        if (snapEv && activeSelectionAccept.length > 0 && activeSelectionAccept.includes(snapEv.kind as ModelEntityKind) && effectiveSelectionKindToggles[snapEv.kind as SpatialPickTargetKind] !== false) {
          const snapTarget: SpatialPickTarget = {
            kind: snapEv.kind as SpatialPickTargetKind,
            id: snapEv.id,
            point: (ev as { point?: Vec3 }).point ?? [0, 0, 0],
          };
          const selection = spatialSelectionTarget(snapTarget);
          const modifiers = (ev as { modifiers?: InteractionEvent["modifiers"] }).modifiers ?? {};
          commitSelection(replMergeSelectionPickInView(boundInteractionSession, activeModelDefinitionId, snapshot.state, rendererSelectionByModel, interactionSelectionByState, [selection], modifiers));
          if (boundInteractionSession) void activeRt.send({ ...replSelectionEvent([selection], (ev as { point?: Vec3 }).point), modifiers });
          return;
        }
      }
      if (ev.kind === "pointer.move" && !pointerMoveActive) return;
      if (ev.kind === "pointer.down" || ev.kind === "pointer.move" || ev.kind === "contextmenu") void activeRt.send(ev);
    },
    [activeSelectionAccept, commitSelection, boundInteractionSession, interactionSelectionByState, rendererSelectionByModel, snapshot.state, pointerMoveActive, activeModelDefinitionId, effectiveSelectionKindToggles, onInteractionEventProp],
  );

  reactHostPort.useEffect(() => {
    const canvas = canvasBinding?.domElement;
    const camera = canvasBinding?.camera;
    if (!canvas || !camera || !hostPickingEnabled || activeSelectionAccept.length === 0) return;
    const clearDragSelection = () => {
      dragCleanupRef.current = null;
      dragSelectionRef.current = null;
      setDragSelection(null);
    };
    const beginDragSelection = (event: PointerEvent) => {
      if (event.button !== 0) return;
      dragCleanupRef.current?.();
      const rect = canvas.getBoundingClientRect();
      const startClient = { x: event.clientX, y: event.clientY };
      const initial: SpatialDragSelectionState = {
        method: selectionMethod,
        coverage: "full",
        startClient,
        currentClient: startClient,
        path: [startClient],
        modifiers: pointerModifiersFromNativeEvent(event),
      };
      dragSelectionRef.current = initial;
      const moveSelection = (moveEvent: PointerEvent) => {
        const current = dragSelectionRef.current;
        if (!current) return;
        const nextClient = { x: moveEvent.clientX, y: moveEvent.clientY };
        const nextPath = current.method === "lasso" && dragDistance(current.path[current.path.length - 1]!, nextClient) >= 2 ? [...current.path, nextClient] : current.method === "lasso" ? current.path : [current.startClient, nextClient];
        const nextState: SpatialDragSelectionState = {
          ...current,
          currentClient: nextClient,
          path: nextPath,
          coverage: spatialSelectionCoverageFromPath(nextPath),
          modifiers: pointerModifiersFromNativeEvent(moveEvent),
        };
        dragSelectionRef.current = nextState;
        if (dragDistance(nextState.startClient, nextClient) >= 4) setDragSelection(nextState);
      };
      const finishSelection = (upEvent: PointerEvent) => {
        window.removeEventListener("pointermove", moveSelection, true);
        window.removeEventListener("pointerup", finishSelection, true);
        const current = dragSelectionRef.current;
        clearDragSelection();
        if (!current) return;
        const finalState: SpatialDragSelectionState = {
          ...current,
          currentClient: { x: upEvent.clientX, y: upEvent.clientY },
          path: current.method === "lasso" ? [...current.path, { x: upEvent.clientX, y: upEvent.clientY }] : [current.startClient, { x: upEvent.clientX, y: upEvent.clientY }],
          modifiers: pointerModifiersFromNativeEvent(upEvent),
        };
        const distance = dragDistance(finalState.startClient, finalState.currentClient);
        const clearSelectionOnEmptyBackgroundPick = () => {
          if (spatialSelectionModeFromModifiers(finalState.modifiers as { readonly alt?: boolean; readonly ctrl?: boolean; readonly meta?: boolean; readonly shift?: boolean }) === "default") {
            commitSelection(replMergeSelectionPickInView(boundInteractionSession, activeModelDefinitionId, snapshot.state, rendererSelectionByModel, interactionSelectionByState, [], finalState.modifiers));
          }
        };
        if (distance < 4) {
          const candidates = spatialPickTargetsFromClientPoint(finalState.currentClient, camera, rect, selectablePickTargets, activeSelectionAccept, {});
          if (candidates.length === 0) {
            clearSelectionOnEmptyBackgroundPick();
            return;
          }
          onSelectionRequest({
            targets: candidates,
            point: candidates[0]!.point,
            client: finalState.currentClient,
            modifiers: finalState.modifiers,
          });
          return;
        }
        const targets = spatialPickTargetsFromScreenSelection(
          { ...finalState, coverage: spatialSelectionCoverageFromPath(finalState.path) },
          selectablePickTargets,
          camera,
          canvas.getBoundingClientRect(),
          activeSelectionAccept,
          {},
          geometryPreviewTransform,
        );
        if (targets.length === 0) {
          clearSelectionOnEmptyBackgroundPick();
          return;
        }
        dispatchSelectionTargets(targets, finalState.modifiers);
      };
      dragCleanupRef.current = () => {
        window.removeEventListener("pointermove", moveSelection, true);
        window.removeEventListener("pointerup", finishSelection, true);
        clearDragSelection();
      };
      window.addEventListener("pointermove", moveSelection, true);
      window.addEventListener("pointerup", finishSelection, true);
    };
    canvas.addEventListener("pointerdown", beginDragSelection, true);
    return () => {
      dragCleanupRef.current?.();
      canvas.removeEventListener("pointerdown", beginDragSelection, true);
    };
  }, [
    activeSelectionAccept,
    canvasBinding,
    commitSelection,
    dispatchSelectionTargets,
    boundInteractionSession,
    interactionSelectionByState,
    onSelectionRequest,
    activeModelDefinitionId,
    snapshot.state,
    rendererSelectionByModel,
    selectionMethod,
    geometryPreviewTransform,
    selectablePickTargets,
    hostPickingEnabled,
  ]);

  const dispatchTransition = reactHostPort.useCallback(
    (row: InteractionKeybindRow) => {
      onTransitionRun?.(row);
      const ev = replBuildDispatchEvent(row, { interactionId: spec.id, model: documentModel.model });
      if (ev) void rtRef.current.send(ev);
    },
    [spec.id, documentModel.model, onTransitionRun],
  );

  const transitionRows = reactHostPort.useMemo(() => listKeyedInteractionTransitions(spec, snapshot.state), [spec, snapshot.state]);

  const allSuggestions = reactHostPort.useMemo((): ReplSuggestion[] => {
    const out: ReplSuggestion[] = [];
    for (const p of scopedInteractions) {
      out.push({
        kind: "interaction",
        key: p.key,
        label: p.label,
        detail: p.id,
        interactionId: p.id,
        onRun: () => onInteractionId(p.id),
      });
    }
    for (const row of transitionRows) {
      out.push({
        kind: "transition",
        key: row.key,
        label: row.label,
        detail: row.eventKind,
        transition: row,
        onRun: () => dispatchTransition(row),
      });
    }
    for (const defn of modelDefinitionScope.selectionOperations) {
      out.push({
        kind: "selection",
        key: defn.key,
        label: defn.label,
        detail: defn.id,
        onRun: () => {
          void rt.query(`CALL ${defn.id}({}) YIELD data.targets AS targets`);
        },
      });
    }
    for (const actionId of modelDefinitionScope.actions) {
      if (actionId.startsWith("selection.")) continue;
      const tail = actionId.includes(".") ? actionId.slice(actionId.lastIndexOf(".") + 1) : actionId;
      out.push({
        kind: "action",
        key: tail,
        label: actionId,
        detail: "action",
        onRun: () => {
          void rt.query(`CALL ${actionId}({})`);
        },
      });
    }
    return out;
  }, [scopedInteractions, transitionRows, modelDefinitionScope, onInteractionId, dispatchTransition, rt]);

  const filtered = reactHostPort.useMemo(() => replPaletteRows(cmdLine, allSuggestions), [cmdLine, allSuggestions]);
  const interactionMatches = reactHostPort.useMemo(() => replInteractionSuggestions(cmdLine, allSuggestions), [cmdLine, allSuggestions]);
  const completionSuffix = reactHostPort.useMemo(() => replActiveCompletionSuffix(cmdLine, filtered, activeIndex), [cmdLine, filtered, activeIndex]);

  reactHostPort.useEffect(() => {
    setActiveIndex((i) => (filtered.length ? Math.min(i, filtered.length - 1) : 0));
  }, [filtered.length, cmdLine]);

  const runSuggestion = reactHostPort.useCallback((s: ReplSuggestion) => {
    s.onRun();
    setCmdLine("");
    setActiveIndex(0);
    setInteractionMenuOpen(false);
  }, []);

  const runInteractionIdFromSpace = reactHostPort.useCallback(
    (id: string | null): boolean => {
      if (!id) return false;
      onInteractionId(id);
      setCmdLine("");
      setActiveIndex(0);
      setInteractionMenuOpen(false);
      return true;
    },
    [onInteractionId],
  );

  const replCmdLineValue = reactHostPort.useCallback((): string => cmdRef.current?.value ?? cmdLine, [cmdLine]);

  const tryCommitNumericEntry = reactHostPort.useCallback(async (): Promise<boolean> => {
    const snap = rt.getSnapshot();
    const state = snap.state;
    if (!interactionInNumericEntryState(spec, state)) return false;
    const parsed = parseNumericCommandLine(replCmdLineValue());
    const value = parsed !== null && parsed !== undefined ? parsed : interactionNumericEntryLockedValue(spec, state, snap.context);
    if (value == null) return false;
    const applyEv = interactionNumericEntryApplyEvent(spec, state, value);
    if (applyEv) await rt.send(applyEv);
    const after = rt.getSnapshot();
    const commitEv = interactionNumericEntryCommitEvent(spec, after.state, after.context);
    if (!commitEv) return false;
    await rt.send(commitEv);
    setCmdLine("");
    setInteractionMenuOpen(false);
    return true;
  }, [replCmdLineValue, rt, spec, setCmdLine]);

  const trySubmitLine = reactHostPort.useCallback((): boolean => {
    const raw = cmdLine.trim();
    if (!raw) return false;
    if (onCommandSubmit?.(raw)) {
      setCmdLine("");
      return true;
    }
    const valEv = replTryParseValueInteraction(raw, spec, rt.getSnapshot().state);
    if (valEv) {
      void rt.send(valEv);
      setCmdLine("");
      return true;
    }
    const interactionHit = resolveScopedSpatialInteractionKey(raw, activeModelDefinitionId ?? defaultModelDefinitionId());
    if (interactionHit) {
      onInteractionId(interactionHit.id);
      setCmdLine("");
      return true;
    }
    const rows = listKeyedInteractionTransitions(spec, rt.getSnapshot().state);
    for (const row of rows) {
      if (row.eventKind === "set.height" || row.eventKind === "set.distance" || row.eventKind === "set.footprint") continue;
      if (row.key === raw || row.key.toLowerCase() === raw.toLowerCase() || row.eventKind.toLowerCase() === raw.toLowerCase()) {
        dispatchTransition(row);
        setCmdLine("");
        return true;
      }
    }
    return false;
  }, [cmdLine, spec, rt, dispatchTransition, onInteractionId, onCommandSubmit, setCmdLine, activeModelDefinitionId]);

  const runTransitionRow = reactHostPort.useCallback(
    (row: InteractionKeybindRow) => {
      if (row.eventKind.startsWith("set.")) {
        setCmdLine(row.key);
        window.setTimeout(() => cmdRef.current?.focus(), 0);
        return;
      }
      dispatchTransition(row);
    },
    [dispatchTransition],
  );

  const onInputKeyDown = reactHostPort.useCallback(
    (e: KeyboardEvent<HTMLInputElement>) => {
      if (e.key === "Escape") {
        e.preventDefault();
        handleEscapeKey();
        return;
      }
      if (e.key === " " && !e.ctrlKey && !e.metaKey && !e.altKey) {
        e.preventDefault();
        if (interactionInNumericEntryState(spec, rt.getSnapshot().state)) {
          const snap = rt.getSnapshot();
          const parsed = parseNumericCommandLine(replCmdLineValue());
          if (parsed !== undefined && (parsed !== null || interactionNumericEntryLockedValue(spec, snap.state, snap.context) != null)) {
            void tryCommitNumericEntry();
            return;
          }
        }
        const interactionIdOnSpace = replInteractionIdOnSpace(cmdLine, filtered, allSuggestions, lastFinalizedInteractionId);
        if (runInteractionIdFromSpace(interactionIdOnSpace)) return;
        setInteractionMenuOpen(false);
        return;
      }
      if (e.key === "ArrowDown" && filtered.length) {
        e.preventDefault();
        setInteractionMenuOpen(false);
        setActiveIndex((i) => (i + 1) % filtered.length);
        return;
      }
      if (e.key === "ArrowUp" && filtered.length) {
        e.preventDefault();
        setInteractionMenuOpen(false);
        setActiveIndex((i) => (i - 1 + filtered.length) % filtered.length);
        return;
      }
      if (e.key === "Tab" && filtered.length) {
        e.preventDefault();
        const suffix = replActiveCompletionSuffix(cmdLine, filtered, activeIndex);
        if (suffix) {
          setCmdLine(replCommandTextWithoutSpaces(cmdLine + suffix));
          return;
        }
        runSuggestion(filtered[activeIndex] ?? filtered[0]!);
        return;
      }
      if (e.key === "Enter") {
        e.preventDefault();
        setInteractionMenuOpen(false);
        if (!cmdLine.trim() && confirmInteractionSelection()) return;
        if (interactionInNumericEntryState(spec, rt.getSnapshot().state)) {
          void tryCommitNumericEntry();
          return;
        }
        if (trySubmitLine()) return;
        if (filtered.length) runSuggestion(filtered[activeIndex]!);
        return;
      }
    },
    [cmdLine, allSuggestions, filtered, activeIndex, runSuggestion, trySubmitLine, tryCommitNumericEntry, replCmdLineValue, handleEscapeKey, lastFinalizedInteractionId, runInteractionIdFromSpace, confirmInteractionSelection, spec, rt],
  );

  const submitEngagementLine = reactHostPort.useCallback(() => {
    setInteractionMenuOpen(false);
    if (!cmdLine.trim() && confirmInteractionSelection()) return;
    if (interactionInNumericEntryState(spec, rt.getSnapshot().state)) {
      void tryCommitNumericEntry();
      return;
    }
    if (trySubmitLine()) return;
    if (filtered.length) runSuggestion(filtered[activeIndex] ?? filtered[0]!);
  }, [cmdLine, confirmInteractionSelection, spec, rt, tryCommitNumericEntry, trySubmitLine, filtered, runSuggestion, activeIndex]);

  const engagementSpec = reactHostPort.useMemo<EngagementSpec | null>(
    () =>
      buildInteractionReplEngagement({
        showEngagement,
        boundInteractionSession,
        interactionId,
        state: snapshot.state,
        lastResponseOk: snapshot.lastResponse ? snapshot.lastResponse.ok : null,
        lastResponseErrorCount: snapshot.lastResponse?.errors?.length ?? 0,
        selectionCount: displayedSelectionTargets.length,
        cmdLine,
        transitions: transitionRows,
        interactions: scopedInteractions,
        onTransition: runTransitionRow,
        onStartInteraction: (id: string) => onInteractionId(id),
        onInputChange: (value: string) => setCmdLine(replCommandTextWithoutSpaces(value)),
        onInputSubmit: () => submitEngagementLine(),
      }),
    [showEngagement, boundInteractionSession, transitionRows, scopedInteractions, runTransitionRow, interactionId, snapshot.state, snapshot.lastResponse, displayedSelectionTargets, cmdLine, setCmdLine, submitEngagementLine],
  );

  reactHostPort.useEffect(() => {
    onEngagementChange?.(engagementSpec);
  }, [engagementSpec, onEngagementChange]);

  reactHostPort.useEffect(() => () => onEngagementChange?.(null), [onEngagementChange]);

  reactHostPort.useEffect(() => {
    const state = snapshot.state;
    const lengthEntry = interactionLengthEntryForState(spec, state);
    const scalarEntry = interactionScalarEntryForState(spec, state);
    const prevLength = interactionLengthEntryForState(spec, numericEntryPrevStateRef.current);
    const prevScalar = interactionScalarEntryForState(spec, numericEntryPrevStateRef.current);
    const leftNumeric = (prevLength && (!lengthEntry || prevLength.state !== lengthEntry.state)) || (prevScalar && (!scalarEntry || prevScalar.state !== scalarEntry.state));
    if (leftNumeric) setCmdLine("");
    numericEntryPrevStateRef.current = state;
    const live = parseNumericCommandLine(cmdLine);
    if (live === undefined) return;
    if (live === null) return;
    const applyEv = interactionNumericEntryApplyEvent(spec, state, live);
    if (applyEv) void rt.send(applyEv);
  }, [cmdLine, snapshot.state, spec, rt, setCmdLine]);

  reactHostPort.useEffect(() => {
    if (!captureGlobalKeys) return;
    const onWinCapture = (e: globalThis.KeyboardEvent) => {
      if (e.defaultPrevented || e.isComposing) return;
      const t = e.target;
      const one = e.key.length === 1 ? e.key : "";
      if (replIsQueryTypingTarget(t)) return;
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "z") {
        e.preventDefault();
        e.stopPropagation();
        if (e.shiftKey) {
          rt.redo();
          onRedo?.();
        } else {
          rt.undo();
          onUndo?.();
        }
        return;
      }
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "y") {
        e.preventDefault();
        e.stopPropagation();
        rt.redo();
        onRedo?.();
        return;
      }
      if ((e.key === "Delete" || e.key === "Backspace") && !e.ctrlKey && !e.metaKey && !e.altKey && onDeleteSelection?.()) {
        e.preventDefault();
        e.stopPropagation();
        return;
      }
      if (e.key === " " && !e.ctrlKey && !e.metaKey && !e.altKey) {
        e.preventDefault();
        e.stopPropagation();
        const snap = rt.getSnapshot();
        if (interactionInNumericEntryState(spec, snap.state)) {
          const line = cmdRef.current?.value ?? cmdLine;
          const parsed = parseNumericCommandLine(line);
          const locked = interactionNumericEntryLockedValue(spec, snap.state, snap.context);
          if (parsed !== undefined && (parsed !== null || locked != null)) {
            void tryCommitNumericEntry();
            return;
          }
        }
        if (!cmdLine.trim() && confirmInteractionSelection()) return;
        const matches = replPaletteRows(cmdLine, allSuggestions);
        const interactionIdOnSpace = replInteractionIdOnSpace(cmdLine, matches, allSuggestions, lastFinalizedInteractionId);
        if (runInteractionIdFromSpace(interactionIdOnSpace)) return;
        else if (replShouldRepeatInteractionOnSpace(e, { interactionActive, cmdTarget: cmdRef.current })) repeatCurrentInteraction();
        return;
      }
      if (t !== cmdRef.current && e.key === "Backspace") {
        e.preventDefault();
        e.stopPropagation();
        cmdRef.current?.focus();
        setCmdLineRef.current((prev) => prev.slice(0, -1));
        return;
      }
      if (t !== cmdRef.current && e.key === "Escape") {
        e.preventDefault();
        e.stopPropagation();
        cmdRef.current?.focus();
        handleEscapeKey();
        return;
      }
      if (t !== cmdRef.current && e.key === "Enter") {
        e.preventDefault();
        e.stopPropagation();
        if (!cmdLine.trim() && confirmInteractionSelection()) return;
        cmdRef.current?.focus();
        const snap = rt.getSnapshot();
        if (interactionInNumericEntryState(spec, snap.state)) {
          void tryCommitNumericEntry();
          return;
        }
        if (cmdLine.trim()) void trySubmitLine();
        return;
      }
      if (!one || e.ctrlKey || e.metaKey || e.altKey) return;
      if (t === cmdRef.current) return;
      e.preventDefault();
      e.stopPropagation();
      cmdRef.current?.focus();
      setCmdLineRef.current((prev) => replCommandTextWithoutSpaces(`${prev}${one}`));
    };
    window.addEventListener("keydown", onWinCapture, true);
    return () => window.removeEventListener("keydown", onWinCapture, true);
  }, [captureGlobalKeys, rt, spec, cmdLine, allSuggestions, trySubmitLine, tryCommitNumericEntry, handleEscapeKey, interactionActive, repeatCurrentInteraction, lastFinalizedInteractionId, runInteractionIdFromSpace, confirmInteractionSelection, onUndo, onRedo, onDeleteSelection]);

  const onScenePointerMove = reactHostPort.useCallback(
    (p: Vec3) => {
      const event = createSpatialPickEvent("pointer.move", p, null);
      void rtRef.current.send(event);
      onScenePointerMoveProp?.(p, event);
    },
    [onScenePointerMoveProp],
  );

  const pickPlaneOn = snapshot.spatialInteraction.spatialGroundPick ? !snapshot.spatialInteraction.pickDisabledStates.includes(snapshot.state) : false;

  const lr = snapshot.lastResponse;
  const dragOverlayRect = canvasBinding?.domElement.getBoundingClientRect() ?? null;
  const dragOverlayPoints = dragSelection && dragOverlayRect ? dragSelection.path.map((point) => ({ x: point.x - dragOverlayRect.left, y: point.y - dragOverlayRect.top })) : [];

  return (
    <div
      className={cn("bg-background text-foreground flex min-h-0 w-full font-sans", fillHost ? "h-full flex-col" : "h-screen flex-row")}
      style={rootStyle}
    >
      <div className="relative min-h-0 min-w-0 flex-1">
        <InteractionCanvas {...canvasOverrides} frameloop={frameloop} className={cn("bg-canvas", canvasOverrides?.className)} onCanvasReady={handleCanvasReady}>
          <InteractionSelectionInvalidateBridge selectionKey={selectionInvalidateKey} />
          <InteractionSpatialView
            previewKernel={rt.previewKernel()}
            snapshot={snapshot}
            onInteractionEvent={onSpatialInteractionEvent}
            onGroundPick={onGroundPickProp}
            onScenePointerMove={pointerMoveActive ? onScenePointerMove : undefined}
            pickEnabled={pickPlaneOn}
            geometry={geometry}
            pickGeometry={pickSourceGeometry}
            committedMeshes={committedMeshesForView}
            activeModelDefinitionId={activeModelDefinitionId}
            modelDefinitionRevision={modelDefinitionRevision}
            displayModel={mergedDisplay}
            renderDisplayItem={renderDisplayItem}
            selectionAccept={hostPickingEnabled ? activeSelectionAccept : []}
            filterKindToggles={viewFilterKindToggles}
            selectionKindToggles={effectiveSelectionKindToggles}
            hoveredTargetKey={hoveredPickKey}
            selectedTargetKey={selectedPickKey}
            selectedTargetKeys={selectedPickKeys}
            hostSelectionEnabled={hostPickingEnabled}
            showPickLayer={showPickLayer}
            onSelectionRequest={onSelectionRequest}
            onCameraNavigate={onCameraNavigate}
            autoFitMeshes={autoFitMeshes}
            autoFitBehavior={autoFitBehavior}
            theme={viewTheme}
            slots={viewSlots}
            transformGumballMode={activeTransformGumballMode}
            transformGumballTargets={displayedSelectionTargets}
            onTransformGumballCommit={onTransformGumballCommit}
            {...spatialViewOverrides}
          />
        </InteractionCanvas>
        {dragSelection && dragOverlayRect ? (
          <svg
            className={cn("pointer-events-none absolute inset-0 z-[4] h-full w-full", dragSelection.coverage === "partial" ? "text-accent" : "text-foreground")}
            width="100%"
            height="100%"
          >
            {dragSelection.method === "rectangle" ? (
              <rect
                x={Math.min(dragOverlayPoints[0]?.x ?? 0, dragOverlayPoints[1]?.x ?? 0)}
                y={Math.min(dragOverlayPoints[0]?.y ?? 0, dragOverlayPoints[1]?.y ?? 0)}
                width={Math.abs((dragOverlayPoints[1]?.x ?? 0) - (dragOverlayPoints[0]?.x ?? 0))}
                height={Math.abs((dragOverlayPoints[1]?.y ?? 0) - (dragOverlayPoints[0]?.y ?? 0))}
                className="fill-current/10 stroke-current"
                strokeDasharray={dragSelection.coverage === "partial" ? "5 4" : undefined}
                strokeWidth={1.5}
              />
            ) : (
              <polygon
                points={dragOverlayPoints.map((point) => `${point.x},${point.y}`).join(" ")}
                className="fill-current/10 stroke-current"
                strokeDasharray={dragSelection.coverage === "partial" ? "5 4" : undefined}
                strokeWidth={1.5}
              />
            )}
          </svg>
        ) : null}
        {selectionMenu ? (
          <div
            onPointerDown={(e) => e.stopPropagation()}
            className={cn("fixed z-[10080] w-[220px] max-h-[210px] overflow-y-auto", cadChromePopoverClass)}
            style={{
              left: Math.min(selectionMenu.client.x + 8, window.innerWidth - 230),
              top: Math.min(selectionMenu.client.y + 8, window.innerHeight - 220),
            }}
          >
            <div className="text-muted-foreground px-single py-half text-2xs">Select target</div>
            {selectionMenu.targets.map((target) => {
              const key = spatialPickTargetKey(target);
              const active = hoveredPickKey === key;
              return (
                <button
                  key={key}
                  type="button"
                  className={cn(cadChromeMenuButtonClass, active && "bg-accent text-accent-foreground")}
                  onPointerEnter={() => setHoveredPickKey(effectiveSelectionKindToggles[target.kind] !== false ? key : null)}
                  onPointerLeave={() => setHoveredPickKey(null)}
                  onPointerDown={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    dispatchSelectionTargets([target], selectionMenu.modifiers, selectionMenu.point);
                  }}
                >
                  <span className="mr-single inline-block size-2 rounded-xs" style={{ background: targetStyle(target, false, false).color }} />
                  <span className="text-muted-foreground">{target.kind}</span> <code className="text-foreground">{target.id}</code>
                </button>
              );
            })}
          </div>
        ) : null}
      </div>
      {showAside ? (
        <aside
          className={cn(cadChromePanelAsideClass, fillHost ? "max-h-[45%] w-full shrink-0 border-l-0 border-t" : "w-[360px] shrink-0")}
          style={asideStyle}
        >
          <strong className="text-sm font-semibold">Spatial play</strong>
          <div className="flex flex-wrap gap-half">
            {transitionRows.map((row) => (
              <Button key={`${row.key}-${row.eventKind}-${row.label}`} type="button" variant="outline" size="sm" className="h-auto px-single py-half text-xs" onClick={() => runTransitionRow(row)}>
                <span className="font-bold underline">{row.key}</span> {row.label}
              </Button>
            ))}
          </div>
          <div className="border-border bg-background relative grid overflow-visible rounded-md border">
            <Input
              ref={cmdRef}
              type="text"
              autoComplete="off"
              spellCheck={false}
              value={cmdLine}
              onChange={(e) => {
                setCmdLine(replCommandTextWithoutSpaces(e.target.value));
                if (interactionMenuOpen) setInteractionMenuOpen(true);
              }}
              onKeyDown={onInputKeyDown}
              placeholder="Type an interaction or transition"
              className="col-start-1 row-start-1 border-0 bg-transparent pr-[34px] shadow-none focus-visible:ring-0"
            />
            <Button
              type="button"
              variant="outline"
              size="sm"
              className={cn("col-start-1 row-start-1 z-[1] mr-single size-[22px] justify-self-end self-center p-0 text-2xs", interactionMenuOpen && "bg-accent text-accent-foreground")}
              onMouseDown={(e) => e.preventDefault()}
              onClick={() => {
                setInteractionMenuOpen((open) => !open);
                cmdRef.current?.focus();
              }}
              aria-label="Show matching interactions"
            >
              v
            </Button>
            {completionSuffix ? (
              <div aria-hidden className="text-foreground pointer-events-none col-start-1 row-start-1 overflow-hidden pr-[34px] pl-[9px] py-[8px] text-sm leading-normal whitespace-pre">
                <span className="text-transparent">{cmdLine}</span>
                <span className="text-muted-foreground">{completionSuffix}</span>
              </div>
            ) : null}
            {interactionMenuOpen ? (
              <div onPointerDown={(e) => e.stopPropagation()} className={cn("absolute top-[calc(100%+6px)] right-0 z-[3] max-h-[220px] w-[280px] max-w-[calc(100vw-32px)] overflow-y-auto", cadChromePopoverClass)}>
                {interactionMatches.length ? (
                  interactionMatches.map((suggestion) => (
                    <button key={`${suggestion.kind}:${suggestion.key}:${suggestion.detail}`} type="button" className={cadChromeMenuButtonClass} onClick={() => runSuggestion(suggestion)}>
                      <div className="flex items-center gap-single">
                        <span className="border-border bg-muted text-foreground inline-flex min-w-6 items-center justify-center rounded-full border px-half py-0 text-2xs font-bold uppercase">{suggestion.key}</span>
                        <span>{suggestion.label}</span>
                      </div>
                      <div className="text-muted-foreground text-2xs">{suggestion.detail}</div>
                    </button>
                  ))
                ) : (
                  <div className="text-muted-foreground px-single py-half text-xs">No matching interactions.</div>
                )}
              </div>
            ) : null}
          </div>
          {asideExtra}
          {onDocumentModelChange ? (
            <SelectionAttributesPanel
              model={documentModel.model}
              activeModelDefinitionId={activeModelDefinitionId ?? defaultModelDefinitionId()}
              selection={displayedSelectionTargets}
              selectionCount={displayedSelectionTargets.length}
              onModelChange={onDocumentModelChange}
            />
          ) : null}
          <div className="flex flex-col gap-single text-xs">
            {hideModelDefinitionControls ? null : (
              <>
                <label className="flex flex-col gap-half">
                  <span>Model definition</span>
                  <select
                    value={activeModelDefinitionId ?? defaultModelDefinitionId()}
                    onChange={(e) => {
                      const next = e.target.value || defaultModelDefinitionId();
                      setActiveModelDefinitionId(next);
                      setModelDefinitionRevision((r) => r + 1);
                      setSelectionMenu(null);
                      setHoveredPickKey(null);
                    }}
                    className={cn(cadFieldClass, "border-border bg-background text-foreground rounded-md border px-single py-half")}
                  >
                    {modelDefinitions.map((row) => (
                      <option key={row.id} value={row.id}>
                        {row.label} ({row.id})
                      </option>
                    ))}
                  </select>
                </label>
                <span className="text-muted-foreground">
                  {modelDefinitionScope.typologies.length} typolog{modelDefinitionScope.typologies.length === 1 ? "y" : "ies"}
                  {" · "}
                  {modelDefinitionScope.interactions.length} interaction{modelDefinitionScope.interactions.length === 1 ? "" : "s"}
                  {" · "}
                  {modelDefinitionScope.attributeDefinitions.length} attribute{modelDefinitionScope.attributeDefinitions.length === 1 ? "" : "s"}
                  {" · "}
                  {modelDefinitionScope.propertyDefinitions.length} propert{modelDefinitionScope.propertyDefinitions.length === 1 ? "y" : "ies"}
                </span>
                {transfersFrom.length ? (
                  <label className="flex flex-col gap-half">
                    <span>Transfer from</span>
                    <select
                      defaultValue=""
                      onChange={(e) => {
                        const qid = e.target.value;
                        if (!qid) return;
                        const spec = transfersFrom.find((row) => qualifiedTransformationId(row.modelDefinitionId, row.id) === qid);
                        if (spec) onApplyTransformation?.(spec);
                        e.target.value = "";
                      }}
                      className={cn(cadFieldClass, "border-border bg-background text-foreground rounded-md border px-single py-half")}
                    >
                      <option value="">Select incoming transformation…</option>
                      {transfersFrom.map((row) => (
                        <option key={qualifiedTransformationId(row.modelDefinitionId, row.id)} value={qualifiedTransformationId(row.modelDefinitionId, row.id)}>
                          {row.label} ({row.source.modelDefinition} → {row.target.modelDefinition})
                        </option>
                      ))}
                    </select>
                  </label>
                ) : null}
                {transfersTo.length ? (
                  <label className="flex flex-col gap-half">
                    <span>Transfer to</span>
                    <select
                      defaultValue=""
                      onChange={(e) => {
                        const qid = e.target.value;
                        if (!qid) return;
                        const spec = transfersTo.find((row) => qualifiedTransformationId(row.modelDefinitionId, row.id) === qid);
                        if (spec) onApplyTransformation?.(spec);
                        e.target.value = "";
                      }}
                      className={cn(cadFieldClass, "border-border bg-background text-foreground rounded-md border px-single py-half")}
                    >
                      <option value="">Select outgoing transformation…</option>
                      {transfersTo.map((row) => (
                        <option key={qualifiedTransformationId(row.modelDefinitionId, row.id)} value={qualifiedTransformationId(row.modelDefinitionId, row.id)}>
                          {row.label} ({row.source.modelDefinition} → {row.target.modelDefinition})
                        </option>
                      ))}
                    </select>
                  </label>
                ) : null}
              </>
            )}
            {!isShapeModelDefinition(activeModelDefinitionId) ? (
              <span className="text-muted-foreground">
                {viewObjectCount} object{viewObjectCount === 1 ? "" : "s"}
              </span>
            ) : null}
            <label className="text-foreground flex items-center gap-single text-xs font-semibold">
              <SpatialChromeMasterToggle state={primitiveShowGroupState} ariaLabel="Show all primitives" onEnabledChange={(enabled) => setFilterPrimitiveToggles(spatialToggleGroupFill(SPATIAL_PRIMITIVE_KINDS, enabled))} />
              Primitives · Show
            </label>
            <div role="group" aria-label="Show primitives" className="flex flex-wrap gap-half">
              {SPATIAL_PRIMITIVE_KINDS.map((kind) => (
                <label key={`show-primitive-${kind}`} className={cn(cadChromeTagClass, filterPrimitiveToggles[kind] !== false ? cadChromeTagOnClass : cadChromeTagOffClass)}>
                  <input
                    type="checkbox"
                    checked={filterPrimitiveToggles[kind] !== false}
                    onChange={(e) => {
                      setFilterPrimitiveToggles((prev) => ({ ...prev, [kind]: e.target.checked }));
                    }}
                  />
                  {kind}
                </label>
              ))}
            </div>
            <label className="text-foreground flex items-center gap-single text-xs font-semibold">
              <SpatialChromeMasterToggle
                state={primitiveFilterGroupState}
                ariaLabel="Filter all primitives"
                onEnabledChange={(enabled) => {
                  setSelectionPrimitiveToggles(spatialToggleGroupFill(SPATIAL_PRIMITIVE_KINDS, enabled));
                  setSelectionMenu(null);
                  setHoveredPickKey(null);
                  if (!enabled) {
                    applySelectionPrune((prev) => {
                      let next = prev;
                      for (const kind of SPATIAL_PRIMITIVE_KINDS) {
                        next = replPruneSelectionByPrimitive(next, kind);
                      }
                      return next;
                    });
                  }
                }}
              />
              Primitives · Filter
            </label>
            <div role="group" aria-label="Filter primitives" className="flex flex-wrap gap-half">
              {SPATIAL_PRIMITIVE_KINDS.map((kind) => (
                <label key={`filter-primitive-${kind}`} className={cn(cadChromeTagClass, selectionPrimitiveToggles[kind] !== false ? cadChromeTagOnClass : cadChromeTagOffClass)}>
                  <input
                    type="checkbox"
                    checked={selectionPrimitiveToggles[kind] !== false}
                    onChange={(e) => {
                      const checked = e.target.checked;
                      setSelectionPrimitiveToggles((prev) => ({ ...prev, [kind]: checked }));
                      setSelectionMenu(null);
                      setHoveredPickKey(null);
                      if (!checked) {
                        applySelectionPrune((prev) => replPruneSelectionByPrimitive(prev, kind));
                      }
                    }}
                  />
                  {kind}
                </label>
              ))}
            </div>
            <label className="text-foreground flex items-center gap-single text-xs font-semibold">
              <SpatialChromeMasterToggle state={typologyShowGroupState} ariaLabel="Show all typologies" onEnabledChange={(enabled) => setFilterTypologyToggles(spatialToggleGroupFill(scopeTypologyIds, enabled))} />
              Typologies · Show
            </label>
            <div role="group" aria-label="Show typologies" className="flex flex-wrap gap-half">
              {modelDefinitionScope.typologies.map((typology) => {
                const label = spatialTypologyToggleLabel(typology.id, typology.label);
                return (
                  <label key={`show-${typology.id}`} className={cn(cadChromeTagClass, filterTypologyToggles[typology.id] !== false ? cadChromeTagOnClass : cadChromeTagOffClass)}>
                    <input
                      type="checkbox"
                      checked={filterTypologyToggles[typology.id] !== false}
                      onChange={(e) => {
                        setFilterTypologyToggles((prev) => ({ ...prev, [typology.id]: e.target.checked }));
                      }}
                    />
                    {label}
                  </label>
                );
              })}
            </div>
            <label className="text-foreground flex items-center gap-single text-xs font-semibold">
              <SpatialChromeMasterToggle
                state={typologySelectionGroupState}
                ariaLabel="Select all typologies"
                onEnabledChange={(enabled) => {
                  setSelectionTypologyToggles(spatialToggleGroupFill(scopeTypologyIds, enabled));
                  setSelectionMenu(null);
                  setHoveredPickKey(null);
                  if (!enabled) {
                    applySelectionPrune((prev) => {
                      let next = prev;
                      for (const typologyId of scopeTypologyIds) {
                        next = replPruneSelectionByTypology(next, documentModel.model, activeModelDefinitionId, typologyId);
                      }
                      return next;
                    });
                  }
                }}
              />
              Typologies · Selection
            </label>
            <div role="group" aria-label="Selection typologies" className="flex flex-wrap gap-half">
              {modelDefinitionScope.typologies.map((typology) => {
                const label = spatialTypologyToggleLabel(typology.id, typology.label);
                return (
                  <label key={`select-${typology.id}`} className={cn(cadChromeTagClass, selectionTypologyToggles[typology.id] !== false ? cadChromeTagOnClass : cadChromeTagOffClass)}>
                    <input
                      type="checkbox"
                      checked={selectionTypologyToggles[typology.id] !== false}
                      onChange={(e) => {
                        const checked = e.target.checked;
                        setSelectionTypologyToggles((prev) => ({ ...prev, [typology.id]: checked }));
                        setSelectionMenu(null);
                        setHoveredPickKey(null);
                        if (!checked) {
                          applySelectionPrune((prev) => replPruneSelectionByTypology(prev, documentModel.model, activeModelDefinitionId, typology.id));
                        }
                      }}
                    />
                    {label}
                  </label>
                );
              })}
            </div>
          </div>
          <div className="text-muted-foreground text-xs">
            {interactionId ? (
              <>
                Interaction <code className="text-foreground">{interactionId}</code> · state <code className="text-foreground">{snapshot.state}</code> · rev {snapshot.revision}
              </>
            ) : (
              <>
                No interaction selected · state <code className="text-foreground">{snapshot.state}</code> · rev {snapshot.revision}
              </>
            )}
          </div>
          <div className="border-border text-xs border-t pt-single">
            <strong className="text-foreground">Last response</strong>
            <pre className="text-muted-foreground mt-half mb-0 max-h-[120px] overflow-auto text-2xs">{lr ? JSON.stringify(lr, null, 2) : "—"}</pre>
            {snapshot.diagnostics.length ? (
              <ul className="text-muted-foreground m-0 list-inside list-disc text-2xs">
                {snapshot.diagnostics.map((d, i) => (
                  <li key={`${d.code}-${i}`}>
                    [{d.severity}] {d.code}: {d.message}
                  </li>
                ))}
              </ul>
            ) : null}
          </div>
        </aside>
      ) : null}
    </div>
  );
}

/** @emoji ­ƒ╝´©Å Canvas-only {@link InteractionRepl} (no model-definition aside); full host props and `on*` callbacks. */
export function InteractionReplViewport(props: InteractionReplProps): ReactNode {
  return <InteractionRepl {...props} showAside={false} fillHost />;
}

export interface SelectionAttributesPanelProps {
  readonly model: Model;
  readonly activeModelDefinitionId: string;
  readonly selection: readonly SelectionTarget[];
  readonly selectionCount?: number;
  readonly onModelChange: (model: Model) => void;
}

/** @emoji 🏷️ Edits {@link Model.metadata} fields for the primary selection using active model-definition attribute assets. */
export function SelectionAttributesPanel({ model, activeModelDefinitionId, selection, selectionCount, onModelChange }: SelectionAttributesPanelProps): ReactNode {
  const target = reactHostPort.useMemo(() => primaryAttributeSelectionTarget(selection), [selection]);
  const definitions = reactHostPort.useMemo(() => (target ? listAttributeDefinitionsForModelDefinitionEntity(activeModelDefinitionId, target.kind) : []), [activeModelDefinitionId, target]);
  if (!target) {
    return (
      <p className="text-muted-foreground text-xs leading-snug">
        Select a primitive or object to edit attributes for <code className="text-foreground">{activeModelDefinitionId}</code>.
      </p>
    );
  }
  if (!definitions.length) {
    return (
      <p className="text-muted-foreground text-xs leading-snug">
        No attribute definitions for <code className="text-foreground">{target.kind}</code> on this model definition.
      </p>
    );
  }
  const fields = model.metadata.get(target.id) ?? {};
  const count = selectionCount ?? selection.length;
  const setField = (defn: AttributeDefinitionSpec, value: unknown) => {
    if (!validateAttributeValue(defn, value)) return;
    model.metadata.setField(target.id, defn.field, value);
    onModelChange(model);
  };
  const clearField = (defn: AttributeDefinitionSpec) => {
    model.metadata.deleteField(target.id, defn.field);
    onModelChange(model);
  };
  const fieldRow = (defn: AttributeDefinitionSpec, current: unknown, control: ReactNode) => (
    <div key={defn.id} className="flex flex-col gap-half">
      <div className="flex items-center justify-between gap-single">
        <Label className="text-xs">{defn.label}</Label>
        {current !== undefined ? (
          <Button type="button" variant="outline" size="sm" className="h-auto px-half py-0 text-2xs" onClick={() => clearField(defn)}>
            Clear
          </Button>
        ) : null}
      </div>
      {control}
    </div>
  );
  return (
    <div className="flex flex-col gap-single text-xs">
      <span className="text-foreground text-sm font-semibold">Attributes</span>
      <span className="text-muted-foreground text-2xs">
        {target.kind} · <code className="text-foreground">{target.id}</code>
        {count > 1 ? ` · ${count} selected` : ""}
      </span>
      {definitions.map((defn) => {
        const editor = attributeDefinitionEditorKind(defn);
        const current = fields[defn.field];
        if (editor === "enum") {
          const options = attributeDefinitionValueOptions(defn) ?? [];
          return fieldRow(
            defn,
            current,
            <select
              value={typeof current === "string" ? current : ""}
              onChange={(e) => {
                if (!e.target.value) clearField(defn);
                else setField(defn, e.target.value);
              }}
              className={cn(cadFieldClass, "border-border bg-background text-foreground rounded-md border px-single py-half")}
            >
              <option value="">—</option>
              {options.map((option) => (
                <option key={option} value={option}>
                  {option}
                </option>
              ))}
            </select>,
          );
        }
        if (editor === "number") {
          return fieldRow(
            defn,
            current,
            <Input
              type="number"
              className={cadFieldClass}
              value={typeof current === "number" ? current : ""}
              onChange={(e) => {
                if (e.target.value === "") clearField(defn);
                else setField(defn, Number(e.target.value));
              }}
            />,
          );
        }
        if (editor === "boolean") {
          return fieldRow(
            defn,
            current,
            <label className="flex items-center gap-single">
              <input type="checkbox" checked={current === true} onChange={(e) => setField(defn, e.target.checked)} />
              <span>Enabled</span>
            </label>,
          );
        }
        return fieldRow(
          defn,
          current,
          <Input
            type="text"
            className={cadFieldClass}
            value={typeof current === "string" ? current : current === undefined || current === null ? "" : JSON.stringify(current)}
            onChange={(e) => {
              if (!e.target.value) clearField(defn);
              else setField(defn, e.target.value);
            }}
          />,
        );
      })}
    </div>
  );
}

export interface SelectionPropertiesPanelProps {
  readonly model: Model;
  readonly kernel: SpatialKernel;
  readonly activeModelDefinitionId: string;
  readonly selection: readonly SelectionTarget[];
  readonly selectionCount?: number;
}

/** @emoji 📐 Displays derived property values for the primary object selection using scoped property definitions. */
export function SelectionPropertiesPanel({ model, kernel, activeModelDefinitionId, selection, selectionCount }: SelectionPropertiesPanelProps): ReactNode {
  const objectRow = reactHostPort.useMemo(() => {
    const objectTarget = selection.find((row) => row.kind === "object");
    return objectTarget ? (model.objects[objectTarget.id] ?? null) : null;
  }, [model, selection]);
  const definitions = reactHostPort.useMemo(() => (objectRow ? listApplicablePropertyDefinitionsForModelDefinition(activeModelDefinitionId, model, objectRow) : []), [activeModelDefinitionId, model, objectRow]);
  const [values, setValues] = reactHostPort.useState<Readonly<Record<string, Record<string, unknown>>>>({});
  reactHostPort.useEffect(() => {
    if (!objectRow || !definitions.length) {
      setValues({});
      return;
    }
    let cancelled = false;
    void (async () => {
      const next: Record<string, Record<string, unknown>> = {};
      for (const defn of definitions) {
        next[defn.id] = await derivePropertyValue(defn, { model, kernel, object: objectRow });
      }
      if (!cancelled) setValues(next);
    })();
    return () => {
      cancelled = true;
    };
  }, [definitions, kernel, model, objectRow]);
  if (!objectRow || !definitions.length) return null;
  const count = selectionCount ?? selection.length;
  return (
    <div className="flex flex-col gap-single text-xs">
      <span className="text-foreground text-sm font-semibold">Properties</span>
      <span className="text-muted-foreground text-2xs">
        object · <code className="text-foreground">{objectRow.id}</code>
        {count > 1 ? ` · ${count} selected` : ""}
      </span>
      {definitions.map((defn) => (
        <div key={defn.id} className="flex flex-col gap-half">
          <span>{defn.label}</span>
          <pre className="text-muted-foreground m-0 overflow-auto text-2xs">{JSON.stringify(values[defn.id] ?? {}, null, 2)}</pre>
        </div>
      ))}
    </div>
  );
}
// #endregion ­ƒ¬®Repl

// #region ­ƒº¬Tests
const __cadRendererTestKernel = import.meta.vitest ? await import("@cad/js/kernel/brepjs") : null;

if (import.meta.vitest) {
  const { BrepjsKernel, preciseSpatialKernelMath: M } = __cadRendererTestKernel!;
  const { describe, it, expect } = import.meta.vitest;

  describe("replIsQueryTypingTarget", () => {
    it("treats text inputs and engagement fields as typing targets", () => {
      const input = document.createElement("input");
      input.type = "text";
      expect(replIsQueryTypingTarget(input)).toBe(true);
      const checkbox = document.createElement("input");
      checkbox.type = "checkbox";
      expect(replIsQueryTypingTarget(checkbox)).toBe(false);
      const engagement = document.createElement("div");
      engagement.setAttribute("data-slot", "engagement");
      const nested = document.createElement("input");
      engagement.append(nested);
      expect(replIsQueryTypingTarget(nested)).toBe(true);
    });
  });

  describe("buildInteractionReplEngagement", () => {
    const baseInputs: InteractionReplEngagementInputs = {
      showEngagement: true,
      boundInteractionSession: true,
      interactionId: "primitive.box",
      state: "first_corner",
      lastResponseOk: null,
      lastResponseErrorCount: 0,
      selectionCount: 0,
      cmdLine: "",
      transitions: [{ eventKind: "confirm", key: "c", label: "Confirm" }],
      interactions: [{ id: "primitive.box", key: "b", label: "Box" }],
      onTransition: () => {},
      onStartInteraction: () => {},
      onInputChange: () => {},
      onInputSubmit: () => {},
    };

    it("returns null when engagement is disabled", () => {
      expect(buildInteractionReplEngagement({ ...baseInputs, showEngagement: false })).toBeNull();
    });

    it("lists active session transitions, command input, and status", () => {
      const transitionRuns: string[] = [];
      const spec = buildInteractionReplEngagement({
        ...baseInputs,
        selectionCount: 2,
        lastResponseOk: true,
        onTransition: (row) => transitionRuns.push(row.key),
      });
      expect(spec?.options?.[0]?.label).toBe("C Confirm");
      expect(spec?.input?.placeholder).toBe("Type a transition or value");
      expect(spec?.status?.map((row) => row.content)).toEqual(["State: first_corner", "Selected: 2", "OK"]);
      spec?.options?.[0]?.onPress?.();
      expect(transitionRuns).toEqual(["c"]);
      expect(spec?.possibleEngagements?.[0]?.label).toBe("C Confirm");
    });

    it("exposes only a command input while idle so a window can start an interaction", () => {
      const submitted: string[] = [];
      const started: string[] = [];
      const spec = buildInteractionReplEngagement({
        ...baseInputs,
        boundInteractionSession: false,
        interactionId: "",
        onInputSubmit: (value) => submitted.push(value),
        onStartInteraction: (id) => started.push(id),
      });
      expect(spec?.options).toBeUndefined();
      expect(spec?.input?.placeholder).toBe("Type an interaction");
      expect(spec?.possibleEngagements?.map((row) => row.label)).toEqual(["Box"]);
      spec?.input?.onSubmit?.("box");
      expect(submitted).toEqual(["box"]);
      spec?.possibleEngagements?.[0]?.onSelect?.();
      expect(started).toEqual(["primitive.box"]);
    });

    it("returns null when idle with no startable interactions and nothing selected", () => {
      expect(
        buildInteractionReplEngagement({
          ...baseInputs,
          boundInteractionSession: false,
          interactionId: "",
          interactions: [],
          selectionCount: 0,
        }),
      ).toBeNull();
    });

    it("keeps selection status visible when idle with no interactions", () => {
      const spec = buildInteractionReplEngagement({
        ...baseInputs,
        boundInteractionSession: false,
        interactionId: "",
        interactions: [],
        selectionCount: 3,
      });
      expect(spec?.options).toBeUndefined();
      expect(spec?.input).toBeUndefined();
      expect(spec?.status?.map((row) => row.content)).toEqual(["Selected: 3"]);
    });

    it("summarizes failed responses with error counts", () => {
      const spec = buildInteractionReplEngagement({ ...baseInputs, lastResponseOk: false, lastResponseErrorCount: 2 });
      expect(spec?.status?.some((row) => row.content === "Error (2)")).toBe(true);
    });
  });

  describe("@cad/js/renderer interaction adapter", () => {
    it("replHostGeometryPickingEnabled follows pickDisabledStates while session is active", () => {
      const spec = loadSpatialInteraction("primitive.box");
      expect(replHostGeometryPickingEnabled("primitive.box", spec, "first_corner")).toBe(false);
      expect(replHostGeometryPickingEnabled("primitive.box", spec, "ready")).toBe(true);
      expect(replHostGeometryPickingEnabled("primitive.box", spec, "committed")).toBe(true);
      expect(replHostGeometryPickingEnabled("primitive.box", spec, "idle")).toBe(true);
      expect(replHostGeometryPickingEnabled("", spec, "first_corner")).toBe(true);
    });

    it("surface.extrudeCrv enables host curve picking and disables ground plane during select_curves_to_extrude", () => {
      const spec = loadSpatialInteraction("surface.extrudeCrv");
      expect(spec).not.toBeNull();
      expect(replHostGeometryPickingEnabled("surface.extrudeCrv", spec!, "select_curves_to_extrude")).toBe(true);
      const snapshot = {
        state: "select_curves_to_extrude",
        spatialInteraction: mergeInteractionSpatial(spec!),
      } satisfies Pick<InteractionSnapshot, "state" | "spatialInteraction">;
      expect(interactionSpatialGroundPickPlaneEnabled(snapshot, true)).toBe(false);
    });

    it("enables spatial ground pick plane during rubber-band states regardless of host selection accept", () => {
      const snapshot = {
        state: "first_corner",
        spatialInteraction: {
          spatialGroundPick: true,
          pickDisabledStates: ["idle", "ready", "committed"],
          groundPointerMoveStates: ["first_corner"],
          heightDragStates: [],
          verticalRodStates: [],
          heightConfirmState: null,
        },
      } satisfies Pick<InteractionSnapshot, "state" | "spatialInteraction">;
      expect(interactionSpatialGroundPickPlaneEnabled(snapshot, true)).toBe(true);
      expect(interactionSpatialGroundPickPlaneEnabled(snapshot, false)).toBe(false);
    });

    it("creates snap and selection metadata for geometry targets", () => {
      const model = new Model();
      model.vertices.v0 = { id: "v0" as VertexRef, position: [1, 2, 3] };
      const targets = createSpatialPickTargets(model);
      expect(targets).toEqual([{ kind: "vertex", geometryKind: "vertex", id: "v0", point: [1, 2, 3], typologyId: undefined }]);
      expect(createSpatialPickEvent("pointer.down", [9, 9, 9], targets[0]!, { shift: true })).toEqual({
        kind: "pointer.down",
        point: [9, 9, 9],
        modifiers: { shift: true },
        snap: { kind: "vertex", id: "v0", point: [1, 2, 3] },
        selection: { kind: "vertex", id: "v0" },
      });
    });

    it("adds typology object picks for non-shape model definitions", async () => {
      const model = new Model();
      const cell = solidRef("c0");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
      model.objects["object-c0"] = {
        id: "object-c0" as ObjectRef,
        typology: "energy.energy.hull",
        primitives: { solid: String(cell) },
      };
      const activeModelDefinitionId = "aec.building.energy";
      const editTargets = createSpatialPickTargets(model, defaultModelDefinitionId());
      const objectTargets = createSpatialPickTargets(model, activeModelDefinitionId);
      expect(editTargets.some((t) => t.kind === "vertex")).toBe(true);
      expect(objectTargets.some((t) => t.kind === "object" && !t.geometryKind)).toBe(true);
      expect(objectTargets.some((t) => t.geometryKind === "vertex")).toBe(true);
      const structureTargets = createSpatialPickTargets(model, "aec.building.structure");
      expect(structureTargets.some((t) => t.kind === "face")).toBe(true);
      expect(structureTargets.some((t) => t.kind === "object")).toBe(true);
    });

    it("filterSpatialPickTargetsForActiveView scopes by model definition entity kinds", () => {
      const targets: SpatialPickTarget[] = [
        { kind: "vertex", geometryKind: "vertex", id: "v0", point: [0, 0, 0] },
        { kind: "face", geometryKind: "face", id: "f0", point: [0.5, 0.5, 0.5] },
        { kind: "object", id: "energy.energy.hull", point: [0.5, 0.5, 0.5] },
      ];
      expect(filterSpatialPickTargetsForActiveView(targets, defaultModelDefinitionId()).map(spatialPickTargetKey)).toEqual(["vertex:v0", "face:f0"]);
      expect(filterSpatialPickTargetsForActiveView(targets, "aec.building.energy").map(spatialPickTargetKey)).toEqual(["vertex:v0", "face:f0", "object:energy.energy.hull"]);
      expect(filterSpatialPickTargetsForActiveView(targets, "aec.building.structure").map(spatialPickTargetKey)).toEqual(["vertex:v0", "face:f0", "object:energy.energy.hull"]);
    });

    it("polylineWireSegments tessellates nurbs edge samples for factory wireframe", () => {
      const model = new Model();
      const v0 = { id: "v0" as VertexRef, position: [0, 0, 0] as Vec3 };
      const v1 = { id: "v1" as VertexRef, position: [4, 0, 0] as Vec3 };
      const edge = {
        id: "e0" as EdgeRef,
        vertexIds: [v0.id, v1.id] as [VertexRef, VertexRef],
        curve: {
          kind: "nurbs" as const,
          poles: [
            [0, 0, 0],
            [2, 1, 0],
            [4, 0, 0],
          ] as Vec3[],
          degree: 2,
          through: true,
        },
      };
      const wire = { id: "w0" as WireRef, edgeIds: [edge.id] };
      applyModelDiff(model, { vertices: { added: [v0, v1] }, edges: { added: [edge] }, wires: { added: [wire] } });
      const buckets = geometryBuckets(model);
      const segments = collectGeometryEdgeSegments(buckets);
      const edgeSegments = geometryEntityWireSegments(buckets, "edge", edge.id);
      expect(segments.length).toBeGreaterThan(2);
      expect(edgeSegments.length).toBeGreaterThan(2);
      expect(segments[0]).toEqual(edgeSegments[0]);
    });

    it("ensureTypologyObjectFromCreateDiff binds wire primitive for interpolate curve", () => {
      const model = new Model();
      const diff: ModelDiff = {
        wires: { added: [{ id: "w0" as WireRef, edgeIds: ["e0" as EdgeRef] }] },
        edges: { added: [{ id: "e0" as EdgeRef, vertexIds: ["v0" as VertexRef, "v1" as VertexRef] }] },
      };
      ensureTypologyObjectFromCreateDiff(model, "spatial.shape.curve.interpolate-curve", diff);
      expect(model.objects["spatial.shape.curve.interpolate-curve"]?.primitives.wire).toBe("w0");
    });

    it("geometryEntityThroughPoints returns nurbs interpolation poles for wire selection", () => {
      const model = new Model();
      const v0 = { id: "v0" as VertexRef, position: [0, 0, 0] as Vec3 };
      const v1 = { id: "v1" as VertexRef, position: [4, 0, 0] as Vec3 };
      const edge = {
        id: "e0" as EdgeRef,
        vertexIds: [v0.id, v1.id] as [VertexRef, VertexRef],
        curve: {
          kind: "nurbs" as const,
          poles: [
            [0, 0, 0],
            [2, 1, 0],
            [4, 0, 0],
          ] as Vec3[],
          degree: 2,
          through: true,
        },
      };
      const wire = { id: "w0" as WireRef, edgeIds: [edge.id] };
      applyModelDiff(model, { vertices: { added: [v0, v1] }, edges: { added: [edge] }, wires: { added: [wire] } });
      const buckets = geometryBuckets(model);
      expect(geometryEntityThroughPoints(buckets, "edge", edge.id)).toHaveLength(3);
      expect(geometryEntityThroughPoints(buckets, "wire", wire.id)).toHaveLength(3);
      expect(nurbsThroughPointsFromEdge(edge)).toEqual(edge.curve!.kind === "nurbs" ? edge.curve.poles : null);
    });

    it("resolveSpatialSceneVisibility switches edit wireframe vs committed object mesh", () => {
      expect(resolveSpatialSceneVisibility(defaultModelDefinitionId(), { edge: true, face: true })).toEqual({
        showFactoryWireframe: true,
        showCommittedFaces: true,
        showCommittedEdges: true,
      });
      expect(resolveSpatialSceneVisibility("aec.building.energy", { edge: true, face: true, object: true })).toEqual({
        showFactoryWireframe: true,
        showCommittedFaces: true,
        showCommittedEdges: true,
      });
    });

    it("transformGumballMatrixDiff translates solid selection vertices", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const solidId = Object.keys(model.solids)[0]!;
      const diff = transformGumballMatrixDiff(
        model,
        [{ kind: "solid", id: solidId, editable: true }],
        { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] },
        { position: [2, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] },
      );
      applyModelDiff(model, diff);
      for (const v of Object.values(model.vertices)) {
        expect(v.position[0]).toBeGreaterThanOrEqual(1.5);
      }
    });

    it("defaultInteractionReplChromeState seeds typology and primitive toggles by default", () => {
      const chrome = defaultInteractionReplChromeState();
      expect(chrome.activeModelDefinitionId).toBe(defaultModelDefinitionId());
      expect(chrome.filterTypologyToggles["spatial.shape.primitive.box"]).toBe(true);
      expect(chrome.filterPrimitiveToggles.vertex).toBe(true);
      expect(chrome.filterPrimitiveToggles.solid).toBe(true);
    });

    it("filterFootprintBoxPreviewDisplayItems removes box-preview when model has solids", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const display: DisplayModel = {
        items: [
          { kind: "box-preview", id: "preview-committed", params: { cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 } },
          { kind: "point", id: "p0", params: { position: [0, 0, 0] } },
        ],
      };
      const filtered = filterFootprintBoxPreviewDisplayItems(display, model);
      expect(filtered.items.some((item) => item.kind === "box-preview")).toBe(false);
      expect(filtered.items.some((item) => item.kind === "point")).toBe(true);
    });

    it("historyEntryArchivesBoxFootprint skips transform and measure interactions", () => {
      expect(historyEntryArchivesBoxFootprint("transform.move")).toBe(false);
      expect(historyEntryArchivesBoxFootprint("transform.copy")).toBe(false);
      expect(historyEntryArchivesBoxFootprint("measure.vertexDistance")).toBe(false);
      expect(historyEntryArchivesBoxFootprint("primitive.box")).toBe(true);
    });

    it("filterSpatialPickTargetsForPrimitiveToggles hides primitive picks by kind", () => {
      const model = new Model();
      model.vertices.v0 = { id: "v0" as VertexRef, position: [0, 0, 0] };
      model.edges.e0 = { id: "e0" as EdgeRef, vertexIds: ["v0" as VertexRef, "v0" as VertexRef], curve: { kind: "line" } };
      const targets = createSpatialPickTargets(model);
      const visible = filterSpatialPickTargetsForPrimitiveToggles(targets, { vertex: false });
      expect(visible.some((row) => row.geometryKind === "vertex")).toBe(false);
      expect(visible.some((row) => row.geometryKind === "edge")).toBe(true);
    });

    it("spatialTypologyToggleLabel uses typology label pascal case", () => {
      expect(spatialTypologyToggleLabel("energy.energy.baseplate", "Base Plate")).toBe("BasePlate");
      expect(spatialTypologyToggleLabel("spatial.shape.primitive.box", "Box")).toBe("Box");
    });

    it("filterSpatialPickTargetsForTypologyToggles hides typology object picks", async () => {
      const model = new Model();
      const cell = solidRef("c0");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
      model.objects["hull"] = {
        id: "hull" as ObjectRef,
        typology: "energy.energy.hull",
        primitives: { solid: String(cell) },
      };
      const targets = createSpatialPickTargets(model, "aec.building.energy");
      const typologyIds = modelDefinitionTypologyIds("aec.building.energy");
      const visible = filterSpatialPickTargetsForTypologyToggles(targets, { "energy.energy.hull": false }, typologyIds);
      expect(visible.some((row) => row.typologyId === "energy.energy.hull")).toBe(false);
    });

    it("scopes displayed selection to activeModelDefinitionId", () => {
      const rendererByModel: SpatialRendererSelectionByModel = {
        [defaultModelDefinitionId()]: [{ kind: "face", id: "f0", editable: true }],
        "aec.building.energy": [
          { kind: "face", id: "f0", editable: true },
          { kind: "object", id: "o0", editable: false },
        ],
      };
      expect(replDisplayedSelectionTargets(false, defaultModelDefinitionId(), "idle", rendererByModel, {})).toEqual([{ kind: "face", id: "f0", editable: true }]);
      expect(replDisplayedSelectionTargets(false, "aec.building.energy", "idle", rendererByModel, {})).toEqual([
        { kind: "face", id: "f0", editable: true },
        { kind: "object", id: "o0", editable: false },
      ]);
      expect(replDisplayedSelectionTargets(false, "aec.building.structure", "idle", rendererByModel, {})).toEqual([]);
    });

    it("creates anchor and shell pick targets for spatial.shape geometry", () => {
      const model = new Model();
      model.anchors["a0"] = { id: "a0" as AnchorRef, position: [0, 0, 0], attachment: { kind: "vertex", id: "v0" } };
      model.vertices["v0"] = { id: "v0" as VertexRef, position: [0, 0, 0] };
      const cell = solidRef("c0");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
      const faceId = Object.keys(model.faces)[0]!;
      model.shells["sh0"] = { id: "sh0" as ShellRef, faceIds: [faceId] };
      const targets = createSpatialPickTargets(model, defaultModelDefinitionId());
      expect(targets.some((t) => t.geometryKind === "anchor")).toBe(true);
      expect(targets.some((t) => t.geometryKind === "shell")).toBe(true);
    });

    it("modelDefinitionPickTargetKinds maps primitive entity kinds to pick toggles", () => {
      expect(modelDefinitionPickTargetKinds(defaultModelDefinitionId()).sort()).toEqual(["edge", "face", "object", "vertex"]);
      expect(modelDefinitionPickTargetKinds("aec.building.structure").sort()).toEqual(["edge", "face", "object", "vertex"]);
    });

    it("merges picks within active model definition without clearing other models", () => {
      const rendererByModel: SpatialRendererSelectionByModel = {
        [defaultModelDefinitionId()]: [{ kind: "wire", id: "w0", editable: true }],
        "aec.building.energy": [{ kind: "object", id: "o0", editable: false }],
      };
      expect(replMergeSelectionPickInView(false, defaultModelDefinitionId(), "idle", rendererByModel, {}, [{ kind: "wire", id: "w1", editable: true }], {})).toEqual([{ kind: "wire", id: "w1", editable: true }]);
      expect(replRendererSelectionTargets(rendererByModel, "aec.building.energy")).toEqual([{ kind: "object", id: "o0", editable: false }]);
    });

    it("clears selection on empty background pick in default modifier mode", () => {
      const rendererByModel: SpatialRendererSelectionByModel = {
        [defaultModelDefinitionId()]: [{ kind: "wire", id: "w0", editable: true }],
      };
      expect(replMergeSelectionPickInView(false, defaultModelDefinitionId(), "idle", rendererByModel, {}, [], {})).toEqual([]);
      expect(replMergeSelectionPickInView(false, defaultModelDefinitionId(), "idle", rendererByModel, {}, [], { shift: true })).toEqual([{ kind: "wire", id: "w0", editable: true }]);
    });

    it("maps selection target keys to pick target keys for highlights", () => {
      const keys = pinnedPickTargetKeys(new Set(["shell:sh0" as string]));
      expect(keys.has("shell:sh0")).toBe(true);
      expect(keys.has("face:sh0")).toBe(true);
    });

    it("keeps interaction selection isolated per state", () => {
      const interactionByState: SpatialInteractionSelectionByState = {
        first_corner: [{ kind: "vertex", id: "v0", editable: true }],
        second_corner: [{ kind: "vertex", id: "v1", editable: true }],
      };
      expect(replDisplayedSelectionTargets(true, defaultModelDefinitionId(), "first_corner", {}, interactionByState)).toEqual([{ kind: "vertex", id: "v0", editable: true }]);
      expect(replMergeSelectionPickInView(true, defaultModelDefinitionId(), "second_corner", {}, interactionByState, [{ kind: "edge", id: "e0", editable: true }], { shift: true })).toEqual([
        { kind: "vertex", id: "v1", editable: true },
        { kind: "edge", id: "e0", editable: true },
      ]);
    });

    it("spatialToggleGroupState reports all, none, and partial chrome groups", () => {
      expect(spatialToggleGroupState(["a", "b"], { a: true, b: true })).toBe("all");
      expect(spatialToggleGroupState(["a", "b"], { a: false, b: false })).toBe("none");
      expect(spatialToggleGroupState(["a", "b"], { a: true, b: false })).toBe("partial");
      expect(spatialToggleGroupFill(["a", "b"], true)).toEqual({ a: true, b: true });
      expect(spatialToggleGroupFill(["a", "b"], false)).toEqual({ a: false, b: false });
    });

    it("filterSpatialPickTargets matches primitive geometryKind in selection accept", () => {
      const targets: SpatialPickTarget[] = [
        { kind: "face", geometryKind: "shell", id: "sh0", point: [0, 0, 0] },
        { kind: "vertex", geometryKind: "anchor", id: "a0", point: [1, 0, 0] },
      ];
      expect(filterSpatialPickTargets(targets, ["shell"], {}).map(spatialPickTargetKey)).toEqual(["face:sh0"]);
      expect(filterSpatialPickTargets(targets, ["anchor"], {}).map(spatialPickTargetKey)).toEqual(["vertex:a0"]);
    });

    it("resolveSpatialPickTargetsToRender draws all enabled kinds", () => {
      const targets: SpatialPickTarget[] = [
        { kind: "vertex", geometryKind: "vertex", id: "v0", point: [0, 0, 0] },
        {
          kind: "edge",
          geometryKind: "edge",
          id: "e0",
          point: [0, 0, 0],
          points: [
            [0, 0, 0],
            [1, 0, 0],
          ],
        },
      ];
      expect(resolveSpatialPickTargetsToRender(targets, { edge: false }).map(spatialPickTargetKey)).toEqual(["vertex:v0"]);
      expect(resolveSpatialPickTargetsToRender(targets, {}).map(spatialPickTargetKey).sort()).toEqual(["edge:e0", "vertex:v0"]);
    });

    it("spatialSceneColors exposes a single product-aligned palette", () => {
      resetSpatialSceneColorCache();
      const palette = spatialSceneColors();
      expect(palette.accent).toBeTruthy();
      expect(palette.construction).toBe(palette.accent);
      expect(palette.selected).toBeTruthy();
      expect(palette.groundPlane).not.toBe(palette.accent);
      resetSpatialSceneColorCache();
    });

    it("defaultInteractionSpatialViewTheme hides the factory ground plane tint", () => {
      expect(defaultInteractionSpatialViewTheme.groundPlaneOpacity).toBe(0);
    });
  });
}

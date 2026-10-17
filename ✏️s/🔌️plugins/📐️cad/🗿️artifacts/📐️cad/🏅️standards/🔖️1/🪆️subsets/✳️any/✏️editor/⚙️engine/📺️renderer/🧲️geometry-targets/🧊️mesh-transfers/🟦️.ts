/** 🧊️ CAD renderer geometry targets: the tessellation worker hooks, the mesh transfers and the visible solid and face meshes of a model definition. */
import { reactHostPort, type BufferGeometry as ThreeBufferGeometry } from "@semio-tech/framework/ui/targets/react";
import {
  kernelGeometry,
  solidRef,
  type FaceGroup,
  type FaceInfo,
  type MeshTransfer,
  type Vec3
} from "@semio-tech/cad-engine-ts/registry";
import { type SpatialObjectRecord, type ModelJson } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { resolveTypologyStyle } from "@semio-tech/s-spatial-kernel-ts/geometry/typology";
import { isShapeModelDefinition, type ResolvedTypologyStyle } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { type SpatialEntityFlags } from "@semio-tech/s-spatial-kernel-ts/geometry/paths";
import { objectPrimitiveEntries, resolvePrimitiveRefKind, parseModelJson, Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { listModelObjectsForModelDefinition } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-ownership";
import { type SpatialKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { faceNormal } from "@semio-tech/s-spatial-kernel-ts/preview/face-geometry";
import { type EdgeRecord, scenePreview, type SolidRef, THREE, type VertexRecord, type WireRecord } from "../../🔌️ports/🟦️.ts";
import type { SpatialPickKindToggles, SpatialTypologyToggles } from "../🎯️pick-model/🟦️.ts";
import { buildGeometryObjectIndex, buildGeometryTypologyIndex, geometryBuckets } from "../🧬️geometry-index/🟦️.ts";

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

// #endregion 🎬️WorkerClient
// #region 🧊️CommittedMesh
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
// #endregion 🧊️CommittedMesh

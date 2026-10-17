/** 🧬️ CAD renderer geometry targets: the scene bounds, the geometry buckets and entity points and segments, the object and typology indexes and the creation and filtering of the spatial pick targets. */
import { kernelGeometry, type Vec3 } from "@semio-tech/cad-engine-ts/registry";
import { type SpatialObjectRecord, type ModelEntityKind, type ModelJson } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { isShapeModelDefinition, defaultModelDefinitionId } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { objectPrimaryPrimitiveRef, objectPrimitiveEntries, resolvePrimitiveRefKind, parseModelJson, Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { modelDefinitionUsesGeometryPicking, listModelObjectsForModelDefinition } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-ownership";
import { modelDefinitionSelectionEntityKinds } from "@semio-tech/s-spatial-kernel-ts/geometry/selection-operations";
import { type InteractionEvent } from "@semio-tech/s-spatial-kernel-ts/geometry/selection";
import { type AnchorRecord, type EdgeRecord, type EdgeRef, type FaceRecord, scenePreview, type ShellRecord, type ShellRef, type SolidRecord, type VertexRecord, type WireRecord, type WireRef } from "../../🔌️ports/🟦️.ts";
import { bboxFromPoints, bboxWireSegments } from "../📐️preview-layout/🟦️.ts";
import { kernelGeometryKindForObjectPick, pickTargetPrimitiveKind, type SpatialGeometryPickTargetKind, type SpatialPickGeometry, type SpatialPickKind, spatialPickKindsForSelectionAccept, type SpatialPickKindToggles, type SpatialPickTarget } from "../🎯️pick-model/🟦️.ts";

// #region 🎬️WorkerClient
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
// #region 🧲️GeometryTargets
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

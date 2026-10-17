/** 🎯️ Pick scoring and selection-gesture math, hover-key matching, render-set resolution and the pick-geometry layer with its target nodes.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | renderer}, which re-exports every public name below so import specifiers stay unchanged.
 */

import {
  marqueeCoverageFromGesture,
  reactHostPort,
  type ThreeEvent,
  type ReactNode,
  type ThreeCamera,
  type Ray as ThreeRay
} from "@semio-tech/framework/ui/targets/react";
import { type Vec3 } from "@semio-tech/cad-engine-ts/registry";
import { type ModelEntityKind } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { resolveTypologyStyle } from "@semio-tech/s-spatial-kernel-ts/geometry/typology";
import { type InteractionSpec } from "@semio-tech/s-spatial-kernel-ts/geometry/interaction-spec";
import { defaultModelDefinitionId, type ResolvedTypologyStyle } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { type SpatialEntityFlags } from "@semio-tech/s-spatial-kernel-ts/geometry/paths";
import { objectPrimitiveEntries, resolvePrimitiveRefKind, Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { modelDefinitionUsesGeometryPicking, listModelObjectsForModelDefinition } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-ownership";
import { type InteractionEvent, type SelectionTarget } from "@semio-tech/s-spatial-kernel-ts/geometry/selection";
import { mergeInteractionSpatial } from "../../🎬️actions/🎰️interaction-state/🟦️.ts";
import { isInteractionSessionActive } from "../../🗿️artifact/🏃️interaction-runtime/🟦️.ts";
import { Line, type SolidRef, THREE } from "../🔌️ports/🟦️.ts";
import { spatialSceneColors } from "../🎨️style/🟦️.ts";
import { raycastNone } from "../🧲️geometry-targets/📐️preview-layout/🟦️.ts";
import { filterSpatialPickTargetsForActiveView, filterSpatialPickTargetsForEntityFlags, filterSpatialPickTargetsForVisibility, GEOMETRY_KIND_TO_OBJECT_PICK, kernelGeometryKindForObjectPick, selectionTargetHoverKey, type SpatialDragSelectionState, type SpatialGeometryPickTargetKind, type SpatialPickGeometry, type SpatialPickKindToggles, type SpatialPickTarget, spatialPickTargetKey, type SpatialPickTargetKind, type SpatialSelectionCoverage, type SpatialSelectionMethod, type SpatialSelectionMode, type SpatialSelectionRequest, spatialSelectionTargetKey } from "../🧲️geometry-targets/🎯️pick-model/🟦️.ts";
import { buildGeometryTypologyIndex, collectGeometryEdgeSegments, collectGeometryEdgeSegmentsForMembers, collectSolidPrimitiveMemberIds, createSpatialPickTargets, filterSpatialPickTargets, geometryBuckets, geometryEntityNurbsPoles, geometryEntityWireSegments } from "../🧲️geometry-targets/🧬️geometry-index/🟦️.ts";
import { pinnedPickTargetKeys, spatialHoverKeyAliases, spatialHoverKeysMatch, targetStyle } from "../🧲️geometry-targets/🎨️pick-styling/🟦️.ts";
import { resolveSpatialEntityFlags } from "../🧲️geometry-targets/🧊️mesh-transfers/🟦️.ts";

// #region 🧲️GeometryInteraction
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
};

function targetRayScore(ray: ThreeRay, target: SpatialPickTarget): number | null {
  const points = target.points?.length ? target.points : [target.point];
  const box = new THREE.Box3();
  for (const point of points) box.expandByPoint(new THREE.Vector3(point[0], point[1], point[2]));
  box.expandByScalar(target.kind === "vertex" ? 0.12 : 0.08);
  const hit = ray.intersectBox(box, new THREE.Vector3());
  if (!hit) return null;
  return ray.origin.distanceTo(hit) + spatialPickPriority[target.kind] * 1e-4;
}

export function pointerModifiersFromNativeEvent(event: PointerEvent): InteractionEvent["modifiers"] {
  return {
    alt: event.altKey,
    ctrl: event.ctrlKey,
    meta: event.metaKey,
    shift: event.shiftKey,
  };
}

export function spatialSelectionModeFromModifiers(modifiers: { readonly alt?: boolean; readonly ctrl?: boolean; readonly meta?: boolean; readonly shift?: boolean } = {}): SpatialSelectionMode {
  if (modifiers.shift && modifiers.ctrl) return "invertive";
  if (modifiers.shift) return "additive";
  if (modifiers.ctrl) return "subtractive";
  return "default";
}

export function uniqueSelectionTargets(targets: readonly SelectionTarget[]): SelectionTarget[] {
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

export function mergeSelectionTargets(current: readonly SelectionTarget[], next: readonly SelectionTarget[], mode: SpatialSelectionMode): SelectionTarget[] {
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

export function dragDistance(a: { readonly x: number; readonly y: number }, b: { readonly x: number; readonly y: number }): number {
  return Math.hypot(b.x - a.x, b.y - a.y);
}

export function spatialSelectionCoverageFromGesture(method: SpatialSelectionMethod, path: readonly { readonly x: number; readonly y: number }[]): SpatialSelectionCoverage {
  const start = path[0];
  const end = path[path.length - 1] ?? start;
  return marqueeCoverageFromGesture({ method, startX: start?.x ?? 0, endX: end?.x ?? 0, path });
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

function projectPointToClient(point: Vec3, camera: ThreeCamera, rect: DOMRect): { readonly x: number; readonly y: number } | null {
  const projected = new THREE.Vector3(point[0], point[1], point[2]).project(camera);
  if (!Number.isFinite(projected.x) || !Number.isFinite(projected.y) || !Number.isFinite(projected.z)) return null;
  if (projected.z < -1 || projected.z > 1) return null;
  return {
    x: rect.left + ((projected.x + 1) / 2) * rect.width,
    y: rect.top + ((1 - projected.y) / 2) * rect.height,
  };
}

export function spatialPickTargetsFromClientPoint(
  client: { readonly x: number; readonly y: number },
  camera: ThreeCamera,
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

export function spatialPickTargetsFromScreenSelection(
  drag: SpatialDragSelectionState,
  targets: readonly SpatialPickTarget[],
  camera: ThreeCamera,
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

function spatialPickTargetsFromRay(ray: ThreeRay, targets: readonly SpatialPickTarget[], selectionAccept: readonly ModelEntityKind[], kindToggles: SpatialPickKindToggles): SpatialPickTarget[] {
  return filterSpatialPickTargets(targets, selectionAccept, kindToggles)
    .map((target) => ({ target, score: targetRayScore(ray, target) }))
    .filter((hit): hit is { readonly target: SpatialPickTarget; readonly score: number } => hit.score !== null)
    .sort((a, b) => a.score - b.score)
    .map((hit) => hit.target);
}

/** 🎨️ Resolves per-solid typology display style from model geometry membership. */
export function createSolidTypologyStyleResolver(model: Model, modelDefinitionId: string): (solid: SolidRef) => ResolvedTypologyStyle | undefined {
  const index = buildGeometryTypologyIndex(model, modelDefinitionId);
  return (solid) => {
    const typology = index.get(`solid:${solid}`);
    return typology ? resolveTypologyStyle(typology) : undefined;
  };
}

export function selectionTargetPickKind(target: SelectionTarget): SpatialPickTargetKind | null {
  if (target.kind === "object") return "object";
  return GEOMETRY_KIND_TO_OBJECT_PICK[target.kind] ?? null;
}

/** 🪪️ Stable factory-geometry member key for object-reveal lookup. */
export function spatialPickTargetMemberKey(target: SpatialPickTarget): string {
  if (target.kind === "object" && !target.geometryKind) return `object:${target.id}`;
  const geometryKind = target.geometryKind ?? kernelGeometryKindForObjectPick(target.kind as SpatialGeometryPickTargetKind, target.geometryKind);
  return `${geometryKind}:${target.id}`;
}

/** 👁️ Object ids whose factory primitives should draw (hover/selection on object or its topology). */
export function revealedObjectIdsFromPickKeys(objectIndex: ReadonlyMap<string, string>, hoveredTargetKey: string | null | undefined, selectedTargetKeys: ReadonlySet<string> = new Set()): ReadonlySet<string> {
  const revealed = new Set<string>();
  const consider = (key: string | null | undefined): void => {
    if (!key) return;
    for (const alias of spatialHoverKeyAliases(key)) {
      const owner = objectIndex.get(alias);
      if (owner) revealed.add(owner);
    }
  };
  consider(hoveredTargetKey);
  for (const key of selectedTargetKeys) consider(key);
  return revealed;
}

function spatialPickTargetObjectRevealed(target: SpatialPickTarget, objectIndex: ReadonlyMap<string, string>, revealedObjectIds: ReadonlySet<string>): boolean {
  if (target.kind === "object" && !target.geometryKind) return true;
  const ownerId = objectIndex.get(spatialPickTargetMemberKey(target));
  return ownerId !== undefined && revealedObjectIds.has(ownerId);
}

/** 🖱️ Maps a document {@link SelectionTarget} to the canvas hover key (typology object when possible). */
export function canvasHoverKeyForSelectionTarget(model: Model, modelDefinitionId: string, target: SelectionTarget): string {
  if (target.kind === "object") return selectionTargetHoverKey(target);
  for (const row of listModelObjectsForModelDefinition(model, modelDefinitionId)) {
    for (const [, primitiveRef] of objectPrimitiveEntries(row)) {
      const kind = resolvePrimitiveRefKind(model, primitiveRef) ?? "solid";
      if (target.kind === kind && target.id === String(primitiveRef)) {
        return selectionTargetHoverKey({ kind: "object", id: String(row.id), editable: false });
      }
      if (kind === "solid") {
        const members = collectSolidPrimitiveMemberIds(geometryBuckets(model), String(primitiveRef));
        if (members.has(`${target.kind}:${target.id}`)) {
          return selectionTargetHoverKey({ kind: "object", id: String(row.id), editable: false });
        }
      }
    }
  }
  return selectionTargetHoverKey(target);
}

export function spatialSelectionTarget(target: SpatialPickTarget): SelectionTarget {
  if (target.kind === "object" && !target.geometryKind) {
    return { kind: "object", id: target.id, editable: false };
  }
  const geometryKind = kernelGeometryKindForObjectPick(target.kind as SpatialGeometryPickTargetKind, target.geometryKind);
  return { kind: geometryKind, id: target.id, editable: true };
}

/** 🎯️ Host geometry picking when browse is idle, session finished, or interaction defers picks (`pickDisabledStates`). */
export function replHostGeometryPickingEnabled(interactionId: string, spec: InteractionSpec, state: string): boolean {
  if (!interactionId) return true;
  if (!isInteractionSessionActive(spec, state)) return true;
  return mergeInteractionSpatial(spec).pickDisabledStates.includes(state);
}

/** 👁️ Pick-target overlay visible whenever the active model definition uses factory geometry picking. */
export function replGeometryPickLayerVisible(modelDefinitionId: string | null): boolean {
  return modelDefinitionUsesGeometryPicking(modelDefinitionId ?? defaultModelDefinitionId());
}

/** 🖱️ Returns the closest pick target eligible for hover highlighting along a ray. */
export function pickHoverTargetFromRay(ray: ThreeRay, targets: readonly SpatialPickTarget[], hoverKindToggles: SpatialPickKindToggles = {}): SpatialPickTarget | null {
  return spatialPickTargetsFromRay(ray, targets, [], hoverKindToggles)[0] ?? null;
}

/** 📌️ Renders visibility-enabled pick highlights plus pinned hover/selection targets. */
export function resolveSpatialPickTargetsToRender(
  viewTargets: readonly SpatialPickTarget[],
  filterKindToggles: SpatialPickKindToggles = {},
  pinnedTargetKeys: ReadonlySet<string> = new Set(),
  flagsForId: (entityId: string) => SpatialEntityFlags = () => ({}),
  objectIndex?: ReadonlyMap<string, string>,
  revealedObjectIds?: ReadonlySet<string>,
): SpatialPickTarget[] {
  const pinnedKeys = pinnedPickTargetKeys(pinnedTargetKeys);
  const enabledTargets = filterSpatialPickTargetsForVisibility(viewTargets, filterKindToggles);
  const seen = new Set<string>();
  const out: SpatialPickTarget[] = [];
  const objectRevealActive = objectIndex !== undefined && revealedObjectIds !== undefined;
  for (const target of enabledTargets) {
    const key = spatialPickTargetKey(target);
    const flags = flagsForId(target.id);
    if (flags.hidden === true && !pinnedKeys.has(key)) {
      continue;
    }
    if (objectRevealActive && !pinnedKeys.has(key) && !spatialPickTargetObjectRevealed(target, objectIndex, revealedObjectIds)) {
      continue;
    }
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

/** 👁️ Visual-only pick-target highlight; hit-testing is handled by `SpatialPickRayCatcher`. */
export function SpatialPickTargetNode({
  target,
  geometry = null,
  geometryPreviewTransform = null,
  hoveredTargetKey,
  selectedTargetKey,
  selectedTargetKeys,
  entityFlagsForId,
}: {
  readonly target: SpatialPickTarget;
  readonly geometry?: SpatialPickGeometry | null;
  readonly geometryPreviewTransform?: ((point: Vec3) => Vec3) | null;
  readonly hoveredTargetKey?: string | null;
  readonly selectedTargetKey?: string | null;
  readonly selectedTargetKeys?: ReadonlySet<string> | null;
  readonly entityFlagsForId?: (entityId: string) => SpatialEntityFlags;
}): ReactNode {
  const mapPt = geometryPreviewTransform ?? ((p: Vec3) => p);
  const displayPoint = mapPt(target.point);
  const displayPoints = target.points?.map(mapPt);
  const targetKey = spatialPickTargetKey(target);
  const hovered = spatialHoverKeysMatch(hoveredTargetKey, targetKey);
  const selected = selectedTargetKeys ? [...selectedTargetKeys].some((key) => spatialHoverKeysMatch(key, targetKey)) : spatialHoverKeysMatch(selectedTargetKey, targetKey);
  const entityFlags = entityFlagsForId?.(target.id) ?? {};
  const typologyStyle = target.typologyId ? resolveTypologyStyle(target.typologyId) : undefined;
  const style = targetStyle(target, hovered, selected, typologyStyle, entityFlags.locked === true);
  const userData = { spatialPickKey: targetKey };
  const wireKind = target.kind === "vertex" ? null : (target.geometryKind ?? (target.kind === "edge" ? "edge" : null));
  const wireSegments = reactHostPort.useMemo(() => {
    if (!geometry || !wireKind) return [] as readonly (readonly [Vec3, Vec3])[];
    return geometryEntityWireSegments(geometryBuckets(geometry), wireKind, target.id);
  }, [geometry, wireKind, target.id]);
  const nurbsPoles = reactHostPort.useMemo(() => {
    if (!geometry || !wireKind) return [] as readonly Vec3[];
    return geometryEntityNurbsPoles(geometryBuckets(geometry), wireKind, target.id);
  }, [geometry, wireKind, target.id]);
  if (target.kind === "vertex") {
    return (
      <mesh position={displayPoint} userData={userData} raycast={raycastNone} renderOrder={8}>
        <sphereGeometry args={[selected || hovered ? 0.12 : 0.085, 16, 16]} />
        <meshStandardMaterial color={style.color} emissive={style.emissive} emissiveIntensity={0.45} depthTest={false} transparent />
      </mesh>
    );
  }
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
        {(selected || hovered) && nurbsPoles.length > 0
          ? nurbsPoles.map((pt, i) => {
              const p = mapPt(pt);
              return (
                <mesh key={`nurbs-pole-${i}`} position={[p[0], p[1], p[2]]} raycast={raycastNone} renderOrder={9}>
                  <sphereGeometry args={[0.04, 10, 10]} />
                  <meshStandardMaterial color={palette.construction} emissive={palette.constructionEmissive} emissiveIntensity={0.35} depthTest={false} />
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

/** 🧵️ Draws all geometry edges for imported factory geometry (one batched `lineSegments`). */
export function GeometryFactoryWireframeLayer({ geometry, visible = true, revealedMemberKeys }: { readonly geometry?: SpatialPickGeometry | null; readonly visible?: boolean; readonly revealedMemberKeys?: ReadonlySet<string> | null }): ReactNode {
  const segments = reactHostPort.useMemo(() => {
    if (!geometry) return [] as readonly (readonly [Vec3, Vec3])[];
    const buckets = geometryBuckets(geometry);
    if (revealedMemberKeys === null || revealedMemberKeys === undefined) return collectGeometryEdgeSegments(buckets);
    return collectGeometryEdgeSegmentsForMembers(buckets, revealedMemberKeys);
  }, [geometry, revealedMemberKeys]);
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

/** 🧲️ Renders optional factory geometry as pickable snap/select targets. */

//#region 🧲️SpatialPickGeometryLayer
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
  entityFlagsForId,
}: {
  readonly geometry?: SpatialPickGeometry | null;
  readonly activeModelDefinitionId?: string | null;
  readonly modelDefinitionRevision?: number;
  readonly geometryPreviewTransform?: ((point: Vec3) => Vec3) | null;
  readonly selectionAccept?: readonly ModelEntityKind[];
  readonly selectionKindToggles?: SpatialPickKindToggles;
  /** 👁️ Which kinds are drawn as pick-target highlights (independent of selection). */
  readonly filterKindToggles?: SpatialPickKindToggles;
  readonly hoveredTargetKey?: string | null;
  readonly selectedTargetKey?: string | null;
  readonly selectedTargetKeys?: ReadonlySet<string> | null;
  readonly hostSelectionEnabled?: boolean;
  readonly onSelectionRequest?: (request: SpatialSelectionRequest) => void;
  readonly entityFlagsForId?: (entityId: string) => SpatialEntityFlags;
}): ReactNode {
  const modelRevision = geometry && typeof geometry === "object" && "revision" in geometry ? Number((geometry as { revision?: unknown }).revision) : 0;
  const resolvedEntityFlagsForId = reactHostPort.useCallback(
    (entityId: string) => entityFlagsForId?.(entityId) ?? (geometry && typeof geometry === "object" && "metadata" in geometry ? resolveSpatialEntityFlags(geometry as Model, activeModelDefinitionId ?? defaultModelDefinitionId(), entityId) : {}),
    [activeModelDefinitionId, entityFlagsForId, geometry, modelRevision],
  );
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
    return resolveSpatialPickTargetsToRender(viewTargets, filterKindToggles, pinnedTargetKeys, resolvedEntityFlagsForId);
  }, [viewTargets, filterKindToggles, pinnedTargetKeys, resolvedEntityFlagsForId]);
  const selectableTargets = reactHostPort.useMemo(
    () => filterSpatialPickTargetsForEntityFlags(filterSpatialPickTargets(viewTargets, selectionAccept, selectionKindToggles), resolvedEntityFlagsForId),
    [viewTargets, selectionAccept, selectionKindToggles, resolvedEntityFlagsForId],
  );
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
          entityFlagsForId={resolvedEntityFlagsForId}
        />
      ))}
      {hostSelectionEnabled && onSelectionRequest
        ? selectableTargets.map((target) => <SpatialPickHitTarget key={`hit:${target.kind}:${target.id}`} target={target} geometryPreviewTransform={geometryPreviewTransform} onPick={requestSelection} />)
        : null}
    </group>
  );
}

/** 🖱️ Invisible pick proxy for a spatial target (visual highlight is on {@link SpatialPickTargetNode}). */
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
//#endregion 🧲️SpatialPickGeometryLayer
// #endregion 🧲️GeometryInteraction

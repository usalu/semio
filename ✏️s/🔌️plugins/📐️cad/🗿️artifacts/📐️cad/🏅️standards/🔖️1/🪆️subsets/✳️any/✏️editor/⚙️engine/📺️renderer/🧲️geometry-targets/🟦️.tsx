/** 🧲️ CAD renderer geometry targets: the archived box footprints and the committed mesh layers (tessellated commit mesh, factory face surface and committed mesh layer) built on the pick, layout, styling and mesh transfer children.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | renderer}, which re-exports every public name below so import specifiers stay unchanged.
 */
// #region 🔌️Adapters
import { reactHostPort, type ThreeEvent, type ReactNode, type BufferGeometry as ThreeBufferGeometry } from "@semio-tech/framework/ui/targets/react";

// #endregion 🔌️Adapters
import { createTemplatePool, ViewRadiusLayer } from "@semio-tech/infinite-world-r3f";
import { type FaceInfo, type MeshTransfer, type Vec3 } from "@semio-tech/cad-engine-ts/registry";
import { type ResolvedTypologyStyle } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { isEmptyModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { type DisplayItem, type DisplayModel } from "../../🎬️actions/🟦️.ts";
import { DocumentHistory, type InteractionSnapshot } from "../../🗿️artifact/📨️interaction-model/🟦️.ts";
import { type SolidRef, THREE } from "../🔌️ports/🟦️.ts";
import { spatialSceneColors } from "../🎨️style/🟦️.ts";
import { isVec3Record, raycastNone } from "./📐️preview-layout/🟦️.ts";
import { createTypologyStyledMaterial, spatialHoverKeysMatch } from "./🎨️pick-styling/🟦️.ts";
import { buildBufferGeometryFromMeshTransfer, type FactoryFaceMeshRow, findFaceGroupAt, listModelSolidRefs, meshTransferContentKey, meshTransferOrigin, visibleFaceRefsForModelDefinition } from "./🧊️mesh-transfers/🟦️.ts";

// #region 🪩️ArchivedFootprints
/** 📦️ Footprint of a finished axis-aligned box for persistent REPL overlays. */
export interface ArchivedBoxLayout {
  readonly cornerA: Vec3;
  readonly cornerB: Vec3;
  readonly height: number;
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
// #region 🧊️CommittedMesh
const CAD_WORLD_CHUNK_SIZE = 256;
const CAD_WORLD_MAX_DISTANCE = 8000;
const cadMeshGeometryPool = createTemplatePool<string, ThreeBufferGeometry>();

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

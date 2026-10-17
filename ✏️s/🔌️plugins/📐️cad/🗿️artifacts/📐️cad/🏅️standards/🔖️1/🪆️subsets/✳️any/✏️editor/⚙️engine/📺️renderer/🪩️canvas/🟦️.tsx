/** 🪩️ Host-state hooks, the CAD world coordinate system, auto-fit, the interaction canvas and the interaction spatial view.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | renderer}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import {
  reactHostPort,
  type ThreeEvent,
  type CSSProperties,
  type ReactMouseEvent,
  type ReactNode,
  type ThreeCamera,
  type ThreeScene,
  type Vector3 as ThreeVector3
} from "@semio-tech/framework/ui/targets/react";
// #endregion 🔌️Adapters
import {
  DEFAULT_LOD_GRID_FACTOR,
  DEFAULT_MANUAL_LOD,
  WorldCameraInvalidator,
  WorldCanvas,
  WorldLayer,
  WorldLodBridge,
  WorldLodGridHelper,
  WorldOrbitCameraViewApplier,
  WorldOrbitGated,
  WorldOrbitViewControls,
  WorldOrbitViewSnapGateProvider,
  orbitCameraViewGumballPlane,
  type OrbitCameraProjection,
  type WorldProjectionSpec,
  WorldReferenceLayer,
  type WorldReferenceProps,
  type WorldReferenceRelocatePayload,
  type OrbitCameraViewId,
  type WorldCanvasProps
} from "@semio-tech/infinite-world-r3f";
import { solidRef, type FaceInfo, type MeshTransfer, type Vec3 } from "@semio-tech/cad-engine-ts/registry";
import { type ModelEntityKind } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { type InteractionSpec } from "@semio-tech/s-spatial-kernel-ts/geometry/interaction-spec";
import { defaultModelDefinitionId } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { type InteractionEvent, type SelectionTarget } from "@semio-tech/s-spatial-kernel-ts/geometry/selection";
import { type SpatialPreviewKernel, type ModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { type DisplayModel } from "../../🎬️actions/🟦️.ts";
import { cadGumballConfigVisible, type CadGumballConfig } from "../../🎬️actions/🧲️selection-targets/🟦️.ts";
import { type InteractionRuntimeOptions, type InteractionSnapshot } from "../../🗿️artifact/📨️interaction-model/🟦️.ts";
import { createInteractionRuntime, type InteractionRuntime } from "../../🗿️artifact/🏃️interaction-runtime/🟦️.ts";
import { bindScenePreviewKernel, r3fPreviewKernel, type SolidRef, useThree } from "../🔌️ports/🟦️.ts";
import { spatialSceneColors } from "../🎨️style/🟦️.ts";
import { CommittedMeshLayer, FactoryFaceSurfaceLayer } from "../🧲️geometry-targets/🟦️.tsx";
import { geometryPreviewTransformFromDisplay } from "../🧲️geometry-targets/📐️preview-layout/🟦️.ts";
import { resolveSpatialSceneVisibility, type SpatialPickGeometry, type SpatialPickKindToggles, type SpatialSelectionRequest } from "../🧲️geometry-targets/🎯️pick-model/🟦️.ts";
import { boundsFromSpatialPickGeometry, createSpatialPickEvent, mergeSpatialSceneBounds } from "../🧲️geometry-targets/🧬️geometry-index/🟦️.ts";
import { boundsFromMeshTransfers, type FactoryFaceMeshRow, meshTransferContentKey } from "../🧲️geometry-targets/🧊️mesh-transfers/🟦️.ts";
import { InteractionDisplay, type SpatialDisplayItemRenderer } from "../🖼️display/🟦️.tsx";
import {
  defaultInteractionSpatialViewTheme,
  GroundPickPlane,
  HeightDragSurface,
  type InteractionSpatialViewSlots,
  type InteractionSpatialViewTheme,
  SpatialConstrainedPointerBridge,
  SpatialTransformGumball,
  vec3FromSnapshotContext,
  VerticalZDragRod
} from "../🔖️interaction/🟦️.tsx";
import { createSolidTypologyStyleResolver, GeometryFactoryWireframeLayer, SpatialPickGeometryLayer } from "../🎯️pick-layer/🟦️.tsx";

// #region 🪝️Hooks
/** 🪝️ Memoized `createInteractionRuntime` for React hosts. */
export function useInteractionRuntime(spec: InteractionSpec, opts: InteractionRuntimeOptions): InteractionRuntime {
  const specId = spec.id;
  return reactHostPort.useMemo(() => createInteractionRuntime(spec, opts), [spec, specId, opts]);
}

/** 🪝️ Subscribes to `InteractionRuntime` revision updates for React hosts. */
export function useInteractionSnapshot(rt: InteractionRuntime): InteractionSnapshot {
  return reactHostPort.useSyncExternalStore(
    (cb) => rt.subscribe(cb),
    () => rt.getSnapshot(),
    () => rt.getSnapshot(),
  );
}

/** 🎛️ Resolves functional or literal host-state updates (testable without React). */
export function resolveHostStateNext<T>(value: T, next: T | ((prev: T) => T)): T {
  return typeof next === "function" ? (next as (prev: T) => T)(value) : next;
}

/** 🎛️ Controlled-or-uncontrolled state slice for embeddable spatial hosts. */
export function useHostState<T>(controlled: T | undefined, onChange: ((value: T) => void) | undefined, initial: T | (() => T)): readonly [T, (next: T | ((prev: T) => T)) => void] {
  const [internal, setInternal] = reactHostPort.useState(initial);
  const isControlled = controlled !== undefined;
  const value = isControlled ? controlled : internal;
  const valueRef = reactHostPort.useRef(value);
  valueRef.current = value;
  const setValue = reactHostPort.useCallback(
    (next: T | ((prev: T) => T)) => {
      if (isControlled) {
        const resolved = resolveHostStateNext(valueRef.current, next);
        valueRef.current = resolved;
        onChange?.(resolved);
        return;
      }
      setInternal((prev) => {
        const resolved = resolveHostStateNext(prev, next);
        valueRef.current = resolved;
        onChange?.(resolved);
        return resolved;
      });
    },
    [isControlled, onChange],
  );
  return [value, setValue] as const;
}
// #endregion 🪝️Hooks

// #region 🪩️Canvas
export const CAD_WORLD_FORWARD: Vec3 = [0, 1, 0];

export const CAD_WORLD_UP: Vec3 = [0, 0, 1];

/** 🧭️ Overrides Three.js' Y-up defaults with the CAD world frame. */
export function applyCadWorldCoordinateSystem(camera: ThreeCamera, scene: ThreeScene): void {
  camera.up.set(CAD_WORLD_UP[0], CAD_WORLD_UP[1], CAD_WORLD_UP[2]);
  scene.up.set(CAD_WORLD_UP[0], CAD_WORLD_UP[1], CAD_WORLD_UP[2]);
}

function InteractionCadWorldCoordinateSystem(): null {
  const { camera, scene } = useThree();
  reactHostPort.useLayoutEffect(() => applyCadWorldCoordinateSystem(camera, scene), [camera, scene]);
  return null;
}

export interface InteractionCanvasProps {
  readonly children: ReactNode;
  readonly onCanvasReady?: (binding: { readonly camera: ThreeCamera; readonly domElement: HTMLCanvasElement }) => void;
  /** 🎞️ `always` while an interaction session runs; `demand` when idle for GPU savings. */
  readonly frameloop?: "always" | "demand";
  readonly background?: string;
  /** 📷️ When true, children own the default camera (display view templates); omit canvas-owned perspective. */
  readonly managedCamera?: boolean;
  readonly cameraPosition?: Vec3;
  readonly cameraFov?: number;
  readonly cameraNear?: number;
  readonly cameraFar?: number;
  readonly dpr?: number | [number, number];
  readonly shadows?: boolean | "basic" | "percentage" | "soft" | "variance";
  readonly style?: CSSProperties;
  readonly className?: string;
  readonly gl?: WorldCanvasProps["gl"];
  readonly onPointerDown?: (event: PointerEvent) => void;
  readonly onPointerMove?: (event: PointerEvent) => void;
  readonly onPointerUp?: (event: PointerEvent) => void;
  readonly onPointerLeave?: (event: PointerEvent) => void;
  readonly onPointerCancel?: (event: PointerEvent) => void;
  readonly onWheel?: (event: WheelEvent) => void;
  readonly onContextMenu?: (event: ReactMouseEvent) => void;
  readonly onDoubleClick?: (event: MouseEvent) => void;
  readonly onLostPointerCapture?: (event: PointerEvent) => void;
  readonly overlay?: ReactNode;
}

/** 📡️ Host event callbacks accepted by {@link InteractionCanvas}. */
export type InteractionCanvasHostCallbacks = Pick<
  InteractionCanvasProps,
  "onCanvasReady" | "onPointerDown" | "onPointerMove" | "onPointerUp" | "onPointerLeave" | "onPointerCancel" | "onWheel" | "onContextMenu" | "onDoubleClick" | "onLostPointerCapture"
>;

export type SpatialAutoFitBehavior = "initial" | "changes";

export function spatialAutoFitShouldRun(behavior: SpatialAutoFitBehavior, key: string, lastKey: string, hasApplied: boolean): boolean {
  if (!key || key === lastKey) return false;
  return behavior === "changes" || !hasApplied;
}

/** 🛰️ Frames the camera to fit committed meshes and/or factory geometry (playground auto-fit). */
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

export function applySpatialAutoFitCamera(camera: ThreeCamera, bounds: { readonly center: Vec3; readonly radius: number }, padding = 1.25, controls?: unknown): void {
  const [cx, cy, cz] = bounds.center;
  const dist = Math.max(bounds.radius * padding, 2);
  camera.position.set(cx + dist, cy + dist, cz + dist * 0.85);
  const orbit = controls as { readonly target?: ThreeVector3; update?: () => void } | undefined;
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

/** 🧊️ Invalidates the canvas when committed meshes first become available (demand frameloop / async tessellation). */
function CommittedMeshesReadyInvalidate({ meshes }: { readonly meshes: readonly MeshTransfer[] }): null {
  const invalidate = useThree((state) => state.invalidate);
  const prevCount = reactHostPort.useRef(0);
  reactHostPort.useEffect(() => {
    if (meshes.length > 0 && prevCount.current === 0) {
      invalidate();
    }
    prevCount.current = meshes.length;
  }, [invalidate, meshes]);
  return null;
}

/** 🔄️ Invalidates demand frameloop when host-driven scene visuals change. */
function InvalidateOnRevision({ revision }: { readonly revision: string | number }): null {
  const invalidate = useThree((state) => state.invalidate);
  reactHostPort.useEffect(() => {
    invalidate();
  }, [revision, invalidate]);
  return null;
}

/** 🎯️ Redraws when host selection pick keys change (demand frameloop). */
export function InteractionSelectionInvalidateBridge({ selectionKey }: { readonly selectionKey: string }): null {
  const invalidate = useThree((state) => state.invalidate);
  reactHostPort.useEffect(() => {
    invalidate();
  }, [selectionKey, invalidate]);
  return null;
}

/** 🪩️ Root infinite-world canvas for factory viewports ({@link WorldCanvas}, z-up). */
export function InteractionCanvas({
  children,
  onCanvasReady,
  frameloop = "demand",
  background = defaultInteractionSpatialViewTheme.background,
  managedCamera = false,
  cameraPosition = managedCamera ? undefined : [10, 10, 8],
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
  overlay,
}: InteractionCanvasProps): ReactNode {
  return (
    <WorldCanvas
      frameloop={frameloop}
      className={className}
      style={{ height: "100%", width: "100%", ...style }}
      dpr={dpr}
      shadows={shadows}
      cameraUp={CAD_WORLD_UP}
      cameraPosition={cameraPosition}
      cameraFov={cameraFov}
      cameraNear={cameraNear}
      cameraFar={cameraFar}
      background={background}
      gl={gl}
      onCanvasReady={onCanvasReady}
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={onPointerUp}
      onPointerLeave={onPointerLeave}
      onPointerCancel={onPointerCancel}
      onWheel={onWheel}
      onContextMenu={onContextMenu}
      onDoubleClick={onDoubleClick}
      onLostPointerCapture={onLostPointerCapture}
      overlay={overlay}
    >
      <InteractionCadWorldCoordinateSystem />
      {children}
    </WorldCanvas>
  );
}

export interface InteractionSpatialViewProps {
  readonly previewKernel?: SpatialPreviewKernel;
  readonly snapshot: InteractionSnapshot;
  readonly onGroundPick?: (point: Vec3, event: InteractionEvent) => void;
  /** 🖱️ `pointer.move` hits ground (XY at fixed Z); height slab passes full 3D. */
  readonly onScenePointerMove?: (point: Vec3, event: InteractionEvent) => void;
  readonly onInteractionEvent?: (event: InteractionEvent) => void;
  readonly pickEnabled?: boolean;
  readonly committedMesh?: MeshTransfer | null;
  readonly committedMeshes?: readonly { readonly solid: SolidRef; readonly mesh: MeshTransfer }[];
  readonly factoryFaceMeshes?: readonly FactoryFaceMeshRow[];
  readonly geometry?: SpatialPickGeometry | null;
  /** 🧲️ Pick-target source; defaults to `geometry` (use spatial.shape geometry when the active model is typology-only). */
  readonly pickGeometry?: SpatialPickGeometry | null;
  readonly activeModelDefinitionId?: string | null;
  readonly modelDefinitionRevision?: number;
  /** 🖼️ When set, drives `InteractionDisplay` instead of `snapshot.display` (e.g. merged archived footprints). */
  readonly displayModel?: DisplayModel;
  readonly renderDisplayItem?: SpatialDisplayItemRenderer;
  readonly selectionAccept?: readonly ModelEntityKind[];
  readonly filterKindToggles?: SpatialPickKindToggles;
  /** 👁️ Committed mesh / factory wireframe visibility; defaults to {@link spatialSceneKindTogglesForModelDefinition}. */
  readonly sceneKindToggles?: SpatialPickKindToggles;
  readonly selectionKindToggles?: SpatialPickKindToggles;
  /** 🖱️ Hover raycast kind filter; defaults to `selectionKindToggles` when omitted. */
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
  /** 🧲️ When false, skips pick-target meshes (during active interaction sessions). */
  readonly showPickLayer?: boolean;
  readonly committedMeshPickable?: boolean;
  readonly autoFitMeshes?: boolean;
  readonly autoFitBehavior?: SpatialAutoFitBehavior;
  readonly theme?: InteractionSpatialViewTheme;
  readonly slots?: InteractionSpatialViewSlots;
  /** 🎛️ When set with targets, shows a gumball at the selection centroid (utility bar move/rotate/scale). */
  readonly transformGumballConfig?: CadGumballConfig | null;
  readonly transformGumballTargets?: readonly SelectionTarget[];
  readonly onTransformGumballCommit?: (diff: ModelDiff) => void;
  readonly onTransformGumballPreview?: (diff: ModelDiff) => void;
  readonly onTransformGumballPreviewEnd?: () => void;
  readonly transformGumballModel?: Model | null;
  readonly cameraView?: OrbitCameraViewId;
  readonly cameraViewSeedKey?: string | number;
  readonly orbitProjection?: OrbitCameraProjection;
  readonly projectionSpec?: WorldProjectionSpec;
  readonly externalPendingProjectionSpec?: WorldProjectionSpec | null;
  readonly onExternalPendingProjectionSpecClear?: () => void;
  readonly showOrbitViewGizmo?: boolean;
  readonly onOrbitProjectionChange?: (projection: OrbitCameraProjection) => void;
  readonly onProjectionSpecChange?: (spec: WorldProjectionSpec) => void;
  readonly onOrbitCameraChange?: (state: import("@semio-tech/infinite-world-r3f").WorldCameraState) => void;
  /** 🖼️ Grid reference planes persisted beside the CAD model. */
  readonly worldReferences?: readonly WorldReferenceProps[];
  readonly selectedReferenceIds?: ReadonlySet<string>;
  readonly hoveredReferenceId?: string | null;
  readonly revealedReferenceIds?: ReadonlySet<string>;
  readonly referenceRelocateActive?: boolean;
  readonly onReferenceSelect?: (id: string, modifiers: { readonly shiftKey: boolean; readonly ctrlKey: boolean; readonly metaKey: boolean }) => void;
  readonly onReferenceHover?: (id: string | null) => void;
  readonly onReferenceRelocate?: (payload: WorldReferenceRelocatePayload) => void;
}

/** 📡️ Host event callbacks accepted by {@link InteractionSpatialView}. */
export type InteractionSpatialViewHostCallbacks = Pick<
  InteractionSpatialViewProps,
  "onGroundPick" | "onScenePointerMove" | "onInteractionEvent" | "onSelectionRequest" | "onCameraNavigate" | "onCommittedFacePointerDown" | "onCommittedFacePointerMove" | "onSnapshotStateChange" | "onSnapshotRevisionChange" | "onPickEnabledChange"
>;

/** 🖱️ Ground-plane picking is action input and must stay independent from host geometry selection. */
export function interactionSpatialGroundPickPlaneEnabled(snapshot: Pick<InteractionSnapshot, "spatialInteraction" | "state">, pickEnabled: boolean): boolean {
  const si = snapshot.spatialInteraction;
  return pickEnabled !== false && si.spatialGroundPick && !si.pickDisabledStates.includes(snapshot.state);
}

/** 🪩️ Lights, orbit controls, ground picking, factory overlays, optional committed mesh. */
export function InteractionSpatialView({
  previewKernel = r3fPreviewKernel,
  snapshot,
  onGroundPick,
  onScenePointerMove,
  onInteractionEvent,
  pickEnabled = true,
  committedMesh,
  committedMeshes,
  factoryFaceMeshes = [],
  geometry,
  pickGeometry: pickGeometryProp,
  activeModelDefinitionId = defaultModelDefinitionId(),
  modelDefinitionRevision = 0,
  displayModel,
  renderDisplayItem,
  selectionAccept = [],
  filterKindToggles = {},
  sceneKindToggles,
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
  transformGumballConfig = null,
  transformGumballTargets = [],
  onTransformGumballCommit,
  onTransformGumballPreview,
  onTransformGumballPreviewEnd,
  transformGumballModel = null,
  cameraView,
  cameraViewSeedKey,
  orbitProjection,
  projectionSpec,
  externalPendingProjectionSpec,
  onExternalPendingProjectionSpecClear,
  showOrbitViewGizmo = true,
  onOrbitProjectionChange,
  onProjectionSpecChange,
  onOrbitCameraChange,
  worldReferences = [],
  selectedReferenceIds,
  hoveredReferenceId = null,
  revealedReferenceIds,
  referenceRelocateActive = true,
  onReferenceSelect,
  onReferenceHover,
  onReferenceRelocate,
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
  const projectionGumballConfig = reactHostPort.useMemo(() => {
    if (!transformGumballConfig) return null;
    const plane = orbitCameraViewGumballPlane(cameraView);
    return plane ? { ...transformGumballConfig, plane } : transformGumballConfig;
  }, [transformGumballConfig, cameraView]);
  const resolvedTheme = { ...defaultInteractionSpatialViewTheme, ...theme };
  const cadLodRef = reactHostPort.useRef(DEFAULT_MANUAL_LOD);
  const layerMeshes = reactHostPort.useMemo(() => {
    if (committedMeshes?.length) return committedMeshes;
    if (committedMesh) return [{ solid: solidRef("committed"), mesh: committedMesh }];
    return [];
  }, [committedMeshes, committedMesh]);
  const autoFitSources = reactHostPort.useMemo(() => [...layerMeshes.map((row) => row.mesh), ...factoryFaceMeshes.map((row) => row.mesh)], [factoryFaceMeshes, layerMeshes]);
  const ctx = snapshot.context;
  const geometryPreviewTransform = reactHostPort.useMemo(() => geometryPreviewTransformFromDisplay(displayModel ?? snapshot.display), [displayModel, snapshot.display]);
  const origin = vec3FromSnapshotContext(ctx, "origin") ?? vec3FromSnapshotContext(ctx, "prevPoint") ?? vec3FromSnapshotContext(ctx, "pointA");
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
  const sceneVisibility = reactHostPort.useMemo(() => resolveSpatialSceneVisibility(activeModelDefinitionId, sceneKindToggles ?? filterKindToggles), [activeModelDefinitionId, sceneKindToggles, filterKindToggles]);
  const scenePickGeometry = geometry ?? pickGeometryProp;
  const pickGeometryRevision = scenePickGeometry && typeof scenePickGeometry === "object" && "revision" in scenePickGeometry ? Number((scenePickGeometry as { revision?: unknown }).revision) : 0;
  const styleForSolid = reactHostPort.useMemo(() => {
    if (!geometry || typeof geometry !== "object" || !("objects" in geometry)) return undefined;
    return createSolidTypologyStyleResolver(geometry as Model, activeModelDefinitionId ?? defaultModelDefinitionId());
  }, [geometry, geometryRevision, modelDefinitionRevision, activeModelDefinitionId]);
  return (
    <>
      {slots?.beforeScene}
      <InvalidateOnRevision
        revision={`${snapshot.revision}:${modelDefinitionRevision}:${geometryRevision}:${pickGeometryRevision}:${layerMeshes.map((row, i) => `${row.solid}:${meshTransferContentKey(row.mesh, i)}`).join("|")}:${hoveredTargetKey ?? ""}:${selectedTargetKey ?? ""}:${selectedTargetKeys?.size ?? 0}`}
      />
      <WorldCameraInvalidator />
      {autoFitMeshes ? (
        <>
          <CommittedMeshesReadyInvalidate meshes={autoFitSources} />
          <SpatialAutoFit meshes={autoFitSources} geometry={geometry} behavior={autoFitBehavior} />
        </>
      ) : null}
      <WorldLodBridge lodRef={cadLodRef} distanceReference={100} gridFactor={DEFAULT_LOD_GRID_FACTOR} gridSnapEnabled={false} showLodGrid={false} automaticLod={false} depthVariableLod={false} manualLod={DEFAULT_MANUAL_LOD} gridDatum={[0, 0, 0]}>
        <WorldOrbitViewSnapGateProvider>
          {slots?.environment}
          {slots?.lights ?? (
            <>
              <ambientLight intensity={resolvedTheme.ambientIntensity ?? 0.45} />
              <directionalLight position={dirPos} intensity={resolvedTheme.directionalIntensity ?? 1.1} />
            </>
          )}
          {cameraView !== undefined && cameraViewSeedKey !== undefined ? <WorldOrbitCameraViewApplier view={cameraView} seedKey={cameraViewSeedKey} projectionOverride={orbitProjection} /> : null}
          <WorldOrbitGated controlsKey={cameraViewSeedKey ?? "default"} onCameraNavigate={onCameraNavigate} projection={orbitProjection} />
          {showOrbitViewGizmo ? (
            <WorldOrbitViewControls
              projectionSpec={projectionSpec}
              externalPendingSpec={externalPendingProjectionSpec}
              onExternalPendingSpecClear={onExternalPendingProjectionSpecClear}
              onCameraChange={onOrbitCameraChange}
            />
          ) : null}
          <WorldLayer order={0} name="cad.grid">
            <WorldLodGridHelper gridDatum={[0, 0, 0]} />
          </WorldLayer>
          <WorldLayer order={5} name="cad.references">
            <WorldReferenceLayer
              references={worldReferences}
              selectedIds={selectedReferenceIds}
              hoveredId={hoveredReferenceId}
              revealedIds={revealedReferenceIds}
              gumballConfig={projectionGumballConfig ?? undefined}
              relocateActive={referenceRelocateActive && cadGumballConfigVisible(projectionGumballConfig ?? {})}
              onSelect={onReferenceSelect}
              onHover={onReferenceHover}
              onRelocate={onReferenceRelocate}
            />
          </WorldLayer>
          <WorldLayer order={10} name="cad.ground-pick">
            <GroundPickPlane
              enabled={pickPlaneEnabled}
              onPick={onGroundPickEvent}
              onContextPick={onGroundContextEvent}
              onPointerMove={onScenePointerMoveEvent}
              pointerMoveEnabled={groundMoveOn}
              planeColor={resolvedTheme.groundPlaneColor ?? spatialSceneColors().groundPlane}
              planeOpacity={resolvedTheme.groundPlaneOpacity}
            />
          </WorldLayer>
          <WorldLayer order={20} name="cad.factory-wireframe">
            <GeometryFactoryWireframeLayer geometry={scenePickGeometry} visible={sceneVisibility.showFactoryWireframe} />
          </WorldLayer>
          <WorldLayer order={30} name="cad.pick">
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
          </WorldLayer>
          <WorldLayer order={35} name="cad.interaction-drag">
            {heightMoveOn && origin && corner ? <HeightDragSurface origin={origin} corner={corner} /> : null}
            {zRodMoveOn && origin ? <VerticalZDragRod origin={origin} /> : null}
            {origin && (heightMoveOn || zRodMoveOn) ? (
              <SpatialConstrainedPointerBridge
                mode={zRodMoveOn ? "vertical-z" : heightMoveOn ? "height-yz" : null}
                origin={origin}
                corner={corner}
                enabled={heightMoveOn || zRodMoveOn}
                onPointerMove={onScenePointerMoveEvent}
                onPointerDown={
                  zRodMoveOn
                    ? (point) => {
                        const event = createSpatialPickEvent("pointer.down", point, null);
                        onInteractionEvent?.(event);
                      }
                    : undefined
                }
              />
            ) : null}
          </WorldLayer>
          <WorldLayer order={40} name="cad.committed">
            <CommittedMeshLayer
              meshes={layerMeshes}
              modelRevision={geometry?.revision ?? 0}
              styleForSolid={styleForSolid}
              pickable={committedMeshPickable}
              showFaces={sceneVisibility.showCommittedFaces}
              showEdges={sceneVisibility.showCommittedEdges}
              onFacePointerDown={onCommittedFacePointerDown}
              onFacePointerMove={onCommittedFacePointerMove}
              hoveredTargetKey={hoveredTargetKey}
              selectedTargetKey={selectedTargetKey}
              selectedTargetKeys={selectedTargetKeys}
            />
            <FactoryFaceSurfaceLayer faces={factoryFaceMeshes} modelRevision={geometry?.revision ?? 0} visible={sceneVisibility.showCommittedFaces} />
          </WorldLayer>
          <WorldLayer order={50} name="cad.display">
            <InteractionDisplay geometry={geometry} model={displayModel ?? snapshot.display} renderItem={renderDisplayItem} />
            {slots?.afterDisplay}
          </WorldLayer>
          <WorldLayer order={60} name="cad.gumball">
            {slots?.afterCommitted}
            {(() => {
              const transformModel = transformGumballModel ?? (geometry instanceof Model ? geometry : null);
              return cadGumballConfigVisible(projectionGumballConfig) && transformModel && onTransformGumballCommit ? (
                <SpatialTransformGumball
                  config={projectionGumballConfig!}
                  model={transformModel}
                  targets={transformGumballTargets}
                  previewKernel={previewKernel}
                  onPreview={onTransformGumballPreview}
                  onPreviewEnd={onTransformGumballPreviewEnd}
                  onCommit={onTransformGumballCommit}
                />
              ) : null;
            })()}
          </WorldLayer>
        </WorldOrbitViewSnapGateProvider>
      </WorldLodBridge>
    </>
  );
}
// #endregion 🪩️Canvas

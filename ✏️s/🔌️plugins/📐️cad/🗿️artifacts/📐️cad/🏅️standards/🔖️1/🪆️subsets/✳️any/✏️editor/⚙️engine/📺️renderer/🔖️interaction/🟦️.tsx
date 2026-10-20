/** 🔖️ The transform gumball, spatial view slots and theme, the ground pick plane, constrained pointer bridge, drag surfaces and the R3F interaction adapter.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | renderer}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import {
  reactHostPort,
  UnifiedGumball,
  type GumballConfig,
  type GumballPose,
  type ThreeEvent,
  type ReactNode,
  type ThreeCamera,
  type Group as ThreeGroup,
  type Object3D as ThreeObject3D,
  type Ray as ThreeRay
} from "@semio-tech/ui-react";
// #endregion 🔌️Adapters
import { type Vec3 } from "@semio-tech/cad-engine-ts/registry";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { type InteractionEvent, type SelectionTarget } from "@semio-tech/s-spatial-kernel-ts/geometry/selection";
import { isEmptyModelDiff, type SpatialPreviewKernel, type ModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { cadGumballConfigVisible, selectionTargetsCenter, selectionTargetsPointTransformDiff, selectionTargetsHaveTransformableVertices, type CadGumballConfig } from "../../🎬️actions/🧲️selection-targets/🟦️.ts";
import { r3fPreviewKernel, THREE, useThree } from "../🔌️ports/🟦️.ts";
import { spatialSceneColors } from "../🎨️style/🟦️.ts";
import { raycastNone, readVec3 } from "../🧲️geometry-targets/📐️preview-layout/🟦️.ts";

// #region 🖼️DisplayPrimitives
// #region 🎨️HostCustomization
//#region 🔖️TransformGumball
/** 📐️ World-space gumball matrix snapshot (Three.js compose order). */
export interface GumballMatrixSnapshot {
  readonly position: Vec3;
  readonly quaternion: readonly [number, number, number, number];
  readonly scale: Vec3;
}

function gumballSnapshotFromObject3D(object: ThreeObject3D): GumballMatrixSnapshot {
  const position = object.position;
  const quaternion = object.quaternion;
  const scale = object.scale;
  return {
    position: [position.x, position.y, position.z],
    quaternion: [quaternion.x, quaternion.y, quaternion.z, quaternion.w],
    scale: [scale.x, scale.y, scale.z],
  };
}

function gumballPoseToMatrixSnapshot(pose: GumballPose | GumballMatrixSnapshot): GumballMatrixSnapshot {
  return {
    position: [pose.position[0], pose.position[1], pose.position[2]],
    quaternion: [pose.quaternion[0], pose.quaternion[1], pose.quaternion[2], pose.quaternion[3]],
    scale: [pose.scale[0], pose.scale[1], pose.scale[2]],
  };
}

/** 🎛️ Applies a gumball world-matrix delta to vertices and nurbs poles on topology-selected targets. */
export function transformGumballMatrixDiff(model: Model, targets: readonly SelectionTarget[], before: GumballMatrixSnapshot, after: GumballMatrixSnapshot, pivot?: Vec3): ModelDiff {
  const pivotPoint = pivot ?? before.position;
  const pivotV = new THREE.Vector3(pivotPoint[0], pivotPoint[1], pivotPoint[2]);
  const mBefore = new THREE.Matrix4().compose(pivotV, new THREE.Quaternion(before.quaternion[0], before.quaternion[1], before.quaternion[2], before.quaternion[3]), new THREE.Vector3(before.scale[0], before.scale[1], before.scale[2]));
  const mAfter = new THREE.Matrix4().compose(
    new THREE.Vector3(after.position[0], after.position[1], after.position[2]),
    new THREE.Quaternion(after.quaternion[0], after.quaternion[1], after.quaternion[2], after.quaternion[3]),
    new THREE.Vector3(after.scale[0], after.scale[1], after.scale[2]),
  );
  const delta = mAfter.multiply(mBefore.clone().invert());
  const point = new THREE.Vector3();
  return selectionTargetsPointTransformDiff(model, targets, (position) => {
    point.set(position[0], position[1], position[2]);
    point.applyMatrix4(delta);
    return [point.x, point.y, point.z];
  });
}

/** 🎛️ R3F gumball for multi-target primitive transforms (pivot at selection bbox center). */
export function SpatialTransformGumball(props: {
  readonly config: CadGumballConfig;
  readonly model: Model;
  readonly targets: readonly SelectionTarget[];
  readonly previewKernel?: SpatialPreviewKernel;
  readonly onPreview?: (diff: ModelDiff) => void;
  readonly onPreviewEnd?: () => void;
  readonly onCommit: (diff: ModelDiff) => void;
}): ReactNode {
  const previewKernel = props.previewKernel ?? r3fPreviewKernel;
  const pivot = reactHostPort.useMemo(() => selectionTargetsCenter(props.model, props.targets, previewKernel), [props.model, props.targets, previewKernel, props.model.revision]);
  const groupRef = reactHostPort.useRef<ThreeGroup>(null);
  const [tcTarget, setTcTarget] = reactHostPort.useState<ThreeObject3D | null>(null);
  const beforeRef = reactHostPort.useRef<GumballMatrixSnapshot | null>(null);
  const pivotRef = reactHostPort.useRef<Vec3 | null>(null);
  const canTransform = selectionTargetsHaveTransformableVertices(props.model, props.targets);

  const previewFromPose = reactHostPort.useCallback(
    (after: GumballPose) => {
      const before = beforeRef.current;
      const pivotPoint = pivotRef.current;
      if (!before || !pivotPoint || !props.onPreview) return;
      const diff = transformGumballMatrixDiff(props.model, props.targets, before, gumballPoseToMatrixSnapshot(after), pivotPoint);
      props.onPreview(diff);
    },
    [props],
  );

  reactHostPort.useLayoutEffect(() => {
    const group = groupRef.current;
    if (!group || !pivot) return;
    group.position.set(pivot[0], pivot[1], pivot[2]);
    group.quaternion.set(0, 0, 0, 1);
    group.scale.set(1, 1, 1);
    group.updateMatrixWorld(true);
    setTcTarget(group);
  }, [pivot, props.targets, props.config]);

  if (!pivot || !canTransform || !cadGumballConfigVisible(props.config)) return null;

  return (
    <>
      <group ref={groupRef}>
        <mesh visible={false}>
          <boxGeometry args={[0.001, 0.001, 0.001]} />
        </mesh>
      </group>
      {tcTarget ? (
        <UnifiedGumball
          target={tcTarget}
          config={props.config as GumballConfig}
          onDragStart={() => {
            if (!groupRef.current || !pivot) return;
            beforeRef.current = gumballSnapshotFromObject3D(groupRef.current);
            pivotRef.current = pivot;
          }}
          onDrag={(_kind, after) => {
            previewFromPose(after);
          }}
          onDragEnd={(_kind, _before, after) => {
            const before = beforeRef.current;
            const pivotPoint = pivotRef.current;
            beforeRef.current = null;
            pivotRef.current = null;
            props.onPreviewEnd?.();
            const group = groupRef.current;
            if (!before || !group || !pivotPoint) return;
            const diff = transformGumballMatrixDiff(props.model, props.targets, before, gumballPoseToMatrixSnapshot(after), pivotPoint);
            if (!isEmptyModelDiff(diff)) props.onCommit(diff);
            group.position.set(pivotPoint[0], pivotPoint[1], pivotPoint[2]);
            group.quaternion.set(0, 0, 0, 1);
            group.scale.set(1, 1, 1);
            group.updateMatrixWorld(true);
          }}
        />
      ) : null}
    </>
  );
}
//#endregion 🔖️TransformGumball

/** 🪩️ Optional scene slots for host overlays (gizmos, annotations, alternate lighting). */
export interface InteractionSpatialViewSlots {
  readonly beforeScene?: ReactNode;
  readonly afterDisplay?: ReactNode;
  readonly afterCommitted?: ReactNode;
  readonly lights?: ReactNode;
  readonly environment?: ReactNode;
}

/** 🎨️ Theme tokens for default scene chrome (hosts override per product). */
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
// #endregion 🎨️HostCustomization
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

/** 🖱️ Ground hit-test on the **XY** working plane at fixed world **Z** (= spatial footprint plane; factory height is world Z). */
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
    <mesh position={[0, 0, planeZ]} renderOrder={1} onPointerDown={onPointerDown} onContextMenu={onContextMenu} onPointerMove={onPointerMoveH}>
      <planeGeometry args={[120, 120]} />
      <meshBasicMaterial transparent opacity={planeOpacity} color={resolvedPlaneColor} side={THREE.DoubleSide} depthWrite={false} />
    </mesh>
  );
}

export function vec3FromSnapshotContext(ctx: Record<string, unknown>, key: string): Vec3 | null {
  return readVec3(ctx[key]);
}

const HEIGHT_DRAG_PLANE_X_OFFSET = 0.06;

/** 📍️ Projects `ray` onto the infinite world-Z line through `origin` (Z may be negative). */
export function projectRayToVerticalZLine(ray: ThreeRay, origin: Vec3): Vec3 {
  const [ox, oy, oz] = origin;
  const ro = ray.origin;
  const rd = ray.direction;
  const eps = 1e-9;
  let z = oz;
  if (Math.abs(rd.x) > Math.abs(rd.y) && Math.abs(rd.x) > eps) {
    z = ro.z + ((ox - ro.x) / rd.x) * rd.z;
  } else if (Math.abs(rd.y) > eps) {
    z = ro.z + ((oy - ro.y) / rd.y) * rd.z;
  } else {
    z = ro.z;
  }
  return [ox, oy, z];
}

/** 📍️ Intersects `ray` with the YZ plane at fixed world X. */
export function projectRayToYzPlaneAtX(ray: ThreeRay, planeX: number): Vec3 | null {
  const plane = new THREE.Plane(new THREE.Vector3(1, 0, 0), -planeX);
  const hit = new THREE.Vector3();
  return ray.intersectPlane(plane, hit) ? ([hit.x, hit.y, hit.z] as unknown as Vec3) : null;
}

function pointerRayFromClient(client: { readonly x: number; readonly y: number }, camera: ThreeCamera, rect: DOMRect): ThreeRay {
  const pointer = new THREE.Vector2(((client.x - rect.left) / rect.width) * 2 - 1, -(((client.y - rect.top) / rect.height) * 2 - 1));
  const raycaster = new THREE.Raycaster();
  raycaster.setFromCamera(pointer, camera);
  return raycaster.ray;
}

type SpatialConstrainedPointerMode = "vertical-z" | "height-yz";

/** 🖱️ Canvas raycast cursor constraint (vertical Z rod or YZ height wall) independent of pick-mesh hit. */
export function SpatialConstrainedPointerBridge({
  mode,
  origin,
  corner,
  enabled,
  onPointerMove,
  onPointerDown,
}: {
  readonly mode: SpatialConstrainedPointerMode | null;
  readonly origin: Vec3;
  readonly corner: Vec3 | null;
  readonly enabled: boolean;
  readonly onPointerMove?: (point: Vec3) => void;
  readonly onPointerDown?: (point: Vec3) => void;
}): null {
  const { camera, gl } = useThree();
  reactHostPort.useEffect(() => {
    if (!enabled || mode === null || !onPointerMove) return;
    const canvas = gl.domElement;
    const resolve = (ray: ThreeRay): Vec3 | null => {
      if (mode === "vertical-z") return projectRayToVerticalZLine(ray, origin);
      if (mode === "height-yz" && corner) return projectRayToYzPlaneAtX(ray, corner[0] + HEIGHT_DRAG_PLANE_X_OFFSET);
      return null;
    };
    const onMove = (event: PointerEvent) => {
      const point = resolve(pointerRayFromClient(event, camera, canvas.getBoundingClientRect()));
      if (point) onPointerMove(point);
    };
    const onDown = (event: PointerEvent) => {
      if (event.button !== 0 || !onPointerDown) return;
      const point = resolve(pointerRayFromClient(event, camera, canvas.getBoundingClientRect()));
      if (point) onPointerDown(point);
    };
    canvas.addEventListener("pointermove", onMove, { passive: true });
    canvas.addEventListener("pointerdown", onDown, { passive: true });
    return () => {
      canvas.removeEventListener("pointermove", onMove);
      canvas.removeEventListener("pointerdown", onDown);
    };
  }, [enabled, mode, origin, corner, camera, gl, onPointerMove, onPointerDown]);
  return null;
}

/** 🖱️ YZ wall at the second corner so `pointer.move` changes world Z (factory height uses |Δz|). */
export function HeightDragSurface({ origin, corner }: { readonly origin: Vec3; readonly corner: Vec3 }): ReactNode {
  const z0 = origin[2];
  const zSpan = 10;
  const zMid = z0 + zSpan / 2;
  const ySpan = 6;
  const xPlane = corner[0] + HEIGHT_DRAG_PLANE_X_OFFSET;
  return (
    <mesh position={[xPlane, corner[1], zMid]} rotation={[0, Math.PI / 2, 0]} raycast={raycastNone} renderOrder={2}>
      <planeGeometry args={[zSpan, ySpan]} />
      <meshStandardMaterial transparent opacity={0.38} color={spatialSceneColors().accent} emissive={spatialSceneColors().accentEmissive} emissiveIntensity={0.25} roughness={0.88} metalness={0.08} depthWrite={false} side={THREE.DoubleSide} />
    </mesh>
  );
}

/** 🖱️ Z-aligned rod at `origin` (visual only; cursor projection is {@link SpatialConstrainedPointerBridge}). */
export function VerticalZDragRod({ origin }: { readonly origin: Vec3 }): ReactNode {
  const h = 22;
  return (
    <mesh position={[origin[0], origin[1], origin[2] + h / 2]} rotation={[Math.PI / 2, 0, 0]} raycast={raycastNone} renderOrder={3}>
      <cylinderGeometry args={[0.14, 0.14, h, 10]} />
      <meshStandardMaterial transparent opacity={0.14} color={spatialSceneColors().accentSecondary} depthWrite={false} side={THREE.DoubleSide} />
    </mesh>
  );
}

/** 🎮️ Maps R3F pointer events to `InteractionEvent` envelopes (point + modifiers). */
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

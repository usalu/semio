// #region Header

// Scene.tsx

// 2025 Ueli Saluz
// 2025 AdrianoCelentano

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion

import { Edges, GizmoHelper, GizmoViewport, Grid, OrbitControls, useGLTF } from "@react-three/drei";
import { Canvas, ThreeEvent, useThree } from "@react-three/fiber";
import React, { FC, ReactNode, Suspense, useCallback, useEffect, useMemo, useRef, useState } from "react";
import * as THREE from "three";
import { Camera, Plane, Point, Vector } from "../../semio";

const getComputedColor = (variable: string): string => getComputedStyle(document.documentElement).getPropertyValue(variable).trim();

// #region SceneModel - Unified abstraction for all interactive 3D models

/**
 * Unified interface for all models in a 3D scene.
 *
 * Models in a scene are:
 * - Hoverable: can be highlighted when the pointer enters their bounds
 * - Clickable: can be selected when clicked
 * - Focusable: camera can zoom towards them
 * - Have a semio plane: positioned in the semio coordinate system
 *
 * Examples:
 * - Pieces in the design editor are SceneModels
 * - Ports in the type editor are SceneModels
 */
export interface SceneModel {
  /** Unique identifier for the model */
  guid: string;

  /**
   * The plane defining the model's position and orientation in 3D space.
   * Uses Semio coordinate system (X-right, Y-forward, Z-up).
   *
   * For point-based models (like Ports), this can be derived from point + direction.
   */
  plane?: Plane;

  /**
   * Whether the model is currently selected.
   * Selected models have visual feedback and may be transformable.
   */
  isSelected?: boolean;

  /**
   * Whether the model is currently hovered.
   * Hovered models show visual feedback to indicate interactivity.
   */
  isHovered?: boolean;

  /**
   * Whether the model is focusable (camera can zoom to it).
   * Defaults to true if plane exists.
   */
  isFocusable?: boolean;

  /**
   * Optional callback when the model is clicked.
   */
  onClick?: () => void;

  /**
   * Optional callback when the pointer enters the model's bounds.
   */
  onPointerEnter?: () => void;

  /**
   * Optional callback when the pointer leaves the model's bounds.
   */
  onPointerLeave?: () => void;
}

/**
 * Interface for any entity that can be transformed in the 3D scene.
 * Extends SceneModel to inherit hoverable, clickable, focusable behavior.
 *
 * Every transformable model:
 * - Has a plane in the semio coordinate system (from SceneModel)
 * - Is hoverable, clickable, focusable (from SceneModel)
 * - Can optionally show transform controls (gumball) when selected
 */
export interface TransformableModel extends SceneModel {
  /**
   * Whether the model can be transformed.
   * If false, transform controls will not be shown even if plane exists.
   */
  isTransformable?: boolean;
}

/**
 * Delta representing how a plane has been transformed.
 */
export interface PlaneTransformDelta {
  /** Translation in x, y, z */
  translation?: { x: number; y: number; z: number };

  /** Rotation as quaternion or euler angles */
  rotation?: { x: number; y: number; z: number; w: number };

  /** Scale factor */
  scale?: number;
}

/**
 * Callback for when a model's plane is updated via transform controls.
 */
export type OnPlaneUpdate = (modelGuid: string, newPlane: Plane) => void;

/**
 * Callback for when multiple models' planes are updated simultaneously.
 */
export type OnMultiPlaneUpdate = (updates: Array<{ modelGuid: string; newPlane: Plane }>) => void;

/**
 * Utility to create a plane from a point and direction vector.
 * This allows point-based models (like Ports) to be treated as SceneModels.
 *
 * @param point - The origin point of the plane
 * @param direction - The normal/direction vector (becomes Z-axis)
 * @returns A Plane in Semio coordinate system
 */
export const planeFromPointAndDirection = (point: Point, direction: Vector): Plane => {
  const dir = new THREE.Vector3(direction.x, direction.y, direction.z).normalize();

  // Create an arbitrary perpendicular vector for X-axis
  const tempVec = Math.abs(dir.z) < 0.9 ? new THREE.Vector3(0, 0, 1) : new THREE.Vector3(1, 0, 0);

  const xAxis = new THREE.Vector3().crossVectors(tempVec, dir).normalize();
  const yAxis = new THREE.Vector3().crossVectors(dir, xAxis).normalize();

  return {
    origin: { x: point.x, y: point.y, z: point.z },
    xAxis: { x: xAxis.x, y: xAxis.y, z: xAxis.z },
    yAxis: { x: yAxis.x, y: yAxis.y, z: yAxis.z },
  };
};

/**
 * Get the position from a plane for focus/camera operations.
 */
export const getPlanePosition = (plane: Plane): THREE.Vector3 => {
  return new THREE.Vector3(plane.origin.x, plane.origin.y, plane.origin.z);
};

/**
 * Check if a model has a valid plane for positioning.
 */
export const hasValidPlane = (model: SceneModel): boolean => {
  return model.plane !== undefined && model.plane !== null;
};

/**
 * Check if a model is focusable (has a plane and isFocusable is not explicitly false).
 */
export const isModelFocusable = (model: SceneModel): boolean => {
  return hasValidPlane(model) && (model.isFocusable === undefined || model.isFocusable === true);
};

// #endregion

/**
 * Base Model component for all interactive 3D objects in the scene.
 *
 * This component provides the foundation for the unified SceneModel abstraction:
 * - Hoverable: responds to pointer enter/leave events with visual feedback
 * - Clickable: handles onClick events for selection
 * - Focusable: can be targeted by camera zoom via userData.id
 *
 * Used by:
 * - Pieces in the design editor (with plane from Piece.plane)
 * - Ports in the type editor (with plane derived from Port.point + Port.direction)
 * - Any other scene object that needs interactive behavior
 *
 * The userData.id property is critical for focus behavior - it allows the Scene
 * to find and zoom to this object by guid.
 */
interface ModelProps {
  children?: ReactNode;
  /** Visual state: model is selected */
  selected?: boolean;
  /** Visual state: model is hovered */
  hovered?: boolean;
  /** Click handler for selection */
  onClick?: (event: ThreeEvent<MouseEvent>) => void;
  /** Hover enter handler */
  onPointerEnter?: (event: ThreeEvent<PointerEvent>) => void;
  /** Hover leave handler */
  onPointerLeave?: (event: ThreeEvent<PointerEvent>) => void;
  color?: string;
  emissiveColor?: string;
  emissiveIntensity?: number;
  showEdges?: boolean;
  edgeColor?: string;
  /** userData.id should be set to the model's guid for focusable behavior */
  userData?: any;
}

export const Model: FC<ModelProps> = ({ children, selected = false, hovered = false, onClick, onPointerEnter, onPointerLeave, color, emissiveColor, emissiveIntensity = 0.45, showEdges = true, edgeColor, userData }) => {
  const foregroundColor = useMemo(() => getComputedColor("--foreground"), []);
  const activeBaseColor = useMemo(() => getComputedColor("--active-base"), []);
  const hoverBaseColor = useMemo(() => getComputedColor("--hover-base"), []);

  const resolvedColor = useMemo(() => {
    if (color) return color;
    if (selected) return activeBaseColor;
    if (hovered) return hoverBaseColor;
    return foregroundColor;
  }, [color, selected, hovered, activeBaseColor, hoverBaseColor, foregroundColor]);

  const resolvedEmissiveColor = emissiveColor || resolvedColor;
  const resolvedEdgeColor = edgeColor || foregroundColor;

  return (
    <group userData={userData} onClick={onClick} onPointerEnter={onPointerEnter} onPointerLeave={onPointerLeave}>
      {children ? (
        children
      ) : (
        <mesh>
          <boxGeometry args={[1, 1, 1]} />
          <meshStandardMaterial color={resolvedColor} emissive={resolvedEmissiveColor} emissiveIntensity={emissiveIntensity} />
          {showEdges && <Edges scale={1.001} color={resolvedEdgeColor} />}
        </mesh>
      )}
    </group>
  );
};

interface GltfProps {
  src: string;
  roughness?: number;
  metalness?: number;
}
const Gltf: FC<GltfProps> = ({ src, roughness, metalness }) => {
  const { scene } = useGLTF(src);

  useEffect(() => {
    if (roughness !== undefined || metalness !== undefined) {
      scene.traverse((node) => {
        if ((node as any).isMesh && (node as any).material) {
          if ((node as any).material.roughness !== undefined && roughness !== undefined) {
            (node as any).material.roughness = roughness;
          }
          if ((node as any).material.metalness !== undefined && metalness !== undefined) {
            (node as any).material.metalness = metalness;
          }

          if (Array.isArray((node as any).material)) {
            (node as any).material.forEach((material: any) => {
              if (material.roughness !== undefined && roughness !== undefined) {
                material.roughness = roughness;
              }
              if (material.metalness !== undefined && metalness !== undefined) {
                material.metalness = metalness;
              }
            });
          }

          if ((node as any).material.needsUpdate !== undefined) {
            (node as any).material.needsUpdate = true;
          }
        }
      });
    }
  }, [scene, roughness, metalness]);

  return <primitive object={scene} />;
};

interface FileProps {
  src: string;
  environment?: string;
  roughness?: number;
  metalness?: number;
}
const File: FC<FileProps> = ({ src, environment, roughness, metalness }) => {
  return (
    <div className="w-full h-full">
      <Model>
        <Suspense fallback={null}>
          <Gltf src={src} roughness={roughness} metalness={metalness} />
        </Suspense>
      </Model>
    </div>
  );
};

interface GizmoProps {
  show?: boolean;
}

const Gizmo: FC<GizmoProps> = ({ show = true }) => {
  const colors = useMemo(() => [getComputedColor("--accent"), getComputedColor("--accent-tertiary"), getComputedColor("--accent-secondary")] as [string, string, string], []);
  const labels = useMemo(() => ["X", "Z", "-Y"] as [string, string, string], []);
  const margin = useMemo(() => [80, 80] as [number, number], []);
  if (!show) return null;
  return (
    <GizmoHelper alignment="bottom-right" margin={margin}>
      <GizmoViewport labels={labels} axisColors={colors} />
    </GizmoHelper>
  );
};

interface SceneInnerProps {
  children?: ReactNode;
  showGrid?: boolean;
  showGizmo?: boolean;
  camera?: Camera;
  onCameraChange?: (camera: Camera) => void;
  focusedItemId?: string;
  onFocusComplete?: () => void;
}

const SceneInner: FC<SceneInnerProps> = ({ children, showGrid = true, showGizmo = true, camera: initialCamera, onCameraChange, focusedItemId, onFocusComplete }) => {
  const [gridColors, setGridColors] = useState({
    sectionColor: getComputedColor("--foreground"),
    cellColor: getComputedColor("--accent-foreground"),
  });

  useEffect(() => {
    const updateColors = () =>
      setGridColors({
        sectionColor: getComputedColor("--foreground"),
        cellColor: getComputedColor("--accent-foreground"),
      });
    updateColors();
    const observer = new MutationObserver(updateColors);
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["class"],
    });
    return () => observer.disconnect();
  }, []);

  const { camera: threeCamera, gl, size, scene: threeScene } = useThree();
  const controlsRef = useRef<any>(null);
  const isUpdatingCameraRef = useRef(false);
  const prevCameraStringRef = useRef<string | undefined>(initialCamera ? JSON.stringify(initialCamera) : undefined);
  const cameraRestoredRef = useRef(false);
  const restoredCameraStringRef = useRef<string | undefined>(undefined);

  const cameraRef = useRef<THREE.OrthographicCamera>(threeCamera as THREE.OrthographicCamera);

  useEffect(() => {
    const cam = cameraRef.current;
    if (cam && cam instanceof THREE.OrthographicCamera) {
      cam.zoom = 50;
      cam.updateProjectionMatrix();
    }
  }, []);

  useEffect(() => {
    if (!cameraRef.current || !controlsRef.current) return;

    const currentCameraString = initialCamera ? JSON.stringify(initialCamera) : undefined;

    if (prevCameraStringRef.current !== currentCameraString) {
      cameraRestoredRef.current = false;
      prevCameraStringRef.current = currentCameraString;
    }
    if (restoredCameraStringRef.current !== currentCameraString) {
      cameraRestoredRef.current = false;
    }

    if (cameraRestoredRef.current) return;

    isUpdatingCameraRef.current = true;

    if (initialCamera) {
      const forwardLength = Math.sqrt(initialCamera.forward.x * initialCamera.forward.x + initialCamera.forward.y * initialCamera.forward.y + initialCamera.forward.z * initialCamera.forward.z);

      if (forwardLength < 0.01) {
        cameraRestoredRef.current = true;
        isUpdatingCameraRef.current = false;
        return;
      }

      requestAnimationFrame(() => {
        if (!cameraRef.current || !controlsRef.current) return;

        cameraRef.current.position.set(initialCamera.position.x, initialCamera.position.y, initialCamera.position.z);
        cameraRef.current.up.set(initialCamera.up.x, initialCamera.up.y, initialCamera.up.z);
        const target = new THREE.Vector3(initialCamera.position.x + initialCamera.forward.x, initialCamera.position.y + initialCamera.forward.y, initialCamera.position.z + initialCamera.forward.z);
        controlsRef.current.target.copy(target);
        cameraRef.current.updateProjectionMatrix();
        controlsRef.current.update();

        setTimeout(() => {
          isUpdatingCameraRef.current = false;
        }, 300);
      });

      cameraRestoredRef.current = true;
      restoredCameraStringRef.current = currentCameraString;
    } else {
      requestAnimationFrame(() => {
        if (!cameraRef.current || !controlsRef.current) return;

        cameraRef.current.position.set(10, 10, 10);
        cameraRef.current.up.set(0, 1, 0);
        controlsRef.current.target.set(0, 0, 0);
        cameraRef.current.updateProjectionMatrix();
        controlsRef.current.update();

        setTimeout(() => {
          isUpdatingCameraRef.current = false;
        }, 300);
      });

      cameraRestoredRef.current = true;
      restoredCameraStringRef.current = currentCameraString;
    }
  }, [initialCamera]);

  const handleEnd = useCallback(() => {
    if (isUpdatingCameraRef.current) return;
    if (cameraRef.current && controlsRef.current && onCameraChange) {
      const position = cameraRef.current.position;
      const target = controlsRef.current.target;
      const forwardVec = new THREE.Vector3().subVectors(target, position);

      if (forwardVec.lengthSq() < 0.0001) return;

      const forward = forwardVec.normalize();
      const up = cameraRef.current.up;
      const newCamera = {
        position: { x: position.x, y: position.y, z: position.z },
        forward: { x: forward.x, y: forward.y, z: forward.z },
        up: { x: up.x, y: up.y, z: up.z },
      };
      onCameraChange(newCamera);
    }
  }, [onCameraChange]);

  // Focus behavior: zoom camera to a model by guid
  // This implements the "focusable" part of the SceneModel abstraction.
  // Models (Pieces, Ports, etc.) set userData.id = model.guid to enable this behavior.
  useEffect(() => {
    if (!focusedItemId || !cameraRef.current || !controlsRef.current) return;

    let targetObject: THREE.Object3D | null = null;

    // Find the model by searching for userData.id matching the focusedItemId (guid)
    threeScene.traverse((obj: THREE.Object3D) => {
      if (obj.userData?.id === focusedItemId || obj.name === focusedItemId) {
        targetObject = obj;
      }
    });

    if (targetObject) {
      // Calculate the bounding box to determine zoom distance
      const box = new THREE.Box3().setFromObject(targetObject);
      const center = box.getCenter(new THREE.Vector3());
      const size = box.getSize(new THREE.Vector3());
      const maxDim = Math.max(size.x, size.y, size.z);
      const distance = maxDim * 2;

      const camera = cameraRef.current;
      const currentPos = camera.position.clone();
      const direction = new THREE.Vector3().subVectors(currentPos, controlsRef.current.target).normalize();
      const newPosition = center.clone().add(direction.multiplyScalar(distance));

      isUpdatingCameraRef.current = true;

      // Smoothly animate camera to the focused model
      const animate = () => {
        const t = 0.1;
        camera.position.lerp(newPosition, t);
        controlsRef.current.target.lerp(center, t);
        camera.updateProjectionMatrix();
        controlsRef.current.update();

        const distanceToTarget = camera.position.distanceTo(newPosition);
        const targetDistanceToCenter = controlsRef.current.target.distanceTo(center);

        if (distanceToTarget > 0.01 || targetDistanceToCenter > 0.01) {
          requestAnimationFrame(animate);
        } else {
          isUpdatingCameraRef.current = false;
          if (onFocusComplete) onFocusComplete();
        }
      };

      requestAnimationFrame(animate);
    } else if (onFocusComplete) {
      onFocusComplete();
    }
  }, [focusedItemId, threeScene, onFocusComplete]);

  return (
    <>
      <OrbitControls
        ref={controlsRef}
        enableDamping={false}
        mouseButtons={{
          LEFT: THREE.MOUSE.ROTATE,
          MIDDLE: undefined,
          RIGHT: undefined,
        }}
        onEnd={handleEnd}
      />
      <ambientLight intensity={1} />
      {children}
      {showGrid && <Grid infiniteGrid={true} sectionColor={gridColors.sectionColor} cellColor={gridColors.cellColor} />}
      {showGizmo && <Gizmo />}
    </>
  );
};

interface SceneProps {
  children?: ReactNode;
  showGrid?: boolean;
  showGizmo?: boolean;
  camera?: Camera;
  onCameraChange?: (camera: Camera) => void;
  onDoubleClickCapture?: (e: React.MouseEvent) => void;
  onPointerMissed?: (e: MouseEvent) => void;
  orthographic?: boolean;
  shadows?: boolean;
  className?: string;
  focusedItemId?: string;
  onFocusComplete?: () => void;
}

const Scene: FC<SceneProps> = ({ children, showGrid = true, showGizmo = true, camera, onCameraChange, onDoubleClickCapture, onPointerMissed, orthographic = true, shadows = false, className = "", focusedItemId, onFocusComplete }) => (
  <div className={`h-full w-full ${className}`} onDoubleClick={onDoubleClickCapture}>
    <Canvas onPointerMissed={onPointerMissed} orthographic={orthographic} shadows={shadows} camera={orthographic ? { zoom: 50, position: [10, 10, 10] } : undefined}>
      <SceneInner showGrid={showGrid} showGizmo={showGizmo} camera={camera} onCameraChange={onCameraChange} focusedItemId={focusedItemId} onFocusComplete={onFocusComplete}>
        {children}
      </SceneInner>
    </Canvas>
  </div>
);

export const SceneSkeleton: FC = () => (
  <div className="h-full w-full bg-background flex items-center justify-center">
    <div className="relative w-32 h-32 animate-pulse">
      <div className="absolute inset-0 border-4 border-muted-foreground/20 rounded-lg" />
      <div className="absolute inset-2 border-2 border-muted-foreground/20 rounded-lg" />
      <div className="absolute inset-4 border border-muted-foreground/20 rounded-lg" />
    </div>
  </div>
);

export default Scene;

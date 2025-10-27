import { TransformControls } from "@react-three/drei";
import { useThree } from "@react-three/fiber";
import { FC, useCallback, useEffect, useMemo, useRef } from "react";
import * as THREE from "three";
import { averagePlane, matrixToPlane, Plane, planeToMatrix, toSemioRotation, toThreeRotation } from "../../../../semio";
import { TransformableModel, OnMultiPlaneUpdate } from "./TransformableModel";

interface SharedTransformControlsProps {
  /** The models currently selected for transformation */
  selectedModels: TransformableModel[];

  /** Callback when models are transformed */
  onUpdate: OnMultiPlaneUpdate;

  /** Callback when transform starts */
  onTransformStart?: () => void;

  /** Callback when transform ends */
  onTransformEnd?: () => void;

  /** Transform mode */
  mode?: "translate" | "rotate" | "scale";
}

/**
 * Shared transform controls that handle multiple selected models.
 * Shows a single gumball at the average plane of all selected models.
 * Transforming affects all selected models simultaneously.
 */
export const SharedTransformControls: FC<SharedTransformControlsProps> = ({
  selectedModels,
  onUpdate,
  onTransformStart,
  onTransformEnd,
  mode = "translate",
}) => {
  const transformControlsRef = useRef<any>(null);
  const groupRef = useRef<THREE.Group>(null);
  const isDraggingRef = useRef(false);
  const isUpdatingPlaneRef = useRef(false);

  // Store initial planes when drag starts
  const initialPlanesRef = useRef<Map<string, Plane>>(new Map());
  const initialTransformRef = useRef<{ position: THREE.Vector3; quaternion: THREE.Quaternion; scale: THREE.Vector3 } | null>(null);

  const { controls: orbitControls } = useThree();

  // Filter to only transformable models with planes
  const transformableModels = useMemo(
    () => selectedModels.filter((m) => m.isTransformable !== false && m.plane !== undefined),
    [selectedModels]
  );

  // Calculate the average plane for all selected models
  const averageReferencePlane = useMemo(() => {
    const planes = transformableModels.map((m) => m.plane).filter((p): p is Plane => p !== undefined);
    return averagePlane(planes);
  }, [transformableModels]);

  // Convert average plane to Three.js matrix
  const averageMatrix = useMemo(() => {
    if (!averageReferencePlane) return null;
    const planeMatrix = planeToMatrix(averageReferencePlane);
    return new THREE.Matrix4().multiplyMatrices(toThreeRotation(), planeMatrix);
  }, [averageReferencePlane]);

  // Transform properties for the gumball
  const transformProps = useMemo(() => {
    if (!averageMatrix) return null;
    const position = new THREE.Vector3();
    const quaternion = new THREE.Quaternion();
    const scale = new THREE.Vector3();
    averageMatrix.decompose(position, quaternion, scale);
    return { position, quaternion, scale };
  }, [averageMatrix]);

  const hasValidPlane = !!(averageReferencePlane && transformProps && transformableModels.length > 0);

  // Update planes for all selected models based on transform delta
  const updatePlanesFromTransform = useCallback(() => {
    const controls = transformControlsRef.current;
    if (!controls?.object || !initialTransformRef.current || initialPlanesRef.current.size === 0) {
      return;
    }

    // Calculate the delta transformation
    const currentPosition = controls.object.position.clone();
    const currentQuaternion = controls.object.quaternion.clone();
    const currentScale = controls.object.scale.clone();

    const initialPosition = initialTransformRef.current.position;
    const initialQuaternion = initialTransformRef.current.quaternion;

    // Calculate position delta
    const positionDelta = currentPosition.clone().sub(initialPosition);

    // Calculate rotation delta as a quaternion
    const rotationDelta = currentQuaternion.clone().multiply(initialQuaternion.clone().invert());

    // Apply the same delta to all selected models
    const updates = transformableModels
      .map((model) => {
        const initialPlane = initialPlanesRef.current.get(model.guid);
        if (!initialPlane) return null;

        // Convert initial plane to matrix
        const initialMatrix = planeToMatrix(initialPlane);
        const initialThreeMatrix = new THREE.Matrix4().multiplyMatrices(toThreeRotation(), initialMatrix);

        // Decompose initial matrix
        const modelPosition = new THREE.Vector3();
        const modelQuaternion = new THREE.Quaternion();
        const modelScale = new THREE.Vector3();
        initialThreeMatrix.decompose(modelPosition, modelQuaternion, modelScale);

        // Apply transformations based on mode
        if (mode === "translate") {
          // Apply position delta
          modelPosition.add(positionDelta);
        } else if (mode === "rotate") {
          // Apply rotation delta around the average position
          const offsetFromAverage = modelPosition.clone().sub(initialPosition);
          offsetFromAverage.applyQuaternion(rotationDelta);
          modelPosition.copy(initialPosition).add(offsetFromAverage);
          modelQuaternion.multiplyQuaternions(rotationDelta, modelQuaternion);
        } else if (mode === "scale") {
          // Scale is more complex - might need to scale relative to average position
          modelScale.multiply(currentScale);
        }

        // Compose new matrix
        const newThreeMatrix = new THREE.Matrix4().compose(modelPosition, modelQuaternion, modelScale);

        // Convert back to semio coordinate system
        const newSemioMatrix = new THREE.Matrix4().multiplyMatrices(toSemioRotation(), newThreeMatrix);

        // Convert to plane
        const newPlane = matrixToPlane(newSemioMatrix);

        return { modelGuid: model.guid, newPlane };
      })
      .filter((update): update is { modelGuid: string; newPlane: Plane } => update !== null);

    if (updates.length > 0) {
      onUpdate(updates);
    }
  }, [transformableModels, onUpdate, mode]);

  // Handle drag start/end
  useEffect(() => {
    const controls = transformControlsRef.current;
    if (!controls) return;

    const handleDraggingChanged = (event: any) => {
      if (event.value) {
        // Started dragging
        isDraggingRef.current = true;
        isUpdatingPlaneRef.current = true;

        // Store initial planes for all selected models
        initialPlanesRef.current.clear();
        transformableModels.forEach((model) => {
          if (model.plane) {
            initialPlanesRef.current.set(model.guid, model.plane);
          }
        });

        // Store initial transform
        if (controls.object) {
          initialTransformRef.current = {
            position: controls.object.position.clone(),
            quaternion: controls.object.quaternion.clone(),
            scale: controls.object.scale.clone(),
          };
        }

        // Disable OrbitControls
        if (orbitControls) {
          (orbitControls as any).enabled = false;
        }

        onTransformStart?.();
      } else {
        // Stopped dragging
        isDraggingRef.current = false;

        // Re-enable OrbitControls
        if (orbitControls) {
          (orbitControls as any).enabled = true;
        }

        // Final update
        updatePlanesFromTransform();

        onTransformEnd?.();

        // Clear initial planes
        initialPlanesRef.current.clear();
        initialTransformRef.current = null;

        // Re-enable plane updates
        setTimeout(() => {
          isUpdatingPlaneRef.current = false;
        }, 50);
      }
    };

    const handleObjectChange = () => {
      if (isDraggingRef.current) {
        updatePlanesFromTransform();
      }
    };

    controls.addEventListener("dragging-changed", handleDraggingChanged);
    controls.addEventListener("objectChange", handleObjectChange);

    return () => {
      controls.removeEventListener("dragging-changed", handleDraggingChanged);
      controls.removeEventListener("objectChange", handleObjectChange);

      if (orbitControls) {
        (orbitControls as any).enabled = true;
      }
    };
  }, [transformableModels, orbitControls, updatePlanesFromTransform, onTransformStart, onTransformEnd]);

  // Update group transform when average plane changes
  useEffect(() => {
    if (groupRef.current && transformProps && !isDraggingRef.current && !isUpdatingPlaneRef.current) {
      groupRef.current.position.copy(transformProps.position);
      groupRef.current.quaternion.copy(transformProps.quaternion);
      groupRef.current.scale.copy(transformProps.scale);
    }
  }, [transformProps]);

  if (!hasValidPlane) {
    return null;
  }

  return (
    <TransformControls ref={transformControlsRef} enabled={hasValidPlane} visible={hasValidPlane} mode={mode}>
      <group ref={groupRef}>
        {/* Empty group - the gumball is just a visual indicator */}
      </group>
    </TransformControls>
  );
};

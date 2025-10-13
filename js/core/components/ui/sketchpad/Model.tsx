// #region Header

// Model.tsx

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

import { GizmoHelper, GizmoViewport, Grid, OrbitControls, OrthographicCamera } from "@react-three/drei";
import { Canvas as ThreeCanvas, useThree } from "@react-three/fiber";
import React, { FC, ReactNode, useCallback, useEffect, useMemo, useRef, useState } from "react";
import * as THREE from "three";
import { Camera } from "../../../semio";

const getComputedColor = (variable: string): string => getComputedStyle(document.documentElement).getPropertyValue(variable).trim();

interface GizmoProps {
  show?: boolean;
}

const Gizmo: FC<GizmoProps> = ({ show = true }) => {
  const colors = useMemo(() => [getComputedColor("--color-primary"), getComputedColor("--color-tertiary"), getComputedColor("--color-secondary")] as [string, string, string], []);
  const labels = useMemo(() => ["X", "Z", "-Y"] as [string, string, string], []);
  const margin = useMemo(() => [80, 80] as [number, number], []);
  if (!show) return null;
  return (
    <GizmoHelper alignment="bottom-right" margin={margin}>
      <GizmoViewport labels={labels} axisColors={colors} />
    </GizmoHelper>
  );
};

interface ModelInnerProps {
  children?: ReactNode;
  showGrid?: boolean;
  showGizmo?: boolean;
  camera?: Camera;
  onCameraChange?: (camera: Camera) => void;
}

const ModelInner: FC<ModelInnerProps> = ({ children, showGrid = true, showGizmo = true, camera: initialCamera, onCameraChange }) => {
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

  const cameraRef = useRef<THREE.OrthographicCamera>(null);
  const controlsRef = useRef<any>(null);
  const cameraRestoredRef = useRef(false);
  const isUpdatingCameraRef = useRef(false);
  const prevCameraStringRef = useRef<string | undefined>(initialCamera ? JSON.stringify(initialCamera) : undefined);

  useEffect(() => {
    const currentCameraString = initialCamera ? JSON.stringify(initialCamera) : undefined;
    console.log('[Camera] Model effect - hasCamera:', !!initialCamera, 'restored:', cameraRestoredRef.current);
    if (prevCameraStringRef.current !== currentCameraString) {
      cameraRestoredRef.current = false;
      prevCameraStringRef.current = currentCameraString;
    }
    if (!cameraRestoredRef.current && initialCamera && cameraRef.current && controlsRef.current) {
      // Check if the camera data is valid (forward vector has length)
      const forwardLength = Math.sqrt(
        initialCamera.forward.x * initialCamera.forward.x +
        initialCamera.forward.y * initialCamera.forward.y +
        initialCamera.forward.z * initialCamera.forward.z
      );

      if (forwardLength < 0.01) {
        console.log('[Camera] SKIP restore - invalid forward vector');
        cameraRestoredRef.current = true; // Mark as "restored" so we don't keep trying
        return;
      }

      console.log('[Camera] RESTORE camera:', initialCamera);
      isUpdatingCameraRef.current = true;
      cameraRef.current.position.set(initialCamera.position.x, initialCamera.position.y, initialCamera.position.z);
      cameraRef.current.up.set(initialCamera.up.x, initialCamera.up.y, initialCamera.up.z);
      const target = new THREE.Vector3(initialCamera.position.x + initialCamera.forward.x, initialCamera.position.y + initialCamera.forward.y, initialCamera.position.z + initialCamera.forward.z);
      controlsRef.current.target.copy(target);
      cameraRef.current.updateProjectionMatrix();
      controlsRef.current.update();
      cameraRestoredRef.current = true;
      setTimeout(() => {
        isUpdatingCameraRef.current = false;
      }, 100);
    }
  }, [initialCamera]);

  const handleChange = useCallback(() => {
    console.log('[Camera] handleChange called', {
      isUpdating: isUpdatingCameraRef.current,
      hasCameraRef: !!cameraRef.current,
      hasControlsRef: !!controlsRef.current,
      hasOnCameraChange: !!onCameraChange
    });
    if (isUpdatingCameraRef.current) {
      console.log('[Camera] SKIP - isUpdating');
      return;
    }
    if (cameraRef.current && controlsRef.current && onCameraChange) {
      const position = cameraRef.current.position;
      const target = controlsRef.current.target;
      const forwardVec = new THREE.Vector3().subVectors(target, position);

      console.log('[Camera] handleChange - position:', position, 'target:', target, 'forwardLength:', forwardVec.length());

      // Don't save camera if the forward vector is too small (invalid state)
      if (forwardVec.lengthSq() < 0.0001) {
        console.log('[Camera] SKIP - forward vector too small');
        return;
      }

      const forward = forwardVec.normalize();
      const up = cameraRef.current.up;
      const newCamera = {
        position: { x: position.x, y: position.y, z: position.z },
        forward: { x: forward.x, y: forward.y, z: forward.z },
        up: { x: up.x, y: up.y, z: up.z },
      };
      console.log('[Camera] SAVE camera:', newCamera);
      onCameraChange(newCamera);
    }
  }, [onCameraChange]);

  return (
    <>
      <OrthographicCamera ref={cameraRef} position={[10, 10, 10]} zoom={50} />
      <OrbitControls
        ref={controlsRef}
        makeDefault
        mouseButtons={{
          LEFT: THREE.MOUSE.ROTATE,
          MIDDLE: undefined,
          RIGHT: undefined,
        }}
        onChange={handleChange}
      />
      <ambientLight intensity={1} />
      {children}
      {showGrid && <Grid infiniteGrid={true} sectionColor={gridColors.sectionColor} cellColor={gridColors.cellColor} />}
      {showGizmo && <Gizmo />}
    </>
  );
};

interface ModelProps {
  children?: ReactNode;
  showGrid?: boolean;
  showGizmo?: boolean;
  camera?: Camera;
  onCameraChange?: (camera: Camera) => void;
  onDoubleClickCapture?: (e: React.MouseEvent) => void;
  onPointerMissed?: (e: MouseEvent) => void;
}

const Model: FC<ModelProps> = ({ children, showGrid = true, showGizmo = true, camera, onCameraChange, onDoubleClickCapture, onPointerMissed }) => (
  <div className="h-full w-full">
    <ThreeCanvas onDoubleClickCapture={onDoubleClickCapture} onPointerMissed={onPointerMissed}>
      <ModelInner showGrid={showGrid} showGizmo={showGizmo} camera={camera} onCameraChange={onCameraChange}>
        {children}
      </ModelInner>
    </ThreeCanvas>
  </div>
);

export default Model;

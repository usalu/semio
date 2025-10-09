// #region Header

// Canvas.tsx

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
import { Canvas as ThreeCanvas } from "@react-three/fiber";
import React, { FC, ReactNode, useEffect, useMemo, useRef, useState } from "react";
import * as THREE from "three";

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

interface CanvasInnerProps {
  children?: ReactNode;
  showGrid?: boolean;
  showGizmo?: boolean;
}

const CanvasInner: FC<CanvasInnerProps> = ({ children, showGrid = true, showGizmo = true }) => {
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

  const camera = useRef<THREE.OrthographicCamera>(null);

  return (
    <>
      <OrthographicCamera ref={camera} />
      <OrbitControls
        makeDefault
        mouseButtons={{
          LEFT: THREE.MOUSE.ROTATE,
          MIDDLE: undefined,
          RIGHT: undefined,
        }}
      />
      <ambientLight intensity={1} />
      {children}
      {showGrid && <Grid infiniteGrid={true} sectionColor={gridColors.sectionColor} cellColor={gridColors.cellColor} />}
      {showGizmo && <Gizmo />}
    </>
  );
};

interface CanvasProps {
  children?: ReactNode;
  showGrid?: boolean;
  showGizmo?: boolean;
  onDoubleClickCapture?: (e: React.MouseEvent) => void;
  onPointerMissed?: (e: MouseEvent) => void;
}

const Canvas: FC<CanvasProps> = ({ children, showGrid = true, showGizmo = true, onDoubleClickCapture, onPointerMissed }) => (
  <div className="h-full w-full">
    <ThreeCanvas onDoubleClickCapture={onDoubleClickCapture} onPointerMissed={onPointerMissed}>
      <CanvasInner showGrid={showGrid} showGizmo={showGizmo}>
        {children}
      </CanvasInner>
    </ThreeCanvas>
  </div>
);

export default Canvas;

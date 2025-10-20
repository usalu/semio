// #region Header

// TypeModel.tsx

// 2025 Ueli Saluz

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

import { useGLTF } from "@react-three/drei";
import { ThreeEvent } from "@react-three/fiber";
import { FC, useCallback, useMemo, useRef, useState } from "react";
import * as THREE from "three";
import SceneComponent from "../../../../elements/Scene";
import { guid, Point, Port, Type, Vector } from "../../../../semio";
import { useKit, useKitCommands, useType } from "../../../kits/store";
import { ToolType } from "../../../store";
import { useTypeEditorActiveTool, useTypeEditorCamera, useTypeEditorCommands, useTypeEditorHover, useTypeEditorSelection } from "../store";

const PortVisual: FC<{ port: Port; isSelected: boolean; isHovered: boolean; onHover: () => void; onLeave: () => void; onClick: () => void }> = ({ port, isSelected, isHovered, onHover, onLeave, onClick }) => {
  const position = useMemo(() => new THREE.Vector3(port.point.x, port.point.y, port.point.z), [port.point]);
  const direction = useMemo(() => new THREE.Vector3(port.direction.x, port.direction.y, port.direction.z).normalize(), [port.direction]);

  const getComputedColor = (variable: string): string => getComputedStyle(document.documentElement).getPropertyValue(variable).trim();
  const selectedColor = useMemo(() => new THREE.Color(getComputedColor("--active-base")).getHex(), []);
  const hoverColor = useMemo(() => new THREE.Color(getComputedColor("--hover-base")).getHex(), []);
  const defaultColor = useMemo(() => new THREE.Color(getComputedColor("--foreground")).getHex(), []);

  const color = isSelected ? selectedColor : isHovered ? hoverColor : defaultColor;

  // Create custom arrow geometry for precise raycasting
  const { lineGeometry, coneGeometry, linePosition, conePosition } = useMemo(() => {
    const arrowLength = 0.5;
    const headLength = 0.1;
    const headWidth = 0.05;
    const shaftRadius = 0.01;

    // Line (shaft) geometry
    const lineGeo = new THREE.CylinderGeometry(shaftRadius, shaftRadius, arrowLength - headLength, 8);
    const linePos = position.clone().add(direction.clone().multiplyScalar((arrowLength - headLength) / 2));

    // Cone (head) geometry
    const coneGeo = new THREE.ConeGeometry(headWidth, headLength, 8);
    const conePos = position.clone().add(direction.clone().multiplyScalar(arrowLength - headLength / 2));

    return {
      lineGeometry: lineGeo,
      coneGeometry: coneGeo,
      linePosition: linePos,
      conePosition: conePos,
    };
  }, [position, direction]);

  // Calculate rotation to align with direction
  const rotation = useMemo(() => {
    const up = new THREE.Vector3(0, 1, 0);
    const quaternion = new THREE.Quaternion();
    quaternion.setFromUnitVectors(up, direction);
    return new THREE.Euler().setFromQuaternion(quaternion);
  }, [direction]);

  const handlePointerEvent = useCallback(
    (callback: () => void) => (e: ThreeEvent<PointerEvent>) => {
      e.stopPropagation();
      callback();
    },
    [],
  );

  return (
    <group>
      {/* Arrow shaft - precise cylinder */}
      <mesh geometry={lineGeometry} position={linePosition} rotation={rotation} onPointerEnter={handlePointerEvent(onHover)} onPointerLeave={handlePointerEvent(onLeave)} onClick={handlePointerEvent(onClick)}>
        <meshBasicMaterial color={color} />
      </mesh>

      {/* Arrow head - precise cone */}
      <mesh geometry={coneGeometry} position={conePosition} rotation={rotation} onPointerEnter={handlePointerEvent(onHover)} onPointerLeave={handlePointerEvent(onLeave)} onClick={handlePointerEvent(onClick)}>
        <meshBasicMaterial color={color} />
      </mesh>
    </group>
  );
};

const PortPreview: FC<{ position: THREE.Vector3; normal: THREE.Vector3 }> = ({ position, normal }) => {
  const arrow = useMemo(() => new THREE.ArrowHelper(normal.normalize(), position, 0.5, 0x00ff00, 0.1, 0.05), [position, normal]);

  return <primitive object={arrow} />;
};

const TypeMesh: FC<{ activeTool: ToolType; onPortPreview: (position: THREE.Vector3, normal: THREE.Vector3) => void; onPortCreate: (position: THREE.Vector3, normal: THREE.Vector3) => void; onClearPreview: () => void }> = ({
  activeTool,
  onPortPreview,
  onPortCreate,
  onClearPreview,
}) => {
  const type = useType() as Type | undefined;
  const kit = useKit();
  const [isPointerDown, setIsPointerDown] = useState(false);
  const pointerDownTimeRef = useRef<number>(0);
  const pointerDownPositionRef = useRef<{ x: number; y: number }>({ x: 0, y: 0 });

  const representationUrl = useMemo(() => {
    if (!type?.representations?.[0]) return null;
    const url = type.representations[0].url;
    if (url.startsWith("http")) return url;
    return null;
  }, [type]);

  const handlePointerDown = useCallback(
    (event: ThreeEvent<PointerEvent>) => {
      console.log("[ORIGIN] handlePointerDown", { activeTool, isPort: activeTool === ToolType.PORT });
      if (activeTool === ToolType.PORT) {
        setIsPointerDown(true);
        pointerDownTimeRef.current = Date.now();
        pointerDownPositionRef.current = { x: event.clientX, y: event.clientY };
      }
    },
    [activeTool],
  );

  const handlePointerUp = useCallback(
    (event: ThreeEvent<PointerEvent>) => {
      if (activeTool === ToolType.PORT && isPointerDown) {
        const timeDiff = Date.now() - pointerDownTimeRef.current;
        const distance = Math.sqrt(Math.pow(event.clientX - pointerDownPositionRef.current.x, 2) + Math.pow(event.clientY - pointerDownPositionRef.current.y, 2));

        if (timeDiff < 300 && distance < 5 && event.face) {
          event.stopPropagation();
          const position = new THREE.Vector3().copy(event.point);
          const normal = event.face.normal.clone();
          const normalMatrix = new THREE.Matrix3().getNormalMatrix((event.object as THREE.Mesh).matrixWorld);
          normal.applyMatrix3(normalMatrix).normalize();
          onPortCreate(position, normal);
        }
        setIsPointerDown(false);
      }
    },
    [activeTool, isPointerDown, onPortCreate],
  );

  const handlePointerMove = useCallback(
    (event: ThreeEvent<PointerEvent>) => {
      console.log("[ORIGIN] handlePointerMove", { activeTool, isPort: activeTool === ToolType.PORT, hasFace: !!event.face, isPointerDown });
      if (activeTool === ToolType.PORT && event.face && !isPointerDown) {
        event.stopPropagation();
        const position = new THREE.Vector3().copy(event.point);
        const normal = event.face.normal.clone();
        const normalMatrix = new THREE.Matrix3().getNormalMatrix((event.object as THREE.Mesh).matrixWorld);
        normal.applyMatrix3(normalMatrix).normalize();
        onPortPreview(position, normal);
      }
    },
    [activeTool, isPointerDown, onPortPreview],
  );

  const handlePointerOut = useCallback(
    (event: ThreeEvent<PointerEvent>) => {
      if (activeTool === ToolType.PORT) {
        onClearPreview();
        setIsPointerDown(false);
      }
    },
    [activeTool, onClearPreview],
  );

  if (!representationUrl) {
    return (
      <mesh onPointerDown={handlePointerDown} onPointerUp={handlePointerUp} onPointerMove={handlePointerMove} onPointerOut={handlePointerOut}>
        <boxGeometry args={[1, 1, 1]} />
        <meshStandardMaterial color="gray" />
      </mesh>
    );
  }

  const { scene } = useGLTF(representationUrl);
  const clonedScene = useMemo(() => {
    const cloned = scene.clone();
    cloned.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        child.raycast = THREE.Mesh.prototype.raycast;
      }
    });
    return cloned;
  }, [scene]);

  return <primitive object={clonedScene} onPointerDown={handlePointerDown} onPointerUp={handlePointerUp} onPointerMove={handlePointerMove} onPointerOut={handlePointerOut} />;
};

const SceneContent: FC = () => {
  const activeTool = useTypeEditorActiveTool();
  const type = useType() as Type | undefined;
  const kit = useKit();
  const kitCommands = useKitCommands();
  const selection = useTypeEditorSelection();
  const hover = useTypeEditorHover();
  const { selectPort, deselectPort, hoverPort, clearHover } = useTypeEditorCommands();

  console.log("[ORIGIN] SceneContent render", { activeTool, typeGuid: type?.guid, hover, selection });

  const [portPreview, setPortPreview] = useState<{ position: THREE.Vector3; normal: THREE.Vector3 } | null>(null);

  const handlePortPreview = useCallback((position: THREE.Vector3, normal: THREE.Vector3) => {
    setPortPreview({ position, normal });
  }, []);

  const handlePortCreate = useCallback(
    (position: THREE.Vector3, normal: THREE.Vector3) => {
      if (type && kit) {
        const newPort: Port = {
          guid: guid(),
          point: {
            x: position.x,
            y: position.y,
            z: position.z,
          } as Point,
          direction: {
            x: normal.x,
            y: normal.y,
            z: normal.z,
          } as Vector,
          t: 0,
          mandatory: false,
        };

        kitCommands.updateType(type.guid, {
          ports: {
            added: [newPort],
          },
        });
        selectPort(newPort.guid);
      }
    },
    [type, kit, kitCommands, selectPort],
  );

  const handleClearPreview = useCallback(() => {
    setPortPreview(null);
    clearHover();
  }, [clearHover]);

  const handlePortClick = useCallback(
    (portId: string) => {
      console.log("[ORIGIN] handlePortClick", { portId, selection });
      const isSelected = selection?.ports?.includes(portId) || false;
      if (isSelected) {
        deselectPort(portId);
      } else {
        selectPort(portId);
      }
    },
    [selection, selectPort, deselectPort],
  );

  const handlePortHover = useCallback(
    (portId: string) => {
      console.log("[ORIGIN] handlePortHover", { portId });
      hoverPort(portId);
    },
    [hoverPort],
  );

  const handlePortLeave = useCallback(() => {
    console.log("[ORIGIN] handlePortLeave");
    clearHover();
  }, [clearHover]);

  return (
    <>
      <TypeMesh activeTool={activeTool} onPortPreview={handlePortPreview} onPortCreate={handlePortCreate} onClearPreview={handleClearPreview} />
      {type?.ports?.map((port) => {
        const isSelected = selection?.ports?.includes(port.guid) || false;
        const isHovered = hover?.port === port.guid;
        console.log("[ORIGIN] Rendering port", { portId: port.guid, isSelected, isHovered, hoverState: hover });
        return <PortVisual key={port.guid} port={port} isSelected={isSelected} isHovered={isHovered} onHover={() => handlePortHover(port.guid)} onLeave={handlePortLeave} onClick={() => handlePortClick(port.guid)} />;
      })}
      {portPreview && <PortPreview position={portPreview.position} normal={portPreview.normal} />}
    </>
  );
};

const Scene: FC = () => {
  const { setCamera } = useTypeEditorCommands();
  const camera = useTypeEditorCamera();

  const onCameraChange = useCallback(
    (newCamera: { position: { x: number; y: number; z: number }; forward: { x: number; y: number; z: number }; up: { x: number; y: number; z: number } }) => {
      setCamera(newCamera);
    },
    [setCamera],
  );

  return (
    <SceneComponent camera={camera} onCameraChange={onCameraChange}>
      <SceneContent />
    </SceneComponent>
  );
};

export default Scene;

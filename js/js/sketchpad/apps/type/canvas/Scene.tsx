// #region Header

// Scene.tsx

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

import { Line, Sphere, useGLTF } from "@react-three/drei";
import { ThreeEvent } from "@react-three/fiber";
import { FC, useCallback, useEffect, useMemo, useRef, useState } from "react";
import * as THREE from "three";
import SceneComponent, { Model } from "../../../../elements/windows/Scene";
import { guid, Kit, Point, Port, Type, Vector } from "../../../../semio";
import { useFocusSafe } from "../../../Navbar";
import { KitStore, useKit, useKitCommands, useKitStore, useType } from "../../../kits/store";
import { ToolType } from "../../../store";
import { useTypeApp, useTypeAppActiveTool, useTypeAppCamera, useTypeAppCommands, useTypeAppFocusedPortGuid, useTypeAppHover, useTypeAppSelectedRepresentationGuid, useTypeAppSelection } from "../store";
import { TypeAppTools } from "../tools_registry";

/**
 * PortVisual component - renders a Port as a SceneModel.
 *
 * Implements the unified SceneModel abstraction:
 * - Hoverable: changes color on pointer enter/leave
 * - Clickable: handles selection with tool-specific behavior
 * - Focusable: sets userData.id to port.guid for camera zoom
 * - Has a semio plane: derived from Port.point and Port.direction
 *
 * Unlike Pieces which have an explicit Plane property, Ports are defined by
 * a point and direction vector. The SceneModel abstraction allows both to be
 * focusable through the unified focus behavior in Scene.tsx.
 */
const PortVisual: FC<{ port: Port; isSelected: boolean; isHovered: boolean; onHover: () => void; onLeave: () => void; onClick: () => void; onDoubleClick: () => void }> = ({ port, isSelected, isHovered, onHover, onLeave, onClick, onDoubleClick }) => {
  const position = useMemo(() => [port.point.x, port.point.y, port.point.z] as [number, number, number], [port.point]);
  const direction = useMemo(() => {
    const dir = new THREE.Vector3(port.direction.x, port.direction.y, port.direction.z).normalize();
    return [dir.x, dir.y, dir.z] as [number, number, number];
  }, [port.direction]);

  const getComputedColor = (variable: string): string => getComputedStyle(document.documentElement).getPropertyValue(variable).trim();
  const selectedColor = useMemo(() => getComputedColor("--active-base"), []);
  const hoverColor = useMemo(() => getComputedColor("--hover-base"), []);
  const defaultColor = useMemo(() => getComputedColor("--foreground"), []);

  const color = isSelected ? selectedColor : isHovered ? hoverColor : defaultColor;

  // Calculate arrow points for line
  const arrowLength = 0.5;
  const endPoint = useMemo(() => [position[0] + direction[0] * arrowLength, position[1] + direction[1] * arrowLength, position[2] + direction[2] * arrowLength] as [number, number, number], [position, direction]);
  const points = useMemo(() => [position, endPoint], [position, endPoint]);

  // userData for making the port focusable by guid
  const userData = useMemo(() => ({ id: port.guid }), [port.guid]);

  const handleClick = useCallback(
    (e: ThreeEvent<MouseEvent>) => {
      e.stopPropagation();
      onClick();
    },
    [onClick],
  );

  const handlePointerEnter = useCallback(
    (e: ThreeEvent<PointerEvent>) => {
      e.stopPropagation();
      onHover();
    },
    [onHover],
  );

  const handlePointerLeave = useCallback(
    (e: ThreeEvent<PointerEvent>) => {
      e.stopPropagation();
      onLeave();
    },
    [onLeave],
  );

  const handleDoubleClick = useCallback(
    (e: ThreeEvent<MouseEvent>) => {
      e.stopPropagation();
      onDoubleClick();
    },
    [onDoubleClick],
  );

  return (
    <Model
      hovered={isHovered}
      onClick={handleClick}
      onDoubleClick={handleDoubleClick}
      onPointerEnter={handlePointerEnter}
      onPointerLeave={handlePointerLeave}
      userData={userData}
      showEdges={false}
    >
      <group>
        <Sphere args={[0.03]} position={position}>
          <meshBasicMaterial color={color} />
        </Sphere>
        <Line points={points} color={color} lineWidth={2} />
        <Sphere args={[0.05]} position={endPoint}>
          <meshBasicMaterial color={color} />
        </Sphere>
      </group>
    </Model>
  );
};

const PortPreview: FC<{ position: THREE.Vector3; normal: THREE.Vector3 }> = ({ position, normal }) => {
  const previewColor = "#00ff00";

  // Calculate arrow points for line
  const arrowLength = 0.5;
  const posArray = useMemo(() => [position.x, position.y, position.z] as [number, number, number], [position]);
  const direction = useMemo(() => {
    const dir = normal.clone().normalize();
    return [dir.x, dir.y, dir.z] as [number, number, number];
  }, [normal]);
  const endPoint = useMemo(() => [posArray[0] + direction[0] * arrowLength, posArray[1] + direction[1] * arrowLength, posArray[2] + direction[2] * arrowLength] as [number, number, number], [posArray, direction]);
  const points = useMemo(() => [posArray, endPoint], [posArray, endPoint]);

  return (
    <group>
      <Sphere args={[0.03]} position={posArray}>
        <meshBasicMaterial color={previewColor} />
      </Sphere>
      <Line points={points} color={previewColor} lineWidth={2} />
    </group>
  );
};

const LoadedTypeMesh: FC<{
  url: string;
  onPointerDown: (e: ThreeEvent<PointerEvent>) => void;
  onPointerUp: (e: ThreeEvent<PointerEvent>) => void;
  onPointerMove: (e: ThreeEvent<PointerEvent>) => void;
  onPointerOut: (e: ThreeEvent<PointerEvent>) => void;
}> = ({ url, onPointerDown, onPointerUp, onPointerMove, onPointerOut }) => {
  const { scene } = useGLTF(url);

  const clonedScene = useMemo(() => {
    const cloned = scene.clone();
    cloned.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        child.raycast = THREE.Mesh.prototype.raycast;
      }
    });
    return cloned;
  }, [scene]);

  return <primitive object={clonedScene} onPointerDown={onPointerDown} onPointerUp={onPointerUp} onPointerMove={onPointerMove} onPointerOut={onPointerOut} />;
};

const TypeMesh: FC<{ activeTool: ToolType; onPortPreview: (position: THREE.Vector3, normal: THREE.Vector3) => void; onPortCreate: (position: THREE.Vector3, normal: THREE.Vector3) => void; onClearPreview: () => void }> = ({
  activeTool,
  onPortPreview,
  onPortCreate,
  onClearPreview,
}) => {
  const type = useType() as Type | undefined;
  const kit = useKit() as Kit | undefined;
  const kitStore = useKitStore() as KitStore;
  const selectedRepresentationGuid = useTypeAppSelectedRepresentationGuid();
  const [isPointerDown, setIsPointerDown] = useState(false);
  const pointerDownTimeRef = useRef<number>(0);
  const pointerDownPositionRef = useRef<{ x: number; y: number }>({ x: 0, y: 0 });

  const representationUrl = useMemo(() => {
    if (!type?.representations || type.representations.length === 0) return null;
    
    // Find the selected representation or use the first one
    const representation = selectedRepresentationGuid 
      ? type.representations.find(r => r.guid === selectedRepresentationGuid) 
      : type.representations[0];
    
    if (!representation) return null;
    
    // Get the file and resolve its URL
    const file = kit?.files?.find((f) => f.guid === representation.file);
    if (!file) return null;
    
    // Use kitStore to get the file URL through the file provider
    const url = kitStore.getFileUrl(file.guid);
    if (!url) return null;
    
    return url;
  }, [type, kit, kitStore, selectedRepresentationGuid]);

  const handlePointerDown = useCallback(
    (event: ThreeEvent<PointerEvent>) => {
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

  return <LoadedTypeMesh url={representationUrl} onPointerDown={handlePointerDown} onPointerUp={handlePointerUp} onPointerMove={handlePointerMove} onPointerOut={handlePointerOut} />;
};

const SceneContent: FC = () => {
  const activeTool = useTypeAppActiveTool();
  const type = useType() as Type | undefined;
  const kit = useKit();
  const kitCommands = useKitCommands();
  const selection = useTypeAppSelection();
  const hover = useTypeAppHover();
  const appState = useTypeApp((s) => s);
  const { selectPort, deselectPort, hoverPort, clearHover, focusPort } = useTypeAppCommands();
  const [portPreview, setPortPreview] = useState<{ position: THREE.Vector3; normal: THREE.Vector3 } | null>(null);
  const focusContext = useFocusSafe();
  const prevItemsRef = useRef<string>("");

  // Set focus items for navbar
  useEffect(() => {
    if (!focusContext || !type?.ports) return;
    const items = type.ports.map((port) => ({
      id: port.guid,
      label: port.description || `Port ${port.guid.substring(0, 8)}`,
      category: "Ports",
    }));
    const itemsKey = items.map((item) => `${item.id}:${item.label}`).join("|");
    if (prevItemsRef.current !== itemsKey) {
      prevItemsRef.current = itemsKey;
      focusContext.setFocusItems(items);
    }
  }, [focusContext, type?.ports]);

  // Register focus handler for navbar focus
  useEffect(() => {
    if (!focusContext) return;
    const handleFocus = (itemId: string) => {
      focusPort(itemId);
    };
    focusContext.setOnFocusItem(handleFocus);
    return () => {
      if (focusContext) focusContext.setOnFocusItem(undefined);
    };
  }, [focusContext, focusPort]);

  const currentTool = useMemo(() => TypeAppTools.find((tool) => tool.id === activeTool), [activeTool]);

  const toolContribution = useMemo(() => {
    if (!currentTool || !kit || !appState) return null;
    return currentTool.render({
      state: appState as any,
      selection: selection as any,
      kit: kit as any,
      activeTool,
    });
  }, [currentTool, kit, appState, selection, activeTool]);

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

        if (kitCommands) {
          kitCommands.updateType(type.guid, {
            ports: {
              added: [newPort],
            },
          });
        }
      }
    },
    [type, kit, kitCommands],
  );

  const handleClearPreview = useCallback(() => {
    setPortPreview(null);
    clearHover();
  }, [clearHover]);

  const handlePortClick = useCallback(
    (portId: string) => {
      const isSelected = selection?.ports?.includes(portId) || false;
      if (activeTool === ToolType.SELECTION_ADDITIVE) {
        if (!isSelected) selectPort(portId);
      } else if (activeTool === ToolType.SELECTION_SUBTRACTIVE) {
        if (isSelected) deselectPort(portId);
      } else {
        const currentPorts = selection?.ports ?? [];
        if (currentPorts.length > 0) {
          currentPorts.forEach((id) => deselectPort(id));
        }
        if (!isSelected || currentPorts.length > 1) {
          selectPort(portId);
        }
      }
    },
    [selection, selectPort, deselectPort, activeTool],
  );

  const handlePortHover = useCallback(
    (portId: string) => {
      hoverPort(portId);
    },
    [hoverPort],
  );

  const handlePortLeave = useCallback(() => {
    clearHover();
  }, [clearHover]);

  const handlePortDoubleClick = useCallback(
    (portId: string) => {
      focusPort(portId);
    },
    [focusPort],
  );

  return (
    <>
      {toolContribution?.scene || (
        <>
          <TypeMesh activeTool={activeTool} onPortPreview={handlePortPreview} onPortCreate={handlePortCreate} onClearPreview={handleClearPreview} />
          {type?.ports?.map((port) => {
            const isSelected = selection?.ports?.includes(port.guid) || false;
            const isHovered = hover?.port === port.guid;
            return <PortVisual key={port.guid} port={port} isSelected={isSelected} isHovered={isHovered} onHover={() => handlePortHover(port.guid)} onLeave={handlePortLeave} onClick={() => handlePortClick(port.guid)} onDoubleClick={() => handlePortDoubleClick(port.guid)} />;
          })}
          {portPreview && <PortPreview position={portPreview.position} normal={portPreview.normal} />}
        </>
      )}
    </>
  );
};

const Scene: FC = () => {
  const { setCamera, deselectAll, clearFocus } = useTypeAppCommands();
  const camera = useTypeAppCamera();
  const focusedPortGuid = useTypeAppFocusedPortGuid();

  const onCameraChange = useCallback(
    (newCamera: { position: { x: number; y: number; z: number }; forward: { x: number; y: number; z: number }; up: { x: number; y: number; z: number } }) => {
      setCamera(newCamera);
    },
    [setCamera],
  );

  const onPointerMissed = useCallback(
    (event: MouseEvent) => {
      if (!(event.ctrlKey || event.metaKey) && !event.shiftKey) deselectAll();
    },
    [deselectAll],
  );

  const onFocusComplete = useCallback(() => {
    clearFocus();
  }, [clearFocus]);

  return (
    <SceneComponent camera={camera} onCameraChange={onCameraChange} onPointerMissed={onPointerMissed} focusedItemId={focusedPortGuid} onFocusComplete={onFocusComplete}>
      <SceneContent />
    </SceneComponent>
  );
};

export default Scene;

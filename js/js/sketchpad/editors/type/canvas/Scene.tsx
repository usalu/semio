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
import { ThreeEvent, useThree } from "@react-three/fiber";
import React, { FC, useCallback, useEffect, useMemo, useRef, useState } from "react";
import * as THREE from "three";
import SceneComponent from "../../../../elements/Scene";
import { guid, Point, Port, Type, Vector } from "../../../../semio";
import { useKit, useKitCommands, useType } from "../../../kits/store";
import { ToolType } from "../../../store";
import { useTypeEditorActiveTool, useTypeEditorCamera, useTypeEditorCommands, useTypeEditorHover, useTypeEditorSelection } from "../store";

const PortVisual: FC<{ port: Port; isSelected: boolean; isHovered: boolean; onHover: () => void; onLeave: () => void; onClick: () => void }> = ({ port, isSelected, isHovered, onHover, onLeave, onClick }) => {
  const position = useMemo(() => new THREE.Vector3(port.point.x, port.point.y, port.point.z), [port.point]);
  const direction = useMemo(() => new THREE.Vector3(port.direction.x, port.direction.y, port.direction.z).normalize(), [port.direction]);
  const color = isSelected ? 0xff0000 : isHovered ? 0xffff00 : 0x0000ff;

  const arrow = useMemo(() => {
    const arrowHelper = new THREE.ArrowHelper(direction, position, 0.5, color, 0.1, 0.05);
    return arrowHelper;
  }, [direction, position, color]);

  return (
    <group onPointerEnter={onHover} onPointerLeave={onLeave} onClick={(e) => { e.stopPropagation(); onClick(); }}>
      <primitive object={arrow} />
    </group>
  );
};

const PortPreview: FC<{ position: THREE.Vector3; normal: THREE.Vector3 }> = ({ position, normal }) => {
  const arrow = useMemo(() => new THREE.ArrowHelper(normal.normalize(), position, 0.5, 0x00ff00, 0.1, 0.05), [position, normal]);

  return <primitive object={arrow} />;
};

const TypeMesh: FC<{ activeTool: ToolType; onPortPreview: (position: THREE.Vector3, normal: THREE.Vector3) => void; onPortCreate: (position: THREE.Vector3, normal: THREE.Vector3) => void; onClearPreview: () => void }> = ({ activeTool, onPortPreview, onPortCreate, onClearPreview }) => {
  const type = useType() as Type | undefined;
  const kit = useKit();
  const representationUrl = useMemo(() => {
    if (!type?.representations?.[0]) return null;
    const url = type.representations[0].url;
    if (url.startsWith("http")) return url;
    return null;
  }, [type]);

  const { scene } = useGLTF(representationUrl || "");

  const handlePointerMove = useCallback(
    (event: ThreeEvent<PointerEvent>) => {
      if (activeTool === ToolType.PORT && event.face) {
        event.stopPropagation();
        const position = new THREE.Vector3().copy(event.point);
        const normal = event.face.normal.clone().normalize();
        onPortPreview(position, normal);
      }
    },
    [activeTool, onPortPreview],
  );

  const handleClick = useCallback(
    (event: ThreeEvent<MouseEvent>) => {
      if (activeTool === ToolType.PORT && event.face) {
        event.stopPropagation();
        const position = new THREE.Vector3().copy(event.point);
        const normal = event.face.normal.clone().normalize();
        onPortCreate(position, normal);
      }
    },
    [activeTool, onPortCreate],
  );

  if (!representationUrl) {
    return (
      <mesh onPointerMove={handlePointerMove} onClick={handleClick} onPointerOut={onClearPreview}>
        <boxGeometry args={[1, 1, 1]} />
        <meshStandardMaterial color="gray" />
      </mesh>
    );
  }

  return <primitive object={scene} onPointerMove={handlePointerMove} onClick={handleClick} onPointerOut={onClearPreview} />;
};

const SceneContent: FC = () => {
  const activeTool = useTypeEditorActiveTool();
  const type = useType() as Type | undefined;
  const kit = useKit();
  const kitCommands = useKitCommands();
  const selection = useTypeEditorSelection();
  const hover = useTypeEditorHover();
  const { selectPort, hoverPort, clearHover } = useTypeEditorCommands();

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

  return (
    <>
      <TypeMesh activeTool={activeTool} onPortPreview={handlePortPreview} onPortCreate={handlePortCreate} onClearPreview={handleClearPreview} />
      {type?.ports?.map((port) => (
        <PortVisual key={port.guid} port={port} isSelected={selection?.ports?.includes(port.guid) || false} isHovered={hover?.port === port.guid} onHover={() => hoverPort(port.guid)} onLeave={() => clearHover()} onClick={() => selectPort(port.guid)} />
      ))}
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

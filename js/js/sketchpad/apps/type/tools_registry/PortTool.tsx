// #region Header

// PortTool.tsx

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

import { Line, Sphere, useFBX, useGLTF } from "@react-three/drei";
import { ThreeEvent, useLoader } from "@react-three/fiber";
import { Crosshair } from "lucide-react";
import { FC, Suspense, useCallback, useEffect, useMemo, useRef, useState } from "react";
import * as THREE from "three";
import { OBJLoader } from "three/addons/loaders/OBJLoader.js";
import { guid, Kit, Point, Port, Type, Vector } from "../../../../semio";
import { Tool, ToolRenderContext } from "../../../Tool";
import { KitStore, useKit, useKitCommands, useKitStore, useType } from "../../../kits/store";
import { ToolType } from "../../../store";
import { TypeAppState, useTypeAppSelectedRepresentationGuid } from "../store";

const PortVisual: FC<{ port: Port; isSelected: boolean; isHovered: boolean; onHover: () => void; onLeave: () => void; onClick: () => void }> = ({ port, isSelected, isHovered, onHover, onLeave, onClick }) => {
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

  const arrowLength = 0.5;
  const endPoint = useMemo(() => [position[0] + direction[0] * arrowLength, position[1] + direction[1] * arrowLength, position[2] + direction[2] * arrowLength] as [number, number, number], [position, direction]);
  const points = useMemo(() => [position, endPoint], [position, endPoint]);

  const handlePointerEvent = useCallback(
    (callback: () => void) => (e: ThreeEvent<PointerEvent>) => {
      e.stopPropagation();
      callback();
    },
    [],
  );

  return (
    <group onPointerEnter={handlePointerEvent(onHover)} onPointerLeave={handlePointerEvent(onLeave)} onClick={handlePointerEvent(onClick)}>
      <Sphere args={[0.03]} position={position}>
        <meshBasicMaterial color={color} />
      </Sphere>
      <Line points={points} color={color} lineWidth={2} />
      <Sphere args={[0.05]} position={endPoint}>
        <meshBasicMaterial color={color} />
      </Sphere>
    </group>
  );
};

const PortPreview: FC<{ position: THREE.Vector3; normal: THREE.Vector3 }> = ({ position, normal }) => {
  const previewColor = "#00ff00";

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

// Separate components for each loader type to avoid conditional hook calls
const GLTFMesh: FC<{ url: string; onPointerDown: any; onPointerUp: any; onPointerMove: any; onPointerOut: any }> = ({ url, onPointerDown, onPointerUp, onPointerMove, onPointerOut }) => {
  const gltf = useGLTF(url);
  const clonedScene = useMemo(() => {
    const cloned = gltf.scene.clone();
    cloned.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        child.raycast = THREE.Mesh.prototype.raycast;
      }
    });
    return cloned;
  }, [gltf.scene]);
  return <primitive object={clonedScene} onPointerDown={onPointerDown} onPointerUp={onPointerUp} onPointerMove={onPointerMove} onPointerOut={onPointerOut} />;
};

const FBXMesh: FC<{ url: string; onPointerDown: any; onPointerUp: any; onPointerMove: any; onPointerOut: any }> = ({ url, onPointerDown, onPointerUp, onPointerMove, onPointerOut }) => {
  const scene = useFBX(url);
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

const OBJMesh: FC<{ url: string; onPointerDown: any; onPointerUp: any; onPointerMove: any; onPointerOut: any }> = ({ url, onPointerDown, onPointerUp, onPointerMove, onPointerOut }) => {
  const obj = useLoader(OBJLoader, url);
  const clonedScene = useMemo(() => {
    const cloned = obj.clone();
    cloned.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        child.raycast = THREE.Mesh.prototype.raycast;
      }
    });
    return cloned;
  }, [obj]);
  return <primitive object={clonedScene} onPointerDown={onPointerDown} onPointerUp={onPointerUp} onPointerMove={onPointerMove} onPointerOut={onPointerOut} />;
};

const LoadedTypeMesh: FC<{
  url: string;
  fileExtension: string;
  onPointerDown: (e: ThreeEvent<PointerEvent>) => void;
  onPointerUp: (e: ThreeEvent<PointerEvent>) => void;
  onPointerMove: (e: ThreeEvent<PointerEvent>) => void;
  onPointerOut: (e: ThreeEvent<PointerEvent>) => void;
}> = ({ url, fileExtension, onPointerDown, onPointerUp, onPointerMove, onPointerOut }) => {
  const ext = fileExtension.toLowerCase();

  // Use separate components to avoid conditional hook calls
  if (ext === "glb" || ext === "gltf") {
    return <GLTFMesh url={url} onPointerDown={onPointerDown} onPointerUp={onPointerUp} onPointerMove={onPointerMove} onPointerOut={onPointerOut} />;
  } else if (ext === "fbx") {
    return <FBXMesh url={url} onPointerDown={onPointerDown} onPointerUp={onPointerUp} onPointerMove={onPointerMove} onPointerOut={onPointerOut} />;
  } else if (ext === "obj") {
    return <OBJMesh url={url} onPointerDown={onPointerDown} onPointerUp={onPointerUp} onPointerMove={onPointerMove} onPointerOut={onPointerOut} />;
  } else {
    // Default to GLTF for unknown types
    return <GLTFMesh url={url} onPointerDown={onPointerDown} onPointerUp={onPointerUp} onPointerMove={onPointerMove} onPointerOut={onPointerOut} />;
  }
};

const TypeMesh: FC<{ onPortPreview: (position: THREE.Vector3, normal: THREE.Vector3) => void; onPortCreate: (position: THREE.Vector3, normal: THREE.Vector3) => void; onClearPreview: () => void }> = ({
  onPortPreview,
  onPortCreate,
  onClearPreview,
}) => {
  const type = useType(undefined, undefined, true) as Type | undefined; // Use deep observation for representations
  const kit = useKit(undefined, undefined, true) as Kit | undefined; // Use deep observation for files
  const kitStore = useKitStore() as KitStore;
  const selectedRepresentationGuid = useTypeAppSelectedRepresentationGuid();
  const [isPointerDown, setIsPointerDown] = useState(false);
  const pointerDownTimeRef = useRef<number>(0);
  const pointerDownPositionRef = useRef<{ x: number; y: number }>({ x: 0, y: 0 });

  // State to hold blob URL that needs cleanup
  const [blobUrl, setBlobUrl] = useState<string | null>(null);

  const { representationUrl, fileExtension, fileGuid } = useMemo(() => {
    if (!type?.representations || type.representations.length === 0) {
      return { representationUrl: null, fileExtension: "", fileGuid: null };
    }

    // Find the selected representation or use the first one
    const representation = selectedRepresentationGuid ? type.representations.find((r) => r.guid === selectedRepresentationGuid) : type.representations[0];

    if (!representation) {
      return { representationUrl: null, fileExtension: "", fileGuid: null };
    }

    // Get the file and resolve its URL
    const file = kit?.files?.find((f) => f.guid === representation.file);
    if (!file) {
      return { representationUrl: null, fileExtension: "", fileGuid: null };
    }

    // Extract file extension
    const ext = file.path.split(".").pop() || "";

    // Use kitStore to get the file URL through the file provider
    const url = kitStore.getFileUrl(file.guid);
    if (!url) {
      return { representationUrl: null, fileExtension: ext, fileGuid: file.guid };
    }

    return { representationUrl: url, fileExtension: ext, fileGuid: file.guid };
  }, [type, kit, kitStore, selectedRepresentationGuid]);

  // Convert file provider URLs to blob URLs that Three.js can load
  useEffect(() => {
    if (!fileGuid) {
      setBlobUrl(null);
      return;
    }

    let cancelled = false;
    let currentBlobUrl: string | null = null;

    (async () => {
      try {
        const url = await kitStore.getFileBlobUrl(fileGuid);
        if (!cancelled && url) {
          currentBlobUrl = url;
          setBlobUrl(url);
        }
      } catch (error) {
        console.error("[TypeMesh] Failed to get blob URL:", error);
      }
    })();

    // Cleanup on unmount or when fileGuid changes
    return () => {
      cancelled = true;
      if (currentBlobUrl && currentBlobUrl.startsWith("blob:")) {
        URL.revokeObjectURL(currentBlobUrl);
      }
    };
  }, [fileGuid, kitStore]);

  const handlePointerDown = useCallback((event: ThreeEvent<PointerEvent>) => {
    setIsPointerDown(true);
    pointerDownTimeRef.current = Date.now();
    pointerDownPositionRef.current = { x: event.clientX, y: event.clientY };
  }, []);

  const handlePointerUp = useCallback(
    (event: ThreeEvent<PointerEvent>) => {
      if (isPointerDown) {
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
    [isPointerDown, onPortCreate],
  );

  const handlePointerMove = useCallback(
    (event: ThreeEvent<PointerEvent>) => {
      if (event.face && !isPointerDown) {
        event.stopPropagation();
        const position = new THREE.Vector3().copy(event.point);
        const normal = event.face.normal.clone();
        const normalMatrix = new THREE.Matrix3().getNormalMatrix((event.object as THREE.Mesh).matrixWorld);
        normal.applyMatrix3(normalMatrix).normalize();
        onPortPreview(position, normal);
      }
    },
    [isPointerDown, onPortPreview],
  );

  const handlePointerOut = useCallback(() => {
    onClearPreview();
    setIsPointerDown(false);
  }, [onClearPreview]);

  if (!blobUrl) {
    return null; // No placeholder - just render nothing if no valid blob URL yet
  }

  return (
    <Suspense fallback={null}>
      <LoadedTypeMesh url={blobUrl} fileExtension={fileExtension} onPointerDown={handlePointerDown} onPointerUp={handlePointerUp} onPointerMove={handlePointerMove} onPointerOut={handlePointerOut} />
    </Suspense>
  );
};

const PortToolContent: FC<ToolRenderContext<TypeAppState>> = ({ state, selection, kit }) => {
  const type = useType() as Type | undefined;
  const kitCommands = useKitCommands();
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

        kitCommands.updateType("semio.sketchpad.app.type.tool.port.addPort", type.guid, {
          ports: {
            added: [newPort],
          },
        });
      }
    },
    [type, kit, kitCommands],
  );

  const handleClearPreview = useCallback(() => {
    setPortPreview(null);
  }, []);

  const handlePortClick = useCallback(
    (portId: string) => {
      const isSelected = (selection as any)?.ports?.includes(portId) || false;
      const currentPorts = (selection as any)?.ports ?? [];
      if (currentPorts.length > 0) {
        currentPorts.forEach((id: string) => {});
      }
      if (!isSelected || currentPorts.length > 1) {
      }
    },
    [selection],
  );

  const handlePortHover = useCallback((portId: string) => {}, []);

  const handlePortLeave = useCallback(() => {}, []);

  return (
    <>
      <TypeMesh onPortPreview={handlePortPreview} onPortCreate={handlePortCreate} onClearPreview={handleClearPreview} />
      {type?.ports?.map((port) => {
        const isSelected = (selection as any)?.ports?.includes(port.guid) || false;
        const isHovered = (state as any)?.hover?.port === port.guid;
        return <PortVisual key={port.guid} port={port} isSelected={isSelected} isHovered={isHovered} onHover={() => handlePortHover(port.guid)} onLeave={handlePortLeave} onClick={() => handlePortClick(port.guid)} />;
      })}
      {portPreview && <PortPreview position={portPreview.position} normal={portPreview.normal} />}
    </>
  );
};

export const PortTool: Tool<TypeAppState> = {
  id: ToolType.PORT,
  label: "tools.port.label",
  tooltipId: "tools.port.addAndEdit",
  icon: <Crosshair className="h-4 w-4" />,
  hotkey: "2",
  render: (context: ToolRenderContext<TypeAppState>) => ({
    scene: <PortToolContent {...context} />,
  }),
};

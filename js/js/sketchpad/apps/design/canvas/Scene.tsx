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

import { Edges, Line, Select, useFBX, useGLTF } from "@react-three/drei";
import { ThreeEvent, useLoader } from "@react-three/fiber";
import React, { FC, Suspense, useCallback, useEffect, useMemo, useState } from "react";
import * as THREE from "three";
import { OBJLoader } from "three/addons/loaders/OBJLoader.js";
import Scene, { Model, TransformableModel } from "../../../../elements/windows/Scene";
import { Camera, Design, DiffStatus, Kit, Piece, Plane, planeToMatrix, Representation, selectBestRepresentation, toThreeRotation, Type } from "../../../../semio";
import { KitStore, PieceScopeProvider, useDesign, useKit, useKitStore, usePiece, useType } from "../../../kits/store";
import { useDiffedPiece, useIsPieceSelected, useIsPieceTransitiveHovered, usePieceStatus } from "../../../kits/designAppIntegration";
import { useAppPanelVisibility } from "../../../store";
import { DesignAppFullscreenWindow, DesignAppPresenceOther, useDesignAppCamera, useDesignAppCommands, useDesignAppFocusedPieceGuid, useDesignAppFullscreen, useDesignAppOthers, useDesignAppPieceColor, useDesignAppSelectedRepresentationTags, useDesignAppSelection } from "../store";
import { SharedTransformControls } from "./SharedTransformControls";

const getComputedColor = (variable: string): string => getComputedStyle(document.documentElement).getPropertyValue(variable).trim();

const PresenceThree: FC<DesignAppPresenceOther> = ({ name, cursor, camera }) => {
  if (!camera) return null;
  const cameraHelper = useMemo(() => {
    const perspectiveCamera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1);
    perspectiveCamera.position.set(camera.position.x, camera.position.y, camera.position.z);
    perspectiveCamera.lookAt(new THREE.Vector3(camera.forward.x, camera.forward.y, camera.forward.z));
    perspectiveCamera.updateProjectionMatrix();
    perspectiveCamera.updateMatrixWorld();
    return new THREE.CameraHelper(perspectiveCamera);
  }, [camera.position.x, camera.position.y, camera.position.z, camera.forward.x, camera.forward.y, camera.forward.z]);

  return <primitive object={cameraHelper} />;
};

interface PlaneThreeProps {
  plane: Plane;
}

const PlaneThree: FC<PlaneThreeProps> = ({ plane }) => {
  const matrix = useMemo(() => planeToMatrix(plane), [plane]);
  return (
    <group matrix={matrix} matrixAutoUpdate={false}>
      <Line points={[new THREE.Vector3(0, 0, 0), new THREE.Vector3(1, 0, 0)]} color={new THREE.Color(getComputedColor("--color-primary"))} />
      <Line points={[new THREE.Vector3(0, 0, 0), new THREE.Vector3(0, 1, 0)]} color={new THREE.Color(getComputedColor("--color-primary"))} />
    </group>
  );
};

const GLTFMesh: FC<{ url: string }> = ({ url }) => {
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
  return <primitive object={clonedScene} />;
};

const FBXMesh: FC<{ url: string }> = ({ url }) => {
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
  return <primitive object={clonedScene} />;
};

const OBJMesh: FC<{ url: string }> = ({ url }) => {
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
  return <primitive object={clonedScene} />;
};

const LoadedPieceMesh: FC<{ url: string; fileExtension: string }> = ({ url, fileExtension }) => {
  const ext = fileExtension.toLowerCase();
  if (ext === "glb" || ext === "gltf") {
    return <GLTFMesh url={url} />;
  } else if (ext === "fbx") {
    return <FBXMesh url={url} />;
  } else if (ext === "obj") {
    return <OBJMesh url={url} />;
  } else {
    return <GLTFMesh url={url} />;
  }
};

const PieceMesh: FC = () => {
  const piece = usePiece() as Piece;
  const type = useType(undefined, piece.type) as Type | undefined;
  const kit = useKit(undefined, undefined, true) as Kit | undefined;
  const kitStore = useKitStore() as KitStore;
  const selectedRepresentationTags = useDesignAppSelectedRepresentationTags();
  const [blobUrl, setBlobUrl] = useState<string | null>(null);

  const { representationUrl, fileExtension, fileGuid } = useMemo(() => {
    if (!type?.representations || type.representations.length === 0) {
      return { representationUrl: null, fileExtension: "", fileGuid: null };
    }
    const tagsForType = selectedRepresentationTags[type.guid] ?? [];
    let representation: Representation | undefined;
    if (tagsForType.length > 0) {
      representation = selectBestRepresentation(type.representations, tagsForType);
    } else {
      const defaultRep = type.representations.find((r) => !r.tags || r.tags.length === 0);
      representation = defaultRep ?? type.representations[0];
    }
    if (!representation) {
      return { representationUrl: null, fileExtension: "", fileGuid: null };
    }
    const file = kit?.files?.find((f) => f.guid === representation.file);
    if (!file) {
      return { representationUrl: null, fileExtension: "", fileGuid: null };
    }
    const ext = file.path.split(".").pop() || "";
    const url = kitStore.getFileUrl(file.guid);
    if (!url) {
      return { representationUrl: null, fileExtension: ext, fileGuid: file.guid };
    }
    return { representationUrl: url, fileExtension: ext, fileGuid: file.guid };
  }, [type, kit, kitStore, selectedRepresentationTags]);

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
        console.error("[PieceMesh] Failed to get blob URL:", error);
      }
    })();
    return () => {
      cancelled = true;
      if (currentBlobUrl && currentBlobUrl.startsWith("blob:")) {
        URL.revokeObjectURL(currentBlobUrl);
      }
    };
  }, [fileGuid, kitStore]);

  if (!blobUrl) {
    return null;
  }

  return (
    <Suspense fallback={null}>
      <LoadedPieceMesh url={blobUrl} fileExtension={fileExtension} />
    </Suspense>
  );
};

interface ModelPieceProps {}

/**
 * ModelPiece component - renders a Piece as a SceneModel.
 *
 * Implements the unified SceneModel abstraction:
 * - Hoverable: changes color on pointer enter/leave
 * - Clickable: handles selection with modifier keys (Ctrl/Cmd, Shift)
 * - Focusable: sets userData.id to piece.guid for camera zoom
 * - Has a semio plane: uses Piece.plane in semio coordinate system
 *
 * The piece's plane is converted to a Three.js matrix for rendering, and
 * userData.id is set to enable the focus behavior defined in Scene.tsx.
 */
const ModelPiece: FC<ModelPieceProps> = () => {
  const piece = usePiece() as Piece;
  const diffedPiece = useDiffedPiece() as Piece;
  const isSelected = useIsPieceSelected();
  const isHovered = useIsPieceTransitiveHovered();
  const status = usePieceStatus();

  const { selectPiece, removePieceFromSelection, addPieceToSelection, hoverPiece, clearHover, focusPiece } = useDesignAppCommands();

  // Use the same color logic as diagram nodes
  const { fill } = useDesignAppPieceColor(undefined, piece.guid);

  const foregroundColor = useMemo(() => getComputedColor("--foreground"), []);
  const mutedForegroundColor = useMemo(() => getComputedColor("--muted-foreground"), []);

  // Check if there's an actual plane difference (compare values, not just references)
  const hasDiff = useMemo(() => {
    if (status === DiffStatus.Unchanged) return false;
    if (!piece.plane || !diffedPiece.plane) return false;

    // Compare plane values
    const p1 = piece.plane;
    const p2 = diffedPiece.plane;
    return (
      p1.origin.x !== p2.origin.x ||
      p1.origin.y !== p2.origin.y ||
      p1.origin.z !== p2.origin.z ||
      p1.xAxis.x !== p2.xAxis.x ||
      p1.xAxis.y !== p2.xAxis.y ||
      p1.xAxis.z !== p2.xAxis.z ||
      p1.yAxis.x !== p2.yAxis.x ||
      p1.yAxis.y !== p2.yAxis.y ||
      p1.yAxis.z !== p2.yAxis.z
    );
  }, [status, piece.plane, diffedPiece.plane]);

  const onSelect = useCallback(
    (e?: ThreeEvent<MouseEvent>) => {
      if (e?.ctrlKey || e?.metaKey) {
        removePieceFromSelection("semio.sketchpad.app.design.canvas.scene.modelPiece.removePieceFromSelection", piece.guid);
      } else if (e?.shiftKey) {
        addPieceToSelection("semio.sketchpad.app.design.canvas.scene.modelPiece.addPieceToSelection", piece.guid);
      } else {
        selectPiece("semio.sketchpad.app.design.canvas.scene.modelPiece.selectPiece", piece.guid);
      }
    },
    [selectPiece, removePieceFromSelection, addPieceToSelection, piece.guid],
  );

  const onDoubleClick = useCallback(
    (e?: ThreeEvent<MouseEvent>) => {
      e?.stopPropagation();
      focusPiece("semio.sketchpad.app.design.canvas.scene.modelPiece.focusPiece", piece.guid);
    },
    [focusPiece, piece.guid],
  );
  // Get computed color including color-mix() resolution
  const materialColor = useMemo(() => {
    // First resolve CSS variables and color-mix() using DOM
    const tempDiv = document.createElement("div");
    tempDiv.style.position = "absolute";
    tempDiv.style.visibility = "hidden";
    tempDiv.style.pointerEvents = "none";
    document.body.appendChild(tempDiv);
    tempDiv.style.color = fill;
    const computedColor = getComputedStyle(tempDiv).color;
    document.body.removeChild(tempDiv);

    // Use Canvas 2D context to convert any color format (including OKLCH) to RGB
    // Canvas always returns colors in rgba(r, g, b, a) format
    const canvas = document.createElement("canvas");
    canvas.width = 1;
    canvas.height = 1;
    const ctx = canvas.getContext("2d");
    if (!ctx) return computedColor;

    ctx.fillStyle = computedColor;
    ctx.fillRect(0, 0, 1, 1);
    const imageData = ctx.getImageData(0, 0, 1, 1);
    const [r, g, b, a] = imageData.data;

    // If transparent (alpha = 0), use foreground color as fallback
    if (a === 0) {
      return foregroundColor;
    }

    // Return RGB format (ignore alpha since THREE.js materials handle transparency differently)
    return `rgb(${r}, ${g}, ${b})`;
  }, [fill, foregroundColor]);
  const emissiveColor = materialColor;

  // Original piece matrix
  const originalMatrix = useMemo(() => {
    if (!piece.plane) return null;
    const planeMatrix = planeToMatrix(piece.plane as Plane);
    // Apply coordinate system transformation: rotationMatrix * planeMatrix
    const threeMatrix = new THREE.Matrix4().multiplyMatrices(toThreeRotation(), planeMatrix);
    return threeMatrix;
  }, [piece.plane]);

  // Diffed piece matrix
  const diffedMatrix = useMemo(() => {
    if (!diffedPiece.plane || !hasDiff) return null;
    const planeMatrix = planeToMatrix(diffedPiece.plane as Plane);
    // Apply coordinate system transformation: rotationMatrix * planeMatrix
    const threeMatrix = new THREE.Matrix4().multiplyMatrices(toThreeRotation(), planeMatrix);
    return threeMatrix;
  }, [diffedPiece, hasDiff]);

  // Use diffed matrix for transform controls, original for visual reference
  const transformProps = useMemo(() => {
    const matrix = diffedMatrix || originalMatrix;
    if (!matrix || !piece.plane) return null;
    const position = new THREE.Vector3();
    const quaternion = new THREE.Quaternion();
    const scale = new THREE.Vector3();
    matrix.decompose(position, quaternion, scale);
    return { position, quaternion, scale };
  }, [diffedMatrix, originalMatrix, piece.plane]);

  // Original piece mesh (edges only when there's a diff)
  const originalMeshContent =
    hasDiff && originalMatrix ? (
      <group matrix={originalMatrix} matrixAutoUpdate={false}>
        <mesh>
          <boxGeometry args={[1, 1, 1]} />
          <meshStandardMaterial transparent opacity={0} />
          <Edges scale={1.001} color={mutedForegroundColor} />
        </mesh>
      </group>
    ) : null;

  // userData for making the model focusable by guid
  const userData = useMemo(() => ({ id: piece.guid }), [piece.guid]);

  // Diffed/current piece mesh using Model component
  const diffedMeshContent = piece.design ? (
    <Model
      selected={isSelected}
      hovered={isHovered}
      onClick={onSelect}
      onDoubleClick={onDoubleClick}
      onPointerEnter={() => hoverPiece("semio.sketchpad.app.design.canvas.scene.modelPiece.hoverPiece", piece.guid)}
      onPointerLeave={() => clearHover("semio.sketchpad.app.design.canvas.scene.modelPiece.clearHover")}
      color={materialColor}
      emissiveColor={emissiveColor}
      emissiveIntensity={0.45}
      showEdges
      edgeColor={foregroundColor}
      userData={userData}
    />
  ) : (
    <group
      onClick={onSelect}
      onDoubleClick={onDoubleClick}
      onPointerEnter={() => hoverPiece("semio.sketchpad.app.design.canvas.scene.modelPiece.hoverPiece", piece.guid)}
      onPointerLeave={() => clearHover("semio.sketchpad.app.design.canvas.scene.modelPiece.clearHover")}
    >
      <PieceMesh />
    </group>
  );

  // Render the piece at its current position (diffed or original)
  const pieceMatrix = diffedMatrix || originalMatrix;

  return (
    <>
      {originalMeshContent}
      {pieceMatrix && (
        <group matrix={pieceMatrix} matrixAutoUpdate={false}>
          {diffedMeshContent}
        </group>
      )}
    </>
  );

  // const transformControlRef = useRef(null);

  // const handleMouseDown = useCallback(
  //   (e?: THREE.Event) => {
  //     startTransaction();
  //   },
  //   [startTransaction],
  // );

  // const handleMouseUp = useCallback(
  //   (e?: THREE.Event) => {
  //     finalizeTransaction();
  //   },
  //   [finalizeTransaction],
  // );

  // // Handle escape key to abort transactions during transform
  // useEffect(() => {
  //   const handleEscape = (event: KeyboardEvent) => {
  //     if (event.key === "Escape" && selected && fixed) {
  //       abortTransaction();
  //     }
  //   };

  //   document.addEventListener("keydown", handleEscape);
  //   return () => document.removeEventListener("keydown", handleEscape);
  // }, [selected, fixed, abortTransaction]);

  // const transformControl = selected && fixed;
  // const userData = useMemo(() => ({ pieceId: piece.id_ }), [piece.id_]);
  // const group = (
  //   <group matrix={matrix} matrixAutoUpdate={false} userData={userData} onClick={onSelect}>
  //     <primitive object={styledScene} />
  //   </group>
  // );

  // if (transformControl)
  //   return (
  //     <TransformControls ref={transformControlRef} enabled={selected && fixed} onMouseDown={handleMouseDown} onMouseUp={handleMouseUp}>
  //       {group}
  //     </TransformControls>
  //   );

  // return group;
};

const ModelDesign: FC = () => {
  const commands = useDesignAppCommands();
  const selection = useDesignAppSelection();
  const others = useDesignAppOthers();
  const design = useDesign();
  const flatDesign = design as Design;

  const { selectPieces, startTransaction, finalizeTransaction, updatePiece } = commands;

  const onChange = useCallback(
    (selected: THREE.Object3D[]) => {
      const newSelectedPieceIds = selected.map((item) => item.parent?.userData.pieceId).filter(Boolean);
      if (newSelectedPieceIds.length !== selection.pieces?.length || newSelectedPieceIds.some((id, index) => id !== selection.pieces?.[index])) {
        selectPieces("semio.sketchpad.app.design.canvas.scene.modelDesign.selectPieces", newSelectedPieceIds);
      }
    },
    [selectPieces, selection.pieces],
  );

  // Convert pieces to TransformableModel format for SharedTransformControls
  const selectedModels = useMemo((): TransformableModel[] => {
    if (!selection.pieces || !flatDesign.pieces) return [];

    return flatDesign.pieces
      .filter((piece) => selection.pieces?.includes(piece.guid))
      .map((piece) => ({
        guid: piece.guid,
        plane: piece.plane,
        isTransformable: !piece.isLocked && piece.plane !== undefined,
        isSelected: true,
      }));
  }, [selection.pieces, flatDesign.pieces]);

  // Handle multi-model transform updates
  const handleMultiPlaneUpdate = useCallback(
    (updates: Array<{ modelGuid: string; newPlane: Plane }>) => {
      updates.forEach(({ modelGuid, newPlane }) => {
        updatePiece("semio.sketchpad.app.design.canvas.scene.modelDesign.updatePiece", modelGuid, { plane: newPlane });
      });
    },
    [updatePiece],
  );

  return (
    <>
      <Select box multiple onChange={onChange}>
        <group>
          {flatDesign.pieces?.map((piece: Piece) => (
            <PieceScopeProvider key={piece.guid} guid={piece.guid}>
              <ModelPiece />
            </PieceScopeProvider>
          ))}
          {others.map((presence, id) => (
            <PresenceThree key={id} {...presence} />
          ))}
        </group>
      </Select>

      {/* Single shared transform control for all selected models */}
      <SharedTransformControls
        selectedModels={selectedModels}
        onUpdate={handleMultiPlaneUpdate}
        onTransformStart={() => startTransaction("semio.sketchpad.app.design.canvas.scene.sharedTransformControls.start")}
        onTransformEnd={() => finalizeTransaction("semio.sketchpad.app.design.canvas.scene.sharedTransformControls.end")}
        mode="translate"
      />
    </>
  );
};

const DesignAppScene: FC = () => {
  const { deselectAll, toggleAccesslFullscreen, setCamera, clearFocus } = useDesignAppCommands();
  const fullscreen = useDesignAppFullscreen() === DesignAppFullscreenWindow.Accessl;
  const camera = useDesignAppCamera();
  const focusedPieceGuid = useDesignAppFocusedPieceGuid();
  const panelVisibility = useAppPanelVisibility();

  const onDoubleClickCapture = useCallback(
    (e: React.MouseEvent) => {
      toggleAccesslFullscreen("semio.sketchpad.app.design.canvas.scene.doubleClickCapture");
    },
    [toggleAccesslFullscreen],
  );
  const onPointerMissed = useCallback(
    (e: MouseEvent) => {
      if (!(e.ctrlKey || e.metaKey) && !e.shiftKey) deselectAll("semio.sketchpad.app.design.canvas.scene.pointerMissed");
    },
    [deselectAll],
  );
  const onCameraChange = useCallback(
    (newCamera: Camera) => {
      setCamera("semio.sketchpad.app.design.canvas.scene.cameraChange", newCamera);
    },
    [setCamera],
  );
  const onFocusComplete = useCallback(() => {
    // Small delay to ensure focus has completed before clearing
    setTimeout(() => {
      clearFocus("semio.sketchpad.app.design.canvas.scene.focusComplete");
    }, 100);
  }, [clearFocus]);

  return (
    <Scene showGizmo={fullscreen && !!panelVisibility.toolbar} camera={camera} onCameraChange={onCameraChange} onDoubleClickCapture={onDoubleClickCapture} onPointerMissed={onPointerMissed} focusedItemId={focusedPieceGuid} onFocusComplete={onFocusComplete}>
      <ModelDesign />
    </Scene>
  );
};

export default DesignAppScene;

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

import { Edges, Line, Select, TransformControls } from "@react-three/drei";
import React, { FC, useCallback, useEffect, useLayoutEffect, useMemo, useRef } from "react";
import * as THREE from "three";
import Scene, { Model } from "../../../../elements/windows/Scene";
import { Camera, Design, DiffStatus, matrixToPlane, Piece, Plane, planeToMatrix, toSemioRotation, toThreeRotation } from "../../../../semio";
import { PieceScopeProvider, useDesign, useDiffedPiece, useIsPieceSelected, useIsPieceTransitiveHovered, usePiece, usePiecePlane, usePieceStatus } from "../../../kits/store";
import { useEditorPanelVisibility } from "../../../store";
import { DesignEditorFullscreenWindow, DesignEditorPresenceOther, useDesignEditorCamera, useDesignEditorCommands, useDesignEditorFullscreen, useDesignEditorOthers, useDesignEditorPieceColor, useDesignEditorSelection } from "../store";

const getComputedColor = (variable: string): string => getComputedStyle(document.documentElement).getPropertyValue(variable).trim();

const PresenceThree: FC<DesignEditorPresenceOther> = ({ name, cursor, camera }) => {
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

interface ModelPieceProps {}

const ModelPiece: FC<ModelPieceProps> = () => {
  const piece = usePiece() as Piece;
  const diffedPiece = useDiffedPiece() as Piece;
  const isSelected = useIsPieceSelected();
  const selection = useDesignEditorSelection();
  const isHovered = useIsPieceTransitiveHovered();
  const piecePlane = usePiecePlane();
  const status = usePieceStatus();

  const { selectPiece, removePieceFromSelection, addPieceToSelection, hoverPiece, clearHover, updatePiece, startTransaction, finalizeTransaction } = useDesignEditorCommands();

  // Use the same color logic as diagram nodes
  const { fill } = useDesignEditorPieceColor(undefined, piece.guid);

  const transformControlsRef = useRef<any>(null);
  const groupRef = useRef<THREE.Group>(null);
  const isDraggingRef = useRef(false);
  const isUpdatingPlaneRef = useRef(false);
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

  // const piece = flatDesign.pieces?.[pieceIndex];
  // const plane = piecePlanes[pieceIndex];
  // const fileUrl = fileUrls.get(pieceRepresentationUrls.get(piece?.id_!)!)!;
  // const selected = selection.pieces?.some((id) => id.id_ === piece?.id_) ?? false;
  // const diffStatus = pieceDiffStatuses[pieceIndex] || DiffStatus.Unchanged;

  // if (!piece) return null;
  // const fixed = piece.plane !== undefined;
  // const matrix = useMemo(() => {
  //   const planeRotationMatrix = planeToMatrix(plane);
  //   planeRotationMatrix.multiply(toSemioRotation());
  //   return planeRotationMatrix;
  // }, [plane]);
  // const styledScene = useMemo(() => {
  //   const scene = useGLTF(fileUrl).scene.clone();
  //   let meshColor: THREE.Color;
  //   let meshOpacity = 1;
  //   let lineOpacity = 1;

  //   if (diffStatus === DiffStatus.Added) {
  //     meshColor = new THREE.Color(getComputedColor("--color-success"));
  //     if (selected) {
  //       const selectedColor = new THREE.Color(getComputedColor("--color-primary"));
  //       meshColor.lerp(selectedColor, 0.5);
  //     }
  //   } else if (diffStatus === DiffStatus.Modified) {
  //     meshColor = new THREE.Color(getComputedColor("--color-warning"));
  //     if (selected) {
  //       const selectedColor = new THREE.Color(getComputedColor("--color-primary"));
  //       meshColor.lerp(selectedColor, 0.5);
  //     }
  //   } else if (diffStatus === DiffStatus.Removed) {
  //     meshColor = new THREE.Color(getComputedColor("--color-error"));
  //     meshOpacity = 0.2;
  //     lineOpacity = 0.25;
  //     if (selected) {
  //       const selectedColor = new THREE.Color(getComputedColor("--color-primary"));
  //       meshColor.lerp(selectedColor, 0.5);
  //     }
  //   } else if (selected) {
  //     meshColor = new THREE.Color(getComputedColor("--color-primary"));
  //   } else {
  //     meshColor = new THREE.Color(getComputedColor("--color-light"));
  //   }

  //   const lineColor = new THREE.Color(getComputedColor("--color-dark"));
  //   scene.traverse((object) => {
  //     if (object instanceof THREE.Mesh) {
  //       object.material = new THREE.MeshBasicMaterial({
  //         color: meshColor,
  //         transparent: meshOpacity < 1,
  //         opacity: meshOpacity,
  //       });
  //     }
  //     if (object instanceof THREE.Line) {
  //       object.material = new THREE.LineBasicMaterial({
  //         color: lineColor,
  //         transparent: lineOpacity < 1,
  //         opacity: lineOpacity,
  //       });
  //     }
  //   });
  //   return scene;
  // }, [fileUrl, diffStatus, selected]);
  const onSelect = useCallback(
    (e?: MouseEvent) => {
      if (e?.ctrlKey || e?.metaKey) {
        removePieceFromSelection(piece.guid);
      } else if (e?.shiftKey) {
        addPieceToSelection(piece.guid);
      } else {
        selectPiece(piece.guid);
      }
    },
    [selectPiece, removePieceFromSelection, addPieceToSelection],
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

  // Listen to dragging-changed and object-change events from TransformControls
  useEffect(() => {
    const controls = transformControlsRef.current;
    console.log("Setting up event listeners for", piece.guid, "controls:", controls);

    if (!controls) {
      console.warn("No TransformControls ref available");
      return;
    }

    const updatePlaneFromTransform = () => {
      const transformedObject = controls.object;
      if (!transformedObject) {
        console.error("No object attached to TransformControls!");
        return;
      }

      isUpdatingPlaneRef.current = true;

      // Get the transform from position/rotation/scale that TransformControls modified
      const position = transformedObject.position.clone();
      const quaternion = transformedObject.quaternion.clone();
      const scale = transformedObject.scale.clone();

      // Compose the matrix from position, rotation, scale
      const threeMatrix = new THREE.Matrix4().compose(position, quaternion, scale);

      // Apply inverse coordinate transformation: inverseRotation * threeMatrix
      const semioMatrix = new THREE.Matrix4().multiplyMatrices(toSemioRotation(), threeMatrix);

      const newPlane = matrixToPlane(semioMatrix);
      console.log("Updating plane during drag:", JSON.stringify(newPlane, null, 2));

      updatePiece(piece.guid, { plane: newPlane });

      // Re-enable useLayoutEffect after a brief delay
      setTimeout(() => {
        isUpdatingPlaneRef.current = false;
      }, 10);
    };

    const handleDraggingChanged = (event: any) => {
      console.log("=== DRAGGING CHANGED ===", event.value, piece.guid);

      if (event.value) {
        // Started dragging
        console.log("Started dragging piece", piece.guid);
        isDraggingRef.current = true;
        startTransaction();
      } else {
        // Stopped dragging
        console.log("Stopped dragging piece", piece.guid);
        isDraggingRef.current = false;

        // Final update
        updatePlaneFromTransform();

        console.log("Calling finalizeTransaction...");
        finalizeTransaction();
        console.log("=== DONE ===");
      }
    };

    const handleObjectChange = () => {
      // Only update while actively dragging
      if (isDraggingRef.current) {
        updatePlaneFromTransform();
      }
    };

    console.log("Adding event listeners to controls");
    controls.addEventListener("dragging-changed", handleDraggingChanged);
    controls.addEventListener("objectChange", handleObjectChange);

    return () => {
      console.log("Removing event listeners from controls");
      controls.removeEventListener("dragging-changed", handleDraggingChanged);
      controls.removeEventListener("objectChange", handleObjectChange);
    };
  }, [piece.guid, startTransaction, updatePiece, finalizeTransaction]);

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

  // Diffed/current piece mesh using Model component
  const diffedMeshContent = (
    <Model
      selected={isSelected}
      hovered={isHovered}
      onClick={onSelect}
      onPointerEnter={() => hoverPiece(piece.guid)}
      onPointerLeave={() => clearHover()}
      color={materialColor}
      emissiveColor={emissiveColor}
      emissiveIntensity={0.45}
      showEdges
      edgeColor={foregroundColor}
    />
  );

  const hasValidPlane = !!(piece.plane && transformProps);

  useLayoutEffect(() => {
    if (groupRef.current && transformProps && !isDraggingRef.current && !isUpdatingPlaneRef.current) {
      groupRef.current.position.copy(transformProps.position);
      groupRef.current.quaternion.copy(transformProps.quaternion);
      groupRef.current.scale.copy(transformProps.scale);
    }
  }, [transformProps]);

  return (
    <>
      {originalMeshContent}
      <TransformControls key={piece.guid} ref={transformControlsRef} enabled={hasValidPlane && isSelected} visible={hasValidPlane} mode="translate">
        <group ref={groupRef}>{diffedMeshContent}</group>
      </TransformControls>
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
  const commands = useDesignEditorCommands();
  const selection = useDesignEditorSelection();
  // const fileUrls = useFileUrls();
  const others = useDesignEditorOthers();
  const design = useDesign();
  // const flatDesign = useFlatDesign();
  const flatDesign = design as Design;
  // const pieceRepresentationUrls = usePieceRepresentationUrls();

  const { selectPieces, startTransaction, finalizeTransaction, abortTransaction } = commands;

  // useEffect(() => {
  //   fileUrls.forEach((url, id) => {
  //     useGLTF.preload(id);
  //   });
  // }, [fileUrls]);

  // useEffect(() => {
  //   flatDesign.pieces?.forEach((p: Piece) => {
  //     if (!p.type) {
  //       console.warn(`No type defined for piece ${p.id_}`);
  //       return;
  //     }
  //     const type = types.find((t) => t.name === p.type?.name && (t.variant || "") === (p.type?.variant || ""));
  //     if (!type) throw new Error(`Type (${p.type.name}, ${p.type.variant}) for piece ${p.id_} not found`);
  //   });
  // }, [flatDesign.pieces, types]);

  // useEffect(() => {
  //   pieceRepresentationUrls.forEach((url, id) => {
  //     if (!fileUrls.has(url)) throw new Error(`Representation url ${url} for piece ${id} not found in fileUrls map`);
  //   });
  // }, [pieceRepresentationUrls, fileUrls]);

  const onChange = useCallback(
    (selected: THREE.Object3D[]) => {
      const newSelectedPieceIds = selected.map((item) => item.parent?.userData.pieceId).filter(Boolean);
      if (newSelectedPieceIds.length !== selection.pieces?.length || newSelectedPieceIds.some((id, index) => id !== selection.pieces?.[index])) {
        selectPieces(newSelectedPieceIds);
      }
    },
    [selectPieces],
  );

  return (
    <Select box multiple onChange={onChange}>
      <group>
        {/* <group quaternion={new THREE.Quaternion(-0.7071067811865476, 0, 0, 0.7071067811865476)}> */}
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
  );
};

const DesignEditorScene: FC = () => {
  const { deselectAll, toggleAccesslFullscreen, setCamera } = useDesignEditorCommands();
  const fullscreen = useDesignEditorFullscreen() === DesignEditorFullscreenWindow.Accessl;
  const camera = useDesignEditorCamera();
  const panelVisibility = useEditorPanelVisibility();
  const onDoubleClickCapture = useCallback(
    (e: React.MouseEvent) => {
      toggleAccesslFullscreen();
    },
    [toggleAccesslFullscreen],
  );
  const onPointerMissed = useCallback(
    (e: MouseEvent) => {
      if (!(e.ctrlKey || e.metaKey) && !e.shiftKey) deselectAll();
    },
    [deselectAll],
  );
  const onCameraChange = useCallback(
    (newCamera: Camera) => {
      setCamera(newCamera);
    },
    [setCamera],
  );

  return (
    <Scene showGizmo={fullscreen && !!panelVisibility.toolbar} camera={camera} onCameraChange={onCameraChange} onDoubleClickCapture={onDoubleClickCapture} onPointerMissed={onPointerMissed}>
      <ModelDesign />
    </Scene>
  );
};

export default DesignEditorScene;

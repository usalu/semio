// #region Header

// DesignModel.tsx

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

import { Edges, Line, Select } from "@react-three/drei";
import React, { FC, useCallback, useMemo } from "react";
import * as THREE from "three";
import Scene from "../../../../elements/Scene";
import { Camera, Piece, Plane, planeToMatrix } from "../../../../semio";
import { PieceScopeProvider, useDesign, useIsPieceSelected, useIsPieceTransitiveHovered, usePiece, usePiecePlane, usePieceStatus } from "../../../kits/store";
import { useEditorPanelVisibility } from "../../../store";
import { DesignEditorFullscreenWindow, DesignEditorPresenceOther, useDesignEditorCamera, useDesignEditorCommands, useDesignEditorFullscreen, useDesignEditorOthers, useDesignEditorSelection } from "../store";

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

const ModelPiece: FC<ModelPieceProps> = React.memo(() => {
  // const flatDesign = useFlatDesign();
  // const piecePlanes = usePiecePlanes();
  // const pieceRepresentationUrls = usePieceRepresentationUrls();
  // const pieceDiffStatuses = usePieceDiffStatuses();
  // const fileUrls = useFileUrls();
  const piece = usePiece() as Piece;
  const isSelected = useIsPieceSelected();
  const selection = useDesignEditorSelection();
  const isHovered = useIsPieceTransitiveHovered();
  const piecePlane = usePiecePlane();
  const status = usePieceStatus();

  const { selectPiece, removePieceFromSelection, addPieceToSelection, hoverPiece, clearHover } = useDesignEditorCommands();
  const plasterColor = useMemo(() => getComputedColor("--plaster"), []);
  const foregroundColor = useMemo(() => getComputedColor("--foreground"), []);
  const hoverColor = useMemo(() => getComputedColor("--hover-base"), []);
  const activeColor = useMemo(() => getComputedColor("--active-base"), []);

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
  const materialColor = useMemo(() => {
    if (isSelected) return activeColor;
    if (isHovered) return hoverColor;
    return plasterColor;
  }, [isSelected, isHovered, activeColor, plasterColor, plasterColor]);
  const emissiveColor = isSelected ? activeColor : isHovered ? hoverColor : plasterColor;
  return (
    <mesh onClick={onSelect} onPointerEnter={() => hoverPiece(piece.guid)} onPointerLeave={() => clearHover()}>
      <boxGeometry args={[1, 1, 1]} />
      <meshStandardMaterial color={materialColor} emissive={emissiveColor} emissiveIntensity={0.45} />
      <Edges scale={1.001} color={foregroundColor} />
    </mesh>
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
});

const ModelDesign: FC = () => {
  const commands = useDesignEditorCommands();
  const selection = useDesignEditorSelection();
  // const fileUrls = useFileUrls();
  const others = useDesignEditorOthers();
  const design = useDesign();
  // const flatDesign = useFlatDesign();
  const flatDesign = design;
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

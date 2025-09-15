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

import { GizmoHelper, GizmoViewport, Grid, Line, OrbitControls, OrthographicCamera, Select } from "@react-three/drei";
import { Canvas } from "@react-three/fiber";
import React, { FC, useCallback, useEffect, useMemo, useRef, useState } from "react";
import * as THREE from "three";
import { Piece, Plane, planeToMatrix } from "../../../semio";
import {
  DesignEditorFullscreenPanel,
  DesignEditorPresenceOther,
  PieceScopeProvider,
  useDesign,
  useDesignEditorCommands,
  useDesignEditorFullscreen,
  useDesignEditorOthers,
  useDesignEditorSelection,
  useIsPieceHovered,
  useIsPieceSelected,
  usePiece,
  usePiecePlane,
  usePieceStatus,
} from "../../../store";

const getComputedColor = (variable: string): string => {
  return getComputedStyle(document.documentElement).getPropertyValue(variable).trim();
};

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
  const isHovered = useIsPieceHovered();
  const piecePlane = usePiecePlane();
  const status = usePieceStatus();

  const { selectPiece, removePieceFromSelection, addPieceToSelection, startTransaction, finalizeTransaction, abortTransaction } = useDesignEditorCommands();

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
        removePieceFromSelection({ id_: piece.id_ });
      } else if (e?.shiftKey) {
        addPieceToSelection({ id_: piece.id_ });
      } else {
        selectPiece({ id_: piece.id_ });
      }
    },
    [selectPiece, removePieceFromSelection, addPieceToSelection],
  );
  return (
    <mesh onClick={onSelect}>
      <boxGeometry args={[1, 1, 1]} />
      <meshBasicMaterial color={isSelected ? "purple" : isHovered ? "orange" : "red"} />
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
      if (newSelectedPieceIds.length !== selection.pieces?.length || newSelectedPieceIds.some((id, index) => id !== selection.pieces?.[index]?.id_)) {
        selectPieces(newSelectedPieceIds.map((id) => ({ id_: id })));
      }
    },
    [selectPieces],
  );

  return (
    <Select box multiple onChange={onChange}>
      <group>
        {/* <group quaternion={new THREE.Quaternion(-0.7071067811865476, 0, 0, 0.7071067811865476)}> */}
        {flatDesign.pieces?.map((piece: Piece, index: number) => (
          <PieceScopeProvider key={`piece-${piece.id_}`} id={{ id_: piece.id_ }}>
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

const Gizmo: FC = () => {
  const colors = useMemo(() => [getComputedColor("--color-primary"), getComputedColor("--color-tertiary"), getComputedColor("--color-secondary")] as [string, string, string], []);
  const labels = useMemo(() => ["X", "Z", "-Y"] as [string, string, string], []);
  const margin = useMemo(() => [80, 80] as [number, number], []);

  return (
    <GizmoHelper alignment="bottom-right" margin={margin}>
      <GizmoViewport labels={labels} axisColors={colors} />
    </GizmoHelper>
  );
};

const ModelInner: FC = () => {
  const fullscreen = useDesignEditorFullscreen() === DesignEditorFullscreenPanel.Model;
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
      {/* <Stage center={{ disable: true }} environment={null}> */}
      <ModelDesign />
      {/* </Stage> */}
      <Grid infiniteGrid={true} sectionColor={gridColors.sectionColor} cellColor={gridColors.cellColor} />
      {fullscreen && <Gizmo />}
    </>
  );
};

const Model: FC = () => {
  const { deselectAll, toggleModelFullscreen } = useDesignEditorCommands();
  const onDoubleClickCapture = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation();
      toggleModelFullscreen();
    },
    [toggleModelFullscreen],
  );
  const onPointerMissed = useCallback(
    (e: MouseEvent) => {
      if (!(e.ctrlKey || e.metaKey) && !e.shiftKey) deselectAll();
    },
    [deselectAll],
  );

  return (
    <div id="model" className="h-full w-full">
      <Canvas onDoubleClickCapture={onDoubleClickCapture} onPointerMissed={onPointerMissed}>
        <ModelInner />
      </Canvas>
    </div>
  );
};

export default Model;

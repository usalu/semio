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

import { GizmoHelper, GizmoViewport, Grid, Line, OrbitControls, OrthographicCamera, Select, TransformControls, useGLTF } from "@react-three/drei";
import { Canvas, ThreeEvent } from "@react-three/fiber";
import { applyDesignDiff, DiffStatus, flattenDesign, getPieceRepresentationUrls, Piece, Plane, planeToMatrix, toSemioRotation, updateDesignInKit } from "@semio/js";
import React, { FC, useCallback, useEffect, useMemo, useRef, useState } from "react";
import * as THREE from "three";
import {
  DesignEditorFullscreenPanel,
  DesignEditorPresenceOther,
  PieceScopeProvider,
  useDesign,
  useDesignEditorDesignDiff,
  useDesignEditorFileUrls,
  useDesignEditorFullscreenPanel,
  useDesignEditorPresenceOthers,
  useDesignEditorSelection,
  useKit,
} from "../../store";
import { useDesignEditorCommands } from "./DesignEditor";

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

interface ModelPieceProps {
  piece: Piece;
  plane: Plane;
  fileUrl: string;
  selected?: boolean;
  updating?: boolean;
  diffStatus?: DiffStatus;
  onSelect: (piece: Piece, e?: MouseEvent) => void;
  onPieceUpdate: (piece: Piece) => void;
}

const ModelPiece: FC<ModelPieceProps> = ({ piece, plane, fileUrl, selected, updating, diffStatus = DiffStatus.Unchanged, onSelect, onPieceUpdate }) => {
  const { startTransaction, finalizeTransaction, abortTransaction } = useDesignEditorCommands();
  const fixed = piece.plane !== undefined;
  const matrix = useMemo(() => {
    const planeRotationMatrix = planeToMatrix(plane);
    planeRotationMatrix.multiply(toSemioRotation());
    return planeRotationMatrix;
  }, [plane]);
  const styledScene = useMemo(() => {
    const scene = useGLTF(fileUrl).scene.clone();
    let meshColor: THREE.Color;
    let meshOpacity = 1;
    let lineOpacity = 1;

    if (diffStatus === DiffStatus.Added) {
      meshColor = new THREE.Color(getComputedColor("--color-success"));
      if (selected) {
        const selectedColor = new THREE.Color(getComputedColor("--color-primary"));
        meshColor.lerp(selectedColor, 0.5);
      }
    } else if (diffStatus === DiffStatus.Modified) {
      meshColor = new THREE.Color(getComputedColor("--color-warning"));
      if (selected) {
        const selectedColor = new THREE.Color(getComputedColor("--color-primary"));
        meshColor.lerp(selectedColor, 0.5);
      }
    } else if (diffStatus === DiffStatus.Removed) {
      meshColor = new THREE.Color(getComputedColor("--color-error"));
      meshOpacity = 0.2;
      lineOpacity = 0.25;
      if (selected) {
        const selectedColor = new THREE.Color(getComputedColor("--color-primary"));
        meshColor.lerp(selectedColor, 0.5);
      }
    } else if (selected) {
      meshColor = new THREE.Color(getComputedColor("--color-primary"));
    } else {
      meshColor = new THREE.Color(getComputedColor("--color-light"));
    }

    const lineColor = new THREE.Color(getComputedColor("--color-dark"));
    scene.traverse((object) => {
      if (object instanceof THREE.Mesh) {
        object.material = new THREE.MeshBasicMaterial({
          color: meshColor,
          transparent: meshOpacity < 1,
          opacity: meshOpacity,
        });
      }
      if (object instanceof THREE.Line) {
        object.material = new THREE.LineBasicMaterial({
          color: lineColor,
          transparent: lineOpacity < 1,
          opacity: lineOpacity,
        });
      }
    });
    return scene;
  }, [fileUrl, diffStatus, selected]);

  const onClick = useCallback(
    (e: ThreeEvent<MouseEvent>) => {
      onSelect(piece, e.nativeEvent);
      e.stopPropagation();
    },
    [onSelect, piece],
  );

  const transformControlRef = useRef(null);

  const handleMouseDown = useCallback(
    (e?: THREE.Event) => {
      console.log("handleMouseDown", e);
      startTransaction();
    },
    [startTransaction],
  );

  const handleMouseUp = useCallback(
    (e?: THREE.Event) => {
      console.log("handleMouseUp", e);
      finalizeTransaction();
    },
    [finalizeTransaction],
  );

  // Handle escape key to abort transactions during transform
  useEffect(() => {
    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === "Escape" && selected && fixed) {
        abortTransaction();
      }
    };

    document.addEventListener("keydown", handleEscape);
    return () => document.removeEventListener("keydown", handleEscape);
  }, [selected, fixed, abortTransaction]);

  const transformControl = selected && fixed;
  if (transformControl) {
    console.log("transformControl", transformControl);
  }
  const group = (
    <group
      matrix={matrix}
      matrixAutoUpdate={false}
      // position={[plane!.origin.x, plane!.origin.z, -plane!.origin.y]}
      userData={{ pieceId: piece.id_ }}
      onClick={onClick}
    >
      <primitive object={styledScene} />
    </group>
  );

  return (
    <PieceScopeProvider id={{ id_: piece.id_ }}>
      {transformControl ? (
        <TransformControls ref={transformControlRef} enabled={selected && fixed} onMouseDown={handleMouseDown} onMouseUp={handleMouseUp}>
          {group}
        </TransformControls>
      ) : (
        group
      )}
    </PieceScopeProvider>
  );
};

const ModelDesign: FC = () => {
  const designId = useDesign();
  const { removePieceFromSelection, selectPiece, addPieceToSelection, selectPieces, startTransaction, finalizeTransaction, abortTransaction, setPiece } = useDesignEditorCommands();
  const selection = useDesignEditorSelection();
  const designDiff = useDesignEditorDesignDiff();
  const fileUrls = useDesignEditorFileUrls();
  const others = useDesignEditorPresenceOthers();
  const storeKit = useKit();
  const baseDesign = useDesign();
  if (!storeKit || !baseDesign) return null;
  const design = applyDesignDiff(baseDesign, designDiff, true);
  const kit = useMemo(() => updateDesignInKit(storeKit, design), [storeKit, design]);
  const types = kit?.types ?? [];

  const flatDesign = useMemo(() => flattenDesign(kit, designId), [kit, designId]);
  const piecePlanes = useMemo(() => flatDesign.pieces?.map((p: Piece) => p.plane!) || [], [flatDesign]);

  const pieceRepresentationUrls = useMemo(() => getPieceRepresentationUrls(flatDesign, types), [flatDesign, types]);

  useEffect(() => {
    fileUrls.forEach((url, id) => {
      useGLTF.preload(id);
    });
  }, [fileUrls]);

  flatDesign.pieces?.forEach((p: Piece) => {
    const type = types.find((t) => t.name === p.type.name && (t.variant || "") === (p.type.variant || ""));
    if (!type) throw new Error(`Type (${p.type.name}, ${p.type.variant}) for piece ${p.id_} not found`);
  });

  useEffect(() => {
    pieceRepresentationUrls.forEach((url, id) => {
      if (!fileUrls.has(url)) throw new Error(`Representation url ${url} for piece ${id} not found in fileUrls map`);
    });
  }, [pieceRepresentationUrls, fileUrls]);

  const pieceDiffStatuses = useMemo(() => {
    return (
      flatDesign.pieces?.map((piece: Piece) => {
        const diffQuality = piece.qualities?.find((q: any) => q.name === "semio.diffStatus");
        return (diffQuality?.value as DiffStatus) || DiffStatus.Unchanged;
      }) || []
    );
  }, [flatDesign]);

  const onChange = useCallback(
    (selected: THREE.Object3D[]) => {
      // const newSelectedPieceIds = selected.map((item) => item.parent?.userData.pieceId).filter(Boolean);
      // if (newSelectedPieceIds.length !== selection.selectedPieceIds.length || newSelectedPieceIds.some((id, index) => id !== selection.selectedPieceIds[index])) {
      //   selectPieces(newSelectedPieceIds.map((id) => ({ id_: id })));
      // }
    },
    [selection, selectPieces],
  );

  const onSelect = useCallback(
    (piece: Piece, e?: MouseEvent) => {
      if (e?.ctrlKey || e?.metaKey) {
        removePieceFromSelection(piece);
      } else if (e?.shiftKey) {
        addPieceToSelection(piece);
      } else {
        selectPiece(piece);
      }
    },
    [removePieceFromSelection, addPieceToSelection, selectPiece],
  );

  const onPieceUpdate = useCallback((piece: Piece) => setPiece(piece), [setPiece]);
  return (
    <Select box multiple onChange={onChange} filter={(items) => items}>
      <group quaternion={new THREE.Quaternion(-0.7071067811865476, 0, 0, 0.7071067811865476)}>
        {flatDesign.pieces?.map((piece: Piece, index: number) => (
          <ModelPiece
            key={`piece-${piece.id_}`}
            piece={piece}
            plane={piecePlanes[index!]}
            fileUrl={fileUrls.get(pieceRepresentationUrls.get(piece.id_)!)!}
            selected={selection.selectedPieceIds.some((id) => id.id_ === piece.id_)}
            diffStatus={pieceDiffStatuses[index]}
            onSelect={onSelect}
            onPieceUpdate={onPieceUpdate}
          />
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

  return (
    <GizmoHelper alignment="bottom-right" margin={[80, 80]}>
      <GizmoViewport labels={["X", "Z", "-Y"]} axisColors={colors} />
    </GizmoHelper>
  );
};

const ModelCore: FC = () => {
  const fullscreen = useDesignEditorFullscreenPanel() === DesignEditorFullscreenPanel.Model;
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
          LEFT: THREE.MOUSE.ROTATE, // Left mouse button for orbit/pan
          MIDDLE: undefined,
          RIGHT: undefined, // Right button disabled to allow selection
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
        <ModelCore />
      </Canvas>
    </div>
  );
};

export default Model;

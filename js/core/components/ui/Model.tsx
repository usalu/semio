import React, { FC, JSX, Suspense, useMemo, useEffect, useState, useRef, useCallback } from 'react';
import { Canvas, ThreeEvent, useLoader } from '@react-three/fiber';
import { Center, Environment, GizmoHelper, GizmoViewport, Grid, Line, OrbitControls, Select, Sphere, Stage, TransformControls, useGLTF } from '@react-three/drei';
import * as THREE from 'three';
import { Kit, Design, DesignId, Piece, Plane, Type, flattenDesign, DesignEditorSelection, getPieceRepresentationUrls, planeToMatrix, ToThreeQuaternion, ToThreeRotation, ToSemioRotation } from '@semio/js';

const getComputedColor = (variable: string): string => {
    return getComputedStyle(document.documentElement).getPropertyValue(variable).trim();
};

interface PlaneThreeProps {
    plane: Plane;
}

const PlaneThree: FC<PlaneThreeProps> = ({ plane }) => {
    const matrix = useMemo(() => {
        return planeToMatrix(plane)
    }, [plane]);
    return (
        <group matrix={matrix} matrixAutoUpdate={false}>
            <Line points={[new THREE.Vector3(0, 0, 0), new THREE.Vector3(1, 0, 0)]} color={new THREE.Color(getComputedColor('--color-primary'))} />
            <Line points={[new THREE.Vector3(0, 0, 0), new THREE.Vector3(0, 1, 0)]} color={new THREE.Color(getComputedColor('--color-primary'))} />
        </group>
    )
}

interface ModelPieceProps {
    piece: Piece;
    plane: Plane;
    fileUrl: string;
    selected?: boolean;
    updating?: boolean;
    onSelect: (piece: Piece) => void
}

const ModelPiece: FC<ModelPieceProps> = ({ piece, plane, fileUrl, selected, updating, onSelect }) => {
    const fixed = piece.plane !== undefined;
    const matrix = useMemo(() => {
        // const threeToSemioRotation = new THREE.Matrix4(1, 0, 0, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 0, 0, 1);
        // const semioToThreeRotation = new THREE.Matrix4(1, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, 0, 0, 1);
        // // const origin = new THREE.Vector3(plane.origin.x, plane.origin.y, plane.origin.z);
        // const xAxis = new THREE.Vector3(plane.xAxis.x, plane.xAxis.y, plane.xAxis.z);
        // const yAxis = new THREE.Vector3(plane.yAxis.x, plane.yAxis.y, plane.yAxis.z);
        // const zAxis = new THREE.Vector3().crossVectors(xAxis, yAxis);
        // const planeRotationMatrix = new THREE.Matrix4();
        // planeRotationMatrix.makeBasis(xAxis.normalize(), yAxis.normalize(), zAxis.normalize());
        // // planeRotationMatrix.setPosition(origin);
        // const m = new THREE.Matrix4();
        // m.multiply(threeToSemioRotation);
        // m.multiply(planeRotationMatrix);
        // m.multiply(semioToThreeRotation);
        // m.multiply(new THREE.Matrix4().makeTranslation(plane.origin.x, -plane.origin.z, plane.origin.y));
        // return m
        const planeRotationMatrix = planeToMatrix(plane)
        planeRotationMatrix.multiply(ToSemioRotation())
        return planeRotationMatrix
    }, [plane]);
    const scene = useMemo(() => {
        return useGLTF(fileUrl).scene.clone()
    }, [fileUrl])
    const selectedScene = useMemo(() => {
        const sceneClone = scene.clone()
        sceneClone.traverse((object) => {
            if (object instanceof THREE.Mesh) {
                const meshColor = new THREE.Color(getComputedColor('--color-primary'))
                object.material = new THREE.MeshBasicMaterial({ color: meshColor })
            }
            if (object instanceof THREE.Line) {
                const lineColor = new THREE.Color(getComputedColor('--color-foreground'))
                object.material = new THREE.LineBasicMaterial({ color: lineColor })
            }
        })
        return sceneClone
    }, [scene])
    const updatingScene = useMemo(() => {
        const sceneClone = scene.clone()
        sceneClone.traverse((object) => {
            if (object instanceof THREE.Mesh) {
                const meshColor = new THREE.Color(getComputedColor('--color-foreground'))
                object.material = new THREE.MeshBasicMaterial({ color: meshColor, transparent: true, opacity: 0.1 })
            }
            if (object instanceof THREE.Line) {
                const lineColor = new THREE.Color(getComputedColor('--color-background'))
                object.material = new THREE.LineBasicMaterial({ color: lineColor, transparent: true, opacity: 0.15 })
            }
        })
        return sceneClone
    }, [scene])

    const handleClick = useCallback((e: ThreeEvent<MouseEvent>) => {
        onSelect(piece);
        e.stopPropagation();
    }, [onSelect, piece]);

    const transformControlRef = useRef(null)
    const handleMouseUp = useCallback((e: ThreeEvent<MouseEvent>) => {
        console.log("handleMouseUp", e);
    }, [plane]);

    const transformControl = selected && fixed
    if (transformControl) {
        console.log("transformControl", transformControl)
    }
    const group = (
        <group
            matrix={matrix}
            matrixAutoUpdate={false}
            // position={[plane!.origin.x, plane!.origin.z, -plane!.origin.y]}
            userData={{ pieceId: piece.id_ }}
            onClick={handleClick}>
            <primitive object={selected ? selectedScene : updating ? updatingScene : scene} />
        </group>
    )

    return (
        transformControl ? (
            <TransformControls
                ref={transformControlRef}
                enabled={selected && fixed}
                onMouseUp={handleMouseUp}
            >
                {group}
            </TransformControls>
        ) : group

    );
};

interface ModelDesignProps {
    kit: Kit;
    designId: DesignId;
    fileUrls: Map<string, string>;
    selection: DesignEditorSelection;
    onSelectionChange: (selection: DesignEditorSelection) => void;
    onDesignChange: (design: Design) => void;
}

const ModelDesign: FC<ModelDesignProps> = ({ kit, designId, fileUrls, selection, onSelectionChange, onDesignChange }) => {
    const normalize = (val: string | undefined) => val === undefined ? "" : val;
    const design = kit.designs?.find(d =>
        d.name === designId.name &&
        (normalize(d.variant) === normalize(designId.variant)) &&
        (normalize(d.view) === normalize(designId.view))
    );
    if (!design) {
        return null;
    }
    const types = kit?.types ?? [];
    const piecePlanes = useMemo(() => {
        const flatDesign = flattenDesign(design, types);
        return flatDesign.pieces?.map(p => p.plane!) || [];
    }, [design, types]);

    const pieceRepresentationUrls = useMemo(() => {
        return getPieceRepresentationUrls(design, types);
    }, [design, types]);

    useEffect(() => {
        fileUrls.forEach((url, id) => {
            useGLTF.preload(id);
        });
    }, [fileUrls]);

    design.pieces?.forEach(p => {
        const type = types.find(t => t.name === p.type.name && t.variant === p.type.variant);
        if (!type) throw new Error(`Type (${p.type.name}, ${p.type.variant}) for piece ${p.id_} not found`);
    });

    useEffect(() => {
        pieceRepresentationUrls.forEach((url, id) => {
            if (!fileUrls.has(url)) throw new Error(`Representation url ${url} for piece ${id} not found in fileUrls map`);
        });
    }, [pieceRepresentationUrls, fileUrls]);


    const handleSelectionChange = useCallback((selected: THREE.Object3D[]) => {
        const newSelectedPieceIds = selected.map(item => item.parent?.userData.pieceId);

        if (!Array.isArray(selection.selectedPieceIds) ||
            newSelectedPieceIds.length !== selection.selectedPieceIds.length ||
            newSelectedPieceIds.some((id, index) => id !== selection.selectedPieceIds[index])) {

            onSelectionChange({
                ...selection,
                selectedPieceIds: newSelectedPieceIds
            });
        }
    }, [selection, onSelectionChange]);

    const handlePieceSelect = useCallback((piece: Piece) => {
        if (selection.selectedPieceIds.includes(piece.id_)) {
            onSelectionChange({
                ...selection,
                selectedPieceIds: selection.selectedPieceIds.filter(id => id !== piece.id_)
            })
        } else {
            onSelectionChange({
                ...selection,
                selectedPieceIds: [...selection.selectedPieceIds, piece.id_]
            })
        }
    }, [selection, onSelectionChange]);


    return (
        <Select box multiple onChange={handleSelectionChange} filter={items => items}>
            <group
                quaternion={new THREE.Quaternion(-0.7071067811865476, 0, 0, 0.7071067811865476)}
            >
                {design.pieces?.map((piece, index) => (
                    <ModelPiece
                        key={`piece-${piece.id_}`}
                        piece={piece}
                        plane={piecePlanes![index!]}
                        fileUrl={fileUrls.get(pieceRepresentationUrls.get(piece.id_)!)!}
                        selected={selection.selectedPieceIds.includes(piece.id_)}
                        onSelect={handlePieceSelect}
                        updating
                    />
                    // <PlaneThree key={`plane-${piece.id_}`} plane={piecePlanes![index]} />
                ))}
            </group>

        </Select>
    );
};

const Gizmo: FC = (): JSX.Element => {
    const colors = useMemo(() => [
        getComputedColor('--color-primary'),
        getComputedColor('--color-tertiary'),
        getComputedColor('--color-secondary')
    ] as [string, string, string], []);

    return (
        <GizmoHelper alignment="bottom-right" margin={[80, 80]}>
            <GizmoViewport
                labels={['X', 'Z', '-Y']}
                axisColors={colors}

            // font='Anta'
            />
        </GizmoHelper>
    )
}

interface ModelProps {
    kit: Kit;
    designId: DesignId;
    fileUrls: Map<string, string>;
    fullscreen: boolean;
    onPanelDoubleClick?: () => void;
    selection: DesignEditorSelection;
    onSelectionChange: (selection: DesignEditorSelection) => void;
    onDesignChange: (design: Design) => void;
    onPieceUpdate: (piece: Piece) => void;
}
const Model: FC<ModelProps> = ({ kit, designId, fileUrls, fullscreen, onPanelDoubleClick, selection, onSelectionChange, onDesignChange }) => {
    const [gridColors, setGridColors] = useState({
        sectionColor: getComputedColor('--foreground'),
        cellColor: getComputedColor('--accent-foreground')
    });
    useEffect(() => {
        const updateColors = () => {
            setGridColors({
                sectionColor: getComputedColor('--foreground'),
                cellColor: getComputedColor('--accent-foreground')
            });
        };
        updateColors();
        const observer = new MutationObserver(updateColors);
        observer.observe(document.documentElement, {
            attributes: true,
            attributeFilter: ['class']
        });

        return () => observer.disconnect();
    }, []);

    const handleDoubleClick = useCallback((e: React.MouseEvent) => {
        e.stopPropagation();
        if (onPanelDoubleClick) onPanelDoubleClick();
    }, [onPanelDoubleClick]);

    const handlePointerMissed = useCallback(() => {
        onSelectionChange({
            selectedPieceIds: [],
            selectedConnections: []
        })
    }, [onSelectionChange]);

    return (
        <div className="w-full h-full">
            <Canvas
                onDoubleClickCapture={handleDoubleClick}
                onPointerMissed={handlePointerMissed}>
                <OrbitControls
                    makeDefault
                    mouseButtons={{
                        LEFT: THREE.MOUSE.ROTATE, // Left mouse button for orbit/pan
                        MIDDLE: undefined,
                        RIGHT: undefined // Right button disabled to allow selection
                    }}
                />
                <ambientLight intensity={1} />
                {/* <Suspense fallback={null}>
                        <Gltf src={src} />
                    </Suspense> */}
                <ModelDesign kit={kit} designId={designId} fileUrls={fileUrls} selection={selection} onSelectionChange={onSelectionChange} onDesignChange={onDesignChange} />
                <Environment files={'schlenker-shed.hdr'} />
                <Grid
                    infiniteGrid={true}
                    sectionColor={gridColors.sectionColor}
                    cellColor={gridColors.cellColor}
                />
                {fullscreen && <Gizmo />}
            </Canvas>
        </div>
    );
};

export default Model;
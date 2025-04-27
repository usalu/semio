import React, { FC, JSX, Suspense, useMemo, useEffect, useState, useRef } from 'react';
import { Canvas, ThreeEvent } from '@react-three/fiber';
import { Center, Environment, GizmoHelper, GizmoViewport, Grid, OrbitControls, Select, Sphere, Stage, useGLTF } from '@react-three/drei';
import * as THREE from 'three';
import { Design, Piece, Plane, Type, flattenDesign, DesignEditorSelection, selectRepresentation, pieceRepresentationUrls, getPieceRepresentationUrls } from '@semio/js';

const getComputedColor = (variable: string): string => {
    return getComputedStyle(document.documentElement).getPropertyValue(variable).trim();
};

interface ModelPieceProps {
    piece: Piece;
    plane: Plane;
    fileUrl: string;
    selected?: boolean;
}

const ModelPiece: FC<ModelPieceProps> = ({ piece, plane, fileUrl, selected }) => {
    const position = useMemo(() => new THREE.Vector3(plane.origin.x, plane.origin.z, -plane.origin.y), [plane]);
    return (
        <group position={position} userData={{ pieceId: piece.id_ }}>
            {/* Example geometry, replace with actual representation */}
            <Sphere args={[0.5, 32, 32]} >
                <meshStandardMaterial color={selected ? 'pink' : 'gold'} roughness={0} metalness={1} />
            </Sphere>
        </group>
    );
};

interface ModelDesignProps {
    design: Design;
    types: Type[];
    fileUrls: Map<string, string>;
    selection: DesignEditorSelection;
    onSelectionChange: (selection: DesignEditorSelection) => void;
}

const ModelDesign: FC<ModelDesignProps> = ({ design, types, fileUrls, selection, onSelectionChange }) => {
    const [gridColors, setGridColors] = useState(() => ({
        sectionColor: getComputedColor('--foreground'),
        cellColor: getComputedColor('--accent-foreground')
    }));

    useEffect(() => {
        fileUrls.forEach((url, id) => {
            useGLTF.preload(id);
        });
    }, [fileUrls]);

    design.pieces?.forEach(p => {
        const type = types.find(t => t.name === p.type.name && t.variant === p.type.variant);
        if (!type) throw new Error(`Type (${p.type.name}, ${p.type.variant}) for piece ${p.id_} not found`);
    });

    const flatDesign = design ? flattenDesign(design, types) : null;
    const piecePlanes = flatDesign?.pieces?.map(p => p.plane);
    const pieceRepresentationUrls = getPieceRepresentationUrls(design, types!);

    pieceRepresentationUrls.forEach((url, id) => {
        if (!fileUrls.has(url)) throw new Error(`Representation url ${url} for piece ${id} not found in fileUrls map`);
    });

    return (
        <Select box multiple onChange={(selected) => {
            const newSelectedPieceIds = selected.map(item => item.parent?.userData.pieceId);

            if (!Array.isArray(selection.selectedPieceIds) ||
                newSelectedPieceIds.length !== selection.selectedPieceIds.length ||
                newSelectedPieceIds.some((id, index) => id !== selection.selectedPieceIds[index])) {

                onSelectionChange({
                    ...selection,
                    selectedPieceIds: newSelectedPieceIds
                });
            }
        }} filter={items => items}>
            {design.pieces?.map((piece, index) => (
                <ModelPiece
                    key={`piece-${piece.id_ || index}`}
                    piece={piece}
                    plane={piecePlanes[index]}
                    fileUrl={fileUrls.get(pieceRepresentationUrls.get(piece.id_))}
                    selected={selection.selectedPieceIds.includes(piece.id_)}
                />
            ))}
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
    design: Design;
    types: Type[];
    fileUrls: Map<string, string>;
    fullscreen: boolean;
    onPanelDoubleClick?: () => void;
    selection: DesignEditorSelection;
    onSelectionChange: (selection: DesignEditorSelection) => void;
}
const Model: FC<ModelProps> = ({ fullscreen, onPanelDoubleClick, design, types, fileUrls, selection, onSelectionChange }) => {
    const [gridColors, setGridColors] = useState({
        sectionColor: getComputedColor('--foreground'),
        cellColor: getComputedColor('--accent-foreground')
    });

    // Update colors when theme changes
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

    return (
        <div className="w-full h-full">
            <Canvas onDoubleClickCapture={(e) => {
                e.stopPropagation();
                if (onPanelDoubleClick) onPanelDoubleClick();
            }}>
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
                <ModelDesign design={design} types={types} fileUrls={fileUrls} selection={selection} onSelectionChange={onSelectionChange} />
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
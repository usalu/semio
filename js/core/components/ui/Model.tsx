import React, { FC, JSX, Suspense, useMemo, useEffect, useState, useRef } from 'react';
import { Canvas } from '@react-three/fiber';
import { Center, Environment, GizmoHelper, GizmoViewport, Grid, OrbitControls, Select, Sphere, Stage, useGLTF } from '@react-three/drei';
import * as THREE from 'three';
import { Design, Piece } from '@semio/js';

const getComputedColor = (variable: string): string => {
    return getComputedStyle(document.documentElement).getPropertyValue(variable).trim();
};

interface ModelPieceProps {
    piece: Piece;
}
const ModelPiece: FC<ModelPieceProps> = ({ piece }) => {
    const { selection, setSelection } = useStudio();
    return (
        <Select
            multiple
            box
            // TODO: If theme becomes customizable, same approach as in Gizmo is needed 🔄️
            border="1px solid var(--color-primary)"
            backgroundColor="color-mix(in srgb, var(--color-primary) 10%, transparent)"
            onClick={(e) => {
                console.log('select clicked', e)
                setSelection({
                    ...selection,
                    // TODO: Update selection to set
                    pieceIds: selection.pieceIds.includes(piece.id_) ? selection.pieceIds.filter((id) => id !== piece.id_) : [...selection.pieceIds, piece.id_]
                });
            }}
        >
            <Sphere args={[1, 100, 100]} position={[piece.plane.origin.x, piece.plane.origin.z, -piece.plane.origin.y]}>
                <meshStandardMaterial color="gold" roughness={0} metalness={1} />
            </Sphere>
        </Select>
    )
}

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
    fullscreen: boolean;
    onPanelDoubleClick?: () => void;
    design: Design;
}
const Model: FC<ModelProps> = ({ fullscreen, onPanelDoubleClick, design }) => {
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

        // Update immediately and set up observer
        updateColors();

        // Watch for class changes on document.documentElement (light/dark theme toggle)
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
                {design.pieces.map((piece) => (
                    <ModelPiece key={piece.id_} piece={piece} />
                ))}
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
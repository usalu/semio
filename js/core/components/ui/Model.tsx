import React, { FC, JSX, Suspense, useMemo, useEffect, useState, useRef } from 'react';
import { Canvas } from '@react-three/fiber';
import { Center, Environment, GizmoHelper, GizmoViewport, Grid, OrbitControls, Sphere, Stage, useGLTF } from '@react-three/drei';
import * as THREE from 'three';

const getComputedColor = (variable: string): string => {
    return getComputedStyle(document.documentElement).getPropertyValue(variable).trim();
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
    fullscreen: boolean;
}
const Model: FC<ModelProps> = ({ fullscreen }) => {
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
            <Canvas>
                <OrbitControls
                    mouseButtons={{
                        LEFT: null, // Disable left click for orbit to allow selection
                        MIDDLE: THREE.MOUSE.DOLLY, // Middle button for zoom (wheel still works too)
                        RIGHT: THREE.MOUSE.ROTATE // Right button for orbit
                    }}
                    enablePan={true}
                    enableRotate={true}
                    enableZoom={true}
                    keyPanSpeed={20}
                    modifierKey="ctrlKey"
                />
                <ambientLight intensity={1} />
                {/* <Suspense fallback={null}>
                        <Gltf src={src} />
                    </Suspense> */}
                <Sphere args={[2, 100, 100]} position={[0, 0, 0]}>
                    <meshStandardMaterial color="gold" roughness={0} metalness={1} />
                </Sphere>
                <Sphere args={[1, 100, 100]} position={[0, 3, 0]}>
                    <meshStandardMaterial color="gold" roughness={0} metalness={1} opacity={0.5} transparent={true} />
                </Sphere>
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
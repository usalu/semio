import React, { FC, JSX, Suspense, useMemo } from 'react';
import { Canvas } from '@react-three/fiber';
import { Center, Environment, GizmoHelper, GizmoViewport, Grid, OrbitControls, Sphere, Stage, useGLTF } from '@react-three/drei';

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
    return (
        <div className="w-full h-full">
            <Canvas>
                <OrbitControls />
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
                <Grid infiniteGrid={true} sectionColor='var(--foreground)' cellColor='var(--accent-foreground)' />
                {fullscreen && <Gizmo />}
            </Canvas>
        </div>
    );
};

export default Model;
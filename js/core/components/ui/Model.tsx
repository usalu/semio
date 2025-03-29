import React, { FC, Suspense } from 'react';
import { GLTFLoader } from 'three/addons';
import { Canvas, useLoader } from '@react-three/fiber';
import { Box, Environment, Grid, OrbitControls } from '@react-three/drei';

interface ModelProps {
    src: string;
}

const Model: FC<ModelProps> = ({ src }) => {
    const gltf = useLoader(GLTFLoader, src);
    return (
        <Canvas>
            <Suspense fallback={null}>
                <OrbitControls />
                <ambientLight intensity={0.5} />
                <Grid />
                <primitive object={gltf.scene} />
                <Environment preset="sunset" background />
            </Suspense>
        </Canvas>
    );
};

export default Model;
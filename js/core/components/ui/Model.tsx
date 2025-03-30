import React, { FC, Suspense } from 'react';
import { Canvas } from '@react-three/fiber';
import { Environment, OrbitControls, Sphere, useGLTF } from '@react-three/drei';

interface GltfProps {
    src: string;
}
const Gltf: FC<GltfProps> = ({ src }) => {
    const { scene } = useGLTF(src);
    return <primitive object={scene} />;
}

interface ModelProps {
    src: string;
}
const Model: FC<ModelProps> = ({ src }) => {
    return (
        <Canvas>
            <Suspense fallback={null}>
                <OrbitControls />
                <ambientLight intensity={1} />
                <Gltf src={src} />
            </Suspense>
        </Canvas>
    );
};

export default Model;
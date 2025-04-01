import React, { FC, Suspense } from 'react';
import { Canvas } from '@react-three/fiber';
import { Center, Environment, OrbitControls, Sphere, Stage, useGLTF } from '@react-three/drei';

interface GltfProps {
    src: string;
}
const Gltf: FC<GltfProps> = ({ src }) => {
    const { scene } = useGLTF(src);
    return <primitive object={scene} />;
}

interface ModelProps {
    src: string;
    environment?: string;
}
const Model: FC<ModelProps> = ({ src, environment }) => {
    return (
        <div style={{ width: '100%', height: '100%' }}>
            <Canvas>
                <Stage environment={null} shadows={false}>
                    <OrbitControls enablePan={false} />
                    <ambientLight intensity={1} />
                    <Suspense fallback={null}>
                        <Gltf src={src} />
                    </Suspense>
                </Stage>
                <Environment files={environment || ''} />
            </Canvas>
        </div>
    );
};

export default Model;
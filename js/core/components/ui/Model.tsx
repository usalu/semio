import React, { FC, Suspense } from 'react';
import { Canvas } from '@react-three/fiber';
import { Center, OrbitControls, Sphere, Stage, useGLTF } from '@react-three/drei';

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
        <div style={{ width: '100%', height: '100%' }}>
            <Canvas>
                <Stage>
                    <Suspense fallback={null}>
                        <OrbitControls enablePan={false} />
                        <ambientLight intensity={1} />
                        <Gltf src={src} />
                    </Suspense>
                </Stage>
            </Canvas>
        </div>
    );
};

export default Model;
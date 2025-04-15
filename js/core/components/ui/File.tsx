import React, { FC, Suspense, useEffect } from 'react';
import { Canvas } from '@react-three/fiber';
import { Center, Environment, OrbitControls, Sphere, Stage, useGLTF } from '@react-three/drei';

interface GltfProps {
    src: string;
    roughness?: number;
    metalness?: number;
}
const Gltf: FC<GltfProps> = ({ src, roughness, metalness }) => {
    const { scene } = useGLTF(src);

    useEffect(() => {
        if (roughness !== undefined || metalness !== undefined) {
            scene.traverse((node) => {
                if (node.isMesh && node.material) {
                    // Apply to single material
                    if (node.material.roughness !== undefined && roughness !== undefined) {
                        node.material.roughness = roughness;
                    }
                    if (node.material.metalness !== undefined && metalness !== undefined) {
                        node.material.metalness = metalness;
                    }

                    // Handle array of materials if present
                    if (Array.isArray(node.material)) {
                        node.material.forEach(material => {
                            if (material.roughness !== undefined && roughness !== undefined) {
                                material.roughness = roughness;
                            }
                            if (material.metalness !== undefined && metalness !== undefined) {
                                material.metalness = metalness;
                            }
                        });
                    }

                    // Make sure changes are visible
                    if (node.material.needsUpdate !== undefined) {
                        node.material.needsUpdate = true;
                    }
                }
            });
        }
    }, [scene, roughness, metalness]);

    return <primitive object={scene} />;
}

interface FileProps {
    src: string;
    environment?: string;
    roughness?: number;
    metalness?: number;
}
const File: FC<FileProps> = ({ src, environment, roughness, metalness }) => {
    return (
        <div className="w-full h-full">
            <Canvas>
                <Stage environment={null} shadows={false}>
                    <OrbitControls enablePan={false} />
                    <ambientLight intensity={1} />
                    <Suspense fallback={null}>
                        <Gltf
                            src={src}
                            roughness={roughness}
                            metalness={metalness}
                        />
                    </Suspense>
                </Stage>
                <Environment files={environment || ''} />
            </Canvas>
        </div>
    );
};

export default File;
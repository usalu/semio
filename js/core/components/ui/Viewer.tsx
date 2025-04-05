import React, { FC, Suspense } from 'react';
import { Canvas } from '@react-three/fiber';
import { Center, Environment, OrbitControls, Sphere, Stage, useGLTF } from '@react-three/drei';



interface ViewerProps {
}
const Viewer: FC<ViewerProps> = ({ }) => {
    return (
        <div style={{ width: '100%', height: '100%' }}>
            <Canvas>
                <OrbitControls enablePan={false} />
                <ambientLight intensity={1} />
                {/* <Suspense fallback={null}>
                        <Gltf src={src} />
                    </Suspense> */}
                <Sphere args={[1, 100, 100]} scale={1.5} position={[0, 0, 0]}>
                    <meshStandardMaterial color="gold" roughness={0} metalness={1} />
                </Sphere>
                <Environment files={'schlenker-shed.hdr'} />
            </Canvas>
        </div>
    );
};

export default Viewer;
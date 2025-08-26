// #region Header

// File.tsx

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion
// #region Header

// File.tsx

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion
import { Environment, OrbitControls, Stage, useGLTF } from "@react-three/drei";
import { Canvas } from "@react-three/fiber";
import { FC, Suspense, useEffect } from "react";

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
        if ((node as any).isMesh && (node as any).material) {
          // Apply to single material
          if ((node as any).material.roughness !== undefined && roughness !== undefined) {
            (node as any).material.roughness = roughness;
          }
          if ((node as any).material.metalness !== undefined && metalness !== undefined) {
            (node as any).material.metalness = metalness;
          }

          // Handle array of materials if present
          if (Array.isArray((node as any).material)) {
            (node as any).material.forEach((material: any) => {
              if (material.roughness !== undefined && roughness !== undefined) {
                material.roughness = roughness;
              }
              if (material.metalness !== undefined && metalness !== undefined) {
                material.metalness = metalness;
              }
            });
          }

          // Make sure changes are visible
          if ((node as any).material.needsUpdate !== undefined) {
            (node as any).material.needsUpdate = true;
          }
        }
      });
    }
  }, [scene, roughness, metalness]);

  return <primitive object={scene} />;
};

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
            <Gltf src={src} roughness={roughness} metalness={metalness} />
          </Suspense>
        </Stage>
        <Environment files={environment || ""} />
      </Canvas>
    </div>
  );
};

export default File;

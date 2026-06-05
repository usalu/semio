import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { Box3, Group, Vector3 } from "three";
import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
import {
  brushPreviewMeshFrameGroup,
  styledMeshTemplate,
  updateWorldMatrixChain,
} from "/Users/ueli/Documents/semio/puzzle/3d/react/index.tsx";

const loader = new GLTFLoader();
const meshDir = "/Users/ueli/Documents/semio/semio/fixtures/kit/folder/abbau-aufbau";
const bytes = readFileSync(resolve(meshDir, "hexagonal-cut-concrete-forest-left.glb"));
const buf = bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength);
const gltf = await new Promise<any>((res, rej) => loader.parse(buf, "", res, rej));

function boundsForTemplate(edgeOutlines: boolean) {
  const template = styledMeshTemplate("/meshes/left.glb", "highlighted", gltf.scene, edgeOutlines);
  const g = new Group();
  g.add(brushPreviewMeshFrameGroup(template));
  updateWorldMatrixChain(g);
  const box = new Box3().setFromObject(g, true);
  return box.getSize(new Vector3()).toArray();
}

console.log("[DEBUG] raw gltf.scene", new Box3().setFromObject(gltf.scene, true).getSize(new Vector3()).toArray());
console.log("[DEBUG] probe false outlines", boundsForTemplate(false));
console.log("[DEBUG] probe true outlines", boundsForTemplate(true));

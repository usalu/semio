import * as THREE from "../../../../../../spatial/js/node_modules/three/build/three.module.js";
import { COMMITTED_MESH_FACE_OPACITY } from "../../../../../../spatial/js/renderer-r3f/index.tsx";

const material = new THREE.MeshStandardMaterial({
	transparent: true,
	opacity: COMMITTED_MESH_FACE_OPACITY,
	depthWrite: false,
});

console.log("[DEBUG] material opacity", material.opacity);
console.log("[DEBUG] material transparent", material.transparent);
console.log("[DEBUG] material depthWrite", material.depthWrite);
process.exit(material.opacity < 1 && material.transparent && !material.depthWrite ? 0 : 1);

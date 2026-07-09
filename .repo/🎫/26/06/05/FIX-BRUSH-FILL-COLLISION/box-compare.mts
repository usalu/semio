import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { Box3, Group, Vector3 } from "three";
import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
import { brushPreviewCollisionBox, brushProbeGroupFromPreview, parseFixtureV1, updateWorldMatrixChain } from "/Users/ueli/Documents/compose/puzzle/3d/react/index.tsx";

const loader = new GLTFLoader();
const meshDir = "/Users/ueli/Documents/compose/compose/fixtures/kit/folder/abbau-aufbau";
const bytes = readFileSync(resolve(meshDir, "hexagonal-cut-concrete-forest-left.glb"));
const buf = bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength);
const gltf = await new Promise<any>((res, rej) => loader.parse(buf, "", res, rej));
const meshRoot = gltf.scene as Group;

const f = parseFixtureV1(JSON.parse(readFileSync("/Users/ueli/Documents/compose/puzzle/3d/fixture/concrete-forest.3d.json", "utf8")));
const obj = f!.objects[0]!;

const probe = brushProbeGroupFromPreview({ origin: obj.origin as [number, number, number], orientation: obj.orientation as [number, number, number, number] }, meshRoot);
const without = brushPreviewCollisionBox(probe, 0);
updateWorldMatrixChain(probe);
const withChain = brushPreviewCollisionBox(probe, 0);
console.log("[DEBUG] without updateWorldMatrixChain min", without.min.toArray(), "max", without.max.toArray());
console.log("[DEBUG] with updateWorldMatrixChain min", withChain.min.toArray(), "max", withChain.max.toArray());
console.log("[DEBUG] equal", without.equals(withChain));

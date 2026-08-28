/** 🧫️ Emit carrier files so the probe suite can be exercised against real bytes. */
import { writeFileSync } from "node:fs";
import Module from "manifold-3d";
import * as THREE from "three";
import { STLExporter } from "three/examples/jsm/exporters/STLExporter.js";
import { OBJExporter } from "three/examples/jsm/exporters/OBJExporter.js";
import { PLYExporter } from "three/examples/jsm/exporters/PLYExporter.js";
import { GLTFExporter } from "three/examples/jsm/exporters/GLTFExporter.js";
(globalThis as { requestAnimationFrame?: (cb: () => void) => number }).requestAnimationFrame ??= (cb) => { cb(); return 0; };
/** 🩹️ three's GLTFExporter reads its buffers through a browser FileReader; supply the two methods it uses. */
class ShimFileReader {
  result: unknown = null;
  onload: (() => void) | null = null;
  onloadend: (() => void) | null = null;
  onerror: ((e: unknown) => void) | null = null;
  private done() { this.onloadend?.(); this.onload?.(); }
  readAsArrayBuffer(blob: Blob) { blob.arrayBuffer().then((b) => { this.result = b; this.done(); }, (e) => this.onerror?.(e)); }
  readAsDataURL(blob: Blob) { blob.arrayBuffer().then((b) => { this.result = `data:${blob.type || "application/octet-stream"};base64,${Buffer.from(b).toString("base64")}`; this.done(); }, (e) => this.onerror?.(e)); }
}
(globalThis as { FileReader?: unknown }).FileReader ??= ShimFileReader as unknown as typeof FileReader;
const wasm = await Module(); wasm.setup(); const { Manifold } = wasm;
const out = process.argv[2]!;
function toThree(m: { numProp: number; vertProperties: Float32Array; triVerts: Uint32Array }) {
  const g = new THREE.BufferGeometry();
  const pos = new Float32Array((m.vertProperties.length / m.numProp) * 3);
  for (let i = 0; i < pos.length / 3; i += 1) { pos[i*3]=m.vertProperties[i*m.numProp]!; pos[i*3+1]=m.vertProperties[i*m.numProp+1]!; pos[i*3+2]=m.vertProperties[i*m.numProp+2]!; }
  g.setAttribute("position", new THREE.BufferAttribute(pos, 3));
  g.setIndex(new THREE.BufferAttribute(new Uint32Array(m.triVerts), 1));
  g.computeVertexNormals(); return g;
}
async function emit(name: string, solid: ReturnType<typeof Manifold.cube>, roughness: number, metallic: number, color: number) {
  const geom = toThree(solid.getMesh());
  const material = new THREE.MeshStandardMaterial({ color, roughness, metalness: metallic });
  material.name = `mat-${name}`;
  const mesh = new THREE.Mesh(geom, material);
  writeFileSync(`${out}/${name}.stl`, new STLExporter().parse(mesh, { binary: false }) as unknown as string);
  writeFileSync(`${out}/${name}.obj`, new OBJExporter().parse(mesh));
  writeFileSync(`${out}/${name}.ply`, String(new PLYExporter().parse(mesh, () => {}, { binary: false })));
  const gltf = await new Promise<object>((res, rej) => new GLTFExporter().parse(mesh, res as (g: object) => void, rej, {}));
  writeFileSync(`${out}/${name}.gltf`, JSON.stringify(gltf));
  console.log(`[emit] ${name} tris=${solid.numTri()} vol=${solid.volume().toFixed(3)} genus=${solid.genus()}`);
}
const bored = Manifold.cube([20,20,20], true).subtract(Manifold.cylinder(30,5,5,64,true));
await emit("bored-cube", bored, 0.4, 0.1, 0xff0000);
// 🎯️Same SOLID, different tessellation — the case a correct gate must ACCEPT.
await emit("sphere-fine", Manifold.sphere(10, 128), 0.4, 0.1, 0xff0000);
await emit("sphere-coarse", Manifold.sphere(10, 16), 0.4, 0.1, 0xff0000);
// 🎯️A genuinely DIFFERENT solid — the case a correct gate must REJECT.
await emit("bored-cube-wrong", Manifold.cube([20,20,20], true).subtract(Manifold.cylinder(30,6,6,64,true)), 0.4, 0.1, 0xff0000);
// 🎯️Same geometry, different MATERIAL — invisible in STL, visible in glTF.
await emit("bored-cube-rough", bored, 0.9, 0.1, 0xff0000);

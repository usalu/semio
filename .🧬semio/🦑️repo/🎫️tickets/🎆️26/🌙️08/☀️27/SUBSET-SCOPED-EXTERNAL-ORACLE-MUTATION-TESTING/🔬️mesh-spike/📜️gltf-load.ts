import { readFileSync } from "node:fs";
import * as THREE from "three";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
class ShimProgressEvent extends Event {
  lengthComputable: boolean; loaded: number; total: number;
  constructor(type: string, init?: { lengthComputable?: boolean; loaded?: number; total?: number }) {
    super(type);
    this.lengthComputable = init?.lengthComputable ?? false;
    this.loaded = init?.loaded ?? 0;
    this.total = init?.total ?? 0;
  }
}
(globalThis as { ProgressEvent?: unknown }).ProgressEvent ??= ShimProgressEvent;
const p = process.argv[2]!;
const t = setTimeout(() => { console.log("[TIMEOUT] GLTFLoader.parse callback never fired"); process.exit(2); }, 8000);
try {
  const gltf = await new Promise<{ scene: THREE.Object3D }>((res, rej) => {
    new GLTFLoader().parse(new TextDecoder().decode(readFileSync(p)), "", (r) => res(r as { scene: THREE.Object3D }), rej);
  });
  clearTimeout(t);
  let meshes = 0, tris = 0, mats: string[] = [];
  gltf.scene.traverse((c) => { const m = c as THREE.Mesh; if (m.isMesh) { meshes++; const g = m.geometry as THREE.BufferGeometry; tris += (g.index ? g.index.count : g.getAttribute("position").count)/3; const mm = m.material as THREE.MeshStandardMaterial; mats.push(`${mm.name}:rough=${mm.roughness},metal=${mm.metalness}`); } });
  console.log(`[ok] meshes=${meshes} tris=${tris} materials=${mats.join(" | ")}`);
  process.exit(0);
} catch (e) { clearTimeout(t); console.log("[ERROR]", String((e as Error).message ?? e).slice(0,200)); process.exit(1); }

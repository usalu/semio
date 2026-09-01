// Empirical spike: what does three@0.182.0's GLTFLoader / GLTFExporter actually expose?
// Run: bun 🗑️temp/gltf-any-retrofit/probe-capabilities.ts
import * as THREE from "three";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import { GLTFExporter } from "three/examples/jsm/exporters/GLTFExporter.js";

class ShimProgressEvent extends Event {
  lengthComputable = false; loaded = 0; total = 0;
  constructor(type: string) { super(type); }
}
(globalThis as any).ProgressEvent ??= ShimProgressEvent;
(globalThis as any).requestAnimationFrame ??= (cb: () => void) => { cb(); return 0; };
class ShimFileReader {
  result: unknown = null; onload: (() => void) | null = null; onloadend: (() => void) | null = null; onerror: ((e: unknown) => void) | null = null;
  private done() { this.onloadend?.(); this.onload?.(); }
  readAsArrayBuffer(blob: Blob) { blob.arrayBuffer().then((b) => { this.result = b; this.done(); }, (e) => this.onerror?.(e)); }
  readAsDataURL(blob: Blob) { blob.arrayBuffer().then((b) => { this.result = `data:${blob.type || "application/octet-stream"};base64,${Buffer.from(b).toString("base64")}`; this.done(); }, (e) => this.onerror?.(e)); }
}
(globalThis as any).FileReader ??= ShimFileReader;

async function parseJSON(json: object): Promise<any> {
  return new Promise((res, rej) => new GLTFLoader().parse(JSON.stringify(json), "", res, rej));
}

async function tryLabel(label: string, fn: () => Promise<void>) {
  try { await fn(); console.log(`[OK] ${label}`); }
  catch (e) { console.log(`[FAIL] ${label}: ${String((e as Error).message ?? e).slice(0, 300)}`); }
}

const MIN_DOC = {
  asset: { version: "2.0" },
  scenes: [{ nodes: [0] }],
  scene: 0,
  nodes: [{ name: "root" }],
};

// 1) Does an unsupported REQUIRED extension throw?
await tryLabel("unsupported required extension -> throws?", async () => {
  await parseJSON({ ...MIN_DOC, extensionsRequired: ["ACME_totally_made_up"], extensionsUsed: ["ACME_totally_made_up"] });
});

// 2) Does a known required extension (KHR_materials_unlit) parse fine?
await tryLabel("known required extension KHR_materials_unlit -> parses", async () => {
  const doc = {
    asset: { version: "2.0" },
    scenes: [{ nodes: [0] }],
    scene: 0,
    nodes: [{ name: "root", mesh: 0 }],
    meshes: [{ primitives: [{ attributes: { POSITION: 0 }, material: 0, mode: 0 }] }],
    materials: [{ extensions: { KHR_materials_unlit: {} } }],
    accessors: [{ bufferView: 0, componentType: 5126, count: 1, type: "VEC3" }],
    bufferViews: [{ buffer: 0, byteOffset: 0, byteLength: 12 }],
    buffers: [{ byteLength: 12, uri: "data:application/octet-stream;base64," + Buffer.alloc(12).toString("base64") }],
    extensionsUsed: ["KHR_materials_unlit"],
    extensionsRequired: ["KHR_materials_unlit"],
  };
  const gltf = await parseJSON(doc);
  console.log("   parser.json.extensionsUsed =", gltf.parser.json.extensionsUsed, "Required =", gltf.parser.json.extensionsRequired);
});

// 3) extras on node -> userData?
await tryLabel("node extras -> userData", async () => {
  const gltf = await parseJSON({ ...MIN_DOC, nodes: [{ name: "root", extras: { foo: "bar" } }] });
  console.log("   node.userData =", JSON.stringify(gltf.scene.children[0]?.userData ?? gltf.scene.userData));
});

// 4) node.children order preserved in Object3D.children?
await tryLabel("node children order preserved", async () => {
  const doc = {
    asset: { version: "2.0" },
    scenes: [{ nodes: [0] }],
    scene: 0,
    nodes: [
      { name: "root", children: [3, 1, 2] },
      { name: "childB" },
      { name: "childC" },
      { name: "childA" },
    ],
  };
  const gltf = await parseJSON(doc);
  console.log("   root.children order =", gltf.scene.children[0].children.map((c: any) => c.name));
});

// 5) scene root node order (multiple root nodes) preserved?
await tryLabel("scene root node order preserved", async () => {
  const doc = {
    asset: { version: "2.0" },
    scenes: [{ nodes: [2, 0, 1] }],
    scene: 0,
    nodes: [{ name: "n0" }, { name: "n1" }, { name: "n2" }],
  };
  const gltf = await parseJSON(doc);
  console.log("   scene.children order =", gltf.scene.children.map((c: any) => c.name), " parser.json.scenes[0].nodes =", gltf.parser.json.scenes[0].nodes);
});

// 6) asset.version / generator / extras readable via parser.json?
await tryLabel("asset fields via parser.json", async () => {
  const gltf = await parseJSON({ asset: { version: "2.0", generator: "acme", copyright: "2026 acme", extras: { revision: 7 } }, scenes: [{ nodes: [] }], scene: 0 });
  console.log("   asset =", JSON.stringify(gltf.parser.json.asset));
});

// 7) camera bound to node -> THREE.Camera params?
await tryLabel("camera bound to node", async () => {
  const doc = {
    asset: { version: "2.0" },
    scenes: [{ nodes: [0] }],
    scene: 0,
    nodes: [{ name: "eye", camera: 0 }],
    cameras: [{ type: "perspective", perspective: { yfov: 0.8, znear: 0.1, zfar: 100, aspectRatio: 1.5 } }],
  };
  const gltf = await parseJSON(doc);
  const cam = gltf.scene.children[0];
  console.log("   camera isPerspectiveCamera=", cam.isPerspectiveCamera, "fov(deg)=", cam.fov, "near=", cam.near, "far=", cam.far);
});

// 8) skin -> SkinnedMesh?
await tryLabel("skin bound to mesh node", async () => {
  const ident = new Array(16).fill(0); for (let i = 0; i < 4; i++) ident[i * 5] = 1;
  const doc: any = {
    asset: { version: "2.0" },
    scenes: [{ nodes: [0, 1] }],
    scene: 0,
    nodes: [
      { name: "joint", },
      { name: "mesh0", mesh: 0, skin: 0 },
    ],
    skins: [{ joints: [0], inverseBindMatrices: 0 }],
    meshes: [{ primitives: [{ attributes: { POSITION: 0, JOINTS_0: 1, WEIGHTS_0: 2 }, material: undefined, mode: 0 }] }],
    accessors: [
      { bufferView: 0, byteOffset: 0, componentType: 5126, count: 1, type: "VEC3" },
      { bufferView: 1, byteOffset: 0, componentType: 5121, count: 1, type: "VEC4" },
      { bufferView: 2, byteOffset: 0, componentType: 5126, count: 1, type: "VEC4" },
      { bufferView: 3, byteOffset: 0, componentType: 5126, count: 1, type: "MAT4" },
    ],
    bufferViews: [
      { buffer: 0, byteOffset: 0, byteLength: 12 },
      { buffer: 0, byteOffset: 12, byteLength: 4 },
      { buffer: 0, byteOffset: 16, byteLength: 16 },
      { buffer: 0, byteOffset: 32, byteLength: 64 },
    ],
    buffers: [{ byteLength: 96, uri: "data:application/octet-stream;base64," + Buffer.concat([Buffer.alloc(12), Buffer.alloc(4), Buffer.alloc(16), Buffer.from(new Float32Array(ident).buffer)]).toString("base64") }],
  };
  const gltf = await parseJSON(doc);
  const meshNode = gltf.scene.children[1];
  console.log("   isSkinnedMesh=", meshNode.isSkinnedMesh, "skeleton.bones.length=", meshNode.skeleton?.bones?.length);
});

// 9) morph weights -> mesh.morphTargetInfluences?
await tryLabel("morph target weights on node", async () => {
  const doc: any = {
    asset: { version: "2.0" },
    scenes: [{ nodes: [0] }],
    scene: 0,
    nodes: [{ name: "n0", mesh: 0, weights: [0.25] }],
    meshes: [{ primitives: [{ attributes: { POSITION: 0 }, targets: [{ POSITION: 1 }], mode: 0 }] }],
    accessors: [
      { bufferView: 0, componentType: 5126, count: 1, type: "VEC3" },
      { bufferView: 1, componentType: 5126, count: 1, type: "VEC3" },
    ],
    bufferViews: [{ buffer: 0, byteOffset: 0, byteLength: 12 }, { buffer: 0, byteOffset: 12, byteLength: 12 }],
    buffers: [{ byteLength: 24, uri: "data:application/octet-stream;base64," + Buffer.alloc(24).toString("base64") }],
  };
  const gltf = await parseJSON(doc);
  const meshNode = gltf.scene.children[0];
  console.log("   morphTargetInfluences=", meshNode.morphTargetInfluences, "geometry.morphAttributes.position.length=", meshNode.geometry.morphAttributes.position?.length);
});

// 10) animations array -> AnimationClip list?
await tryLabel("animation clip present", async () => {
  const doc: any = {
    asset: { version: "2.0" },
    scenes: [{ nodes: [0] }],
    scene: 0,
    nodes: [{ name: "n0" }],
    animations: [{
      name: "clip0",
      channels: [{ sampler: 0, target: { node: 0, path: "translation" } }],
      samplers: [{ input: 0, output: 1 }],
    }],
    accessors: [
      { bufferView: 0, componentType: 5126, count: 2, type: "SCALAR" },
      { bufferView: 1, componentType: 5126, count: 2, type: "VEC3" },
    ],
    bufferViews: [{ buffer: 0, byteOffset: 0, byteLength: 8 }, { buffer: 0, byteOffset: 8, byteLength: 24 }],
    buffers: [{ byteLength: 32, uri: "data:application/octet-stream;base64," + Buffer.from(new Float32Array([0, 1, 0, 0, 0, 1, 1, 1]).buffer).toString("base64") }],
  };
  const gltf = await parseJSON(doc);
  console.log("   animations.length=", gltf.animations.length, "name=", gltf.animations[0]?.name, "tracks=", gltf.animations[0]?.tracks.length);
});

// 11) primitive.mode TRIANGLE_STRIP conversion witnessable via index/vertex count change?
await tryLabel("primitive mode conversion (5=TRIANGLE_STRIP)", async () => {
  const doc: any = {
    asset: { version: "2.0" },
    scenes: [{ nodes: [0] }],
    scene: 0,
    nodes: [{ name: "n0", mesh: 0 }],
    meshes: [{ primitives: [{ attributes: { POSITION: 0 }, mode: 5 }] }],
    accessors: [{ bufferView: 0, componentType: 5126, count: 4, type: "VEC3" }],
    bufferViews: [{ buffer: 0, byteOffset: 0, byteLength: 48 }],
    buffers: [{ byteLength: 48, uri: "data:application/octet-stream;base64," + Buffer.from(new Float32Array([0,0,0, 1,0,0, 0,1,0, 1,1,0]).buffer).toString("base64") }],
  };
  const gltf = await parseJSON(doc);
  const mesh = gltf.scene.children[0];
  console.log("   geometry.index count=", mesh.geometry.index?.count, "drawMode-ish (isMesh)=", mesh.isMesh, "position count=", mesh.geometry.attributes.position.count);
});

// 12) GLTFExporter: custom plugin hook to inject arbitrary top-level JSON (orphan buffer/accessor)?
await tryLabel("GLTFExporter plugin can inject raw json fields", async () => {
  const scene = new THREE.Scene();
  const mesh = new THREE.Mesh(new THREE.BoxGeometry(1, 1, 1), new THREE.MeshStandardMaterial());
  scene.add(mesh);
  const exporter = new GLTFExporter();
  exporter.register((writer: any) => ({
    afterParse() {
      writer.json.extras = writer.json.extras || {};
      writer.json.extras.injectedByPlugin = true;
    },
  }));
  const out: any = await new Promise((res, rej) => exporter.parse(scene, res, rej, { binary: false }));
  console.log("   exported top-level keys=", Object.keys(out).join(","), " extras=", JSON.stringify(out.extras));
});

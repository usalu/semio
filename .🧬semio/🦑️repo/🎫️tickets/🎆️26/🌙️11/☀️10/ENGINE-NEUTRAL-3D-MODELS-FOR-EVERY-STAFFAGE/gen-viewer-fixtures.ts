import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { Box3, Matrix4, Raycaster, Vector3 } from "three";
import { featureEdges, tessellate } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { boundsOf, cameraOf, createFrame, fitCamera, frameOf, sceneOf, PRESETS, basisOf, clipPlanes, viewMatrix, projectionMatrix, type CameraPreset } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🖥️viewer/🟦️.ts";
import { castPixels, threeCamera, threeWorld, withoutLines } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🖥️viewer/🧪️tests/🧰️support/🟦️.ts";

const out = resolve(process.cwd(), "🧰️framework/🔨️modules/🧊️3d/🗿️model/🖥️viewer/🧫️fixtures");
const write = (name: string, data: unknown) => {
  mkdirSync(resolve(out, name), { recursive: true });
  writeFileSync(resolve(out, name, "🔣️.json"), JSON.stringify(data, (_, v) => (typeof v === "number" ? Math.round(v * 1e6) / 1e6 + 0 : v), 1) + "\n");
};
const t = (en: string) => ({ en, de: en });
const I: number[] = [0, 0, 0, 1];
const node = (id: string, parent: string | null, translation: number[], extra: object = {}) => ({ id, parent, translation, rotation: I, scale: [1, 1, 1], ...extra });
const mat = (id: string, baseColor: number[], extra: object = {}) => ({ id, title: t(id), baseColor, metallic: 0, roughness: 1, emissive: [0, 0, 0], alphaMode: "opaque", doubleSided: false, unlit: false, ...extra });
const q = (axis: number, angle: number) => { const s = Math.sin(angle / 2); const v = [0, 0, 0, Math.cos(angle / 2)]; v[axis] = s; return v; };

const procedural: any = {
  version: "semio.model/v1", id: "viewer-rig", title: t("Viewer rig"),
  nodes: [
    node("root", null, [0, 0, 0]),
    node("upper", "root", [0, 1, 0], { joint: { type: "revolute", axis: [0, 0, 1], min: -1.5, max: 1.5 } }),
    node("lower", "upper", [0, 1, 0], { joint: { type: "revolute", axis: [0, 0, 1], min: -1.5, max: 1.5 } }),
    node("head", "lower", [0, 0.9, 0]),
    node("halo-a", "root", [0, 1, -1]),
    node("halo-b", "root", [0, 1, 1]),
  ],
  solids: [
    { id: "limb", kind: "capsule", node: "root", material: "skin", p0: [0, 0, 0], p1: [0, 2, 0], r0: 0.25, r1: 0.2, weights: [{ node: "upper", weight: 1 }, { node: "lower", weight: 1 }] },
    { id: "head", kind: "ellipsoid", node: "head", material: "cloth", radii: [0.3, 0.35, 0.3] },
    { id: "pad", kind: "box", node: "root", material: "cloth", size: [1, 0.1, 1], transform: { translation: [0, -0.05, 0] } },
    { id: "a", kind: "box", node: "halo-a", material: "glass", size: [0.8, 0.8, 0.05] },
    { id: "b", kind: "box", node: "halo-b", material: "glass", size: [0.8, 0.8, 0.05] },
  ],
  meshes: [], skins: [],
  materials: [
    mat("skin", [0.8, 0.6, 0.5, 1]), mat("cloth", [0.2, 0.4, 0.8, 1], { doubleSided: true }), mat("glass", [0.2, 0.8, 0.4, 0.4], { alphaMode: "blend" }),
    mat("skin-dark", [0.3, 0.2, 0.15, 1]), mat("cloth-dark", [0.1, 0.15, 0.3, 1]), mat("ink", [0.05, 0.05, 0.05, 1], { unlit: true }), mat("ink-light", [0.95, 0.95, 0.95, 1], { unlit: true }),
  ],
  variants: [
    { id: "light", title: t("Light"), bindings: {} },
    { id: "dark", title: t("Dark"), bindings: { skin: "skin-dark", cloth: "cloth-dark", ink: "ink-light" } },
  ],
  poses: [{ id: "bent", title: t("Bent"), nodes: { upper: { rotation: q(2, 0.5) }, lower: { rotation: q(2, -0.7) } } }],
  animations: [{
    id: "wave", title: t("Wave"), durationS: 2, loop: true, events: [],
    channels: [
      { target: { node: "upper", path: "rotation" }, interpolation: "linear", times: [0, 1, 2], values: [...q(2, 0), ...q(2, 0.8), ...q(2, 0)] },
      { target: { material: "skin-dark", path: "baseColor" }, interpolation: "linear", times: [0, 2], values: [0.1, 0.1, 0.1, 1, 0.9, 0.1, 0.1, 1] },
      { target: { material: "skin", path: "emissive" }, interpolation: "step", times: [0, 1], values: [0, 0, 0, 0.5, 0.25, 0] },
    ],
  }],
};
const tess = await tessellate(procedural, { chordM: 0.05, minSegments: 8, maxSegments: 12 });
const doc: any = await featureEdges(tess, (30 * Math.PI) / 180, { material: "ink" });
write("🧱️rig", { document: doc });

const scene = sceneOf(doc);
const frame = createFrame(scene);
const near = (v: number[]) => v.map((x) => Math.round(x * 1e6) / 1e6 + 0);

const boxOf = { min: [-1, 0, -1], max: [1, 2, 1] };
const cameraCases = (["front", "elevation", "plan", "axonometric", "perspective"] as CameraPreset[]).flatMap((preset) => [1, 1.6].map((aspect) => {
  const cam = { ...cameraOf(preset), target: [0.3, 1.1, -0.2] as [number, number, number], distance: 6, orthoHeight: 5, fovY: 0.7, extent: 2 };
  const tc = threeCamera(cam, aspect);
  return {
    name: `${preset}-${aspect}`, preset, aspect, camera: cam, eye: near(tc.position.toArray()),
    view: near(tc.matrixWorldInverse.toArray()), projection: near(tc.projectionMatrix.toArray()),
    ray: { ndc: [0.25, -0.5], origin: near(new Raycaster().ray.origin.toArray()), },
  };
}));
for (const c of cameraCases) {
  const tc = threeCamera(c.camera as any, c.aspect);
  const r = new Raycaster();
  r.setFromCamera({ x: 0.25, y: -0.5 } as any, tc);
  (c as any).ray = { ndc: [0.25, -0.5], origin: near(r.ray.origin.toArray()), direction: near(r.ray.direction.toArray()) };
}
const fitCases = (["front", "elevation", "plan", "axonometric", "perspective"] as CameraPreset[]).flatMap((preset) => [1, 1.6].map((aspect) => {
  const fitted = fitCamera(cameraOf(preset), boxOf as any, aspect, 1.1);
  return { name: `${preset}-${aspect}`, preset, aspect, bounds: boxOf, margin: 1.1, fitted };
}));
write("📷️camera", { tolerance: 1e-5, cameras: cameraCases, fits: fitCases, presets: PRESETS });

const bentPose = "bent";
const sampleAt = (pose: any, variant: string | null) => {
  const f = frameOf(scene, { pose, variant, camera: cameraOf("front"), aspect: 1 }, createFrame(scene));
  return f;
};
const w = threeWorld(doc, bentPose);
const palette = w.skeletons[0]!.boneMatrices!;
const waveWorld = threeWorld(doc, { animation: "wave", timeS: 0.5 });
const poseCases = [
  { name: "bent", pose: bentPose, palette: near([...palette]) },
  { name: "wave-0.5", pose: { animation: "wave", timeS: 0.5 }, palette: near([...waveWorld.skeletons[0]!.boneMatrices!]) },
];
const skinnedVertices = (world: typeof w, step: number) => {
  const m = world.skinned[0]!, n = m.geometry.getAttribute("position").count, res: number[][] = [];
  for (let v = 0; v < n; v += step) { const p = new Vector3(); m.getVertexPosition(v, p); p.applyMatrix4(m.matrixWorld); res.push([v, ...near(p.toArray())]); }
  return res;
};
const materialCases = [
  { pose: null, variant: null, material: "skin", baseColor: [0.8, 0.6, 0.5, 1], emissive: [0, 0, 0] },
  { pose: { animation: "wave", timeS: 0.5 }, variant: "dark", material: "skin-dark", baseColor: [0.3, 0.1, 0.1, 1], emissive: [0, 0, 0] },
  { pose: { animation: "wave", timeS: 1.5 }, variant: "dark", material: "skin-dark", baseColor: [0.7, 0.1, 0.1, 1], emissive: [0, 0, 0] },
  { pose: { animation: "wave", timeS: 1.5 }, variant: "dark", material: "skin", baseColor: [0.8, 0.6, 0.5, 1], emissive: [0.5, 0.25, 0] },
  { pose: { animation: "wave", timeS: 2.5 }, variant: null, material: "skin", baseColor: [0.8, 0.6, 0.5, 1], emissive: [0, 0, 0] },
];
const rig = { jointCount: scene.jointCount, nodes: scene.nodeIds, primitives: scene.primitives.map((p) => ({ node: p.node, mode: p.mode, material: p.material, skin: p.skin, vertexCount: p.vertexCount, indexCount: p.indexCount })), vertexCount: scene.vertices.length / 10, indexCount: scene.indices.length };
write("🎞️frame", { tolerance: 1e-5, rig, poses: poseCases, skinnedVertices: { bent: skinnedVertices(w, 9), wave: skinnedVertices(waveWorld, 9) }, materials: materialCases, boxOf });

const solidOfNode: Record<string, string> = Object.fromEntries(procedural.solids.map((s: any) => [s.weights ? "skinned" : s.node, s.id]));
const ray = new Raycaster();
const pickCases = [
  { name: "head-front", origin: [...new Vector3().setFromMatrixPosition(w.objects.get("head")!.matrixWorld).toArray().map((x) => Math.round(x * 1e6) / 1e6)].map((x, i) => (i === 2 ? x + 5 : x)), direction: [0, 0, -1] },
  { name: "limb-side", origin: [5, 0.4, 0], direction: [-1, 0, 0] },
  { name: "pad-down", origin: [0.4, 4, 0.1], direction: [0, -1, 0] },
  { name: "miss", origin: [8, 8, 8], direction: [0, 1, 0] },
  { name: "glass-front", origin: [0, 1, 5], direction: [0, 0, -1] },
].map((c) => {
  const dir = new Vector3(...(c.direction as [number, number, number])).normalize();
  ray.set(new Vector3(...(c.origin as [number, number, number])), dir);
  const hit = ray.intersectObjects(w.meshes, false)[0];
  const node = hit ? hit.object.userData.node : null;
  return { ...c, node, solid: node === null ? null : solidOfNode[node], distance: hit ? Math.round(hit.distance * 1e6) / 1e6 : null };
});
write("🎯️picking", { tolerance: 1e-4, pose: bentPose, cases: pickCases });

const stripped = withoutLines(doc);
const strippedScene = sceneOf(stripped);
const sw = threeWorld(stripped, bentPose);
const W = 40, H = 32;
const rasters = (["front", "elevation", "plan", "axonometric", "perspective"] as CameraPreset[]).map((preset) => {
  const f0 = frameOf(strippedScene, { pose: bentPose, camera: cameraOf(preset), aspect: W / H }, createFrame(strippedScene));
  const cam = fitCamera(cameraOf(preset), boundsOf(strippedScene, f0), W / H, 1.1);
  const o = castPixels(sw, threeCamera(cam, W / H), W, H);
  const mask: string[] = [];
  for (let y = 0; y < H; y++) mask.push([...o.hit.slice(y * W, y * W + W)].join(""));
  const depth: number[][] = [];
  for (let y = 2; y < H; y += 6) for (let x = 2; x < W; x += 6) if (o.hit[y * W + x]) depth.push([x, y, Math.round(o.distance[y * W + x]! * 1e5) / 1e5]);
  return { preset, width: W, height: H, camera: cam, mask, depth };
});
write("🖼️raster", { pose: bentPose, maxMismatch: 0.005, depthTolerance: 2e-3, document: stripped.id, images: rasters });
console.log("ok", doc.meshes.map((m: any) => m.id), scene.primitives.length, scene.vertices.length / 10);

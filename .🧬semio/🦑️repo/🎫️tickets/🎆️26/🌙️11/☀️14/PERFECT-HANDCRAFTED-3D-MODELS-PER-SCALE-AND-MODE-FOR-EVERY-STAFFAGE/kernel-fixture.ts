// Generates 🧫️fixtures/🎞️scenes/🔣️.json: hand-listed structure, digests recorded from the implementation (cross-checked by the oracle tests).
// usage: bun kernel-fixture.ts
import { createHash } from "node:crypto";
import { writeFileSync, mkdirSync } from "node:fs";
const M = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🗿️model";
const { tessellate, encodeGlb, encodeFbx, selectScene, solidsOfScene, nodesOfScene, boundsOf } = await import(`${M}/🟦️.ts`);
const id = [0, 0, 0, 1], one = [1, 1, 1];
const node = (id_: string, parent: string | null, translation: number[], extra: object = {}) => ({ id: id_, parent, translation, rotation: id, scale: one, ...extra });
const text = (en: string, de: string) => ({ en, de });
const material = (id_: string, en: string, de: string, baseColor: number[]) => ({ id: id_, title: text(en, de), baseColor, metallic: 0, roughness: 0.8, emissive: [0, 0, 0], alphaMode: "opaque", doubleSided: false, unlit: false });
const document = {
  version: "semio.model/v1", id: "scenes", title: text("Scenes", "Szenen"),
  nodes: [
    node("root", null, [0, 0, 0]),
    node("spine", "root", [0, 1, 0], { joint: { type: "fixed", axis: [0, 1, 0] }, tags: ["bone"] }),
    node("arm", "spine", [0.5, 0, 0], { joint: { type: "revolute", axis: [0, 0, 1], min: -1.5, max: 1.5 }, tags: ["bone"] }),
    node("hull-body", "spine", [0, 0, 0]),
    node("hull-arm", "arm", [0, 0, 0]),
    node("frame-body", "spine", [0, 0, 0]),
    node("frame-arm", "arm", [0, 0, 0]),
    node("skin-shared", "root", [0, 0, 0]),
    node("region-a", "spine", [0, 0, 0]),
    node("skin-thermal", "root", [0, 0, 0]),
  ],
  scenes: [
    { id: "body", title: text("Body", "Körper"), nodes: ["hull-body", "hull-arm", "skin-shared"], tags: ["tone"] },
    { id: "structure", title: text("Structure", "Tragwerk"), nodes: ["frame-body", "frame-arm", "skin-shared"] },
    { id: "thermal", title: text("Thermal", "Thermik"), nodes: ["region-a", "skin-thermal"] },
  ],
  solids: [
    { id: "hull-torso", kind: "box", node: "hull-body", material: "hull", size: [0.4, 0.8, 0.3] },
    { id: "hull-upper-arm", kind: "capsule", node: "hull-arm", material: "hull", p0: [0, 0, 0], p1: [0.4, 0, 0], r0: 0.06, r1: 0.05 },
    { id: "frame-column", kind: "box", node: "frame-body", material: "frame", size: [0.1, 0.9, 0.1] },
    { id: "frame-beam", kind: "ellipsoid", node: "frame-arm", material: "frame", radii: [0.2, 0.04, 0.04], transform: { translation: [0.2, 0, 0] } },
    { id: "shared-skin", kind: "ellipsoid", node: "skin-shared", material: "skin", radii: [0.1, 0.1, 0.1], transform: { translation: [0.5, 1, 0] }, weights: [{ node: "spine", weight: 0.5 }, { node: "arm", weight: 0.5 }] },
    { id: "region-core", kind: "box", node: "region-a", material: "heat", size: [0.3, 0.3, 0.3], transform: { translation: [0, 0.2, 0.3] } },
    { id: "thermal-skin", kind: "ellipsoid", node: "skin-thermal", material: "heat", radii: [0.08, 0.08, 0.08], transform: { translation: [0.5, 1, 0.3] }, weights: [{ node: "arm", weight: 1 }] },
  ],
  meshes: [], skins: [],
  materials: [material("hull", "Hull", "Hülle", [0.8, 0.7, 0.6, 1]), material("frame", "Frame", "Rahmen", [0.4, 0.4, 0.45, 1]), material("skin", "Skin", "Haut", [0.9, 0.6, 0.5, 1]), material("heat", "Heat", "Wärme", [1, 0.4, 0.1, 1]), material("hull-dark", "Dark hull", "Dunkle Hülle", [0.3, 0.25, 0.2, 1])],
  variants: [{ id: "dark", title: text("Dark", "Dunkel"), bindings: { hull: "hull-dark" } }],
  poses: [{ id: "bent", title: text("Bent", "Gebeugt"), nodes: { arm: { rotation: [0, 0, 0.3826834323650898, 0.9238795325112867] } } }],
  animations: [{ id: "wave", title: text("Wave", "Winken"), durationS: 1, loop: true, events: [],
    channels: [
      { target: { node: "arm", path: "rotation" }, interpolation: "linear", times: [0, 0.5, 1], values: [0, 0, 0, 1, 0, 0, 0.3826834323650898, 0.9238795325112867, 0, 0, 0, 1] },
      { target: { material: "hull", path: "baseColor" }, interpolation: "linear", times: [0, 1], values: [0.8, 0.7, 0.6, 1, 0.6, 0.5, 0.4, 1] },
    ] }],
};
const quality = { chordM: 0.001, minSegments: 12, maxSegments: 32 };
const quiet = { report: () => undefined, cancelled: () => false };
const resolved = await tessellate(document, quality, quiet);
const sha = (b: Uint8Array) => createHash("sha256").update(b).digest("hex");
const glbOptions = { animations: true, variants: true };
const glb = {
  all: sha(encodeGlb(resolved, glbOptions, quiet)),
  perScene: Object.fromEntries(document.scenes.map((s) => [s.id, sha(encodeGlb(resolved, { ...glbOptions, scene: s.id }, quiet))])),
  posed: sha(encodeGlb(resolved, { ...glbOptions, pose: "bent" }, quiet)),
};
const fbx = Object.fromEntries(document.scenes.map((s) => [s.id, sha(encodeFbx(resolved, { format: "binary", scene: s.id, animation: "wave" }, quiet))]));
const bounds = Object.fromEntries(document.scenes.map((s) => [s.id, { rest: boundsOf(resolved, undefined, { scene: s.id }), bent: boundsOf(resolved, "bent", { scene: s.id }) }]));
const expected = {
  closure: { body: ["root", "spine", "arm", "hull-body", "hull-arm", "skin-shared"], structure: ["root", "spine", "arm", "frame-body", "frame-arm", "skin-shared"], thermal: ["root", "spine", "arm", "region-a", "skin-thermal"] },
  solids: { body: ["hull-torso", "hull-upper-arm", "shared-skin"], structure: ["frame-column", "frame-beam", "shared-skin"], thermal: ["region-core", "thermal-skin"] },
  resolvedSceneNodes: Object.fromEntries(resolved.scenes.map((s: { id: string; nodes: string[] }) => [s.id, s.nodes])),
  skinnedNodes: resolved.nodes.filter((n: { skin?: string }) => n.skin).map((n: { id: string }) => n.id),
  glb, fbx, bounds,
};
const body = { id: "body", title: text("Body", "Körper"), nodes: ["hull-body", "hull-arm", "skin-shared"] };
const invalid = [
  { name: "unknown-node", scenes: [{ ...body, nodes: [...body.nodes, "ghost"] }, document.scenes[1], document.scenes[2]], issues: [{ code: "unknown-node", path: "/scenes/0/nodes/3" }] },
  { name: "duplicate-scene-id", scenes: [body, { ...document.scenes[1], id: "body" }, document.scenes[2]], issues: [{ code: "duplicate-id", path: "/scenes/1/id" }] },
  { name: "duplicate-node-in-scene", scenes: [{ ...body, nodes: ["hull-body", "hull-arm", "skin-shared", "hull-body"] }, document.scenes[1], document.scenes[2]], issues: [{ code: "duplicate-id", path: "/scenes/0/nodes/3" }] },
  { name: "unscened-node", scenes: [body, document.scenes[1], { ...document.scenes[2], nodes: ["skin-thermal"] }], issues: [{ code: "unscened-node", path: "/nodes/8" }] },
  { name: "empty-scene", scenes: [body, document.scenes[1], { ...document.scenes[2], nodes: [] }], issues: [{ code: "empty-scene", path: "/scenes/2/nodes" }, { code: "unscened-node", path: "/nodes/8" }, { code: "unscened-node", path: "/nodes/9" }] },
  { name: "bad-slug", scenes: [{ ...body, id: "Body 1" }, document.scenes[1], document.scenes[2]], issues: [{ code: "bad-slug", path: "/scenes/0/id" }] },
];
mkdirSync(`${M}/🧫️fixtures/🎞️scenes`, { recursive: true });
writeFileSync(`${M}/🧫️fixtures/🎞️scenes/🔣️.json`, JSON.stringify({ schema: "semio.model.scenes-vector/v1", quality, document, invalid, expected }, null, 2) + "\n");
console.log(JSON.stringify(expected, null, 1).slice(0, 3000));

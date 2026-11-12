import { createHash } from "node:crypto";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { encodeGlb } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🧊️glb/🟦️.ts";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🧊️glb/🧫️fixtures");
const q = Math.SQRT1_2;
const text = (en: string, de: string) => ({ en, de });
const operation = { report() {}, cancelled: () => false };

const rig = {
  version: "semio.model/v1",
  id: "rig",
  title: text("Rig", "Gerüst"),
  provenance: { generator: "glb-fixture", notes: [text("hand-made", "von Hand")] },
  nodes: [
    { id: "root", parent: null, translation: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1] },
    { id: "spine", parent: "root", translation: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1], joint: { type: "fixed", axis: [0, 1, 0] }, tags: ["bone"] },
    { id: "arm", parent: "spine", translation: [0, 0.5, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1], joint: { type: "revolute", axis: [0, 0, 1], min: -2, max: 2 }, tags: ["bone"] },
    { id: "body", parent: "root", translation: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1], mesh: "body", skin: "rig" },
    { id: "outline", parent: "root", translation: [0.25, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1], mesh: "outline" },
  ],
  solids: [],
  meshes: [
    {
      id: "body",
      primitives: [
        {
          mode: "triangles",
          material: "skin",
          positions: [-0.1, 0, 0, 0.1, 0, 0, -0.1, 0.5, 0, 0.1, 0.5, 0, -0.1, 1, 0, 0.1, 1, 0],
          normals: [0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
          colors: [1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1],
          joints: [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
          weights: [1, 0, 0, 0, 1, 0, 0, 0, 0.5, 0.5, 0, 0, 0.5, 0.5, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
          indices: [0, 1, 2, 1, 3, 2, 2, 3, 4, 3, 5, 4],
          solids: [{ id: "torso", first: 0, count: 6 }, { id: "belly", first: 6, count: 6 }],
        },
      ],
    },
    { id: "outline", primitives: [{ mode: "lines", material: "ink", positions: [0, 0, 0, 0, 1, 0, 0.2, 1, 0], indices: [0, 1, 1, 2], solids: [{ id: "torso", first: 0, count: 2 }, { id: "belly", first: 2, count: 2 }] }] },
  ],
  skins: [{ id: "rig", joints: ["spine", "arm"], inverseBind: [[1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1], [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, -0.5, 0, 1]] }],
  materials: [
    { id: "skin", title: text("Skin", "Haut"), baseColor: [0.9, 0.7, 0.6, 1], metallic: 0, roughness: 0.75, emissive: [0, 0, 0], alphaMode: "opaque", doubleSided: false, unlit: false },
    { id: "skin-dark", title: text("Dark skin", "Dunkle Haut"), baseColor: [0.2, 0.15, 0.1, 1], metallic: 0, roughness: 0.75, emissive: [0, 0, 0], alphaMode: "opaque", doubleSided: false, unlit: false },
    { id: "ink", title: text("Ink", "Tinte"), baseColor: [0, 0, 0, 1], metallic: 0, roughness: 1, emissive: [0, 0, 0], alphaMode: "opaque", doubleSided: false, unlit: true },
    { id: "ink-light", title: text("Light ink", "Helle Tinte"), baseColor: [1, 1, 1, 0.5], metallic: 0, roughness: 1, emissive: [0.1, 0.1, 0.1], alphaMode: "blend", doubleSided: true, unlit: true },
  ],
  variants: [
    { id: "light-solid", title: text("Light solid", "Hell massiv"), bindings: {} },
    { id: "dark-solid", title: text("Dark solid", "Dunkel massiv"), bindings: { skin: "skin-dark", ink: "ink-light" } },
  ],
  poses: [{ id: "bent", title: text("Bent", "Gebeugt"), nodes: { arm: { rotation: [0, 0, q, q] }, outline: { translation: [0.5, 0, 0] } } }],
  animations: [
    {
      id: "wave",
      title: text("Wave", "Winken"),
      durationS: 1,
      loop: true,
      channels: [
        { target: { node: "arm", path: "rotation" }, interpolation: "linear", times: [0, 0.5, 1], values: [0, 0, 0, 1, 0, 0, q, q, 0, 0, 0, 1] },
        { target: { node: "spine", path: "translation" }, interpolation: "step", times: [0, 1], values: [0, 0, 0, 0, 0.1, 0] },
        { target: { node: "outline", path: "scale" }, interpolation: "linear", times: [0, 1], values: [1, 1, 1, 2, 2, 2] },
        { target: { material: "skin", path: "baseColor" }, interpolation: "linear", times: [0, 1], values: [0.9, 0.7, 0.6, 1, 0.5, 0.3, 0.2, 1] },
        { target: { material: "skin", path: "emissive" }, interpolation: "step", times: [0, 0.5], values: [0, 0, 0, 0.5, 0.25, 0] },
      ],
      events: [{ t: 0.5, id: "peak" }],
    },
  ],
};

const minimal = {
  version: "semio.model/v1",
  id: "triangle",
  title: text("Triangle", "Dreieck"),
  nodes: [{ id: "root", parent: null, translation: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1], mesh: "triangle" }],
  solids: [],
  meshes: [{ id: "triangle", primitives: [{ mode: "triangles", material: "grey", positions: [0, 0, 0, 1, 0, 0, 0, 1, 0], indices: [0, 1, 2] }] }],
  skins: [],
  materials: [{ id: "grey", title: text("Grey", "Grau"), baseColor: [0.5, 0.5, 0.5, 1], metallic: 0, roughness: 1, emissive: [0, 0, 0], alphaMode: "opaque", doubleSided: false, unlit: false }],
  variants: [],
  poses: [],
  animations: [],
};

const cases: Array<[string, unknown, { pose?: unknown; animations: boolean; variants: boolean }]> = [
  ["minimal-triangle", minimal, { animations: false, variants: false }],
  ["rig-full", rig, { animations: true, variants: true }],
  ["rig-bent-pose", rig, { pose: "bent", animations: false, variants: false }],
  ["rig-wave-quarter", rig, { pose: { animation: "wave", timeS: 0.25 }, animations: false, variants: true }],
];

for (const [name, document, options] of cases) {
  const bytes = encodeGlb(document as never, options as never, operation);
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const jsonLength = view.getUint32(12, true);
  const json = JSON.parse(new TextDecoder().decode(bytes.subarray(20, 20 + jsonLength)));
  const binLength = bytes.length > 20 + jsonLength ? view.getUint32(20 + jsonLength, true) : 0;
  const expected = {
    glbByteLength: bytes.length,
    jsonChunkByteLength: jsonLength,
    binChunkByteLength: binLength,
    sha256: createHash("sha256").update(bytes).digest("hex"),
    accessorCounts: (json.accessors ?? []).map((a: { type: string; count: number }) => `${a.type}:${a.count}`),
    json,
  };
  const dir = resolve(root, `🧊️${name}`);
  mkdirSync(dir, { recursive: true });
  writeFileSync(resolve(dir, "🔣️.json"), `${JSON.stringify({ schema: "semio.model.glb-vector/v1", name, document, options, expected }, null, 2)}\n`);
  console.log(name, bytes.length, expected.sha256);
}

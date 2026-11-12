import { mkdirSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { fileURLToPath } from "node:url";
import { Matrix4, Quaternion, Vector3 } from "three";
import { encodeFbxDetailed } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🎬️fbx/🟦️.ts";
import { connectionList, objectCounts, outline, readFbx } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🎬️fbx/📖️reader/🟦️.ts";
import { deflateZlib } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🎬️fbx/🗜️deflate/🟦️.ts";

const root = fileURLToPath(new URL("../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🎬️fbx/🧫️fixtures/", import.meta.url));
const text = (en: string, de: string) => ({ en, de });
const qx = (angle: number) => [Math.sin(angle / 2), 0, 0, Math.cos(angle / 2)];
const qy = (angle: number) => [0, Math.sin(angle / 2), 0, Math.cos(angle / 2)];
const ID = [0, 0, 0, 1];
const node = (id: string, parent: string | null, translation = [0, 0, 0], rotation = ID, scale = [1, 1, 1], extra: object = {}) => ({ id, parent, translation, rotation, scale, ...extra });
const material = (id: string, baseColor: number[], extra: object = {}) => ({ id, title: text(id, id), baseColor, metallic: 0, roughness: 0.8, emissive: [0, 0, 0], alphaMode: "opaque", doubleSided: false, unlit: false, ...extra });
const base = { $schema: "../../../../🧬️schema/🔣️.json", version: "semio.model/v1", solids: [], variants: [], poses: [], animations: [], skins: [], meshes: [], materials: [] };

function matrixOf(translation: number[], rotation: number[]): number[] {
  return new Matrix4().compose(new Vector3(...translation), new Quaternion(...rotation), new Vector3(1, 1, 1)).toArray();
}
const inverse = (m: number[]) => new Matrix4().fromArray(m).invert().toArray();

function box(prefix: string, lo: number[], hi: number[]) {
  const positions: number[] = [];
  const corners: number[][] = [];
  for (const z of [lo[2], hi[2]]) for (const y of [lo[1], hi[1]]) for (const x of [lo[0], hi[0]]) corners.push([x, y, z]);
  corners.forEach((c) => positions.push(...c));
  const centre = [(lo[0] + hi[0]) / 2, (lo[1] + hi[1]) / 2, (lo[2] + hi[2]) / 2];
  const dir = corners.map((c) => {
    const d = [c[0] - centre[0], c[1] - centre[1], c[2] - centre[2]];
    const l = Math.hypot(...d);
    return d.map((v) => Math.round((v / l) * 1e6) / 1e6);
  });
  const idx = (x: number, y: number, z: number) => x + 2 * y + 4 * z;
  const quad = (a: number, b: number, c: number, d: number) => [a, b, c, a, c, d];
  const side = [quad(idx(0, 0, 1), idx(1, 0, 1), idx(1, 1, 1), idx(0, 1, 1)), quad(idx(1, 0, 0), idx(0, 0, 0), idx(0, 1, 0), idx(1, 1, 0)), quad(idx(1, 0, 1), idx(1, 0, 0), idx(1, 1, 0), idx(1, 1, 1)), quad(idx(0, 0, 0), idx(0, 0, 1), idx(0, 1, 1), idx(0, 1, 0))].flat();
  const caps = [quad(idx(0, 1, 1), idx(1, 1, 1), idx(1, 1, 0), idx(0, 1, 0)), quad(idx(0, 0, 0), idx(1, 0, 0), idx(1, 0, 1), idx(0, 0, 1))].flat();
  return { positions, normals: dir.flat(), side, caps, corners, prefix };
}

function riggedArm() {
  const upperWorld = matrixOf([0, 1, 0], ID);
  const lowerWorld = matrixOf([0, 1.5, 0], qx(0.3));
  const b = box("arm", [-0.1, 1, -0.1], [0.1, 2, 0.1]);
  const joints: number[] = [];
  const weights: number[] = [];
  b.corners.forEach((c) => {
    if (c[1] === 1) {
      joints.push(0, 0, 0, 0);
      weights.push(1, 0, 0, 0);
    } else {
      joints.push(0, 1, 0, 0);
      weights.push(0.25, 0.75, 0, 0);
    }
  });
  const colors = b.corners.flatMap((_, i) => [0.2 + i * 0.1, 0.5, 0.9 - i * 0.1, 1]);
  const capNormals = b.corners.flatMap((c) => [0, c[1] === 2 ? 1 : -1, 0]);
  return {
    ...base,
    id: "rigged-arm",
    title: text("Rigged arm", "Gerigter Arm"),
    nodes: [
      node("root", null),
      node("upper", "root", [0, 1, 0], ID, [1, 1, 1], { joint: { type: "revolute", axis: [1, 0, 0], min: -1, max: 1.5 } }),
      node("lower", "upper", [0, 0.5, 0], qx(0.3), [1, 1, 1], { joint: { type: "revolute", axis: [1, 0, 0], min: 0, max: 2.5 } }),
      node("body", "root", [0, 0, 0], ID, [1, 1, 1], { mesh: "arm-mesh", skin: "arm-skin" }),
    ],
    meshes: [
      {
        id: "arm-mesh",
        primitives: [
          { mode: "triangles", material: "skin", positions: b.positions, normals: b.normals, colors, joints, weights, indices: b.side, solids: [{ id: "upper-arm", first: 0, count: b.side.length / 2 }, { id: "lower-arm", first: b.side.length / 2, count: b.side.length / 2 }] },
          { mode: "triangles", material: "cloth", positions: b.positions, normals: capNormals, joints, weights, indices: b.caps, solids: [{ id: "upper-arm", first: 0, count: b.caps.length / 2 }, { id: "lower-arm", first: b.caps.length / 2, count: b.caps.length / 2 }] },
          { mode: "lines", material: "cloth", positions: [0, 1, 0, 0, 2, 0], indices: [0, 1], solids: [{ id: "upper-arm", first: 0, count: 2 }] },
        ],
      },
    ],
    skins: [{ id: "arm-skin", joints: ["upper", "lower"], inverseBind: [inverse(upperWorld), inverse(lowerWorld)] }],
    materials: [material("skin", [0.8, 0.5, 0.4, 1]), material("cloth", [0.1, 0.2, 0.6, 1], { doubleSided: true }), material("skin-hot", [0.9, 0.2, 0.1, 1], { emissive: [0.2, 0, 0.01] }), material("paper", [1, 1, 1, 1], { unlit: true })],
    variants: [{ id: "thermal", title: text("Thermal", "Thermisch"), bindings: { skin: "skin-hot", cloth: "paper" } }],
    animations: [
      {
        id: "wave",
        title: text("Wave", "Winken"),
        durationS: 2,
        loop: true,
        channels: [
          { target: { node: "lower", path: "rotation" }, interpolation: "linear", times: [0, 1, 2], values: [...qx(0.3), ...qx(2), ...qx(0.3)] },
          { target: { node: "root", path: "translation" }, interpolation: "linear", times: [0, 2], values: [0, 0, 0, 0, 0, 0.5] },
          { target: { node: "body", path: "scale" }, interpolation: "step", times: [0, 1], values: [1, 1, 1, 1.1, 1.1, 1.1] },
          { target: { material: "skin-hot", path: "baseColor" }, interpolation: "linear", times: [0, 2], values: [0.9, 0.2, 0.1, 1, 0.2, 0.2, 0.9, 1] },
          { target: { material: "skin", path: "emissive" }, interpolation: "linear", times: [0, 2], values: [0, 0, 0, 0.5, 0.25, 0] },
        ],
        events: [{ t: 1, id: "touch" }],
      },
    ],
    __cases: [{ name: "rest", options: { format: "binary" } }, { name: "wave", options: { animation: "wave", format: "binary" } }, { name: "wave-thermal", options: { animation: "wave", variant: "thermal", format: "binary" } }],
  };
}

function staticCube() {
  const b = box("cube", [-0.5, 0, -0.5], [0.5, 1, 0.5]);
  const colors = b.corners.flatMap((_, i) => [i / 7, 1 - i / 7, 0.5, i % 2 ? 0.5 : 1]);
  return {
    ...base,
    id: "static-cube",
    title: text("Static cube", "Statischer Würfel"),
    nodes: [node("stage", null, [1, 0, 0], qy(0.5), [2, 2, 2]), node("cube", "stage", [0, 0.5, 0], ID, [1, 1, 1], { mesh: "cube-mesh" })],
    meshes: [{ id: "cube-mesh", primitives: [{ mode: "triangles", material: "red", positions: b.positions, colors, indices: b.side, solids: [{ id: "cube-body", first: 0, count: b.side.length }] }, { mode: "triangles", material: "glass", positions: b.positions, normals: b.normals, indices: b.caps }] }],
    materials: [material("red", [1, 0.05, 0.05, 1]), material("glass", [0.7, 0.8, 0.9, 0.4], { alphaMode: "blend", metallic: 1, roughness: 0.1 })],
    __cases: [{ name: "rest", options: { format: "binary" } }],
  };
}

function rotationContinuity() {
  const turns = [0, 2.1, 4.2, 6.3, 8.4];
  return {
    ...base,
    id: "rotation-continuity",
    title: text("Rotation continuity", "Rotationskontinuität"),
    nodes: [
      node("a", null),
      node("slider", "a", [0, 0, 0.1], ID, [1, 1, 1], { joint: { type: "prismatic", axis: [0, 0, 1], min: -0.2, max: 0.4 } }),
      node("diagonal", "a", [1, 0, 0], ID, [1, 1, 1], { joint: { type: "revolute", axis: [1, 1, 0], min: 0, max: 1 } }),
      node("ball", "a", [2, 0, 0], qy(0.4), [1, 1, 1], { joint: { type: "spherical", axis: [0, 1, 0], max: 0.5, twistMin: -1, twistMax: 1 } }),
      node("rigid", "a", [3, 0, 0], ID, [1, 1, 1], { joint: { type: "fixed", axis: [0, 1, 0] } }),
      node("flipped", "a", [4, 0, 0], ID, [1, 1, 1], { joint: { type: "revolute", axis: [0, -1, 0], min: -0.5, max: 2 } }),
      node("spinner", "a", [5, 0, 0], qy(0.2)),
    ],
    animations: [
      {
        id: "spin",
        title: text("Spin", "Drehen"),
        durationS: 4,
        loop: true,
        channels: [
          { target: { node: "spinner", path: "rotation" }, interpolation: "linear", times: turns.map((_, i) => i), values: turns.flatMap(qy) },
          { target: { node: "flipped", path: "rotation" }, interpolation: "step", times: [0, 1, 2], values: [...qy(0), ...qy(3.5), ...qy(7)] },
          { target: { node: "slider", path: "translation" }, interpolation: "linear", times: [0, 4], values: [0, 0, 0.1, 0, 0, 0.3] },
          { target: { node: "spinner", path: "rotation" }, interpolation: "linear", times: [0], values: ID },
        ],
        events: [],
      },
    ],
    __cases: [{ name: "rest", options: { format: "binary" } }, { name: "spin", options: { animation: "spin", format: "binary" } }],
  };
}

function procedural() {
  return { ...base, id: "procedural", title: text("Procedural", "Prozedural"), nodes: [node("root", null)], solids: [{ id: "block", node: "root", material: "grey", kind: "box", size: [1, 1, 1] }], materials: [material("grey", [0.5, 0.5, 0.5, 1])], __cases: [] };
}

const sha = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const operation = { report() {}, cancelled: () => false };
const sources: Record<string, () => any> = { "🧍️rigged-arm": riggedArm, "🧊️static-cube": staticCube, "🎞️rotation-continuity": rotationContinuity, "🚫️procedural": procedural };
for (const [folder, make] of Object.entries(sources)) {
  const { __cases, ...document } = make();
  const cases = __cases.map((c: any) => {
    const result = encodeFbxDetailed(document, c.options, operation);
    const file = readFbx(result.bytes);
    return { name: c.name, options: c.options, expected: { byteLength: result.bytes.length, sha256: sha(result.bytes), objectCounts: objectCounts(file), notices: result.notices.map((n) => n.code + ":" + n.subject), connections: connectionList(file), outline: outline(file) } };
  });
  mkdirSync(root + folder, { recursive: true });
  writeFileSync(root + folder + "/🔣️.json", JSON.stringify(folder.includes("procedural") ? { document, expectedError: "procedural-document" } : { document, cases }, null, 1) + "\n");
}

const rows = [
  { name: "empty", spec: { kind: "repeat", pattern: [], count: 0 } },
  { name: "single-byte", spec: { kind: "repeat", pattern: [7], count: 1 } },
  { name: "zeros-4096", spec: { kind: "repeat", pattern: [0], count: 4096 } },
  { name: "text-repeat", spec: { kind: "repeat", pattern: Array.from(new TextEncoder().encode("semio model fbx ")), count: 300 } },
  { name: "lcg-noise-3000", spec: { kind: "lcg", seed: 12345, length: 3000 } },
  { name: "float-ramp", spec: { kind: "float64-ramp", count: 2000, step: 0.125 } },
];
function bytesOf(spec: any): Uint8Array {
  if (spec.kind === "repeat") return Uint8Array.from({ length: spec.pattern.length * spec.count }, (_, i) => spec.pattern[i % spec.pattern.length]);
  if (spec.kind === "lcg") {
    let s = spec.seed >>> 0;
    return Uint8Array.from({ length: spec.length }, () => ((s = (Math.imul(s, 1664525) + 1013904223) >>> 0), s >>> 24));
  }
  const out = new DataView(new ArrayBuffer(spec.count * 8));
  for (let i = 0; i < spec.count; i++) out.setFloat64(i * 8, i * spec.step, true);
  return new Uint8Array(out.buffer);
}
mkdirSync(root + "🗜️deflate", { recursive: true });
writeFileSync(root + "🗜️deflate/🔣️.json", JSON.stringify({ rows: rows.map((r) => { const input = bytesOf(r.spec); const z = deflateZlib(input); return { ...r, inputSha256: sha(input), inputLength: input.length, zlibLength: z.length, zlibSha256: sha(z) }; }) }, null, 1) + "\n");
console.log("fixtures written");

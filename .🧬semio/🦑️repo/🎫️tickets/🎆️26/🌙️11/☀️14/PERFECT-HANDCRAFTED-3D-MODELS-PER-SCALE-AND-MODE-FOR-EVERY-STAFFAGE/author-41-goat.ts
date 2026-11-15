/** 🐐️ Author 41: handcrafted goat bodies per scale (lofts from the scale's own drawings: barrel trunk with back and belly masses, real pelvis, horns, beard, hooves). Usage: bun author-41-goat.ts <50|100|200|500> [--write] */
import { Zoo, torso2, range, type Look } from "./author-41-lib.ts";
const scale = Number(process.argv[2]);
const write = process.argv.includes("--write");
const S = new Zoo("animals/goat", scale);
const C = (tone: number): Look => ({ material: "short-coat", tone });
const H = (tone: number): Look => ({ material: "horn", tone });
const SK = (tone: number): Look => ({ material: "bare-skin", tone });
const T = ["hindbody", "chest"];
const fl = S.feature;
const BZ = Number(process.env.BZ ?? 0.06);
const ball = (id: string, bone: string, c: [number, number, number], r: [number, number, number], look: Look) => S.loft(id, bone, [[-0.85, 0.7], [0, 1], [0.85, 0.7]].map(([dy, f]) => ({ c: [c[0], c[1] + dy! * r[1], c[2]] as [number, number, number], a: [0, 0, r[2] * f!] as [number, number, number], b: [r[0] * f!, 0, 0] as [number, number, number], e: 2 })), look);
const WF = Number(process.env.WF ?? 1.0);
const sides = [["f", "far", 0.55], ["n", "near", 0.72]] as const;

if (scale === 50 || scale === 100) {
  const fine = scale === 50;
  const k = (a: number, b: number) => Math.max(3, Math.round(fine ? a : b));
  S.hullX("rump", "pelvis", { elev: T, plan: T, xs: range(-0.122, 0.1, k(8, 5)), look: C(0.72), e: 2.3 });
  torso2(S, "flank", "lumbar", { ids: T, xs: range(0.1, 0.4, k(9, 6)), look: C(0.72), bellyW: 0.86 });
  torso2(S, "ribs", "thorax", { ids: T, xs: [...range(0.4, 0.74, k(8, 5)), 0.765, 0.785, 0.8, 0.81], look: C(0.72), bellyW: 0.84 });
  S.limb("neck", "neck", { elev: ["neck"], plan: ["neck"], n: k(8, 5), look: C(0.72), e: 2.2 });
  S.hullX("head", "head", { elev: ["head"], plan: ["head"], xs: range(0.94, 1.178, k(11, 7)), look: C(0.72), e: 2.2 });
  if (fine) S.hullX("jaw", "jaw", { elev: ["jaw"], plan: ["jaw"], xs: range(0.994, 1.112, k(6, 4)), look: C(0.66), e: 2.2 });
  if (fine) S.hullY("beard", "jaw", { elev: ["beard"], front: ["beard"], ys: range(0.708, 0.78, 5), look: C(0.9), e: 2.2 });
  for (const [id, bone, sg, tone] of [["ear-n", "ear-n", -1, 0.6], ["ear-f", "ear-f", 1, 0.5]] as const) {
    const ec = S.rings("elevation", [id])[0]!;
    const cy = 0.908; void ec;
    S.ell(id, bone, [1.1121, cy, sg * 0.104], [0.074, 0.014, 0.027], SK(tone), [[0.099, 0.025, sg * 0.14], [0, 1, 0]]);
  }
  for (const [id, tone, sg] of [["horn-n", 0.72, -1], ["horn-f", 0.55, 1]] as const) S.strip(id, "head", { shape: id, from: [id === "horn-n" ? 0.998 : 1.0, 0.93], n: fine ? 10 : 6, z0: sg * 0.04, z1: sg * 0.075, r0: 0.014, r1: 0.01, look: H(tone), e: 2.2 });
  S.limb("tail-1", "tail-1", { elev: ["tail-root"], plan: ["tail-root"], n: 4, look: C(0.66), e: 2.1, round: true });
  S.limb("tail-2", "tail-2", { elev: ["tail-tip"], plan: ["tail-root"], n: 3, look: C(0.66), e: 2.1, round: true });
  for (const [sd, nm, tone] of sides) {
    const n = (a: number, b: number) => Math.max(3, Math.round(fine ? a : b));
    S.limb(`${nm}-thigh`, `thigh-h${sd}`, { elev: ["thigh-n"], plan: [`${nm}-thigh`], n: n(8, 5), look: C(tone), e: 2.1 });
    S.limb(`${nm}-gaskin`, `shank-h${sd}`, { elev: [`gaskin-${sd === "n" ? "n" : "f"}`], plan: [`${nm}-gaskin`], n: n(8, 5), look: C(tone), e: 2.1 });
    S.limb(`${nm}-cannon-h`, `metatarsus-h${sd}`, { elev: [`cannon-h-${sd}`], plan: [`${nm}-cannon-h`], n: n(7, 4), look: C(tone), e: 2.1 });
    S.limb(`${nm}-hoof-h`, `hoof-h${sd}`, { elev: [`hoof-h-${sd}`], plan: [`${nm}-hoof-h`], n: n(5, 4), look: H(tone), e: 2.2 });
    S.limb(`${nm}-shoulder`, `scapula-f${sd}`, { elev: ["shoulder-n"], plan: [`${nm}-shoulder`], s0: 0.0, s1: 0.18, n: n(7, 5), look: C(tone), e: 2.1 });
    S.limb(`${nm}-humerus`, `humerus-f${sd}`, { elev: ["shoulder-n"], plan: [`${nm}-humerus`], s0: 0.0, s1: 0.17, n: n(6, 4), look: C(tone), e: 2.1 });
    S.limb(`${nm}-forearm`, `forearm-f${sd}`, { elev: [`forearm-${sd}`], plan: [`${nm}-forearm`], n: n(8, 5), look: C(tone), e: 2.1 });
    S.limb(`${nm}-cannon-f`, `metacarpus-f${sd}`, { elev: [`cannon-f-${sd}`], plan: [`${nm}-cannon-f`], n: n(7, 4), look: C(tone), e: 2.1 });
    S.limb(`${nm}-hoof-f`, `hoof-f${sd}`, { elev: [`hoof-f-${sd}`], plan: [`${nm}-hoof-f`], n: n(5, 4), look: H(tone), e: 2.2 });
  }
}

if (scale === 200) {
  S.hullX("hindbody", "lumbar", { elev: T, plan: T, xs: range(-0.12, 0.4, 9), look: C(0.72), e: 2.4 });
  S.hullX("chest", "thorax", { elev: T, plan: T, xs: [...range(0.4, 0.74, 6), 0.765, 0.785, 0.8, 0.81], look: C(0.72), e: 2.4 });
  S.limb("neck", "neck", { elev: ["neck"], plan: ["neck"], n: 5, look: C(0.72), e: 2.2 });
  S.hullX("head", "head", { elev: ["head"], plan: ["head"], xs: range(0.94, 1.178, 7), look: C(0.72), e: 2.2 });
  S.ell("ear", "ear-n", [1.1121, 0.908, -0.09], [0.062, 0.0165, 0.027], SK(0.6), [[0.099, 0.025, -0.09], [0, 1, 0]]);
  S.strip("horn", "head", { shape: "horn", from: [0.998, 0.93], n: 6, z0: -0.04, z1: -0.075, r0: 0.0165, r1: 0.0165, look: H(0.9), e: 2.2, cut: 1, taper: 0.9 });
  S.tube("tail", "tail-1", [-0.03, 0.708, 0], [0.024, 0.826, 0], 0.0215, 0.0165, C(0.66));
  for (const [sd, nm, tone] of sides) {
    const e = sd;
    S.limb(`${nm}-thigh`, `thigh-h${sd}`, { elev: ["thigh-n"], plan: [`${nm}-thigh`], n: 6, look: C(tone), e: 2.1 });
    S.limb(`${nm}-gaskin`, `shank-h${sd}`, { elev: [`gaskin-${e}`], plan: [`${nm}-gaskin`], n: 6, look: C(tone), e: 2.1 });
    S.limb(`${nm}-cannon-h`, `metatarsus-h${sd}`, { elev: [`cannon-h-${e}`], plan: [`${nm}-hind-lower`], n: 5, look: C(tone), e: 2.1 });
    S.limb(`${nm}-hoof-h`, `hoof-h${sd}`, { elev: [`hoof-h-${e}`], plan: [`${nm}-hind-lower`], n: 3, look: H(tone), e: 2.2, floor: 0.0125 });
    const hf = S.frame(`humerus-f${sd}`);
    S.tube(`${nm}-humerus`, `humerus-f${sd}`, [hf.o[0]!, hf.o[1]!, sd === "n" ? -0.09 : 0.09], [hf.o[0]! + hf.u[0]! * 0.18, hf.o[1]! + hf.u[1]! * 0.18, sd === "n" ? -0.09 : 0.09], 0.05, 0.04, C(tone));
    S.limb(`${nm}-forearm`, `forearm-f${sd}`, { elev: [`forearm-${e}`], plan: [`${nm}-forearm`], n: 6, look: C(tone), e: 2.1 });
    S.limb(`${nm}-cannon-f`, `metacarpus-f${sd}`, { elev: [`cannon-f-${e}`], plan: [`${nm}-fore-lower`], n: 5, look: C(tone), e: 2.1 });
    S.limb(`${nm}-hoof-f`, `hoof-f${sd}`, { elev: [`hoof-f-${e}`], plan: [`${nm}-fore-lower`], n: 3, look: H(tone), e: 2.2, floor: 0.0125 });
  }
}

if (scale === 500) {
  S.hullX("hindbody", "lumbar", { elev: T, plan: T, xs: range(-0.118, 0.4, 8), look: C(0.72), e: 2.4, half: (_x, h) => h * WF });
  S.hullX("chest", "thorax", { elev: T, plan: T, xs: [0.4, 0.5, 0.6, 0.68, 0.74, 0.78, 0.81], look: C(0.72), e: 3.0, half: (_x, h) => h * WF });
  S.limb("neck", "neck", { elev: ["neck"], plan: ["neck"], n: 3, look: C(0.72), e: 2.2 });
  S.hullX("head", "head", { elev: ["head"], plan: ["head"], xs: range(0.94, 1.178, 4), look: C(0.72), e: 2.2 });
  for (const [sd, nm, tone] of sides) {
    S.limb(`${nm}-leg-hind-upper`, `shank-h${sd}`, { elev: ["leg-hind-upper-n"], plan: [`${nm}-leg-hind-upper`], n: 4, look: C(tone), e: 2.1, floor: 0.0377 });
    S.limb(`${nm}-leg-hind-lower`, `metatarsus-h${sd}`, { elev: ["leg-hind-lower-n"], plan: [`${nm}-leg-hind-lower`], n: 4, look: C(tone), e: 2.1, floor: 0.0377, trim: 0.008 });
    S.limb(`${nm}-leg-fore-upper`, `forearm-f${sd}`, { elev: ["leg-fore-upper-n"], plan: [`${nm}-leg-fore-upper`], n: 4, look: C(tone), e: 2.1, floor: 0.0377 });
    S.limb(`${nm}-leg-fore-lower`, `metacarpus-f${sd}`, { elev: ["leg-fore-lower-n"], plan: [`${nm}-leg-fore-lower`], n: 4, look: C(tone), e: 2.1, floor: 0.0377, trim: 0.008 });
    const z = sd === "n" ? -0.09 : 0.09;
    ball(`${nm}-haunch`, `shank-h${sd}`, [0.0145, 0.55, z], [0.07, 0.1, BZ], C(tone));
    ball(`${nm}-shoulder`, `forearm-f${sd}`, [0.66, 0.5, z], [0.06, 0.075, BZ], C(tone));
  }
  S.tube("horn", "head", [0.99, 0.93, -0.04], [0.945, 1.11, -0.075], 0.05, 0.05, H(0.9));
  S.add({ id: "ear", kind: "box", bone: "ear-n", material: "bare-skin", role: "body", tone: 0.6, size: [0.08, 0.08, 0.41], transform: { translation: S.local("ear-n", [1.0795, 0.913, 0]) } });
}

if (process.argv.includes("--extra") && scale === 500) {
  const q = (id: string, bone: string, at: [number, number, number], r: [number, number, number], look: Look) => S.ell(id, bone, at, r, look);
  const th: Record<string, string[]> = {};
  if (scale === 500) {
    q("t-tail", "tail-1", [-0.005, 0.76, 0], [0.0376, 0.0376, 0.0376], C(0.66)); th.tail = ["t-tail"];
    for (const [id, bone, x] of [["t-hoof-h", "hoof-hn", 0.075], ["t-hoof-f", "hoof-fn", 0.71]] as const) S.add({ id, kind: "box", bone, material: "horn", role: "body", tone: 0.9, size: [0.0752, 0.0752, 0.2], transform: { translation: S.local(bone, [x, 0.0376, 0]) } });
    th.hooves = ["t-hoof-h", "t-hoof-f"];
  }
  S.takeThermal(th);
  process.exit(0);
}

console.log(`goat 1:${scale}: ${S.body.length} body solids`);
if (write) S.save();
void fl;

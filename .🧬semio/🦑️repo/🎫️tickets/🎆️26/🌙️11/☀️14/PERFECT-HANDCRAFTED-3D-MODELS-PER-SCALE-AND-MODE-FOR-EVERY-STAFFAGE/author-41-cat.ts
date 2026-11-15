/** 🐈️ Author 41: handcrafted domestic cat bodies per scale (lofts from the scale's own repaired drawings, real pelvis/rib cage/haunch masses, jaw rami around the tongue). Usage: bun author-41-cat.ts <50|100|200> [--write] */
import { Zoo, torso2, range, type Look } from "./author-41-lib.ts";
const scale = Number(process.argv[2]);
const write = process.argv.includes("--write");
const S = new Zoo("animals/domestic-cat", scale);
const F = (tone: number): Look => ({ material: "short-fur", tone });
const SKIN: Look = { material: "bare-skin", tone: 0.1, role: "detail" };
const T = ["hindbody", "midbody", "chest"];
const TB = ["tail-1", "tail-2", "tail-3", "tail-4", "tail-5"];
const TL = [0.06, 0.06, 0.06, 0.055, 0.045];
const TR = [0.02, 0.019, 0.018, 0.017, 0.016, 0.012].map((r) => Math.max(r, 0.55 * S.feature));
const trunkX = (id: string, bone: string, a: number, b: number, n: number, tone: number, e = 2.3, ids = T) => S.hullX(id, bone, { elev: ids, plan: ids, xs: range(a, b, n), look: F(tone), e });

if (scale === 50) {
  trunkX("rump", "pelvis", -0.053, 0.05, 8, 0.72);
  torso2(S, "flank", "lumbar", { ids: T, xs: range(0.05, 0.14, 6), look: F(0.72) });
  torso2(S, "ribs", "thorax-a", { ids: T, xs: range(0.14, 0.235, 7), look: F(0.72), bellyW: 0.8 });
  torso2(S, "chest", "thorax-b", { ids: T, xs: range(0.235, 0.34, 6), look: F(0.72), bellyW: 0.8 });
  S.limb("neck", "neck", { elev: ["neck"], plan: ["neck"], n: 7, look: F(0.72), e: 2.2 });
  S.hullX("head", "head", { elev: ["head"], plan: ["head"], xs: range(0.355, 0.467, 12), look: F(0.72), e: 2.2 });
  S.hullX("jaw-n", "jaw", { elev: ["jaw"], xs: range(0.383, 0.457, 5), look: F(0.72), e: 2.2, zc: -0.0185, half: () => 0.0075, floor: 0.0075 });
  S.hullX("jaw-f", "jaw", { elev: ["jaw"], xs: range(0.383, 0.457, 5), look: F(0.72), e: 2.2, zc: 0.0185, half: () => 0.0075, floor: 0.0075 });
  S.hullX("tongue", "jaw", { elev: ["tongue"], xs: range(0.408, 0.446, 5), look: { material: "tongue-mucosa", tone: 0.42 }, e: 2.2, zc: 0, half: () => 0.0072, floor: 0.0075 });
  S.ell("nose-leather", "head", [0.4655, 0.279, 0], [0.0075, 0.0075, 0.0095], { material: "bare-skin", tone: 0.08 });
  const hs = S.span("plan", ["head"], "x", 0.4315)!;
  for (const [id, z] of [["eye-a", hs[0] + 0.001], ["eye-b", hs[1] - 0.001]] as const) S.ell(id, "head", [0.4315, 0.2925, z], [0.0055, 0.0063, 0.005], { material: "cornea", tone: 0.04, role: "detail" });
  for (const [id, bone, tone] of [["near-ear", "ear-n", 0.6], ["far-ear", "ear-f", 0.5]] as const) S.limb(id, bone, { elev: [id], plan: [id], front: [id], n: 6, look: F(tone), e: 2.2, floor: 0.0055 });
  S.chain("tail", TB, TL, TR, F(0.66));
  for (const [sd, nm, tone] of [["f", "far", 0.55], ["n", "near", 0.72]] as const) {
    S.limb(`${nm}-thigh`, `thigh-h${sd}`, { elev: [`${nm}-thigh`], plan: [`${nm}-thigh`], n: 8, look: F(tone), e: 2.1 });
    S.limb(`${nm}-shank`, `shank-h${sd}`, { elev: [`${nm}-shank`], plan: [`${nm}-shank`], n: 7, look: F(tone), e: 2.1 });
    S.limb(`${nm}-hock`, `metatarsus-h${sd}`, { elev: [`${nm}-hock`], plan: [`${nm}-hock`], n: 6, look: F(tone), e: 2.1 });
    S.limb(`${nm}-hind-paw`, `paw-h${sd}`, { elev: [`${nm}-hind-paw`], plan: [`${nm}-hind-paw`], front: [`${nm}-hind-paw`], n: 6, look: F(tone), e: 2.2, cv: (_s, c, h) => c + 0.35 * h, hv: (_s, h) => 0.65 * h });
    S.limb(`${nm}-scapula`, `scapula-f${sd}`, { elev: [`${nm}-scapula`], plan: [`${nm}-scapula`], n: 7, look: F(tone), e: 2.1 });
    S.limb(`${nm}-humerus`, `humerus-f${sd}`, { elev: [`${nm}-humerus`], plan: [`${nm}-humerus`], n: 7, look: F(tone), e: 2.1 });
    S.limb(`${nm}-forearm`, `forearm-f${sd}`, { elev: [`${nm}-forearm`], plan: [`${nm}-forearm`], n: 7, look: F(tone), e: 2.1 });
    S.limb(`${nm}-wrist`, `metacarpus-f${sd}`, { elev: [`${nm}-wrist`], plan: [`${nm}-wrist`], n: 5, look: F(tone), e: 2.1 });
    S.limb(`${nm}-fore-paw`, `paw-f${sd}`, { elev: [`${nm}-fore-paw`], plan: [`${nm}-fore-paw`], front: [`${nm}-fore-paw`], n: 6, look: F(tone), e: 2.2, cv: (_s, c, h) => c + 0.35 * h, hv: (_s, h) => 0.65 * h });
    for (const [pn, pawPlan, pawBone, x0, x1] of [["hind", `${nm}-hind-paw`, `paw-h${sd}`, -0.037, 0.026], ["fore", `${nm}-fore-paw`, `paw-f${sd}`, 0.245, 0.308]] as const) {
      const pl = S.span("plan", [pawPlan], "x", (x0 + x1) / 2)!;
      const zc = (pl[0] + pl[1]) / 2;
      S.ell(`${nm}-${pn}-pad`, pawBone, [(x0 + x1) / 2, 0.0102, zc], [(x1 - x0) / 2 * 0.97, 0.0082, Math.min(0.0104, (pl[1] - pl[0]) / 2)], SKIN);
    }
  }
}

if (scale === 100) {
  trunkX("rump", "pelvis", -0.053, 0.05, 6, 0.72);
  torso2(S, "flank", "lumbar", { ids: T, xs: range(0.05, 0.14, 4), look: F(0.72) });
  torso2(S, "ribs", "thorax-a", { ids: T, xs: range(0.14, 0.235, 5), look: F(0.72), bellyW: 0.8 });
  trunkX("chest", "thorax-b", 0.235, 0.372, 6, 0.72, 2.4);
  S.limb("neck", "neck", { elev: ["neck"], plan: ["neck"], n: 5, look: F(0.72), e: 2.2 });
  S.hullX("head", "head", { elev: ["head"], plan: ["head"], xs: range(0.355, 0.467, 8), look: F(0.72), e: 2.2 });
  S.hullX("jaw-n", "jaw", { elev: ["jaw"], xs: range(0.383, 0.457, 4), look: F(0.72), e: 2.2, zc: -0.0185, half: () => 0.0105, floor: 0.0075 });
  S.hullX("jaw-f", "jaw", { elev: ["jaw"], xs: range(0.383, 0.457, 4), look: F(0.72), e: 2.2, zc: 0.0185, half: () => 0.0105, floor: 0.0075 });
  S.hullX("tongue", "jaw", { elev: ["tongue"], xs: range(0.405, 0.447, 4), look: { material: "tongue-mucosa", tone: 0.42 }, e: 2.2, zc: 0, half: () => 0.0085, floor: 0.0075 });
  S.ell("nose-leather", "head", [0.4655, 0.279, 0], [0.0095, 0.0095, 0.0105], { material: "bare-skin", tone: 0.08 });
  const hs = S.span("plan", ["head"], "x", 0.4315)!;
  for (const [id, z] of [["eye-a", hs[0] + 0.001], ["eye-b", hs[1] - 0.001]] as const) S.ell(id, "head", [0.4315, 0.2925, z], [0.0075, 0.0075, 0.0075], { material: "cornea", tone: 0.04, role: "detail" });
  for (const [id, bone, tone] of [["near-ear", "ear-n", 0.6], ["far-ear", "ear-f", 0.5]] as const) S.limb(id, bone, { elev: [id], plan: [id], front: [id], n: 4, look: F(tone), e: 2.2, floor: 0.0085 });
  S.chain("tail", TB, TL, TR, F(0.66));
  for (const [sd, nm, tone] of [["f", "far", 0.55], ["n", "near", 0.72]] as const) {
    S.limb(`${nm}-thigh`, `thigh-h${sd}`, { elev: [`${nm}-thigh`], plan: [`${nm}-thigh`], n: 6, look: F(tone), e: 2.1 });
    S.limb(`${nm}-shank`, `shank-h${sd}`, { elev: [`${nm}-shank`], plan: [`${nm}-shank`], n: 5, look: F(tone), e: 2.1 });
    S.limb(`${nm}-hock`, `metatarsus-h${sd}`, { elev: [`${nm}-hock`], plan: [`${nm}-hock`], n: 4, look: F(tone), e: 2.1 });
    S.limb(`${nm}-scapula`, `scapula-f${sd}`, { elev: [`${nm}-scapula`], plan: [`${nm}-scapula`], n: 5, look: F(tone), e: 2.1 });
    S.limb(`${nm}-humerus`, `humerus-f${sd}`, { elev: [`${nm}-humerus`], plan: [`${nm}-humerus`], n: 5, look: F(tone), e: 2.1 });
    S.limb(`${nm}-forearm`, `forearm-f${sd}`, { elev: [`${nm}-forearm`], plan: [`${nm}-forearm`], n: 5, look: F(tone), e: 2.1 });
    S.limb(`${nm}-wrist`, `metacarpus-f${sd}`, { elev: [`${nm}-wrist`], plan: [`${nm}-wrist`], n: 3, look: F(tone), e: 2.1, floor: 0.0085 });
    for (const [pn, pawBone, x0, x1] of [["hind", `paw-h${sd}`, -0.037, 0.026], ["fore", `paw-f${sd}`, 0.245, 0.308]] as const) {
      const pawId = `${nm}-${pn}-paw`;
      S.limb(pawId, pawBone, { elev: [pawId], plan: [pawId], front: [pawId], n: 4, look: F(tone), e: 2.2, cv: (_s, c, h) => c + 0.35 * h, hv: (_s, h) => Math.max(0.0085, 0.65 * h) });
      const pl = S.span("plan", [pawId], "x", (x0 + x1) / 2)!;
      const fl = S.span("front", [pawId], "y", 0.02);
      S.ell(`${nm}-${pn}-pad`, pawBone, [(x0 + x1) / 2, 0.0105, (pl[0] + pl[1]) / 2], [(x1 - x0) / 2 * 0.97, 0.0085, Math.max(0.0078, Math.min((pl[1] - pl[0]) / 2, fl === null ? 1 : (fl[1] - fl[0]) / 2))], SKIN);
    }
  }
}

if (scale === 200) {
  S.hullX("hindbody", "lumbar", { elev: ["hindbody", "chest"], plan: ["hindbody", "chest"], xs: range(-0.053, 0.215, 9), look: F(0.72), e: 2.4 });
  S.hullX("chest", "thorax-b", { elev: ["hindbody", "chest"], plan: ["hindbody", "chest"], xs: range(0.215, 0.37, 5), look: F(0.72), e: 2.4 });
  S.limb("neck", "neck", { elev: ["neck"], plan: ["neck"], n: 4, look: F(0.72), e: 2.2 });
  S.hullX("head", "head", { elev: ["head"], plan: ["head"], xs: range(0.355, 0.467, 6), look: F(0.72), e: 2.2 });
  S.hullX("jaw", "jaw", { elev: ["jaw"], plan: ["jaw"], xs: range(0.383, 0.457, 4), look: F(0.72), e: 2.2, floor: 0.0165 });
  for (const [id, bone, tone] of [["near-ear", "ear-n", 0.6], ["far-ear", "ear-f", 0.5]] as const) S.limb(id, bone, { elev: [id], plan: [id], n: 3, look: F(tone), e: 2.1, floor: 0.0165 });
  S.chain("tail", TB, TL, [0.0198, 0.0185, 0.0175, 0.0165, 0.0158, 0.0155], F(0.66));
  for (const [sd, p, nm, tone] of [["f", "f", "far", 0.55], ["n", "n", "near", 0.72]] as const) {
    S.limb(`${p}-hind-thigh`, `thigh-h${sd}`, { elev: [`${p}-hind-thigh`], plan: [`${nm}-hind-thigh`], n: 5, look: F(tone), e: 2.1 });
    S.limb(`${p}-hind-lower`, `shank-h${sd}`, { elev: [`${p}-hind-lower`], plan: [`${nm}-hind-lower`], n: 4, look: F(tone), e: 2.1, floor: 0.0165 });
    S.limb(`${p}-hind-foot`, `metatarsus-h${sd}`, { elev: [`${p}-hind-foot`], plan: [`${nm}-hind-foot`], n: 4, look: F(tone), e: 2.1, floor: 0.0165 });
    S.limb(`${p}-fore-upper`, `humerus-f${sd}`, { elev: [`${p}-fore-upper`], plan: [`${nm}-fore-upper`], n: 5, look: F(tone), e: 2.1 });
    S.limb(`${p}-fore-lower`, `forearm-f${sd}`, { elev: [`${p}-fore-lower`], plan: [`${nm}-fore-lower`], n: 4, look: F(tone), e: 2.1, floor: 0.0165 });
    S.limb(`${p}-fore-foot`, `metacarpus-f${sd}`, { elev: [`${p}-fore-foot`], plan: [`${nm}-fore-foot`], n: 4, look: F(tone), e: 2.1, floor: 0.0165 });
  }
}

if (process.argv.includes("--extra") && scale === 200) {
  const SK = (id: string, bone: string, at: [number, number, number], r: [number, number, number], mat: string, tone: number) => S.ell(id, bone, at, r, { material: mat, tone });
  SK("t-nose", "head", [0.4445, 0.279, 0], [0.0165, 0.0165, 0.0165], "bare-skin", 0.08);
  SK("t-eye-a", "head", [0.4315, 0.2925, -0.0235], [0.0165, 0.0165, 0.0165], "cornea", 0.04);
  SK("t-eye-b", "head", [0.4315, 0.2925, 0.0235], [0.0165, 0.0165, 0.0165], "cornea", 0.04);
  SK("t-tongue", "jaw", [0.43, 0.237, 0], [0.0215, 0.0165, 0.0165], "tongue-mucosa", 0.42);
  const th = { nose: ["t-nose"], eyes: ["t-eye-a", "t-eye-b"], tongue: ["t-tongue"] } as Record<string, string[]>;
  const pads: string[] = [];
  for (const [nm, bone, x] of [["fh", "paw-hf", 0.006], ["fn", "paw-ff", 0.287], ["nh", "paw-hn", 0.006], ["nn", "paw-fn", 0.287]] as const) {
    const near = nm[0] === "n";
    SK(`t-pad-${nm}`, bone, [x, 0.0165, near ? -0.0345 : 0.0345], [0.027, 0.0165, 0.0185], "bare-skin", 0.12);
    pads.push(`t-pad-${nm}`);
  }
  th.pads = pads;
  th["nose-skin"] = th.nose!;
  delete th.nose;
  S.takeThermal(th);
  process.exit(0);
}
console.log(`cat 1:${scale}: ${S.body.length} body solids`);
if (write) S.save();

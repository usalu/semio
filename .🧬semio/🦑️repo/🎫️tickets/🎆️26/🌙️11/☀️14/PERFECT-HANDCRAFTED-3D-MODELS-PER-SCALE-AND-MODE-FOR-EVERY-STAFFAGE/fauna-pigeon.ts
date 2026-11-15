/** 🕊️ Fauna: rock pigeon (Columba livia) bodies per scale from the fauna templates: breast-forward barrel, three-part neck, small round head with cere and slender bill, eyes with orbital ring, rectrix wedge tail with undertail coverts, folded wings as coverts lens plus tapered primaries (and shoulder, tertials at 1:50), feathered thigh, bare shank and tarsus, ankle and a toed foot with hallux. Usage: bun fauna-pigeon.ts <50|100|200> [--write] [--extra]
 * @see ./fauna-lib.ts */
import { Fauna, range, type Look } from "./fauna-lib.ts";
const scale = Number(process.argv[2]);
const write = process.argv.includes("--write");
const S = new Fauna("animals/pigeon", scale);
const lod = (scale === 50 ? 2 : scale === 100 ? 1 : 0) as 0 | 1 | 2;
const P = (tone: number): Look => ({ material: "plumage", tone });
const SKIN = (tone: number): Look => ({ material: "bare-skin", tone });
const BK: Look = { material: "beak-keratin", tone: 0.3 };
const k = (a: number, b: number, c: number) => (scale === 50 ? a : scale === 100 ? b : c);

if (process.argv.includes("--extra")) {
  if (scale === 200) {
    S.ell("t-beak", "head", [0.136, 0.2235, 0], [0.016, 0.015, 0.015], BK);
    S.ell("t-cere", "head", [0.126, 0.2285, 0], [0.015, 0.015, 0.015], { material: "cere-wax", tone: 0.9 });
    S.ell("t-eye", "head", [0.1185, 0.2245, -0.0225], [0.015, 0.015, 0.015], { material: "eye-tissue", tone: 0.1 });
    S.ell("t-eye-f", "head", [0.1185, 0.2245, 0.0225], [0.015, 0.015, 0.015], { material: "eye-tissue", tone: 0.1 });
    S.takeThermal({ beak: ["t-beak"], cere: ["t-cere"], eye: ["t-eye", "t-eye-f"] });
  }
  process.exit(0);
}

S.hullX("body", "trunk", { elev: ["body"], plan: ["body"], xs: range(-0.088, 0.147, k(14, 10, 8)), look: P(0.72), e: 2.2, half: (_x, h) => (h > 0.04 ? h + 0.007 : h) });
if (scale === 200) S.hullY("neck", "neck-2", { elev: ["neck"], front: ["neck"], ys: range(0.104, 0.245, 6), look: P(0.72) });
else {
  S.hullY("neck-1", "neck-1", { elev: ["neck-1"], front: ["neck-1"], ys: range(scale === 50 ? 0.139 : 0.103, 0.213, k(7, 5, 5)), look: P(0.72), e: 2.2 });
  S.hullY("neck-2", "neck-2", { elev: ["neck-2"], front: ["neck-2"], ys: range(0.179, 0.227, 5), look: P(0.74), e: 2.2 });
  S.hullY("neck-3", "neck-3", { elev: ["neck-3"], front: ["neck-3"], ys: range(0.198, 0.243, 5), look: P(0.76), e: 2.2 });
}
S.hullX("head", "head", { elev: ["head"], plan: ["head"], xs: range(scale === 200 ? 0.025 : 0.071, scale === 200 ? 0.151 : 0.132, k(9, 6, 7)), look: P(0.72), e: 2.2 });
if (scale !== 200) {
  S.bill("beak", "head", { elev: ["beak"], plan: ["beak"], x0: 0.126, x1: 0.1532, look: BK, n: k(6, 4, 4), gape: () => (scale === 50 ? 0.2195 : 0.2195), lowerW: 0.8 });
  S.ell("cere", "head", [scale === 50 ? 0.1355 : 0.1335, scale === 50 ? 0.2245 : 0.2235, 0], [0.0105, 0.0105, 0.0095], { material: "cere-wax", tone: 0.9 });
  S.hullX("jaw", "jaw", { elev: ["jaw"], xs: range(scale === 50 ? 0.123 : 0.105, 0.146, 4), look: BK, zc: 0, half: () => (scale === 50 ? 0.0042 : 0.0075) });
  for (const sd of [-1, 1] as const) {
    if (lod === 2) S.eye(`eye${sd < 0 ? "" : "-f"}`, "head", [0.1185, 0.2245, sd * 0.0215], 0.0048, sd, { material: "plumage", tone: 0.35 }, { material: "eye-tissue", tone: 0.1 });
    else S.ell(`eye${sd < 0 ? "" : "-f"}`, "head", [0.1185, 0.2245, sd * 0.0205], [0.0076, 0.0076, 0.0076], { material: "eye-tissue", tone: 0.1 });
  }
}
if (scale === 200) {
  const tl = range(-0.176, -0.05, 6);
  S.hullX("tail-f", "tail-f", { elev: ["tail", "far-tail"], plan: ["tail", "far-tail"], xs: tl, look: P(0.55), e: 2.4, vfix: (_x, lo, hi) => [lo, lo + 0.6 * (hi - lo)] });
  S.hullX("tail", "tail", { elev: ["tail", "far-tail"], plan: ["tail"], xs: tl, look: P(0.66), e: 2.4, vfix: (_x, lo, hi) => [hi - 0.6 * (hi - lo), hi] });
} else {
  S.tail("tail", "tail", { elev: ["tail"], plan: ["tail"], x0: -0.06, x1: -0.1785, look: P(0.66), thick: 0.018, n: k(8, 6, 5), coverts: lod === 2 ? P(0.7) : undefined });
  S.tail("tail-f", "tail-f", { elev: ["far-tail"], plan: ["far-tail"], x0: -0.06, x1: -0.1785, look: P(0.5), thick: 0.018, n: k(8, 6, 5) });
}
for (const [sd, nm, id, tone] of [[-1, "near", "n", 0.72], [1, "far", "f", 0.55]] as const) {
  const elev = scale === 200 ? (sd < 0 ? ["near-primaries", "near-coverts", "near-shoulder"] : ["far-wing"]) : [`${nm}-primaries`, `${nm}-coverts`, ...(scale === 100 ? [`${nm}-shoulder`] : [])];
  if (scale === 200 && sd > 0) {
    S.hullX("far-wing", "hand-f", { elev: ["far-wing"], plan: ["far-wing"], xs: range(-0.124, 0.1, 9), look: P(0.5), e: 2.3 });
    continue;
  }
  if (scale === 100) S.hullX(`${nm}-shoulder`, `humerus-${id}`, { elev: [`${nm}-shoulder`], plan: [`${nm}-coverts`, `${nm}-primaries`], xs: range(sd < 0 ? -0.051 : -0.024, sd < 0 ? 0.062 : 0.048, 6), look: P(tone - 0.1), e: 2.3 });
  if (scale === 200 && sd < 0) S.hullX("near-shoulder", "humerus-n", { elev: ["near-shoulder"], plan: ["near-shoulder"], xs: range(-0.071, 0.072, 7), look: P(0.6), e: 2.3 });
  if (scale === 200 && sd < 0) {
    S.hullX("near-primaries", "hand-n", { elev: ["near-primaries"], plan: ["near-primaries"], xs: range(-0.124, 0.103, 10), look: P(0.55), e: 2.3 });
    S.hullX("near-coverts", "forearm-n", { elev: ["near-coverts"], plan: ["near-coverts"], xs: range(-0.086, 0.076, 8), look: P(0.6), e: 2.3 });
    continue;
  }
  S.wing(sd, { elev: elev.filter((q) => !q.endsWith("shoulder")), x0: k(0.078, 0.082, 0.078), xj: 0.0, x1: -0.126, xc: k(-0.02, -0.043, -0.086), xp: k(0.045, 0.139, 0.103), zc: 0.06, zh: 0.0625, th0: 0.0145, thh: 0.011, th1: 0.005, bones: [`forearm-${id}`, `hand-${id}`], id: `${nm}-wing`, lod, look: P(tone - 0.1), tip: P(tone - 0.25), roll: 0, ec: 2.5, ...(scale === 50 && sd > 0 ? { elevC: ["near-coverts"], vfixC: [0.086, 0.184] as [number, number] } : {}) });
}
if (lod === 2) for (const [nm, id, tone] of [["near", "n", 0.72], ["far", "f", 0.55]] as const) {
  void tone;
  void id;
  void nm;
}
if (scale !== 200) for (const [sd, name, z, tone] of [[-1, "near", scale === 50 ? -0.026 : -0.0255, 0.72], [1, "far", scale === 50 ? 0.026 : 0.0255, 0.55]] as const) {
  const id = sd < 0 ? "n" : "f";
  const sh = S.c.state.frames[`shank-${id}`] as any;
  const tars = S.c.state.frames[`tarsus-${id}`] as any;
  const toe = S.c.state.frames[`toes-${id}`] as any;
  S.hullY(`${name}-thigh`, `thigh-${id}`, { elev: [`${name}-thigh`], front: [`${name}-thigh`], ys: range(scale === 50 ? 0.044 : 0.064, 0.124, k(7, 5, 5)), look: P(tone), zc: z });
  if (scale === 50) {
    S.cap(`${name}-shank`, `shank-${id}`, [sh.o[0], sh.o[1], z], [tars.o[0], tars.o[1], z], 0.0062, 0.005, SKIN(tone));
    S.cap(`${name}-tarsus`, `tarsus-${id}`, [tars.o[0], tars.o[1], z], [toe.o[0], toe.o[1] + 0.003, z], 0.0052, 0.0045, SKIN(tone));
    S.foot(`${name}-foot`, `toes-${id}`, { at: [toe.o[0], 0.0042, z], sd, len: 0.042, spread: 0.62, thick: 0.0082, toeW: 0.0082, hind: 0.021, look: SKIN(tone), lod: 2 });
    S.ell(`${name}-ankle`, `tarsus-${id}`, [tars.o[0] + 0.004, tars.o[1] + 0.001, z], [0.011, 0.011, 0.0085], SKIN(tone));
  } else {
    S.cap(`${name}-shank`, `shank-${id}`, [sh.o[0], sh.o[1], z], [tars.o[0], tars.o[1], z], 0.0078, 0.0068, SKIN(tone));
    S.cap(`${name}-tarsus`, `tarsus-${id}`, [tars.o[0], tars.o[1], z], [toe.o[0], toe.o[1] + 0.003, z], 0.0076, 0.0068, SKIN(tone));
    S.loft(`${name}-foot`, `toes-${id}`, [-0.016, 0.004, 0.012, 0.03, 0.046].map((dx, i) => ({ c: [toe.o[0] + dx, 0.0075, z] as [number, number, number], a: [0, 0, [0.012, 0.02, 0.026, 0.022, 0.012][i]!] as [number, number, number], b: [0, [0.0075, 0.0075, 0.0075, 0.0075, 0.0075][i]!, 0] as [number, number, number], e: 2.4 })), SKIN(tone));
  }
} else for (const [sd, name, z, tone] of [[-1, "near", -0.026, 0.72], [1, "far", 0.026, 0.55]] as const) {
  const id = sd < 0 ? "n" : "f";
  const sh = S.c.state.frames[`shank-${id}`] as any;
  const tars = S.c.state.frames[`tarsus-${id}`] as any;
  S.cap(`${name}-leg`, `shank-${id}`, [sh.o[0], sh.o[1], z], [tars.o[0], tars.o[1], z], 0.0155, 0.0152, SKIN(tone));
  S.hullX(`${name}-foot`, `tarsus-${id}`, { elev: [`${name}-foot`], xs: range(-0.012, 0.055, 6), look: SKIN(tone), zc: z, half: () => 0.0155, floor: 0.0155, e: 2.4 });
}
console.log(`pigeon 1:${scale}: ${S.body.length} body solids`);
if (write) S.save();

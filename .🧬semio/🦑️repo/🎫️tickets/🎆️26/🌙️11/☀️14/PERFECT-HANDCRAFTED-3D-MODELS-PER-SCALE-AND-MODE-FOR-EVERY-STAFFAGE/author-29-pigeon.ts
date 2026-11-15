/** 🕊️ Author 29: handcrafted pigeon bodies (world-space sculpting per scale). Usage: bun author-29-pigeon.ts <50|100|200> [--write] */
import { Sculpt, type Look } from "./author-29-lib.ts";
const scale = Number(process.argv[2]);
const write = process.argv.includes("--write");
const S = new Sculpt("animals/pigeon", scale);
const f = S.feature;
const P = (tone: number): Look => ({ material: "plumage", tone });
const SKIN = (tone: number): Look => ({ material: "bare-skin", tone });
const range = (a: number, b: number, n: number) => Array.from({ length: n }, (_, i) => a + ((b - a) * i) / (n - 1));

if (scale === 50) {
  S.hullX("body", "trunk", { elev: ["body"], plan: ["body"], xs: range(-0.088, 0.147, 13), look: P(0.72), e: 2.2, half: (_x, h) => (h > 0.04 ? h + 0.007 : h) });
  S.hullY("neck-1", "neck-1", { elev: ["neck-1"], front: ["neck-1"], ys: range(0.139, 0.213, 6), look: P(0.72) });
  S.hullY("neck-2", "neck-2", { elev: ["neck-2"], front: ["neck-2"], ys: range(0.179, 0.227, 5), look: P(0.72) });
  S.hullY("neck-3", "neck-3", { elev: ["neck-3"], front: ["neck-3"], ys: range(0.198, 0.243, 5), look: P(0.72) });
  S.hullX("head", "head", { elev: ["head"], plan: ["head"], xs: range(0.071, 0.132, 7), look: P(0.72), e: 2.2 });
  S.hullX("beak", "head", { elev: ["beak"], plan: ["beak"], xs: range(0.124, 0.152, 5), look: { material: "beak-keratin", tone: 0.3 } });
  S.ell("cere", "head", [0.1355, 0.2245, 0], [0.0105, 0.0105, 0.0095], { material: "cere-wax", tone: 0.9 });
  S.hullX("jaw", "jaw", { elev: ["jaw"], xs: range(0.123, 0.146, 4), look: { material: "beak-keratin", tone: 0.3 }, zc: 0, half: () => 0.0042 });
  for (const [sd, z, tone] of [["", -0.0225, 0.1], ["-f", 0.0225, 0.1]] as const) {
    S.ell(`eye${sd}`, "head", [0.1185, 0.2245, z * 0.9], [0.0048, 0.0058, 0.0042], { material: "eye-tissue", tone });
  }
  const tl = range(-0.176, -0.048, 9);
  S.hullX("tail-f", "tail-f", { elev: ["tail", "far-tail"], plan: ["tail", "far-tail"], xs: tl, look: P(0.55), e: 2.4, vfix: (_x, lo, hi) => [lo, lo + 0.56 * (hi - lo)] });
  S.hullX("tail", "tail", { elev: ["tail", "far-tail"], plan: ["tail"], xs: tl, look: P(0.66), e: 2.4, vfix: (_x, lo, hi) => [hi - 0.56 * (hi - lo), hi] });
  S.hullX("near-primaries", "hand-n", { elev: ["near-primaries"], plan: ["near-primaries"], xs: range(-0.124, 0.054, 11), look: P(0.55), e: 2.3 });
  S.hullX("near-coverts", "forearm-n", { elev: ["near-coverts"], plan: ["near-coverts"], xs: range(-0.019, 0.076, 8), look: P(0.6), e: 2.3 });
  S.hullX("far-primaries", "hand-f", { elev: ["far-primaries"], plan: ["far-primaries"], xs: range(-0.124, 0.054, 11), look: P(0.45), e: 2.3 });
  S.hullX("far-coverts", "forearm-f", { elev: ["near-coverts"], plan: ["far-coverts"], xs: range(-0.019, 0.066, 7), look: P(0.6), e: 2.6, vfix: () => [0.086, 0.184] });
  for (const [sd, name, z, tone] of [["n", "near", -0.026, 0.72], ["f", "far", 0.026, 0.55]] as const) {
    const th = S.c.state.frames[`thigh-${sd}`] as any;
    S.hullY(`${name}-thigh`, `thigh-${sd}`, { elev: [`${name}-thigh`], front: [`${name}-thigh`], ys: range(0.044, 0.124, 6), look: P(tone), zc: z });
    const sh = S.c.state.frames[`shank-${sd}`] as any;
    const tars = S.c.state.frames[`tarsus-${sd}`] as any;
    const toe = S.c.state.frames[`toes-${sd}`] as any;
    S.cap(`${name}-shank`, `shank-${sd}`, [sh.o[0], sh.o[1], z], [tars.o[0], tars.o[1], z], 0.0062, 0.005, SKIN(tone));
    S.cap(`${name}-tarsus`, `tarsus-${sd}`, [tars.o[0], tars.o[1], z], [toe.o[0], toe.o[1] + 0.003, z], 0.0052, 0.0045, SKIN(tone));
    const g = 0.0042;
    const x0 = toe.o[0];
    S.cap(`${name}-toe-mid`, `toes-${sd}`, [x0, g, z], [x0 + 0.04, g, z], 0.0042, 0.004, SKIN(tone));
    S.cap(`${name}-toe-in`, `toes-${sd}`, [x0, g, z], [x0 + 0.034, g, z + (sd === "n" ? 0.024 : -0.024)], 0.004, 0.004, SKIN(tone));
    S.cap(`${name}-toe-out`, `toes-${sd}`, [x0, g, z], [x0 + 0.034, g, z - (sd === "n" ? 0.024 : -0.024)], 0.004, 0.004, SKIN(tone));
    S.cap(`${name}-hallux`, `toes-${sd}`, [x0, g, z], [x0 - 0.021, g, z], 0.0042, 0.004, SKIN(tone));
    S.ell(`${name}-ankle`, `tarsus-${sd}`, [tars.o[0] + 0.004, tars.o[1] + 0.001, z], [0.011, 0.011, 0.0085], SKIN(tone));
  }
}

if (scale === 100) {
  S.hullX("body", "trunk", { elev: ["body"], plan: ["body"], xs: range(-0.088, 0.149, 10), look: P(0.72), e: 2.2, half: (_x, h) => (h > 0.04 ? h + 0.007 : h) });
  S.hullY("neck-1", "neck-1", { elev: ["neck-1"], front: ["neck-1"], ys: range(0.103, 0.216, 5), look: P(0.72) });
  S.hullY("neck-2", "neck-2", { elev: ["neck-2"], front: ["neck-2"], ys: range(0.178, 0.226, 4), look: P(0.72) });
  S.hullY("neck-3", "neck-3", { elev: ["neck-3"], front: ["neck-3"], ys: range(0.196, 0.243, 4), look: P(0.72) });
  S.hullX("head", "head", { elev: ["head"], plan: ["head"], xs: range(0.053, 0.131, 6), look: P(0.72), e: 2.2 });
  S.hullX("beak", "head", { elev: ["beak"], plan: ["beak"], xs: range(0.124, 0.152, 4), look: { material: "beak-keratin", tone: 0.3 } });
  S.ell("cere", "head", [0.1335, 0.2235, 0], [0.0105, 0.0105, 0.0095], { material: "cere-wax", tone: 0.9 });
  S.hullX("jaw", "jaw", { elev: ["jaw"], xs: range(0.105, 0.146, 4), look: { material: "beak-keratin", tone: 0.3 }, zc: 0, half: () => 0.0075 });
  for (const [sd, z] of [["", -0.0225], ["-f", 0.0225]] as const) S.ell(`eye${sd}`, "head", [0.1185, 0.2245, z * 0.9], [0.0076, 0.0076, 0.0076], { material: "eye-tissue", tone: 0.1 });
  const tl = range(-0.176, -0.054, 7);
  S.hullX("tail-f", "tail-f", { elev: ["tail", "far-tail"], plan: ["tail", "far-tail"], xs: tl, look: P(0.55), e: 2.4, vfix: (_x, lo, hi) => [lo, lo + 0.56 * (hi - lo)] });
  S.hullX("tail", "tail", { elev: ["tail", "far-tail"], plan: ["tail"], xs: tl, look: P(0.66), e: 2.4, vfix: (_x, lo, hi) => [hi - 0.56 * (hi - lo), hi] });
  S.hullX("near-primaries", "hand-n", { elev: ["near-primaries"], plan: ["near-primaries"], xs: range(-0.124, 0.139, 12), look: P(0.55), e: 2.3 });
  S.hullX("near-coverts", "forearm-n", { elev: ["near-coverts"], plan: ["near-coverts"], xs: range(-0.043, 0.082, 8), look: P(0.6), e: 2.3 });
  S.hullX("near-shoulder", "humerus-n", { elev: ["near-shoulder"], plan: ["near-coverts", "near-primaries"], xs: range(-0.051, 0.062, 7), look: P(0.6), e: 2.3 });
  S.hullX("far-primaries", "hand-f", { elev: ["far-primaries"], plan: ["far-primaries"], xs: range(-0.124, 0.056, 9), look: P(0.45), e: 2.3 });
  S.hullX("far-coverts", "forearm-f", { elev: ["far-coverts"], plan: ["far-coverts"], xs: range(-0.019, 0.076, 7), look: P(0.6), e: 2.3 });
  S.hullX("far-shoulder", "humerus-f", { elev: ["far-shoulder"], plan: ["far-coverts", "far-primaries"], xs: range(-0.024, 0.048, 5), look: P(0.5), e: 2.3 });
  for (const [sd, name, z, tone] of [["n", "near", -0.0255, 0.72], ["f", "far", 0.0255, 0.55]] as const) {
    const sh = S.c.state.frames[`shank-${sd}`] as any;
    const tars = S.c.state.frames[`tarsus-${sd}`] as any;
    const toe = S.c.state.frames[`toes-${sd}`] as any;
    S.hullY(`${name}-thigh`, `thigh-${sd}`, { elev: [`${name}-thigh`], front: [`${name}-thigh`], ys: range(0.064, 0.124, 4), look: P(tone), zc: z });
    S.cap(`${name}-shank`, `shank-${sd}`, [sh.o[0], sh.o[1], z], [tars.o[0], tars.o[1], z], 0.0078, 0.0068, SKIN(tone));
    S.cap(`${name}-tarsus`, `tarsus-${sd}`, [tars.o[0], tars.o[1], z], [toe.o[0], toe.o[1] + 0.003, z], 0.0076, 0.0068, SKIN(tone));
    S.loft(`${name}-foot`, `toes-${sd}`, [-0.016, 0.004, 0.012, 0.03, 0.046].map((dx, i) => ({ c: [toe.o[0] + dx, 0.0075, z] as [number, number, number], a: [0, 0, [0.012, 0.02, 0.026, 0.022, 0.012][i]!] as [number, number, number], b: [0, [0.0075, 0.0075, 0.0075, 0.0075, 0.0075][i]!, 0] as [number, number, number], e: 2.4 })), SKIN(tone));
  }
}

if (scale === 200) {
  S.hullX("body", "trunk", { elev: ["body"], plan: ["body"], xs: range(-0.086, 0.147, 8), look: P(0.72), e: 2.2, half: (_x, h) => (h > 0.04 ? h + 0.007 : h) });
  S.hullY("neck", "neck-2", { elev: ["neck"], front: ["neck"], ys: range(0.104, 0.245, 6), look: P(0.72) });
  S.hullX("head", "head", { elev: ["head"], plan: ["head"], xs: range(0.025, 0.151, 7), look: P(0.72), e: 2.2 });
  const tl = range(-0.176, -0.05, 6);
  S.hullX("tail-f", "tail-f", { elev: ["tail", "far-tail"], plan: ["tail", "far-tail"], xs: tl, look: P(0.55), e: 2.4, vfix: (_x, lo, hi) => [lo, lo + 0.6 * (hi - lo)] });
  S.hullX("tail", "tail", { elev: ["tail", "far-tail"], plan: ["tail"], xs: tl, look: P(0.66), e: 2.4, vfix: (_x, lo, hi) => [hi - 0.6 * (hi - lo), hi] });
  S.hullX("near-primaries", "hand-n", { elev: ["near-primaries"], plan: ["near-primaries"], xs: range(-0.124, 0.103, 10), look: P(0.55), e: 2.3 });
  S.hullX("near-coverts", "forearm-n", { elev: ["near-coverts"], plan: ["near-coverts"], xs: range(-0.086, 0.076, 8), look: P(0.6), e: 2.3 });
  S.hullX("near-shoulder", "humerus-n", { elev: ["near-shoulder"], plan: ["near-shoulder"], xs: range(-0.071, 0.072, 7), look: P(0.6), e: 2.3 });
  S.hullX("far-wing", "hand-f", { elev: ["far-wing"], plan: ["far-wing"], xs: range(-0.124, 0.1, 9), look: P(0.5), e: 2.3 });
  for (const [sd, name, z, tone] of [["n", "near", -0.026, 0.72], ["f", "far", 0.026, 0.55]] as const) {
    const sh = S.c.state.frames[`shank-${sd}`] as any;
    const tars = S.c.state.frames[`tarsus-${sd}`] as any;
    const toe = S.c.state.frames[`toes-${sd}`] as any;
    S.cap(`${name}-leg`, `shank-${sd}`, [sh.o[0], sh.o[1], z], [tars.o[0], tars.o[1], z], 0.0155, 0.0152, SKIN(tone));
    S.hullX(`${name}-foot`, `tarsus-${sd}`, { elev: [`${name}-foot`], xs: range(-0.012, 0.055, 6), look: SKIN(tone), zc: z, half: () => 0.0155, floor: 0.0155, e: 2.4 });
  }
}

if (process.argv.includes("--extra")) {
  if (scale === 200) {
    S.ell("t-beak", "head", [0.136, 0.2235, 0], [0.016, 0.015, 0.015], { material: "beak-keratin", tone: 0.3 });
    S.ell("t-cere", "head", [0.126, 0.2285, 0], [0.015, 0.015, 0.015], { material: "cere-wax", tone: 0.9 });
    S.ell("t-eye", "head", [0.1185, 0.2245, -0.0225], [0.015, 0.015, 0.015], { material: "eye-tissue", tone: 0.1 });
    S.ell("t-eye-f", "head", [0.1185, 0.2245, 0.0225], [0.015, 0.015, 0.015], { material: "eye-tissue", tone: 0.1 });
    S.takeThermal({ beak: ["t-beak"], cere: ["t-cere"], eye: ["t-eye", "t-eye-f"] });
  }
  process.exit(0);
}
console.log(`pigeon 1:${scale}: ${S.body.length} body solids`);
if (write) S.save();

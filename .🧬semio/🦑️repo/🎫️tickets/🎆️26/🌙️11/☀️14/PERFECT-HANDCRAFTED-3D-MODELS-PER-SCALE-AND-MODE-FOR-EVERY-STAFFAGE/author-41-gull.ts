/** 🐦️ Author 41: handcrafted herring gull bodies per scale: egg-section trunk (back and belly masses), flat fan tail, S-neck, head with bill, folded wings as cambered lofts (coverts arm and primaries hand) lying on the flanks, legs with webbed feet. Usage: bun author-41-gull.ts <50|100|200> [--write] */
import { Zoo, torso2, range, type Look } from "./author-41-lib.ts";
const scale = Number(process.argv[2]);
const write = process.argv.includes("--write");
const S = new Zoo("animals/herring-gull", scale);
const P = (tone: number): Look => ({ material: "plumage", tone });
const W = (tone: number): Look => ({ material: "wing-feathers", tone });
const BL: Look = { material: "bill-keratin", tone: 0.8 };
const SC = (tone: number): Look => ({ material: "scaled-skin", tone });
const NH = Number(process.env.NH ?? 0.046);
const WE = Number(process.env.WE ?? 4.5);
const TL = Number(process.env.TL ?? (scale === 200 ? 0.138 : 0.147));
const k = (a: number, b: number, c: number) => (scale === 50 ? a : scale === 100 ? b : c);

torso2(S, "body", "trunk", { ids: ["body"], xs: [...range(-0.2, 0.14, k(13, 12, 11)), 0.152, 0.159, 0.164, 0.1665], look: P(0.9), bellyW: Number(process.env.BW ?? 0.93), minY: Number(process.env.MY ?? 0.15) });
S.hullX("tail", "tail", { elev: ["tail"], plan: ["tail"], xs: range(-0.325, -0.133, k(9, 7, 5)), look: W(0.4), e: 2.4, half: (_x, h) => Math.min(h, 0.065), vfix: (_x, lo, hi) => [Math.max(lo, TL), hi] });
S.limb("neck-1", "neck-1", { elev: ["neck-1"], plan: ["neck-1"], n: k(6, 5, 4), look: P(0.9), e: 2.2, half: () => NH });
S.limb("neck-2", "neck-2", { elev: ["neck-2"], plan: ["neck-2"], n: k(6, 5, 4), look: P(0.9), e: 2.2, half: () => NH });
S.hullX("head", "head", { elev: ["head"], plan: ["head"], xs: range(0.068, 0.187, k(8, 6, 5)), look: P(0.95), e: 2.2 });
if (scale !== 200) S.hullX("bill", "head", { elev: ["bill"], plan: ["bill"], xs: range(0.165, 0.247, k(7, 5, 4)), look: BL, e: 2.2 });
else S.tube("bill", "head", [0.185, 0.46, 0], [0.235, 0.455, 0], 0.0165, 0.0165, BL);
const sides: [string, string, string, number][] = [["f", "far", "f", 0.7], ["n", "near", "n", 0.9]];
for (const [sd, nm, , tone] of sides) {
  const arm = scale === 50 ? `${nm}-arm` : scale === 100 ? `${nm}-arm` : "arm";
  const hand = scale === 50 ? `${nm}-hand` : scale === 100 ? `${nm}-hand` : "hand";
  const planArm = `${nm}-arm`;
  const planHand = `${nm}-hand`;
  const eA = scale === 200 ? "arm" : sd === "n" ? "near-arm" : "far-arm";
  void arm; void hand;
  S.hullX(`${nm}-arm`, `arm-${sd}`, { elev: [eA], plan: [planArm], xs: range(-0.222, 0.04, k(9, 8, 7)), look: W(tone), e: WE });
  const eH = scale === 200 ? ["hand"] : sd === "n" ? ["near-hand", "wingtip"] : ["far-hand"];
  S.hullX(`${nm}-hand`, `hand-${sd}`, { elev: eH, plan: [planHand], xs: range(-0.44, -0.155, k(10, 8, 6)), look: W(tone - 0.15), e: WE });
}
if (scale === 50) {
  for (const [sd, nm, , tone] of sides) {
    S.limb(`${nm}-thigh`, `thigh-${sd}`, { elev: [`${nm}-thigh`], plan: [`${nm}-thigh`], front: [`${nm}-thigh`], n: 6, look: P(tone), e: 2.1 });
    S.limb(`${nm}-shank`, `shank-${sd}`, { elev: [`${nm}-shank`], plan: [`${nm}-shank`], n: 6, look: SC(tone), e: 2.1 });
    S.limb(`${nm}-tarsus`, `tarsus-${sd}`, { elev: [`${nm}-tarsus`], plan: [`${nm}-tarsus`], n: 6, look: SC(tone), e: 2.1 });
    S.hullX(`${nm}-foot`, `foot-${sd}`, { elev: [`${nm}-foot`], plan: [`${nm}-foot`], xs: range(sd === "n" ? -0.042 : -0.102, sd === "n" ? 0.081 : 0.021, 6), look: SC(tone), e: 2.4 });
  }
}
if (scale !== 50) {
  const pen = scale === 100 ? 0.01 : 0.0165;
  for (const [sd, nm, , tone] of sides) {
    const sh = (S.c.state.elevation as any[]).find((q) => q.shape.id === `${nm}-leg`)!;
    const ring: [number, number][] = (sh.rings[0] as { x: number; y: number }[]).map((p) => [p.x, p.y]);
    const pts: [number, number][] = [];
    for (let i = 0; i < ring.length; i++) { pts.push(ring[i]!); if (i < ring.length - 1) pts.push([(ring[i]![0] + ring[i + 1]![0]) / 2, (ring[i]![1] + ring[i + 1]![1]) / 2]); }
    const st = pts.map((q, i) => {
      const a = pts[Math.max(0, i - 1)]!;
      const b = pts[Math.min(pts.length - 1, i + 1)]!;
      const t = [b[0] - a[0], b[1] - a[1]];
      const l = Math.hypot(t[0]!, t[1]!) || 1;
      const sp = S.span("plan", [`${nm}-leg`], "x", q[0]);
      const hz = scale === 100 ? 0.0125 : 0.017; void sp;
      return { c: [q[0], Math.max(q[1], pen), (nm === "near" ? -1 : 1) * (scale === 100 ? 0.0415 : 0.041)] as [number, number, number], a: [0, 0, hz] as [number, number, number], b: [(-t[1]! / l) * pen, (t[0]! / l) * pen, 0] as [number, number, number], e: 2.2 };
    });
    S.loft(`${nm}-leg`, `shank-${sd}`, st, SC(tone));
  }
}
if (process.argv.includes("--extra") && scale !== 50) {
  const r: [number, number, number] = scale === 100 ? [0.045, 0.0078, 0.03] : [0.05, 0.0165, 0.03];
  const y = scale === 100 ? 0.0095 : 0.0175;
  S.ell("t-foot-n", "shank-n", [0.02, y, -0.0415], r, SC(0.9), [[1, 0, 0], [0, 1, 0]]);
  S.ell("t-foot-f", "shank-f", [-0.05, y, 0.0415], r, SC(0.7), [[1, 0, 0], [0, 1, 0]]);
  S.takeThermal({ "web-feet": ["t-foot-n", "t-foot-f"] });
  process.exit(0);
}
console.log(`gull 1:${scale}: ${S.body.length} body solids`);
if (write) S.save();

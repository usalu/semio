/** 🐦️ Fauna: herring gull (Larus argentatus) bodies per scale from the fauna templates: egg-section trunk, rectrix wedge tail with undertail coverts, tapered neck, head with brow and eye ring, hooked bill (upper mandible with culmen, lower with gonys), folded wings as stacked aerofoil lens (coverts, carpal knob, secondaries, stepped primaries), legs with feathered thigh, tibiotarsus, ankle, slim tarsus and webbed foot. Usage: bun fauna-gull.ts <50|100|200> [--write]
 * @see ./fauna-lib.ts */
import { Fauna, range, type Look } from "./fauna-lib.ts";
import { torso2 } from "./author-41-lib.ts";
const scale = Number(process.argv[2]);
const write = process.argv.includes("--write");
const S = new Fauna("animals/herring-gull", scale);
const lod = (scale === 50 ? 2 : scale === 100 ? 1 : 0) as 0 | 1 | 2;
const P = (tone: number): Look => ({ material: "plumage", tone });
const W = (tone: number): Look => ({ material: "wing-feathers", tone });
const BL: Look = { material: "bill-keratin", tone: 0.8 };
const BL2: Look = { material: "bill-keratin", tone: 0.7 };
const SC = (tone: number): Look => ({ material: "scaled-skin", tone });
const k = (a: number, b: number, c: number) => (scale === 50 ? a : scale === 100 ? b : c);
const TL = scale === 200 ? 0.138 : 0.147;

torso2(S, "body", "trunk", { ids: ["body"], xs: [...range(-0.2, 0.14, k(13, 12, 11)), 0.152, 0.159, 0.164, 0.1665], look: P(0.9), bellyW: 0.93, minY: 0.15 });
S.tail("tail", "tail", { elev: ["tail"], plan: ["tail"], x0: -0.17, x1: -0.327, look: W(0.4), thick: scale === 200 ? 0.02 : 0.018, n: k(8, 6, 4), coverts: lod === 2 ? P(0.85) : undefined });
S.limb("neck-1", "neck-1", { elev: ["neck-1"], plan: ["neck-1"], n: k(6, 5, 4), look: P(0.9), e: 2.2, half: (s) => 0.054 - 0.1 * Math.min(s, 0.08) });
S.limb("neck-2", "neck-2", { elev: ["neck-2"], plan: ["neck-2"], n: k(6, 5, 4), look: P(0.93), e: 2.2, half: (s) => 0.044 - 0.06 * Math.min(s, 0.07) });
S.hullX("head", "head", { elev: ["head"], plan: ["head"], xs: range(0.068, 0.187, k(9, 6, 5)), look: P(0.95), e: 2.2, half: (_x, zh) => Math.min(0.045, zh * 1.07) });
if (scale !== 200) S.bill("bill", "head", { elev: ["bill"], plan: ["bill"], x0: 0.166, x1: 0.247, look: BL, lower: BL2, n: k(8, 5, 4), gape: (t) => 0.4435 + 0.0045 * (1 - t) + 0.001 });
else S.tube("bill", "head", [0.185, 0.455, 0], [0.235, 0.452, 0], 0.0165, 0.0165, BL);
if (lod === 2) for (const sd of [-1, 1] as const) {
  S.eye(`eye-${sd < 0 ? "n" : "f"}`, "head", [0.158, 0.479, sd * 0.0375], 0.0047, sd, { material: "bill-keratin", tone: 0.9 }, { material: "bill-keratin", tone: 0.05 });
}
for (const [sd, nm, id, tone] of [[-1, "near", "n", 0.9], [1, "far", "f", 0.7]] as const) {
  const eA = scale === 200 ? ["arm", "hand"] : [`${nm}-arm`, `${nm}-hand`];
  S.wing(sd, { elev: eA, x0: 0.036, xj: -0.1956, x1: -0.444, zc: 0.08, zh: 0.081, th0: 0.017, thh: 0.009, th1: 0.0045, bones: [`arm-${id}`, `hand-${id}`], id: `${nm}-wing`, lod, look: W(tone), tip: W(tone - 0.2), roll: 0 });
}
if (scale === 50) {
  for (const [sd, nm, id, tone] of [[-1, "near", "n", 0.9], [1, "far", "f", 0.7]] as const) {
    S.limb(`${nm}-thigh`, `thigh-${id}`, { elev: [`${nm}-thigh`], plan: [`${nm}-thigh`], front: [`${nm}-thigh`], n: 7, trim: 0.1, look: P(tone), e: 2.1, half: (s, hz) => hz * (1 - 0.45 * Math.min(1, Math.max(0, s / 0.06))), hv: (s, v) => v * (1 - 0.4 * Math.min(1, Math.max(0, s / 0.06))) });
    const kn = S.frame(`shank-${id}`);
    S.ell(`${nm}-knee`, `shank-${id}`, [kn.o[0]!, kn.o[1]!, sd * 0.04], [0.0085, 0.0085, 0.0085], P(tone));
    S.limb(`${nm}-shank`, `shank-${id}`, { elev: [`${nm}-shank`], plan: [`${nm}-shank`], n: 6, look: SC(tone), e: 2.1 });
    S.limb(`${nm}-tarsus`, `tarsus-${id}`, { elev: [`${nm}-tarsus`], plan: [`${nm}-tarsus`], n: 6, look: SC(tone), e: 2.1 });
    const f: any = S.frame(`foot-${id}`);
    S.foot(`${nm}-foot`, `foot-${id}`, { at: [f.o[0] + 0.0, 0.0045, sd * 0.04], sd, len: 0.082, spread: 0.42, thick: 0.009, toeW: 0.0095, web: 1, hind: 0.014, look: SC(tone), lod, yaw: 0 });
  }
}
if (scale !== 50) {
  const pen = scale === 100 ? 0.01 : 0.0165;
  for (const [sd, nm, , tone] of [[-1, "near", "n", 0.9], [1, "far", "f", 0.7]] as const) {
    const sh = (S.c.state.elevation as any[]).find((q) => q.shape.id === `${nm}-leg`)!;
    const ring: [number, number][] = (sh.rings[0] as { x: number; y: number }[]).map((p) => [p.x, p.y]);
    const pts: [number, number][] = [];
    for (let i = 0; i < ring.length; i++) { pts.push(ring[i]!); if (i < ring.length - 1) pts.push([(ring[i]![0] + ring[i + 1]![0]) / 2, (ring[i]![1] + ring[i + 1]![1]) / 2]); }
    const st = pts.map((q, i) => {
      const a = pts[Math.max(0, i - 1)]!;
      const b = pts[Math.min(pts.length - 1, i + 1)]!;
      const t = [b[0] - a[0], b[1] - a[1]];
      const l = Math.hypot(t[0]!, t[1]!) || 1;
      const hz = scale === 100 ? 0.0125 : 0.017;
      return { c: [q[0], Math.max(q[1], pen), sd * (scale === 100 ? 0.0415 : 0.041)] as [number, number, number], a: [0, 0, hz] as [number, number, number], b: [(-t[1]! / l) * pen, (t[0]! / l) * pen, 0] as [number, number, number], e: 2.2 };
    });
    S.loft(`${nm}-leg`, `shank-${sd < 0 ? "n" : "f"}`, st, SC(tone));
  }
}
void TL;
console.log(`gull 1:${scale}: ${S.body.length} body solids`);
if (write) S.save();

/** 🐑️ Author 68: sheep as a porous fleece over a real trunk: a narrower skin core (barrel, loin) under a fleece of overlapping locks (back, flank and belly masses that reach the drawn outline, staggered along the trunk so the top line and flanks are scalloped like staple clumps), a shoulder and a ham mass on both sides, and a neck ruff; the fleece solids are porous envelopes (C26). Usage: bun fauna-68-sheep.ts <50|100|200> [--write]
 * Sources: Goldfinger "Animal Anatomy for Artists" (sheep: woolly barrel over a lean frame, thin legs, short neck, small head); Ellenberger/Dittrich/Baum; Scholander 1950 (fleece depth). */
import { Fauna, range, type Look } from "./fauna-lib.ts";
const scale = Number(process.argv[2]);
const write = process.argv.includes("--write");
const S = new Fauna("animals/sheep", scale);
const k = <T>(a: T, b: T, c: T): T => (scale === 50 ? a : scale === 100 ? b : c);
const FL = (tone: number): Look => ({ material: "fleece", tone });
const SK: Look = { material: "bare-skin", tone: 0.35 };
const old = new Set(["hindbody", "chest", "near-thigh", "near-humerus", "near-ham", "far-ham", "near-shoulder", "far-shoulder"]);
const mine = (id: string): boolean => /^(lock-|core-|ruff|ham-|shoulder-)/.test(id);
for (const s of S.c.vol.body as any[]) if (mine(s.id)) old.add(s.id);
S.keep([...old]);
const T = ["hindbody", "chest"];
S.hullX("core-hind", "lumbar", { elev: ["hindbody"], plan: ["hindbody"], xs: range(-0.1495, 0.5925, k(10, 8, 6)), look: SK, e: 2.2, half: (_x, zh) => zh * 0.86, vfix: (_x, lo, hi) => [lo + 0.05, hi - 0.05] });
S.hullX("core-chest", "thorax", { elev: ["chest"], plan: ["chest"], xs: range(0.4755, 0.7605, k(8, 6, 4)), look: SK, e: 2.2, half: (_x, zh) => zh * 0.86, vfix: (_x, lo, hi) => [lo + 0.05, hi - 0.05] });
const nc = k(7, 5, 4);
const step = 0.915 / (nc + 1.64);
const rx = step * 1.1;
const cols = range(-0.152 + 1.2 * rx, 0.763 - 1.2 * rx, nc);
const boneAt = (x: number): string => (x < 0.47 ? "lumbar" : "thorax");
const ring = k(0.1, 0.11, 0.12);
for (const [i, x] of cols.entries()) {
  const sp = S.ys(T, x);
  const pl = S.zs(T, x);
  if (sp === null || pl === null) continue;
  const W = 1.01 * Math.min(-pl[0], pl[1]);
  const H = 0.485 * (sp[1] - sp[0]);
  const yc = (sp[0] + sp[1]) / 2;
  const angles = scale === 50 ? (i % 2 === 0 ? [-66, -24, 6, 58] : [-80, -40, -4, 40]) : i % 2 === 0 ? [-60, -6, 45] : [-38, 6, 62];
  const put = (id: string, th: number, sd: number, tone: number): void => {
    const px = W * Math.cos((th * Math.PI) / 180);
    const py = H * Math.sin((th * Math.PI) / 180);
    const f = 1 - ring / Math.hypot(px, py);
    S.blob(id, boneAt(x), [x, yc + py * f, sd * px * f], [rx, ring, ring], FL(tone), [1, 0, 0]);
  };
  put(`lock-top-${i}`, 90, 0, 0.66);
  put(`lock-belly-${i}`, -90, 0, 0.45);
  for (const sd of [-1, 1] as const) for (const [a, th] of angles.entries()) put(`lock-${i}${sd < 0 ? "n" : "f"}${a}`, th, sd, 0.5 + 0.05 * ((i + a) % 3));
}
for (const [i, x] of range(0.6, k(0.84, 0.82, 0.8), k(4, 3, 2)).entries()) {
  const sp = S.ys(["neck"], x);
  const pl = S.zs(["neck"], x);
  if (sp === null || pl === null) continue;
  const W = 0.94 * Math.min(-pl[0], pl[1]);
  const H = 0.47 * (sp[1] - sp[0]);
  const yc = (sp[0] + sp[1]) / 2;
  const r = Math.min(W, H) * 0.62;
  for (const [a, th, sd] of [[0, 90, 0], [1, 30, -1], [2, 30, 1], [3, -35, -1], [4, -35, 1]] as const) {
    const px = W * Math.cos((th * Math.PI) / 180);
    const py = H * Math.sin((th * Math.PI) / 180);
    const f = 1 - r / Math.hypot(px, py);
    S.blob(`ruff-${i}-${a}`, "neck", [x, yc + py * f, sd * px * f], [0.075, r, r], FL(0.56 + 0.04 * ((i + a) % 3)), [1, 0.15, 0]);
  }
}
const W = (x: number): number => { const s = S.zs(T, x); return s === null ? 0.2 : Math.min(-s[0], s[1]); };
for (const [sd, nm, thigh, hum, tone] of [[-1, "near", "thigh-hn", "humerus-fn", 0.58], [1, "far", "thigh-hf", "humerus-ff", 0.48]] as const) {
  const hy = Math.min(0.72, (S.ys(["hindbody"], 0.0)?.[1] ?? 0.72) - 0.05);
  S.mass(`ham-${nm}`, thigh, [-0.02, hy, sd * (W(0.0) - 0.07)], [0.06, 0.36, sd * (W(0.0) - 0.07) * 0.88], { rl: 0.07, rs: 0.115, peak: 0.42, n: k(9, 7, 5), e: 2.3 }, FL(tone));
  const sy = Math.min(0.72, (S.ys(["chest"], 0.6)?.[1] ?? 0.72) - 0.05);
  S.mass(`shoulder-${nm}`, hum, [0.6, sy, sd * (W(0.6) - 0.065)], [0.66, 0.38, sd * (W(0.6) - 0.065) * 0.88], { rl: 0.065, rs: 0.105, peak: 0.42, n: k(9, 7, 5), e: 2.3 }, FL(tone));
}
console.log(`sheep 1:${scale}: ${S.body.length} body solids`);
if (write) S.save();

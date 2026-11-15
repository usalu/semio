/** 🦆️ Author 29: handcrafted mallard bodies at 1:100 and 1:200 (world-space sculpting). Usage: bun author-29-duck.ts <100|200> [--write] */
import { Sculpt, type Look } from "./author-29-lib.ts";
const scale = Number(process.argv[2]);
const write = process.argv.includes("--write");
const S = new Sculpt("animals/mallard-duck", scale);
const range = (a: number, b: number, n: number) => Array.from({ length: n }, (_, i) => a + ((b - a) * i) / (n - 1));
const PL = (tone: number): Look => ({ material: "plumage", tone });
const WF = (tone: number): Look => ({ material: "wing-feathers", tone });
const SK = (tone: number): Look => ({ material: "scaled-skin", tone });
const BILL: Look = { material: "bill-keratin", tone: 0.35 };
const r = scale === 100 ? 0.0086 : 0.0155;
const n = scale === 100 ? 1 : 0;

if (scale === 50) {
  const legs = ["shank", "tarsus", "foot"].flatMap((p) => ["near", "far"].map((n) => `${n}-${p}`));
  S.keep(legs);
  for (const [sd, name, tone] of [["n", "near", 0.55], ["f", "far", 0.45]] as const) {
    const lk = SK(tone);
    const z = sd === "n" ? -0.046 : 0.046;
    S.hullY(`${name}-shank`, `shank-${sd}`, { elev: [`${name}-shank`], front: [`${name}-shank`], ys: range(sd === "n" ? 0.034 : 0.038, sd === "n" ? 0.137 : 0.091, 6), look: lk, e: 2.1 });
    S.hullY(`${name}-tarsus`, `tarsus-${sd}`, { elev: [`${name}-tarsus`], front: [`${name}-tarsus`], ys: range(-0.003, 0.076, 5), look: lk, e: 2.1 });
    const x0 = sd === "n" ? -0.02 : -0.08;
    S.hullX(`${name}-foot`, `foot-${sd}`, { elev: [`${name}-foot`], xs: range(x0, x0 + 0.106, 8), look: lk, e: 2.4, zc: z, half: (x) => 0.012 + 0.028 * Math.sin(Math.min(1, Math.max(0, (x - x0) / 0.09)) * Math.PI * 0.6) });
  }
  console.log(`duck 1:50 legs rebuilt: ${S.body.length} body solids`);
  if (write) S.save();
  process.exit(0);
}
S.hullX("body", "trunk", { elev: ["body"], plan: ["body"], xs: range(-0.202, 0.1685, scale === 100 ? 15 : 13), look: PL(0.3), e: 2.1 });
S.hullX("tail", "tail", { elev: ["tail"], plan: ["tail"], xs: range(-0.28, -0.145, scale === 100 ? 6 : 5), look: PL(0.5), e: 2.3 });
if (scale === 100) {
  S.hullX("near-wing", "wing-n", { elev: ["near-wing"], plan: ["near-wing"], xs: range(-0.282, 0.056, 10), look: WF(0.45), e: 2.3 });
  S.hullX("far-wing", "wing-f", { elev: ["far-wing"], plan: ["far-wing"], xs: range(-0.276, 0.049, 10), look: WF(0.55), e: 2.3 });
} else {
  S.hullX("near-wing", "wing-n", { elev: ["wing"], plan: ["wing"], xs: range(-0.276, -0.008, 9), look: WF(0.45), e: 2.3 });
  S.hullX("far-wing", "wing-f", { elev: ["wing"], plan: ["wing-far"], xs: range(-0.267, 0.041, 9), look: WF(0.55), e: 2.3, vfix: (_x, lo, hi) => [lo - 0.03, hi] });
}
S.hullY("neck-lower", "neck-1", { elev: ["neck-lower"], front: ["neck-lower"], ys: range(0.226, 0.34, scale === 100 ? 5 : 4), look: PL(0.5) });
S.hullY("neck-upper", "neck-3", { elev: ["neck-upper"], front: ["neck-upper"], ys: range(0.288, 0.446, scale === 100 ? 6 : 5), look: PL(0.6) });
S.hullX("head", "head", { elev: ["head"], plan: ["head"], xs: range(scale === 100 ? 0.152 : 0.141, scale === 100 ? 0.268 : 0.272, 6), look: PL(0.78), e: 2.2 });
S.hullX("bill", "head", { elev: ["bill"], plan: ["bill"], xs: range(scale === 100 ? 0.176 : 0.168, scale === 100 ? 0.321 : 0.327, 6), look: BILL, e: 2.4 });

for (const [sd, name, z, tone] of [["n", "near", -0.046, 0.5], ["f", "far", 0.046, 0.4]] as const) {
  const sh = S.c.state.frames[`shank-${sd}`] as any;
  const ta = S.c.state.frames[`tarsus-${sd}`] as any;
  const ft = S.c.state.frames[`foot-${sd}`] as any;
  const knee: [number, number, number] = [sh.o[0], sh.o[1], z];
  const ankle: [number, number, number] = [ta.o[0], ta.o[1], z];
  const toe: [number, number, number] = [ft.o[0], ft.o[1], z];
  S.cap(`${name}-shank`, `shank-${sd}`, knee, ankle, r, r * 0.98, SK(tone));
  S.cap(`${name}-tarsus`, `tarsus-${sd}`, ankle, toe, r * 0.98, r * 0.98, SK(tone));
  const y = scale === 100 ? 0.0079 : 0.0155;
  const th = scale === 100 ? 0.0078 : 0.0155;
  const x0 = ft.o[0];
  S.loft(`${name}-foot`, `foot-${sd}`, [-0.012, 0.02, 0.05, 0.078, 0.092].map((dx, i) => ({ c: [x0 + dx, y, z] as [number, number, number], a: [0, 0, [0.01, 0.018, 0.026, 0.03, 0.022][i]!] as [number, number, number], b: [0, th, 0] as [number, number, number], e: 2.4 })), SK(tone));
}
void n;
console.log(`duck 1:${scale}: ${S.body.length} body solids`);
if (write) S.save();

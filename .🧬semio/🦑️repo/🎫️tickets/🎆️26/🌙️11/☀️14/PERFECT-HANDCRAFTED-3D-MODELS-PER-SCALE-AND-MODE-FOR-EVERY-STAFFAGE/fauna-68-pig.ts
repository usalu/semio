/** 🐖️ Author 68: domestic pig anatomy on the repaired drawings: the trunk is a narrower core (ribs and loin) and the shoulder blade, ham and jowl are separate masses that join it and reach the drawn outline; the elevation outline gets an arched back. Usage: bun fauna-68-pig.ts <50|100|200> [--write]
 * Sources: Goldfinger "Animal Anatomy for Artists" (pig: arched back, deep barrel, ham as a rounded mass hanging from the croup, shoulder, heavy jowl, disc snout); Ellenberger/Dittrich/Baum. */
import { Fauna, range, type Look } from "./fauna-lib.ts";
const scale = Number(process.argv[2]);
const write = process.argv.includes("--write");
const S = new Fauna("animals/pig", scale);
const k = <T>(a: T, b: T, c: T): T => (scale === 50 ? a : scale === 100 ? b : c);
const L = (tone: number): Look => ({ material: "skin-bristles", tone });
const sample = S.c.vol.body.find((b: any) => b.id === "hindbody");
const mat = (sample as any).material as string;
const look = (tone: number): Look => ({ material: mat, tone });
S.keep(["hindbody", "chest", "near-ham", "far-ham", "near-shoulder", "far-shoulder", "jowl-n", "jowl-f"]);
S.hullX("hindbody", "lumbar", { elev: ["hindbody"], plan: ["hindbody"], xs: range(-0.1355, 0.5925, k(13, 10, 8)), look: look(0.55), e: 2.15, half: (x, zh) => zh * (x < 0.2 ? k(0.93, 0.95, 0.95) : 1) });
S.hullX("chest", "thorax", { elev: ["chest"], plan: ["chest"], xs: range(0.4695, 0.7995, k(10, 8, 6)), look: look(0.55), e: 2.15, half: (x, zh) => zh * (x < 0.62 ? 1 : k(0.93, 0.95, 0.95)) });
const W = (x: number): number => { const s = S.zs(["hindbody", "chest"], x); return s === null ? 0.19 : Math.min(-s[0], s[1]); };
for (const [sd, nm, thigh, scap, tone] of [[-1, "near", "thigh-hn", "scapula-fn", 0.52], [1, "far", "thigh-hf", "scapula-ff", 0.4]] as const) {
  const rl = 0.082;
  const zh = sd * (W(0.0) - rl * 1.0);
  const hy = Math.min(0.69, (S.ys(["hindbody"], 0.005)?.[1] ?? 0.69) - 0.04);
  S.mass(`${nm}-ham`, thigh, [-0.01, hy, zh], [0.065, 0.31, zh * 0.88], { rl, rs: 0.13, peak: 0.42, n: k(10, 8, 6), e: 2.3 }, look(tone));
  const rs = 0.07;
  const zs = sd * (W(0.63) - rs * 1.0);
  const sy = Math.min(0.73, (S.ys(["chest"], 0.6)?.[1] ?? 0.73) - 0.02);
  S.mass(`${nm}-shoulder`, scap, [0.6, sy, zs], [0.69, 0.36, zs * 0.88], { rl: rs, rs: 0.115, peak: 0.42, n: k(10, 8, 6), e: 2.3 }, look(tone));
}
for (const sd of [-1, 1] as const) S.blob(`jowl-${sd < 0 ? "n" : "f"}`, "head", [0.875, 0.5, sd * 0.052], k([0.075, 0.07, 0.04], [0.08, 0.075, 0.045], [0.09, 0.085, 0.05]), look(0.5), [1, -0.35, 0]);
console.log(`pig 1:${scale}: ${S.body.length} body solids`);
if (write) S.save();

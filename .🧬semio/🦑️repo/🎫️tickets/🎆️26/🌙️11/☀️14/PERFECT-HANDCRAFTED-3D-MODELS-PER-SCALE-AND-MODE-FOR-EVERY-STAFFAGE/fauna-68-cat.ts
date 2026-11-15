/** 🐈️ Author 68: domestic cat head modelled as a cat skull on the repaired 2D head (see fauna-68-cat2d.ts): a round cranium that ends in a forehead face with a stop, a narrow nose bridge and two whisker pads (a short blunt muzzle), cheeks at the zygomatic arches, brow ridges, large forward-facing eyes on the face plane beside the bridge, a rounded nose leather, and thin pointed triangular ears set wide at the upper corners. Usage: bun fauna-68-cat.ts <50|100|200> [--write]
 * Sources: Goldfinger "Animal Anatomy for Artists" (cat head: short muzzle, wide zygomatic width, large forward eyes, ears at the upper corners of the skull); Ellenberger/Dittrich/Baum. */
import { Fauna, range, type Look } from "./fauna-lib.ts";
const scale = Number(process.argv[2]);
const write = process.argv.includes("--write");
const S = new Fauna("animals/domestic-cat", scale);
const F = (tone: number): Look => ({ material: "short-fur", tone });
const k = <T>(a: T, b: T, c: T): T => (scale === 50 ? a : scale === 100 ? b : c);
S.keep(["head", "near-ear", "far-ear", "whisker-pad-n", "whisker-pad-f", "nose-leather", "eye-a", "eye-b", "muzzle", "cheek-n", "cheek-f", "brow-n", "brow-f"]);
const fl = scale === 200 ? 0.0165 : 0;
S.hullX("head", "head", { elev: ["head"], plan: ["head"], xs: range(0.3535, 0.424, k(8, 6, 4)), look: F(0.72), e: 2.5, half: (x, zh) => zh * (x > 0.406 ? 1 - 0.3 * Math.min(1, (x - 0.406) / 0.018) : 1) });
S.hullX("muzzle", "head", { elev: ["head"], plan: ["head"], xs: range(0.41, 0.4675, k(7, 5, 3)), look: F(0.74), e: 2.3, vfix: (_x, lo, hi) => [Math.max(lo, 0.264), Math.min(hi, 0.304)], half: (x, zh) => Math.max(fl, Math.min(zh * 0.9, 0.0105 + 0.004 * ((x - 0.41) / 0.06))) });
if (scale !== 200) {
  for (const sd of [-1, 1] as const) {
    const n = sd < 0 ? "n" : "f";
    S.blob(`whisker-pad-${n}`, "head", [0.4405, 0.2735, sd * 0.0165], k([0.016, 0.0105, 0.0108], [0.017, 0.0115, 0.0115], [0, 0, 0]) as [number, number, number], F(0.72), [1, 0, 0]);
    S.blob(`cheek-${n}`, "head", [0.395, 0.275, sd * 0.03], k([0.022, 0.017, 0.012], [0.022, 0.017, 0.013], [0, 0, 0]) as [number, number, number], F(0.72), [1, 0, 0]);
    S.blob(`brow-${n}`, "head", [0.4185, 0.3085, sd * 0.0205], k([0.012, 0.003, 0.009], [0.013, 0.0055, 0.01], [0, 0, 0]) as [number, number, number], F(0.7), [1, 0, 0]);
  }
  S.ell("nose-leather", "head", [0.4635, 0.2845, 0], k([0.0075, 0.0075, 0.0095], [0.0095, 0.0095, 0.0105], [0, 0, 0]) as [number, number, number], { material: "bare-skin", tone: 0.08 });
  for (const [id, z] of [["eye-a", -0.0205], ["eye-b", 0.0205]] as const) S.ell(id, "head", [0.4235, 0.2935, z], k([0.0058, 0.0066, 0.0056], [0.0075, 0.0078, 0.0075], [0, 0, 0]) as [number, number, number], { material: "cornea", tone: 0.04, role: "detail" });
}
for (const [id, bone, tone, sd] of [["near-ear", "ear-n", 0.6, -1], ["far-ear", "ear-f", 0.5, 1]] as const) S.ear(id, bone, { elev: [id], front: [id], y0: 0.305, y1: 0.356, n: k(7, 5, 4), look: F(tone), lean: sd * 0.003, thin: k(0.45, 0.68, 1.06), cup: 0.003, floor: 0.0012 });
console.log(`cat 1:${scale}: ${S.body.length} body solids`);
if (write) S.save();

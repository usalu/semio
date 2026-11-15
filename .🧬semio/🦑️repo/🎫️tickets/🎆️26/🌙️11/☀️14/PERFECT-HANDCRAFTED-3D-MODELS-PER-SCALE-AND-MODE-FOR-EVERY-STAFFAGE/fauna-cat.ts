/** 🐈️ Fauna: domestic cat refinement on top of the author-41 build (kept: trunk, limbs, tail): leaf ears (flat, broad toward the front, pointed, cupped) instead of cones, whisker pads that make the muzzle short and blunt, a chin pad that closes the lower jaw between the rami. Usage: bun fauna-cat.ts <50|100|200> [--write]
 * Sources: Goldfinger "Animal Anatomy for Artists" (cat head: short broad muzzle, zygomatic width, ear set at the upper corners of the skull); Ellenberger/Dittrich/Baum. */
import { Fauna, type Look } from "./fauna-lib.ts";
const scale = Number(process.argv[2]);
const write = process.argv.includes("--write");
if (scale === 200) { console.log("cat 1:200 kept (ears at the 30 mm floor stay cones)"); process.exit(0); }
const S = new Fauna("animals/domestic-cat", scale);
const F = (tone: number): Look => ({ material: "short-fur", tone });
const k = (a: number, b: number, c: number) => (scale === 50 ? a : scale === 100 ? b : c);
S.keep(["near-ear", "far-ear", "whisker-pad-n", "whisker-pad-f", "cheek-n", "cheek-f"]);
for (const [id, bone, tone, sd] of [["near-ear", "ear-n", 0.6, -1], ["far-ear", "ear-f", 0.5, 1]] as const) S.ear(id, bone, { elev: [id], front: [id], y0: 0.305, y1: 0.357, n: k(7, 5, 3), look: F(tone), lean: sd * 0.006, thin: k(0.42, 0.5, 0.55), cup: 0.004 });
if (scale !== 200) {
  for (const sd of [-1, 1] as const) {
    S.blob(`whisker-pad-${sd < 0 ? "n" : "f"}`, "head", [0.4375, 0.2795, sd * 0.0175], [0.0115, 0.009, 0.0095], F(0.72), [1, 0, 0]);
  }
}
console.log(`cat 1:${scale}: ${S.body.length} body solids`);
if (write) S.save();

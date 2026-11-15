/** 🦌️ Fauna: roe deer trunk rebuilt from the repaired outlines (flank tuck, deep brisket, waist), dorsal ridge (spinous processes of the withers and loin) and joint masses (elbow, carpus, hock) on top of the kept legs, neck, head and ears. Usage: bun fauna-roe.ts <50|100|200> [--write]
 * Sources: Goldfinger (roe deer), Ellenberger/Dittrich/Baum (cervid skeleton: high hock, long cannon bones). */
import { Fauna, range, type Look } from "./fauna-lib.ts";
const scale = Number(process.argv[2]);
const write = process.argv.includes("--write");
const S = new Fauna("animals/roe-deer", scale);
const C = (tone: number): Look => ({ material: "summer-coat", tone });
const k = (a: number, b: number, c: number) => (scale === 50 ? a : scale === 100 ? b : c);
S.keep(["hindbody", "chest", "spine-hind", "spine-fore", ...["near", "far"].flatMap((s) => [`${s}-hock`, `${s}-carpus`, `${s}-elbow`])]);
S.hullX("hindbody", "lumbar", { elev: ["hindbody"], plan: ["hindbody"], xs: range(-0.1425, 0.4995, k(13, 10, 8)), look: C(0.55), e: 2.3 });
S.hullX("chest", "thorax", { elev: ["chest"], plan: ["chest"], xs: range(0.4505, 0.9035, k(10, 8, 6)), look: C(0.55), e: 2.3 });
if (scale <= 100) {
  S.ridge("spine-hind", "lumbar", { elev: ["hindbody"], x0: -0.08, x1: 0.46, n: 8, w: 0.012, h: 0.009, sink: 0.006, look: C(0.5) });
  S.ridge("spine-fore", "thorax", { elev: ["chest"], x0: 0.5, x1: 0.8, n: 5, w: 0.014, h: 0.01, sink: 0.006, look: C(0.5) });
}
if (scale === 50) {
  for (const [sd, nm, tone] of [[-1, "near", 0.6], [1, "far", 0.5]] as const) {
    const sd2 = sd < 0 ? "n" : "f";
    const spec: [string, string, string, number][] = [["hock", `cannon-h${sd2}`, `${nm}-shank`, 0.0155], ["carpus", `cannon-f${sd2}`, `${nm}-forearm`, 0.0125], ["elbow", `forearm-f${sd2}`, `${nm}-humerus`, 0.0165]];
    for (const [id, bone, shape, r] of spec) {
      const f = S.frame(bone);
      const x = f.o[0]!;
      const pl = S.zs([shape], x) ?? S.zs([`${nm}-forearm`], x);
      const z = pl === null ? sd * 0.06 : (pl[0] + pl[1]) / 2;
      S.blob(`${nm}-${id}`, bone, [x, f.o[1]!, z], [r, r * 1.15, r * 0.9], C(tone), [0, 1, 0]);
    }
  }
}
console.log(`roe 1:${scale}: ${S.body.length} body solids`);
if (write) S.save();

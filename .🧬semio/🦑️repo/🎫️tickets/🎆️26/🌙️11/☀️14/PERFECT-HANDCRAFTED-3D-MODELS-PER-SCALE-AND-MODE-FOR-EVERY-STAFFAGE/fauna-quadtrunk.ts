/** 🐖️ Fauna: rebuilds the two trunk lofts (hindbody on lumbar, chest on thorax) of a quadruped from the repaired outlines of the scale and adds a dorsal ridge, keeping every other solid. Usage: bun fauna-quadtrunk.ts <asset> <50|100|200> <material> <hx0,hx1> <cx0,cx1> [--write] [--ridge x0,x1,w,h,sink]
 * Sources: Goldfinger (pig, sheep), Ellenberger/Dittrich/Baum. */
import { Fauna, range } from "./fauna-lib.ts";
const [asset, sc, material, hr, cr] = process.argv.slice(2);
const scale = Number(sc);
const write = process.argv.includes("--write");
const S = new Fauna(asset!, scale);
const look = { material: material!, tone: 0.55 };
const [h0, h1] = hr!.split(",").map(Number) as [number, number];
const [c0, c1] = cr!.split(",").map(Number) as [number, number];
const k = (a: number, b: number, c: number) => (scale === 50 ? a : scale === 100 ? b : c);
S.keep(["hindbody", "chest", "spine-hind", "spine-fore"]);
S.hullX("hindbody", "lumbar", { elev: ["hindbody"], plan: ["hindbody"], xs: range(h0 + 0.0025, h1 - 0.0005, k(13, 10, 8)), look, e: 2.3 });
S.hullX("chest", "thorax", { elev: ["chest"], plan: ["chest"], xs: range(c0 + 0.0005, c1 - 0.0005, k(10, 8, 6)), look, e: 2.3 });
const ri = process.argv.indexOf("--ridge");
if (ri > 0 && scale <= 100) {
  const [x0, x1, w, h, sink] = process.argv[ri + 1]!.split(",").map(Number) as [number, number, number, number, number];
  const mid = (h1 + c0) / 2;
  S.ridge("spine-hind", "lumbar", { elev: ["hindbody"], x0, x1: mid, n: 8, w, h, sink, look: { material: material!, tone: 0.5 } });
  S.ridge("spine-fore", "thorax", { elev: ["chest"], x0: mid + 0.02, x1, n: 5, w, h, sink, look: { material: material!, tone: 0.5 } });
}
console.log(`${asset} 1:${scale}: ${S.body.length} body solids`);
if (write) S.save();

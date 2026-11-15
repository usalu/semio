import { fillSmallHoles as live, boundarySpread as liveSpread } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { fillSmallHoles as next, boundarySpread as nextSpread } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes-next/🌓️silhouette/🟦️.ts";
let seed = 7;
const rnd = () => ((seed = (seed * 1103515245 + 12345) & 0x7fffffff) / 0x7fffffff);
let bad = 0;
for (let t = 0; t < 300; t++) {
  const [w, h] = [20 + Math.floor(rnd() * 60), 20 + Math.floor(rnd() * 60)];
  const density = 0.3 + rnd() * 0.5;
  const mk = () => Uint8Array.from({ length: w * h }, () => (rnd() < density ? 1 : 0));
  const a = mk();
  const b = Uint8Array.from(a, (x) => (rnd() < 0.1 ? 1 - x : x));
  const max = 1 + Math.floor(rnd() * 30);
  if (live(a, w, h, max).some((x, i) => x !== next(a, w, h, max)[i])) bad++;
  const soft = rnd() < 0.5 ? mk() : undefined;
  const core = { i0: 2, i1: w - 2, j0: 2, j1: h - 2 };
  const bins = 6 + Math.floor(rnd() * 20);
  const [p, q] = [liveSpread(a, b, soft, w, h, 1, core, bins, max), nextSpread(a, b, soft, w, h, 1, core, bins, max)];
  if (JSON.stringify(p) !== JSON.stringify(q)) { bad++; if (bad < 4) console.log(t, JSON.stringify(p).slice(0, 200), JSON.stringify(q).slice(0, 200)); }
}
console.log("mismatches", bad);

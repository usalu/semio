/** 📏️ Round 3 roe deer: the bounding box of the 1:50 body in the default pose as `height length width` (m, to the millimetre), the values `dimensions` takes; with `all` the box of every scale. Usage: bun r3-roe-deer-bbox.ts [all] */
import { openCtx } from "./kit-lib.ts";
import { meshesAt } from "./volumes-lib.ts";
import { SPEC } from "./r3-roe-deer-kit.ts";
const box = (scale: number): number[] => {
  const c = openCtx(SPEC, scale);
  const lo = [Infinity, Infinity, Infinity];
  const hi = [-Infinity, -Infinity, -Infinity];
  for (const m of meshesAt(c.m, scale, c.vol.body)) for (let i = 0; i < m.world.length; i++) { lo[i % 3] = Math.min(lo[i % 3]!, m.world[i]!); hi[i % 3] = Math.max(hi[i % 3]!, m.world[i]!); }
  return [hi[1]! - Math.min(0, lo[1]!), hi[0]! - lo[0]!, hi[2]! - lo[2]!, lo[0]!, hi[0]!, lo[1]!];
};
if (process.argv[2] === "all") for (const s of [50, 100, 200, 500]) console.log(s, box(s).map((v) => v.toFixed(4)).join(" "));
else console.log(box(50).slice(0, 3).map((v) => v.toFixed(3)).join(" "));

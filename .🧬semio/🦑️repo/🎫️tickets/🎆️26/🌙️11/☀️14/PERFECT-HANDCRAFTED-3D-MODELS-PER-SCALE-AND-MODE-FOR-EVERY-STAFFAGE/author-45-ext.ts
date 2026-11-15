/** 📏 Extent per body solid in the rest pose (world): bun author-45-ext.ts <asset> <scale> [axis 0|1|2]; prints the solids that set the min/max of the axis. */
import { openCtx } from "./kit-lib.ts";
import { scaleMeshes } from "./volumes-lib.ts";
const c = openCtx(process.argv[2]!, Number(process.argv[3]));
const axis = Number(process.argv[4] ?? "0");
const m = scaleMeshes(c.m as never, c.scale, c.vol as never);
const rows = m.body.map((s: any, i: number) => {
  let lo = Infinity, hi = -Infinity;
  const v = s.world;
  for (let k = axis; k < v.length; k += 3) { lo = Math.min(lo, v[k]); hi = Math.max(hi, v[k]); }
  return { id: s.id, lo, hi };
}).filter((r) => Number.isFinite(r.lo));
rows.sort((a, b) => a.lo - b.lo);
console.log("min", rows.slice(0, 4).map((r) => `${r.id} ${r.lo.toFixed(4)}`).join(" | "));
rows.sort((a, b) => b.hi - a.hi);
console.log("max", rows.slice(0, 4).map((r) => `${r.id} ${r.hi.toFixed(4)}`).join(" | "));
console.log(m.body.length, m.unresolved, c.vol.body.length, Object.keys(c));

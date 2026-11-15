/** 📐️ Extent of every drawn shape (default pose) per view: bun author-63-shapes.ts <asset> <scale> [id,id] */
import { drawn, openCtx, VIEWS } from "./kit-lib.ts";
const [spec, sc, ids] = process.argv.slice(2);
const c = openCtx(spec!, Number(sc));
for (const v of VIEWS) for (const s of drawn(c, v)) {
  if (ids && !ids.split(",").includes(s.id)) continue;
  let [x0, x1, y0, y1] = [Infinity, -Infinity, Infinity, -Infinity];
  for (const r of s.rings) for (const p of r) { x0 = Math.min(x0, p.x); x1 = Math.max(x1, p.x); y0 = Math.min(y0, p.y); y1 = Math.max(y1, p.y); }
  console.log(v.padEnd(9), s.id.padEnd(16), (s.bone ?? "").padEnd(12), `x ${x0.toFixed(3)}..${x1.toFixed(3)}  y ${y0.toFixed(3)}..${y1.toFixed(3)}`);
}

/** 📐️ Drawn extents per view and the shapes at each extreme: bun author-24-ext.ts <asset> <scale> */
import { entryOf, loadSource, merged, poseState, VIEWS } from "./volumes-lib.ts";

const [spec, sc] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!);
const st = poseState(m, Number(sc));
for (const v of VIEWS) {
  let b = { x0: Infinity, x1: -Infinity, y0: Infinity, y1: -Infinity };
  const who: Record<string, string> = {};
  for (const s of st[v] as any[]) {
    if (!s.closed || s.shape.filled === false || !["body", "detail", "glazing"].includes(s.shape.role) || s.rings.length === 0) continue;
    for (const ring of s.rings) for (const p of ring) {
      if (p.x < b.x0) { b.x0 = p.x; who.x0 = s.shape.id; }
      if (p.x > b.x1) { b.x1 = p.x; who.x1 = s.shape.id; }
      if (p.y < b.y0) { b.y0 = p.y; who.y0 = s.shape.id; }
      if (p.y > b.y1) { b.y1 = p.y; who.y1 = s.shape.id; }
    }
  }
  console.log(v, `x ${b.x0.toFixed(3)}..${b.x1.toFixed(3)} (${(b.x1 - b.x0).toFixed(3)}) y ${b.y0.toFixed(3)}..${b.y1.toFixed(3)} (${(b.y1 - b.y0).toFixed(3)})`, JSON.stringify(who));
}

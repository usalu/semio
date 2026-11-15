/** 🔬️ Drawn extents per shape: bun author-33-ext.ts <asset> <scale> [axis x|y] */
import { drawn, openCtx, VIEWS } from "./kit-lib.ts";
const c = openCtx(process.argv[2]!, Number(process.argv[3]));
for (const v of VIEWS) {
  let lo = [Infinity, Infinity], hi = [-Infinity, -Infinity];
  const rows: string[] = [];
  for (const s of drawn(c, v)) {
    const l = [Infinity, Infinity], h = [-Infinity, -Infinity];
    for (const r of s.rings) for (const q of r) { l[0] = Math.min(l[0]!, q.x); l[1] = Math.min(l[1]!, q.y); h[0] = Math.max(h[0]!, q.x); h[1] = Math.max(h[1]!, q.y); }
    lo = [Math.min(lo[0]!, l[0]!), Math.min(lo[1]!, l[1]!)]; hi = [Math.max(hi[0]!, h[0]!), Math.max(hi[1]!, h[1]!)];
    rows.push(`${s.id.padEnd(16)} x ${l[0]!.toFixed(3)}..${h[0]!.toFixed(3)}  y ${l[1]!.toFixed(3)}..${h[1]!.toFixed(3)}`);
  }
  console.log(`== ${v} whole x ${lo[0]!.toFixed(3)}..${hi[0]!.toFixed(3)} y ${lo[1]!.toFixed(3)}..${hi[1]!.toFixed(3)}`);
  const cut = process.argv[4] === "all" ? rows : rows.filter((r) => /cane|hand|shoe|cap|head|torso|trunk/.test(r));
  console.log(cut.join("\n"));
}

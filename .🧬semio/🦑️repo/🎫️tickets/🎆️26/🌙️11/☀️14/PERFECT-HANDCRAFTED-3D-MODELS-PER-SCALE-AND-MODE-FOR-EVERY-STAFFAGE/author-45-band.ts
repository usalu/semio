/** 📐 x-extent of the drawn elevation silhouette per height band: bun author-45-band.ts <asset> <scale> [y0 y1 step] [shapeid] */
import { drawn, openCtx } from "./kit-lib.ts";
const c = openCtx(process.argv[2]!, Number(process.argv[3]));
const y0 = Number(process.argv[4] ?? 0), y1 = Number(process.argv[5] ?? 1.8), st = Number(process.argv[6] ?? 0.05);
const only = process.argv[7];
const sh = drawn(c, "elevation").filter((s) => (!only || s.id === only) && s.id !== "wall-surface");
for (let y = y1; y >= y0; y -= st) {
  const xs: number[] = [];
  for (const s of sh) for (const ring of s.rings) for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
    const a = ring[k]!, b = ring[i]!;
    if ((a.y <= y) !== (b.y <= y)) xs.push(a.x + ((y - a.y) / (b.y - a.y)) * (b.x - a.x));
  }
  xs.sort((p, q) => p - q);
  console.log(y.toFixed(2), xs.map((x) => x.toFixed(3)).join(" "));
}

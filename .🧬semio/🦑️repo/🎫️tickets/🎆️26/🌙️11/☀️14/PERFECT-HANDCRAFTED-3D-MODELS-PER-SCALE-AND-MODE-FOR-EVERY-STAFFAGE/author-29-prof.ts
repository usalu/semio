/** 📈 Author 29: scanline profiles of drawn shapes. Usage: bun author-29-prof.ts <asset> <scale> <view> <shapeId[,shapeId]> <along x|y> <from> <to> <step> */
import { openCtx, drawnSpan } from "./kit-lib.ts";
const [spec, scaleS, view, ids, along, a, b, st] = process.argv.slice(2);
const c = openCtx(spec!, Number(scaleS));
const set = new Set(ids!.split(","));
const out: string[] = [];
for (let t = Number(a); t <= Number(b) + 1e-9; t += Number(st)) {
  const sp = drawnSpan(c, view as any, undefined, along as any, t, (s) => set.has(s.id));
  out.push(`${t.toFixed(3)}:${sp ? sp[0].toFixed(3) + ".." + sp[1].toFixed(3) : "-"}`);
}
console.log(out.join("  "));

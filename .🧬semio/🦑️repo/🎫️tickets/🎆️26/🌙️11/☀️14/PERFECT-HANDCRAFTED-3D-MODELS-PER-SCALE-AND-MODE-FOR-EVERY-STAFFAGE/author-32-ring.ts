/** 🔬️ Drawn body ring of a shape in one view, bottom edge samples. Usage: bun author-32-ring.ts cat/slug scale view shapeId x0 x1 */
import { openCtx, drawn } from "./kit-lib.ts";
const [spec, sc, view, id, x0, x1] = process.argv.slice(2);
const c = openCtx(spec!, Number(sc));
for (const s of drawn(c, view as any)) {
  if (s.id !== id) continue;
  for (const r of s.rings) {
    const pts = r.filter((p) => p.x >= Number(x0) && p.x <= Number(x1));
    console.log(s.id, r.length, pts.map((p) => `(${p.x.toFixed(2)},${p.y.toFixed(2)})`).join(" "));
  }
}

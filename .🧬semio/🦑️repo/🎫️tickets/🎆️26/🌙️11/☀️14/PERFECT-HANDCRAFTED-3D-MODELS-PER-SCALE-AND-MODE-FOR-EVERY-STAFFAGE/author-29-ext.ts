import { openCtx } from "./kit-lib.ts";
const [spec, scaleS] = process.argv.slice(2);
const c = openCtx(spec!, Number(scaleS));
for (const v of ["elevation", "front", "plan"] as const) {
  const b = { x0: 1e9, x1: -1e9, y0: 1e9, y1: -1e9 };
  const who: Record<string, string> = {};
  for (const s of c.state[v] as any[]) {
    if (!["body", "detail", "glazing"].includes(s.shape.role)) continue;
    for (const r of s.rings) for (const p of r) {
      if (p.x < b.x0) { b.x0 = p.x; who.x0 = s.shape.id; }
      if (p.x > b.x1) { b.x1 = p.x; who.x1 = s.shape.id; }
      if (p.y < b.y0) { b.y0 = p.y; who.y0 = s.shape.id; }
      if (p.y > b.y1) { b.y1 = p.y; who.y1 = s.shape.id; }
    }
  }
  console.log(v, JSON.stringify(b, (k, x) => typeof x === "number" ? +x.toFixed(4) : x), JSON.stringify(who), "w", (b.x1 - b.x0).toFixed(4), "h", (b.y1 - b.y0).toFixed(4));
}

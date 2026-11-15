import { openCtx, drawn } from "./kit-lib.ts";
const [spec, sc] = [process.argv[2]!, Number(process.argv[3])];
const c = openCtx(spec, sc);
const f = (n: number[]) => n.map((x) => x.toFixed(3)).join(",");
for (const [id, fr] of Object.entries(c.state.frames)) console.log("F", id, "o", f(fr.o), "u", f(fr.u), "v", f(fr.v), "w", f(fr.w));
for (const view of ["elevation", "front", "plan"] as const) for (const d of drawn(c, view)) {
  let minX = 1e9, maxX = -1e9, minY = 1e9, maxY = -1e9, n = 0;
  for (const r of d.rings) for (const p of r) { n++; minX = Math.min(minX, p.x); maxX = Math.max(maxX, p.x); minY = Math.min(minY, p.y); maxY = Math.max(maxY, p.y); }
  console.log(view, d.id, d.bone, d.role, d.region ?? "", "x", f([minX, maxX]), "y", f([minY, maxY]), "pts", n);
}

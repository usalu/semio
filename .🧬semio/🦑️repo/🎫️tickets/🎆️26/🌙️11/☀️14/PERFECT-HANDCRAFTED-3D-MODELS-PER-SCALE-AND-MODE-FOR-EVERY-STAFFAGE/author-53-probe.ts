import { openCtx, drawn, type Json } from "./kit-lib.ts";
const [spec, sc, filt] = [process.argv[2]!, Number(process.argv[3] ?? 50), process.argv[4]];
const c = openCtx(spec, sc);
console.log("feature", c.feature);
for (const [id, f] of Object.entries(c.state.frames)) if (!filt || id.includes(filt)) console.log("frame", id, "o", f.o.map((x) => +x.toFixed(2)), "u", f.u.map((x) => +x.toFixed(2)), "v", f.v.map((x) => +x.toFixed(2)), "w", f.w.map((x) => +x.toFixed(2)));
for (const view of ["elevation", "front", "plan"] as const) {
  console.log("==", view);
  for (const s of (c.state[view] as Json[])) {
    const pts = s.rings.flat();
    if (!pts.length) continue;
    const xs = pts.map((p: Json) => p.x), ys = pts.map((p: Json) => p.y);
    const f = (a: number[]) => [Math.min(...a), Math.max(...a)].map((x) => +x.toFixed(2));
    console.log(s.shape.id.padEnd(16), s.shape.bone.padEnd(14), s.shape.role.padEnd(8), s.closed ? "C" : "o", s.shape.filled ? "F" : "-", "n" + pts.length, "x", f(xs), "y", f(ys), s.shape.region ?? "");
  }
}

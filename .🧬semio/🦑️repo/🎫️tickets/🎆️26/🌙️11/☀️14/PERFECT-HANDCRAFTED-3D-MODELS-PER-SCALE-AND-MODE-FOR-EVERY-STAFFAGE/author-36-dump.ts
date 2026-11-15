import { openCtx } from "./kit-lib.ts";
const [spec, sc] = [process.argv[2]!, Number(process.argv[3] ?? 50)];
const c = openCtx(spec, sc);
const r = (x: number) => (Math.round(x * 1000) / 1000).toString();
for (const [b, f] of Object.entries(c.state.frames as Record<string, any>)) console.log("frame", b, "o", f.o.map(r).join(","), "u", f.u.map(r).join(","), "v", f.v.map(r).join(","), "w", f.w.map(r).join(","));
for (const view of ["elevation", "front", "plan"] as const) {
  console.log("## " + view);
  for (const s of c.state[view] as any[]) {
    const pts = s.rings.flat();
    if (!pts.length) { console.log(s.shape.id, "-"); continue; }
    const xs = pts.map((p: any) => p.x), ys = pts.map((p: any) => p.y);
    console.log(`${s.shape.id} | ${s.shape.bone} | ${s.shape.role} | ${s.closed ? "closed" : "open"} | x ${r(Math.min(...xs))}..${r(Math.max(...xs))} y ${r(Math.min(...ys))}..${r(Math.max(...ys))}`);
  }
}

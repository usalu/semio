import { openCtx } from "./kit-lib.ts";
import { writeFileSync } from "node:fs";
const out: any[] = [];
for (const scale of [100, 200]) {
  const c = openCtx("animals/mallard-duck", scale);
  const f: any = c.state.frames["shank-f"], ta: any = c.state.frames["tarsus-f"], ft: any = c.state.frames["foot-f"];
  const loc = (p: number[]) => { const d = [p[0]! - f.o[0], p[1]! - f.o[1]]; return [Math.round((d[0]! * f.u[0] + d[1]! * f.u[1]) * 1e5) / 1e5, Math.round((d[0]! * f.v[0] + d[1]! * f.v[1]) * 1e5) / 1e5]; };
  const pts = [loc([f.o[0], f.o[1]]), loc([ta.o[0], ta.o[1]]), loc([ft.o[0], ft.o[1]]), loc([ft.o[0] + 0.082, ft.o[1]])];
  out.push({ scale: String(scale), view: "elevation", id: "far-leg", clone: "near-leg", set: { bone: "shank-f", points: pts, tone: 0.4 } });
}
console.log(JSON.stringify(out));
writeFileSync("🗑️generated/author-29/duck-farleg.json", JSON.stringify(out));

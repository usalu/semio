import { drawn, flag, openCtx } from "./kit-lib.ts";
const a = process.argv.slice(2);
const c = openCtx(a[0]!, Number(flag(a, "--scale", "50")));
const only = flag(a, "--bones", "");
for (const view of ["elevation", "front", "plan"] as const) {
  for (const s of drawn(c, view)) {
    if (only && !only.split(",").includes(s.bone)) continue;
    let x0 = 9, x1 = -9, y0 = 9, y1 = -9;
    for (const r of s.rings) for (const p of r) { x0 = Math.min(x0, p.x); x1 = Math.max(x1, p.x); y0 = Math.min(y0, p.y); y1 = Math.max(y1, p.y); }
    console.log(view.padEnd(9), s.bone.padEnd(11), s.id.padEnd(14), `x ${x0.toFixed(3)}..${x1.toFixed(3)}  y ${y0.toFixed(3)}..${y1.toFixed(3)}`);
  }
}

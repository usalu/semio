import { openCtx, drawn, flag } from "./kit-lib.ts";
const a = process.argv.slice(2);
const c = openCtx(a[0]!, Number(flag(a, "--scale")));
const f = (x: readonly number[]) => x.map((q) => q.toFixed(3)).join(",");
if (a.includes("--frames")) for (const [b, fr] of Object.entries<any>(c.state.frames)) console.log(b.padEnd(14), "o", f(fr.o), "u", f(fr.u), "v", f(fr.v), "w", f(fr.w));
const bb = (view: any, bone?: string) => {
  for (const s of drawn(c, view, bone)) {
    let x0 = 1e9, x1 = -1e9, y0 = 1e9, y1 = -1e9;
    for (const r of s.rings) for (const p of r) { x0 = Math.min(x0, p.x); x1 = Math.max(x1, p.x); y0 = Math.min(y0, p.y); y1 = Math.max(y1, p.y); }
    console.log(view.padEnd(9), s.id.padEnd(16), s.bone.padEnd(14), "x", x0.toFixed(3), x1.toFixed(3), "y", y0.toFixed(3), y1.toFixed(3));
  }
};
if (a.includes("--bbox")) for (const v of ["elevation", "front", "plan"]) bb(v);

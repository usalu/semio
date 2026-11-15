import { openCtx, drawn } from "./kit-lib.ts";
const [spec, scaleS, bonesArg] = process.argv.slice(2);
const c = openCtx(spec!, Number(scaleS));
console.log("dims", JSON.stringify(c.m.dimensions), "feature", c.feature);
for (const b of Object.keys(c.state.frames)) { const f: any = c.state.frames[b]; console.log("bone", b, "o", f.o.map((x: number) => +x.toFixed(4)).join(","), "u", f.u.map((x: number) => +x.toFixed(3)).join(",")); }
for (const v of ["elevation", "front", "plan"] as const) for (const s of drawn(c, v)) {
  const xs: number[] = [], ys: number[] = [];
  for (const r of s.rings) for (const p of r) { xs.push(p.x); ys.push(p.y); }
  const f = (a: number[]) => `${Math.min(...a).toFixed(3)}..${Math.max(...a).toFixed(3)}`;
  console.log(v, s.id, s.bone, s.role, "x", f(xs), "y", f(ys), "n", xs.length);
}

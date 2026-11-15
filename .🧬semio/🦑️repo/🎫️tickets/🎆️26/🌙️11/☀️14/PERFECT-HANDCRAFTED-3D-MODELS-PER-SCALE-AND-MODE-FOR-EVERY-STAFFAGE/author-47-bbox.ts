import { openCtx, flag } from "./kit-lib.ts";
const a = process.argv.slice(2);
const c = openCtx(a[0]!, Number(flag(a, "--scale")));
for (const s of c.vol.body as any[]) {
  if (s.kind !== "loft") continue;
  const f = c.state.frames[s.bone]!;
  let zmin = 1e9, zmax = -1e9;
  for (const q of s.sections) { const w = f.o[2]! + q.at[0] * f.u[2]! + q.at[1] * f.v[2]! + q.at[2] * f.w[2]!; zmin = Math.min(zmin, w); zmax = Math.max(zmax, w); }
  if (Math.abs(zmax) > 0.3 || Math.abs(zmin) > 0.3) console.log(s.id, zmin.toFixed(3), zmax.toFixed(3));
}

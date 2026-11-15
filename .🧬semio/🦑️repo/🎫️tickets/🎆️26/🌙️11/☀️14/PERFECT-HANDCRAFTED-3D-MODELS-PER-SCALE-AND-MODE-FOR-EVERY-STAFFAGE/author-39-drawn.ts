import { openCtx, drawn } from "./kit-lib.ts";
const [spec, scale] = [process.argv[2]!, Number(process.argv[3])];
const c = openCtx(spec, scale);
for (const view of ["elevation", "front", "plan"] as const) {
  console.log("#", view);
  for (const s of drawn(c, view)) {
    const xs = s.rings.flatMap((r) => r.map((p) => p.x)), ys = s.rings.flatMap((r) => r.map((p) => p.y));
    const f = (n: number) => n.toFixed(3);
    console.log(s.id.padEnd(24), s.bone.padEnd6 ?? s.bone, s.role, "x", f(Math.min(...xs)), f(Math.max(...xs)), "y", f(Math.min(...ys)), f(Math.max(...ys)));
  }
}

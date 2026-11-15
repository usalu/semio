import { openCtx, flag } from "./kit-lib.ts";
import { scaleMeshes } from "./volumes-lib.ts";
const a = process.argv.slice(2);
const sc = Number(flag(a, "--scale"));
const c = openCtx(a[0]!, sc);
const m: any = scaleMeshes(c.m, sc, c.vol);
const bb = (list: any[]) => {
  const lo = [1e9, 1e9, 1e9], hi = [-1e9, -1e9, -1e9];
  for (const s of list) for (let i = 0; i < s.world.length; i += 3) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, s.world[i + k]!); hi[k] = Math.max(hi[k]!, s.world[i + k]!); }
  return [lo, hi];
};
const [bl, bh] = bb(m.body);
console.log("body", bl.map((x) => x.toFixed(3)), bh.map((x) => x.toFixed(3)));
for (const list of ["thermal", "structure"] as const) for (const s of m[list]) {
  const [l, h] = bb([s]);
  const out = [0, 1, 2].map((k) => Math.max(bl[k]! - l[k]!, h[k]! - bh[k]!));
  if (Math.max(...out) > 0.002) console.log(list, s.id, "outside", out.map((x) => x.toFixed(3)).join(","));
}

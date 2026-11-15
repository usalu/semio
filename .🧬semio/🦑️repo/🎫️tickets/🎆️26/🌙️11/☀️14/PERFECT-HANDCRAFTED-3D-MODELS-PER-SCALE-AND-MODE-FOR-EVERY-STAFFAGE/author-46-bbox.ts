import { openCtx, flag } from "./kit-lib.ts";
import { scaleMeshes } from "./volumes-lib.ts";
const a = process.argv.slice(2);
const sc = Number(flag(a, "--scale"));
const c = openCtx(a[0]!, sc);
const ids = (flag(a, "--ids") ?? "").split(",");
const m: any = scaleMeshes(c.m, sc, c.vol);
for (const s of m.body) {
  if (!ids.includes(s.id)) continue;
  const v: number[] = s.world;
  const lo = [1e9, 1e9, 1e9], hi = [-1e9, -1e9, -1e9];
  for (let i = 0; i < v.length; i += 3) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, v[i + k]!); hi[k] = Math.max(hi[k]!, v[i + k]!); }
  console.log(s.id, "min", lo.map((x) => x.toFixed(3)).join(","), "max", hi.map((x) => x.toFixed(3)).join(","));
}

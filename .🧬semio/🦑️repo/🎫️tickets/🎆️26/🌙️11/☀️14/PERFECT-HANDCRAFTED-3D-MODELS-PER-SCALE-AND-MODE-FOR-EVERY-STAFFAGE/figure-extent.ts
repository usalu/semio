/** 📐️ World extent of every body solid at one scale (which solid sets the min and max of each axis): bun figure-extent.ts <asset> --scale N */
import { openCtx, flag } from "./kit-lib.ts";
import { scaleMeshes } from "./volumes-lib.ts";
const args = process.argv.slice(2);
const c = openCtx(args[0]!, Number(flag(args, "--scale", "50")));
const sm = scaleMeshes(c.m, c.scale, c.m.volumes[String(c.scale)]);
const rows = sm.body.map((m: any, i: number) => {
  const p: Float64Array = m.world; const lo = [1e9, 1e9, 1e9]; const hi = [-1e9, -1e9, -1e9];
  for (let k = 0; k < p.length; k += 3) for (let a = 0; a < 3; a++) { lo[a] = Math.min(lo[a]!, p[k + a]!); hi[a] = Math.max(hi[a]!, p[k + a]!); }
  return { id: m.id, lo, hi };
});
for (const [a, n] of [[0, "x"], [1, "y"], [2, "z"]] as const) {
  const mn = rows.reduce((b, r) => (r.lo[a]! < b.lo[a]! ? r : b));
  const mx = rows.reduce((b, r) => (r.hi[a]! > b.hi[a]! ? r : b));
  console.log(`${n}: min ${mn.lo[a]!.toFixed(4)} (${mn.id})  max ${mx.hi[a]!.toFixed(4)} (${mx.id})  extent ${(mx.hi[a]! - mn.lo[a]!).toFixed(4)}`);
}

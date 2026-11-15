/** 📏️ World extents (x,y,z) of every body solid of a scale: bun author-69-zext.ts <asset> <scale> [bone-prefix] */
import { entryOf, loadSource, merged, scaleMeshes } from "./volumes-lib.ts";
const [spec, sc, pre] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!);
const sm = scaleMeshes(m, Number(sc), m.volumes[sc]);
const f = (x: number) => x.toFixed(3);
let all = [Infinity, -Infinity];
sm.body.forEach((mesh, i) => {
  const s = sm.solids.body[i]!;
  if (pre && !String(s.bone).startsWith(pre) && !String(s.id).startsWith(pre)) return;
  const lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity];
  for (let k = 0; k < mesh.world.length; k += 3) for (let a = 0; a < 3; a++) { lo[a] = Math.min(lo[a]!, mesh.world[k + a]!); hi[a] = Math.max(hi[a]!, mesh.world[k + a]!); }
  all = [Math.min(all[0]!, lo[2]!), Math.max(all[1]!, hi[2]!)];
  console.log(`${s.id.padEnd(22)} ${String(s.bone).padEnd(14)} x ${f(lo[0]!)}..${f(hi[0]!)} y ${f(lo[1]!)}..${f(hi[1]!)} z ${f(lo[2]!)}..${f(hi[2]!)}`);
});
console.log("z overall", f(all[0]!), f(all[1]!));

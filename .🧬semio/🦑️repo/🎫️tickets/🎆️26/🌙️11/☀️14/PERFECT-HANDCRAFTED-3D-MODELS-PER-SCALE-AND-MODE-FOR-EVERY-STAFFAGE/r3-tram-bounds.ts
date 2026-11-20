/** 📦️ Round 3 tram: world bounds of the body solids whose id contains a text (`bun r3-tram-bounds.ts 50 motor`). */
import { entryOf, loadSource, merged, scaleMeshes } from "./volumes-lib.ts";

const [scale, part] = [Number(process.argv[2] ?? 50), process.argv[3] ?? ""];
const { entry, asset } = entryOf("vehicles/tram");
const a = merged(asset, loadSource(entry)!);
const sm = scaleMeshes(a, scale, a.volumes[String(scale)]);
sm.solids.body.forEach((s, i) => {
  if (!String(s.id).includes(part) || !sm.body[i]) return;
  const w = sm.body[i]!.world;
  const lo = [Infinity, Infinity, Infinity];
  const hi = [-Infinity, -Infinity, -Infinity];
  for (let k = 0; k < w.length; k++) {
    lo[k % 3] = Math.min(lo[k % 3]!, w[k]!);
    hi[k % 3] = Math.max(hi[k % 3]!, w[k]!);
  }
  console.log(`[DEBUG] ${s.id} x ${lo[0]!.toFixed(3)}..${hi[0]!.toFixed(3)} y ${lo[1]!.toFixed(3)}..${hi[1]!.toFixed(3)} z ${lo[2]!.toFixed(3)}..${hi[2]!.toFixed(3)}`);
});

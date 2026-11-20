/** 🗼️ Prints the rest frames and tips of every bone of the Lombardy poplar (world x ahead, y up, z lateral) and the member bones of every scale. */
import { entryOf, frames, memberBones, toWorld } from "./volumes-lib.ts";
const { asset } = entryOf("vegetation/lombardy-poplar");
const f = frames(asset);
const r = (n: number) => n.toFixed(3);
for (const b of asset.rig.bones) {
  const fr = f[b.id]!;
  const tip = toWorld(fr, [b.lengthM, 0, 0]);
  console.log(b.id.padEnd(12), "o", fr.o.map(r).join(" "), "tip", tip.map(r).join(" "), "u", fr.u.map(r).join(" "), "v", fr.v.map(r).join(" "), "w", fr.w.map(r).join(" "));
}
for (const s of [50, 100, 200, 500]) console.log(s, memberBones(asset, s).join(" "));

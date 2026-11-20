/** 🧭️ Prints the rest frames and tips of every bone of the ivy on trellis (world x ahead, y up, z lateral). */
import { entryOf, frames, projectWorld, toWorld } from "./volumes-lib.ts";
const { asset } = entryOf("vegetation/ivy-on-trellis");
const f = frames(asset);
const r = (n: number) => n.toFixed(3);
for (const b of asset.rig.bones) {
  const fr = f[b.id]!;
  const tip = toWorld(fr, [b.lengthM, 0, 0]);
  console.log(b.id.padEnd(12), "o", fr.o.map(r).join(" "), "tip", tip.map(r).join(" "), "u", fr.u.map(r).join(" "), "v", fr.v.map(r).join(" "), "w", fr.w.map(r).join(" "));
}
for (const view of ["elevation", "front", "plan"] as const) console.log(view, JSON.stringify(projectWorld(view, [1, 2, 3])));

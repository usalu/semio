/** 🔎 Prints the world frames of chosen bones of `humans/person-with-umbrella` for every pose (catalog asset as it is).
 *
 * Usage: `bun r3-person-with-umbrella-probe.ts [bone ...]` */
import { rigAt } from "./r3-lib.ts";
import { entryOf } from "./volumes-lib.ts";
const { asset } = entryOf("humans/person-with-umbrella");
const want = process.argv.slice(2);
const bones = want.length > 0 ? want : ["torso", "upper-arm-r", "forearm-r", "hand-r", "umbrella", "rib-front", "foot-r", "head"];
const f = (x: number): string => x.toFixed(3);
for (const p of asset.poses) {
  const rig = rigAt(asset, 50, { kind: "pose", pose: p.id });
  console.log(p.id);
  for (const b of bones) {
    const fr = rig.frames[b]!;
    console.log("  ", b.padEnd(14), "o", fr.o.map(f).join(" "), "u", fr.u.map(f).join(" "), "deg", ((Math.atan2(fr.u[1], fr.u[0]) * 180) / Math.PI).toFixed(1), "w", fr.w.map(f).join(" "));
  }
}

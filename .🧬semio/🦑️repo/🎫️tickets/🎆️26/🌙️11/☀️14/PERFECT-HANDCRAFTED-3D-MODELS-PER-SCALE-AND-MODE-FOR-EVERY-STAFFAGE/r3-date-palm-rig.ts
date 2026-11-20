/** 🌴️ Prints the evaluated frames of every date-palm bone in a pose (world x ahead, y up, z left), with azimuth and elevation of each bone direction. */
import { load, rigAt } from "./r3-lib.ts";
const { asset } = load("vegetation/date-palm", "r3-date-palm-rig.ts [pose]");
const rig = rigAt(asset, 50, { kind: "pose", pose: process.argv[2] ?? "calm" });
const r = (n: number) => n.toFixed(3).padStart(7);
for (const b of rig.bones) {
  const f = rig.frames[b.id]!;
  const az = (Math.atan2(-f.u[2], f.u[0]) * 180) / Math.PI;
  const el = (Math.asin(Math.max(-1, Math.min(1, f.u[1]))) * 180) / Math.PI;
  console.log(b.id.padEnd(10), "o", b.o.map(r).join(""), " tip", b.tip.map(r).join(""), " az", r(az), " el", r(el), " v", f.v.map(r).join(""), " w", f.w.map(r).join(""));
}

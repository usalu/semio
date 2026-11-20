/** 🔎️ Probe of the round 3 seated reader: rest frames of every bone and the joint positions of a pose (`bun r3-seated-reader-probe.ts [pose]`). */
import { framesOf, rooted } from "./r3-seated-reader-roots.ts";
import { restFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
const document = rooted([]);
const rig = document.asset.rig;
const f3 = (p: readonly number[]): string => p.map((x) => x.toFixed(3)).join(",");
const pose = process.argv[2];
if (pose === undefined) for (const [id, b] of Object.entries(restFrames(rig) as Record<string, any>)) console.log(id.padEnd(22), "o", f3(b.frame.o), "u", f3(b.frame.u), "v", f3(b.frame.v), "w", f3(b.frame.w));
else {
  const p = document.asset.poses.find((q: any) => q.id === pose);
  for (const [id, f] of Object.entries(framesOf(rig, p.bones))) console.log(id.padEnd(22), "o", f3(f.o), "u", f3(f.u), "v", f3(f.v), "w", f3(f.w));
}

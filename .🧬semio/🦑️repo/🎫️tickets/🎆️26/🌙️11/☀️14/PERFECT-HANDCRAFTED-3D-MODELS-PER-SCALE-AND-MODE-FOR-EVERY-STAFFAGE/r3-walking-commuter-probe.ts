/** 🔬️ Prints the bone frames of a pose and the anchors in world space for `humans/walking-commuter`. */
import { entryOf, loadSource, merged, poseState } from "./volumes-lib.ts";
const { entry, asset } = entryOf("humans/walking-commuter");
const m = merged(asset, loadSource(entry)!);
const pose = process.argv[2];
if (pose !== undefined) m.presentation = { ...m.presentation, defaultPose: pose };
const st = poseState(m, 50);
const r = (a: number[]) => a.map((x) => x.toFixed(4)).join(" ");
for (const [id, f] of Object.entries(st.frames)) console.log(id.padEnd(10), "o", r(f.o), "u", r(f.u), "v", r(f.v), "w", r(f.w));
for (const a of m.anchors) {
  const f = st.frames[a.bone]!;
  console.log("anchor", a.id, r([0, 1, 2].map((k) => f.o[k]! + a.x * f.u[k]! + a.y * f.v[k]! + (a.z ?? 0) * f.w[k]!)));
}

/** 🧭️ Prints the bone frames of the candidate document of `humans/runner-with-blade-prosthesis` in a pose (default: its default pose; `rest` for the rest pose of the rig) and the world position of every anchor.
 * Usage: `bun r3-runner-frames.ts [pose|rest]` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { restFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { poseState } from "./volumes-lib.ts";
import { OUT } from "./r3-runner-roots.ts";
const m = JSON.parse(readFileSync(join(OUT, "🔣️.json"), "utf8")).asset;
const pose = process.argv[2];
const r = (a: readonly number[]) => a.map((x) => x.toFixed(4)).join(" ");
if (pose === "rest") {
  for (const [id, b] of Object.entries(restFrames(m.rig) as Record<string, { frame: { o: number[]; u: number[]; v: number[]; w: number[] } }>)) console.log(id.padEnd(12), "o", r(b.frame.o), "u", r(b.frame.u), "v", r(b.frame.v), "w", r(b.frame.w));
} else {
  if (pose !== undefined) m.presentation = { ...m.presentation, defaultPose: pose };
  const st = poseState(m, 50);
  for (const [id, f] of Object.entries(st.frames)) console.log(id.padEnd(12), "o", r(f.o), "u", r(f.u), "v", r(f.v), "w", r(f.w));
  for (const a of m.anchors) {
    const f = st.frames[a.bone]!;
    console.log("anchor", a.id, r([0, 1, 2].map((k) => f.o[k]! + a.x * f.u[k]! + a.y * f.v[k]! + (a.z ?? 0) * f.w[k]!)));
  }
}

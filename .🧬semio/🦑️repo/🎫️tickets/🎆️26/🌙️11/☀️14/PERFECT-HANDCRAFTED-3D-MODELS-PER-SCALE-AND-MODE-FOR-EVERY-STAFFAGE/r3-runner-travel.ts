/** 👣️ Prints the planted travel of `run-cycle` of the candidate document, the root x and y per frame and the height of the contact anchors over the ground, to tune the stride kinematics of `r3-runner-rig.ts`.
 * Usage: `bun r3-runner-travel.ts [--frames]` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { sampleClip } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { OUT } from "./r3-runner-roots.ts";

const asset = JSON.parse(readFileSync(join(OUT, "🔣️.json"), "utf8")).asset;
const clip = asset.clips.find((c: any) => c.id === "run-cycle");
const travel = sampleClip(asset, clip, clip.durationS - 1e-9).root.x - sampleClip(asset, clip, 0).root.x;
console.log("travel", travel.toFixed(4), "advanceM", clip.advanceM);
if (process.argv.includes("--frames"))
  for (let k = 0; k <= 42; k++) {
    const t = k / 60;
    const st = evaluate(asset as never, { kind: "clip", clip: "run-cycle", timeS: t } as never) as any;
    const at = (id: string): string => {
      const a = asset.anchors.find((x: any) => x.id === id);
      const f = st.bones[a.bone].frame;
      return `${(f.o.x + a.x * f.u.x + a.y * f.v.x).toFixed(3)},${(f.o.y + a.x * f.u.y + a.y * f.v.y).toFixed(3)}`;
    };
    console.log(t.toFixed(3), "root", st.bones.pelvis.frame.o.x.toFixed(3), st.bones.pelvis.frame.o.y.toFixed(3), "ball", at("ball-l"), "heel", at("heel-l"), "blade", at("blade-toe"));
  }

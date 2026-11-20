/** 🧵 Prints the world direction of the trunk and right arm bones around the loop seam of the gait (catalog state). */
import { rigAt } from "./r3-lib.ts";
import { entryOf } from "./volumes-lib.ts";

const { asset } = entryOf("humans/elderly-with-cane");
const ang = (u: number[]): string => ((Math.atan2(u[1]!, u[0]!) * 180) / Math.PI).toFixed(2);
for (const t of [1.6433, 1.6717, 1.68, 1.69, 1.699, 1.6999, 0, 0.0001, 0.01, 0.0283]) {
  const rig = rigAt(asset, 50, { kind: "clip", clip: "cane-gait", timeS: t });
  console.log(t, ["pelvis", "torso", "arm-r", "forearm-r", "hand-r", "cane", "thigh-l"].map((b) => `${b}=${ang(rig.frames[b]!.u)}`).join(" "), "hip", rig.frames.pelvis!.o.map((x) => x.toFixed(4)).join(","));
}

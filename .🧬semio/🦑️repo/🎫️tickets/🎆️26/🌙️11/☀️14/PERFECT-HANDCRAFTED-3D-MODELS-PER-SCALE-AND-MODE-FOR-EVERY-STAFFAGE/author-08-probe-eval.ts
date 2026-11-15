import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { entryOf, frames } from "./volumes-lib.ts";
const { asset } = entryOf(process.argv[2]!);
const s: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } });
const rest = frames(asset);
for (const b of ["pelvis", "torso", "head", "arm-l", "arm-r", "hand-r", "cane", "thigh-l", "foot-l"]) {
  console.log(b, "pose", JSON.stringify(s.bones[b]?.frame), "rest", JSON.stringify(rest[b]));
}

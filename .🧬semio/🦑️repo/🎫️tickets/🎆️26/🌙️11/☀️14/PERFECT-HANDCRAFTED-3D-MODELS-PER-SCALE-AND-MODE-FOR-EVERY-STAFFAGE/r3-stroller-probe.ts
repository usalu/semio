/** 🔎️ Probe of the evaluated state of the stroller asset. */
import { entryOf, loadSource, merged, poseState } from "./volumes-lib.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const { entry, asset } = entryOf("humans/parent-with-stroller");
const m = merged(asset, loadSource(entry));
const st: any = evaluate(m, { kind: "pose", pose: m.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } });
console.log(Object.keys(st));
console.log(JSON.stringify(st.shapes.elevation[0]).slice(0, 900));
console.log(JSON.stringify(st.bones["torso"] ?? Object.values(st.bones)[0]).slice(0, 600));
const ps = poseState(m, 50);
for (const [k, f] of Object.entries(ps.frames)) console.log(k, JSON.stringify(f, (_, v) => (typeof v === "number" ? Math.round(v * 1e4) / 1e4 : v)));

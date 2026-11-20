/** 🧭️ Prints the bone frames of the working hedgehog in a pose (default the default pose). Usage: bun r3-hedgehog-frames.ts [pose] */
import { rigAt } from "./r3-lib.ts";
import { working } from "./r3-hedgehog-work.ts";
const a = working();
const pose = process.argv[2] ?? a.presentation.defaultPose;
const rig = rigAt(a, 50, { kind: "pose", pose });
for (const b of rig.bones) console.log(b.id.padEnd(12), "o", b.o.map((x) => x.toFixed(4)).join(", "), "| tip", b.tip.map((x) => x.toFixed(4)).join(", "));

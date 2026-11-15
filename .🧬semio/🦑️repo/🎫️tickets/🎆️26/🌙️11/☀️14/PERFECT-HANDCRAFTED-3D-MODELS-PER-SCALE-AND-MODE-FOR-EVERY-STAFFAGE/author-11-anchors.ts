import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { entryOf } from "./volumes-lib.ts";
const { asset } = entryOf(process.argv[2]!);
const pose = process.argv[3] ?? asset.presentation.defaultPose;
const st = evaluate(asset, { kind: "pose", pose } as never, { kind: "scale", scale: { denominator: 50 } } as never) as never as { anchors: Record<string, { anchor: { id: string }; at: { x: number; y: number }; z: number }> };
for (const a of Object.values(st.anchors)) console.log(a.anchor.id, a.at.x.toFixed(4), a.at.y.toFixed(4), a.z);

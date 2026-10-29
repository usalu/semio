import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import doc from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/🔣️.json";
for (const a of (doc as any).assets.filter((x: any) => ["running-child", "walking-commuter"].includes(x.id))) {
  const p = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }).bounds;
  const c = evaluate(a, { kind: "clip", clip: a.presentation.defaultClip, timeS: 0.3 }).bounds;
  console.log(a.id, JSON.stringify(p), JSON.stringify(c));
}

import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [, , id, ...bones] = process.argv;
const a = catalog().find((x) => x.id === id)!;
const st: any = evaluate(a, { kind: "pose", pose: "rest" in {} ? "" : a.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } } as any);
for (const b of bones) console.log(b, JSON.stringify(st.bones[b].origin), st.bones[b].angleDeg ?? "", JSON.stringify(st.bones[b].tip));

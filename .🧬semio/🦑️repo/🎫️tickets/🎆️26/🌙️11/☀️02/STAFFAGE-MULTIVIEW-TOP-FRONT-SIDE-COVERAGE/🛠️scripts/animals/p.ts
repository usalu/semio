import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const a: any = catalog().find((x) => x.id === "dairy-cow")!;
const st: any = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } } as any);
console.log(a.presentation.defaultPose, JSON.stringify(st.bones["forearm-fn"]).slice(0, 600));
console.log(JSON.stringify(st.shapes.front.find((s: any) => s.shape.id === "near-forearm")?.rings).slice(0,400));
console.log(JSON.stringify(st.shapes.plan.find((s: any) => s.shape.id === "near-forearm")?.rings).slice(0,400));

import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [, , id, scale, view, axis] = process.argv;
const a = catalog().find((x) => x.id === id)!;
const st: any = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: +scale } } as any);
const rows = st.shapes[view].filter((s: any) => s.shape.role !== "ground").map((s: any) => { const v = s.rings.flat().map((p: any) => p[axis]); return { id: s.shape.id, bone: s.shape.bone, kind: s.shape.solid?.kind ?? s.shape.kind, lo: Math.min(...v), hi: Math.max(...v) }; });
rows.sort((x: any, y: any) => x.lo - y.lo); console.log("min:", rows.slice(0, 3).map((r: any) => `${r.id}/${r.bone}/${r.kind} ${r.lo.toFixed(3)}`).join(" | "));
rows.sort((x: any, y: any) => y.hi - x.hi); console.log("max:", rows.slice(0, 3).map((r: any) => `${r.id}/${r.bone}/${r.kind} ${r.hi.toFixed(3)}`).join(" | "));

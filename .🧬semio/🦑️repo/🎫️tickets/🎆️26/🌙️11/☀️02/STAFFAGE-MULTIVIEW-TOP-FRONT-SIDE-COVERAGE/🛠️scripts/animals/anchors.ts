import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [, , ...ids] = process.argv;
for (const a of catalog().filter((x) => ids.includes(x.id))) {
  const st: any = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } } as any);
  const E = (s: any[]) => { const p = s.filter((x) => x.shape.role !== "ground").flatMap((x) => x.rings.flat()); return [Math.min(...p.map((q: any) => q.x)), Math.max(...p.map((q: any) => q.x)), Math.min(...p.map((q: any) => q.y)), Math.max(...p.map((q: any) => q.y))].map((v) => +v.toFixed(3)); };
  const th: any = (a as any).thermal;
  const em = (th.emitters ?? []).map((e: any) => `${e.id}@${e.anchor ?? e.anchorId}`);
  console.log(`## ${a.id} elev[${E(st.shapes.elevation)}] front[${E(st.shapes.front)}] plan[${E(st.shapes.plan)}] emitters ${em.join(",")}`);
  for (const an of a.anchors.filter((x) => x.kind === "emitter")) { const w = st.anchors?.[an.id]; console.log(`   ${an.id} bone=${an.bone} local(${an.x},${an.y}) z=${(an as any).z ?? "-"} world=${w ? JSON.stringify(w) : "?"}`); }
}

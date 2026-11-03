import { loadCatalog, extentOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🧰️.ts";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
for (const a of loadCatalog().filter((x) => ["elderly-with-cane", "person-with-umbrella", "worker-carrying-box", "person-with-white-cane", "parent-carrying-toddler"].includes(x.id))) {
  const s = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } });
  const cen = (v: "front" | "plan") => s.shapes[v].filter((x) => x.shape.role !== "ground").map((x) => { const e = extentOf([x])!; return [x.shape.id, +(v === "front" ? (e.minX + e.maxX) / 2 : (e.minY + e.maxY) / 2).toFixed(3)]; }).filter((x) => Math.abs(x[1] as number) > 0.03).slice(0, 6);
  console.log(a.id, a.presentation.defaultPose, "front", JSON.stringify(cen("front")), "plan", JSON.stringify(cen("plan")), "anchorsZ", JSON.stringify(Object.entries(s.anchors).filter(([, v]: any) => Math.abs(v.z ?? 0) > 0.01).map(([k, v]: any) => [k, v.z])));
}

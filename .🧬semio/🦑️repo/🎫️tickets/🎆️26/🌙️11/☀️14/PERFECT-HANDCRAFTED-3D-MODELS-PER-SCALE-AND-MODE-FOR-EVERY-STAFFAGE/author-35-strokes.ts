import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const [spec, scaleS, view, filter] = process.argv.slice(2) as [string, string, string, string?];
const { entry, asset } = entryOf(spec);
const m = merged(asset, loadSource(entry)!) as any;
const st = evaluate(m, { kind: "pose", pose: m.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scaleS) } }) as any;
for (const s of st.shapes[view]) {
  if (!["body", "detail", "glazing"].includes(s.shape.role)) continue;
  if (filter && !s.shape.id.startsWith(filter) && s.shape.bone !== filter) continue;
  const pts = s.rings.map((r: any[]) => r.map((p) => `${p.x.toFixed(3)},${p.y.toFixed(3)}`).join(" "));
  console.log(s.shape.id, s.shape.bone, s.closed ? "closed" : "open", s.shape.filled === false ? "unfilled" : "filled", "pen", s.shape.penMm, "|", pts.map((p: string) => p.slice(0, 400)).join(" // "));
}

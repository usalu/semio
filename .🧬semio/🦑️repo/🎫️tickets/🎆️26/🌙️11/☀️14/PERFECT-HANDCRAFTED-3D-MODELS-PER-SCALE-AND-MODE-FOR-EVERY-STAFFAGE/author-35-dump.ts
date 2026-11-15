import { writeFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const [spec, out] = process.argv.slice(2) as [string, string];
const { entry, asset } = entryOf(spec);
const m = merged(asset, loadSource(entry)!) as any;
const res: any = {};
for (const sc of [50, 100, 200, 500]) {
  let st: any;
  try { st = evaluate(m, { kind: "pose", pose: m.presentation.defaultPose }, { kind: "scale", scale: { denominator: sc } }) as any; } catch { continue; }
  res[sc] = {};
  for (const view of ["elevation", "front", "plan"]) {
    res[sc][view] = {};
    for (const s of st.shapes[view]) {
      if (!["body", "detail", "glazing"].includes(s.shape.role)) continue;
      res[sc][view][s.shape.id] = { bone: s.shape.bone, closed: s.closed, filled: s.shape.filled !== false, pen: s.shape.penMm ?? null, kind: s.shape.kind, rings: s.rings.map((r: any[]) => r.map((p) => [Number(p.x.toFixed(4)), Number(p.y.toFixed(4))])) };
    }
  }
}
writeFileSync(out, JSON.stringify(res));

import { readFileSync, writeFileSync } from "node:fs";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [file, out] = process.argv.slice(2);
const doc = JSON.parse(readFileSync(file!, "utf8"));
const IDS = ["worker-carrying-box", "parent-with-stroller", "jogger", "person-with-umbrella", "leaning-person", "tour-guide"];
const res: any = {};
for (const a of doc.assets) {
  if (!IDS.includes(a.id)) continue;
  res[a.id] = {};
  for (const p of a.poses) {
    res[a.id][p.id] = {};
    const st = evaluate(a, { kind: "pose", pose: p.id });
    for (const s of ["l", "r"]) {
      const toe = a.rig.bones.find((b: any) => b.id === `toe-${s}`);
      const foot = st.bones[`foot-${s}`], t = st.bones[`toe-${s}`];
      const clear = t.origin.y + toe.lengthM * Math.sin(foot.angleRad) >= 0.0003;
      const need = Math.asin(Math.max(-1, Math.min(1, (0.0003 - t.origin.y) / toe.lengthM))) - foot.angleRad;
      const deg = clear ? 0 : Math.max(0, Math.min(toe.joint.maxDeg, (need * 180) / Math.PI));
      res[a.id][p.id][s] = Math.round(deg * 100) / 100;
    }
  }
}
writeFileSync(out!, JSON.stringify(res, null, 1));

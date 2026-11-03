import { readdirSync, readFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const root = `${L}/🗂️catalog/🌳️vegetation`;
for (const d of readdirSync(root)) {
  const a = JSON.parse(readFileSync(`${root}/${d}/🔣️.json`, "utf8")).asset;
  const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: 100 } });
  console.log("##", d, a.rig.bones.length);
  for (const b of a.rig.bones) { const e = st.bones[b.id]; if (e.origin.y <= 1.6 && b.section) console.log(`  ${b.id} par ${b.parent} o(${e.origin.x.toFixed(2)},${e.origin.y.toFixed(2)}) tip(${e.tip.x.toFixed(2)},${e.tip.y.toFixed(2)}) r ${b.section.extremeFibreM} ${b.section.material}`); }
}

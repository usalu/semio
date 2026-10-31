import { readFileSync } from "node:fs";
import { SCALE_DENOMINATORS } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { validateAsset } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

for (const file of process.argv.slice(2)) {
  const asset = JSON.parse(readFileSync(file, "utf8")).asset;
  console.log(`== ${asset.id}`);
  const issues = validateAsset(asset);
  for (const i of issues) console.log(`ISSUE ${i.code} ${i.path}: ${i.message.en}`);
  for (const d of SCALE_DENOMINATORS) {
    const rep = asset.representations[`${d}`];
    if (rep.status !== "authored") { console.log(`1:${d} unavailable`); continue; }
    const s = measureSuitability(asset, rep, d, DEFAULT_PRINT_PROFILE);
    console.log(`1:${d} ${s.suitable ? "OK" : "FAIL"} stroke ${s.minStrokeMm.toFixed(3)} gap ${s.minGapMm.toFixed(3)} feature ${s.minFeatureMm.toFixed(3)}`);
    for (const v of s.violations) console.log(`   ${v.check} ${v.shapes.join(",")} ${v.measuredMm.toFixed(3)} < ${v.requiredMm}`);
  }
  for (const p of asset.poses) {
    const st = evaluate(asset, { kind: "pose", pose: p.id }, { kind: "scale", scale: { denominator: asset.representations[`${asset.presentation.defaultScale}`].denominator } });
    const b = st.bounds;
    console.log(`pose ${p.id} bounds x ${b.minX.toFixed(3)}..${b.maxX.toFixed(3)} y ${b.minY.toFixed(3)}..${b.maxY.toFixed(3)} w ${(b.maxX-b.minX).toFixed(3)} h ${(b.maxY-b.minY).toFixed(3)}`);
  }
}

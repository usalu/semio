/** 📏️ The evaluated extents (mm) of every drawn shape of one pose at one scale, per view, for finding the shape behind a synchrony finding: bun ext.ts <id> <scale> <pose> [bone or id substring] */
import { join } from "node:path";
const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const [id, scale, pose, filter] = process.argv.slice(2);
const asset = engine.loadCatalog().find((a: any) => a.id === id);
const state = evaluate(asset, { kind: "pose", pose: pose === "-" ? asset.presentation.defaultPose : pose }, engine.scaleSelector(Number(scale)));
const mm = (v: number): string => (v * 1000).toFixed(0);
for (const view of ["elevation", "front", "plan"] as const) {
  const whole = engine.extentOf(state.shapes[view]);
  console.log(`${view}: x ${mm(whole.minX)}..${mm(whole.maxX)} y ${mm(whole.minY)}..${mm(whole.maxY)}`);
  for (const shape of state.shapes[view]) {
    if (shape.shape.role === "ground") continue;
    if (filter !== undefined && !shape.shape.id.includes(filter) && !shape.shape.bone.includes(filter)) continue;
    const e = engine.extentOf([shape]);
    if (e) console.log(`   ${shape.shape.id} [${shape.shape.bone}] x ${mm(e.minX)}..${mm(e.maxX)} y ${mm(e.minY)}..${mm(e.maxY)}`);
  }
}

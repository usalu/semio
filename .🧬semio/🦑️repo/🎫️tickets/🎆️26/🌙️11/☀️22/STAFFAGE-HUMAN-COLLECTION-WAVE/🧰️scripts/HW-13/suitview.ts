/** 🔬️ Which view of a scale holds a suitability violation of a shape: drops the shape from one view at a time and re-measures.
 *  bun suitview.ts <slug> <denominator> <shape> */
import { measureSuitability, DEFAULT_PRINT_PROFILE } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const [slug, d, id] = process.argv.slice(2);
const a = (await Bun.file(`/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/${slug}/🔣️.json`).json()).asset;
for (const v of ["elevation", "front", "plan"]) {
  const rep = structuredClone(a.representations[d!]);
  rep.geometry[v] = rep.geometry[v].filter((s: any) => s.id !== id);
  const s: any = measureSuitability(a, rep, Number(d), DEFAULT_PRINT_PROFILE);
  console.log(`without ${id} in ${v}:`, s.violations.map((x: any) => `${x.check} ${x.shapes.join("+")} ${x.measuredMm}`).join("; ") || "none");
}

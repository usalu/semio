import { loadDoc } from "./lib.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const [slug, sc] = process.argv.slice(2);
const full = loadDoc(slug!).asset;
for (const p of full.poses.filter((x: any) => !x.key)) {
  for (const view of ["elevation", "front", "plan"]) {
    const a = structuredClone(full); a.poses = a.poses.filter((x: any) => x.id === p.id || x.key);
    const rep = structuredClone(a.representations[sc!]); for (const v of ["elevation", "front", "plan"]) if (v !== view) rep.geometry[v] = rep.geometry[v].filter((s: any) => s.bone === "head").slice(0, 1);
    const s = measureSuitability(a, rep, Number(sc) as never, DEFAULT_PRINT_PROFILE);
    console.log(p.id, view, s.minFeatureMm.toFixed(3), s.violations.map((v: any) => v.shapes.join("+") + ":" + v.measuredMm).join(" "));
  }
}

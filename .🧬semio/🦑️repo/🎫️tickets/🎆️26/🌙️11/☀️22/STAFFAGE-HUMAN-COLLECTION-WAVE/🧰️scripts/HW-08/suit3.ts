import { loadDoc } from "./lib.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const [slug, sc] = process.argv.slice(2);
const full = loadDoc(slug!).asset;
for (const keep of [["standing-with-poles"], ["resting-on-poles"], ["uphill-stride"], ["standing-with-poles", "resting-on-poles", "uphill-stride"]]) {
  const a = structuredClone(full); a.poses = a.poses.filter((x: any) => keep.includes(x.id));
  const s = measureSuitability(a, a.representations[sc!], Number(sc) as never, DEFAULT_PRINT_PROFILE);
  console.log(keep.join(","), s.minFeatureMm.toFixed(3), s.violations.map((v: any) => v.shapes.join("+") + ":" + v.measuredMm).join(" "));
}

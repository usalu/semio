import { loadDoc } from "./lib.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const full = loadDoc("hiker-with-trekking-poles").asset;
const c = full.clips.find((x: any) => x.id === "uphill-walk");
for (const i of [4, 6, 8, 10, 12, 14, 22, 26]) {
  const a = structuredClone(full); const p = a.poses.find((x: any) => x.id === "uphill-stride"); p.bones = structuredClone(c.keys[i].bones); a.poses = [p];
  const res = ["500", "200", "100"].map((sc) => { const s = measureSuitability(a, a.representations[sc], Number(sc) as never, DEFAULT_PRINT_PROFILE); return `${sc}:${s.minFeatureMm.toFixed(3)}/${s.minGapMm.toFixed(2)}${s.suitable ? "" : "!"}`; });
  console.log(i, res.join(" "));
}

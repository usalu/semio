import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { measureSuitability, DEFAULT_PRINT_PROFILE } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const [, , scale, ...ids] = process.argv;
for (const a of catalog().filter((x) => ids.includes(x.id))) {
  const rep: any = (a.representations as any)[scale];
  if (rep.status !== "authored") { console.log(a.id, scale, "unavailable"); continue; }
  const s: any = measureSuitability(a, rep, +scale, DEFAULT_PRINT_PROFILE);
  console.log(a.id, scale, s.suitable ? "SUITABLE" : "UNSUITABLE", `stroke ${s.minStrokeMm.toFixed(3)} gap ${s.minGapMm.toFixed(3)} feature ${s.minFeatureMm.toFixed(3)} profile ${s.profile.id}`);
  for (const v of s.violations.slice(0, 6)) console.log("   ", JSON.stringify(v));
}

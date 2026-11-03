import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { measureSuitability, DEFAULT_PRINT_PROFILE } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const ids = (process.env.IDS ?? "").split(",").filter(Boolean);
for (const a of catalog()) { if (!["machinery","vehicles"].includes(a.category)) continue; if (ids.length && !ids.includes(a.id)) continue;
  for (const r of Object.values<any>(a.representations)) { if (r.status !== "authored") continue;
    const s: any = measureSuitability(a, r, r.denominator, DEFAULT_PRINT_PROFILE);
    if (!s.suitable) { console.log(`${a.id} 1:${r.denominator} UNSUITABLE ${s.violations.length} violations; min stroke ${s.minStrokeMm.toFixed(2)} gap ${s.minGapMm.toFixed(2)} feature ${s.minFeatureMm.toFixed(2)}`);
      const by = new Map<string, number>(); for (const v of s.violations) by.set(`${v.check}:${v.shapes.join("+")}`, (by.get(`${v.check}:${v.shapes.join("+")}`) ?? 0) + 1);
      for (const [k, n] of [...by].slice(0, Number(process.env.N ?? 12))) console.log("   ", k, n); } } }
console.log("done");

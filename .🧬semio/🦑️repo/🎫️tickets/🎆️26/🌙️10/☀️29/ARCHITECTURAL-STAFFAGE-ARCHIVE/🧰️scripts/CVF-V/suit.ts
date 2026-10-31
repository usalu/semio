import { readFileSync } from "node:fs";
import { measureSuitability, DEFAULT_PRINT_PROFILE } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const C = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation";
for (const id of process.argv.slice(2)) {
  const asset = JSON.parse(readFileSync(`${C}/${id}/🔣️.json`, "utf8")).asset;
  for (const k of ["500", "200", "100", "50"]) {
    const s: any = measureSuitability(asset, asset.representations[k], Number(k), DEFAULT_PRINT_PROFILE);
    console.log(id, k, s.suitable ?? s.status, "stroke", s.minStrokeMm, "gap", s.minGapMm, "feat", s.minFeatureMm, "viol", (s.violations ?? []).length);
    for (const v of (s.violations ?? []).slice(0, 12)) console.log("   ", JSON.stringify(v).slice(0, 160));
  }
}

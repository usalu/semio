/** 🧾 Print suitability of every authored representation of appliances/circulation-pump under the default print profile (why `r3-2d.ts` reports "no suitable representation"). */
import { DEFAULT_PRINT_PROFILE, measureSuitability, printProfile } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { loadAsset } from "./volumes-lib.ts";

const asset = (loadAsset(process.argv[2]) as any);
const profile = DEFAULT_PRINT_PROFILE as any;
for (const s of ["50", "100", "200", "500"]) {
  const r = measureSuitability(asset, asset.representations[s], Number(s), profile);
  console.log(s, JSON.stringify(r).slice(0, 1500));
}

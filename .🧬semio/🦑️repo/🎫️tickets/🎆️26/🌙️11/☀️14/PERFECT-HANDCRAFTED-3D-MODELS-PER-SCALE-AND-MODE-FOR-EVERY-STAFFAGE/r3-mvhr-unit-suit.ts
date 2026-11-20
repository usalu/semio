/** 🖨️ Round 3 MVHR unit: prints the print-suitability violations of every authored representation of an asset file under the default print profile, so an unsuitable scale can be traced to its shape.
 *
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts
 */
import { readFileSync } from "node:fs";

import { measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { defaultDepictionOptions } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";

const asset = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
for (const scale of ["50", "100", "200"]) {
  const profile = (defaultDepictionOptions as any)("light-line", Number(scale), "en").profile;
  const s: any = measureSuitability(asset, asset.representations[scale], Number(scale), profile);
  console.log(profile.id, scale, s.suitable, JSON.stringify(s.violations).slice(0, 900));
}

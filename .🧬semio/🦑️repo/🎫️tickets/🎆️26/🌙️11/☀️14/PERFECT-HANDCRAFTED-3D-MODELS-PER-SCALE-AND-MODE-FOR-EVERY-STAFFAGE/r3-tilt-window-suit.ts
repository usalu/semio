/** 🖨 Print suitability of the three authored drawings of a candidate tilt-window document, with every failing check. */
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const doc = JSON.parse(await Bun.file(process.argv[2]).text());
for (const s of ["50", "100", "200"]) console.log(s, JSON.stringify(measureSuitability(doc.asset, doc.asset.representations[s], Number(s), DEFAULT_PRINT_PROFILE)).slice(0, 2500));

/** 🔬️ Lists the print-suitability violations of an HW-13 asset at a scale (default profile), with view and position.
 *  bun suit.ts <slug> <denominator> */
import { measureSuitability, DEFAULT_PRINT_PROFILE } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const [slug, d] = process.argv.slice(2);
const a = (await Bun.file(`/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/${slug}/🔣️.json`).json()).asset;
const s: any = measureSuitability(a, a.representations[d!], Number(d), DEFAULT_PRINT_PROFILE);
console.log(JSON.stringify(s.violations, null, 1).slice(0, 3000));

import { L, load } from "./lib.ts";
const { measureSuitability, DEFAULT_PRINT_PROFILE } = await import(`${L}/🔨️modules/🖨️representation/🟦️.ts`);
const slug = process.argv[2]!; const den = process.argv[3]!;
const a = load(slug).asset;
const r = a.representations[den];
const s = measureSuitability(a, r, +den, DEFAULT_PRINT_PROFILE);
console.log(slug, den, "suitable", s.suitable, "minStroke", s.minStrokeMm, "minGap", s.minGapMm, "minFeature", s.minFeatureMm, JSON.stringify(DEFAULT_PRINT_PROFILE).slice(0, 400));
for (const v of s.violations.slice(0, 40)) console.log(JSON.stringify(v));
console.log("violations", s.violations.length);

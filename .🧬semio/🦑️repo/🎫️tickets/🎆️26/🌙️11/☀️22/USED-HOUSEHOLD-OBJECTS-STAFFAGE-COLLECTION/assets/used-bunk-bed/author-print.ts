/** 🖨️ Prints the print-suitability violations of one scale of the candidate with their poses and views, for locating gaps and slivers. */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const asset = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")).asset;
const scale = process.argv[2] ?? "50";
const requested = Number(process.argv[3] ?? scale);
const s = measureSuitability(asset, asset.representations[scale], requested, DEFAULT_PRINT_PROFILE) as any;
console.log("[DEBUG]", scale, requested, "suitable", s.suitable, "minGap", s.minGapMm, "minFeature", s.minFeatureMm);
for (const v of s.violations.slice(0, 40)) console.log("[DEBUG]", JSON.stringify(v));

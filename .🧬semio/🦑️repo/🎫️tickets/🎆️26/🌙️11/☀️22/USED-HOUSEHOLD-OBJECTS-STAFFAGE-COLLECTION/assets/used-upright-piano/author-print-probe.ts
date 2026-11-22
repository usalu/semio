/** 🔭️ Measures the print suitability of the piano of one representation of the candidate per named pose, so a failing pose can be found. Usage: bun author-print-probe.ts <scale> <requested> */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { measureSuitability } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const doc = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
const asset = doc.asset;
const scale = process.argv[2] ?? "500";
const requested = Number(process.argv[3] ?? "350");
const profile = { id: "office-laser", title: { en: "", de: "" }, description: { en: "", de: "" }, minStrokeMm: 0.1, minGapMm: 0.2, minFeatureMm: 0.2, minHatchSpacingMm: 0.3, source: "illustrative" };
for (const pose of asset.poses.filter((p: { key?: boolean }) => !p.key)) {
  const one = { ...asset, poses: [pose], clips: [], presentation: { ...asset.presentation, defaultPose: pose.id } };
  const s = measureSuitability(one, one.representations[scale], requested, profile as never);
  console.log("[DEBUG]", pose.id, JSON.stringify(s.violations).slice(0, 900));
}

/** 🔬️ Prints the print-feature findings of one scale of the candidate per named pose and per viewpoint, to find the pose and view of a finding. */
import { readFileSync } from "node:fs";
import { measureSuitability } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const doc = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8"));
const scale = process.argv[2] ?? "200";
const profile = { id: "office-laser", title: { en: "", de: "" }, description: { en: "", de: "" }, minStrokeMm: 0.1, minGapMm: 0.15, minFeatureMm: 0.2, minHatchSpacingMm: 0.4, source: "authored-estimate" as const };
for (const pose of doc.asset.poses.filter((p: { key?: boolean }) => !p.key)) {
  for (const view of ["elevation", "front", "plan"]) {
    const rep = structuredClone(doc.asset.representations[scale]);
    for (const v of ["elevation", "front", "plan"]) if (v !== view) rep.geometry[v] = [];
    const asset = { ...doc.asset, poses: [pose], representations: { ...doc.asset.representations, [scale]: rep } };
    const s = measureSuitability(asset, rep, Number(scale), profile);
    for (const v of s.violations) console.log(`[DEBUG] ${pose.id} ${view} ${v.check} ${v.shapes.join(",")} ${v.measuredMm.toFixed(3)}`);
  }
}

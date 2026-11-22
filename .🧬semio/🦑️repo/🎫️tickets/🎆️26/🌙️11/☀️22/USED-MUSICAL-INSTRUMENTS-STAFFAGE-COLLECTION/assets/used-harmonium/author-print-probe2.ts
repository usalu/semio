/** 🧪️ Finds which front shape of a scale makes the print-feature finding of one shape in one pose, by leaving each other shape out in turn. */
import { readFileSync } from "node:fs";
import { measureSuitability } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const doc = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8"));
const [scale, poseId, view] = [process.argv[2] ?? "200", process.argv[3] ?? "opened", process.argv[4] ?? "front"];
const profile = { id: "office-laser", title: { en: "", de: "" }, description: { en: "", de: "" }, minStrokeMm: 0.1, minGapMm: 0.15, minFeatureMm: 0.2, minHatchSpacingMm: 0.4, source: "authored-estimate" as const };
const pose = doc.asset.poses.find((p: { id: string }) => p.id === poseId);
const base = doc.asset.representations[scale];
for (const drop of base.geometry[view]) {
  const rep = structuredClone(base);
  for (const v of ["elevation", "front", "plan"]) rep.geometry[v] = v === view ? base.geometry[v].filter((s: { id: string }) => s.id !== drop.id) : [];
  const asset = { ...doc.asset, poses: [pose], representations: { ...doc.asset.representations, [scale]: rep } };
  const s = measureSuitability(asset, rep, Number(scale), profile);
  if (s.violations.length === 0) console.log(`[DEBUG] leaving out ${drop.id} clears it`);
}

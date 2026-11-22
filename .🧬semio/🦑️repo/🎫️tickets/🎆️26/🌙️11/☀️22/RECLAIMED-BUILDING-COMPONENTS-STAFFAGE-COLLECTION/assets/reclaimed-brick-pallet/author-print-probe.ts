/** 🔬️ Print-feature probe: measures one representation of the candidate at one denominator per named pose, so a finding can be traced to its pose. */
import { readFileSync } from "node:fs";
import { measureSuitability } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/";
const doc = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8"));
const profiles = JSON.parse(readFileSync(S + "🗂️catalog/🖨️print-profiles/🔣️.json", "utf8"));
const profile = profiles.profiles.find((p: { id: string }) => p.id === profiles.defaultProfile);
const [scale, denominator] = process.argv.slice(2).map(Number);
for (const pose of doc.asset.poses) {
  const asset = { ...doc.asset, poses: [pose] };
  const s = measureSuitability(asset, asset.representations[String(scale)], denominator, profile);
  console.log("[DEBUG]", pose.id, JSON.stringify(s.violations.map((v: { check: string; shapes: string[]; measuredMm: number }) => [v.check, v.shapes.join("+"), v.measuredMm.toFixed(3)])));
}

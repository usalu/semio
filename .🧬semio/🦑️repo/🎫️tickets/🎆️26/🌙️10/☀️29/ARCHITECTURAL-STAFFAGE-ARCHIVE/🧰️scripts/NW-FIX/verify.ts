/** [DEBUG] End-to-end check of radiator, van and scooter. */
import { readFileSync, readdirSync } from "node:fs";
import { checkAssetDetailed } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";

const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const ids = ["panel-radiator", "delivery-van", "motor-scooter"];
for (const cat of readdirSync(root)) {
  for (const id of ids) {
    let folder = "";
    try { folder = readdirSync(root + "/" + cat + "/" + id).find((f) => f.endsWith(".json")) ?? ""; } catch { continue; }
    if (!folder) continue;
    const asset = JSON.parse(readFileSync(root + "/" + cat + "/" + id + "/" + folder, "utf8")).asset;
    const report = checkAssetDetailed(asset, { samples: 5, physics: true });
    const pose = report.problems.filter((p) => p.startsWith("pose") || p.startsWith("clip") || p.startsWith("support"));
    console.log("[DEBUG]", asset.id, "problems", report.problems.length, "pose/clip", pose.length);
    for (const p of pose.slice(0, 12)) console.log(" ", p.slice(0, 180));
    for (const p of report.problems.filter((x) => !x.startsWith("suitability")).slice(0, 8)) if (!p.startsWith("pose") && !p.startsWith("clip")) console.log(" ", p.slice(0, 180));
  }
}

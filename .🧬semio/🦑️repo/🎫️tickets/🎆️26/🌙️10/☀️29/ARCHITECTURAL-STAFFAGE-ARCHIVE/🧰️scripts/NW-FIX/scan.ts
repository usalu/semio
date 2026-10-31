/** [DEBUG] List office-laser suitability violations for the six categories. */
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const cats = ["vegetation", "furniture", "objects", "vehicles", "machinery", "components"];
const lines: string[] = [];
for (const cat of readdirSync(root)) {
  if (!cats.some((c) => cat.includes(c))) continue;
  for (const folder of readdirSync(`${root}/${cat}`, { withFileTypes: true })) {
    if (!folder.isDirectory()) continue;
    const name = readdirSync(`${root}/${cat}/${folder.name}`).find((f) => f.endsWith(".json"));
    if (!name) continue;
    const asset = JSON.parse(readFileSync(`${root}/${cat}/${folder.name}/${name}`, "utf8")).asset;
    const bad: string[] = [];
    for (const d of [500, 200, 100, 50]) {
      const drawing = asset.representations[String(d)];
      if (!drawing || drawing.status !== "authored") continue;
      const s = measureSuitability(asset, drawing, d, DEFAULT_PRINT_PROFILE);
      if (!s.suitable) bad.push("1:" + d + " " + s.violations.map((v) => v.check + " " + v.shapes.join("+") + " " + v.measuredMm.toFixed(3)).join("; "));
    }
    if (bad.length) {
      lines.push(asset.id + "\n  " + bad.join("\n  "));
      console.log("[DEBUG]", asset.id, bad.length);
    }
  }
}
writeFileSync(process.argv[2]!, lines.join("\n") + "\n");
console.log("[DEBUG] failing", lines.length);

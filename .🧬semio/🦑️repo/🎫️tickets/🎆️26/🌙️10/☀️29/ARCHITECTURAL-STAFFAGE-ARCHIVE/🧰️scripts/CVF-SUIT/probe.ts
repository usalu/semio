/** [DEBUG] Suitability violation counts for the six staffage categories. */
import { readdirSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { measureSuitability, PRINT_PROFILES } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const cats = ["vegetation", "furniture", "objects", "vehicles", "machinery", "components"];
const profile = PRINT_PROFILES.profiles.find((p) => p.id === "office-laser")!;
let assets = 0;
let failed = 0;
for (const cat of readdirSync(root)) {
  if (!cats.some((c) => cat.includes(c))) continue;
  for (const folder of readdirSync(resolve(root, cat), { withFileTypes: true })) {
    if (!folder.isDirectory()) continue;
    const docName = readdirSync(resolve(root, cat, folder.name)).find((f) => f.endsWith(".json"));
    if (!docName) continue;
    const asset = JSON.parse(readFileSync(resolve(root, cat, folder.name, docName), "utf8")).asset;
    assets++;
    const lines: string[] = [];
    for (const d of ["500", "200", "100", "50"]) {
      const rep = asset.representations[d];
      if (!rep || rep.status !== "authored") continue;
      const suit = measureSuitability(asset, rep, Number(d), profile);
      if (suit.violations.length) lines.push(`1:${d} ${suit.violations.map((v) => `${v.check}:${v.shapes.join("+")}:${v.measuredMm.toFixed(3)}`).join(" ")}`);
    }
    if (lines.length) {
      failed++;
      console.log(`[DEBUG] ${asset.id}\n  ${lines.join("\n  ")}`);
    }
  }
}
console.log(`[DEBUG] suitability failed ${failed}/${assets}`);

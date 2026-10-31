/** [DEBUG] Audit animals: validateAsset and office-laser suitability. */
import { readdir, readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { validateAsset } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { measureSuitability } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals";
const profile = { id: "office-laser", title: { en: "Office laser", de: "Bürolaser" }, description: { en: "x", de: "x" }, minStrokeMm: 0.10, minGapMm: 0.15, minFeatureMm: 0.20, minHatchSpacingMm: 0.40, source: "illustrative" as const };
const only = process.argv[2];
const out: Record<string, unknown> = {};
for (const folder of (await readdir(root)).sort()) {
  if (only && folder !== only) continue;
  const doc = JSON.parse(await readFile(resolve(root, folder, "🔣️.json"), "utf8"));
  const asset = doc.asset;
  const issues = validateAsset(asset).map((i) => `${i.code} ${i.path}: ${i.message.en}`);
  const suits: string[] = [];
  for (const d of [500, 200, 100, 50]) {
    const rep = asset.representations[String(d)];
    if (!rep) continue;
    if (rep.status !== "authored") { suits.push(`1:${d} unavailable`); continue; }
    const t0 = Date.now();
    const s = measureSuitability(asset, rep, d, profile);
    const bits = s.violations.map((v) => `${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)}<${v.requiredMm}`);
    suits.push(`1:${d} suitable=${s.suitable} n=${s.violations.length} ${Date.now()-t0}ms ${bits.join(" | ")}`);
    console.log("[DEBUG]", asset.id, suits[suits.length-1]!.slice(0, 400));
  }
  out[asset.id] = { issues, suits };
  console.log("[DEBUG] VALID", asset.id, issues.length ? issues.join(" ;; ") : "ok");
}
const dest = process.argv[3] ?? "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/ANIMALS-FIX/audit.json";
await writeFile(dest, JSON.stringify(out, null, 1));

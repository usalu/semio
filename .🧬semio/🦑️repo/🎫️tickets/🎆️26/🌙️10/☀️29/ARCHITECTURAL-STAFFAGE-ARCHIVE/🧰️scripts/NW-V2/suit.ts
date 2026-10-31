import { readFileSync } from "node:fs";
import { measureSuitability, DEFAULT_PRINT_PROFILE } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { validateAsset } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
const file = process.argv[2]!;
const only = process.argv.slice(3);
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
for (const i of validateAsset(asset)) console.log("ISSUE", i.code, i.path, i.message.en);
for (const d of [500, 200, 100, 50]) {
  if (only.length && !only.includes(String(d))) continue;
  const rep = asset.representations[String(d)];
  if (rep.status !== "authored") continue;
  const s = measureSuitability(asset, rep, d, DEFAULT_PRINT_PROFILE);
  console.log("1:" + d, s.suitable ? "ok" : "BAD", s.minGapMm.toFixed(3), s.minFeatureMm.toFixed(3));
  for (const v of s.violations) console.log("  ", v.check, v.shapes.join("+"), v.measuredMm.toFixed(3));
}

/** 🔎️ prints the validateAsset issues of every fixture that holds an asset with volumes */
import { readFileSync, readdirSync, existsSync } from "node:fs";
import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
const root = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures";
for (const dir of readdirSync(root)) {
  const file = `${root}/${dir}/🔣️.json`;
  if (!existsSync(file)) continue;
  let doc: any;
  try { doc = JSON.parse(readFileSync(file, "utf8")); } catch { continue; }
  const asset = doc?.asset ?? doc;
  if (asset?.rig === undefined || asset.volumes === undefined) continue;
  const issues = validateAsset(asset);
  console.log(dir, issues.length);
  for (const i of issues.slice(0, 8)) console.log("  ", i.code, i.path, i.message.en.slice(0, 170));
}

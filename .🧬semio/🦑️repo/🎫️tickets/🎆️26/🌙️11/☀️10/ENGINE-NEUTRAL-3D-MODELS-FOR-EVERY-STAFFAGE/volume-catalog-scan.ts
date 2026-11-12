import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";

const root = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const counts = new Map<string, number>();
let invalid = 0;
let withVolume = 0;
for (const category of readdirSync(root)) {
  const dir = join(root, category);
  if (!statSync(dir).isDirectory()) continue;
  for (const slug of readdirSync(dir)) {
    const file = join(dir, slug, "🔣️.json");
    try {
      statSync(file);
    } catch {
      continue;
    }
    const asset = JSON.parse(readFileSync(file, "utf8")).asset;
    if (asset.volume) withVolume++;
    const issues = validateAsset(asset);
    if (issues.length) {
      invalid++;
      console.log(slug, [...new Set(issues.map((i) => i.code))].join(","), issues[0]!.path, issues[0]!.message.en.slice(0, 100));
    }
    for (const i of issues) counts.set(i.code, (counts.get(i.code) ?? 0) + 1);
  }
}
console.log({ invalid, withVolume }, Object.fromEntries(counts));

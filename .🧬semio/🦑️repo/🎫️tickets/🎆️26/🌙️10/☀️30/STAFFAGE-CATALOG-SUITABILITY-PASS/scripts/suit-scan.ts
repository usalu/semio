import { readdir, readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { validateAsset } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import type { AssetDocument, AuthoredRepresentation } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";

const cat = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const files: string[] = [];
for (const category of await readdir(cat, { withFileTypes: true })) {
  if (!category.isDirectory()) continue;
  if (category.name.includes("print-profiles") || category.name.includes("environments")) continue;
  const dir = resolve(cat, category.name);
  for (const asset of await readdir(dir, { withFileTypes: true })) {
    if (!asset.isDirectory()) continue;
    const folder = resolve(dir, asset.name);
    for (const file of await readdir(folder)) if (file.endsWith(".json")) files.push(resolve(folder, file));
  }
}
console.log("assets", files.length);
const docs = await Promise.all(files.map(async (file) => JSON.parse(await readFile(file, "utf8")) as AssetDocument));
const known = new Set(docs.map((d) => d.asset.id));
let bad = 0;
for (const doc of docs) {
  const asset = doc.asset;
  const issues = validateAsset(asset, { assets: known });
  const probs: string[] = issues.map((i) => `validate ${i.code} ${i.path}: ${i.message.en}`);
  for (const den of [500, 200, 100, 50] as const) {
    const rep = asset.representations[`${den}`];
    if (rep?.status !== "authored") continue;
    try {
      const s = measureSuitability(asset, rep as AuthoredRepresentation, den, DEFAULT_PRINT_PROFILE);
      for (const v of s.violations) probs.push(`suit ${den} ${v.check} ${v.shapes.join("+")}: ${v.measuredMm.toFixed(3)} < ${v.requiredMm}`);
    } catch (e) {
      probs.push(`suit-error ${den}: ${(e as Error).message}`);
    }
  }
  if (probs.length) {
    bad++;
    console.log("FAIL", asset.id, probs.length);
    for (const p of probs.slice(0, 15)) console.log(" ", p);
    if (probs.length > 15) console.log("  ...", probs.length - 15);
  } else console.log("OK", asset.id);
}
console.log("FAILED", bad, "/", docs.length);

/** 🎲️ Picks the stratified sample of the matrix diagnosis: per category the assets at the 12.5, 37.5, 62.5 and 87.5 % quantiles of the asset document size; prints one line `category id,id,id,id`. */
import { readdirSync, readFileSync, statSync, existsSync } from "node:fs";
import { join, resolve } from "node:path";

const root = resolve(process.argv[2]!);
const byCategory = new Map<string, { id: string; bytes: number }[]>();
for (const group of readdirSync(root, { withFileTypes: true }).filter((entry) => entry.isDirectory()))
  for (const folder of readdirSync(join(root, group.name), { withFileTypes: true }).filter((entry) => entry.isDirectory())) {
    const file = join(root, group.name, folder.name, "🔣️.json");
    if (!existsSync(file)) continue;
    const asset = JSON.parse(readFileSync(file, "utf8")).asset;
    if (asset === undefined) continue;
    (byCategory.get(asset.category) ?? byCategory.set(asset.category, []).get(asset.category)!).push({ id: asset.id, bytes: statSync(file).size });
  }
for (const [category, assets] of [...byCategory].sort((a, b) => a[0].localeCompare(b[0]))) {
  const sorted = assets.sort((a, b) => a.bytes - b.bytes || a.id.localeCompare(b.id));
  const picks = [0.125, 0.375, 0.625, 0.875].map((q) => sorted[Math.min(sorted.length - 1, Math.floor(q * sorted.length))]!);
  console.log(`${category} ${picks.map((pick) => pick.id).join(",")} # n=${sorted.length} bytes=${picks.map((pick) => pick.bytes).join("/")}`);
}

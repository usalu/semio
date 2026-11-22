/** 🔎️ Compares every started used-park-objects candidate with its catalog copy, its manifest row and its membership of the collection `used-park-objects`, without writing anything.
 *
 * `$BUN verify.ts [slug …]` prints one line per asset: `ok` or the differences found.
 *
 * @see ./publish.ts — the row this script recomputes */
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { totalPowerW } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/⚡️power/🟦️.ts";

const HERE = import.meta.dir;
const CATALOG = join(resolve(HERE, "../../../../../../.."), "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const manifest = JSON.parse(readFileSync(join(CATALOG, "🔣️.json"), "utf8"));
const members: string[] = manifest.collections.find((row: { id: string }) => row.id === "used-park-objects")?.assets ?? [];
const slugs = process.argv.length > 2 ? process.argv.slice(2) : readdirSync(join(HERE, "assets")).filter((name) => existsSync(join(HERE, "assets", name, "🔣️.json"))).sort();
let bad = 0;
for (const slug of slugs) {
  const findings: string[] = [];
  const bytes = readFileSync(join(HERE, "assets", slug, "🔣️.json"), "utf8");
  const asset = JSON.parse(bytes).asset;
  const folder = manifest.categories.find((row: { id: string }) => row.id === asset.category)?.folder;
  const target = join(CATALOG, folder, slug, "🔣️.json");
  if (!existsSync(target)) findings.push("no catalog file");
  else if (readFileSync(target, "utf8") !== bytes) findings.push("catalog copy differs from candidate");
  const distinctionFile = join(HERE, "assets", slug, "distinction.json");
  const distinction = existsSync(distinctionFile) ? JSON.parse(readFileSync(distinctionFile, "utf8")) : (findings.push("no distinction.json"), {});
  const entry = {
    id: asset.id,
    category: asset.category,
    title: asset.title,
    heightM: asset.dimensions.heightM,
    widthM: asset.dimensions.widthM,
    powerW: Math.round(totalPowerW(asset) * 10) / 10,
    poses: asset.poses.filter((pose: { key?: boolean }) => !pose.key).map((pose: { id: string }) => pose.id),
    clips: asset.clips.map((clip: { id: string }) => clip.id),
    representations: Object.fromEntries(["50", "100", "200", "500"].map((scale) => [scale, asset.representations[scale].status])),
    symbols: asset.symbols.map((symbol: { id: string }) => symbol.id),
    analytics: [...(asset.thermal.regions.length > 0 ? ["thermal"] : []), ...(asset.thermal.emitters.length > 0 ? ["transport"] : []), "structural"],
    exports: asset.exports,
    distinction: { en: distinction.en, de: distinction.de },
    completion: asset.completion,
  };
  const rows = manifest.entries.filter((row: { id: string }) => row.id === slug);
  if (rows.length !== 1) findings.push(`${rows.length} manifest rows`);
  else if (JSON.stringify(rows[0]) !== JSON.stringify(entry)) findings.push("manifest row differs from the candidate");
  if (!members.includes(slug)) findings.push("not in collection used-park-objects");
  if (findings.length > 0) bad++;
  console.log(`${slug}: ${findings.length === 0 ? "ok" : findings.join("; ")}`);
}
console.log(`${slugs.length} assets, ${bad} with findings, collection has ${members.length} members, manifest has ${manifest.entries.length} entries`);
process.exit(bad === 0 ? 0 : 1);

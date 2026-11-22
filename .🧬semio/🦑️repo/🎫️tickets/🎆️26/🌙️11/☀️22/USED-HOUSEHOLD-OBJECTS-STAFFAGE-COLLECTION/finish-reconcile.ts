/** 🧾️ Reconciles every candidate of this ticket with the catalog and the manifest without writing anything: the catalog copy must equal the candidate byte for byte, and the manifest row must equal the row `publish.ts` derives from the candidate, with membership in the collection `used-household-objects`.
 *
 * `$BUN finish-reconcile.ts` prints one line per asset and a final count.
 *
 * @see ./publish.ts — the derivation of the manifest row this mirrors */
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { totalPowerW } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/⚡️power/🟦️.ts";

const HERE = import.meta.dir;
const CATALOG = join(resolve(HERE, "../../../../../../.."), "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const manifest = JSON.parse(readFileSync(join(CATALOG, "🔣️.json"), "utf8"));
const members = manifest.collections.find((row: { id: string }) => row.id === "used-household-objects")?.assets ?? [];
let bad = 0;
for (const slug of readdirSync(join(HERE, "assets")).sort()) {
  const file = join(HERE, "assets", slug, "🔣️.json");
  if (!existsSync(file)) continue;
  const bytes = readFileSync(file, "utf8");
  const asset = JSON.parse(bytes).asset;
  const distinction = JSON.parse(readFileSync(join(HERE, "assets", slug, "distinction.json"), "utf8"));
  const folder = manifest.categories.find((row: { id: string }) => row.id === asset.category)?.folder;
  const target = join(CATALOG, folder, slug, "🔣️.json");
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
  const copy = existsSync(target) && readFileSync(target, "utf8") === bytes;
  const row = rows.length === 1 && JSON.stringify(rows[0]) === JSON.stringify(entry);
  const member = members.includes(slug);
  if (!copy || !row || !member) bad++;
  console.log(`${copy && row && member ? "ok  " : "DIFF"} ${slug}: copy ${copy ? "equal" : "differs"}, row ${row ? "equal" : rows.length === 0 ? "missing" : "differs"}, collection ${member ? "member" : "missing"}, ${asset.category}, ${asset.dimensions.massKg} kg, ${entry.powerW} W`);
}
console.log(`${bad} of ${members.length} members need a publish`);

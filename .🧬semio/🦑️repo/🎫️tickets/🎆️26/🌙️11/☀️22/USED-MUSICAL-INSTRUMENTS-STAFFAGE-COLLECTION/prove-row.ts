/** 🧾️ Compares the published state of the candidates of this ticket with what `publish.ts` would write: the catalog copy must be byte-equal to the candidate, the manifest row must equal the row derived from the candidate and the collection `used-instruments` must list the asset.
 *
 * `$BUN prove-row.ts [slug …]` prints one line per asset and exits 1 when any differs.
 *
 * @see ./publish.ts — the row derivation mirrored here */
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { totalPowerW } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/⚡️power/🟦️.ts";

const HERE = import.meta.dir;
const CATALOG = join(resolve(HERE, "../../../../../../.."), "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const manifest = JSON.parse(readFileSync(join(CATALOG, "🔣️.json"), "utf8"));
const slugs = process.argv.length > 2 ? process.argv.slice(2) : readdirSync(join(HERE, "assets")).sort();
let bad = 0;
for (const slug of slugs) {
  const bytes = readFileSync(join(HERE, "assets", slug, "🔣️.json"), "utf8");
  const asset = JSON.parse(bytes).asset;
  const distinction = JSON.parse(readFileSync(join(HERE, "assets", slug, "distinction.json"), "utf8"));
  const folder = manifest.categories.find((row: { id: string }) => row.id === asset.category).folder;
  const target = join(CATALOG, folder, slug, "🔣️.json");
  const copy = existsSync(target) && readFileSync(target, "utf8") === bytes;
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
  const row = rows.length === 1 && JSON.stringify(rows[0]) === JSON.stringify(entry);
  const member = manifest.collections.find((item: { id: string }) => item.id === "used-instruments")?.assets.includes(slug) === true;
  if (!copy || !row || !member) bad++;
  console.log(`${copy && row && member ? "OK  " : "DIFF"} ${slug}: catalog copy ${copy ? "equal" : "DIFFERS"}, manifest row ${row ? "equal" : rows.length === 0 ? "MISSING" : "DIFFERS"}, collection ${member ? "member" : "MISSING"}; ${asset.category} ${asset.dimensions.heightM}×${asset.dimensions.widthM} m, ${asset.dimensions.massKg} kg, ${entry.powerW} W, poses ${entry.poses.join("/")}, clips ${entry.clips.join("/")}, reps ${Object.values(entry.representations).join("/")}`);
}
process.exit(bad === 0 ? 0 : 1);

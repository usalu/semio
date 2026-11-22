/** 🧮️ Finisher proof: recomputes the manifest row of every candidate exactly as `publish.ts` writes it and compares it with the row the manifest holds, the catalog copy with the candidate byte for byte, and the membership of the collection `reclaimed-components`.
 * @see ./publish.ts — the row this proof repeats */
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { totalPowerW } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/⚡️power/🟦️.ts";

const HERE = import.meta.dir;
const CATALOG = join(resolve(HERE, "../../../../../../.."), "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const manifest = JSON.parse(readFileSync(join(CATALOG, "🔣️.json"), "utf8"));
const members: string[] = manifest.collections.find((row: { id: string }) => row.id === "reclaimed-components")?.assets ?? [];
const slugs = readdirSync(join(HERE, "assets")).sort();
let bad = 0;
for (const slug of slugs) {
  const bytes = readFileSync(join(HERE, "assets", slug, "🔣️.json"), "utf8");
  const asset = JSON.parse(bytes).asset;
  const distinction = JSON.parse(readFileSync(join(HERE, "assets", slug, "distinction.json"), "utf8"));
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
  const target = join(CATALOG, "🧱️components", slug, "🔣️.json");
  const findings = [
    ...(rows.length === 1 ? [] : [`${rows.length} manifest rows`]),
    ...(rows.length === 1 && JSON.stringify(rows[0]) !== JSON.stringify(entry) ? ["manifest row differs from the candidate"] : []),
    ...(members.includes(slug) ? [] : ["not in the collection"]),
    ...(existsSync(target) && readFileSync(target, "utf8") === bytes ? [] : ["catalog copy differs from the candidate"]),
  ];
  if (findings.length > 0) bad++;
  console.log(`${findings.length === 0 ? "OK " : "BAD"} ${slug}${findings.length === 0 ? "" : `: ${findings.join("; ")}`}`);
}
const strangers = members.filter((slug) => !slugs.includes(slug));
console.log(`collection reclaimed-components: ${members.length} assets${strangers.length > 0 ? `, without a candidate here: ${strangers.join(", ")}` : ""}; candidates ${slugs.length}; bad ${bad}`);
process.exit(bad === 0 ? 0 : 1);

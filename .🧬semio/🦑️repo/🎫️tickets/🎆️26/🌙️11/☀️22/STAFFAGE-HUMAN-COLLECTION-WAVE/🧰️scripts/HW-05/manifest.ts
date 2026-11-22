/** 📥️ Writes the manifest row of each HW-05 asset to T/📥️manifest-rows/<id>.json, every value derived from the evaluated asset and checked with `validateManifest` against a copy of the catalog manifest that holds the row: bun manifest.ts <distinctions.json> */
import { join, resolve } from "node:path";
import { totalPowerW, validateManifest } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const rows = resolve(import.meta.dir, "../../📥️manifest-rows");
const distinctions = await Bun.file(process.argv[2]!).json();
const manifest = await Bun.file(join(catalog, "🔣️.json")).json();
for (const [id, distinction] of Object.entries(distinctions)) {
  const asset = (await Bun.file(join(catalog, "👤️humans", id, "🔣️.json")).json()).asset;
  const row = {
    id, category: "humans", title: asset.title,
    heightM: asset.dimensions.heightM, widthM: asset.dimensions.widthM,
    powerW: Math.round(totalPowerW(asset) * 10) / 10,
    poses: asset.poses.filter((p: { key?: boolean }) => !p.key).map((p: { id: string }) => p.id),
    clips: asset.clips.map((c: { id: string }) => c.id),
    representations: Object.fromEntries(Object.entries(asset.representations).map(([k, v]) => [k, (v as { status: string }).status])),
    symbols: (asset.symbols ?? []).map((s: { id: string }) => s.id),
    analytics: ["thermal", "transport", "structural"],
    exports: asset.exports, distinction, completion: asset.completion,
  };
  const trial = { ...manifest, entries: [...manifest.entries.filter((e: { id: string }) => e.id !== id), row] };
  const issues = validateManifest(trial, [asset]).filter((i: { message: { en: string } }) => i.message.en.includes(id));
  for (const i of issues) console.log(`[DEBUG] ${id} ${i.code}: ${i.message.en}`);
  await Bun.write(join(rows, `${id}.json`), JSON.stringify(row, null, 2) + "\n");
  console.log(`${id} powerW ${row.powerW} issues ${issues.length}`);
}

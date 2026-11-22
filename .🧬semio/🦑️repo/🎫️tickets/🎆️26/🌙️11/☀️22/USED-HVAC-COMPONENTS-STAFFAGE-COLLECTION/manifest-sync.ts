/** 🔄️ Re-derives the manifest fields that `validateManifest` compares with the asset files (height, width, power, poses, clips) for every row that drifted, under the publish lock shared by the component waves, and rewrites the manifest in its own layout.
 *
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts */
import { mkdirSync, readFileSync, rmdirSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { totalPowerW } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/⚡️power/🟦️.ts";
import { validateManifest } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";

const CATALOG = resolve(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const LOCK = join(import.meta.dir, "../RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/publish.lock");
const FILE = join(CATALOG, "🔣️.json");
for (let attempt = 0; ; attempt++) {
  try {
    mkdirSync(LOCK);
    break;
  } catch {
    if (attempt > 600) throw new Error(`lock held: ${LOCK}`);
    Bun.sleepSync(200);
  }
}
try {
  const before = readFileSync(FILE, "utf8");
  const manifest = JSON.parse(before);
  const folders = Object.fromEntries(manifest.categories.map((c: { id: string; folder: string }) => [c.id, c.folder]));
  const assets = manifest.entries.map((e: { id: string; category: string }) => JSON.parse(readFileSync(join(CATALOG, folders[e.category], e.id, "🔣️.json"), "utf8")).asset);
  const drifted = new Set(validateManifest(manifest, assets).map((issue) => Number(/^entries\[(\d+)\]/.exec(issue.path)?.[1])).filter((i) => !Number.isNaN(i)));
  for (const index of drifted) {
    const entry = manifest.entries[index];
    const asset = assets[index];
    entry.heightM = asset.dimensions.heightM;
    entry.widthM = asset.dimensions.widthM;
    entry.powerW = Math.round(totalPowerW(asset) * 10) / 10;
    entry.poses = asset.poses.filter((pose: { key?: boolean }) => !pose.key).map((pose: { id: string }) => pose.id);
    entry.clips = asset.clips.map((clip: { id: string }) => clip.id);
    console.log(`synced ${entry.id}`);
  }
  const left = validateManifest(manifest, assets);
  for (const issue of left) console.log(`[DEBUG] still ${JSON.stringify(issue)}`);
  let text = before;
  for (const index of drifted) text = swap(text, manifest.entries[index]);
  if (JSON.stringify(JSON.parse(text)) !== JSON.stringify(manifest)) throw new Error("row swap did not produce the intended manifest; nothing written");
  writeFileSync(FILE, text);
  console.log(`${drifted.size} rows synced, ${left.length} issues left`);
} finally {
  rmdirSync(LOCK);
}

/** 🔁️ Replaces the manifest row of `entry.id` with `entry`, laid out like the rows the publish tools write. */
function swap(text: string, entry: { id: string }): string {
  const start = text.indexOf(`\n    {\n      "id": "${entry.id}",`);
  const end = text.indexOf("\n    }", start + 1);
  if (start < 0 || end < 0) throw new Error(`row ${entry.id} not found in the expected layout; nothing written`);
  const row = JSON.stringify(entry, null, 2).split("\n").map((line) => `    ${line}`).join("\n");
  return `${text.slice(0, start + 1)}${row}${text.slice(end + 6)}`;
}

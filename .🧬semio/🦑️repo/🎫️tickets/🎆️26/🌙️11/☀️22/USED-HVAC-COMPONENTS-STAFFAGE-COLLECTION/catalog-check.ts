/** 🧾️ Checks every manifest row against its asset file with the product's own manifest validator and power sum, and lists which issues belong to the used-HVAC collection. */
import { readFileSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { totalPowerW } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/⚡️power/🟦️.ts";
import { validateManifest } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";

const CATALOG = resolve(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const manifest = JSON.parse(readFileSync(join(CATALOG, "🔣️.json"), "utf8"));
const folders = Object.fromEntries(manifest.categories.map((c: { id: string; folder: string }) => [c.id, c.folder]));
const assets = manifest.entries.map((e: { id: string; category: string }) => JSON.parse(readFileSync(join(CATALOG, folders[e.category], e.id, "🔣️.json"), "utf8")).asset);
const ours = new Set(readdirSync(join(import.meta.dir, "assets")));
const issues = validateManifest(manifest, assets);
for (const issue of issues) console.log(`${[...ours].some((s) => JSON.stringify(issue).includes(s)) ? "OURS" : "other"} ${JSON.stringify(issue)}`);
for (const e of manifest.entries) {
  const a = assets.find((x: { id: string }) => x.id === e.id);
  const gap = Math.abs(e.powerW - totalPowerW(a));
  const pose = a.poses.find((p: { id: string }) => p.id === a.presentation.defaultPose);
  if (gap > 0.5 || !pose) console.log(`${ours.has(e.id) ? "OURS" : "other"} ${e.id} powerGap=${gap.toFixed(1)} defaultPose=${pose ? "ok" : "missing"}`);
}
console.log(`collection used-hvac-components: ${manifest.collections.find((c: { id: string }) => c.id === "used-hvac-components")?.assets.length} assets; ours ${ours.size}`);

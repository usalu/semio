/** 📒️ What the coordinator has to carry into the manifest for the batch SH-1: per person the declared `widthM`, `heightM` and `depthM` this repair changed (against a file of the session start when `--before <dir>` holds one, else against the manifest row for width and height) and the evaluated `powerW` where the manifest row is out of step. Writes `manifest-changes.json` beside this file and prints it.
 *  bun manifest-changes.ts [--before <dir of <slug>.json>] <slug>… */
import { parseArgs } from "node:util";
import { POWER_TOLERANCE_W, totalPowerW } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { catalogFile, S } from "./people.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { before: { type: "string" } } });
const manifest = await Bun.file(`${S}/🗂️catalog/🔣️.json`).json();
const out: Record<string, Record<string, number>> = {};
for (const slug of positionals) {
  const asset = (await Bun.file(catalogFile(slug)).json()).asset;
  const row = manifest.entries.find((e: { id: string }) => e.id === slug) ?? {};
  const old = values.before !== undefined && (await Bun.file(`${values.before}/${slug}.json`).exists()) ? (await Bun.file(`${values.before}/${slug}.json`).json()).asset.dimensions : { widthM: row.widthM, heightM: row.heightM, depthM: asset.dimensions.depthM };
  const change: Record<string, number> = {};
  for (const k of ["widthM", "heightM", "depthM"] as const) if (asset.dimensions[k] !== old[k]) change[k] = asset.dimensions[k];
  for (const k of ["widthM", "heightM"] as const) if (row[k] !== undefined && row[k] !== asset.dimensions[k]) change[k] = asset.dimensions[k];
  const power = totalPowerW(asset);
  if (row.powerW !== undefined && Math.abs(row.powerW - power) > Math.max(POWER_TOLERANCE_W, 0.05)) change.powerW = Number(power.toFixed(1));
  if (Object.keys(change).length > 0) out[slug] = change;
}
await Bun.write(`${import.meta.dir}/manifest-changes.json`, JSON.stringify(out, null, 2) + "\n");
console.log(JSON.stringify(out, null, 2));

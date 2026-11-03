import { loadCatalogFromFiles, loadEnvironments } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { solveThermalAt, thermalSession } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";

const catalog = await loadCatalogFromFiles();
const envs = await loadEnvironments();
const asset = catalog.assets.find((a) => a.id === process.argv[2])!;
const preset = envs!.presets.find((p: any) => p.id === "office-neutral")!;
const clip = process.argv[3]!;
const session = thermalSession();
for (const t of (process.argv[4] ?? "0,1,2,5,10,30,60").split(",").map(Number)) {
  const f = solveThermalAt(asset, { kind: "clip", clip, timeS: t } as any, preset.environment, { session });
  console.log(t, JSON.stringify(Object.fromEntries(Object.entries(f.regions).map(([k, v]: any) => [k, Math.round(v.surfaceTempC * 100) / 100]))));
}

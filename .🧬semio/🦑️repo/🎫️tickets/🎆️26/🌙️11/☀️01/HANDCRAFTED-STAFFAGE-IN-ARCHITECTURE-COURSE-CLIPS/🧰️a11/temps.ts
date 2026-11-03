import { loadCatalogFromFiles, loadEnvironments } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { solveThermalAt } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";

const id = process.argv[2]!;
const pose = process.argv[3] ?? "rated";
const catalog = await loadCatalogFromFiles();
const asset = catalog.assets.find((a) => a.id === id)!;
const envs = await loadEnvironments();
const env = envs!.presets.find((p) => p.id === "indoor-heated-winter")!.environment;
const field = solveThermalAt(asset, { kind: "pose", pose }, env);
for (const [region, f] of Object.entries(field.regions)) console.log(region.padEnd(22), f.surfaceTempC.toFixed(1), "C", f.steadyTempC.toFixed(1));

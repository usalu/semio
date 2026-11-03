import { readFileSync } from "node:fs";
import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveThermal } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";

const id = process.argv[2]!;
const items = process.argv.slice(3);
const cat = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const asset = cat.assets.find((a) => a.id === id)!;
const presets = JSON.parse(readFileSync(`${DEFAULT_CATALOG_ROOT}/🌤️environments/🔣️.json`, "utf8")).presets as { id: string; environment: never }[];
for (const item of items) {
  const [kind, name] = item.split(":");
  const q = kind === "pose" ? { kind: "pose", pose: name } : { kind: "clip", clip: name, timeS: 0.3 };
  const state = evaluate(asset, q as never);
  for (const p of presets.filter((x) => ["office-neutral", "indoor-heated-winter", "winter-overcast", "summer-heatwave-calm", "summer-hot-sun"].includes(x.id))) {
    const f = solveThermal(state, p.environment);
    const b = f.body!;
    const area = asset.thermal.body!.skinAreaM2;
    console.log(`${item.padEnd(22)} ${p.id.padEnd(22)} met=${(b.metabolicW / area / 58.15).toFixed(2)} core=${b.coreTempC.toFixed(2)} skin=${b.meanSkinTempC.toFixed(1)} wet=${b.skinWettedness.toFixed(2)} bf=${b.skinBloodFlowLM2H.toFixed(1)} sens=${f.totals.sensibleW.toFixed(0)} lat=${f.totals.latentW.toFixed(0)}`);
  }
}

import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = process.argv[2];
const only = process.argv.slice(3);
const plan = [
  ["gas-hob", "🔌️appliances", "./hob.mjs", "hob"],
  ["cooking-pot", "🪑️objects", "./pot.mjs", "pot"],
  ["electric-kettle", "🔌️appliances", "./kettle.mjs", "kettle"],
  ["scroll-compressor", "🏗️machinery", "./compressor.mjs", "compressor"],
  ["expansion-valve", "🏗️machinery", "./valve.mjs", "valve"],
  ["finned-heat-exchanger", "🏗️machinery", "./coil.mjs", "coil"],
  ["sorption-wheel", "🔌️appliances", "./wheel.mjs", "wheel"],
  ["plate-heat-exchanger", "🔌️appliances", "./plate.mjs", "plate"],
  ["evaporative-humidifier", "🔌️appliances", "./humidifier.mjs", "humidifier"],
];
for (const [id, folder, file, name] of plan) {
  if (only.length && !only.includes(id)) continue;
  const doc = (await import(file))[name]();
  const dir = join(root, folder, id);
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, "🔣️.json"), `${JSON.stringify(doc, null, 2)}\n`);
  console.log("wrote", id);
}

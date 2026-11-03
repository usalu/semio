import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = process.argv[2];
const only = process.argv.slice(3);
const plan = [
  ["room-thermostat", "🔌️appliances", "./thermostat.mjs", "roomThermostat"],
  ["thermostatic-radiator-valve", "🔌️appliances", "./trv.mjs", "trv"],
  ["oil-pumpjack", "🏗️machinery", "./pumpjack.mjs", "pumpjack"],
];
for (const [id, folder, file, name] of plan) {
  if (only.length && !only.includes(id)) continue;
  const doc = (await import(file))[name]();
  const dir = join(root, folder, id);
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, "🔣️.json"), `${JSON.stringify(doc, null, 2)}\n`);
  console.log("wrote", id);
}

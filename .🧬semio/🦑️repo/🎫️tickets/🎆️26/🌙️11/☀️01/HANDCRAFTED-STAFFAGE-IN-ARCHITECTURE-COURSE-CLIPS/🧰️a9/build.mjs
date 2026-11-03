import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = process.argv[2];
const only = process.argv.slice(3);
const plan = [
  ["circulation-pump", "./pump.mjs", "pump"],
  ["gas-boiler", "./boiler.mjs", "boiler"],
  ["hot-water-cylinder", "./cylinder.mjs", "cylinder"],
];
for (const [id, file, name] of plan) {
  if (only.length && !only.includes(id)) continue;
  const doc = (await import(file))[name]();
  const dir = join(root, "🔌️appliances", id);
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, "🔣️.json"), `${JSON.stringify(doc, null, 2)}\n`);
  console.log("wrote", id);
}

import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = process.argv[2];
const only = process.argv.slice(3);
const load = async (file, name) => (await import(file))[name];
const plan = [
  ["operating-table", "🛋️furniture", "./table.mjs", "table"],
  ["chalkboard", "🪑️objects", "./board.mjs", "board"],
  ["kitchen-extractor-hood", "🔌️appliances", "./hood.mjs", "hood"],
  ["storage-crate-stack", "🪑️objects", "./crates.mjs", "crates"],
];
for (const [id, folder, file, name] of plan) {
  if (only.length && !only.includes(id)) continue;
  const doc = (await load(file, name))();
  const dir = join(root, folder, id);
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, "🔣️.json"), `${JSON.stringify(doc, null, 2)}\n`);
  console.log("wrote", id);
}

import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { vintageCar } from "./vintage-car.mjs";

const root = process.argv[2];
const only = process.argv[3];
const out = [["🚜️vehicles", "vintage-car-1929", vintageCar]];
try {
  const { splitAc } = await import("./split-ac.mjs");
  out.push(["🔌️appliances", "split-ac-indoor-unit", splitAc]);
} catch (e) {
  if (e.code !== "ERR_MODULE_NOT_FOUND") throw e;
}
for (const [cat, id, make] of out) {
  if (only && only !== id) continue;
  const dir = join(root, cat, id);
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, "🔣️.json"), `${JSON.stringify(make(), null, 2)}\n`);
  console.log("wrote", id);
}

import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { wrap } from "./comp.mjs";
import { wall } from "./walls.mjs";

const root = process.argv[2];
const only = process.argv.slice(3);
const plan = [
  ["solid-brick-wall-1960", () => wall("solid-brick-wall-1960")],
  ["etics-brick-wall-12", () => wall("etics-brick-wall-12")],
  ["etics-brick-wall-30", () => wall("etics-brick-wall-30")],
];
for (const [id, make] of plan) {
  if (only.length && !only.includes(id)) continue;
  const out = make();
  const dir = join(root, "🧱️components", id);
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, "🔣️.json"), `${JSON.stringify(wrap(out.asset), null, 2)}\n`);
  console.log("wrote", id, "U", out.u.toFixed(3), "thick", out.thick, "mass", out.mass);
}

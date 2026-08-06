import { readFileSync, writeFileSync } from "node:fs";

const OLD = "📺️renderer/🧑️‍🎨️engine/🧊️wgpu/⚡️implementations/🦀️rust";
const NEW = "📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu";

const files = [
  "/Users/ueli/Documents/semio/.vscode/launch.json",
  "/Users/ueli/Documents/semio/.vscode/🧩️launch.seed.jsonc",
];

for (const f of files) {
  const text = readFileSync(f, "utf8");
  const count = (text.match(new RegExp(OLD.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"), "g")) || []).length;
  const replaced = text.split(OLD).join(NEW);
  writeFileSync(f, replaced);
  console.log(`${f}: replaced ${count} occurrences`);
}

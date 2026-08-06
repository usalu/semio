import { readFileSync, writeFileSync } from "node:fs";

const OLD = "📺️renderer/🧑️‍🎨️engine/🧊️wgpu/⚡️implementations/🦀️rust";
const NEW = "📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu";

const files = [
  "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/⚡️implementations/🟦️typescript/📜️script.ts",
  "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/⚡️implementations/🟦️typescript/⚙️vite.config.ts",
  "/Users/ueli/Documents/semio/.storybook/scopes.ts",
];

for (const f of files) {
  const text = readFileSync(f, "utf8");
  const count = (text.match(new RegExp(OLD.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"), "g")) || []).length;
  const replaced = text.split(OLD).join(NEW);
  writeFileSync(f, replaced);
  console.log(`${f}: replaced ${count} occurrences`);
}

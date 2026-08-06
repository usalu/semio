import { existsSync } from "node:fs";
import { resolve } from "node:path";
const dir = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu";
const checks: [string, string][] = [
  ["package.json $schema", "../../../../../../../../../../node_modules/nx/schemas/project-schema.json"],
  ["package.json exports", "./📦️index.ts"],
  ["project.json $schema", "../../../../../../../../../../node_modules/nx/schemas/project-schema.json"],
  ["build.rs icons_dir", "../../../../../../../../../🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🔣️icons"],
];
for (const [name, rel] of checks) {
  const abs = resolve(dir, rel);
  console.log(`${existsSync(abs) ? "OK  " : "MISS"} ${name}: ${rel}`);
}

import { existsSync } from "node:fs";
import { join, relative, resolve } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const OLD_DIR = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧊️wgpu/⚡️implementations/🦀️rust");
const NEW_DIR = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu");

const aux: [string, string][] = [
  ["build.rs icons_dir", "../../../../../../../../🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🔣️icons"],
  ["project.json $schema", "../../../../../../../../../node_modules/nx/schemas/project-schema.json"],
  ["package.json $schema", "../../../../node_modules/nx/schemas/project-schema.json"],
  ["script.ts repo-lib index.ts", "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts"],
  ["script.ts vite-elements-assets.ts", "../../../../../../../../🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts"],
  ["script.ts playgrounds.ts (type)", "../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🟦️typescript/📇️registry/🤖️generated/🟦️playgrounds.ts"],
  ["script.ts registry script.ts", "../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🟦️typescript/📇️registry/📜️script.ts"],
];

let allOk = true;
for (const [name, oldRel] of aux) {
  const abs = resolve(OLD_DIR, oldRel);
  const oldOk = existsSync(abs);
  const newRel = relative(NEW_DIR, abs);
  const newAbs = resolve(NEW_DIR, newRel);
  const newOk = existsSync(newAbs) && newAbs === abs;
  if (!oldOk) console.log(`  [NOTE: old target does not exist as a file — may be intentionally-broken pre-existing ref] ${name}`);
  if (!newOk) allOk = false;
  console.log(`${name}`);
  console.log(`  old: ${oldRel}  [exists=${oldOk}]`);
  console.log(`  abs: ${abs}`);
  console.log(`  new: ${newRel}  [resolvesOk=${newOk}]`);
  console.log("");
}
console.log(allOk ? "ALL OK" : "SOME MISMATCH");

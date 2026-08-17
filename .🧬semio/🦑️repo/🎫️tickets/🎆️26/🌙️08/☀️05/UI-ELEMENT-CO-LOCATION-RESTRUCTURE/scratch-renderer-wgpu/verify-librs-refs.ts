import { existsSync } from "node:fs";
import { resolve } from "node:path";
const NEW_DIR = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu";
const OLD_DIR = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧊️wgpu/⚡️implementations/🦀️rust"; // no longer exists but used to compute absolute target
const checks: [string, string][] = [
  ["generated_plugin_hosts #[path]", "../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🟦️typescript/📇️registry/🤖️generated/🦀️hosts.rs"],
  ["ANTA_LATIN include_bytes!", "../../../../../../../../🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🔤️fonts/🔤️anta/🔤️latin.ttf"],
];
for (const [name, oldRel] of checks) {
  const abs = resolve(OLD_DIR, oldRel);
  console.log(`${name}: old target abs = ${abs} exists=${existsSync(abs)}`);
  // compute new relative via path.relative equivalent (add one more ../ since NEW dir is one level deeper, same dir as lib.rs itself)
}
import { relative } from "node:path";
for (const [name, oldRel] of checks) {
  const abs = resolve(OLD_DIR, oldRel);
  const newRel = relative(NEW_DIR, abs);
  const newAbs = resolve(NEW_DIR, newRel);
  console.log(`${name}: new rel = ${newRel}  resolvesOk=${existsSync(newAbs) && newAbs === abs}`);
}

import { existsSync, readdirSync, statSync } from "fs";
import { join } from "path";
const paths = [
  "✏️s/🔨️modules/🧊️3d/🎬️scene/⚡️implementations/🦀️rust/Cargo.toml",
  "✏️s/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml",
  "✏️s/🔨️modules/🧊️3d/🎬️scene/🦀️component.rs",
  "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🎗wgpu/Cargo.toml",
  "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🎗wgpu/build.rs",
];
const root="/Users/ueli/Documents/semio";
for (const p of paths) {
  console.log(existsSync(join(root,p)) ? "OK" : "MISSING", p);
}
// list scene dir
const scene=join(root,"✏️s/🔨️modules/🎗3d");

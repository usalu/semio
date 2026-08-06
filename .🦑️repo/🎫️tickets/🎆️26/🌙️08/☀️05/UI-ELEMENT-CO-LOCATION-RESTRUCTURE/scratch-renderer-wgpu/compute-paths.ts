import { existsSync } from "node:fs";
import { join, relative, resolve } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const OLD_DIR = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧊️wgpu/⚡️implementations/🦀️rust");
const NEW_DIR = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu");

const deps: [string, string][] = [
  ["semio-framework-core", "../../../../../../../../../🧰️framework/⚡️implementations/🦀️rust"],
  ["framework_surface_node_graph", "../../../../../../../../🔨️modules/🗺️surface/🕸️node-graph/⚡️implementations/🦀️rust"],
  ["framework_editor", "../../../../../../../../../🧰️framework/🔨️modules/✍️editor/⚡️implementations/🦀️rust"],
  ["infinite_canvas", "../../../../../♾️infinite/🖼️canvas/⚡️implementations/🦀️rust"],
  ["infinite_world", "../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/⚡️implementations/🦀️rust/🌍️world"],
  ["kernel_3d_scene", "../../../../../../../../../✏️s/🔨️modules/🧊️3d/🎬️scene/⚡️implementations/🦀️rust"],
  ["framework_surface_tiled_map", "../../../../../../../../🔨️modules/🗺️surface/🗺️tiled-map/⚡️implementations/🦀️rust"],
  ["puzzle", "../../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust"],
  ["flow_core", "../../../../../🌊️flow/🫀️core/⚡️implementations/🦀️rust"],
  ["ui_wgpu", "../../../../../../../../🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu"],
  ["semio-framework-plugin", "../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🦀️rust"],
  // target.'cfg(not(wasm32))'.dependencies
  ["semio-framework-plugin-host", "../../../../../🔌️plugin/🖥️host/⚡️implementations/🦀️rust"],
  ["store_sync", "../../../../../🏪️store/🔄️sync/⚡️implementations/🦀️rust"],
  ["vcs", "../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/⚡️implementations/🦀️rust"],
  ["store", "../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚡️implementations/🦀️rust"],
  ["dsl", "../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/⚡️implementations/🦀️rust"],
  ["protocol", "../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/⚡️implementations/🦀️rust"],
];

console.log(`OLD_DIR=${OLD_DIR}`);
console.log(`NEW_DIR=${NEW_DIR}`);
console.log(`OLD_DIR exists: ${existsSync(OLD_DIR)}, NEW_DIR exists: ${existsSync(NEW_DIR)}`);
console.log("");

let allOk = true;
for (const [name, oldRel] of deps) {
  const abs = resolve(OLD_DIR, oldRel);
  const oldOk = existsSync(abs);
  const newRel = relative(NEW_DIR, abs);
  const newAbs = resolve(NEW_DIR, newRel);
  const newOk = existsSync(newAbs) && newAbs === abs;
  if (!oldOk || !newOk) allOk = false;
  console.log(`${name}`);
  console.log(`  old: ${oldRel}  [exists=${oldOk}]`);
  console.log(`  abs: ${abs}`);
  console.log(`  new: ${newRel}  [resolvesOk=${newOk}]`);
  console.log("");
}
console.log(allOk ? "ALL OK" : "SOME MISMATCH — DO NOT PROCEED");

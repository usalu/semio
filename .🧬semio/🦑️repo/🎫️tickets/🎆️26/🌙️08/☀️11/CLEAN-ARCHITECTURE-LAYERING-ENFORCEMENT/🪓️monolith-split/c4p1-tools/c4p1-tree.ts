// 🌳️ c4p1: the repo's own module-tree reader over the seven world roots (every child must resolve) and the four assertions of the scene-input-residency source law.
import { join } from "node:path";
import { listModuleTree, readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const world = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world";
const crates = ["🗃️model", "🧩️components", "🧲️cursors", "🧱️scene", "🌉️bridge", "🧵️interaction", "🎛️host"];
let total = 0;
for (const crate of crates) {
  const files = listModuleTree(join(world, crate, "🦀️.rs"));
  total += files.length;
  console.log(crate, files.length, files.map((f) => f.split("/").at(-2)).join(" "));
}
const source = crates.map((crate) => readModuleTreeSource(join(world, crate, "🦀️.rs"))).join("\n");
console.log("contains grid_snap_enabled: bool", source.includes("grid_snap_enabled: bool"));
console.log("snap_world_point_to_grid( count", source.match(/snap_world_point_to_grid\(/g)?.length);
console.log("contains cancel_reference_image_url", source.includes("cancel_reference_image_url"));
console.log("contains world3d_reference_url_is_current", source.includes("world3d_reference_url_is_current"));
console.log("files", total);

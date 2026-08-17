import { readFileSync, writeFileSync } from "fs";

const cargoPath = readFileSync("/tmp/surf-cargo.txt", "utf8").trim();
let cargo = readFileSync(cargoPath, "utf8");
cargo = cargo.replace(
  `[features]
default = ["board-2d"]
# 🧩 board-2d session needs the puzzle plugin crate; tiled-map/terrain/paint/node-graph do not.
board-2d = ["dep:puzzle"]
`,
  `[features]
default = ["board-2d", "session-bindgen"]
# 🧩 board-2d session needs the puzzle plugin crate; tiled-map/terrain/paint/node-graph do not.
board-2d = ["dep:puzzle"]
# 🌉️ wasm-bindgen session wrappers (Map/Terrain/…) — disabled when this file is path-mounted into
# infinite (avoids duplicate TerrainSession symbols linking surface+infinite into one cdylib).
session-bindgen = []
`,
);
writeFileSync(cargoPath, cargo);
console.log("cargo features updated");

const terrain = readFileSync("/tmp/terrain-path.txt", "utf8").trim();
let t = readFileSync(terrain, "utf8");
const count1 = (t.match(/#\[cfg\(target_arch = "wasm32"\)\]/g) || []).length;
t = t.replaceAll('#[cfg(target_arch = "wasm32")]', '#[cfg(all(target_arch = "wasm32", feature = "session-bindgen"))]');
writeFileSync(terrain, t);
console.log("terrain wasm cfgs updated", count1);

const script = readFileSync("/tmp/surf-script.txt", "utf8").trim();
let s = readFileSync(script, "utf8");
if (!s.includes('cargoFeatures: ["session-bindgen"]')) {
  s = s.replace(
    `noDefaultFeatures: true,`,
    `noDefaultFeatures: true,\n      cargoFeatures: ["session-bindgen"],`,
  );
  writeFileSync(script, s);
  console.log("script cargoFeatures session-bindgen");
}

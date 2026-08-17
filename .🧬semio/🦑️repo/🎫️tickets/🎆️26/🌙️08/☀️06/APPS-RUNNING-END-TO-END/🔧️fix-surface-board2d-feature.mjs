import { readFileSync, writeFileSync } from "fs";

const cargoPath = readFileSync("/tmp/surf-cargo.txt", "utf8").trim();
const gluePath = readFileSync("/tmp/surf-glue.txt", "utf8").trim();

let cargo = readFileSync(cargoPath, "utf8");
if (!cargo.includes("[features]")) {
  // Insert features after [lints] / before wasm-pack metadata or after package.metadata.semio
  const anchor = "[lints]\nworkspace = true\n";
  if (!cargo.includes(anchor)) throw new Error("lints anchor missing");
  cargo = cargo.replace(
    anchor,
    `${anchor}
[features]
default = ["board-2d"]
# 🧩 board-2d session needs the puzzle plugin crate; tiled-map/terrain/paint/node-graph do not.
board-2d = ["dep:puzzle"]
`,
  );
} else if (!cargo.includes("board-2d")) {
  throw new Error("features exist but board-2d missing — inspect manually");
}

cargo = cargo.replace(
  `puzzle = { path = "../../../../../✏️s/🔌️plugins/🧩puzzle/📦️packages/🦀️rust", package = "semio-s-plugin-puzzle" }`,
  `puzzle = { path = "../../../../../✏️s/🔌️plugins/🧩puzzle/📦️packages/🦀️rust", package = "semio-s-plugin-puzzle", optional = true }`,
);
// emoji might differ - try from file
if (!cargo.includes("optional = true") || !cargo.includes('package = "semio-s-plugin-puzzle", optional = true')) {
  cargo = cargo.replace(
    /puzzle = \{ path = "[^"]+", package = "semio-s-plugin-puzzle" \}/,
    (m) => m.replace('package = "semio-s-plugin-puzzle" }', 'package = "semio-s-plugin-puzzle", optional = true }'),
  );
}
writeFileSync(cargoPath, cargo);
console.log("cargo patched");

let glue = readFileSync(gluePath, "utf8");
const old = `#[path = "../../🎲️board-2d/🦀️component.rs"]
pub mod board_2d;`;
const neu = `#[cfg(feature = "board-2d")]
#[path = "../../🎲️board-2d/🦀️component.rs"]
pub mod board_2d;`;
if (!glue.includes(old)) {
  console.error("glue board_2d block missing");
  process.exit(1);
}
if (!glue.includes('#[cfg(feature = "board-2d")]')) {
  glue = glue.replace(old, neu);
  writeFileSync(gluePath, glue);
  console.log("glue patched");
} else {
  console.log("glue already gated");
}

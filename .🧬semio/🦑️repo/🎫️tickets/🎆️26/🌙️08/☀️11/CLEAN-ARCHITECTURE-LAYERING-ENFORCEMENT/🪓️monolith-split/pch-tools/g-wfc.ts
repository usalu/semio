// 🀄️ wfc: the bitmap SQLite entity module and its pixel law helper get one owner crate; the 2d and grid2d snapshots depend on it instead of path-mounting it.
import {DRY, log, put, read, replaceRegexOnce, addToSection, workspaceDependencyTable, lockEnsure, ownerRowsAddViaGenerator, relPosix} from "./dd.ts";
const W = "✏️s/🔌️plugins/🀄️wfc";
const B = `${W}/🪶️sqlite/🎨️bitmap`;
const PKG = `${B}/📦️packages/🦀️rust`;
const NAME = "semio-s-wfc-sqlite-bitmap";
const FW = "🧰️framework/🔨️modules";
const LOCK_DEPS = ["semio-framework-os-kernel", "semio-framework-pack", "semio-framework-value", "serde_json"];
const manifest = `[package]
workspace = "${relPosix(PKG, W)}"
name = "${NAME}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "Explicit indexed bitmap entities of the wfc SQLite snapshots: canonical byte indices or exact literal Unicode scalars, shared by wfc 2d and grid2d"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[features]
testing = ["dep:pack", "dep:serde_json"]

[dependencies]
semio-framework-os-kernel = { workspace = true }
semio-framework-value = { path = "${relPosix(PKG, `${FW}/🌱️value/📦️packages/🦀️rust`)}" }
pack = { path = "${relPosix(PKG, `${FW}/🎒️pack/📦️packages/🦀️rust`)}", package = "semio-framework-pack", optional = true }
serde_json = { workspace = true, optional = true }
`;
put(`${PKG}/Cargo.toml`, manifest, true);
const lib = `${B}/🦀️.rs`;
put(lib, read(lib).replace(/\s*$/u, "\n") + '\n#[cfg(feature = "testing")]\n#[path = "🧪️tests/🦀️.rs"]\npub mod pixel_laws;\n');
lockEnsure(`${W}/Cargo.lock`, NAME, "0.1.0", LOCK_DEPS, []);
await ownerRowsAddViaGenerator([{name: NAME, root: PKG}]);
for (const [art, pkg] of [["◻️2d", "semio-s-artifact-wfc-2d"], ["🔲️grid2d", "semio-s-artifact-wfc-grid2d"]] as const) {
  const A = `${W}/🗿️artifacts/${art}`;
  const S = `${A}/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot`;
  workspaceDependencyTable(`${A}/Cargo.toml`, NAME, relPosix(A, PKG));
  addToSection(`${A}/📦️packages/🦀️rust/Cargo.toml`, "[dependencies]", `${NAME} = { workspace = true }`);
  addToSection(`${A}/📦️packages/🦀️rust/Cargo.toml`, "[dev-dependencies]", `${NAME} = { workspace = true, features = ["testing"] }`);
  replaceRegexOnce(`${S}/🦀️.rs`, /#\[path="[^"]*🪶️sqlite\/🎨️bitmap\/🦀️\.rs"\]mod bitmap;/, "use semio_s_wfc_sqlite_bitmap as bitmap;");
  replaceRegexOnce(`${S}/🧪️tests/🦀️.rs`, /#\[path="[^"]*🪶️sqlite\/🎨️bitmap\/🧪️tests\/🦀️\.rs"\]mod bitmap_pixel_tests;/, "use semio_s_wfc_sqlite_bitmap::pixel_laws as bitmap_pixel_tests;");
  lockEnsure(`${A}/Cargo.lock`, NAME, "0.1.0", LOCK_DEPS, [pkg]);
}
log(DRY ? "dry run" : "written");

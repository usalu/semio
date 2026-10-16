// 🧩️ Puzzle: jobs, retained command and board scene each get exactly one owner crate; 2d and 5d depend on them instead of path-mounting the sources.
import {ROOT, DRY, log, put, replaceOnce, addToSection, workspaceDependencyTable, lockEnsure, ownerRowsAddViaGenerator, relPosix, abs} from "./dd.ts";
import {join} from "node:path";
const Z = "✏️s/🔌️plugins/🧩️puzzle";
const SCENE_DIR = `${Z}/🔨️modules/🎲️board/🎬️scene/📦️packages/🦀️rust`;
const RETAINED_DIR = `${Z}/🎮️commands/🧵️retained/📦️packages/🦀️rust`;
const JOBS_DIR = `${Z}/🎮️commands/📤️jobs/📦️packages/🦀️rust`;
const SCENE = "semio-s-plugin-puzzle-board-scene";
const manifest = `[package]
workspace = "../../../../.."
name = "${SCENE}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "The puzzle-owned projection into the framework's neutral board scene contract, shared by Puzzle 2d and 5d"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
semio-framework-value = { workspace = true }

[dev-dependencies]
semio-framework-pack-json = { workspace = true }
serde_json = "1.0.149"
`;
put(`${SCENE_DIR}/Cargo.toml`, manifest, true);
lockEnsure(`${Z}/Cargo.lock`, SCENE, "0.1.0", ["semio-framework-pack-json", "semio-framework-value", "serde_json"], []);
await ownerRowsAddViaGenerator([{name: SCENE, root: SCENE_DIR}]);
for (const [art, pkg] of [["◻️2d", "semio-s-artifact-puzzle-2d"], ["🖐️5d", "semio-s-artifact-puzzle-5d"]] as const) {
  const A = `${Z}/🗿️artifacts/${art}`;
  workspaceDependencyTable(`${A}/Cargo.toml`, SCENE, relPosix(A, SCENE_DIR));
  workspaceDependencyTable(`${A}/Cargo.toml`, "semio-s-plugin-puzzle-retained", relPosix(A, RETAINED_DIR));
  void JOBS_DIR;
  const M = `${A}/📦️packages/🦀️rust/Cargo.toml`;
  replaceOnce(M, "\nsemio-s-plugin-puzzle-jobs = { workspace = true }\n", '\nsemio-s-plugin-puzzle-jobs = { workspace = true, features = ["testing"] }\nsemio-s-plugin-puzzle-retained = { workspace = true, features = ["testing"] }\n');
  addToSection(M, "[dependencies]", `${SCENE} = { workspace = true }`);
  addToSection(M, "[dependencies]", "semio-s-plugin-puzzle-jobs = { workspace = true }");
  addToSection(M, "[dependencies]", "semio-s-plugin-puzzle-retained = { workspace = true }");
  const R = `${A}/🦀️.rs`;
  replaceOnce(R, '#[path = "../../🔨️modules/🎲️board/🎬️scene/🦀️.rs"]\npub mod board_scene;', "pub use semio_s_plugin_puzzle_board_scene as board_scene;");
  replaceOnce(R, '#[path = "../../🎮️commands/📤️jobs/🦀️.rs"]\npub mod puzzle_job;', "pub use semio_s_plugin_puzzle_jobs as puzzle_job;");
  replaceOnce(R, '#[path = "../../🎮️commands/🧵️retained/🦀️.rs"]\npub mod retained_command;', "pub use semio_s_plugin_puzzle_retained as retained_command;");
  lockEnsure(`${A}/Cargo.lock`, SCENE, "0.1.0", ["semio-framework-pack-json", "semio-framework-value", "serde_json"], [pkg]);
  lockEnsure(`${A}/Cargo.lock`, "semio-s-plugin-puzzle-jobs", "0.1.0", [], [pkg]);
  lockEnsure(`${A}/Cargo.lock`, "semio-s-plugin-puzzle-retained", "0.1.0", [], [pkg]);
}
log(DRY ? "dry run" : "written");
void ROOT; void join;

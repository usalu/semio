// 🌀️ procedural: the neutral SQLite fixture builder of the two generation artifacts gets one owner crate in the core workspace; generation 2d and 3d depend on it.
import {DRY, log, put, read, replaceOnce, addToSection, workspaceDependencyTable, lockEnsure, ownerRowsAddViaGenerator, relPosix} from "./dd.ts";
const P = "✏️s/🔌️plugins/🌀️procedural";
const CORE = `${P}/🫀️core`;
const OWNER = `${CORE}/🧬️generation/🪶️sqlite/🧪️tests/🧰️support/📦️packages/🦀️rust`;
const NAME = "semio-s-procedural-core-generation-sqlite-test-support";
const manifest = `[package]
workspace = "${relPosix(OWNER, CORE)}"
name = "${NAME}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "Explicit shared native fixture for the two actual procedural persisted type pairs"

[package.metadata.semio]
role = "test"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
semio-framework-artifact-flow-flow = { workspace = true }
semio-framework-artifact-playbook-playbook = { workspace = true }
semio-framework-value = { workspace = true }
serde_json = "1"
`;
put(`${OWNER}/Cargo.toml`, manifest, true);
const deps = ["semio-framework-artifact-flow-flow", "semio-framework-artifact-playbook-playbook", "semio-framework-value", "serde_json"];
lockEnsure(`${CORE}/Cargo.lock`, NAME, "0.1.0", deps, []);
await ownerRowsAddViaGenerator([{name: NAME, root: OWNER}]);
for (const [art, pkg] of [["🌀️generation2d", "semio-s-artifact-procedural-2d-io-sqlite"], ["🧊️generation3d", "semio-s-artifact-procedural-3d-io-sqlite"]] as const) {
  const A = `${P}/🗿️artifacts/${art}`;
  const S = `${A}/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite`;
  const crate = `${S}/📦️packages/🦀️rust`;
  workspaceDependencyTable(`${A}/Cargo.toml`, NAME, relPosix(A, OWNER));
  addToSection(`${crate}/Cargo.toml`, "[dev-dependencies]", `${NAME} = { workspace = true }`);
  replaceOnce(`${S}/📸️snapshot/🧪️tests/🦀️.rs`, '#[path="../../../../../../../../../../🫀️core/🧬️generation/🪶️sqlite/🧪️tests/🧰️support/🦀️.rs"]mod neutral;', "use semio_s_procedural_core_generation_sqlite_test_support as neutral;");
  lockEnsure(`${A}/Cargo.lock`, NAME, "0.1.0", deps, [pkg]);
}
void read;
log(DRY ? "dry run" : "written");

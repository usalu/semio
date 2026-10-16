// 📊️ csv receiving test depends on the trace memory testing owner; 🪐️ the space collection and space artifacts depend on the space sqlite cells owner. No crate path-mounts another crate's lib root.
import {DRY, log, replaceOnce, addToSection, lockEnsure, relPosix, depPackages, packageNameOf} from "./dd.ts";
const C = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv";
const CSV_SNAP = `${C}/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust`;
const TRACE = "🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/📦️packages/🦀️rust";
const traceName = packageNameOf(`${TRACE}/Cargo.toml`);
replaceOnce(`${C}/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🦀️.rs`, '#[path="../../../../../../../../../../../../../../🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]\nmod heap;', "use semio_framework_trace_memory_testing as heap;");
addToSection(`${CSV_SNAP}/Cargo.toml`, "[dev-dependencies]", `${traceName} = { path = "${relPosix(CSV_SNAP, TRACE)}" }`);
lockEnsure(`${C}/Cargo.lock`, traceName, "0.1.0", depPackages(`${TRACE}/Cargo.toml`, "🧰️framework/Cargo.toml"), [packageNameOf(`${CSV_SNAP}/Cargo.toml`)]);
const SP = "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space";
const CELLS = `${SP}/🪶️sqlite/📦️packages/🦀️rust`;
const cellsName = packageNameOf(`${CELLS}/Cargo.toml`);
for (const art of ["🗂️collection", "🪐️space"]) {
  const crate = `${SP}/🗿️artifacts/${art}/📦️packages/🦀️rust`;
  replaceOnce(`${SP}/🗿️artifacts/${art}/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`, '#[path="../../../../../🪶️sqlite/🦀️.rs"]mod fields;', "use semio_framework_os_space_sqlite_cells as fields;");
  addToSection(`${crate}/Cargo.toml`, "[dependencies]", `${cellsName} = { path = "${relPosix(crate, CELLS)}" }`);
}
log(DRY ? "dry run" : "written");

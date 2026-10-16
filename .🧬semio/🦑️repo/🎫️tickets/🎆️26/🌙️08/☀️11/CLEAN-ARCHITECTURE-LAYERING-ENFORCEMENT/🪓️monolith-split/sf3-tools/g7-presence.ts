// 👥️ sf3 target 5 (presence): the Presence retirement test helper gets one owner per tier. The generic factory and observers become a value-tier test crate; the registry-bound witnesses become a testkit module of value-snapshot-read; ephemeral and bin-06 use that module.
// usage: bun g7-presence.ts [--write]
import {mkdirSync, rmSync, rmdirSync, existsSync} from "node:fs";
import {dirname, relative} from "node:path";
import {log, read, put, addToSection, packageNameOf, propagateEdges, createCrate, membersInsert, depLine, abs, lockEnsure, replaceOnce} from "../dm2-tools/lib.ts";
import {DRY} from "../dm2-tools/dd.ts";

const FW = "🧰️framework";
const OS = `${FW}/🛍️products/💻️os/🔨️modules`;
const OLD = `${OS}/🏪️store/👥️presence/🧪️testing/♻️retirement/🦀️.rs`;
const VALUE = `${FW}/🔨️modules/🌱️value`;
const READ = `${VALUE}/📸️snapshot/👁️read`;
const G_DIR = `${VALUE}/🧪️testing/👥️presence-retirement`;
const G_CRATE = `${G_DIR}/📦️packages/🦀️rust`;
const G_NAME = "semio-framework-value-testing-presence-retirement";
const R_FILE = `${READ}/🧪️testing/👥️presence-retirement/🦀️.rs`;
const TRACE = `${FW}/🔨️modules/⏱️trace/📦️packages/🦀️rust`;
const rel = (from: string, to: string) => relative(abs(from), abs(to)).replaceAll("\\", "/");

const text = read(OLD);
const iBox = text.indexOf("pub(crate) fn observed_box_close");
const iFinish = text.indexOf("pub(crate) fn finish(owner");
const iAdmit = text.indexOf("pub(crate) fn admit_returned_string");
if (![iBox, iFinish, iAdmit].every(i => i > 0) || !(iBox < iFinish && iFinish < iAdmit)) throw new Error("helper layout changed");
const generic = text.slice(0, iBox) + text.slice(iFinish, iAdmit);
const bound = text.slice(iBox, iFinish) + text.slice(iAdmit);
const pubify = (s: string) => s.replaceAll("pub(crate) ", "pub ");
const gText = pubify(generic).replace("//! 🧪️ Test-only Presence factories report their original physical ownership receipts.", "//! 🧪️ Test-only Presence factories report their original physical ownership receipts (the generic factory and the heap observers; the snapshot-read bound witnesses live with value-snapshot-read).").trimEnd() + "\n";
const rText = `//! 👥️ Presence retirement witnesses that need the snapshot-read registry: boxed closes, returned-string admission and registry alias closure; the generic factory and observers are re-exported from the testing crate.
use semio_framework_value::{retained_clone::{RetainedCloneGrant, RetainedCloneStep}, ErasedSnapshotRetirement};
use semio_framework_value_testing_presence_retirement::{observed_step, CLOSE_GRANT};
use std::sync::Arc;

pub use semio_framework_value_testing_presence_retirement::*;

${pubify(bound).replaceAll("semio_framework_value_snapshot_read::", "crate::").trimEnd()}
`;
log(`generic ${gText.split("\n").length} lines, bound ${rText.split("\n").length} lines`);

createCrate({
  dir: G_CRATE, name: G_NAME, description: "Test-only Presence factories report their original physical ownership receipts: the generic factory and the heap observers shared by the value, store and ephemeral suites.", role: "test", libPath: "../../🦀️.rs", workspaceRel: rel(G_CRATE, FW), id: "value-testing-presence-retirement",
  dependencies: [depLine("semio-framework-trace", G_CRATE, TRACE), depLine("semio-framework-value", G_CRATE, `${VALUE}/📦️packages/🦀️rust`)], framework: true, ownerName: "@semio-tech/value-testing-presence-retirement-rs",
});
membersInsert(`${FW}/Cargo.toml`, G_CRATE.slice(FW.length + 1));
lockEnsure(`${FW}/Cargo.lock`, G_NAME, "0.1.0", ["semio-framework-trace", "semio-framework-value"], []);
put(`${G_DIR}/🦀️.rs`, gText, true);
put(R_FILE, rText, true);

const SR_CRATE = `${READ}/📦️packages/🦀️rust`;
replaceOnce(`${READ}/🦀️.rs`, `#[cfg(test)]\n#[path = "../../../../🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🧪️testing/♻️retirement/🦀️.rs"]\nmod presence_test_retirement;`, `#[cfg(any(test, feature = "testkit"))]\n#[path = "🧪️testing/👥️presence-retirement/🦀️.rs"]\npub mod presence_test_retirement;`);
replaceOnce(`${SR_CRATE}/Cargo.toml`, "[features]\ntestkit = []\n", `[features]\ntestkit = ["dep:semio-framework-trace", "dep:${G_NAME}"]\n`);
addToSection(`${SR_CRATE}/Cargo.toml`, "[dependencies]", depLine(G_NAME, SR_CRATE, G_CRATE, ", optional = true"));
addToSection(`${SR_CRATE}/Cargo.toml`, "[dependencies]", depLine("semio-framework-trace", SR_CRATE, TRACE, ", optional = true"));
addToSection(`${SR_CRATE}/Cargo.toml`, "[dev-dependencies]", depLine(G_NAME, SR_CRATE, G_CRATE));

const EPH = `${OS}/🏪️store/🫧️ephemeral`;
const BIN6 = `${OS}/🏪️store/🧪️tests/🌴️bin-06`;
replaceOnce(`${EPH}/🦀️.rs`, `#[cfg(test)]\n#[path = "../👥️presence/🧪️testing/♻️retirement/🦀️.rs"]\nmod presence_test_retirement;`, `#[cfg(test)]\nuse semio_framework_value_snapshot_read::presence_test_retirement;`);
replaceOnce(`${BIN6}/🦀️.rs`, `#[path = "../../👥️presence/🧪️testing/♻️retirement/🦀️.rs"]\nmod presence_test_retirement;`, `use semio_framework_value_snapshot_read::presence_test_retirement;`);
for (const dir of [`${EPH}/📦️packages/🦀️rust`, `${BIN6}/📦️packages/🦀️rust`]) {
  addToSection(`${dir}/Cargo.toml`, "[dev-dependencies]", depLine("semio-framework-value-snapshot-read", dir, SR_CRATE, ', features = ["testkit"]'));
}

const TS = `${OS}/🏪️store/🧪️tests/🔬️unit/🟦️.ts`;
replaceOnce(TS, '"../../👥️presence/🧪️testing/♻️retirement/🦀️.rs",', `"${rel(dirname(TS), `${G_DIR}/🦀️.rs`)}",`);

const edges = [
  {consumer: packageNameOf(`${SR_CRATE}/Cargo.toml`), dep: G_NAME, dev: true},
  {consumer: packageNameOf(`${SR_CRATE}/Cargo.toml`), dep: "semio-framework-trace", dev: true},
];
const SR_NAME = packageNameOf(`${SR_CRATE}/Cargo.toml`);
lockEnsure("Cargo.lock", G_NAME, "0.1.0", ["semio-framework-trace", "semio-framework-value"], [SR_NAME]);
lockEnsure("Cargo.lock", "semio-framework-trace", "0.1.0", [], [SR_NAME]);
const result = propagateEdges(edges, [{name: G_NAME, version: "0.1.0", deps: ["semio-framework-trace", "semio-framework-value"]}]);
log(`locks changed: ${result.changed.length}; skipped: ${result.skipped.length}`);
for (const l of result.changed) log(`  ${l}`);
if (!DRY) {
  rmSync(abs(OLD));
  for (const d of [dirname(OLD), dirname(dirname(OLD))]) if (existsSync(abs(d))) { try { rmdirSync(abs(d)); } catch { log(`  kept non-empty ${d}`); } }
  log(`deleted ${OLD}`);
} else log(`(dry) delete ${OLD}`);
void mkdirSync;
log(DRY ? "dry run" : "written");

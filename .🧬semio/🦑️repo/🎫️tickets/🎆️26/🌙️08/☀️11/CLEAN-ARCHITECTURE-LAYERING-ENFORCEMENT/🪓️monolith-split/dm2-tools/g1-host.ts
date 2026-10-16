// 🔌️ Plugin-host test sites: deadline and wake-root tests move to their owning crates, the receiving authority and the channel fixture become imports of their owners.
import {DRY, log, replaceOnce, addToSection, propagateEdges} from "./lib.ts";
const P = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const ENGINE = `${P}/🖥️host/🔧️engine/📦️packages/🦀️rust`;
const WAKE = `${P}/🖥️host/🔔️wake/📦️packages/🦀️rust`;
const INTERP = `${P}/🧠️interpreter/📦️packages/🦀️rust`;
const RUNTIME = `${P}/🧠️interpreter/🏃️runtime/📦️packages/🦀️rust`;
const HOST = `${P}/🖥️host/📦️packages/🦀️rust`;

replaceOnce(`${INTERP}/Cargo.toml`, `\n[[test]]\nname = "original_host_deadline"\npath = "../../../🖥️host/⏱️deadline/🧪️tests/🦀️.rs"\n\n[[test]]\nname = "original_host_root"\npath = "../../../🖥️host/🔔️wake/🎟️root/🧪️tests/🦀️.rs"\n`, "");
replaceOnce(`${ENGINE}/Cargo.toml`, "[dev-dependencies]\n", `[dev-dependencies]\nserde_json = { workspace = true }\n\n[[test]]\nname = "original_host_deadline"\npath = "../../../⏱️deadline/🧪️tests/🦀️.rs"\n`);
replaceOnce(`${P}/🖥️host/⏱️deadline/🧪️tests/🦀️.rs`, `#[path="../🦀️.rs"]\nmod original_deadline;\nuse original_deadline::{OriginalDeadlineError,remaining_original_deadline_us};`, `use semio_framework_plugin_host_engine::original_deadline::{OriginalDeadlineError,remaining_original_deadline_us};`);
addToSection(`${WAKE}/Cargo.toml`, "[dev-dependencies]", `\n[[test]]\nname = "original_host_root"\npath = "../../🎟️root/🧪️tests/🦀️.rs"`);
replaceOnce(`${P}/🖥️host/🔔️wake/🎟️root/🧪️tests/🦀️.rs`, `#[path="../🦀️.rs"]\nmod original;\nuse original::*;`, `use semio_framework_plugin_host_wake::{GuestRelayOriginalRoot,GuestRelayRootPolicy};`);
replaceOnce(`${P}/🧠️interpreter/🏃️runtime/🦀️.rs`, `#[cfg(test)]\n#[allow(dead_code)]\n#[path = "../../🖥️host/🧪️tests/🛂️receiving/🦀️.rs"]\npub(crate) mod test_native_authority;`, `#[cfg(test)]\npub(crate) use semio_framework_plugin_host_operation::test_native_authority;\n\n#[cfg(any(test, feature = "testkit"))]\n#[path = "🧪️tests/🧫️fixtures/🌱️channel/🦀️.rs"]\npub mod channel_fixture;`);
replaceOnce(`${P}/🧠️interpreter/🏃️runtime/🧪️tests/🧫️fixtures/🌱️channel/🦀️.rs`, `use super::OwnedSemioExport;`, `use semio_framework_plugin_interpreter::OwnedSemioExport;`);
replaceOnce(`${P}/🧠️interpreter/🏃️runtime/🧪️tests/🔬️owned-runtime/🦀️.rs`, `#[path="../🧫️fixtures/🌱️channel/🦀️.rs"]\nmod channel_fixture;\npub(super) use channel_fixture::owned_channel_guest;`, `pub(super) use crate::channel_fixture::owned_channel_guest;`);
replaceOnce(`${RUNTIME}/Cargo.toml`, `[dev-dependencies]\n`, `[features]\ntestkit = []\n\n[dev-dependencies]\nsemio-framework-plugin-host-operation = { path = "../../../../🖥️host/🚪️io/🎛️operation/📦️packages/🦀️rust", features = ["testkit"] }\n`);
replaceOnce(`${P}/🖥️host/🏗️instance/🧪️tests/🦀️.rs`, `#[path="../../../🧠️interpreter/🏃️runtime/🧪️tests/🧫️fixtures/🌱️channel/🦀️.rs"]\nmod channel_fixture;`, `use semio_framework_plugin_interpreter_runtime::channel_fixture;`);
replaceOnce(`${HOST}/Cargo.toml`, `semio-framework-plugin-interpreter = { workspace = true }\nsemio-framework-job = { workspace = true, features = ["testkit"] }`, `semio-framework-plugin-interpreter = { workspace = true }\nsemio-framework-plugin-interpreter-runtime = { workspace = true, features = ["testkit"] }\nsemio-framework-job = { workspace = true, features = ["testkit"] }`);
const result = propagateEdges([
  {consumer: "semio-framework-plugin-interpreter-runtime", dep: "semio-framework-plugin-host-operation", dev: true},
  {consumer: "semio-framework-plugin-host-engine", dep: "serde_json", dev: true},
], []);
log(`locks changed: ${result.changed.length}`);
for (const l of result.changed) log(`  ${l}`);
log(`skipped: ${result.skipped.length}`);

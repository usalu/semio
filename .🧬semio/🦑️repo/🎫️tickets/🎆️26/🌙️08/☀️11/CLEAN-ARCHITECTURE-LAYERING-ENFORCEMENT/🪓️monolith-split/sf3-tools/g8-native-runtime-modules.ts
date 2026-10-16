// 📥️ sf3 target 3 (native-entrypoint): the native runtime manifest reader module becomes its own os crate; program-bridge re-exports it and the staged reader binary depends on it instead of mounting the source.
// usage: bun g8-native-runtime-modules.ts [--write]
import {relative} from "node:path";
import {log, read, put, addToSection, packageNameOf, propagateEdges, createCrate, membersInsert, depLine, abs, replaceOnce, lockEnsure} from "../dm2-tools/lib.ts";
import {DRY} from "../dm2-tools/dd.ts";

const WGPU = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu";
const MODULES = `${WGPU}/⌨️native-entrypoint/📦️modules`;
const CRATE = `${MODULES}/📦️packages/🦀️rust`;
const NAME = "semio-framework-os-renderer-wgpu-native-runtime-modules";
const BRIDGE = `${WGPU}/🪢️program-bridge`;
const BRIDGE_CRATE = `${BRIDGE}/📦️packages/🦀️rust`;
const READER = `${MODULES}/🧪️tests/📥️reader`;
const READER_CRATE = `${READER}/📦️packages/🦀️rust`;
const rel = (from: string, to: string) => relative(abs(from), abs(to)).replaceAll("\\", "/");

createCrate({
  dir: CRATE, name: NAME, description: "Native runtime manifest and JSON page reader: the compact profile-specific artifact manifest with rejected ambiguous identities and nonportable paths.", role: "framework", libPath: "../../🦀️.rs", workspaceRel: rel(CRATE, "."), id: "renderer-wgpu-native-runtime-modules",
  dependencies: ['serde = { version = "1.0.219", features = ["derive"] }', 'serde_json = "1.0.140"'], framework: true, ownerName: "@semio-tech/renderer-wgpu-native-runtime-modules-rs",
});
membersInsert("Cargo.toml", CRATE);
lockEnsure("Cargo.lock", NAME, "0.1.0", ["serde", "serde_json"], []);
replaceOnce(`${BRIDGE}/🦀️.rs`, `#[cfg(not(target_arch = "wasm32"))]\n#[path = "../⌨️native-entrypoint/📦️modules/🦀️.rs"]\npub mod native_runtime_modules;`, `#[cfg(not(target_arch = "wasm32"))]\npub use semio_framework_os_renderer_wgpu_native_runtime_modules as native_runtime_modules;`);
addToSection(`${BRIDGE_CRATE}/Cargo.toml`, "[dependencies]", depLine(NAME, BRIDGE_CRATE, CRATE));
replaceOnce(`${READER_CRATE}/🦀️.rs`, `#[path = "../../../../🦀️.rs"]\nmod native;`, `use semio_framework_os_renderer_wgpu_native_runtime_modules as native;`);
addToSection(`${READER_CRATE}/Cargo.toml`, "[dependencies]", depLine(NAME, READER_CRATE, CRATE));
const result = propagateEdges([{consumer: packageNameOf(`${BRIDGE_CRATE}/Cargo.toml`), dep: NAME, dev: false}], [{name: NAME, version: "0.1.0", deps: ["serde", "serde_json"]}]);
log(`locks changed: ${result.changed.length}; skipped: ${result.skipped.length}`);
for (const l of result.changed) log(`  ${l}`);
log(DRY ? "dry run" : "written");
void read; void put;

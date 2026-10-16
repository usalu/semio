// 🧮️ The requested-allocation observer has one owner (`trace-memory-testing`); the mcp crates depend on it instead of mounting its source through the foundation.
import {DRY, log, read, put, replaceOnce, replaceRegexOnce, packageNameOf, propagateEdges, propagateRemovals} from "./lib.ts";
const M = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp";
const TRACE = "🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/📦️packages/🦀️rust";
const traceName = packageNameOf(`${TRACE}/Cargo.toml`);
const FOUNDATION = `${M}/🧱️foundation/📦️packages/🦀️rust`;
replaceOnce(`${M}/🧱️foundation/🦀️.rs`, `#[cfg(any(test, feature = "testkit"))]\n#[path = "../../../../../🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]\npub mod test_allocation;\n\n`, "");
replaceOnce(`${FOUNDATION}/Cargo.toml`, `[features]\ntestkit = ["dep:semio-framework-trace"]\n\n`, "");
replaceOnce(`${FOUNDATION}/Cargo.toml`, `semio-framework-trace = { workspace = true, optional = true }\n\n[dev-dependencies]\nsemio-framework-trace = { workspace = true }\n`, "");
const crates: {dir: string; sources: string[]; foundationLine: string}[] = [
  {dir: `${M}/⚙️plugin-runtime/📦️packages/🦀️rust`, sources: [`${M}/⚙️plugin-runtime/🦀️.rs`, `${M}/🏠️workspace/🪶️sqlite/🧪️tests/guest-input-slot.rs`], foundationLine: `semio-framework-os-mcp-foundation = { path = "../../../🧱️foundation/📦️packages/🦀️rust", features = ["testkit"] }\n`},
  {dir: `${M}/📦️packages/🦀️rust`, sources: [`${M}/📦️packages/🦀️rust/🦀️.rs`, `${M}/🏠️workspace/🧪️tests/🔬️quick/🦀️.rs`], foundationLine: `semio-framework-os-mcp-foundation = { path = "../../🧱️foundation/📦️packages/🦀️rust", features = ["testkit"] }\n`},
];
const edges = [];
for (const crate of crates) {
  const rel = (to: string) => require("node:path").relative(`/Users/ueli/Documents/semio/${crate.dir}`, `/Users/ueli/Documents/semio/${to}`);
  replaceOnce(`${crate.dir}/Cargo.toml`, crate.foundationLine, `${traceName} = { path = "${rel(TRACE)}" }\n`);
  for (const source of crate.sources) {
    const text = read(source);
    const count = text.split("semio_framework_os_mcp_foundation::test_allocation::").length - 1;
    if (count === 0) throw new Error(`no test_allocation use in ${source}`);
    put(source, text.replaceAll("semio_framework_os_mcp_foundation::test_allocation::", "semio_framework_trace_memory_testing::"));
  }
  edges.push({consumer: packageNameOf(`${crate.dir}/Cargo.toml`), dep: traceName, dev: true});
}
const removed = propagateRemovals([{consumer: "semio-framework-os-mcp-foundation", dep: "semio-framework-trace", dev: true}]);
const added = propagateEdges(edges, []);
log(`locks: removed ${removed.length}, added ${added.changed.length}, skipped ${added.skipped.length}`);
for (const l of [...removed, ...added.changed]) log(`  ${l}`);

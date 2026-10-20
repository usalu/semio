import { create, edit, load, set } from "./lib.mjs";
import { HOST, src } from "./paths.mjs";

const ENGINE = `${HOST}/🔧️engine`;
const COMMENT = "# 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): `component-model-async` is\n# declared EXPLICITLY rather than left to feature unification through `wasmtime-wasi` —\n# `Config::wasm_component_model_async`, `Store::run_concurrent`, `Accessor`, `StreamReader` and the\n# `*_async`/concurrent call shapes of every `bindgen!` export are all `#[cfg(feature = ...)]`-gated\n# on it, and the collapsed `world actor` cannot be linked or called without them.\n";
const WASMTIME = 'wasmtime = { version = "47.0.3", features = ["cranelift", "component-model", "component-model-async"] }\n';
const WASI = 'wasmtime-wasi = "47.0.3"\n';
const USE_WASM = "use semio_framework_plugin_host_engine::wasm;\n";

export const rewrites = {};

function toPort(path, text) {
  let count = 0;
  const lines = text.split("\n").map((line) => {
    if (/^\s*\/\//.test(line)) return line;
    return line.replace(/(?<![\w"])(?<!::)wasmtime_wasi::/g, () => (count++, "wasm::wasi::")).replace(/(?<![\w"])(?<!::)wasmtime::/g, () => (count++, "wasm::"));
  });
  rewrites[path] = count;
  return lines.join("\n");
}

function port(path) {
  const text = load(path);
  const lines = toPort(path, text).split("\n");
  const first = lines.findIndex((line) => /^(pub\(crate\) )?use wasm::/.test(line));
  if (first >= 0) lines.splice(first, 0, USE_WASM.trimEnd());
  const next = lines.join("\n");
  const left = next.split("\n").filter((line) => !/^\s*\/\//.test(line) && /(?<![\w"])(?<!::)wasmtime(_wasi)?::/.test(line));
  if (left.length) throw Error(`unrewritten wasmtime path in ${path}: ${left[0].slice(0, 100)}`);
  set(path, next);
}

export function build() {
  create(`${ENGINE}/🔌️wasm-port/🦀️.rs`, src("wasm/port.rs"));
  edit(`${ENGINE}/🦀️.rs`, `#[path = "../⏱️deadline/🦀️.rs"]\npub mod original_deadline;\n`, `#[path = "../⏱️deadline/🦀️.rs"]\npub mod original_deadline;\n\n#[path = "🔌️wasm-port/🦀️.rs"]\npub mod wasm;\n\n#[cfg(test)]\n#[path = "🧪️tests/🔌️wasm-port/🦀️.rs"]\nmod wasm_port_tests;\n`);
  create(`${ENGINE}/🧪️tests/🔌️wasm-port/🦀️.rs`, src("wasm/port-test.rs"));
  edit(`${ENGINE}/📦️packages/🦀️rust/Cargo.toml`, WASMTIME, `${WASMTIME}# 🔌️ The only crate of the plugin host that declares \`wasmtime\` and \`wasmtime-wasi\`: every other host layer reaches them through the explicit re-exports of \`wasm\` (\`🔌️wasm-port/🦀️.rs\`).\n${WASI}`);

  port(`${HOST}/📥️imports/🦀️.rs`);
  edit(`${HOST}/📥️imports/📦️packages/🦀️rust/Cargo.toml`, `${COMMENT}${WASMTIME}${WASI}`, ``);

  const runtime = `${HOST}/🎭️guest-runtime`;
  port(`${runtime}/🦀️.rs`);
  port(`${runtime}/🏠️wasmtime-host-state-and-wit-hosts/🦀️.rs`);
  port(`${runtime}/🎛️guest-runtimes-and-test-doubles/🦀️.rs`);
  port(`${runtime}/🐎️wasmtime-runtime/🦀️.rs`);
  edit(`${runtime}/📦️packages/🦀️rust/Cargo.toml`, `${COMMENT}${WASMTIME}${WASI}`, ``);

  const wit = `${HOST}/🔗️wit-bindings`;
  edit(`${wit}/🦀️.rs`, `    wasmtime::component::bindgen!({\n        world: "actor",\n        path: "../../../../🧬️schema",\n    });`, `    semio_framework_plugin_host_engine::wasm::component::bindgen!({\n        world: "actor",\n        path: "../../../../🧬️schema",\n        wasmtime_crate: ::semio_framework_plugin_host_engine::wasm::generated_code_support,\n    });`);
  edit(`${wit}/📦️packages/🦀️rust/Cargo.toml`, `${COMMENT}${WASMTIME}`, `semio-framework-plugin-host-engine = { path = "../../../🔧️engine/📦️packages/🦀️rust" }\n`);

  const operation = `${HOST}/🚪️io/🎛️operation`;
  port(`${operation}/🐎️wasmtime/🦀️.rs`);
  port(`${operation}/🧪️tests/🐎️wasmtime/🦀️.rs`);
  edit(`${operation}/📦️packages/🦀️rust/Cargo.toml`, `${COMMENT}${WASMTIME}`, `semio-framework-plugin-host-engine = { path = "../../../../🔧️engine/📦️packages/🦀️rust" }\n`);
  return rewrites;
}

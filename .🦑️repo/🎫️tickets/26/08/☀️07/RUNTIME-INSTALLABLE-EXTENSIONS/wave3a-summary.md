# Wave 3.a summary — Flow light builtins as packaged extensions

Ticket: `26/08/07/RUNTIME-INSTALLABLE-EXTENSIONS`

## Crates created

Under `✏️s/🔌️plugins/🌊️flow/️️extensions/<emoji>/` (BIM anatomy):

| Crate | Folder |
|-------|--------|
| `semio-s-plugin-flow-extension-core` | `🫀️core` |
| `semio-s-plugin-flow-extension-math` | `🧮️math` |
| `semio-s-plugin-flow-extension-text` | `📝️text` |
| `semio-s-plugin-flow-extension-logic` | `🧠️logic` |
| `semio-s-plugin-flow-extension-dictionary` | `📖️dictionary` |
| `semio-s-plugin-flow-extension-list` | `📃️list` |

Each crate has: `role=extension`, `extends=flow`, `contributes=["flow.extension"]`, `cdylib+rlib`, owner `️️component.rs` with operators + `pub fn register` + `extension_manifest_json` + `ExtensionGuest`/`extension_exports!(bundle)`, dual `Contribution::FlowExtension` for `flow-play` **and** `procedural3d-play`, `.handler("evaluate", …)` via `flow_extension_sdk::evaluate_json`, no `standalone-wasm`/`WasmExt`.

Root `Cargo.toml` members + aliases added. Host lists them as `[dev-dependencies]` (`default-features = false`).

## Removed from `install_builtin_flow_extensions`

Compile-time registration of the six lights is gone. Current function:

```rust
pub fn install_builtin_flow_extensions(_registry: &mut neural::Registry) {
    // Light/draw/brep operator packs are runtime-installable packaged extensions.
}
```

(Parallel Wave 3.b/3.c also emptied draw/brep from this hook; geometry kernel helpers live under `️️core/️️brep-geometry`.)

## Glue split

`semio-framework-os-flow` `📦️glue.rs` no longer path-mods the six lights. Framework `️️extensions/` retains only `️️wasm` (SDK helpers). Current glue:

```rust
//! 🌊️ OS flow family glue — wires core, brep geometry kernel surface, and wasm SDK.
//! Light/draw/brep operator packs are packaged extensions under ✏️s/🔌️plugins/🌊️flow.

extern crate self as flow_core;
extern crate self as flow_extension_wasm;
extern crate self as flow_extension_sdk;

#[path = "../../🫀️core/🦀️component.rs"]
pub mod core;
pub use core::*;

#[path = "../../🫀️core/📐️brep-geometry/🦀️component.rs"]
pub mod brep_geometry;
pub use brep_geometry::{
    dispose_geometry, export_solid_json, import_solid_json, retain_geometry_handles, tessellate_geometry,
};

#[path = "."]
pub mod extensions {
  #[path = "../../🧩️extensions/🕸️wasm/🦀️component.rs"]
  pub mod wasm;
}

pub use extensions::wasm::*;
```

## Tests

`install_first_party_light_flow_extensions_for_tests()` registers the six (and brep) into `FLOW_EXTENSION_STATE` for host unit tests. Production path = contribution + invoke only; `register` stays public for crate-local tests.

## Remaining blockers

1. **Xcode license** on this agent host blocks `cargo check`/`test` (`cc` exit 69). Re-verify after license accept.
2. **Empty default palette** until extension store/ledger installs the packages (Wave 4/5 E2E).
3. Concurrent **draw/brep** packaging + `brep-geometry` extract need their own wave validation.
4. Repo MCP unavailable this session.

## Audit

`wave3a-crate-audit.json`: all six pass role/extends/guest/exports/no-WasmExt checks.

# Rust Incremental Sizes

Measured 2026-10-14 before teaching `clean` to reclaim them.

| Path | Size |
| --- | --- |
| `.🧬semio/🦑️repo/⚡️cache/cargo` | 234G |
| `.🧬semio/🦑️repo/⚡️cache/cargo/build` | 159G |
| `.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build` | 84G |
| `.🧬semio/🦑️repo/⚡️cache/cargo/build/prototype/incremental` | 29G |
| `.🧬semio/🦑️repo/⚡️cache/cargo/canonical-architecture` | 68G |
| `.🧬semio/🦑️repo/⚡️cache/cargo/canonical-architecture/prototype/incremental` | 26G |
| `.🧬semio/🦑️repo/⚡️cache/cargo/canonical-architecture/wasm32-unknown-unknown/prototype/incremental` | 6.3G |
| `.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/incremental` | 4.5G |

`canonical-architecture` is a Cargo output root (`CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` both point there). Workspace clean protects the whole `⚡️cache`, and cache prune only walks the configured `cargo/build` and `cargo/target` directories, so the canonical-architecture incremental trees were never candidates.

`clean` now removes a rustc `incremental` directory under the shared cargo cache, or under another configured Cargo build/target directory inside the workspace, when that directory is larger than 1 GiB. Crates with a `-working` session stay. Dependency units and uplifted artifacts stay.

# 🦀 Norm Rust Crate Map

- Norm has its own Cargo workspace `✏️s/🔌️plugins/📕️norm/Cargo.toml` (38 members, all present); root `Cargo.toml` excludes norm paths. Each of the 15 `🗿️artifacts/*` is a separate workspace with own Cargo.lock.
- Integrity: 1,015 Cargo.toml, no missing path deps/members/lib paths; 10,834 `#[path]` targets resolve; 11,200 `workspace = true` deps defined.
- nx Rust projects: `@semio-tech/norm-<code>-rs` (15), `@semio-s/plugin-norm-mechanics`, `@semio-tech/norm-artifact-contract-rs`.
- Gaps: 34/38 norm workspace members have no project.json → unreachable from `repo`. Registry projects `semio-s-plugin-norm-inspector-forms`, `semio-s-plugin-norm-app-surface-live-editor` have names but no targets.
- Naming inconsistency: `@semio-s/plugin-norm-mechanics` vs `@semio-tech/*`.
- All manifests: `cargo-features = ["trim-paths"]`, `rust-version = "1.95"`.

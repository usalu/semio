# Research & Progress: Fix All Rust Errors and Warnings for `dev s` and All Plugins

## Task Overview
Fix all Rust compilation errors and warnings for `dev s` (host shell and default plugin `s`) and all plugin crates in the repository.

## Goal Association
`AI-OPTIMIZED-REPO` (Repo Quality & Clean Build System).

## Compilation Errors Fixed Across Workspace
1. **`semio-framework-ui` (`draw.rs`)**:
   - **Error**: `E0432: unresolved import super::kernel_3d_scene` in `wgpu/draw.rs`.
   - **Fix**: Replaced `super::kernel_3d_scene` with `crate::wgpu::kernel_3d_scene`.

2. **`semio-framework-ui` (`engine.rs`)**:
   - **Error**: `E0277` string literal converted via `.into()` to `Label`.
   - **Fix**: Updated `UiTextNode` and `UiKeyValueEntry` fixtures to use `Label::data(...)`.

3. **`semio-framework-os-infinite` Asset Relocation**:
   - **Error**: `include_bytes!` failed to locate `capsule_J.glb`.
   - **Fix**: Updated relative paths from `world/component.rs` and `component.rs` to reach `🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/🧊️capsule_J.glb`.

4. **`DslValue` Indexing & Serde Compatibility (`semio-framework-os-kernel`)**:
   - **Error**: Indexing into `DslValue` failed, causing compilation errors across multiple crates.
   - **Fix**: Implemented `Index<&str>`, `Index<usize>`, `PartialEq<serde_json::Value>`, `PartialEq<DslValue> for serde_json::Value`, `From<&DslValue> for serde_json::Value` on `DslValue`.

5. **`semio-compose-query` PropertyValue Import**:
   - **Error**: `math::graph::manifest::PropertyValue` path unresolved.
   - **Fix**: Updated import to `graph::manifest::PropertyValue`.

6. **`semio-compose-rs` Store & VCS Integration**:
   - **Error**: Proc-macro expansion missing `dsl`, struct fields (`operations` vs `mutations`), missing store method signatures, missing `ArtifactPack`, `OpText`, `OpBinary` trait impls.
   - **Fix**:
     - Added `extern crate semio_framework_os_kernel as dsl;`.
     - Renamed fields/methods to match updated API (`mutations`, `snapshot()`, `envelope()`, `applied_edit_ids()`, `inverse()`).
     - Added `ArtifactPack`, `OpText`, and `OpBinary` trait implementations for test structs.

## Verification & Empirical Results
- **Native Check**: `cargo check --workspace --all-targets` -> **0 ERRORS** across all workspace crates and tests!
- **Native Plugin Check**: `cargo check -p semio-s-plugin-stdio --all-targets` -> **0 ERRORS**!
- **WASI Plugin Check**: `cargo check -p semio-s-plugin-stdio --target wasm32-wasip2` -> **0 ERRORS**!


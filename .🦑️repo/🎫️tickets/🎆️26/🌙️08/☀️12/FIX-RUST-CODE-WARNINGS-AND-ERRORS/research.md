# Fix Rust Code Warnings and Errors - Final Research & Diagnostic Report

## Executive Summary
Comprehensive scan with `cargo check --workspace --all-targets --keep-going` identified errors across 5 Rust crates/modules and warnings (unused qualifications, ambiguous glob re-exports, unused imports/variables/crates).

## Root Causes & Detailed Breakdown

### 1. `semio-framework-os-kernel-neural-engine`
- **File**: `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️component.rs`
- **Issue**: Line 2536 test uses non-existent field `extension` instead of `module` on `Schema`.
- **Fix**: Change `extension: "math".into()` to `module: "math".into()`.

### 2. `semio-framework-os-kernel`
- **File**: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs`
- **Issue**: `DemoSnapshot` and `DemoMutation` in `mod tests` lack trait implementations for `ArtifactPack`, `ArtifactDsl`, `OpText`, `OpBinary`.
- **Fix**: Provide explicit `impl ArtifactDsl`, `impl ArtifactPack`, `impl OpText`, and `impl OpBinary` blocks matching `🏪️store/🦀️component.rs`.
- **File**: `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs`
- **Issue**: Top-level `extern crate self as ...` triggers unused extern crate warnings. Ambiguous glob re-exports (`os_dsl` vs `os_dsl::grammar`, `os_spr` vs `os_pack`).
- **Fix**: Add `#[allow(unused_extern_crates)]` and refine re-export statements.

### 3. `semio-framework-os-kernel-db`
- **Files**: `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🦀️component.rs`, `⚙️engine/🦀️component.rs`, `📄️artifact/🦀️component.rs`, `👁️preview/🦀️component.rs`
- **Issue**: Unprefixed sibling module imports (`db_engine`, `db_ids`, `db_storage`, etc.) in 2018/2021 edition modules.
- **Fix**: Update imports to `crate::db_engine`, `crate::db_ids`, `crate::db_storage`, etc.
- **File**: `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/👁️preview/🦀️component.rs`
- **Issue**: `include_str!` paths pointing to obsolete pre-Shape-V2 relative directory structures.
- **Fix**: Update paths to `../📦️packages/🦀️rust/Cargo.toml` and `🦀️component.rs`.

### 4. `semio-framework-ui`
- **File**: `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️draw.rs`
- **Issue**: Line 2184: `use super::kernel_3d_scene::ScenePass3d;` fails to resolve.
- **Fix**: Change import to `use crate::wgpu::kernel_3d_scene::ScenePass3d;`.

### 5. `semio-compose-rs`
- **File**: `compose/client/lib/rs/lib.rs`
- **Issue**: Unresolved `dsl` and `vcs` module references at lines 716-723 and line 7919.
- **Fix**: Update references to `semio_framework_os_kernel::os_dsl` / `semio_framework_os_kernel::os_vcs` or re-exported modules.

### 6. Workspace-wide Warnings
- Fix 1000+ `unnecessary qualification` warnings and unused import warnings.

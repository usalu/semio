# Cargo Workspace Topology Exploration

## 1. Root `Cargo.toml` Summary

**Location:** `/Users/ueli/Documents/semio/Cargo.toml` (line 1)

### Members
- **Count:** 227 members in `[workspace] members` array (lines 4-236)
- All members are Rust crates under three top-level folders: `🧰️framework`, `✏️s`, `🌎️hub`

### Workspace Dependencies
- **Count:** 200+ workspace-level dependency definitions (lines 251-463)
- Categorized by usage: core (24 refs), math (13 refs), UI (10 refs), plugin (62 refs)
- Includes canonical versions for highest-fanout external deps:
  - `serde = { version = "1.0.228", features = ["derive"] }`
  - `serde_json = "1.0.149"`
  - `wasm-bindgen = "0.2.106"`
  - `js-sys = "0.3.83"`
  - `tokio = { version = "1" }`

### Profile Settings (Full Verbatim)

**[profile.dev]** (lines 465-470)
```
debug = false
incremental = true

[profile.dev.build-override]
opt-level = 3
```

**[profile.wasm-dev]** (lines 474-490)
```
inherits = "dev"
codegen-units = 1

# Package-specific overrides:
[profile.wasm-dev.package.semio-s-artifact-lowpoly-lowpoly]
opt-level = 2

[profile.wasm-dev.package.semio-s-artifact-puzzle-3d]
opt-level = 2
```

**[profile.release]** (lines 494-500)
```
opt-level = 3
lto = "thin"
codegen-units = 1
strip = "debuginfo"
incremental = false
trim-paths = "object"
```

**[profile.wasm-release]** (lines 521-533)
```
inherits = "release"
opt-level = "s"
lto = "thin"
codegen-units = 1
strip = "symbols"
incremental = false
trim-paths = "object"

[profile.wasm-release.package.semio-framework-os-kernel]
codegen-units = 1
```

### Workspace Metadata & Lints
- **Lints:** `[workspace.lints.rust]` and `[workspace.lints.clippy]` (lines 539-555)
  - Clippy all warnings, with phase B future: `unwrap_used = "warn"`
  - Rust idioms and unsafe_op_in_unsafe_fn at "warn"
- **No patch section** defined in root

### Resolver & Package Config
- `resolver = "2"` (line 239)
- `[workspace.package]` provides shared version, edition (2021), rust-version (1.95) (lines 246-249)

---

## 2. Standalone Workspace Roots & Non-Member Crates

### Root Workspace Exclusions
- **No explicit `exclude` list** in root `Cargo.toml`

### Standalone Workspace Roots (52 found, excluding root)
All have `[workspace]` section with their own members, built independently:

**Test/Fixture Fixtures:**
1. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧫️fixtures/🌊️actor-import/👽️guest/📦️packages/🦀️rust/Cargo.toml`
   - Browser fixture; fixture-scoped, not in root member list
2. `🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/Cargo.toml`
   - Testkit probe; fixture-scoped
3. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust/Cargo.toml`
   - Repository test platform host (no external dependencies by design)
4. `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/Cargo.toml`
   - Test oracle for stdio plugin; defines feature `oracles` for third-party reference impls

**Codecs/Standards Generators:**
- 48 artifact-specific standalone workspaces under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/*/🏅️standards/*/🪆️subsets/*/🏭️generator` and `🔬️probes`
  - Examples: `riff-avi-codec`, `image-bmp-codec`, `quick-xml-svg-codec`, `jpeg-jfif-codec`, json/svg engines, oracle-probes
  - Pattern: each format's generator/probe is an isolated workspace to keep build parallelism and versioning independent

**Cargo.lock presence:**
- Only the root workspace and explicitly standalone test/fixture crates check in `Cargo.lock`
- Generator/probe crates do NOT have committed Cargo.lock

**Target directory:**
- Root workspace and all member crates share the root `target/` directory
- Standalone fixtures/generators/probes build to their local adjacent `target/` (e.g., `🧫️fixtures/.../👽️guest/target/`, `🏭️generator/target/`)

---

## 3. Package Inventory from `cargo metadata --no-deps --format-version 1 --offline`

### Total Package Count
- **231 packages** across root workspace members

### Package Distribution by Top-Level Folder
| Folder | Count |
|--------|-------|
| `✏️s/` (plugins/artifacts) | 159 |
| `🧰️framework/` (core framework) | 71 |
| `🌎️hub/` (inference/services) | 1 |

### Target Architecture Breakdown

**wasm32-wasip2 Component Plugins (60 packages)**
- All have `[package.metadata.component]` section defining WASI component
- Compiled for `wasm32-wasip2` target (wasi-preview-2)
- Include crate-type `"cdylib"` for component linking
- Examples:
  - `semio-s-artifact-procedural-generation3d`
  - `semio-s-plugin-cad`
  - `semio-s-plugin-flow`
  - `semio-s-plugin-puzzle`
  - `semio-s-artifact-stdio-*` (60+ stdio artifact formats)
  - All 60 participate in the plugin lifecycle and build with `--profile wasm-release`

**wasm32-unknown-unknown (browser renderer - estimated 1-2 packages)**
- Target for browser-side WASM execution
- Uses `[target.wasm32-unknown-unknown]` rustflags in `.cargo/config.toml` (line 16)
- Shadow stack: 16 MiB reserved for wgpu debug builds
- Packages: likely `semio-framework-ui-render` and related UI packages (inferred from config)

**Native-only (168 packages)**
- Standard Rust library and binary crates
- Compiled for host target (`x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, macOS, Windows)
- Include framework core, UI host, shell, plugin host, testkit, tools

### Packages with Multiple Features
**Feature-varied packages requiring distinct artifacts in shared build-dir:**

- `semio-framework-os-kernel`: `["default", "deflate", "sync", "testkit", "typegen", "ureq", "worker"]`
- `semio-framework-ui`: `["default", "testkit", "tui", "tui-bindgen", "tui-terminal", "typegen", "wgpu", "wgpu-engine"]`
- `semio-framework-plugin`: `["component-extension-guest", "component-guest", "component-guest-async", "default"]`
- `semio-s-artifact-stdio-semio`: `["component-app-assembly", "conversion-*" (13 formats)]`
- All `semio-s-artifact-*` packages: `["component-app-assembly", "default"]` (distinct builds for plugin vs. test contexts)

**Impact on shared build-dir:**
- A single cargo invocation may build the same package multiple times with different feature sets
- Example: `semio-framework-os-kernel` with `--profile wasm-release --features testkit` vs. without
- Artifact filename varies by feature combination; `-Zchecksum-freshness` detects missing/stale rebuilds

---

## 4. Build Scripts & Side Effects

### `build.rs` Files (2 found)

1. **`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/build.rs`** (134 lines)
   - **Purpose:** Generate icon shortcode lookup tables and embed SVG catalog assets
   - **Env vars read:** `CARGO_MANIFEST_DIR`, `OUT_DIR`
   - **Side effects:** 
     - Reads external asset paths (up 6 dirs from manifest: `🔨️modules/🖼️assets/`)
     - **Writes outside OUT_DIR:** NONE (all files written to `OUT_DIR/🔎️shortcodes.rs`, `OUT_DIR/🖼️catalog/`, `OUT_DIR/🌱️metabolism/`)
     - Copies icon SVG files into OUT_DIR for inclusion via `include_str!()` macro
     - Dependency scanning: `cargo:rerun-if-changed` on asset sources (lines 51-53)
   - **Caching implications:** Deterministic output; depends on fixed asset files; safe for shared build-dir

2. **`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/build.rs`** (7 lines)
   - **Purpose:** ASCII-named stub that includes canonical builder logic
   - **Env vars read:** NONE (deferred to include file)
   - **Side effects:** NONE (include target `🏗️builder/🦀️.rs` does not exist yet; currently a no-op)
   - **Caching implications:** Harmless; safe for shared build-dir

### No OTHER problematic build scripts detected
- No build.rs writes to paths outside `OUT_DIR`
- No environment variable pollution that would break fingerprinting
- Safe for `-Zchecksum-freshness` caching

---

## 5. Toolchain & Configuration Files

### `rust-toolchain.toml` (root only)
**Location:** `/Users/ueli/Documents/semio/rust-toolchain.toml` (5 lines)
```toml
[toolchain]
channel = "nightly-2026-07-07"
components = ["rust-src", "llvm-tools-preview"]
targets = ["wasm32-unknown-unknown", "wasm32-wasip2"]
```
- **Targets declared:** Both wasm targets + implicit host target
- **No per-crate toolchains** (no rust-toolchain.toml in subdirectories)

### `.cargo/config.toml` (root only)
**Location:** `/Users/ueli/Documents/semio/.cargo/config.toml` (31 lines)

**Build configuration:**
```toml
[build]
rustc-wrapper = "sccache"
rustflags = ["-Z", "threads=8"]

[unstable]
no-embed-metadata = true
```

**Target-specific rustflags:**
- `[target.wasm32-unknown-unknown]`: 16 MiB shadow stack for wgpu debug
- `[target.wasm32-wasip2]`: 512 MiB memory max, -Z threads=8
- `[target.x86_64-unknown-linux-gnu]`: `--link-arg=-fuse-ld=mold`
- `[target.aarch64-unknown-linux-gnu]`: `--link-arg=-fuse-ld=mold`

- **No per-crate .cargo/config.toml** (no overrides in subdirectories)

---

## Summary Statistics

| Metric | Value |
|--------|-------|
| Root workspace members | 227 |
| Standalone workspace roots | 52 (fixtures, generators, test oracle) |
| Total packages in metadata | 231 |
| wasm32-wasip2 component plugins | 60 |
| Packages with features | ~50 (varying by build context) |
| build.rs scripts | 2 (both safe for shared build-dir) |
| Profiles defined | 4 (dev, wasm-dev, release, wasm-release) |
| workspace.dependencies entries | 200+ |
| Rust toolchain versions | 1 (nightly-2026-07-07) |

---

## Implications for One Shared Build-Dir

### ✅ Compatible with `-Zbuild-dir` and `-Zfine-grain-locking`

1. **Feature Matrix Correctness**
   - 60+ wasm32-wasip2 plugins build with `--profile wasm-release --features component-app-assembly`
   - Same packages build natively with different features for tests (e.g., `--features testkit`)
   - **Risk:** Feature-set collisions on re-runs with partial feature selection
   - **Mitigation:** `-Zchecksum-freshness` detects stale artifacts; both `build.rs` scripts are deterministic

2. **Profile Layering**
   - `[profile.wasm-dev.package.*]` and `[profile.wasm-release.package.*]` overrides are per-package
   - Profiles inherit cleanly; no cross-package conflicts
   - **Risk:** Dev loop overrides for lowpoly/puzzle3d have hard-coded `opt-level = 2`; these must persist across the shared dir

3. **Build Script Safety**
   - Both `build.rs` files write only to `OUT_DIR`
   - Asset dependencies are static and version-pinned (no dynamic repo walks)
   - `cargo:rerun-if-changed` declarations properly scan only committed assets
   - **Safe:** No fingerprint pollution

4. **Standalone Workspaces**
   - 52 fixture/generator/oracle workspaces build to their local `target/` dirs
   - **Do not pollute** the root shared `target/` dir
   - Safe for concurrent builds

5. **Target Architecture Separation**
   - `target/wasm32-wasip2/` holds 60 plugin `.wasm` components
   - `target/wasm32-unknown-unknown/` holds 1-2 browser renderer artifacts
   - `target/{x86_64,aarch64}-(linux,macos,windows)/` holds native binaries
   - **No cross-target contamination**

6. **Workspace Dependencies Optimization**
   - 200+ pre-defined workspace.dependencies entries enable `.workspace = true` adoption incrementally
   - Each crate can opt in without modifying root Cargo.toml or 630 consumer manifests
   - **Safe:** Purely additive; no existing member uses `.workspace = true` yet

### ⚠️ Critical Constraints

1. **Incremental Cache Hygiene**
   - `[profile.release]` and `[profile.wasm-release]` disable `incremental = false`
   - `[profile.dev]` enables `incremental = true` (native dev loops only)
   - **Constraint:** Ensure `-Zbuild-dir` does NOT share incremental metadata across targets
   - **Mitigation:** Separate `debug/incremental` by target (`debug/incremental/{wasm32-wasip2,wasm32-unknown-unknown,x86_64}`)

2. **Feature-Scoped Rebuilds**
   - `semio-framework-os-kernel` with 7+ feature combinations
   - `semio-framework-ui` with 7+ feature combinations (especially `wgpu` vs. `tui` vs. neither)
   - `semio-s-artifact-stdio-*` rebuild for test-time with `"oracles"` feature
   - **Constraint:** Each distinct feature set = separate rmeta + rlib + dylib in the shared dir
   - **Validation:** Run `cargo build -p <name> --features feat1 && cargo build -p <name> --features feat2` and verify both artifacts exist

3. **Plugin Codegen Coupling**
   - All 60 plugins link against `wit-bindgen 0.57.1` (hard-pinned in testkit fixture)
   - Any upstream schema change requires re-running `generate!` macro across all plugins
   - **Constraint:** Pin wit-bindgen to workspace.dependencies; do not allow per-plugin overrides
   - **Validation:** `verify dependencies literal-external` audit confirms no drift

4. **sccache Determinism**
   - `rustc-wrapper = "sccache"` requires deterministic builds
   - `[profile.release]` and `[profile.wasm-release]` set `trim-paths = "object"` and `incremental = false`
   - **Constraint:** Do not use `-Zincremental` or rebuild under different working directories
   - **Validation:** Two runs with identical inputs must produce byte-identical artifacts

### ⚠️ Recommendations for Implementation

1. **Partition `target/` by Target + Profile Pair**
   - Root Cargo.toml defines `build.build-dir = "target-shared"`
   - Intern configs use `-Zbuild-dir=target-shared/target-{wasm32-wasip2,wasm32-unknown-unknown,native}/{dev,release,wasm-dev,wasm-release}`
   - Keeps cross-compilation safe under concurrent `-Zfine-grain-locking`

2. **Pre-cache Plugin .wasm Artifacts**
   - Plugins (60 packages) are relatively stable
   - `cargo build -p semio-s-artifact-* --profile wasm-release --features component-app-assembly` can be parallelized across N shards
   - Store artifact hashes in `.🧬semio/🦑️repo/⚡️cache/plugin-wasm-hashes.json` to detect stale binaries

3. **Separate Incremental Metadata by Target**
   - Do NOT share `debug/incremental` between wasm32-wasip2 and wasm32-unknown-unknown
   - Create separate `incremental` dirs: `target-shared/target-wasm32-wasip2/debug/incremental`, etc.

4. **Monitor build.rs Changes**
   - Both `build.rs` scripts are currently safe
   - If either gains `env::var("SEMIO_BUILD_VARIANT")` or similar dynamic behavior, audit it immediately
   - Add a CI gate: `cargo build -p <name> --offline` must never fail due to missing asset dependencies

5. **Feature Matrix Testing**
   - Validate nx `target-caching` against at least 3 feature combinations per multi-feature package
   - Example: `semio-framework-ui` with `[wgpu, tui, neither]`
   - Confirm that Nx's artifact cache tracks feature combinations correctly (nx does not natively do this; Cargo does)

# Lowpoly Plugin: Build, Test, Lint & Boot Guide

## 1. Exact Build & Verification Commands

### A. Cargo Checks (Native & WASM)
```bash
cd "/Users/ueli/Documents/semio"

# Native build check (host target)
cargo check -p semio-s-plugin-lowpoly

# WASM32-WASIP2 check (component target)
cargo check -p semio-s-plugin-lowpoly --target wasm32-wasip2

# Clippy linting with all warnings as errors
cargo clippy -p semio-s-plugin-lowpoly --all-targets -D warnings

# Unit tests (budgeted via framework)
nx run "@semio-tech/lowpoly-plugin:test"
# Or direct: bun ./📜️script.ts test

# Quick tests only
nx run "@semio-tech/lowpoly-plugin:test-quick"

# Long-running tests
nx run "@semio-tech/lowpoly-plugin:test-long"

# Exhaustive test suite
nx run "@semio-tech/lowpoly-plugin:test-exhaustive"
```

### B. WASM Component Build & Descriptor Generation
```bash
cd "/Users/ueli/Documents/semio"

# Build WASM component for wasm32-wasip2 target
cargo build -p semio-s-plugin-lowpoly --target wasm32-wasip2 --release

# Generate descriptor files (🛂️descriptor.semio + 🔣️descriptor.json)
# Located at: ✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/
nx run "@semio-tech/lowpoly-plugin:describe"
# Or direct: bun ./✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/📜️script.ts describe
```

### C. TypeScript Package Build & Test
```bash
cd "/Users/ueli/Documents/semio"

# Test lowpoly TypeScript package (interactive-job fixture validation)
nx run "@semio-tech/lowpoly-js:test"
# Or direct: bun ./✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📜️script.ts test
```

### D. Boot Development Environment
```bash
cd "/Users/ueli/Documents/semio"

# Launch lowpoly dev app (React UI + WASM plugin, port 6078 for React, 6178 for wgpu)
bun ./📜️script.ts dev lowpoly

# Via nx directly:
bun nx run "@semio-tech/framework-os-dev:dev" -- lowpoly

# The dev server will:
#   - Start framework-os dev shell (hosting lowpoly plugin)
#   - Load semio_s_plugin_lowpoly_component.core.wasm from dev cache
#   - Open browser to http://localhost:6078 (React) or 6178 (wgpu variant)
```

---

## 2. Crate Metadata

**Rust Plugin Crate:**
- **Name:** `semio-s-plugin-lowpoly`
- **Path:** `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust`
- **Manifest:** `Cargo.toml`
- **Lib Entry Point:** `📦️glue.rs` (defined in Cargo.toml `[lib] path = "📦️glue.rs"`)
- **Crate Types:** `cdylib` (WASM component) + `rlib` (Rust library)

**Features:**
- `cad-fixtures` (optional): Enables CAD fixture round-trip tests when `dep:cad_plugin` is available
  - Note: `cad_plugin` is embedded as a library (not a component), so cannot link with lowpoly's `#[no_mangle] plugin_exports!`
  - Both crates define `semio_plugin_install_bundle`, causing duplicate symbol errors at link time
  - Error only surfaces during actual WASM build, never in `cargo check` or `cargo test`

**Direct Dependencies:**
```
semio-framework (workspace)
semio-framework-job (workspace)
semio-framework-ui-contract (workspace)
semio-s-plugin-stdio (path)
semio-framework-os-kernel (path)
semio-framework-3d (path)
semio-framework-plugin (path, features: ["component-guest"])
semio-framework-dispatch-macros (path)
semio-framework-schema (path)
serde (workspace)
serde_json (workspace)
base64 0.22.1
png 0.17.16
```

**Dev Dependencies:**
```
semio-framework-async-macros (path)
```

**TypeScript Package:**
- **Name:** `@semio-tech/lowpoly-js`
- **Path:** `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript`
- **Primary Test:** Interactive job fixture validation (47 routes total: 19 Migrated + 28 BatchOnlyPendingRewrite)

---

## 3. Workspace Health

**Cargo Workspace Status:** ✅ **HEALTHY**
```
Exit Code: 0
```

All 126 members resolved successfully. Lowpoly is a proper workspace member at:
```toml
# Cargo.toml workspace.members (line 75)
"✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust"
```

**No workarounds needed.** The workspace uses standard resolver = "2" with workspace.package and workspace.dependencies for shared configuration.

---

## 4. All Nx Targets for Lowpoly

### Rust Plugin Package (`@semio-tech/lowpoly-plugin`)

Located in: `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/📋️project.json`

| Target | Command | Purpose |
|--------|---------|---------|
| `test` | `bun ./📜️script.ts test` | Run budgeted unit tests (default) |
| `test-quick` | `bun ./📜️script.ts test quick` | Fast test subset |
| `test-long` | `bun ./📜️script.ts test long` | Extended tests |
| `test-exhaustive` | `bun ./📜️script.ts test exhaustive` | Full test suite (uncached) |
| `describe` | `bun ./📜️script.ts describe` | Build WASM component + emit 🛂️descriptor.semio & 🔣️descriptor.json |

### TypeScript Package (`@semio-tech/lowpoly-js`)

Located in: `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📋️project.json`

| Target | Command | Purpose |
|--------|---------|---------|
| `test` | `bun ./📜️script.ts test` | Validate interactive-job fixture schema/structure |

---

## 5. Launch Configuration Entries

**File:** `.claude/launch.json`

Current entries do **NOT** include lowpoly-specific launch config. Available generic plugin dev entries:

```json
{
  "name": "s-react",
  "runtimeExecutable": "bun",
  "runtimeArgs": ["./📜️script.ts", "dev", "s"],
  "port": 6070
}
```

**To boot lowpoly dev app, use:**
```bash
# Direct shell command:
bun ./📜️script.ts dev lowpoly

# Or add to .claude/launch.json (optional):
{
  "name": "lowpoly-react",
  "runtimeExecutable": "bun",
  "runtimeArgs": ["./📜️script.ts", "dev", "lowpoly"],
  "port": 6078
}
```

---

## 6. WASM Artifact Path & Registration

**Artifact Output Location (Dev Environment):**
```
🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/lowpoly/
  ├── semio_s_plugin_lowpoly_component.core.wasm  ← Main WASM binary
  ├── semio_s_plugin_lowpoly_component.js         ← JS glue code
  ├── semio_s_plugin_lowpoly.js                   ← Host shim
  ├── 🟨️host-shim.js
  ├── 🟨️plugin-worker.js
  └── interfaces/                                  ← Type definitions
```

**Plugin Registration:**
- Crate Path: `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust`
- Plugin ID: `lowpoly`
- WASM Output: `semio_s_plugin_lowpoly.wasm` (logical name; actual file is `semio_s_plugin_lowpoly_component.core.wasm`)
- Role: `plugin`
- Loaded by: Framework OS dev shell via plugin registry catalog
- Playground Variant: Yes (line 19 in Cargo.toml: `[[package.metadata.semio.playground]] variant = "lowpoly"`)

**Discovery & Load Flow:**
1. Dev app queries playground registry for variant `lowpoly`
2. Registry points to crate path & expected WASM output name
3. Dev shell compiles Rust → WASM (wasm32-wasip2 target)
4. WASM placed in `.../🔌️plugin-modules/lowpoly/`
5. Framework OS loads & instantiates component in host

---

## 7. Integration Points

**TypeScript Consumer:**
- No npm package exports yet (marked private in package.json)
- Interactive job fixture test validates schema alignment

**Artifact Standards:**
- Version: 1 (v1 standards, see `🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/`)
- Subset: `✳️any` (universal, platform-agnostic)

**Editor/Viewer:**
- Editor module: `✏️editor` (full mesh & paint editing)
- Viewer module: `👁️viewer` (read-only render)
- Both compiled into single plugin binary via modular decomposition

---

## 8. Common Workflows

### Quick Verification Loop
```bash
cd /Users/ueli/Documents/semio

# 1. Check everything compiles
cargo check -p semio-s-plugin-lowpoly --target wasm32-wasip2

# 2. Run quick tests
nx run "@semio-tech/lowpoly-plugin:test-quick"

# 3. Boot dev app & test interactively
bun ./📜️script.ts dev lowpoly
# → Open http://localhost:6078
```

### Pre-Commit Validation
```bash
cd /Users/ueli/Documents/semio

# Clippy + tests + describe
cargo clippy -p semio-s-plugin-lowpoly --all-targets -D warnings && \
  nx run "@semio-tech/lowpoly-plugin:test" && \
  nx run "@semio-tech/lowpoly-plugin:describe"
```

### CI/CD Pipeline (Hypothetical)
```bash
# 1. Lint
cargo clippy -p semio-s-plugin-lowpoly --all-targets -D warnings

# 2. Test
nx run "@semio-tech/lowpoly-plugin:test"

# 3. Build WASM
cargo build -p semio-s-plugin-lowpoly --target wasm32-wasip2 --release

# 4. Verify descriptor
nx run "@semio-tech/lowpoly-plugin:describe"

# 5. TS fixture check
nx run "@semio-tech/lowpoly-js:test"
```

---

## Notes

- **Crate consolidation in progress:** Single crate (lowpoly) combines artifact (diff/op/dsl/pack/spr/engine) + app (commands/modes/windows/options/panels) via modular path decomposition in `📦️glue.rs`
- **CAD fixture feature:** Optional `cad-fixtures` gate is never enabled in normal builds to avoid duplicate symbol linker errors
- **TypeScript package:** Validates interactive job routes (47 total) match source & fixture; no runtime exports yet
- **Port allocation:** React UI @ 6078, wgpu renderer @ 6178 (defined in Cargo.toml metadata)


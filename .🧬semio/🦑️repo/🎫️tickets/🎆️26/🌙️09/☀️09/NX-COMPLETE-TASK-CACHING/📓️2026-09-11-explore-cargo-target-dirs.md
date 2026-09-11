# Cargo Build Directory Inventory (2026-09-11)

## Executive Summary
This repo has 75GB of build artifacts spread across multiple `target*` directories. Configuration occurs in 15+ file types, with 3 primary patterns: (1) `CARGO_TARGET_DIR` env var (most common, fully overridable), (2) hardcoded paths in launch configs and specific test scripts, and (3) `.cargo/config.toml` with sccache + rustc wrappers. Migration to a single shared `.🧬semio/🦑️repo/⚡️cache/cargo` cache is feasible but requires consolidating 50+ script.ts references and 100+ launch.json entries.

---

## On-Disk Target Directories

| Path | Size | Creator(s) |
|------|------|-----------|
| `./target` | 74 GB | Root workspace cargo build |
| `./target-gen3d-opt` | 1.9 GB | Procedural 3D opt builds |
| `./🧰️framework/📦️packages/🦀️rust/target` | 147 MB | Framework package tests |
| `./🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust/target` | 12 MB | Hash module tests |
| `./🧰️framework/🛍️products/💻️os/🧫️fixtures/⏳️asyncprobe/🖥️host/target` | (nested) | Async probe fixture builds |
| `./🧰️framework/🛍️products/💻️os/🧫️fixtures/⏳️asyncprobe/🖥️host/target/debug/.fingerprint/target-lexicon-*` | | Dependency artifacts |
| `./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/target` | | Plugin host builds |
| `./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🗑️generated/cargo-plugin-host` | | Generated plugin host |
| `./🧰️framework/🛍️products/💻️os/🪧testkit/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/target` | | JCO probe guest builds |
| `./🧰️framework/📦️packages/🦀️rust/target` | | Framework root package tests |
| `./🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/target` | | Schema module tests |
| `./🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust/target` | | Shell module target |
| `./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust/target` | | Test module builds |

---

## Configuration by File Type

### 1. `.cargo/config.toml`
**File:** `/Users/ueli/Documents/semio/.cargo/config.toml`

Settings found:
```toml
[build]
rustc-wrapper = "sccache"
rustflags = ["-Z", "threads=8"]

[unstable]
no-embed-metadata = true

[target.wasm32-unknown-unknown]
rustflags = ['--cfg', 'getrandom_backend="wasm_js"', '-C', 'link-arg=-zstack-size=16777216']

[target.wasm32-wasip2]
rustflags = ["-Z", "threads=8", "-C", "link-arg=--max-memory=536870912"]

[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=mold"]

[target.aarch64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

**Note:** No `target-dir` config here. Uses global sccache wrapper.

---

### 2. Environment Variable Configuration: CARGO_TARGET_DIR

**Primary Override Mechanism** — Most flexible, used across:

**Root Script (📜️script.ts) References:**
- Line 295-343: sccache setup (version 0.10.0, auto-install)
- Line 7434-7474: CARGO_TARGET_DIR + CARGO_INCREMENTAL + RUSTC_WRAPPER in ticket-scoped builds

**Framework Modules (📜️script.ts files):**

| Path | Key Lines | Pattern |
|------|-----------|---------|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts` | 338-362, 475, 552, 765, 827 | `cargoTargetRoot()`, resolves env var or defaults to `join(repoRoot, "target")` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` | 344-351, 4099-4159 | `cargoTargetRoot` logic; `PARITY_CARGO_TARGET_DIR` override for parity mode |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts` | 466, 516 | Hardcoded `CARGO_BUILD_JOBS: "1"` with empty `RUSTC_WRAPPER` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/📜️script.ts` | 92, 122 | Probes: `CARGO_TARGET_DIR ?? join(…, "🦀️oracle-probe", "target")` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/📜️script.ts` | 86, 94 | CAD oracle probe: `CARGO_TARGET_DIR ?? join(…, "target", "probe")` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🧪️tests/🕸️flow/🔬️unit/🦀️.rs` | (Rust test) | Likely hardcoded at compile time |
| `✏️s/🔌️plugins/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🏭️bridge/📜️script.ts` | 39, 43 | Bridge: `CARGO_TARGET_DIR ?? join(…, "target", "bridge")` |
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | 861, 1183, 5154-5168, 10233, 10823, 12950, 12974 | Hub: conditional logic on `CARGO_TARGET_DIR`, fallback to `join(repoRoot, "target")` |

**Common Pattern in script.ts:**
```typescript
const target = process.env.CARGO_TARGET_DIR ?? join(process.env.SEMIO_AGENT_CACHE ?? join(CRATE_DIR, "target"), "probe");
// or
const cargoTargetRoot = process.env.CARGO_TARGET_DIR ? resolve(repoRoot, process.env.CARGO_TARGET_DIR) : join(repoRoot, "target");
```

---

### 3. `.claude/launch.json`
**File:** `/Users/ueli/Documents/semio/.claude/launch.json`

**Hardcoded Entries:**
- Line 169: `CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-engines` (mit-bestand-demonstrator-fast)
- Line 209: `CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-engines` (mit-bestand-demonstrator-noengine)

**Env Var Overridable:** Yes, inline during launch.

---

### 4. `.vscode/launch.json`
**File:** `/Users/ueli/Documents/semio/.vscode/launch.json`

**Hardcoded CARGO_TARGET_DIR Entries (8+):**
- Lines 4383: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/native-openable-provider-sol-target`
- Lines 4476, 4669, 4855, 4870, 4900, 4923, 5125, 5139, 5164, 5187, 5210, 5265: Similar ticket-scoped paths

**RUSTC_WRAPPER Entries (60+):**
- Lines 4787, 4857, 4872, 4902, 4925, 5189, 5212, 5268, 5293, etc.: Empty string `""` to disable sccache

**CARGO_BUILD_JOBS Entries (50+):**
- Hardcoded to `"1"` across most test/build configurations

**Env Var Overridable:** No — hardcoded in JSON.

---

### 5. Caching Policy Configuration

**File:** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json`

**Cargo Toolchain Environment Variables Tracked:**
```json
"environment": [
  "RUSTUP_TOOLCHAIN",
  "RUSTFLAGS",
  "CARGO_ENCODED_RUSTFLAGS",
  "CARGO_BUILD_TARGET",
  "CARGO_PROFILE_DEV_OPT_LEVEL",
  "CARGO_PROFILE_RELEASE_OPT_LEVEL",
  "CARGO_PROFILE_DEV_DEBUG",
  "CARGO_PROFILE_RELEASE_DEBUG",
  "CARGO_TARGET_DIR",
  "CARGO_UNSTABLE_EMBED_METADATA",
  ...
]
```

**Generated Directories Excluded:**
```json
"generatedDirectories": [
  "target",
  "⚡️cache",
  "🗑️generated",
  ...
]
```

---

### 6. TypeScript Test/Library Files with --target-dir CLI Passing

**Files with explicit `--target-dir` cargo CLI args:**

| Path | Lines | Context |
|------|-------|---------|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts` | 52 | Binary gate resolution: `--target-dir`, `targetDirectory` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts` | 213, 345, 368, 401, 423, 446, 464 | Reference oracle context tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts` | 197, 213, 239 | Divergence callback tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts` | 168 | Artifact support tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts` | 1569, 1595, 2101 | Library utility functions |
| `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts` | 65-68 | Sequence generator: explicit `--target-dir` pass |
| `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts` | 37 | Equation generator |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/📜️script.ts` | 441-442 | FEM 3D generator |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/📜️script.ts` | 425-426 | FEM 2D generator |

---

### 7. Specific Cargo-Related Environment Variables

**Wrapper & Compiler Disabling:**
- `RUSTC_WRAPPER=""` — Used in 60+ vscode/launch.json entries and multiple script.ts files to disable sccache
- `SCCACHE_DISABLE="1"` — Set in devToolingEnv calls (plugin describe, procedural, hub)
- `RUSTC_WORKSPACE_WRAPPER=""` — Used in dev browser host staging tests

**Build Parallelism:**
- `CARGO_BUILD_JOBS="1"` — Set across 50+ test configurations in .vscode/launch.json
- `CARGO_BUILD_JOBS="4"` — Default fallback in dev server scripts (with env check first)

**Profiling & Optimization:**
- `CARGO_PROFILE_DEV_OPT_LEVEL` — Declared in caching policy
- `CARGO_PROFILE_RELEASE_OPT_LEVEL` — Declared in caching policy
- `CARGO_PROFILE_DEV_DEBUG` — Declared in caching policy
- `CARGO_PROFILE_RELEASE_DEBUG` — Declared in caching policy

**Incremental Compilation:**
- `CARGO_INCREMENTAL="0"` — Used in 7+ script.ts files (ticket cleanup mode, devToolingEnv)

---

### 8. Special Test Fixture References

**Cargo Target Discovery Skip Pattern:**
- File: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🎯️cargo-target-discovery-skip/🔣️.json`
- Directories explicitly listed to skip in discovery:
  ```json
  "target-demonstrator",
  "target-demonstrator-dev",
  "target-gen3d"
  ```

**Binary Gate Configuration:**
- File: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🎚️config/🧱️binary-gate.json`
- Unix: `"CARGO_TARGET_DIR": "scratch/cargo"`
- Windows: `"CARGO_TARGET_DIR": "scratch\\cargo"`

---

### 9. Agent Cache Root Mechanism

**Function:** `agentCacheRoot(repoRoot, agentId?)`
- **Defined in:** `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts:1186`
- **Resolves to:** `.🧬semio/🦑️repo/⚡️cache/agents/<id>/`
- **Usage:** Default fallback in probe/generator scripts when `CARGO_TARGET_DIR` not set
- **Default Agent ID:** `"local"` or `process.env.SEMIO_AGENT_ID`

**Test Module Usage:**
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts:498`
  ```typescript
  CARGO_TARGET_DIR: process.env.CARGO_TARGET_DIR ?? join(agentCacheRoot(repoRoot), "cargo-test-hosts")
  ```

---

### 10. Build Environment Function (wasmBuildEnvironment)

**File:** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:2905`

```typescript
function wasmBuildEnvironment(repoRoot: string, configuredEnv: Record<string, string>): Record<string, string> {
  return {
    ...env,
    CARGO_TARGET_DIR: resolve(repoRoot, env.CARGO_TARGET_DIR ?? ".🧬semio/🦑️repo/⚡️cache/cargo/browser")
  };
}
```

**Key Insight:** Already defaults to the shared cache location `.🧬semio/🦑️repo/⚡️cache/cargo/browser` when not overridden.

---

### 11. MCP Binary Resolution

**File:** `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts:27`

```typescript
function resolveMcpTargetDirectory(repoRoot: string): string {
  return pathApi(platform).resolve(
    repoRoot,
    env.CARGO_TARGET_DIR ?? "target"
  );
}
```

**Env Var Overridable:** Yes.

---

## Summary Table: Target Directory Configuration Sites

| Site | Current Dir | Resolved From | Env Overridable? | Proposed Change |
|------|-------------|---------------|------------------|-----------------|
| Root `.cargo/config.toml` | (none explicit) | Global sccache | No | Add rustc-wrapper="" option for shared cache mode |
| Root cargo build | `./target` | Default | Via `CARGO_TARGET_DIR` | Set to `.🧬semio/🦑️repo/⚡️cache/cargo/native` |
| `.claude/launch.json` | `target-engines` | Hardcoded path | Via env override at runtime | Update to use `$CARGO_TARGET_DIR` or absolute cache path |
| `.vscode/launch.json` (60+ entries) | Hardcoded ticket paths | JSON config | No (JSON-only) | Refactor to use env var substitution or programmatic generation |
| Framework module tests | Per-module `target/` | Default | Via `CARGO_TARGET_DIR` | Consolidate under shared cache with `cargo/framework/<module>` structure |
| Plugin probes/generators | Fallback via `agentCacheRoot()` | Script logic | Via `CARGO_TARGET_DIR` | Already uses cache-aware fallback; ensure consistency |
| wasm builds | `.🧬semio/🦑️repo/⚡️cache/cargo/browser` | Default in code | Via `CARGO_TARGET_DIR` | Already correct; confirm enforcement |
| Hub/Demonstrator builds | `target` or `target-demonstrator` | Script/env logic | Via `CARGO_TARGET_DIR` | Consolidate to `cache/cargo/hub` |
| Test module builds | `cargo-test-hosts` under agentCacheRoot | Function call | Via `CARGO_TARGET_DIR` | Route through shared cache |
| sccache configuration | Global `~/.cache/sccache` | Cargo config | Via env vars | Add `SCCACHE_DIR=.🧬semio/🦑️repo/⚡️cache/sccache` |

---

## Migration Checklist

### High Priority (Blocking shared cache adoption)
- [ ] Update root `📜️script.ts` to set `CARGO_TARGET_DIR` to shared cache for all subcommands
- [ ] Update `.cargo/config.toml` to reference `-Zfine-grain-locking` for parallel access
- [ ] Refactor `.vscode/launch.json` to use `${CARGO_TARGET_DIR}` variable substitution (or regenerate from script)
- [ ] Update `.claude/launch.json` entries to use `$CARGO_TARGET_DIR` env var instead of hardcoded paths

### Medium Priority (Cache hygiene)
- [ ] Add `SCCACHE_DIR` env var to launch configs
- [ ] Consolidate `CARGO_INCREMENTAL` disabling into caching policy, not per-script
- [ ] Document `RUSTC_WRAPPER=""` override pattern in AGENTS.md

### Low Priority (Cleanup)
- [ ] Remove now-unused `target-demonstrator`, `target-gen3d-opt` directories after verification
- [ ] Update discovery patterns in `binary-gate.json` and fixture skip lists
- [ ] Verify all framework module `target/` dirs route through shared cache


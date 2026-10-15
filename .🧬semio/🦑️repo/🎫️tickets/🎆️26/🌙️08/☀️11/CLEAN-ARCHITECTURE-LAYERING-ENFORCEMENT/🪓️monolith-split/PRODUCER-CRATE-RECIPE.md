# Producer Crate Recipe

Reverse-engineered from the live repository and the peer report `MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/draw-prepared-crate-cuts.md`. Worked examples: `semio-framework-ui-surface-identity` (zero dependencies) and `semio-framework-ui-prepared` (downward dependencies, `testkit` feature). Sibling examples: `semio-framework-ui-draw-contract`, `semio-framework-ui-prepared-staging`.

Evidence labels: **[observed]** read in a file or launch entry. **[code]** read in a repo script. **[ticket-local]** exists only in the peer ticket, not in permanent repo files. **[inferred]** derived, not executed. Nothing in this recipe was built or executed by the auditor.

## 0. Placeholders

| Placeholder | Meaning | Example (surface identity) |
| --- | --- | --- |
| `<MODULE>` | Framework module root that owns the carved code | `🧰️framework/🔨️modules/🖱️ui` |
| `<NODE>` | Taxonomy node that becomes the producer (under `<MODULE>`) | `🪟️surface/🪪️identity` |
| `<PKG>` | Package dir `<MODULE>/<NODE>/📦️packages/🦀️rust` | |
| `<ID>` | `[package.metadata.semio] id`, lowercase kebab, prefixed by domain (`ui-…`) | `ui-surface-identity` |
| `<CRATE>` | Cargo package name = `semio-framework-<ID>` | `semio-framework-ui-surface-identity` |
| `<PROJECT>` | Nx project name = `@semio-tech/<ID>-rs` | `@semio-tech/ui-surface-identity-rs` |
| `<UP>` | `..` repeated once per path segment from `<PKG>` up to `🧰️framework` | 6 for surface identity |
| `<PARENT>` | The crate the code was carved from | `semio-framework-ui` |

Derivation rules **[observed]**:
- `<CRATE>` = `semio-framework-` + `<ID>` holds for all four carved producers and for `semio-framework-ui` (`id = "ui"`).
- `<PROJECT>` = `@semio-tech/` + `<ID>` + `-rs` holds for the four carved producers and for the `ui-*` owners in `📋️build-owners.json`. Outside the UI cluster the repo does not follow one rule (e.g. `@semio-tech/framework-tool-run-rs` for `id = "tool-run"`; `semio-framework-geometry` has no `@semio-tech` scope). Do not assume; use the name the `project.json` declares, and check it is unique in `📋️build-owners.json`.
- `<UP>` is the number of segments between `🧰️framework/` and `<PKG>`. Observed: `🪪️identity` package → 6, `🎟️prepared` package → 7, `🎟️prepared/📤️staging` package → 8, `🖍️draw/🏷️types` package → 8.

## 1. Admission budget and what the numbers mean

- Hard ceiling for every Rust producer: **4000 active lines, 32 source files, 262144 full bytes**. Per-package metadata cannot relax it **[observed: crate-production-boundaries.md]**.
- The manifest must declare exactly these three keys (`maxActiveLines`, `maxSourceBytes`, `maxSourceFiles`, no others), with the ceiling values **[code: 📦️units/📥️descriptor/🟦️.ts line 17 validates key set exactly; 📜️script.ts line 28 enforces the byte and line limits]**.
- Measure without compiling: the descriptor path calls `cargo metadata --offline --locked --no-deps --format-version=1 --manifest-path <manifest>` **[code]**, then the canonical source helper `readRustSourceClosureV1([libRootEntry], root, cfgSnapshot, reader)` from `🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts` **[code]**. The helper walks mounted modules and sums `activeLines` and bytes per cfg snapshot. The reader callback must return file contents and refuse symlinks. The call shape is observed, not executed here. Record the measured numbers in the ticket markdown, never as chat output.
- The peer report's numbers are *authored cfg-selected* counts, not compiler dep-info or proc-macro expansion **[observed]**. Treat them as admission evidence only.

## 2. Canonical file layout

For a producer at `<MODULE>/<NODE>`:

```
<MODULE>/<NODE>/🦀️.rs                      library root (lib target entry)
<MODULE>/<NODE>/🧪️tests/🦀️.rs              native unit tests, mounted by the lib root under cfg(test)
<MODULE>/<NODE>/🧪️tests/🧫️fixtures/*.csv  language-neutral vectors (optional)
<MODULE>/<NODE>/📦️packages/🦀️rust/Cargo.toml
<MODULE>/<NODE>/📦️packages/🦀️rust/📋️project.json
```

Observed for surface identity and staging: the **library root lives beside the package dir, not inside it**. `[lib] path = "../../🦀️.rs"` resolves from `<PKG>` to `<MODULE>/<NODE>/🦀️.rs`. The package dir holds only `Cargo.toml` and `📋️project.json` **[observed]**. Source files of a producer are the taxonomy node's own files; they are not copied (AGENTS.md: no copies of checked-in sources into ticket folders; path limit 256 characters). Measured example: the staging `Cargo.toml` path is 138 characters from repo root.

## 3. Step-by-step checklist

### Step 1. Choose the carve and the taxonomy node

- [ ] Confirm the carved code is a self-contained node that the parent only consumes through `pub use` or `use`.
- [ ] Confirm the directory emoji names in `<NODE>` are registered in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`. **[observed]** The taxonomy contains `🪪️identity` and `🪟️surface` already; a brand-new emoji directory needs a taxonomy entry first **[observed: taxonomy validation in 📚️library/🔍️discovery/🟦️.ts]**.
- [ ] Ensure the lib root and package dir follow the layout in section 2.

### Step 2. Create the manifest

Path: `<PKG>/Cargo.toml`. Copy and fill:

```toml
[package]
workspace = "<UP>"
name = "semio-framework-<ID>"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true

[package.metadata.semio]
role = "framework"
id = "<ID>"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[features]
testkit = []

[dependencies]

[dev-dependencies]
```

Notes:
- `workspace` points to the directory that holds `🧰️framework/Cargo.toml` (the framework workspace), not to repo root. Surface identity: `"../../../../../.."`. Prepared: `"../../../../../../.."`. **[observed]**
- `role = "framework"`: 220 framework manifests use it **[observed]**. Other roles exist (`library`, `product`, `tool`, `test`, `artifact`); do not change the role without a decision.
- `testkit = []` only if a consumer needs it; delete the feature otherwise. Prepared and staging both have it **[observed]**.
- `[dependencies]` must list only lower-layer first-party crates (section 3 step 4). Dependency edges must form a DAG; see section 5.
- Target-specific dependencies copy the parent's pattern, e.g. `[target.'cfg(all(target_arch = "wasm32", not(target_env = "p2")))'.dependencies]` **[observed in prepared]**.

Dependency path template (from `<PKG>` to a sibling producer's package dir):

```toml
<dep-ident> = { path = "<relative path from <PKG> to <OTHER_PKG>>" }
```

Relative path is computed from `<PKG>`. Example from prepared to draw contract: `"../../../🖍️draw/🏷️types/📦️packages/🦀️rust"`. Optional dependencies add `optional = true`; the parent turns them on through its own feature list.

### Step 3. Create the library root and test mount

File: `<MODULE>/<NODE>/🦀️.rs`. Copy and fill:

```rust
//! <EMOJI> <one-line purpose, docstring starts with a unique fitting emoji>
pub const ...;
pub struct ...;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
```

Observed surface identity root (abridged): a `//!` docstring, `pub const UI_LAYOUT_SURFACE_SLOTS: usize = 64;`, a `pub struct UiSurfaceToken`, and the `#[cfg(test)] #[path = "🧪️tests/🦀️.rs"] mod tests;` mount **[observed]**. Do not put comments inside definitions (AGENTS.md).

Tests file: `<MODULE>/<NODE>/🧪️tests/🦀️.rs`. Language-neutral CSV vectors go beside it under `🧪️tests/🧫️fixtures/`. Surface identity's vectors are `leftSlot,leftGeneration,rightSlot,rightGeneration,equal` **[observed]**.

### Step 4. Move the code and rewire the parent

For each moved item:
1. Move the source text into the producer's lib root or submodule. Do not leave a copy in the parent.
2. In the parent crate's `🦀️.rs` or consumer modules, replace the old `use crate::…` path with the producer path:

   - Consumer that only needs the type: `use semio_framework_ui_surface_identity::UiSurfaceToken;` **[observed in mounted_layout/🦀️.rs:6]**.
   - API the old parent exported to its own clients: an explicit re-export, not a compat layer. Observed: `pub use semio_framework_ui_surface_identity::{UiSurfaceToken, UI_LAYOUT_SURFACE_SLOTS};` in `⚙️engine/🦀️.rs:957` **[observed]**. AGENTS.md requires explicit re-exports when a client needs the item.

3. Crate names with hyphens become underscores in Rust (`semio-framework-ui-surface-identity` → `semio_framework_ui_surface_identity`).

### Step 5. Register the dependency in the parent manifest

File: parent `Cargo.toml` (surface identity: `🖱️ui/📦️packages/🦀️rust/Cargo.toml`).

Under `[dependencies]`:

```toml
<CRATE> = { path = "<relative path from parent PKG to <PKG>>", optional = true }
```

Dependency key = package name, no rename, as the parent does for `semio-framework-ui-surface-identity` **[observed]**. Under the parent feature that needs it, add `"dep:<CRATE>"`, e.g. `wgpu-engine` contains `"dep:semio-framework-ui-surface-identity"` **[observed]**. If the producer is needed by tests, add it under `[dev-dependencies]` with `features = ["testkit"]` **[observed pattern]**.

Feature forwarding: the peer added `browser-presenter` to the prepared crate and forwarded it from `wgpu-engine` as `"semio-framework-ui-prepared/browser-presenter"` **[observed]**. Forward any new feature the same way; do not reintroduce a dependency edge that bypasses the producer.

### Step 6. Register the workspace member

File: `🧰️framework/Cargo.toml`, `[workspace] members`. Insert `"<NODE-PATH-FROM-FRAMEWORK>/📦️packages/🦀️rust"` in byte-sorted order among its siblings. Observed: lines 78 to 95 show the UI members sorted by path (`🎟️prepared/📤️staging` before `🎟️prepared`) **[observed]**.

- The root `Cargo.toml` does not list UI producers as members. It reaches them through `[workspace.dependencies]` (e.g. `semio-framework-ui`, line 459). Add a root entry only if a root consumer needs the producer directly **[observed]**.
- `[workspace.metadata.semio.repository] member-manifests` is a glob (`🧰️framework/**/📦️packages/🦀️rust/Cargo.toml` in root; `📦️packages/🦀️rust/Cargo.toml` and `🔨️modules/**/📦️packages/🦀️rust/Cargo.toml` in framework). A new manifest is picked up by the glob **[observed]**.

### Step 7. Update both lock files

- `🧰️framework/Cargo.lock` and `Cargo.lock` at repo root both contain `name = "semio-framework-ui-surface-identity"` **[observed]**. Each lock records the new package and the parent's `dependencies` list.
- Regenerate by resolving offline, without `--locked`: `cargo metadata --offline --format-version=1 --manifest-path 🧰️framework/Cargo.toml` and the same for root `Cargo.toml` **[inferred from the report's "offline Cargo metadata exited0 after manifests/locks resolved"]**. Do not hand-edit lock entries.
- Do not run any build or `cargo test` to refresh locks. The Cargo ingress rule applies: raw cargo is refused since 2026-10-14; use `bun repo cargo ...` (memory: Cargo Compiler Ingress Gate).

### Step 8. Nx project file

File: `<PKG>/📋️project.json`. Data only, no targets. Copy:

```json
{
  "name": "@semio-tech/<ID>-rs",
  "tags": [
    "scope:framework",
    "language:rust"
  ]
}
```

- Observed exactly for surface identity, prepared, staging and draw contract **[observed]**.
- Targets are inferred, not declared. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs` `cargoTargets()` adds `build`, `check` and `test` for every package that has a `Cargo.toml` with `[package]`. Each target has `prototype` and `release` configurations **[code]**. The command is `bun <caching>/🦀️cargo/📜️script.ts native cargo <target> --manifest <manifest>`, `build` is cached with outputs under `dist/build/{mode}`, and `check` and `test` have `parallelism: false` **[code]**.
- `nx.json` loads `📋️project.json` through the repo plugin `📚️library/🟨️.mjs` **[observed]**.
- The AGENTS.md rule "project.json MUST only call `📜️script.ts`" is satisfied trivially: the Rust producer `project.json` declares no command.

### Step 9. Owner registry row

File: `📋️build-owners.json` (repo root). Insert into the `owners` array in lexical order by `name`. Observed row for surface identity (line 4193):

```json
{
  "name": "@semio-tech/<ID>-rs",
  "root": "<MODULE>/<NODE>/📦️packages/🦀️rust",
  "manifest": "<MODULE>/<NODE>/📦️packages/🦀️rust/📋️project.json",
  "packages": []
}
```

- `root` is the package dir (not the module root) **[observed]**.
- `packages` is `[]` for Rust producers **[observed]**.
- The graph code requires the registry name to equal the `project.json` name and the requested target to exist; otherwise it throws `Unknown admitted build target` or `Catalog build producer is absent from its owner's project` **[code: 🟨️.mjs ~1588, ~1633]**.
- Registry sanity count: 28 rows named `@semio-tech/ui-*` (surface identity at line 4193, draw contract at 4061, prepared at 4099, staging at 4105) **[observed]**.

### Step 10. Six launch routes in `.vscode/launch.json`

File: `.vscode/launch.json` (not the seed; the seed contains no producer routes **[observed]**). AGENTS.md: every executable command is registered in `launch.json`.

For `<ID>` with package name `<CRATE>`, add six objects. The group/order fields are per route:

| Route | `presentation.group` | `presentation.order` band | Name suffix |
| --- | --- | --- | --- |
| build prototype | `4_build` | 900.0xxxx | `🏗️build 🛠️prototype` |
| build release | `4_build` | 900.0xxxx | `🏗️build 🚀️release` |
| check prototype | `4_gate` | 922.04xxx | `🔎️check 🛠️prototype` |
| check release | `4_gate` | 922.04xxx | `🔎️check 🚀️release` |
| test prototype | `4_test` | 903.58xxx | `🧪️test 🛠️prototype` |
| test release | `4_test` | 903.58xxx | `🧪️test 🚀️release` |

Template (copy six times, changing name, configuration, target and order):

```json
{
  "name": "🦀️ <CRATE> 🏗️build 🛠️prototype",
  "type": "node-terminal",
  "request": "launch",
  "command": "bun repo run @semio-tech/<ID>-rs:build --configuration=prototype --parallel=1 -- --lib --offline",
  "cwd": "${workspaceFolder}",
  "presentation": {
    "group": "4_build",
    "order": 900.01042
  }
}
```

Rules observed across the four carved producers **[observed]**:
- Key order: `name`, `type`, `request`, `command`, `cwd`, `presentation`.
- `command` target is `<target>` in `build|check|test`, configuration `prototype|release`, `--parallel=1`, and `-- --lib --offline` (library only, offline).
- `order` values are decimal literals in a shared band. Observed: surface identity build 900.01032 / 900.01033, check 922.04132 / 922.04133, test 903.58232 / 903.58233; draw 900.01001 / 900.0100199999999 (float artefact in the file), check 922.04101 / 922.04102, test 903.58201 / 903.5820199999999; staging 900.01011 / 900.0101199999999, check 922.04111 / 922.04112, test 903.58211 / 903.5821199999999; prepared 900.01021 / 900.01022, check 922.0412100000001 / 922.0412200000001, test 903.58221 / 903.58222.
- Choose an unused key in the band; do not renumber existing rows. Grep before writing: `/usr/bin/grep -n '"order": 900.0104' .vscode/launch.json`.
- Place the six objects next to the existing `🦀️ semio-framework-ui-*` route block of the sibling crate (surface identity sits near line 2577, draw contract 2643, staging 2709, prepared 2789).

Dashboard generator note **[code: 🎛️dashboard/🌳️command-tree/🚀️launch/🟦️.ts]**: `launch generate` only rewrites entries whose `presentation.group` starts with `semio.dashboard.`, and keeps all other rows in order. Producer rows survive a dashboard regeneration. The producer routes themselves were written by hand or by a ticket-local tool; no permanent generator for them was found **[observed]**.

### Step 11. Boundary corpus row (permanent, enforced by a gate)

The corpus is `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json` **[observed]**. It has 29 rows, which is the report's "29 UI/asset corpus rows". Its schema is `🖱️ui/🧬️schema/📦️crate-boundaries/🔣️.json` **[observed]**. Each row:

```json
{
  "package": "semio-framework-<ID>",
  "manifest": "<path relative to 🖱️ui, to the Cargo.toml>",
  "entry": "<path relative to 🖱️ui, to the lib root 🦀️.rs>",
  "dependencies": ["<every declared dependency package name, including target-specific ones>"],
  "forbiddenDependencies": ["semio-framework-ui", "semio-framework-os-kernel", "semio-framework-plugin", "wgpu", "winit", "taffy"]
}
```

- `manifest` and `entry` are relative to the UI module root, not to repo root. Surface identity: `🪟️surface/🪪️identity/📦️packages/🦀️rust/Cargo.toml` and `🪟️surface/🪪️identity/🦀️.rs` **[observed]**.
- `dependencies` must equal the manifest's declared package names, sorted, or the test fails **[observed in test]**. Target-specific dependency tables count.
- The test `🖱️ui/🧪️tests/📦️crate-boundaries/🟦️.ts` runs for every row and asserts:
  - the Cargo manifest parses identically under Bun's TOML and `@iarna/toml`, and `package.name` equals `package`;
  - each of the six `launch.json` commands exists exactly (step 10);
  - `[lib] path` resolves to `entry`;
  - declared dependencies equal `dependencies` and none of `forbiddenDependencies` appears;
  - the source closure under `aarch64/macos/unix` stays within **4000 active lines, 262144 bytes, 32 files**, with no `.svg` input;
  - no `.rs` input contains the tokens `semio_framework_ui`, `semio_framework_os_kernel`, `semio_framework_plugin` or `wgpu` **[observed]**.
- For the four carved producers (draw contract, prepared, prepared staging, surface identity), the test repeats the budget and token checks across seven cfgs: macOS, Linux gnu, Windows msvc, wasm32 unknown, wasm32 with `browser-presenter`, wasm32 WASI p2, and native test with `testkit` **[observed]**.
- Surface identity's neutral vectors are checked against SQLite `equals` **[observed]**.
- The producer closure is checked for cycles twice: a DFS over the authored first-party edges, and a graphlib SCC run **[observed]**.
- The enclosing renderer closure must not contain any producer entry or `🎟️prepared/📤️staging/` source, and must consume producers only through `path` edges **[observed]**.

Adding a producer therefore requires adding its row here, or the gate will not cover it. Rows for the 26 remaining entries share the same fields.

Test route: launch `🧪️test🖱️ui📦️crate-boundaries`, which runs `bun repo run @semio-tech/ui-rs:test-crate-boundaries --skip-cache --exclude-dependencies` **[observed in launch.json line 59937]**. Target `test-crate-boundaries` is defined in `🖱️ui/📦️packages/🦀️rust/📋️project.json` and runs `bun ./📜️script.ts test crate-boundaries` **[observed]**.

### Step 12. Parent consumer checks

- Parent `🦀️.rs` and consumers compile against the new paths. Confirm the parent's dependency line, feature lists and `use` lines are consistent (step 4 and 5).
- Parent dev-tests that used the moved helpers import them through the producer's `testkit` feature. Observed: parent dev-deps enable `testkit` on prepared, draw contract, raster owner and retained tree **[observed]**.

## 4. Verification routes

Run through launch.json entries (AGENTS.md: devs do not use the CLI). The CLI equivalents are listed for traceability.

| Purpose | Launch name | Equivalent command |
| --- | --- | --- |
| Build library, prototype | `🦀️ <CRATE> 🏗️build 🛠️prototype` | `bun repo run @semio-tech/<ID>-rs:build --configuration=prototype --parallel=1 -- --lib --offline` |
| Build library, release | `🦀️ <CRATE> 🏗️build 🚀️release` | same with `--configuration=release` |
| Check | `🦀️ <CRATE> 🔎️check 🛠️prototype` / `🚀️release` | `bun repo run @semio-tech/<ID>-rs:check --configuration=… --parallel=1 -- --lib --offline` |
| Native tests | `🦀️ <CRATE> 🧪️test 🛠️prototype` / `🚀️release` | `bun repo run @semio-tech/<ID>-rs:test --configuration=… --parallel=1 -- --lib --offline` |
| Boundary corpus gate | `🧪️test🖱️ui📦️crate-boundaries` | `bun repo run @semio-tech/ui-rs:test-crate-boundaries --skip-cache --exclude-dependencies` |
| Runtime witness (genuine Nx route) | `⚖️native-storage-specimens prototype` / `release` | runs the ticket headroom gate; the peer's only runtime proof for surface identity |

- The `--lib` flag restricts test runs to the library's own `#[cfg(test)]` modules **[inferred from the flag]**. Integration tests declared with `[[test]]` elsewhere are not part of these routes **[inferred]**.
- Source admission without compiling: `cargo metadata --offline --locked --no-deps` and the source helper from section 1 **[code]**.
- Runtime evidence: the peer ran `native-storage-specimens` in prototype and release under a fixed 32 MiB backing. It compiled and ran surface identity's native test with six vectors and recorded source, test and CSV SHA256 digests **[observed in report]**. The report's digests:
  - Source SHA256: `bd95edfb2b651e0b224d483ca5dd5db8246331a186c0581e7cb21e2466b8ba73`
  - Native law: `6e192308e40fc12136fbb680565da1552ce27eac67f13c21ff2b093a2b7819e8`
  - Neutral CSV: `b0fb44f38b6fee75b1db913c2da44c75d11cdf484dd46ffc7816995fcd022cae`
- AGENTS.md requires `[DEBUG] ` prefixes on temporary logs and console evidence before saying runtime works.

## 5. Gates the peer ran (pitfalls included)

Permanent repo gates **[code and observed]**:
- Rust source boundary check in the descriptor (`📥️descriptor/🟦️.ts`, `📜️script.ts`): exact key set, byte and line ceilings.
- Taxonomy and owner registry validation.
- The UI boundary test `test-crate-boundaries` (step 11): corpus schema validation, TOML agreement, launch route presence, dependency lists, forbidden dependencies, per-cfg budgets, forbidden-token scan of every selected `.rs` input, SQLite surface-identity vectors, DFS and graphlib SCC cycle checks, and enclosing-renderer duplication checks.

Ticket-local only (not in the repo): the peer's aggregate run "37 PASS / 0 FAIL / 1390 assertions" and the native-storage witness (`native-storage-specimens`, headroom gate). Report those as ticket evidence, not as repo features.

Pitfalls the peer recorded:
1. **Initial native descriptor only accepts dependency-free libraries** (dev-deps allowed) **[code: descriptor 🟦️.ts]**. Crates with normal dependencies are not admitted by that path. Draw contract, prepared and staging have authored Source, TOML and metadata evidence, but their real bytemuck, scene, Job and Value compiler frontier is **not qualified** **[observed report]**.
2. **Cfg snapshots must be complete.** Admission is authored cfg-selected; compiler dep-info is not proof. Cover every target the producer is selected for.
3. **Native test selection is its own admission.** Do not infer the test closure from the production closure **[observed report]**.
4. **Prohibited facade/backend tokens** are scanned in every selected input (ticket-local).
5. **Cycle detection** must include target-specific and workspace-inherited edges (ticket-local).
6. **bytemuck:** keep the parent's exact feature set (`derive`, `extern_crate_alloc`, `min_const_generics`) so all components compile one bytemuck unit **[observed comment in parent Cargo.toml]**. Draw contract declares the same features directly.
7. **Feature forwarding:** `browser-presenter` must be forwarded by the parent's `wgpu-engine` feature; the browser `js-sys` dependency stays optional and target-bound **[observed]**.
8. **Do not refresh expected law hashes** to make gates pass. Five prepared-engine laws and native grant semantics still need deliberate port review **[observed report]**.
9. **Private fields stay private.** Prepared packet fields remain private; GPU reads borrowed draw, overlay and time accessors **[observed report]**.
10. **Old zero-argument producer and rejected close_step law calls** remain untouched; close functions need `RetainedCloneGrant` **[observed report]**.
11. **Path length:** keep every file path at or below 256 characters (AGENTS.md).
12. **No migration or compat:** no deprecated re-exports, no shims; fix the consumers in the same change.

## 6. Touched-artifact inventory for the surface identity example

New:
- `🧰️framework/🔨️modules/🖱️ui/🪟️surface/🪪️identity/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🪟️surface/🪪️identity/🧪️tests/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🪟️surface/🪪️identity/🧪️tests/🧫️fixtures/📐️identity.csv`
- `🧰️framework/🔨️modules/🖱️ui/🪟️surface/🪪️identity/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖱️ui/🪟️surface/🪪️identity/📦️packages/🦀️rust/📋️project.json`

Edited:
- `🧰️framework/Cargo.toml` members (surface identity at line 95)
- `🧰️framework/Cargo.lock` and `Cargo.lock` (new package entry and parent edge)
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml` (dependency line and `wgpu-engine` feature entry)
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs` line 957 (explicit `pub use`)
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs` line 6 (`use`)
- `📋️build-owners.json` (owner row at line 4193)
- `.vscode/launch.json` (six route objects around lines 2577 to 2653)

Not touched by the peer for this crate: `.vscode/🧩️launch.seed.jsonc`, `nx.json`, root `Cargo.toml` members.

## 7. Checklist summary

- [ ] Taxonomy emoji names registered (`🔣️taxonomy.json`)
- [ ] `🦀️.rs` lib root beside package dir, `#[cfg(test)]` mount of `🧪️tests/🦀️.rs`
- [ ] `Cargo.toml`: `workspace` depth, `name`, `role`, `id`, exact three boundary keys, `[lints] workspace = true`, `[lib] path = "../../🦀️.rs"`
- [ ] Dependencies downward only, acyclic; optional where the parent gates them
- [ ] Code moved, parent rewired, explicit `pub use` only for real clients
- [ ] Parent `Cargo.toml`: dependency line, feature `dep:` entries, dev-deps with `testkit`
- [ ] `🧰️framework/Cargo.toml` member inserted in byte order
- [ ] `Cargo.lock` (framework and root) regenerated offline, not hand-edited
- [ ] `📋️project.json` with `@semio-tech/<ID>-rs`, `scope:framework`, `language:rust`
- [ ] `📋️build-owners.json` row in lexical order, `root` = package dir
- [ ] Six launch routes with correct groups, names, configuration and unused order keys
- [ ] Boundary corpus row added to `🖱️ui/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json` with exact dependency list and forbidden list
- [ ] `test-crate-boundaries` passes (launch `🧪️test🖱️ui📦️crate-boundaries`)
- [ ] Source admission measured within 4000 / 32 / 262144 for production and native test selections
- [ ] Runtime witness through a genuine Nx route, output recorded in the ticket markdown
- [ ] No copy of source into ticket folders; every path at most 256 characters

## 8. Gaps this recipe does not close

- No permanent generator for the six producer launch routes was found; they are hand-maintained. The boundary test only checks that they exist.
- The exact `cargo metadata` lock refresh form is inferred from the report, not reproduced here.
- The boundary test's cfg matrix and fixture are hand-authored. The test was read, not run, in this audit.

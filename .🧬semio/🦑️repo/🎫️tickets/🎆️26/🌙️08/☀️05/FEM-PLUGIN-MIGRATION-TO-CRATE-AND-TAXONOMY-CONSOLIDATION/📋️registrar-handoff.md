# 📋️ Registrar Handoff — 🏗️fem Plugin Crate Consolidation (FINAL)

Status on disk (2026-08-06): **Shape V2 complete**, temporary verification overlay removed, all 17 old
`⚡️implementations` trees deleted, only `📦️packages/🦀️rust/📦️lib.rs` remains as the lib entry
(`[lib] path = "📦️lib.rs"`, leaf `#[path]` values use `../../`, grouping resets stay `#[path = "."]`).

New crate: **`semio-s-plugin-fem`** at `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust`
(`crate-type = ["cdylib", "rlib"]`, `role = "plugin"`, playgrounds `fem2d` → `fem2d-play`, `fem3d` → `fem3d-play`).

Owner root now contains only: `AGENTS.md`, `README.md`, `📦️packages/`, `🗿️artifacts/`, `🎛️apps/`, `🫀️core/`.

---

## 1. Root `Cargo.toml` — already applied (do not re-edit unless drift returns)

### 1.1 Member present

```toml
    "✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust",
```

### 1.2 Old members — already gone

The former 17 `✏️s/🔌️plugins/🏗️fem/**/⚡️implementations/🦀️rust` member lines are no longer in root
`Cargo.toml` (and the directories are deleted from disk).

### 1.3 `[workspace.dependencies]` — already gone

These aliases are no longer present:

```toml
semio-s-app-fem-2d = { path = "…" }
semio-s-app-fem-3d = { path = "…" }
semio-s-plugin-fem-core = { path = "…" }
```

No new `workspace.dependencies` alias for `semio-s-plugin-fem` is required (dependents use explicit
`package = "semio-s-plugin-fem"` path deps).

---

## 2. Cross-cutting dependents — already applied

### 2.1 `semio-s-plugin-norm`

`✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/Cargo.toml` already has:

```toml
fem = { path = "../../../🏗️fem/📦️packages/🦀️rust", package = "semio-s-plugin-fem" }
```

Engine components already import:

```rust
use fem::core::elements2d::BeamEb2;
use fem::core::{Dof, MemberUdl, Model, Node, Support};
```

(Only docstring mentions of `fem_core` remain — cosmetic.)

### 2.2 `semio-framework-os-kernel-dsl` fixture-sweep

Cargo.toml already has:

```toml
fem = { path = "../../../../../../../../✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust", package = "semio-s-plugin-fem" }
```

`📦️lib.rs` already uses:

```rust
use fem::artifacts::fem2d::Fem2dDocument;
use fem::artifacts::fem3d::Fem3dDocument;
```

Envelope ids are `fem.fem2d` / `fem.fem3d` (see §4).

---

## 3. Module path mapping (consumers)

| Old crate | New path |
| --- | --- |
| `semio-s-plugin-fem-core` | `semio_s_plugin_fem::core::…` (`BeamEb2` → `::core::elements2d::BeamEb2`) |
| `semio-s-plugin-fem-shared` | `semio_s_plugin_fem::core::shared::…` |
| `semio-s-app-fem-2d` (+ op/dsl/pack/protocol/engine) | `…::artifacts::fem2d::…` (`protocol` → `spr`) |
| `semio-s-app-fem-2d-ui` | `…::apps::fem2d::…` |
| `semio-s-app-fem-3d*` | `…::artifacts::fem3d::…` / `…::apps::fem3d::…` |
| `semio-s-plugin-fem-artifact-manifest` | `semio_s_plugin_fem::core::register_all_engines()` |

---

## 4. Behaviour note (not a registrar edit)

Documents use `#[dsl(id = "fem.fem2d")]` / `#[dsl(id = "fem.fem3d")]`. Fixtures ship as
`semio fem.fem2d.dsl v1` / `semio fem.fem3d.dsl v1`.

---

## 5. Verification (this ticket)

With `DEVELOPER_DIR=/Library/Developer/CommandLineTools`:

| Check | Result |
| --- | --- |
| `cargo test -p semio-s-plugin-fem --lib` | **318 passed; 10 failed** (evidence: ticket `*test-final2.txt`) |
| `cargo check -p semio-s-plugin-fem --target wasm32-wasip2` | **Finished ok** (evidence: ticket `*check-wasm.txt`) |
| Shape V2 | **done** — single lib in packages; no root `📦️lib.rs`; no overlay; no nested `target/`/`Cargo.lock` |
| Old `⚡️implementations` | **0 remaining** (17 deleted) |

### About the 10 failures

All 10 are DSL/pack/spr round-trips failing inside `store::test_support::assert_dsl_round_trip` with
`dsl parse failed: expected List, found Absent at 1:1`. Printed preamble is
`semio fem.fem2d dsl v1` (space before format) while fixtures/parser expect
`semio fem.fem2d.dsl v1` (dotted). The same failure reproduces on `semio-s-plugin-block` dsl round-trips
— **framework-wide printer/parser asymmetry, not a fem consolidation defect**. Old per-crate baseline
summed to ~222 passing tests; consolidated crate has **318 passing** lib tests plus these 10 blocked
on the shared DSL regression.

One fem-local assertion was corrected: `import_media_geometry_in_adds_a_new_solid_3d` now expects
material id `m0` (`next_id(..., "m")`), not the material *name* `concrete`.

---

## 6. Registrar action remaining

**None for fem membership / dependents** — root member, workspace.deps cleanup, norm + fixture-sweep
repoints are already on disk. Track the shared DSL preamble round-trip regression separately (affects
fem + block + likely peers).

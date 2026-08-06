# 📋️ Registrar Handoff — 🏗️fem Plugin Crate Consolidation

The 🏗️fem plugin is now a single Shape V2 crate. Everything inside `✏️s/🔌️plugins/🏗️fem/**` is done.
The edits below are OUTSIDE that ownership boundary (root `Cargo.toml`, two dependent crates) and are
left to the registrar — do not expect them to be applied by this ticket.

New crate: **`semio-s-plugin-fem`** at `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust`
(`[lib] name` defaults to `semio_s_plugin_fem`, `crate-type = ["cdylib", "rlib"]`,
`role = "plugin"`, playgrounds `fem2d` → `fem2d-play` and `fem3d` → `fem3d-play`).

---

## 1. Root `Cargo.toml` — `[workspace] members`

### 1.1 Add (1 line)

```toml
    "✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust",
```

### 1.2 Remove (17 lines — currently lines 174–190, all now deleted from disk)

```toml
    "✏️s/🔌️plugins/🏗️fem/🔨️modules/🫀️core/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏗️fem/🔨️modules/🤝️shared/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/🔨️modules/⚙️engine/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/🔨️modules/🗣️dsl/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/🔨️modules/🔧️op/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/🔨️modules/🎒️pack/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/🔨️modules/📡️protocol/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/🔨️modules/🖱️ui/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/🔨️modules/⚙️engine/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/🔨️modules/🗣️dsl/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/🔨️modules/🔧️op/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/🔨️modules/🎒️pack/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/🔨️modules/📡️protocol/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/🔨️modules/🖱️ui/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🏗️fem/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust",
```

## 2. Root `Cargo.toml` — `[workspace.dependencies]`

### 2.1 Remove (3 lines — currently 317, 318, 334)

```toml
semio-s-app-fem-2d = { path = "✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/⚡️implementations/🦀️rust" }  # 8 refs
semio-s-app-fem-3d = { path = "✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/⚡️implementations/🦀️rust" }  # 8 refs
semio-s-plugin-fem-core = { path = "✏️s/🔌️plugins/🏗️fem/🔨️modules/🫀️core/⚡️implementations/🦀️rust" }  # 9 refs
```

### 2.2 Add — nothing

Both remaining dependents alias the crate with an explicit `package = …` (§3), which per TEMPLATE §13.4
stays a plain `path` dependency rather than a `workspace.dependencies` alias. No new alias is needed.

The other 14 removed crates never had a `workspace.dependencies` alias (they were referenced by
sibling path only), so nothing else in that table mentions fem. Verified:
`rg -n '🏗️fem' Cargo.toml` returns only the 17 member lines and the 3 alias lines above.

---

## 3. Cross-cutting dependents (grep-verified — exactly two)

`rg -l 'semio-s-app-fem-2d|semio-s-app-fem-3d|semio-s-plugin-fem-core|fem_core|fem2d_ui|…'`
outside `✏️s/🔌️plugins/🏗️fem/**` yields root `Cargo.toml` plus these two crates and nothing else.
There is no `s` run-crate/registry wiring for fem yet, and `🤖️generated/🦀️registry.rs` does not
mention it.

### 3.1 `semio-s-plugin-norm` — the 3-line fix

`✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/Cargo.toml` line 105 — replace:

```toml
fem_core = { path = "../../../🏗️fem/🔨️modules/🫀️core/⚡️implementations/🦀️rust", package = "semio-s-plugin-fem-core" }
```

with:

```toml
fem = { path = "../../../🏗️fem/📦️packages/🦀️rust", package = "semio-s-plugin-fem" }
```

(The comment on lines 103–104 already anticipates this repoint and should be dropped with it.)

Then in `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/⚙️engine/🦀️component.rs` (line 412) and
`✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/⚙️engine/🦀️component.rs` (line 821), replace the single
`use` line:

```rust
use fem_core::{BeamEb2, Dof, MemberUdl, Model, Node, Support};
```

with:

```rust
use fem::core::elements2d::BeamEb2;
use fem::core::{Dof, MemberUdl, Model, Node, Support};
```

and rename the remaining qualified references in those two files (`fem_core::StaticResult`,
`fem_core::ElementResult`, `fem_core::FemError`, `fem_core::solve_linear_static`) from `fem_core::`
to `fem::core::`. All four are re-exported at `core`'s root, so only the crate segment changes:

```
sd 'fem_core::' 'fem::core::'   # both en1992 and en1993 engine components
```

⚠️ `BeamEb2` is the ONE symbol whose path is not a plain prefix rename: in the old flat `fem_core`
crate it sat at the crate root, and in the consolidated crate it lives in the `elements2d`
sub-component (`fem::core::elements2d::BeamEb2`). A blanket `fem_core::` → `fem::core::` alone leaves
it unresolved. Nothing else norm imports moved.

### 3.2 `semio-framework-os-kernel-dsl` fixture-sweep

`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/⚡️implementations/🦀️rust/Cargo.toml`
lines 68–69 — replace the two aliases:

```toml
fem2d = { path = "../../../../../../../../✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/⚡️implementations/🦀️rust", package = "semio-s-app-fem-2d" }
fem3d = { path = "../../../../../../../../✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/⚡️implementations/🦀️rust", package = "semio-s-app-fem-3d" }
```

with one:

```toml
fem = { path = "../../../../../../../../✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust", package = "semio-s-plugin-fem" }
```

and in its `📦️lib.rs` lines 51–52 replace:

```rust
    use fem2d::Fem2dDocument;
    use fem3d::Fem3dDocument;
```

with:

```rust
    use fem::artifacts::fem2d::Fem2dDocument;
    use fem::artifacts::fem3d::Fem3dDocument;
```

Lines 108–109 of that file need no change — they only name the two documents through those `use`
aliases and read `envelope_id()` off them.

⚠️ Both fem envelope ids changed as part of this ticket (§5): the sweep's `("fem2d", …)` /
`("fem3d", …)` label column still reads `fem2d`/`fem3d`, but `envelope_id()` now returns
`fem.fem2d`/`fem.fem3d`. That column is only a human label, so it stays correct either way — but if
the sweep's fixture-discovery keys off the label, point it at `envelope_id()` instead.

---

## 4. Module path mapping table for consumers

| Old crate (`package` name) | Old symbol path | New symbol path |
| --- | --- | --- |
| `semio-s-plugin-fem-core` | `fem_core::{Dof, Model, Node, Support, MemberUdl, StaticResult, ElementResult, FemError, solve_linear_static, …}` | `semio_s_plugin_fem::core::…` |
| `semio-s-plugin-fem-core` | `fem_core::BeamEb2` (+ other 2-D element kernels) | `semio_s_plugin_fem::core::elements2d::…` |
| `semio-s-plugin-fem-core` | 3-D element kernels | `semio_s_plugin_fem::core::elements3d::…` |
| `semio-s-plugin-fem-core` | analyses / formulation / mesh / sparse / shared kernels | `…::core::{analyses, formulation, mesh, sparse, shared}::…` |
| `semio-s-plugin-fem-shared` | `fem_shared::…` | `semio_s_plugin_fem::core::shared::…` |
| `semio-s-app-fem-2d` | `fem2d::{Fem2dDocument, FemNode, FemElement, FemCamera, FEM_2D_SCHEMA, …}` | `semio_s_plugin_fem::artifacts::fem2d::…` |
| `semio-s-app-fem-2d-op` | `fem2d_op::{Fem2dOperation, Fem2dStore, …}` | `…::artifacts::fem2d::op::…` |
| `semio-s-app-fem-2d-dsl` | `fem2d_dsl::{parse_dsl, print_dsl, FEM2D_EXAMPLE_TEXT}` | `…::artifacts::fem2d::dsl::…` |
| `semio-s-app-fem-2d-pack` | `fem2d_pack::…` | `…::artifacts::fem2d::pack::…` |
| `semio-s-app-fem-2d-protocol` | `fem2d_protocol::{encode_op, decode_op}` | `…::artifacts::fem2d::spr::…` (node renamed `protocol` → `spr`) |
| `semio-s-app-fem-2d-engine` | `fem2d_engine::…` | `…::artifacts::fem2d::engine::…` (+ `::engine::{meshing, mesh_preview, modal_buckling}`) |
| `semio-s-app-fem-2d-ui` | `fem2d_ui::…` | `semio_s_plugin_fem::apps::fem2d::…` (+ `::commands::*`, `::modes::edit::windows::*`, `::config`, `::wasm`) |
| `semio-s-app-fem-3d*` | same shape as 2-D | `…::artifacts::fem3d::…` / `…::apps::fem3d::…` |
| `semio-s-plugin-fem-artifact-manifest` | `register_fem_exports()` | `semio_s_plugin_fem::core::register_all_engines()` (also invoked by the crate's own `semio_plugin!{ setup: … }`) |

The hand-rolled `Fem2dCommand`/`Fem3dCommand` enums that used to live in the two `📡️protocol` crates
are NOT in the `spr` nodes: they are regenerated by `app_commands!` and live at
`semio_s_plugin_fem::apps::fem2d::Fem2dCommand` / `…::apps::fem3d::Fem3dCommand`.

---

## 5. One behaviour change to be aware of (not a registrar edit)

Both fem documents were still declaring `#[dsl(extension = "fem2d")]` / `#[dsl(extension = "fem3d")]`,
which the universal-semio-format work made illegal — `SemioEnvelope::from_envelope_id` now requires a
`plugin.artifact` id, and every fem dsl/pack/spr round-trip test was panicking with
`InvalidPreamble("envelope id must be plugin.artifact, got fem2d")`. They are now
`#[dsl(id = "fem.fem2d")]` / `#[dsl(id = "fem.fem3d")]`, matching the already-regenerated
`🧬️component.fem.fem2d.dsl.semio` / `🧬️component.fem.fem3d.dsl.semio` fixtures and the
`plugin.artifact` convention every migrated peer (`block.block2d`, `norm.en1992`, …) follows. The
derived `EXTENSION` const is unchanged (`fem2d`/`fem3d` — it is the id's last dot-segment), so file
extensions and pack/binary encodings are untouched; only the one-line text preamble changed. The
identical fix was needed by the old crates, which were failing the same way before deletion.

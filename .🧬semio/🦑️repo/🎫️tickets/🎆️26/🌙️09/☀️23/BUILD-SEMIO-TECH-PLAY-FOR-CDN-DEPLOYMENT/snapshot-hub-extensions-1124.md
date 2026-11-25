# 🧩️ Snapshot Hub Extensions: dyn_enum_close! Value Dependency and artifact_kind Paths (11/24)

Tree edited: `/Users/ueli/.sinc/play` (only under `🌎️hub/`). Live repo used as read-only reference. **Nothing was compiled**: no cargo/rustc runs; the release build is the verifier.

## Trigger

`semio-hub-stdio-bim` (`/Users/ueli/.sinc/play/.cache/b/r/m/components/runs/wrMkAas/l/cargo-release-00-b2.log`, lines 176770–176854) had exactly two errors and two `unused-qualifications` warnings:

1. E0433 `semio_framework_value` not found in the expansion of `dyn_enum_close!` (`🏠️bim/🦀️.rs:8`).
2. E0425 `semio_s_artifact_stdio_ifc::artifact_kind` not found (`🏠️bim/🦀️.rs:47`).

The snapshot and live copies of the bim `🦀️.rs` and its `Cargo.toml` are identical, so there was nothing to port from live.

## Defect 1: dyn_enum_close! without semio-framework-value

- **Macro inventory:** `semio-framework-dispatch-macros` (`🧰️framework/🔨️modules/🔀️dispatch`) exports two macros, `#[dyn_enum]` and `dyn_enum_close!`.
  - The macro crate never names `semio_framework_value` itself.
  - The paths come from the trait signature captured in `__semio_dispatch_<Trait>`.
  - `PluginApp` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔌️app/🦀️.rs:96`) names `semio_framework_value::…` in its default method bodies.
  - `Decider` and `PrincipalResolver`, which `semio-hub-stores` closes, do not.
- **Hub has no `#[dyn_enum]` attribute uses** (comments only).
- **Sweep method:**
  - Scanned all 162 `[package]` crates under `🌎️hub/`.
  - For each crate, followed `#[path]` mounts from its `[lib] path`.
  - In those sources, found every `dyn_enum_close!` invocation.
  - Checked for `semio-framework-value` in `[dependencies]` (not `[dev-dependencies]`), or an `as semio_framework_value` alias.
- **Results:** 48 crates invoke the macro: 47 over `PluginApp`, plus `semio-hub-stores`, which needs nothing.
  - `semio-hub-layout` already has the alias `use semio_plugin::{…, __value as semio_framework_value}`.
  - Four crates were missing the dependency, and all four are fixed:

| Crate | Manifest change | Lockfile change (`/Users/ueli/.sinc/play/🌎️hub/Cargo.lock`) |
|---|---|---|
| `semio-hub-stdio-bim` | `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/🏠️bim/📦️packages/🦀️rust/Cargo.toml`: `semio-framework-value = { workspace = true }` added after `semio-framework-job` (`📘️pdf` sibling style) | `"semio-framework-value",` inserted after `"semio-framework-pack",` |
| `semio-hub-stdio-cad` | `…/🧩️extensions/🛠️cad/📦️packages/🦀️rust/Cargo.toml`: same line | same insertion |
| `semio-hub-stdio-semio` | `…/🧩️extensions/🧿️semio/📦️packages/🦀️rust/Cargo.toml`: same line | same insertion |
| `semio-hub-vcs` | `🌎️hub/🧩️compositions/🌿️vcs/📦️packages/🦀️rust/Cargo.toml`: the existing path line `semio-framework-value = { path = "../../../../../🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust" }` moved from `[dev-dependencies]` to `[dependencies]` (after `semio-framework-dispatch-macros`) | none: lockfile lists already include dev-dependencies, so `semio-hub-vcs` already lists `semio-framework-value` |

- **Lockfile edits:** 3 lines added in total.
  - A script asserted that each list was sorted and lacked the entry before inserting.
  - `semio-framework-value` has exactly one version in the lockfile.
- **Other lockfiles:** no other `Cargo.lock` outside `.cache`/`node_modules`/`target` contains any of the four packages.
- **Why the 1124 family-F sweep missed vcs:** it counted vcs's dev-dependency as present.

## Defect 2: artifact_kind() path

- `pub fn artifact_kind()` for ifc lives in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧩️composition/🦀️.rs:57`.
  - That file is the `[lib] path` of `semio-s-artifact-stdio-ifc-catalog` (`🏗️ifc/🧩️composition/📦️packages/🦀️rust/Cargo.toml`).
  - `semio-s-artifact-stdio-ifc` (lib `🏗️ifc/🦀️.rs`, deps only `semio-plugin` + `semio-s-artifact-stdio-contract`) neither defines nor re-exports it.
- **How the other stdio extensions get their kinds:** each format crate defines or re-exports `artifact_kind` at its root. For example, bcf has `pub use semio_s_artifact_stdio_bcf_inferences::{artifact_kind};`. ifc is the one format whose composition is split into a separate catalog crate, which bim already uses for `declaration(…)` on line 31.
- **Fix:** in `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/🏠️bim/🦀️.rs:47`, `semio_s_artifact_stdio_ifc::artifact_kind().id` became `semio_s_artifact_stdio_ifc_catalog::artifact_kind().id`.
  - No new dependency: `semio-s-artifact-stdio-ifc-catalog` is already a dependency.
  - The catalog crate has no hub dependencies, so there is no cycle.
- **Sweep method:** found 95 `<path>::artifact_kind()` call sites in 46 hub crates, all lib-reachable. Each named crate's root was resolved (through the workspace or path deps), and the module path was traced to a defining `fn` or `pub use`:
  - **Root re-exports:** `crate::artifacts::<x>` aliases in animate, architect, dag, demonstrator, flow, forms, imperative, lowpoly, mathematical, note, playbook, raster, reasoning, sequence, shooting, sourcing, vcs and writer all resolve to a root `pub use …::{artifact_kind}`. Lowpoly is the one exception: its inline `artifacts::lowpoly` module re-exports `lowpoly_model::snapshot::artifact_kind` itself.
  - **Module paths:** `cad_cad_model::snapshot`, `lowpoly`/`process` `::snapshot`, `bim_model_schema::…::schema`, `architect_program::…::schema` (via `program_schema`), `trinity_rewriting::…::schema::model` (via `rewriting_model` mounting `🔨️modules/🔺️model`), `gis_gismap` (root glob `artifact_declaration::*` via `gis_gismap_gismap`), and `puzzle_2d::…::schema` (`semio-hub-puzzle` compiled cleanly in the same log) all resolve.
  - All stdio, norm, block, wfc, space, procedural and remodel format crates export it at root.
  - **Result:** no other broken `artifact_kind()` call.

## Other errors in the bim block

None. The block has only the two errors above. The two `unused-qualifications` warnings on line 30 (`semio_plugin::PluginBuilder`, `semio_plugin::NeedsLabel`) are warnings only, and that line is the shared style of all stdio extensions, so it was left unchanged.

## Unresolved

- Nothing within scope.
- Out of scope but noted: the same log fails these non-hub crates:
  - `semio-hub-space-app` (hub, but a different defect, not part of this task)
  - `semio-s-artifact-raster-raster-model`, `semio-s-artifact-space-home-examples`, `semio-s-artifact-space-home-editor`, `semio-s-artifact-sequence-sequence-schema`
  - `semio-s-artifact-shooting-shooting-mutations`, `semio-s-artifact-stdio-dwg-editor(-ac1018)`, `semio-s-artifact-stdio-step-cc*-editor`
- Not verified by build: nothing was compiled.

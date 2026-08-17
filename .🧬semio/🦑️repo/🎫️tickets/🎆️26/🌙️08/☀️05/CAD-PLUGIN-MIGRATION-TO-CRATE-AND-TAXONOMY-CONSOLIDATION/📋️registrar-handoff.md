# 📋️ Registrar handoff — 📐️cad

Ticket `26/08/05/CAD-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`. The migrating agent
never touches root `Cargo.toml`/`Cargo.lock`, the registry script or `launch.json` — everything below
is the registrar's serialized pass.

## Root `Cargo.toml` — `members`

Remove these 8 literal member lines (all 8 directories are already deleted on disk):

```
    "✏️s/🔌️plugins/📐️cad/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🔨️modules/⚙️engine/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🔨️modules/🗣️dsl/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🔨️modules/🔧️op/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🔨️modules/🎒️pack/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🔨️modules/📡️protocol/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🔨️modules/🖱️ui/⚡️implementations/🦀️rust",
```

Add:

```
    "✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust",
```

(As of this handoff they were at lines 458 and 460-466 — line 459 is 🎪️demonstrator's member line and
must stay.)

## Root `Cargo.toml` — `[workspace.dependencies]`

Remove (the crate no longer exists; nothing consumes the alias):

```
semio-s-app-cad = { path = "✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/⚡️implementations/🦀️rust" }  # 9 refs
```

Optionally replace with the merged crate if the table is meant to stay exhaustive:

```
semio-s-plugin-cad = { path = "✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust" }
```

## Cross-cutting files this agent DID edit

* `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/⚡️implementations/🦀️rust/Cargo.toml`
  — `cad_document` dev-dependency repointed from `semio-s-app-cad` to `semio-s-plugin-cad`
  (`📦️packages/🦀️rust`), matching the writer/vcs/animate precedent already in that file.
* `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/⚡️implementations/🦀️rust/📦️lib.rs`
  — `use cad_document::CadProjection;` → `use cad_document::artifacts::cad::CadProjection;`.

## ⚠️ Cross-plugin dependents this agent did NOT edit

1. **🎪️demonstrator** (`✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust/Cargo.toml`,
   lines 121-123) — three real `[dependencies]`:
   ```
   cad_document        = semio-s-app-cad          → semio-s-plugin-cad, type at cad::artifacts::cad::*
   cad_document_engine = semio-s-app-cad-engine   → semio-s-plugin-cad, items at cad::artifacts::cad::engine::*
   cad_document_ui     = semio-s-app-cad-ui       → semio-s-plugin-cad, items at cad::apps::cad::*
                                                     (`create_cad_app`, `CadPlayApp`)
   ```
   All three collapse to ONE dependency on
   `{ path = "…/✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust", package = "semio-s-plugin-cad" }`.
   Demonstrator migrates strictly last, so this is expected to stay red until then.

2. **💠️lowpoly** (`✏️s/🔌️plugins/💠️lowpoly/🎛️apps/💠️lowpoly/🔨️modules/⚙️engine/⚡️implementations/🦀️rust/Cargo.toml`,
   `[dev-dependencies]`) — `cad_document_engine = semio-s-app-cad-engine`, used by exactly one
   test module (`📦️lib.rs` `//#region 🔖️ExportConcreteForestMeshTests`,
   `use cad_document_engine::geometry_import::{objects_from_fixture_model, parse_geometry};`).
   Repoint to:
   ```toml
   cad_plugin = { path = "…/✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust", package = "semio-s-plugin-cad" }
   ```
   ```rust
   use cad_plugin::artifacts::cad::engine::geometry_import::{objects_from_fixture_model, parse_geometry};
   ```
   Lib builds are unaffected; only `cargo test -p semio-s-app-lowpoly-engine` breaks until this lands.
   **This was NOT flagged anywhere in the plan or TEMPLATE** — lowpoly is a plugin outside this
   agent's ownership, so a scoped repair agent (or lowpoly's own migration agent) must do it.

## Metadata preserved verbatim

`[package.metadata.component] package = "semio:cad"`; playground `variant = "cad"`, `app = "cad-play"`,
`ports = { react = 6020, wgpu = 6120 }`; the `/cad-fixture` static-dir asset row. Launch regeneration
should therefore be a no-op for cad.

## Still un-run (needs a healthy root workspace)

* `cargo check --workspace` / `cargo clippy --workspace`
* `bun nx run @semio-tech/framework-os-dev:plugin -- cad`
* `bun ./📜️script.ts dev cad` boot smoke, `parity probe cad`
* `bun ./📜️script.ts verify gate`
* `bun …/📇️registry/📜️script.ts generate`

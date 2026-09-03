# 🔍️ Sourcing End to End — Diagnosis

## Reported symptom
`bun run dev:sourcing` exits status 1 (`nx run @semio-tech/framework-os-dev:dev -- sourcing`).

## What actually happens
1. The `dev` launcher itself is not the fault. Re-running it boots Vite on `127.0.0.1:6081`.
   The launcher only ran in `serving only` mode (plugin-build lease held by a peer pid), so it
   served whatever was already in `🧑️‍💻️dev/🔌️plugin-modules/`.
2. `🔌️plugin-modules/sourcing/*.wasm` is dated **Aug 26** while every healthy plugin was rebuilt
   Sep 1. The sourcing wasm is stale because the crate no longer compiles.
3. `cargo check -p semio-s-plugin-sourcing --target wasm32-wasip2` → **109 errors**.

## Root causes (2)

### A. `#[derive(dsl::Mutations)]` source-authority failure (108 of the 109 errors)
```
🧬️schema/🧬️mutations/🦀️component.rs:12:1: error: Mutations source authority failed:
aggregate source is not the taxonomy canonical mutation primary
```
`mutation_aggregate_source_authority` (🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs:91)
requires the file carrying the derive to be named after the taxonomy's
`mutationComponentFileKindId` = `rust-source` → emoji `🦀️` + `.rs` = **`🦀️.rs`**.
Sourcing's aggregate is still at `🧬️mutations/🦀️component.rs`.

The derive returns `to_compile_error()`, so **no** `impl protocol::Mutation<CurationSnapshot> for
SourcingMutation` is generated at all — which cascades into ~100 `E0277
SourcingMutation: Mutation<CurationSnapshot> is not satisfied` at every `MutationKind` impl,
the editor/viewer `ArtifactEditor::Mutation` associated type, io, and the operations surface.

Sourcing's mutation **leaves** were already migrated (`🌱create-curated-item/🦀️.rs` + `🔣️.json`);
only the aggregate primary was left behind, plus stale duplicate `🔣️component.json` descriptors
inside each leaf directory. Peers already on the canonical name: `🗄️stdio` (all artifacts),
`🌀️procedural` (2d/3d/assembly), `🌍️gis` (editor config + presence).

### B. Hand-written `Mutation` impls missing the new required items (2 × E0046)
`protocol::Mutation` (🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:105) now requires
`const DESCRIPTORS: &'static [MutationLeafDescriptor]` and `fn descriptor(&self)`.
Two sourcing impls predate that and declare neither:
- `✏️editor/🎚️config/🦀️component.rs:183` — `SourcingCurationConfigMutation` (8 variants)
- `✏️editor/👥️presence/🦀️component.rs:93` — `SourcingCurationPresenceMutation` (1 variant)

Reference pattern for a config/presence enum with no leaf triads on disk (placeholder descriptor
rows): `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs:194`.

## Fix plan
1. Rename `🧬️schema/🧬️mutations/🦀️component.rs` → `🦀️.rs`; repoint `📦️glue.rs:90`.
2. Point the aggregate's structural-correspondence test at the canonical `🔣️.json` leaf descriptor
   and drop the legacy duplicate `🔣️component.json` in each of the three leaf directories.
3. Add `DESCRIPTORS` + `descriptor()` to the config and presence `Mutation` impls.
4. Rebuild the wasm component, restart `dev sourcing`, verify the curation app live on :6081.

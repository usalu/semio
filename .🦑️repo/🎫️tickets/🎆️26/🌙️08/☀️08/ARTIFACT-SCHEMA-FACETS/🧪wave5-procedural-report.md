# Wave 5 Report — Procedural (Artifact Schema Facets)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Wave W5 agent owns `✏️s/🔌️plugins/🌀️procedural/**` plus this ticket folder.

Crate: `semio-s-plugin-procedural`. Artifacts: `procedural2d` (`Procedural2d*`), `procedural3d` (`Procedural3d*`).

## 1. What changed

### Fifteen facet leaves × 2

| Facet | Dir | Types |
| --- | --- | --- |
| artifact | `🗿️artifacts/<a>/🧬️schema/` (5 formats) | `Procedural2dArtifact` / `Procedural3dArtifact` |
| snapshot | `🗿️artifacts/<a>/📸️snapshot/🧬️schema/` (5) | `Procedural2dSnapshot` / `Procedural3dSnapshot` |
| diff | `🗿️artifacts/<a>/🔺️diff/🧬️schema/` (5) | `Procedural2dDiff` / `Procedural3dDiff` |

Schema ids: `s.procedural.procedural2d`, `s.procedural.procedural3d`.

### Pack relocation

`🎒️pack/` moved under `📸️snapshot/🎒️pack/` for both artifacts (plain `mv`).

### Rename

`Procedural2dDocument` / `Procedural3dDocument` replaced by `Procedural2dSnapshot` / `Procedural3dSnapshot` everywhere in the plugin (no alias). Defined in `snapshot::schema`; artifact roots re-export.

### Diff as sparse field delta

`Procedural*Diff` is a sparse field delta (not a mutation list):

- `artifact: Option<Box<XArtifact>>` whole replacement
- `fixture: Option<FlowFixture>` / `generation: Option<GenerationPlayState>` persistent entries
- UI entries for every non-effect artifact field
- optional lists wrapped as `Procedural*StringList`
- `MutationDiff<XSnapshot>` applies persistent entries; `apply_to_artifact` applies all
- Mutations construct deltas via `diff_fixture_from_helpers` / `diff_generation_from_ops`

### Engine

Each engine owns a real `XArtifact` + cached `XSnapshot` (`type Artifact = XArtifact`, never collapsed to Snapshot). `apply` diffs + mutates snapshot then `artifact.set_snapshot(...)`.

### Registry

`procedural2d_artifact_schema_descriptor()` / `procedural3d_artifact_schema_descriptor()` `include_str!` all 15 leaves. Registered from `engine::register()`.

### Glue

Leaf-prefixed + grouping `#[path = "."]`. Nested `snapshot` / `diff` keep `../../`. Diff runtime: `pub use super::schema::*;`. TS index mirrors schema/snapshot/diff/pack paths.

## 2. Field inventory

### procedural2d

**Persistent** (= snapshot): `fixture` (`FlowFixture`), `generation` (`GenerationPlayState`).

**SharedUi**: `selectedIds`, `selectedGenerationId?`.

**LocalUi**: `graphCamera`, `showMode`, `locale`.

**Preview**: `generationPreviewText?`.

**Effect:** none.

### procedural3d

**Persistent** (= snapshot): `fixture`, `generation`.

**SharedUi**: `selectedNodeIds`, `selectedGenerationId?`, `activeUtilityId`.

**LocalUi**: `lodMode`, `showMode`, `selectionMethod`, `graphCamera`, `previewCamera` (named XYZ scalars), `sunJson`, `locale`, `contributionsJson`.

**Preview**: `hoveredNodeId?`, `generationPreviewText?`.

**Effect:** none.

### Diff-delta shape

`artifact?`, `fixture?`, `generation?`, plus `Option` / `Option<Option<_>>` / `Option<XStringList>` entries for every non-effect artifact field.

## 3. Glue convention

Leaf-prefixed + grouping `#[path = "."]` (same as lowpoly pilot §15.5).

## 4. Gate tails (verbatim)

### 1. `cargo check -p semio-s-plugin-procedural`

```
error[E0599]: no method named `projection` found for struct `DocumentStore<P, Mutation>` in the current scope
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/../../🖥️host/🦀️component.rs:1772:48
     |
1772 |             let committed = self.history_store.projection().unwrap_or_else(|_| self.fixture.clone());
     |                                                ^^^^^^^^^^ method not found in `DocumentStore<document::FlowFixture, vcs::FlowMutation>`

error[E0599]: no method named `projection` found for struct `DocumentStore<P, Mutation>` in the current scope
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/../../🖥️host/🦀️component.rs:1786:51
     |
1786 |         let Ok(mut restored) = self.history_store.projection() else {
     |                                                   ^^^^^^^^^^ method not found in `DocumentStore<document::FlowFixture, vcs::FlowMutation>`

error[E0599]: no method named `projection` found for struct `DocumentStore<P, Mutation>` in the current scope
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/../../🖥️host/🦀️component.rs:1801:51
     |
1801 |         let Ok(mut restored) = self.history_store.projection() else {
     |                                                   ^^^^^^^^^^ method not found in `DocumentStore<document::FlowFixture, vcs::FlowMutation>`

For more information about this error, try `rustc --explain E0599`.
warning: `semio-framework-os-flow` (lib) generated 128 warnings
error: could not compile `semio-framework-os-flow` (lib) due to 3 previous errors; 128 warnings emitted
```

**FAIL** — blocked by shared framework surface (see §5). No procedural-plugin compile errors reached.

### 2. `cargo test -p semio-s-plugin-procedural --lib`

```
error[E0599]: no method named `projection` found for struct `DocumentStore<P, Mutation>` in the current scope
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/../../🖥️host/🦀️component.rs:1772:48
     |
1772 |             let committed = self.history_store.projection().unwrap_or_else(|_| self.fixture.clone());
     |                                                ^^^^^^^^^^ method not found in `DocumentStore<document::FlowFixture, vcs::FlowMutation>`

error[E0599]: no method named `projection` found for struct `DocumentStore<P, Mutation>` in the current scope
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/../../🖥️host/🦀️component.rs:1786:51
     |
1786 |         let Ok(mut restored) = self.history_store.projection() else {
     |                                                   ^^^^^^^^^^ method not found in `DocumentStore<document::FlowFixture, vcs::FlowMutation>`

error[E0599]: no method named `projection` found for struct `DocumentStore<P, Mutation>` in the current scope
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/../../🖥️host/🦀️component.rs:1801:51
     |
1801 |         let Ok(mut restored) = self.history_store.projection() else {
     |                                                   ^^^^^^^^^^ method not found in `DocumentStore<document::FlowFixture, vcs::FlowMutation>`

For more information about this error, try `rustc --explain E0599`.
warning: `semio-framework-os-flow` (lib) generated 128 warnings
error: could not compile `semio-framework-os-flow` (lib) due to 3 previous errors; 128 warnings emitted
```

**FAIL** — same dependency failure; crate tests never started.

### 3. `bun ./📜️script.ts policy 2>&1 | rg -i 'procedural'`

```
(empty — no lines matched)
```

Direct `policyArtifactSchemaBreaches()` filter: **procedural artifact-schema breaches: 0** (`🧪wave5-procedural-policy-probe.ts`).

## 5. Shared-surface blocker — FIXUP REQUIRED

`semio-framework-os-flow` `🖥️host/🦀️component.rs` still calls `DocumentStore::projection()` at lines **1772, 1786, 1801** (plus test-only sites ~2999–3007). Store API is now `snapshot()` after the DocumentApp Projection→Snapshot fixup.

Outside `✏️s/🔌️plugins/🌀️procedural/` — not touched per fan-out brief. **Fixup:** rename those call sites to `snapshot()`.

## 6. Could not validate

- Full compile / lib tests of `semio-s-plugin-procedural` (blocked by §5).
- Runtime engine ownership of `XArtifact`.
- `semio-framework-schema` registry table-driven test against registered descriptors.

## 7. Ticket probes

- `🧪wave5-procedural-gen-leaves.py`
- `🧪wave5-procedural-policy-probe.ts`
- `🧪wave5-procedural-report.md`

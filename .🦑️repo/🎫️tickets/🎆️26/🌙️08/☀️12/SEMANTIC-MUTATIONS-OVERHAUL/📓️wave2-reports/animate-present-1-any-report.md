# Wave 2 facet report — `animate` / `present` / standard `1` / subset `any` / `🧬️mutations`

Crate: `semio-s-plugin-animate`. Facet: `.../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`.

## Vocabulary derived

Old generic dispatch (`PresentMutation`):
- `Tiles(protocol::CollectionMutation<String, FigureTileDraft, FigureTileDraftPatch>)`
- `SetSource { source: FigureTileSource }`
- `SetTiles { tiles: Vec<FigureTileDraft> }`
- `SetSnapshot { snapshot: PresentSnapshot }`

New semantic dispatch (9 variants, all `#[derive(dsl::Mutations)]`-wired, replacing the above 1:1
in coverage minus the banned whole-document replace):

| Variant | Verb | Entity | Replaces | Triad dir |
|---|---|---|---|---|
| `ResizeSourceFrame` | resize | source-frame | `SetSource` (frame-only slice; the app's `set-frame` gesture) | `🔲resize-source-frame/` |
| `ReplaceSource` | replace | source | `SetSource` (whole-source swap; the app's `set-source` gesture) | `🖼replace-source/` |
| `CreateTile` | create | tile | `Tiles(CollectionMutation::Add)` | `🆕create-tile/` |
| `DeleteTile` | delete | tile | `Tiles(CollectionMutation::Remove)` | `🗑delete-tile/` |
| `DeleteTiles` | delete | tiles | new plural (the app's real `delete-selection` multi-target gesture, formerly N individual `Tiles(Remove)` mutations) | `🗑delete-tiles/` |
| `RenameTile` | rename | tile | `Tiles(CollectionMutation::Patch{name})` | `✏rename-tile/` |
| `ResizeTileCrop` | resize | tile-crop | `Tiles(CollectionMutation::Patch{crop})` | `✂resize-tile-crop/` |
| `ReorderTiles` | reorder | tiles | `Tiles(CollectionMutation::Move)` | `🔀reorder-tiles/` |
| `ReplaceTiles` | replace | tiles | `SetTiles` (both the grid-reseed and the clear-on-source-change gestures — an empty `new_tiles` payload is the "clear" case, no separate `clear-tiles` verb needed) | `🔁replace-tiles/` |

`SetSnapshot` has **no replacement** — per the taxonomy's locked decision, whole-document replace
is not an in-history mutation; it must go through `ArtifactStore::reset` at the app layer (see
`sharedFileRequests`).

Each triad leaf follows the derivation-rules shape exactly (payload struct in `🦠️mutation`
deriving `dsl::DslRecord` + `impl protocol::MutationKind`, real handcrafted `pub fn diff` in
`🔺️diff`, real handcrafted `pub fn inverse` in `↩️inverse`, delegated from the payload's
`MutationKind::diff`/`inverse` via `super::diff::diff`/`super::inverse::inverse`) — mirrors the
already-landed `gis`/`cad`/`fem` wave2 facets byte-for-byte in style. `diff` construction reuses the
sibling `🔺️diff` facet's existing `tiles_delta_from_collection_mutation`/`tiles_delta_from_set_tiles`
helpers (internal `CollectionMutation`-based diff engine, never surfaced in the public enum — same
pattern `gismap`'s `create-position`/`delete-position`/`reorder-positions` use).

## Module wiring (the one deliberate deviation from the sibling-plugin precedent)

`gis`/`cad`/`fem`'s wave2 facets wire each new triad leaf directly in their plugin's `📦️glue.rs`
(`pub mod <slug> { #[path=...] pub mod mutation; pub mod diff; pub mod inverse; }` nested inside the
`mutations` block). This task's instructions explicitly put `📦️glue.rs` out of bounds (plugin-shared,
edited by other concurrent facet sessions for this same plugin). Since `📦️glue.rs` already loads
`🧬️mutations/🦀️component.rs` via `#[path]` as `mod component; pub use component::*;`, and any
`pub mod X { ... }` declared **inside** that component file is transparently re-exported through the
same `pub use component::*;` chain, I moved the identical wiring shape one level down: each new
leaf's group module (`pub mod create_tile { #[path="🆕create-tile/🦠️mutation/🦀️component.rs"] pub mod
mutation; ... }`) is declared directly inside `🧬️mutations/🦀️component.rs` itself, with `#[path]`
targets relative to that file's own directory. The resulting external paths are byte-identical to
the sibling convention (`crate::artifacts::present::mutations::create_tile::mutation::CreateTile`,
etc.) — confirmed by successful cross-references from the `📝️text`/`💾️binary`/`🔺️diff` facets and by
`cargo check`. No `📦️glue.rs` edit was needed or made.

The 4 old generic triad directories (`🎞tiles`, `📋set-tiles`, `📎set-source`, `📸set-snapshot`)
could **not** be deleted, because `📦️glue.rs` still has fixed `#[path]` mod declarations pointing at
their 12 files. Deleting the files would break the crate at the parse stage (missing `#[path]`
target), and I'm not allowed to edit `📦️glue.rs` to remove those declarations. Each of the 12 files
was instead emptied to a doc-comment-only stub explaining the supersession and pointing at this
report. They compile (trivially) but are dead code. **`sharedFileRequests` asks a later pass to
delete `🎞tiles/`, `📋set-tiles/`, `📎set-source/`, `📸set-snapshot/` and their 3 `📦️glue.rs`
`pub mod { ... }` blocks (lines documented below) once `📦️glue.rs` is back in scope.**

## Files touched

Created (27 new files — 9 triad leaves × 3):
`🔲resize-source-frame/`, `🖼replace-source/`, `🆕create-tile/`, `🗑delete-tile/`, `🗑delete-tiles/`,
`✏rename-tile/`, `✂resize-tile-crop/`, `🔀reorder-tiles/`, `🔁replace-tiles/`, each with
`🦠️mutation/🦀️component.rs`, `🔺️diff/🦀️component.rs`, `↩️inverse/🦀️component.rs`.

Rewritten:
- `🧬️mutations/🦀️component.rs` — dispatch enum + leaf module wiring + tests + `apply_present_mutation`/`inverse_present_mutation`.
- `🧬️mutations/📝️text/🦀️component.rs` — `PresentMutation` now derives `dsl::DslEnum` directly (no
  more `PresentMutationDsl` mirror enum — that bridge existed only because the old `Tiles` variant
  wrapped the foreign `protocol::CollectionMutation<..>` generic, an orphan-rule dead end; every new
  payload is a plain local struct, so `dsl::DslRecord` applies directly, matching `gis`/`cad`).
- `🧬️mutations/💾️binary/🦀️component.rs` — 3 tests updated to the new variants.
- `🔺️diff/📝️text/🦀️component.rs` (sibling `diff` facet, same artifact, **not** my assigned facet but
  broken by my required enum change) — 1 test (`set_source_diff_applies_onto_the_base_snapshot` →
  `replace_source_diff_applies_onto_the_base_snapshot`) updated to construct `ReplaceSource` instead
  of the deleted `SetSource`.

Emptied to stub (kept only because `📦️glue.rs` still `#[path]`-wires them — see above):
`🎞tiles/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`,
`📋set-tiles/{...}/🦀️component.rs`, `📎set-source/{...}/🦀️component.rs`,
`📸set-snapshot/{...}/🦀️component.rs` (12 files).

## Tests

Extended the existing `🧪️Tests` regions (no new test files): `🧬️mutations/🦀️component.rs` gained 8
new `#[test]` fns covering create/rename/resize/delete/reorder/delete-tiles/replace-tiles/
replace-source/resize-source-frame round trips, missing-target → `Vec::new()` cases, and
`PresentMutation::kinds()` coverage. Added `protocol::testkit::assert_mutation_inverse_law` /
`assert_mutation_diff_absorb_law` calls for `create-tile`, `rename-tile`, `replace-source` (crate
already depends on the framework crate that hosts `testkit` — confirmed via `gis`/`cad`/`fem`
precedent, no new Cargo dependency needed). `🧬️mutations/📝️text/🦀️component.rs`'s existing op-text
round-trip tests were rewritten for the 9 new variants; `💾️binary/🦀️component.rs`'s 3 existing tests
updated in place.

**Could not actually execute (`cargo test`)**: the crate fails to *compile* its test binary because
5 app-level files (outside my boundary, see `sharedFileRequests`) still construct the deleted
`SetSource`/`SetTiles`/`SetSnapshot`/`Tiles(CollectionMutation::..)` variants. Confirmed via
`cargo test -p semio-s-plugin-animate --lib -- ...mutations::` that **all 17 remaining compile
errors are in `🎛️apps/🎬️present/**`** (zero errors anywhere under `🗿️artifacts/🎬️present/`, for both
plain `cargo check` and the test-cfg-enabled `cargo test` compile pass — the latter did catch and I
fixed one real issue of my own, a missing `protocol::SemanticMutation` import for `PresentMutation::
kinds()`). I have not run the tests and am not claiming they pass at runtime; `lawTestsPass` is
reported `false` for that reason, not because of a known logic defect.

## sharedFileRequests — exact changes needed once `🎛️apps/🎬️present/**` is back in scope

1. `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🎮️commands/🖼️source/🦀️component.rs`
   - L23: `PresentMutation::SetSource { source: payload.source.clone() }` → `PresentMutation::ReplaceSource(replace_source::mutation::ReplaceSource { new_source: payload.source.clone() })`
   - L26: `PresentMutation::SetTiles { tiles: Vec::new() }` → `PresentMutation::ReplaceTiles(replace_tiles::mutation::ReplaceTiles { new_tiles: Vec::new() })`
   - L49: `PresentMutation::SetSource { source }` → `PresentMutation::ReplaceSource(replace_source::mutation::ReplaceSource { new_source: source })`
   - L66: `PresentMutation::SetSnapshot { snapshot: default_present_snapshot() }` — whole-document reset (the "load demo example" gesture). No `PresentMutation` replacement exists by design; needs a real design decision to route through `ArtifactStore::reset`/a non-history reset path instead of `Emit::artifact_mutations`, which is out of this facet's scope.
2. `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🎮️commands/🌐️grid/🦀️component.rs` — L25, L46, L60: `PresentMutation::SetTiles { tiles }` / `{ tiles: Vec::new() }` → `PresentMutation::ReplaceTiles(replace_tiles::mutation::ReplaceTiles { new_tiles: tiles })` (empty vec for the clear case).
3. `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🎮️commands/⌨️engagement/🦀️component.rs`
   - L28: `PresentMutation::SetTiles { tiles }` → `PresentMutation::ReplaceTiles(replace_tiles::mutation::ReplaceTiles { new_tiles: tiles })`
   - L38: `PresentMutation::Tiles(protocol::CollectionMutation::Add { index: deck.tiles.len(), item: tile })` → `PresentMutation::CreateTile(create_tile::mutation::CreateTile { index: deck.tiles.len(), tile })`
   - L44: `PresentMutation::SetTiles { tiles: Vec::new() }` → `PresentMutation::ReplaceTiles(replace_tiles::mutation::ReplaceTiles { new_tiles: Vec::new() })`
4. `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🎮️commands/🀄️tile/🦀️component.rs`
   - L30: `PresentMutation::Tiles(CollectionMutation::Add { index: deck.tiles.len(), item: tile })` → `PresentMutation::CreateTile(create_tile::mutation::CreateTile { index: deck.tiles.len(), tile })`
   - L57 (`delete_tile`) and L81 (`delete_selection`): `.map(|id| PresentMutation::Tiles(CollectionMutation::Remove { id }))` → either `.map(|id| PresentMutation::DeleteTile(delete_tile::mutation::DeleteTile { id }))`, or (better, `delete_selection` is exactly the multi-select gesture the new plural was minted for) a single `PresentMutation::DeleteTiles(delete_tiles::mutation::DeleteTiles { ids: targets })`.
   - L110 (`rename_tiles`): `PresentMutation::Tiles(CollectionMutation::Patch { id, patch: FigureTileDraftPatch { name: Some(name.into()), crop: None } })` → `PresentMutation::RenameTile(rename_tile::mutation::RenameTile { id, new_name: name.into() })`
   - L143 (`patch_tile_crops`): `PresentMutation::Tiles(CollectionMutation::Patch { id: tile.id.clone(), patch: FigureTileDraftPatch { name: None, crop: Some(clamp_tile_crop(&crop)) } })` → `PresentMutation::ResizeTileCrop(resize_tile_crop::mutation::ResizeTileCrop { id: tile.id.clone(), new_crop: clamp_tile_crop(&crop) })`
5. `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🦀️component.rs`
   - L157: `PresentMutation::SetSnapshot { snapshot }` — same whole-document-reset concern as item 1's L66.
   - L175: `PresentMutation::Tiles(CollectionMutation::Add { index: count, item: tile })` → `PresentMutation::CreateTile(create_tile::mutation::CreateTile { index: count, tile })`

All of the above need `use crate::artifacts::present::mutations::{create_tile, delete_tile,
delete_tiles, rename_tile, resize_tile_crop, replace_source, replace_tiles};` (or the equivalent
already-in-scope alias) added to each file.

6. `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/📦️glue.rs` — once this facet's directory restructure is
   final across the whole plugin: delete the 3 stale `pub mod tiles { ... }` / `pub mod set_tiles {
   ... }` / `pub mod set_source { ... }` / `pub mod set_snapshot { ... }` blocks under `pub mod
   mutations { ... }` (they now point at doc-comment-only stub files), then delete
   `🎞tiles/`, `📋set-tiles/`, `📎set-source/`, `📸set-snapshot/` on disk. Optional (not required for
   correctness, since `pub use component::*;` already surfaces everything): mirror this facet's 9 new
   `pub mod <slug> { ... }` blocks (currently declared inside `🧬️mutations/🦀️component.rs`) directly
   in `📦️glue.rs` to match the `gis`/`cad`/`fem` convention exactly.

## Grammar / protocol text mirrors

Not updated (`📖️component.grammar.semio`/`📡️component.protocol.semio` under `📝️text`/`💾️binary`
weren't touched, and no grammar file exists at the `🧬️mutations/` root for this facet, unlike
`gismap`) — step (f) is explicitly non-blocking and time did not allow it this pass.

## Verification

- `cargo check -p semio-s-plugin-animate`: 0 errors under `🗿️artifacts/🎬️present/`; 17 pre-existing
  `E0599` errors, all in `🎛️apps/🎬️present/**` (listed above), all directly caused by this facet's
  required deletion of the generic variants and requiring the `sharedFileRequests` changes above —
  not unrelated workspace churn.
- `cargo test -p semio-s-plugin-animate --lib -- ...mutations::`: same 17 app-level errors (one more
  than the plain `check` run at first — a missing `protocol::SemanticMutation` import in this
  facet's own test code for `PresentMutation::kinds()`, fixed). No compile errors of my own remain;
  actual test execution is blocked until the app layer is reconciled.

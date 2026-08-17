# W6 — `semio-s-plugin-animate` Hover/Selection Migration

## Scope

`✏️s/🔌️plugins/🎞️animate` (app `animate-present-play` / window `tile-editor`) held REAL selection
state — `PresentConfig.selected_ids`, mirrored into `PresentPresence.selected_ids` for peer broadcast,
written by `set-selected-ids`/`canvas-pointer-down`/`add-tile`/`delete-tile`/`delete-selection`/
`seed-grid`/`reset-grid`/`clear-tiles`/`set-source`/`set-active-example`/`engagement-submit`, and read
by the document panel tree, the tile-editor canvas highlight, and a rich per-tile inspection panel
(crop x/y/width/height editors, rename, delete). This was a genuine migration, not just signature
appeasement.

## Domain declared

`tiles` (granularity `tile`), `HierarchyProvider::Flat`, `SelectionMode::[Multiple, Single]`,
`SelectionMethod::[Pick]`, `MergeMode::[Replace, Additive, Subtractive, Invertive]`, `broadcast: true`,
non-transitive. Bound to the `tile-editor` window (`.window_kind_interactions`) and to the document
panel's `UiTree` (`PanelTreeBuilder::interaction_domain("tiles")`).

## What changed

- **Config** (`🎚️config`): deleted `selected_ids` field + `SetSelectedIds` mutation variant (+ its
  `"selection"` dsl key, diff/inverse arms, tests) from `PresentConfig`/`PresentConfigMutation`, and
  the 5 schema-leaf mirrors (rust/ts/graphql/json/proto).
- **Presence** (`👥️presence`): `PresentPresence` had NO other field, so it is now genuinely empty
  (`pub struct PresentPresence {}`) across all 5 language mirrors — selection broadcasts through the
  framework's typed `PresenceInteraction` now.
- **Commands**: deleted the `set-selected-ids` command dir + its `📦️glue.rs` mount + its manifest
  `.view_action(...)` row + its `PresentCommand` row. `canvas-pointer-down` keeps its hit-test (the
  canvas is the only thing that knows which tile a click landed on) but now emits the framework's
  `interactionSelect` verb via `HostEffect::ReplayShellCommand` instead of a `PresentConfigMutation`
  (mirrors `🖍️draw`'s `canvas-pointer-down`). `add-tile`/`seed-grid`/`reset-grid`/`engagement-submit`
  ("add"/grid-pattern branches) likewise emit `interactionSelect` to preserve the pre-migration "select
  the newly created tile" UX; `clear-tiles`/`set-source`(on src change)/`set-active-example`("demo")
  emit an empty-selection `interactionSelect` to preserve "clear the selection" UX. `delete-tile` and
  `delete-selection` no longer write selection back — `tiles` is `Flat`, so the framework never
  auto-prunes it on document change (documented, accepted gap, exact precedent `🖍️draw`'s
  `delete-layer` established); `delete-selection` reads the live selection via `app_commands!`'s
  `ctx = PresentDispatchCtx` mechanism (mirrors `🗒️note`/`🖍️draw`'s session/ctx pattern), populated
  once per dispatch in `AnimatePresentPlayApp::handle` from `interaction.selection("tiles").ids`. Every
  one of the 18 remaining command handlers gained the `ctx: &mut PresentDispatchCtx` 4th param (the
  `app_commands!` `ctx = ...` arm applies it uniformly; most ignore it as `_ctx`).
- **App** (`🦀️component.rs`): fixed a pre-existing, unrelated compile break first (top-of-file
  `use commands::{engagement, grid, shell, source, tile, view}` grouped-module imports that never
  existed in `📦️glue.rs`'s flat command mounts — same commit `865fa1cc5b` introduced both app.rs and
  glue.rs; rewired to flat imports matching every other migrated app's convention). Added
  `PRESENT_INTERACTION_DOMAIN`/`PRESENT_INTERACTION_GRANULARITY` consts, `PresentDispatchCtx`,
  `interaction_select_effect`/`interaction_targets_json` helpers (mirrors `🖍️draw`). `ArtifactApp::handle`
  gained the `interaction: &InteractionView<'_>` param and builds `ctx` from it. `render` dropped the
  `config.selected_ids` read (no longer exists) and no longer threads `selected` into the three body
  renderers. Manifest gained `.interaction(InteractionDefinition { .. })` +
  `.window_kind_interactions(...)`; dropped the `setSelectedIds` `.view_action`.
- **Document panel** (`📌️panels/📄️artifact`): rebuilt on `PanelTreeBuilder` +
  `.interaction_domain("tiles")` — deleted the raw `UiTreeItemNode`/`UiTreeNode` literals that used the
  now-gone `hover_action`/`unhover_action`/`selected_ids`/`highlighted_ids`/`selection_change` fields
  and the per-row `setSelectedIds` action (row clicks now auto-inject `interactionSelect`).
- **Tile-editor window** (`🎭️modes/🖊️main/🪟️windows/🖼️tile-editor`): added the required `interactions:
  Vec::new()` field (populated by `.window_kind_interactions` in the manifest). Canvas layer rendering
  dropped the `selected` param and the `"tile-selected"` kind it drove — `ArtifactApp::render` is never
  given an `InteractionView`, so the client renders the selection highlight itself now (same reasoning
  and same precedent as `🖍️draw`'s canvas render).
- **Inspection panel** (`📌️panels/🔍️inspection`): **documented reduced-fidelity gap**, same shape as
  `🖍️draw`'s `properties` panel and `📐️cad`'s `inspection` panel — the per-selected-tile field group
  (crop x/y/width/height editors, name, delete-tile/delete-selection buttons) is deleted because
  `ArtifactApp::render(body_key, doc, cfg)` is never given an `InteractionView` (only
  `handle`/`copy_fragment`/`cut_operations` are — verified against the real W3b-landed trait, not the
  master plan's aspirational text). Falls through to a schema/tile-count summary. Two new terminology
  labels (`details_schema_field`/`details_tiles_field`) replace 13 now-dead label keys
  (`details_select_tile`, `details_tile_not_found`, `field_name`, `field_id`, `selected_suffix`,
  `delete_tile`, `delete_selection`, `group_crop`, `field_x/y/width/height`, `group_identity`), which
  were deleted from `🗣️terminology`.
- **Tests**: extended in-file `mod tests` throughout (no new test files). Two tests now drive the real
  `interactionSelect` action end-to-end (`delete-selection`'s own file, `add-tile`'s own file) mirroring
  `🪐️space`'s `delete_selection_removes_the_live_selected_node` precedent. `canvas-pointer-down` gained
  its own test asserting the emitted `HostEffect::ReplayShellCommand`. Relocated the two
  locale-resolution tests that lived inside the deleted `set-selected-ids` file into `🗣️terminology`'s
  existing test mod (unrelated to selection). Dropped one test
  (`build_details_tree_reports_tile_not_found_for_stale_selection`) whose behavior no longer exists
  (inspection panel can't distinguish selection state any more — same accepted gap as above).

## Known gap flagged, not touched

`🗿️artifacts/🎬️present/…/🧬️schema/🦀️component.rs`'s `PresentArtifact`/`🔺️diff/🦀️component.rs`'s
`PresentDiff` (the `ArtifactSchema`-derive-driven "whole artifact tuple" descriptor used for the 20
handcrafted `s.animate.present` schema leaves) still carry a `#[state(presence)] selected_ids` field.
Traced and confirmed **disconnected from real runtime state**: `PresentArtifact::to_snapshot`/
`from_snapshot`/`set_snapshot` never touch it, and `PresentDiff`'s real `MutationDiff<PresentSnapshot>::
apply` never touches it either (only `schema`/`presentation`) — it only feeds `PresentDiff::
apply_to_artifact`/`absorb`, which nothing outside this file's own tests calls. Pre-dates the B1
pure-trait pivot that introduced the real `PresentConfig`/`PresentPresence` types and was never cleaned
up. Left alone: touching the `ArtifactSchema`/`dsl::Mutations` derive-governed multi-language schema
tier (5 more leaf files: ts/graphql/json/proto) for a field with zero live readers was judged
out-of-proportion risk for this wave; flagged here for a future pass.

## Acceptance (real output, same folder)

- `cargo check -p semio-s-plugin-animate` → 0 errors, 16 pre-existing/unrelated warnings (`w6-animate-cargo-check.txt`).
- `cargo test -p semio-s-plugin-animate` → **228 passed, 0 failed** (`w6-animate-cargo-test.txt`).

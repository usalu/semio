# W6 — `semio-s-plugin-reasoning-mindmap` (app `🔌️wires`) Hover/Selection Migration

Crate missed by the original 17-crate inventory (W4), migrated as part of W6's mop-up sweep.
Directory: `✏️s/🔌️plugins/💡️reasoning`.

## Decision: domain shape disagrees with the pre-staged brief

The per-crate brief guessed `HierarchyProvider::Topology over parent links`. The real schema
(`✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`)
is a normal undirected identity/relationship graph built on `infinite_board_normal_undirected`
(literally that crate's own name) — no `parent`/`group_id` field exists anywhere in the node/edge
shape, unlike writer's AST or procedural's DAG. I declared the domain `HierarchyProvider::Flat`
instead, matching the master doc's own rule ("the rest are Flat") and layout's/note's identical
Flat-domain pattern. `interaction_topology` is NOT overridden (trait default is correct for Flat).

## What changed

- **Domain**: `"graph"`, granularities `"node"`/`"edge"` (first = default), `Flat`, single-select,
  `Pick` method only, `Replace` merge only (matches the pre-migration click-to-select behaviour —
  no marquee/lasso/modifier-merge ever existed in this app), hover left at framework defaults
  (`HoverSpec::default()` — the app never hand-rolled its own hover).
- **`WiresConfig`/`WiresConfigMutation`** (`🎚️config/🦀️component.rs` + schema leaves, all 5
  facets: rust/ts/graphql/json/proto): deleted `selected_ids` field and the `SetSelection`
  mutation variant + its `diff`/`inverse` arms; kept `drag_node_id`/`drag_last_x`/`drag_last_y`
  (genuinely app-specific in-flight drag state) and `locale`.
- **`WiresPresence`** (`👥️presence/🦀️component.rs` + schema leaves): deleted `selected_ids`,
  kept drag fields (peer-visible drag is still app-specific; selection now broadcasts via the
  framework's own typed `PresenceInteraction`).
- **Artifact-combined schema + diff facets** (`🗿️artifacts/…/🧬️schema/🦀️component.rs` and
  `…/🔺️diff/{🦀️component.rs,📝️text/🦀️component.rs}` + their ts/graphql/json/proto mirrors):
  deleted `selected_ids`/`WiresStringList` everywhere they appeared (these are separate handcrafted
  facet leaves that mirrored the same field, missed by a config/presence-only sweep).
- **Deleted command dirs**: `🎮️commands/🗂️set-selection`, `🎮️commands/🗂️document-select` (both
  did exactly `WiresConfigMutation::SetSelection`) — removed from `📦️glue.rs`'s `pub mod` list and
  from the `app_commands!` table (2 rows). `WiresCommand` row count: 12 → 10.
- **`canvas-pointer-down`**: hit-test unchanged; no longer writes `SetSelection` — requests a
  `HostEffect::DispatchAction{action: interactionSelect}` for the "graph"/"node" target (mirrors
  layout's/space's identical pattern). `SetDrag` config mutation is untouched (app-specific).
- **`add-node`/`add-relationship`**: the newly-created node/edge is selected via the same
  `interactionSelect` effect instead of a `SetSelection` config mutation.
- **`set-active-example`**: dropped the `SetSelection{ids:[]}` mutation (selection is no longer a
  config field to clear); kept `SetDrag` reset.
- **`delete-selection`**: `handle`/`apply` split (per the brief's pointer to
  `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎮️commands/🧩️delete-selection`) — `apply` reads
  `interaction.selection("graph").ids`; `handle` (reachable only through the macro-generated,
  interaction-less `dispatch`) degrades to an empty selection.
- **`ReasoningWiresPlayApp::handle`**: gained the `interaction: &InteractionView<'_>` parameter;
  special-cases `WiresCommand::DeleteSelection` to `delete_selection::apply`, everything else still
  goes through `command.dispatch(doc, cfg)`.
- **Canvas window** (`🎭️modes/✏️edit/🪟️windows/🕸️canvas`): `WindowKindDefinition` literal gained
  `interactions: Vec::new()`, populated by `.window_kind_interactions(WIRES_PLAY_WINDOW_CANVAS,
  vec![InteractionRef::new("graph")])` in the manifest.
- **Document panel** (`📌️panels/📄️artifact`): row ids switched from namespaced
  (`wires-play-document.identity.{id}`) to BARE identity/edge ids (required for the framework's
  post-render presence stamping to match `state.selection`/`.hover` ids against a row's own `id`
  verbatim — mirrors layout's identical fix); `.selected()`/`.selection_change()` deleted, replaced
  with `.interaction_domain("graph")`; per-row click actions now dispatch
  `INTERACTION_SELECT_ACTION_ID` with the correct granularity instead of the deleted `setSelection`.
- **Inspection panel** (`📌️panels/🔍️inspection`): `render` dropped the `selected: &[String]`
  parameter and the per-selected-node field-editor branch — `ArtifactApp::render` never gained an
  `InteractionView` parameter (confirmed against the trait definition), so this panel always falls
  through to the document-wide summary now, matching the exact same accepted gap layout's/gis2d's/
  puzzle3d's inspection panels already carry (documented inline with a doc comment, not silently
  dropped).
- **Manifest**: `.interaction(InteractionDefinition{…})` + `.window_kind_interactions(…)` added;
  `.view_action("setSelection"/"documentSelect", …)` removed.
- **Tests**: extended in-file `mod tests` only (no new test files). Added: domain-declaration test,
  `wires_select_action_args` shape test, canvas-pointer-down effect tests (hit + empty-space),
  delete-selection handle-vs-apply tests (mirrors space's `INTERACTION_SELECT_ACTION_ID` +
  `handle_action` round trip). Fixed the binary-ordinal shift in
  `commands_keep_their_pre_migration_wire_bytes` (`CanvasPointerUp`/`SetLocale` ordinals shifted
  from 10/11 to 8/9 after the 2 row deletions — real consequence of this migration, not a
  pre-existing issue). Also fixed 3 pre-existing (unrelated) missing-`use` compile errors that only
  surface under `cargo test` (not bare `cargo check`): `canvas-pointer-down`'s test referenced
  `canvas_pointer_move`/`canvas_pointer_up` without importing them, `force-layout`'s test
  referenced `reorganize` without importing it.

## Pre-existing, unrelated, NOT fixed (left exactly as found)

1. **`E0432` unresolved import `WIRES_PLAY_EXAMPLE_METABOLISM_ID`** in
   `🎛️apps/🔌️wires/🦀️component.rs:18` — `pub const WIRES_PLAY_EXAMPLE_METABOLISM_ID` lives inside
   the `set_active_example` submodule and was never re-exported at the `commands` module level; a
   drift from the "Semantic Command Names" refactor (commit `865fa1cc5b`, 2026-08-13 22:29, `git log
   --date=iso`-confirmed to predate this ticket's work in this crate). Genuinely unrelated to
   hover/selection — reported per this task's explicit instruction, not fixed. **This is the ONLY
   thing blocking `cargo check`/`cargo test` from reaching green for this crate right now** — see
   the scratch-verification note below for proof the interaction migration itself is correct.
2. **`set_node_root_round_trip` test failure** (content-addressed child-handle hash mismatch in the
   pre-existing set-node-root mutation's inverse law) — file last touched commit `1cf6018596`
   (2026-08-13 15:56, `git log --date=iso`-confirmed), deterministic across reruns, zero code path
   through anything this migration touched. Not fixed — out of scope.

## Verification

A scratch, NOT-committed one-line import fix was applied temporarily to prove the interaction
migration compiles/passes cleanly despite pre-existing issue (1) above, then reverted — see
`w6-reasoning-mindmap-scratch-verification-with-preexisting-import-patched.txt` for the exact
patch and results (**0 compile errors; 80 passed, 1 failed — only pre-existing issue (2)**). The
real, final, unpatched acceptance command output (issue (1) restored, exactly as found) is in
`w6-reasoning-mindmap-final-cargo-check.txt` / `w6-reasoning-mindmap-final-cargo-test.txt`.

**Can this crate reach green without fixing the pre-existing bugs?** No — `cargo check`/`cargo
test` both currently fail to compile because of pre-existing issue (1), which is outside this
migration's scope per the task brief's explicit instruction to report rather than fix it. Every
hover/selection-specific change in this crate is verified correct (scratch-patched run: 80/81
in-crate tests passing, the single remaining failure being pre-existing issue (2), also
unrelated).

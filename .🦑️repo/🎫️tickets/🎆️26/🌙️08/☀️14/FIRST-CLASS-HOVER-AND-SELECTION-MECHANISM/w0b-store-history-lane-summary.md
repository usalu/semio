# W0b — HistoryLane mechanism in the store

## File touched (primary)
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` (crate `semio-framework-os-kernel`, module `os_store`)

## Fallout fixes (required for compilation, not a design choice)
Adding a field to `ArtifactEnvelope` (defined in the file above) forced two struct-literal
construction sites in `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` to add
`lanes: std::collections::BTreeMap::new()` — the sole edits outside the primary file. Verified via
repo-wide grep that no other `ArtifactEnvelope { .. }` construction or destructuring exists
(`🧰️framework/🛍️products/💻️os/🦀️component.rs` also constructs one but is not mounted by
`glue.rs` — dead/unused file, left untouched).

## Design decision: why no field was added to `Edit<Mutation>`
`Edit<Mutation>` and `MutationMeta` are defined in
`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`, a different file the
task scoped me out of. Adding a lane there would also be forbidden because `ArtifactCommand::Apply`/
`AmendLast` are constructed by struct literal in ~10 other files repo-wide (flow, space, db-engine,
run/bin, infinite/dag, os-host, sync tests, …) — adding a required field to those variants would
break every one of them, far outside a single sequential W0 task's blast radius.

Instead: `HistoryLane` (enum `{ Document, Interaction }`, `Document` is `#[default]`) lives beside
`ArtifactCursor`, and a sparse ledger `ArtifactEnvelope.lanes: BTreeMap<String, HistoryLane>` maps
`Edit.id → HistoryLane` **only for non-`Document` entries** (an ordinary document edit never gets a
map entry — absence means `Document`). This is fully additive: existing persisted envelopes decode
with an empty map, i.e. every edit reads back as `Document`, matching prior undo/redo behavior
exactly.

To avoid a required-field break, lane tagging rides **new, purely additive `ArtifactCommand`
variants** rather than new fields on `Apply`/`AmendLast`:

```rust
pub enum HistoryLane { Document, Interaction }   // #[default] Document, serde camelCase

pub enum ArtifactCommand<Mutation> {
    // ...unchanged existing variants (Apply/AmendLast/Undo/Redo/UndoWithPolicy/…)...
    ApplyInLane { mutations: Vec<Mutation>, description: Option<String>, lane: HistoryLane },
    AmendLastInLane { mutations: Vec<Mutation>, coalesce_key: Option<String>, lane: HistoryLane },
    UndoInLane { lane: HistoryLane },
    RedoInLane { lane: HistoryLane },
}
```

All four are wired through the full existing `ArtifactCommand` codec surface (text `print_command`/
`parse_command` via new `CommandHeaderLine` variants `apply-in-lane`/`amend-in-lane`/`undo-in-lane`/
`redo-in-lane`, and binary `OpBinary` ordinals 11–14), since that surface already exhaustively
matches every variant in this same file.

## Undo/redo semantics
- Default `Undo`/`Redo` (and `UndoWithPolicy { ExactBaseOnly | TransformAgainstConcurrent }`) now
  search `applied_edit_ids`/`redo_edit_ids` **from the tail** for the nearest entry whose lane is
  `Document`, skipping — never removing — any non-`Document` entries in between. Those entries stay
  applied/persisted throughout. If none match, `NothingToUndo`/`NothingToRedo` (so an all-interaction
  history is a correct no-op for default undo).
- `UndoInLane { lane }` / `RedoInLane { lane }` run the identical search filtered on an arbitrary
  caller-chosen lane, so any lane (not just `Interaction`) can be walked explicitly and
  independently of the `Document` cursor's own position.
- Shared plumbing: `ArtifactStore::edit_lane(&self, id) -> HistoryLane` (private; envelope's `lanes`
  field is `pub` so external callers can read it directly via `store.envelope().lanes`),
  `undo_lane_position`/`redo_lane_position` (position-based removal + fold, generalizing the
  pre-existing `TransformAgainstConcurrent` "remove from anywhere, not just the tail" pattern), and
  `apply_command`/`amend_command` (the refactored bodies `Apply`/`AmendLast`/`ApplyInLane`/
  `AmendLastInLane` all share).

## Known gap (flagged for a later wave, not fixed here — out of this task's file scope)
`envelope.lanes` round-trips through `ArtifactStore::envelope_json` (plain serde JSON — tested), but
**not** through the `.pack`+`.spr` binary/text persistence path (`print_document_spr`/
`parse_document_spr`, `print_document_text`/`parse_document_text`) — that codec is
`crate::os_spr::HistoryLog`/`HistoryEdit` in a different file (`📡️spr/📜️history/🦀️component.rs`),
same file-scope reasoning as above (mirrors the existing `composition_pins`/`owner` deferral already
documented at `parse_document_spr`'s `dialect`/`migrated_from`/`owner` fields). A `.pack`+`.spr`
reload today loses non-`Document` lane tags. Whichever wave wires the real framework-owned
persisted-local `InteractionState` through actual browser reload (W3, per `📋️master.md`) needs to
extend `HistoryLog`/`HistoryEdit`/`HistoryComposition` with a lane overlay analogous to how
`composition_pins`/`owner` were added there. Flagged inline in `parse_document_spr`'s doc comment too.

## Tests added (extended the existing `mod tests` in the same file, region `🔖️HistoryLaneTests`)
1. `history_lane_defaults_to_document`
2. `history_lane_default_undo_and_redo_skip_interaction_entries` — interleaves Document/Interaction
   edits, asserts default undo/redo only ever move Document-lane entries, in both directions, even
   with interaction edits on both sides of the target.
3. `history_lane_undo_in_lane_and_redo_in_lane_walk_only_the_requested_lane` — proves the explicit
   lane-scoped API moves independently of the Document lane's own cursor.
4. `history_lane_default_undo_is_a_no_op_when_every_edit_is_interaction_lane` — all-interaction
   history + default `Undo` → `NothingToUndo`, then `UndoInLane` still reaches it.
5. `history_lane_interaction_entries_survive_envelope_json_round_trip` — envelope_json round trip
   preserves the lane tag; a store rebuilt from the reloaded envelope still lane-skips on undo.

Also extended `command_text_binary_equivalence_holds_for_every_document_command_variant` with the
four new `ArtifactCommand` variants (text+binary codec round trip).

## Acceptance
`cd /Users/ueli/Documents/semio && cargo test -p semio-framework-os-kernel store 2>&1 | tail -40`
→ **111 passed; 0 failed** (was 106 before this change — 5 new tests). Full output saved to
`w0b-store-history-lane-test-output.txt` in this ticket folder.

Also spot-checked (not part of the formal acceptance command, but relevant since this task touched
one file outside `store/component.rs`): `cargo check -p semio-framework-os-kernel --all-targets`,
`cargo check -p semio-framework-plugin`, `cargo check -p semio-framework-os` all green (warnings
only, pre-existing and unrelated to this change).

## Public API surface for later waves (W3 plugin SDK, W4 per-app)
- `HistoryLane` enum (`Document` default, `Interaction`) — extensible, add more variants later
  without touching undo/redo logic (the search predicates are generic over `HistoryLane`).
- `ArtifactCommand::{ApplyInLane, AmendLastInLane, UndoInLane, RedoInLane}` — the dispatchable
  surface. A plugin/wrapper wanting framework-owned persisted-local `InteractionState` writes it via
  `store.dispatch(ArtifactCommand::ApplyInLane { mutations, description, lane: HistoryLane::Interaction })`.
- `ArtifactEnvelope.lanes: BTreeMap<String, HistoryLane>` — `pub`, sparse, readable directly
  (`store.envelope().lanes.get(edit_id)`, `None` ⇒ `Document`).
- Plain `Undo`/`Redo`/`Apply`/`AmendLast` are UNCHANGED in signature and, for an all-`Document`
  history (every app before W4 migrates), in behavior — this is a strictly additive mechanism.

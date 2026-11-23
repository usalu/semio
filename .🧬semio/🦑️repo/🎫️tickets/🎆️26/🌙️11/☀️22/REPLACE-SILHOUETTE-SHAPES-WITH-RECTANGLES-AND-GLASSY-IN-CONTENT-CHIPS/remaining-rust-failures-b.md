# 🧯 Remaining Rust Failures B: shell-tests-input-1

Crate `semio-renderer-wgpu-shell-tests-input-1`. Start: 24 passed, 49 failed. Last verified run: 27 passed, 46 failed. Not green.

## Root causes found
1. `StaleGeneration` (46 tests): since commit 749 `Ui::begin_document` refuses a document unless the turn generation equals the lease header generation. The pointer fixtures (`paint_tree_pointer_documents`, `paint_component_pointer_documents`) ran every document on `FixtureTurnV1::new`, a fixed Generation(7), while their leases carry generation 1.
2. Past that, a one-unit, one-item turn cannot hold a paint turn: work loan overspend, then layout session birth spinning forever on `Layout.Mount` (retained grant of one item).
3. Fixture close never finished because the engine windows were never registered with `own_surface`, and the one-unit close turn could not retire them.

## Fixes made
- `FixtureTurnV1::for_generation(grant, generation)` added.
- Shell fixture paints keep one caller per document generation (`fixture_turn_for_document`), retire and finish the previous one on a switch, quote `POINTER_DOCUMENT_TURN_WORK` fuel per paint turn, use a wide retained grant, and call `own_surface`.
- `FixtureTurnV1::close` quotes `CLOSE_TURN_WORK` fuel per close turn.
- Temporary `[DEBUG]` edits in session-admission, the document consumer and the fixture were removed again.

## Still failing (46), by class
- 32 assertions inside the tests (accessible address of the published pane, settled camera bursts, canvas owner dispatch, `Option::unwrap` on a missing pane): documents now ingest and paint, so these are behavioral and need per-test triage; not analysed.
- 9 `FixtureTurnV1::close` never reaches `ui_document_close_pending_for == false` within 65536 turns.
- 4 `UI value arena requires its actual registered Source` (window-publication journals tests build shells without a `UiArenaFixture`) and 1 `ordered-map root must be explicitly retired before drop`.
- Other agents were editing `job/context`, `testing/caller` (with `[DEBUG] service heap`), `native-decoding` and the store engine during the runs; several runs were lost to their transient compile errors.

## Files changed
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎟️original-turn/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧰️fixtures/🐚️shell/🦀️.rs`

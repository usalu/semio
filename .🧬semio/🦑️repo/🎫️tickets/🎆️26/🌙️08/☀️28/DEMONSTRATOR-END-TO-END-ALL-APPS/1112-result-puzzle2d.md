# 1112 puzzle2d (11/12)
## Step 1
- Ported `puzzle-2d/schema/snapshot/🔎️lookup` (Puzzle2dLookupCursor) to the reply/effect protocol: `advance(source,target,effect)`, `close_step(effect)->OriginalRetirementPoll`, `close_demands(body)`; helpers `pay`, `settled_step`, `replied`. Linear scan kept (the snapshot rows are lists, not an ordered map); comparator = PagedUtf8BoundedOrdCursor via `compare_into`/`begin_close_into`, bindings closed with `RetainedCloneBinding::close_one(.., body, effect)`.
- W-puzzle2d-2 wasm `-p semio-s-artifact-puzzle-2d-snapshot` rc 0.
## Step 2 (prepare family, stage A)
- Shared helpers (records crate, `preparation::native_preparation_child`): `pay` (one local event charged through the effect), `settled_step` (Pending = zero progress, retried), `replied` (runs one non-suspending reply-shaped callee). Re-exported from the snapshot `🔎️lookup`, which also owns `Puzzle2dIdentifierComparison` (owned PagedUtf8 comparison; closes its bindings lazily under the effect).
- Child trait `Puzzle2dNativePreparation::close_granted(grant)` -> `close_step(effect)->OriginalRetirementPoll`; `Puzzle2dPreparationChild::ensure(effect)` births the child inline (no Box, no heap allocation).
- Prepare cursors ported (advance(.., effect), `close_step(effect)->OriginalRetirementPoll`, each local event paid with `pay`): text, flag, create-node, move-node, connect-handles, delete-node. Work is charged on the effect; denied events keep state.
- W-puzzle2d-5 hub wasm `-p semio-s-artifact-puzzle-2d-records -p semio-s-artifact-puzzle-2d-snapshot` rc 0.
- Found: the prepare family lives in crate `semio-s-artifact-puzzle-2d-mutations` (mirror include), together with the candidate / inverse / edge / literal-id-inverse cursors (22 files) that clone whole snapshots through RetainedFieldCursor / PagedListEditCursor / ControlledRetirement::step(grant). Those are not in the coordinator ruling (W-puzzle2d-3: 179 errors in the mutations crate).
## Step 3 (candidate / inverse family, stage B)
- Shared adapters in `native_preparation_child` (records crate): `cursor_advance/take/taken/begin_close/close/retire` (native clone cursors under the effect, non-suspending replies, denial = zero progress), `controlled_step`, `edit_step/edit_progress/edit_fault` (PagedListEditCursor poll), `list_reserve` (page reservation), plus `replied/replied_value/pay/settled_step`.
- Ported with them: delete-node candidate+inverse, create-node candidate, connect-handles edge assembly, literal-id inverse (and mechanically text/flag/move-node/connect candidate+inverse). Candidate/inverse cursors' `advance(.., effect)`, `close_step(effect)->OriginalRetirementPoll`.
- W-puzzle2d-9 hub wasm `-p semio-s-artifact-puzzle-2d-mutations` rc 0.

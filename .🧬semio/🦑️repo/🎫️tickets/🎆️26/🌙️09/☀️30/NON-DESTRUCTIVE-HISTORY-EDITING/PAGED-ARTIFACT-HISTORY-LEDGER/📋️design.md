# Paged Artifact History Ledger

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING/PAGED-ARTIFACT-HISTORY-LEDGER`. Goal `🎯runningframework`.

## Failure

`ArtifactHistoryLedger` is one `Vec` of 64 slots (`ARTIFACT_HISTORY_LEDGER_CAPACITY`). `reserve_slot` returns `Capacity` once `slots.len() == 64`. The store maps that to `edit history ledger is saturated`. The same constant caps `applied_edit_ids`, `redo_edit_ids`, `CursorRevisionAccumulator::{applied,redo}`, and the change, checkpoint, and alternative ledgers. A 65th edit never compacts and never opens another page. Config and window-config lanes are `ArtifactStore` (`ConfigStore` is that alias), so they die on the same line.

## Constraints

- Guest `dlmalloc` does not shrink wasm memory. Growing one `Vec` reallocates a larger buffer and abandons the old one. Page by allocating a fixed block once and keeping it.
- One contiguous request stays under 64 KiB. A page is 64 slots. An `Edit` slot is a few hundred bytes, so one page is well under 16 KiB. The directory chunk is 512 page pointers (4 KiB), linked, never reallocated.
- Owner catalogs stay fixed-capacity per page. `HistoryPageStack` pre-admits one page and opens the next only when the current page is full.
- Displaced owners still retire through the existing retirement queue. Slot generations still reject ABA. Empty pages stay allocated at the high-water mark and are reused through the free list; they are not returned and re-grown.
- `CursorRevisionAccumulator` records and the applied/redo id catalogs use the same page stack, so batch publication's `len == capacity` preflight is no longer a 64-edit ceiling. Publication admits another edit while a further page can be opened (`admits_one`).
- Batch footprints (`work_items`, retained bytes) are unchanged. They count gesture rows, not ledger slots.

## Shape

- `ARTIFACT_HISTORY_PAGE_SLOTS = 64`. `ARTIFACT_HISTORY_LEDGER_CAPACITY` is that page size.
- Logical order stays the linked list. A key index is a `u32` flat slot (`page * 64 + slot`).
- `vacancies` counts free tombstones, unused slots in allocated pages, and one not-yet-opened page. A full page is not saturation.
- `try_from_preflighted` admits any length the `u32` page index can name. Reload of a history longer than 64 edits uses the same path.
- Address-space exhaustion (`u32` page index) is the only remaining capacity fault.

## Oracle

`🌿️vcs/🧪️tests/🔬️paged-ledger/📜️oracle.py` is a Python list. The Rust ledger's order after 80 pushes, a removal at logical index 17, and a tail replacement must equal that list. A store test applies 65 `SetN` edits and checks the snapshot.

The demo store close loop is a hang detector, not a session ceiling. One page of edits fit inside 4096 one-item steps. A second page of edits, inverse mutations, and id catalogs needs more of those steps, so the detector is 16384. Retirement still drains one owner per grant.

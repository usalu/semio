# Close Birth Ledger

## Finding

The 62.9 MB released per app close were never recorded as births on any ledger. Measured at every app close: caller capacity 20,816 B at close start, receiving-child ledger 0, constructor receipts a few hundred bytes. The eager backings come from `MutationDagFixedSlots::new()` (via `MutationDag::new()`/`Default`), allocated with no caller context and so unmetered. The close then charged their whole release to the one acceptance caller (8.5 closes x 62.9 MB = 538 MB > 512 MiB). The earlier assumption "births were charged to the receiving-child ledger" was wrong: they were charged nowhere, so a symmetric law needs the birth recorded first.

## Law (framework, retirement ladder)

A release is charged to the ledger that recorded the birth of the freed backing.

- `semio_framework_value::birth_ledger` (`🌱️value/🧾️birth-ledger/🦀️.rs`): `OriginalBirthLedger` (`record`, `settle`, `outstanding`; a settle never exceeds what was born) plus the per-thread ledger `record_original_birth` / `settle_original_birth` / `original_birth_ledger`.
- Birth side: `MutationDagFixedSlots::new()` records its exact backing bytes (`backing_bytes()`).
- Settle side: `StepContext::settle_original_birth(declared, before)` removes the settled bytes from the caller receipt and from the Source ledger (`OriginalReturnBudget::refund_released`).
- Ladder: `original_lifecycle_step` and `original_lifecycle_effect_step_at` (semio-plugin-ownership) call `settle_births` after posting, and return the step with the settled bytes removed from its release, so nested wrappers stay consistent. Every close, maintenance and constructor ladder inherits the law.
- Fixture stock untouched.

## Law test

`🧵️job/⏱️context/🧪️tests/📊️native-progress/close-birth-ledger.rs` with `🧫️fixtures/close-birth-ledger.json` and `.schema.json`, TS twin in `🟦️.ts` (ajv schema plus SQLite window-function cross-check of the same formula). Rust passes (job crate, `--features testkit`, all targets green). The TS twin was not executed (no bun on this machine); its SQL was run with the sqlite3 CLI and the fixture checked in Python against the Rust formula.

## Results

- Per app close release: 62.9 MB -> 25.5 MB (about 37 MB of eager DAG backing settled to its births).
- `history_edits_end_to_end`: passes alone and inside the full smoke target (17 passed, 1 failed; `conflict_history_edits_end_to_end` passes too).
- Job crate: green.
- `bearbeiten_pane_loads_applies_undoes_and_closes` still fails with host lifetime fuel (Goal 2, unrelated to this law).

## DAG pages eager vs lazy (note, not fixed)

`MutationDag::default()` allocates 3 structures x 128 pages eagerly (about 4.85 MB per store, 9 stores per app including children). A lazy/metered path already exists (`MutationDag::empty()` plus `admit_backing_step`, used by replay initialization). Switching the store constructors to it would remove the unmetered birth altogether, so the ledger recording would no longer be needed for the DAG. Not done. Remaining unsettled release per close (about 25 MB) are other eager backings (fixed registries/rings, lease backings) that do not record births yet; they are charged to the caller as before.

## Other changes

- Removed the `post_retained` release-trace block from `⏱️context/🦀️.rs` (coordinator request).
- Repaired `🎟️caller/🦀️.rs` of the job testing caller, whose tail of `turn_original` and `fund_retirement`/`lend_context` had been lost by an earlier debug cleanup (restored from HEAD, minus the `[DEBUG]` print in `fixture_remaining`).
- All my `[DEBUG]` prints are removed (artifact-app-laws construction file).
- Builds ran in a clone (`🗑️generated/native-b`) because two cargo processes on the shared replay dirs deadlocked on locks; scripts `birth-cargo.sh`, `birth-seal.sh`.

## Files changed

- `🧰️framework/🔨️modules/🌱️value/🧾️birth-ledger/🦀️.rs` (new)
- `🧰️framework/🔨️modules/🌱️value/🦀️.rs`
- `🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/📬️outcome/🪙️budget/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/⚠️failure/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs` (debug added and removed, net no change)
- `🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️tests/📊️native-progress/close-birth-ledger.rs` (new), `🦀️.rs`, `🟦️.ts`, `🧫️fixtures/close-birth-ledger.json` (new), `🧫️fixtures/close-birth-ledger.schema.json` (new)
- ticket: `birth-cargo.sh`, `birth-seal.sh`, `close-birth-ledger.md`

## Follow-up (coordinator items)

1. Lazy DAG pages: already in the tree from another agent before I started on it. `MutationDagFixedSlots::new()` now creates an all-`None` page table, `slot_mut` allocates a page on first use and records that page's bytes on the birth ledger, and the metered quote (`next_push_page_bytes`, `admit_next_push_page`) exists. I did not duplicate it. With it, the per-close release of the DAG fell to about 1 MB per store (metadata arrays only). `cargo check --tests` of the editor crate with everything in place is green.
2. Births recorded for the remaining eager backings (each one records the exact bytes it allocates):
   - edit message ledger slot pages and index arrays (`📒️ledger/📨️messages`; this was the 136 x 3072 B per store)
   - `prefix_ring` of both store constructors
   - displaced-retirement queue (`🔗️read/🗑️envelope-genesis-and-displaced-snapshot-retirements`)
   - `ArtifactFixedRegistry::with_slots` page table and `ArtifactFixedQueue::new`
   Measured before these edits (net business release by frontier, per app close): about 12 MB, of which OwnedStores about 1 MB per store, FinalBackings 1.27 MB, ClosingChildMember 5.9 MB. Not yet re-measured after them. Not covered: registry pages born on use, child member store backings (another agent is reworking `ChildMemberRegistry` into releasable pages), snapshot lease registry. Where an owner already has a metered `admit_*` path its birth is charged to the admitting ledger and deliberately not recorded here, so the free stays symmetric.
3. TS twin run with the repo bun (`bun test ./🟦️.ts` in the native-progress folder): 7 pass, 0 fail.
4. The `native-b` clone is deleted. Its build deps (and those of the shared `native` build dirs) had been wiped by something else mid-session, so I built the final checks in a scratch dir, now deleted too.

## Verification state (honest)

- Green: job crate with `testkit` (run before items 1 and 2), TS twin, and `cargo check --tests` of the process3d editor with all edits.
- Not proven green after items 1 and 2: the editor smoke. The last full run of `history_edits_end_to_end` after my edits fails inside the archive load with `native receiving original type, field or phase is invalid` (decoder phase refusal, no accounting involved), while other agents have uncommitted work in the decode/receiving, loan/frame and child-registry crates. I could not separate it from their edits within the shared tree, and I did not re-run the job crate after items 1 and 2. Before items 1 and 2 (and before their changes) the same test and the whole smoke target passed except the known Goal 2 test.
- The temporary debug prints of this follow-up are removed.

## Cleanup

Large test binaries and builds are deleted; only the scripts `birth-cargo.sh`, `birth-seal.sh`, `run-cargo.sh`, `retry-run.sh` remain in the ticket folder.

## Additional files changed in the follow-up

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/📨️messages/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/📥️lifted/🏪️store/🗒️draft-config-store-and-checkpoint-identity/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/📥️lifted/🏪️store/🌱️runtime-seed-construction-and-owned-close/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🗑️envelope-genesis-and-displaced-snapshot-retirements/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/🦀️.rs`

# Snapshot read issuer exhaustion: "snapshot read issuer has no admitted vacant slot"

Status block is at the end (what ran green, what could not run, what is unproven). Scripts: `vs-law.sh` here; the private run scripts were `/c/t/vs-*.sh` (copied intent below). Private dirs `C:\t\vs`, `C:\t\vstmp` are deleted at the end.

## What the error is
`SnapshotReadRegistryHandle::try_issue_original` (`🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🔒️registry/🦀️.rs`) refuses with `OwnershipLimit` when `free_len == 0` or the slot pages are not resident. Every typed read (`ArtifactStore::snapshot_read`, tree refresh, progress refresh, export capture, render, solve jobs) goes through it, and the vcs wrapper turns the refusal into `module.vcs: validation failed: ...`. Each Store owns its own table of `SNAPSHOT_READ_LEASE_CAPACITY = 1_024` slots (8 pages of 128). The table is not undersized: the legitimate concurrent holders are a render job, three live-query reads, one typed command (document, draft, 2 window-config duplicates) and the tool operations, i.e. tens, never hundreds. So 1,024 occupied slots always mean slots that nobody will vacate.

## Slot lifecycle (before this change)
1. Admit: `try_issue_original` takes `free[free_read]`, writes the slot, sets the occupancy bit, returns a bare `SnapshotReadLease`.
2. The lease is wrapped in `SnapshotRead` / `ErasedSnapshotRead`, whose `Drop` calls `return_snapshot_read` -> `SnapshotReadLease::return_now`.
3. `return_now` vacates at once only when the root is still aliased (`try_release_aliased`); otherwise it marks the slot returned (bit 63 of the generation word, `returned += 1`) and the slot waits for the maintenance pump (`try_admit_one_returned`, ladder stage 8 of 27, one read per visit).
4. The pump hands the root to a paid retirement and vacates the slot.

## Where it could leak (found by reading and by laws)
1. **A bare `SnapshotReadLease` had no `Drop`.** Any path that issued a lease and left before wrapping it (a refusal, a `?`, a cancelled caller, a moved-away tuple) dropped it silently: the slot stayed occupied and nothing could ever vacate it. The vacancy depended on every caller wrapping the lease. The new law reproduces it: with the old behaviour `every_terminal_of_a_read_vacates_its_slot_exactly_once` fails at the `bare-lease` terminal (verified with a temporary switch before it was removed), with the fix it passes.
2. **A returned-read admission that consumed its original and then failed stranded the slot.** In `try_admit_one_returned` the slot value is moved out (`assume_init_read`) before the admission runs. On `(Err, None)` and `(Ok(None), None)` the function returned the error with the occupancy bit still set, the slot still counted as returned and its memory logically moved out. The slot was never vacated, and the next pump visit read a moved-out `Arc` (`assume_init_ref` on a moved value). This is the "fault" terminal, and it is the one that can make the pump itself fail on every revisit.
3. **Pump starvation by order and by rate.** The ladder visits fixed units first (command prune, private groups, direct ingress, granted mounted, tool overlay, tool runs, time travel, the three displaced-owner pressure units) and only then the 27 fair stages; a unit that reports progress ends the turn. The returned-read pump (stage 8) therefore only runs when everything before it is idle and then drains one read per visit. Nothing raised its priority as returned reads piled up, unlike the displaced-owner queues (`ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY`).
4. The refusal text did not say whether the table was full of live reads or of returned reads nobody pumped, so the browser matrix could not tell the two apart.

## What the browser evidence says
In `🗑️generated/matrix-v3/3d/results.jsonl` the slot refusal starts at `3d.res.resolve-after-edit` (record 180, about 180 interactions in). The record before it (`3d.res.source-live`, 160) already shows `plugin.internal.prior-outcome: runtime live cleanup faulted for instance 1`, which the reactor raises after `RUNTIME_LIVE_PRIOR_OUTCOME_LIMIT = 8` consecutive faulted or cancelled maintenance jobs (`⚛️reactor/🏃️runtime/📥️staging/♻️reactor-lifecycle/🧹️cleanup-vocabulary`). The record before that (`3d.act.setActiveExample`, 159) took 151 s for the House example ("no effect before timeout") with 108 console noise lines. Example switches (`199`, `200`) still pass after the wedge because a store replacement brings a fresh registry, so the exhausted table is the document Store's. The consistent reading is: the maintenance ladder was failing or starved for a long stretch, returned reads piled up unpumped, and the table filled. Failure 2 above is a concrete way for the pump to fail every time it revisits a slot. I could not capture the first faulting job from the browser (the wasm cannot be rebuilt by me and the console excerpt is truncated), so this causal chain is a hypothesis; items 1 to 3 are proven defects independent of it.

## What did not reproduce natively
Mounted native FEM sessions did not leak in any store I could count (document, config, draft, interaction): gumball stream/commit/abort, gumball flags, playback ticks, edits, undo/redo, renders, example switches, and superseded results solves all ended with 0 returned and 0 outstanding reads. Both new mounted laws also pass with the lease guard switched off, so the ordinary native paths never dropped a bare lease. The window-config partitions, the transient stores and the presence local registry were not counted. The browser differs by long idle waits, worker-hosted solves and the 151 s House load, none of which the native harness reproduces.

## Fix
Value layer (`🔨️modules/🌱️value/📸️snapshot/👁️read/🦀️.rs`, the registry file it includes):
- `SnapshotReadLease` implements `Drop` and returns itself. It is now a linear token: complete, cancel, refuse, fault (unwind) and supersede all end in the same return, whoever holds it. Exactly-once rests on the generation word: a second return, or a drop after the slot was vacated and reused, sees a different generation and does nothing. Holders that hand the registry on after a settled return use the private `disarm`.
- `try_admit_one_returned` vacates a consumed slot on every error exit through one helper, `vacate_returned_locked` (occupancy bit, free list, returned count and generation word move together; the success path uses the same helper).
- `SNAPSHOT_READ_RETURN_PRESSURE = capacity / 8` and `returned_under_pressure()` on the registry, the owner, `StoreOwners` and `ArtifactStore`.
- The refusal now names its cause: pages released, returned reads awaiting their pump, or every slot held by a live read.

Plugin ladder (`🔌️plugin/🪜️maintenance-ladder`, `🧵️instance/📥️staging/🪜️maintenance-ladder`, `📄️docs/📥️staging/🚪️ingress`):
- Position 0 of the ladder is a new unit, the returned-read pump, present only while a document, config, draft or interaction Store has at least an eighth of its table returned. It runs ahead of every fixed unit and the fair rotation, so unrelated perpetual work can no longer starve the issuers. Idle turns are not charged fuel for it. It is the same stage 8 work, only prioritised (`MaintenanceUnit::Pressure(8)`, funded like the archive dependency stages).

## Laws
Language-agnostic vector: `🏪️store/🧪️tests/🧫️fixtures/snapshot-read-terminal-vacancy-v1/🔣️.json` (capacity, multiples, pump interval, terminals, expected empty state). Rust law file: `🏪️store/🧪️tests/🏙️bin-01/🔬️unit/🎟️terminal-vacancy-of-snapshot-read-slots-laws/🦀️.rs` (bin-01).
- `every_terminal_of_a_read_vacates_its_slot_exactly_once`: 3 x capacity + 6 cycles, each ending in complete, cancel, refuse (busy issuer), fault (panic unwound), supersede or a bare dropped lease, half of them aliased, the pump running every 64 cycles. Afterwards: 0 occupied, 0 returned, `terminal_is_empty`, 1,024 further reads admit and the 1,025th is refused. The std `Vec`/`Arc` bookkeeping in the test is the independent oracle for the counts (the fixture holds the expected numbers).
- `a_lease_is_returned_exactly_once`: second `return_now` refused, stale lease drop never vacates the reused slot.
- `returned_reads_raise_pressure_at_an_eighth_of_the_table_and_the_pump_clears_it`.
- `a_faulted_returned_admission_that_consumed_its_original_still_vacates_the_slot` (both the faulting and the consumed-without-publishing variants).
Mounted FEM laws (3D and 2D twins):
- 3D `history_undo_and_alternatives::a_long_mixed_session_never_exhausts_the_snapshot_read_issuers_3d` (gumball streams, commits and aborts, flags, playback, edits, undo/redo, renders, example switches).
- 3D `results_window::a_long_supersede_session_never_exhausts_the_snapshot_read_issuer_3d` and 2D `results_settle::a_long_supersede_session_never_exhausts_the_snapshot_read_issuers_2d` (edits and undos superseding the solve in flight, jobs spawned and cancelled).
Each asserts the peak of held reads per Store stays below the pressure mark, a settled session holds at most 2, and a fresh `snapshot_read()` is admitted at the end. They are bounded (16 to 40 rounds, a few minutes) because a native round costs seconds; they guard the regression rather than prove the browser cause.

## Files
Product: `🔨️modules/🌱️value/📸️snapshot/👁️read/🦀️.rs`, `🏪️store/🔗️read/🔒️registry/🦀️.rs`, `🏪️store/⚙️engine/🧱️store-components/📥️lifted/🏪️store/🏗️store-owners-retained-reads-and-retirements/🦀️.rs`, `🏪️store/⚙️engine/📥️lifted/🏪️store/🪞️delegating-accessors-and-reads/🦀️.rs`, `🔌️plugin/🪜️maintenance-ladder/🦀️.rs`, `🔌️plugin/🧵️instance/📥️staging/🪜️maintenance-ladder/🦀️.rs`, `🔌️plugin/🧵️instance/📄️docs/📥️staging/🚪️ingress/🦀️.rs`.
Tests: bin-01 `🔬️unit/🦀️.rs` (include), the new law file and fixture above, 3D `🕰️history-undo-and-alternatives/🦀️.rs`, 3D results window `🧪️tests/🔬️unit/🦀️.rs`, 2D `🔁️results-settle/🦀️.rs`.

## Status
See the final block below.

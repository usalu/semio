# P1q R4 Terminal Sweep Remediation

Date: 2026-08-24  
Executor: Sol High  
Input audit: `📓️terra-post-sol-p1q-b1-b6-r4-source-reaudit-2026-08-24.md`  
Status: source-audit-ready

## Outcome

The three latest R4 blockers and their semantic equivalents are repaired without changing the accepted core facade, `Lane::Io`, `ArtifactId`, or `LeaseInfo` boundaries. Artifact staging refusal now transfers its exact refusal/source/staged owners into a fixed mounted cursor. CLI WAL, snapshot, replay, verification, and migration terminal owners now close through typed futures that perform one owner opportunity per poll and publish an exact exit/opportunity witness. Snapshot consolidation now retains at most 64 fixed, page-credit-witnessed `db_state::Page` owners, publishes from that fixed source, and closes incrementally through a mounted future and fixed retirement ring.

No P6h FEM, P5 external-caller propagation, stdio, oracle, renderer, or unrelated peer region was intentionally changed.

## Exact Files

- `📜️script.ts` — exact `interactivityP1qR4Failures`/self-test region only.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️artifact/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📸️snapshot/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔢️index/🦀️component.rs`
- This report.

## Retained Owner Census

| Surface | Fixed authority | One opportunity | Interrupted/refusal handback |
| --- | --- | --- | --- |
| Artifact staged admission | `ArtifactStateRetirementCursor`: one `StateEntryRejected`, `[Option<StateEntry>; 64]`, phase and slot cursors | one writer page, refused source, staged slot/page, or key per `artifact_state_retirement_maintenance_step` grant | 64-slot `ARTIFACT_STATE_RETIREMENT`; saturated candidate is losslessly fail-closed with a permanent saturation witness |
| Artifact ordinary replacement/removal | one single-entry `ArtifactStateRetirementCursor` | same maintenance step | same fixed ring; ordinary owner Drop remounts the cursor |
| CLI record | `MountedWalRecordCommandClose` | one `WalRecord::close_step` per poll | unfinished Drop records `Interrupted`; nested retained owners enter their existing mounted handback |
| CLI batch | `MountedWalBatchCommandClose` over fixed `WalRecordBatch` | one record/page owner per poll; `WalRecordBatch::close_step` no longer scans or clears all 64 records | exact `Closed`/`Fault`/`Interrupted` witness and opportunity count |
| CLI replay | `MountedWalReplayCommandClose` | one `WalReplayCursor::close_owner_step` per poll | cursor and nested page/list owners remain owned until terminal or interrupted Drop |
| CLI snapshot | `MountedSnapshotCommandClose` | one `SnapshotChainCursor::close_step` per poll | exact command exit witness |
| Compaction pages | `CompactionRetainedPages`: `[Option<Page>; 64]`, `[u8; 64]` real backing-page credits, fixed length cursor | one DB page or one page shell per close poll | 64-slot `COMPACTION_PAGE_RETIREMENT`; max+1 page returned to a single-owner retirement cursor; saturated candidate retained fail-closed |
| Snapshot publication | `OptionalSnapshotPages` borrowed fixed source | one indexed page/fragment publication opportunity under existing snapshot control grants | no `Vec<Page>` is constructed; frozen `SnapshotDescriptor::new_pages: Vec<ContentHash>` remains metadata-only |
| Index retained collections | fixed `RunEntries` and `IndexBlobList` tail cursors | one entry/blob page/key/shell per call | no `for` scan or bulk slot clearing inside close |

## Production Sweep Census

- Removed artifact's ungranted `for entry in staged.iter_mut().flatten()` rejection sweep.
- Removed CLI terminal `loop { match record.close_step() ... }` and replay/snapshot/batch close loops from `cmd_wal_inspect`, `cmd_snapshot_inspect`, `cmd_replay`, and `cmd_migrate`.
- Migrated `verify_document` to the same mounted record/replay close authorities.
- Replaced `WalRecordBatch::close_step`, `RunEntries::close_step`, and `IndexBlobList::close_step` internal `for` scans/bulk clears with fixed tail cursors.
- Replaced compaction `Vec<db_state::Page>` and `HashSet` page-owner staging with a fixed array and linear fixed dedup witness.
- Remaining production `loop`/`for` constructs in the authorized sources are data traversal, bounded cursor work, or per-owner work with an explicit control grant; they are not terminal close drains. Test-only exhaustive drains are excluded by `interactivityProductionSource`.
- No production `Vec<db_state::Page>` or `Vec<Page>` remains in the compaction source. Snapshot descriptor hash vectors remain the explicitly frozen metadata schema and do not own page backings.

## Hostile Laws

- `artifact_staging_retirement_success_refusal_cancel_stale_fault_drop_interrupted_close_and_max_plus_one_are_lossless`
- `compaction_fixed_pages_success_refusal_cancel_stale_fault_drop_interrupted_close_and_max_plus_one_return_exact_credit`
- `cli_command_close_success_refusal_cancel_stale_fault_drop_interrupted_and_max_plus_one_have_exact_exit_witnesses`
- Existing accepted async-worker, artifact/lease, WAL yield, query Drop, engine Drop, cancellation, panic, ABA-stale, saturation, and exact ledger-return laws remain present.

The new laws exercise exact refused-owner identity, interruption/remount, fixed MAX/MAX+1 refusal, one-opportunity witness counts, fixed page-credit return, and terminal-empty outcomes. Cancellation, stale generation, and backend fault remain additionally covered by the preserved core B1-B6 hostile laws; the new authorities do not bypass those owners.

## Verifier Predicates and Mutations

The isolated R4 verifier now:

- parses Rust items and inner `loop`/`for` blocks rather than checking only same-line `while` predicates;
- rejects `while ... close_step`, `loop { match ... close_step ... Ok(false) => break }`, and ungranted `for` owner-close sweeps;
- applies the sweep to artifact staging, the four CLI commands, compaction, `WalRecordBatch`, `RunEntries`, and `IndexBlobList`;
- rejects `Vec<db_state::Page>` and `Vec<Page>` in production compaction;
- requires every fixed retirement/publish authority and all three hostile laws.

Hostile self-mutations restore:

1. snapshot whole-chunk materialization;
2. index `while` page drain;
3. WAL segment `while` drain;
4. WAL batch `for` drain;
5. index collection `for` drain;
6. query/engine Drop drains;
7. artifact `loop` close drain;
8. artifact `for` staged close drain;
9. compaction `Vec<db_state::Page>`;
10. CLI replay `loop` close drain;
11. CLI migration `for` close drain;
12. missing compaction law;
13. missing artifact fixed cursor;
14. missing CLI exact witness.

Every mutation is rejected by the focused self-test.

## Validation

- `bun ./📜️script.ts verify interactivity p1q-b1-b6` — PASS: `live-source and hostile mutations clean.`
- Scoped `rustfmt --edition 2021 --config skip_children=true` on the six touched Rust files — PASS (parse and format).
- Scoped `rustfmt --edition 2021 --check --config skip_children=true` on the six touched Rust files — PASS.
- Scoped `git diff --check` on the exact seven source files — PASS.

Cargo, Nx, Wasm, browser, and broad build/runtime gates were not run, as explicitly required while overlapping Rust packets remain active. No claim is made for those deferred gates.

## Ticket Infrastructure

The mandated repository MCP could not initialize in this agent environment (`resources/list` ended during MCP handshake), so `repo://goals`, `ticket_reopen`, and `ticket_close` were unavailable. Work remained inside the existing `INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-ONE-POOL-WORKER-RUNTIME` ticket folder, and no parallel ticket or goal state was created.

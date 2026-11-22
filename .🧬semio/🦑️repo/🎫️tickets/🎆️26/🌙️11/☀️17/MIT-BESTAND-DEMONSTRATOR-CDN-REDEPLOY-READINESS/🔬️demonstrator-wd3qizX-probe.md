# Demonstrator wd3qizX Linked Guest Probe

Command: `bash probe-linked.sh demonstrator wd3qizX 8791 both` against the acceptance boot test (`♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts`), one browser at a time, workers=1. Guest SHA and bridge are materialized from the run by the probe; the site is the captured Vite release `.cache/b/p/m/vite/release/5dc1b077e86de2aa/out`.

## Run 1: unmodified captured site

All 10 pane x locale cases fail identically: shell outcome `error`, console error `Framework OS boot failed Error: original-turn.native-return-refused: authority` (bridge `nativeRefusalError`, raised from the very first `reactor.poll`).

Root cause (not a Demonstrator fault): the same defect documented in `GET-NORM-PLUGIN-WORKING-END-TO-END/eurocode-boot-authority-2026-11-22.md`. The new guest lift requires `purpose` on `instance-open`; the TypeScript producers sent none. The producers are fixed in source (`shard-client/🚪️instance-lifecycle/🟦️.ts` forwards `purpose`; store worker `🚀️guest-activation/🟦️.ts` sends `purpose: { tag: "ordinary" }`), but the captured site `5dc1b077e86de2aa` was built 2026-11-21 11:31 and the instance-lifecycle source changed 17:12, so the staged bundle (`assets/🌐️-DLq6yBBu.js`, `assets/🟦️-x1UQdzrR.js`) still emits `instance-open` without `purpose`. Fix: the site (Vite release) must be rebuilt, not the Demonstrator component.

## Run 2: private site copy with both producers patched

A copy-on-write clone of the site had `purpose:{tag:"ordinary"}` inserted into the two minified `instance-open` producers (diagnostic only, not release provenance; the clone was removed afterwards).

| pane | en | de |
|---|---|---|
| koordinator | FAIL | FAIL |
| aussuchen | FAIL | FAIL |
| bearbeiten | FAIL | FAIL |
| aggregator | pass | pass |
| verfolgen | FAIL | FAIL |

Failing assertion in all eight: `pane "<id>": expected its shell to reach "ready"` (received `error`), plus the console assertion.

Console / shell errors:

- koordinator, aussuchen, bearbeiten: `setContributions command failed demonstrator typed-operation failed: {"code":"interactive-job.publication-stalled","message":"typed operation 'setContributions' advanced 4096 publication units without moving its ladder", ... retainedProgress all 0}`; bearbeiten and one koordinator run additionally `[DEBUG] FEM cleanup semantic failure instance=1 stage=24 progress=Some(Progress(zero))` followed by `typed-operation progress refresh failed` / `completion effects failed` with `original-turn.native-return-refused: semantic`.
- verfolgen: shell alert `typed-operation failed: ... 'setActiveExample' advanced 4096 publication units without moving its ladder` (no console error captured).

The earlier lease, app-owned classification and Bearbeiten parser fixes work: the old `original host classification lost its registered action` error and the Bearbeiten factory trap are gone. The commands are now accepted and reach the Publishing stage.

## Diagnosis of the remaining fault (not fixed)

- `interactive-job.publication-stalled` is minted by `fault_stalled_typed_operation_publication` (`🧵️instance/👷️jobs/📥️staging/🧾️typed-operations/🦀️.rs`) after 4096 unchanged units of a Publishing-stage operation: its publication ladder returns `Ok(())` without moving (completion not ready or store owner `Blocked`).
- The cleanup diagnostic names maintenance stage 24 = `drive_document_archive_load_retirements` (`🪜️maintenance-ladder/🦀️.rs`, `🧵️instance/📄️docs/.../🪜️maintenance-ladder/🦀️.rs`). So a document archive load is live and non-terminal, and every unit it is offered returns `Progress(empty)`; `runtime_live_cleanup_nonterminal_status` counts empty Progress as zero progress and faults after `RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT`. Phases that return empty progress while waiting on another owner: `AwaitingMembers` (members not yet beginnable, replacement `Initializing`), `AwaitReplacement` (replacement poll pending or ACK refused), `FillMember` and `MergeParent`.
- The wait target is the store replacement job (stage 14) and the host acknowledgement; whether the stuck phase is `AwaitingMembers` or `AwaitReplacement` cannot be told from the retained output because the diagnostic prints only the stage, not the load phase. The earlier per-owner cleanup trace for wmJDEkF (stage 104) is a different, older symptom.
- Candidate owner: an archive load that is waiting for the replacement driver or the original host ACK is reported as `Progress(empty)` instead of `AwaitingInput` (the idiom the ladder already uses for `document_archive_waits_for_host`). No change was made: per the earlier audits, relabelling or raising stall limits without a per-owner witness is unsound.
- Required next evidence: extend the `[DEBUG] FEM cleanup semantic failure` line to print the archive load phase and the replacement state (`active.phase`, `replacement.state`), rebuild the Demonstrator, rerun this probe with the patched site. A red regression then reproduces that phase under the mounted operation.

## Status

- Demonstrator rebuild: not queued; no Demonstrator source was changed (the blocking fault of run 1 is in the stale site bundle, the remaining fault needs the phase witness above).
- Site rebuild required before any linked-guest probe can pass: the captured site predates the `purpose` producer fix.
- No servers or browsers remain running; the patched site clone and probe output directories were deleted.

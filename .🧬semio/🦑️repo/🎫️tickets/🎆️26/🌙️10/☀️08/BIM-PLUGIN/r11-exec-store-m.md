# r11-exec-store-m: live maintenance ladder in grant currency

New module `FW/🔨️modules/🔌️plugin/🪜️maintenance-ladder/🦀️.rs` (`FW = 🧰️framework/🛍️products/💻️os`), declared in the plugin root next to the envelope ladder (`#[path = "🪜️maintenance-ladder/🦀️.rs"] mod maintenance_ladder;`). The `PluginApp for VcsArtifactApp` impl now delegates `maintenance_retirement_demands`, `maintenance_step`, `maintenance_terminal_is_empty`, `maintenance_under_pressure` to it. One addition outside the module: `PresenceStore::maintenance_local_reads_demands(&self, body)` in `FW/🔨️modules/🏪️store/🦀️.rs` (mirror of `TransientStore::maintenance_returned_reads_demands`, local-read factory birth capacity, depth + 1).

## Rotation (quote and step are one ordered unit list)

`maintenance_ladder_demands(body)` quotes the first unit with work; `maintenance_ladder_step(grant)` visits the same list in the same order and spends the unchanged grant on the first unit that releases something.

Order: command prune, closing private child group, window-config direct ingress, granted mounted retirement, tool overlay, tool runs, time travel (priority); document displaced / config-lane displaced / child root / child member (only under pressure, the stage cursor does not move); then the 26 fixed stages starting at `maintenance_stage`.

Fairness rules kept from the 676 loop: an empty stage never consumes the call and the cursor moves past it; a unit whose quote is not funded by the grant is skipped (a depth below the demand is an error, `plugin_turn_admitted`); `AwaitingInput` and `Blocked` units are remembered and reported only when nothing else moved (AwaitingInput first); when nothing moved the answer is `Complete` iff `maintenance_terminal_is_empty()`, else `Progress(default)`. `LAST_MAINTENANCE_STAGE` is still written (stage number, 100.. for the priority units). The old "Err UnsupportedOwner when something is owed but unquoted" guard is gone (in-flight typed operations in `Publishing` are legitimately owed nothing); an unaccounted frontier now shows as the runtime's zero-progress stall.

| stage | frontier | quote | step |
|---|---|---|---|
| 0 | typed operation worker (`Worker` stage; `Retiring` belongs to the granted-mounted unit) | `worker_step_demands(body)`; all-awaiting-ACK = default | `drive_worker_step(pool, grant)`; a worker fault goes through `fault_typed_operation_worker` (operation fault, never the turn's) |
| 1 | media closures | `ActiveMediaExport::retirement_demands` | `close_step`; on `Complete`: terminal check, `quarantine_media_snapshot`, remove, item receipt |
| 2 | segmented closures | release `ARTIFACT_OUTPUT_CHUNK_BYTES` while chunks remain | `close_take_chunk` (release = chunk length), then terminal check + remove |
| 3 | snapshot retirements | `ArtifactSnapshotCloseRetention::retirement_demands` | `close_step`; waits (Blocked) for its live media owner; remove on `Complete` |
| 4 / 20 | child root / child member | `child_root_retirement_demands` / `child_member_retirement_demands` | `child_root_retirement_step` / `child_member_retirement_step` |
| 5 / 6 | peer presence root / app-typed presence peers | `retirement_demands()` / `(body)` | `close_step(grant)`, remove on `Complete`; a zero receipt advances the cursor |
| 7 | peer roster | faulted `close_demands`; `Ready` = Arc capacity; else `step_demands` | `step(&presence_store, grant)` / `close_step`; on `Ready`: `validate_peer_roster_publication`, `take_candidate`, `publish_peer_roster_candidate_admitted`, outcome hand-off, reservation release; a step fault is retained with `fail` |
| 8 | snapshot-read returns | `snapshot_read_returns_demands(body)` | up to three pump turns per visit (document, config, interaction), first with progress wins |
| 9 / 25 | displaced owners of document / config, draft, interaction, window config | `maintenance_retirements_demands(body)` | `maintenance_retirements_step(grant)` |
| 10 | envelope ingress (closing ones only) | `envelope_ingress_demands(false, body)` | `drive_envelope_ingress(grant, false)` |
| 11 / 12 / 13 | envelope decode jobs / field-decoder returns / completed-record returns | envelope-ladder `*_demands(false, body)` | envelope-ladder `drive_*(grant, false)` |
| 14 | store replacement | `store_replacement_demands(false, body)` | `drive_store_replacement_jobs(grant, false)` |
| 15 | mounted jobs | `A::mounted_job_maintenance_demands` | `A::mounted_job_maintenance_step` |
| 16 | instance operation owner | `retirement_demands(body)` | `maintenance_step(grant)` |
| 17 | latest-wins keys | `advance_demands()` | `advance(grant)` |
| 18 | cancellation slot cleanup | item | `cleanup_finished_slot` |
| 19 | presence local reads | `maintenance_local_reads_demands(body)` (new) | `maintenance_local_reads_step(grant)` |
| 21 | displaced document windows | `document_windows_retirement_demands(body, false)` | `retire_document_windows_step(grant, false)` |
| 22 | window transient store | `maintenance_demands(body)` | `maintenance_step(grant)` |
| 23 | parked worker-job sessions | item (no job-crate quote exists) | `pump_worker_job_retirements(1, grant)` |
| 24 | archive loads | `document_archive_demands(false, body)` | `drive_document_archive_load_retirements(grant, false)` |

## Signatures exposed for the close ladder (`impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M>`, all `pub(crate)`)

- `store_replacement_demands(&self, closing: bool, body: usize) -> Result<RetirementDemand, ValueError>`
- `drive_store_replacement_jobs(&mut self, grant: RetainedCloneGrant, closing: bool) -> Result<PluginLifecycleStep, Fault>`
- `document_archive_demands(&self, closing: bool, body: usize) -> Result<RetirementDemand, ValueError>`
- `drive_document_archive_load_retirements(&mut self, grant: RetainedCloneGrant, closing: bool) -> Result<PluginLifecycleStep, Fault>`
- `envelope_ingress_demands(&self, closing: bool, body: usize) -> Result<RetirementDemand, ValueError>`
- `drive_envelope_ingress(&mut self, grant: RetainedCloneGrant, closing: bool) -> Result<PluginLifecycleStep, Fault>`
- also `maintenance_ladder_demands/step/terminal_is_empty/under_pressure`.

Contract of the three drivers: no entry in the target registry = `Complete(default)`; a grant axis below the current quote = `Progress(default)` (depth below the quote = `Err`); `closing = true` visits every entry (not only drivable ones), cancels/terminates it first (`request_cancel`, `request_terminal(Cancelled)`, `ingress.closing = true`) and removes the entry in the turn that finds it terminal-empty (receipt `copied_items: 1`); `closing = false` only visits entries that can move (store replacement not `Complete` and not waiting for its member ingress, archive load not terminal-empty and not waiting for the host merge, ingress already closing). The cursors used are `close_*_cursor` when closing and `maintenance_*_cursor` otherwise.

## Restored drivers

- Store replacement state dispatcher: `Initializing -> drive_initializer`, cancel check -> `refuse(Cancelled)`, `AwaitingMembers` (childless seal or `complete_store_replacement_genesis`), `OpeningMembers -> drive_member_open`, `ClosingRejectedMember`, `ValidatingClosure -> drive_closure`, `PreparingCandidateViews -> drive_candidate_views`, `Retiring*Members -> drive_rejected_members(&child_content_root)` (builds the disposer first), `CandidateReady -> commit_store_replacement_candidate` (guard evidence, `A::validate_document_store_publication`, `publish_boxed_document_store_candidate_if_authoritative`, window reset, child registry swap; quote = capacity of the fresh 64-slot content-retirement registry, receipt retains it), `Retiring{Committed,Rejected}Store -> drive_retained_store`.
- Archive load: `advance_document_archive_load` with all live phases in grant currency (decode, auxiliary retirement through `artifact_retirement_admit_owned` + `artifact_retirement_box_close_step`, hydration, replacement hand-over, genesis, member begin/fill/retire-source/admit, seal, await) and `drive_document_archive_terminal` (cancel/poll/acknowledge the replacement, then `ActiveDocumentArchiveLoad::close_step`). A phase that hits a primary fault records it (`request_fault`) and continues through the terminal drive in the same call, exactly as 676. The next turn of an `AwaitReplacement` that reached its outcome is the terminal turn (no cross-quote spending in one call).
- Envelope ingress: close step per owner, entry removed on `Complete`.

## Gate results

`cargo check --manifest-path Cargo.toml -p semio-framework-plugin --lib --message-format=short` (gate label `r11-store-m`, logs `T/🗑️generated/r11-store-m/c1..c6.txt`):
- c1-c4: errors only in peers' regions (cancellation lease module, `Fault: From<ValueError>` in the window-config capture); none in the maintenance module (typeck and borrowck of the module were already clean in c1).
- c5: 0 errors. c6 (after deleting the dead `DOCUMENT_ARCHIVE_POLL_STEP_BYTES` and routing the retiring replacement states through `ActiveArtifactStoreReplacement::retirement_demands`): 0 errors, 945 warnings, none in the maintenance module; the never-used warnings of `drive_initializer/drive_member_open/drive_closure/drive_candidate_views/drive_rejected_members/drive_retained_store`, the archive-load `close_step`/`retirement_demands`, `PeerRosterPublication::{step,close_*}`, `PeerPresence*Retirement`, `OwnedDocumentMemberIngress*`, `SnapshotReadReturnPump::retirement_demands` and `ActiveArtifactEnvelopeIngress` are gone.
- Not run: `--tests`, `cargo test` (test files still call `maintenance_step(items, bytes)`: `🧪️tests/🔬️app-typed-command-full-operation`, `🔬️plugin-runtime-plugin-builder-contract`, `⏳️completion`). No runtime behaviour was exercised.

Other root edit: deleted `DOCUMENT_ARCHIVE_POLL_STEP_BYTES` (no user left; the poll loop spends `runtime_lifecycle_grant()`).

## Open issues

1. Worker-job retirement array (stage 23): the job crate offers no pre-quote for `pump_worker_job_retirements`; the unit quotes only `depth: 1` and the pump yields internally when the grant is short. Needs a `next_worker_job_retirement_demands()` in the job crate for an honest quote.
2. Archive hydration (`RetainedPersistedDocumentHydration::step`) has no live-step quote: the phase quotes one item plus an upper bound for retiring a rejected envelope (`retire_document_envelope_birth_bytes + size_of::<DocumentStoreOwners>`, depth 4). Likewise the history decoder (`DecodeParent`) is quoted at 1 copy byte and its receipt claims `min(grant copy, spr length)` because the decoder does not report consumed bytes.
3. Unquoted allocations inside the replacement commit: the displaced window-transient registry reset and the disposer box (only the 64-slot child-content retirement registry is quoted and receipted as retained capacity).
4. `retire_document_archive_envelope` leaks (`mem::forget`) the owners and envelope if the grant-funded uninstalled retirement is refused after the quote admitted it; this is an invariant violation path that returns a fault.
5. Stages 16, 18 and 22 are always candidates (cheap per visit, zero progress when idle); an app with a live latest-wins scope or an unpresented typed result keeps `maintenance_terminal_is_empty()` false and therefore answers `Progress(default)`, counted by the runtime's 256-step zero-progress limit exactly as the 676 `Pending{0,0}` was.
6. Left for the close-ladder owner: `drive_artifact_owned_disposer` (never used until the disposer rungs call it), `ArtifactCacheRetirement` (`close_cache` is never set), `DOCUMENT_ARCHIVE_*` fields `close_*_drained` that the 676 ladder used.

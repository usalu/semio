# r11-exec-store-p2c

Chunk: plugin root `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, from `ToolPublicationClaim` to just before `pub struct VcsArtifactApp` (about 20 500 - 26 100 at the end of the run).
Gate: `cargo check -p semio-framework-plugin --lib` is clean (0 errors, last run `T/🗑️generated/r11-store-p2c/check7.txt`). `--lib --tests` shows no error inside my root-file test fixtures; the remaining test errors are in included test files (list below).

## Shared helpers added (root, before `RetainedLatestWinsKeys`)

| helper | meaning |
| --- | --- |
| `plugin_grant_funds(demand, grant) -> bool` | alias of `mounted_retirement_grant_funds` (every axis + items + depth) |
| `plugin_turn_admitted(demand, grant) -> Result<bool, Fault>` | under-granted axis = `Ok(false)` (yield), depth below demand = `Err(DepthLimit)` |
| `plugin_page_grant(page_bytes) -> RetainedCloneGrant` | one item, every byte axis = page, depth = AVL height budget (normal-operation pumps) |
| `plugin_demand_grant(demand) -> RetainedCloneGrant` | one item, funds exactly the quoted axes (self-funded callers) |
| `plugin_retirement_fault(ValueError) -> Fault` | |
| `plugin_quoted_demand(grant)`, `plugin_job_close_demand(job, body)`, `plugin_outcome_close_demand(outcome)` | demand readers |
| `plugin_outcome_close_progress(outcome, grant)` | releases one retained job-result page (job crate still has the old `StepOutcome::close_step(items, bytes)`) |
| `plugin_interactive_close_progress(step, grant, terminal)`, `plugin_worker_close_progress(step, grant, terminal)` | `Ok(None)` = blocked, refusal = fault, receipt must `fits(grant)`, Complete needs terminal witness |
| `plugin_text_retirement_demand/plugin_text_close`, `plugin_bytes_retirement_demand/plugin_bytes_close` | copy axis = truncation, release axis = backing capacity |
| `plugin_typed_owner_demand/plugin_typed_owner_step` | original typed owner -> `ControlledRetirement` frame, then its frontier |

## Ported (old shape -> new)

- `ToolLatestWinsKeyCopy`: `advance(parts, grant) -> RetainedCloneStep` (copy axis = 1 byte per hex digit, 2 per source byte), `retirement_demands(parts)`, `close_demands()`, `close_step(grant) -> RetainedCloneStep` (truncate = copy, backing = release).
- `ToolLatestWinsRegistry`: `advance(grant) -> Result<RetainedCloneStep, Fault>` + `advance_demands()`; `retirement_step` returns a progress receipt; an `ordered` `Blocked` is a yield; `Complete` only at `terminal_is_empty()`.
- `ActiveMediaExport`: `retirement_demands(body)`, `close_step(grant) -> Result<PluginLifecycleStep, Fault>`; `retire_nonterminal_outcome() -> Result<(), Fault>` (live pump, page-funded).
- `ArtifactFixedRegistry::close_empty_backing_step(grant) -> Result<RetainedCloneStep, Fault>` (an active slot is an `Err`, was `Blocked`).
- `ChildEmissionPreviewJob`: now holds `emit: RefCell<Option<Emit>>` (the demand fns are `&self` but `Emit::close_child_demands` needs `&mut`); full `InteractiveJob` close + four demand fns; `drive_agent_lane_preview` closes rejected sessions/sessions with `next_close_demands(PAGE).map(close_step)`.
- `PendingLatestWinsCommand`, `PendingChildGroupPublication` (new field `mutations_retirement: Option<ControlledRetirement<Vec<A::Mutation>>>`; mutations retire through the typed controlled frame, backing and receipt text are released on the release axis), `TypedCommandFullOperationJob` (`InteractiveJob` close + demands), `ActiveArtifactEnvelopeDecode`, `ActiveArtifactEnvelopeIngress`, `CompositionPinsRetirement`, `ActiveArtifactStoreReplacement` (all drivers), `PendingDocumentArchiveMember`, `ActiveDocumentArchiveLoad` (new field `retained_pending`, admitted through `store::artifact_retirement_admit_owned`), `ArtifactCacheRetirement`.
- `PendingArtifactStorePublication::close_step`: Presence/Transient arms convert to `ArtifactStoreOneItemGrant`.
- Test fixtures in root: `test_retained_keyed_dispatch` (key retirement law re-expressed with grants: scalar copy credit, then backing release, then hand-off item), identity macro on `store.dispatch`, `fixture_close_turn` / `fixture_maintenance_turn` helpers (self-funded from `close_retirement_demands` / `maintenance_retirement_demands`).

## Deleted as dead

- `ArtifactFixedRegistry`: nothing else.
- `artifact_store_initialization_release_bytes` (only users were the two old initializer branches; the quoted demand replaced it).
- `MountedTypedCommandFullOperation::retire_string_scalar`, `reserved_close_byte_demand` (superseded by `plugin_text_close` and the demand quotes), `ActiveArtifactStoreReplacement::member_ingress_release_bytes`, `next_close_byte_demand` (replaced by `rejected_members_demands`/`retained_store_demands`/`retirement_demands`), `ActiveDocumentArchiveLoad::next_close_byte_demand`.
- unused imports `latest_wins_ordered`, `JobFault`.
Proof: `git grep -nw <name>` over the plugin dir (also tests) returned no other user.

## LIVE rung table (field of `VcsArtifactApp` -> element methods)

| field | type | demand fn | step fn |
| --- | --- | --- | --- |
| `latest_wins_keys` | `ToolLatestWinsRegistry` | `advance_demands(&self) -> Result<RetirementDemand, ValueError>` | `advance(&mut self, grant) -> Result<RetainedCloneStep, Fault>` (call `begin_close()` first; Complete == `terminal_is_empty()`) |
| `latest_wins_commands` | `PendingLatestWinsCommand<A>` | `retirement_demands(&self, body) -> Result<RetirementDemand, ValueError>` | `close_step(&mut self, grant) -> Result<RetainedCloneStep, Fault>`; remove the entry when `command_owners_are_empty()`; key sub-rung: `key_retirement_demands(body)` / `close_key_step(grant)` |
| (inside the above) | `ToolLatestWinsKeyCopy` | `close_demands()` / `retirement_demands(parts)` | `close_step(grant)` / `advance(parts, grant)` |
| `media_exports`, `media_closures` | `ActiveMediaExport` | `retirement_demands(&self, body) -> Result<RetirementDemand, ValueError>` | `close_step(&mut self, grant) -> Result<PluginLifecycleStep, Fault>`; `terminal_is_empty() -> Result<bool, Fault>`; ladder then quarantines the snapshot and removes it |
| all `ArtifactFixedRegistry<T>` fields | `ArtifactFixedRegistry<T>` | `empty_backing_byte_demand(&self) -> Option<usize>` | `close_empty_backing_step(&mut self, grant) -> Result<RetainedCloneStep, Fault>` (private-group owner file already calls it with the old shape: needs its p1 owner to switch) |
| `tool_operations` | `MountedTypedCommandFullOperation<A>` | `close_demands(body)`, `retirement_demands(body)`, `worker_step_demands(body)` | `close_step(grant) -> Result<PluginLifecycleStep, Fault>`, `retirement_step(grant)`, `drive_worker_step(pool, grant)` (this impl was concurrently rewritten by another agent while I worked; their version is the one in the file, I did not touch it again) |
| `envelope_ingress` | `ActiveArtifactEnvelopeIngress` | `retirement_demands(&self) -> RetirementDemand` | `close_step(&mut self, grant) -> Result<RetainedCloneStep, Fault>`; remove entry when `terminal_is_empty()` |
| `envelope_decode_jobs` | `ActiveArtifactEnvelopeDecode<P, Mutation>` | `drive_demands(&self, body) -> Result<RetirementDemand, ValueError>` | `drive(&mut self, pool, live_generation, completed, grant) -> Result<PluginLifecycleStep, Fault>` (same fn pumps live work and closes; `terminal_is_empty(completed)` then remove); `force_worker_session_terminal(&mut self, budget) -> bool` is self-funded |
| `store_replacement_jobs` | `ActiveArtifactStoreReplacement<P, Mutation, M>` | `member_open_demands(body)`, `closure_demands()`, `candidate_views_demands()`, `rejected_members_demands(body)`, `initializer_demands(body)`, `retained_store_demands(body)`, `retirement_demands(body)` (by state) | `drive_initializer(pool, grant)`, `drive_member_open(grant)`, `drive_closure(grant)`, `drive_candidate_views(grant)`, `drive_rejected_members(current_content, grant)`, `drive_retained_store(grant)`, all `Result<PluginLifecycleStep, Fault>`. The state dispatcher that picks one (and the `CandidateReady` commit with `publish_boxed_document_store_candidate_if_authoritative`) lived in the deleted `drive_store_replacement_jobs`; nothing calls these yet. Live steps (`drive_member_open`) spend the caller's grant: pass `plugin_page_grant(store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES)`-sized grants, the store `MemberOpenOperation::step` has no quote |
| `document_archive_loads` | `ActiveDocumentArchiveLoad<P, Mutation>` | `retirement_demands(&self, body) -> Result<RetirementDemand, ValueError>` | `close_step(&mut self, grant) -> Result<PluginLifecycleStep, Fault>`; entry removable when `terminal_is_empty()`. Member staging: `PendingDocumentArchiveMember::{source_retirement_demands(), retire_source_step(grant) -> RetainedCloneStep, retirement_demands(), close_step(grant) -> Result<RetainedCloneStep, Fault>}` |
| `pending_child_publication` (inside a mounted operation) | `PendingChildGroupPublication<A>` | `retirement_demands(body)` | `close_step(grant) -> Result<RetainedCloneStep, Fault>` |
| `ToolCancellationHandle` slots | | none (no byte currency) | `cleanup_slot(index)` unchanged; the new ladder already uses it |
| `pending_reserved`, `reserved_commits`, `reserved_commit_outcome` | | none | items only: `pending.permit.finish()` / drop, nothing to migrate |

## Edits outside my chunk (trivial, call-site only)

- line ~28442 (archive genesis): `retire_source_step` call now builds its grant from `source_retirement_demands()` with a page copy credit.
- r11-store already adapted the callers of `ToolLatestWinsRegistry::advance`, `close_key_step`, `ChildEmissionPreviewJob` literal, `retire_source_step`.

## Open items

1. Test files still on the old shapes (not compiled by `--lib`): `🧪️tests/🔬️app-artifact-fixed-registry`, `🔬️app-child-member-registry`, `🔬️app-typed-command-full-operation`, `🔬️plugin-runtime-plugin-builder-contract` (`ArtifactCacheRetirement::close_step(1)`), `🧩️composition`, plus `🪟️window/🫧️transient/🧪️tests/🔁️document-replacement` and the composition-owner file (`close_empty_backing_step`).
2. `VcsArtifactApp.close_cache` is never set (only initialised `None`); `ArtifactCacheRetirement` now has `retirement_demands()/close_step(grant)` but nothing drains `self.cache`; r11-store decides whether to wire or delete.
3. None of the `drive_*` drivers of `ActiveArtifactStoreReplacement`/`ActiveDocumentArchiveLoad` has a caller in the root file after r11-store deleted `drive_store_replacement_jobs`/`drive_document_archive_*`; the feature (`Effect::LoadDocument`) needs them again.
4. `HistoryLog` retirement: `retained_pending` uses `(Option<HistoryLog>, Option<(Vec<String>, Vec<String>)>)` as the original owner; it compiles, so the tuple/`HistoryLog` already satisfy `RetireOwned`.

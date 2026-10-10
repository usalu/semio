# r11-exec-store-p2: plugin root chunk 3000-24999 (grant currency)

Root = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (anchors by item name). I took lines ~3000-15800 myself and delegated the rest to two sub-helpers (reports `T/r11-exec-store-p2b.md` = `PluginCloseStep`..`FrameworkConfigurationBinaryJob`, `T/r11-exec-store-p2c.md` = `ToolPublicationClaim`..before `VcsArtifactApp`).

## Result
- `cargo check -p semio-framework-plugin --lib` (gate label `r11-store-p2`, last log `T/🗑️generated/r11-store-p2/check8.txt`): 0 errors, 964 warnings. Whole crate, not only my chunk (peers finished concurrently).
- Not run: `--tests`, `cargo test` (included test files still use old shapes, see open items).

## Files
- Root file, my regions (below). Store crate: `artifact_retirement_owner_demands/close` made `pub` (were `pub(crate)`); r11-store added `SpaceMember::{snapshot_read_retirement_demand, returned_snapshot_read_retirement_demand}` on my request; p2b added two pub fns (see its report).
- Scratch only under `T/🗑️generated/r11-store-p2/` (`splice.py` = atomic temp-file+`os.replace` splice used for the large rewrites, `errsnip.py`, `rooterr.sh`, `warnsnip.py`).

## Changed in my regions (old -> new)
- `DocumentCodecSpec::{bare,codec}`, `document_codec_bare(_async)`: `Snapshot/Mutation: RetireOwned` bounds. `NoConfig`, `NoConfigMutation`: `#[derive(semio_framework_value::RetireOwned)]`.
- `OwnedDocumentMemberIngress`, `OwnedDocumentMemberIngressRegistry`: `retirement_demands(body)` + `close_step(grant)->Result<RetainedCloneStep,Fault>` (request closed as nested owner with child grant, identity strings released on the release axis, handoff = `copied_items:1`).
- `PreparedChildContentEntry::{close_step,close_metadata_step}`, `ChildContentView::{take_prepared_entry (returns RetainedCloneStep), close_prepared_structure_step, insert_prepared_entry (capacity axis)}`: grant currency; `PreparedChildContentEntry::prepare` uses `maximum_capacity_bytes`.
- `ChildContentView::take_one` rewritten in place (`Arc::get_mut`, no unwrap/re-wrap allocation) with exact Arc release bytes (`ChildContentTake::{Blocked,Released(b),Snapshot(entry,b),Complete(b)}`) + `next_take_release_bytes()` quote.
- `ChildContentRetirement`: `retirement_demands(children, retiring, body)` + `close_step(children, retiring, current, owners, grant)`; `active_member` now keeps the dialect strings and releases all identity strings in one counted step; funded via `SpaceMember::{retire_snapshot_read_erased(&mut Option,grant), take_returned_snapshot_read_retirement(grant)}`; new read-only `child_snapshot_owner_ref` / `retiring_member`.
- `SnapshotReadReturnPump::{retirement_demands, drive(next(grant), grant)}`, `ChildMemberRetirement::{retirement_demands, close_step(grant)}` (final identity strings counted).
- `PeerPresenceEntryRetirement`, `PeerPresenceRootRetirement`: `retirement_demands` + `close_step(grant)` (Arc root/entry shells counted).
- `PeerRosterPublication`: `fail`->`PluginLifecycleStep`, new `is_faulted`, `step_demands`, `step(presence_store, grant)->PluginLifecycleStep`, `close_demands(body)`, `close_step(grant)->PluginLifecycleStep`; new field `rejected_original` (rejected app-typed presence is admitted by a funded close turn via `PresencePeerAdmissionRejected::into_retirement(child_grant)`, original handed back on refusal).
- `ChildEmitGenesis::close_one(grant)`, `ChildEmit::{close_one(grant)->RetainedCloneStep, return_one(parent, grant)}`; `Emit::{child_preparation_demands, prepare_child_one(grant), prepare_child_preview_one(grant), close_child_demands(&mut self, body), close_child_one(grant)->Result<Option<PluginLifecycleStep>,Fault>, return_child_one(parent, grant)}` (replaces `next_child_preparation_byte_demand`).
- `NaturalFileDecodeCursor<T>`: `close_demands(&self, body)`, `close_step(grant)->Result<PluginLifecycleStep,Fault>` (no `next_close_byte_demand`).
- `artifact_app_laws`: `fixture_lifecycle_grant`, `drain_maintenance_pressure`, `settle_registered_typed_operation`, `close_registered_fixture_app` re-expressed with grants/receipts (`runtime_lifecycle_grant()` widened to the quoted demand, `progress.fits(grant)` law kept).

## Deleted as dead
Nothing beyond what p2b/p2c report: my regions had no unreferenced functions (name census over the plugin dir). `PluginCloseStep` is NOT deleted: users remain in the submodule files owned by p1 (`composition` owner files, `window/config`, `child/document`, preparation), `ChildEmission` helpers of p1/p3 and external artifact crates.

## LIVE rung pairs of my chunk (for r11-store's ladder)
| VcsArtifactApp field | type | demand | step |
|---|---|---|---|
| `child_content_retirements` | `ChildContentRetirement` | `retirement_demands(&children, Option<&retiring>, body)` | `close_step(&mut children, Option<&mut retiring>, &current, &owners, grant)->Result<RetainedCloneStep,Fault>` (owners: `ArtifactFixedRegistry<ChildContentRetirement>::sibling_content_owners`) |
| `child_member_retirements` | `ChildMemberRetirement<M>` | `retirement_demands(body)` | `close_step(grant)->Result<RetainedCloneStep,Fault>` |
| `document_/config_/interaction_snapshot_read_returns` | `SnapshotReadReturnPump` | `retirement_demands(&self, body, next_demand)` (`next_demand` = store `returned_snapshot_read_retirement_demand()`) | `drive(|g| store.take_returned_snapshot_read_retirement(g).map_err(..), grant)->Result<RetainedCloneStep,Fault>` |
| `peer_presence_retirements` | `PeerPresenceRootRetirement` | `retirement_demands()` | `close_step(grant)->Result<RetainedCloneStep,Fault>` |
| `peer_roster_publications` | `PeerRosterPublication<A>` | healthy: `step_demands()`; faulted (`is_faulted()`): `close_demands(body)` | `step(&presence_store, grant)->Result<PluginLifecycleStep,Fault>` / `close_step(grant)->Result<PluginLifecycleStep,Fault>`; completion path unchanged (`take_candidate`, `retain_rejected_candidate`) |
| `store_replacement_jobs` (via p2c) | `OwnedDocumentMemberIngressRegistry` / `OwnedDocumentMemberIngress` | `retirement_demands(body)` | `close_step(grant)->Result<RetainedCloneStep,Fault>` |
| retained fields / jobs | `Emit` (children) | `close_child_demands(&mut self, body)` | `close_child_one(grant)->Result<Option<PluginLifecycleStep>,Fault>` (None = nothing left) |
| `ArtifactApp` / `NaturalFileImportJob` | `NaturalFileDecodeCursor<T>` | `close_demands(&self, body)` | `close_step(grant)` |
p2b's and p2c's tables (instance operation owner, disposers, media completion, tool-run raw input, store-initialization job, latest-wins, media exports, envelope decode, store replacement drivers, archive loads, ...) are in their reports.

## Not wired yet (compiler dead-code warnings after the lib went green)
Live frontiers of `close_terminal_is_empty` whose rung pairs exist but nothing calls: `PeerRosterPublication` (step+close), `PeerPresenceRootRetirement`/`PeerPresenceEntryRetirement`, `OwnedDocumentMemberIngress(+Registry)` (only reachable through the store-replacement drivers), `SnapshotReadReturnPump::retirement_demands`, p2c's `drive_*` store-replacement/archive-load drivers, `ArtifactCacheRetirement` (`close_cache` never set). r11-store: wire (maintenance stage 7 peer-roster in the 676 copy ~35140, `maintenance_step`) or delete.

## Open items
1. `ChildContentView::take_one`, peer-presence retirements and `ChildMemberRetirement` count only the allocations the code frees (Arc shells, identity strings, vector backings); `Vec`/`Box` shells inside `M` and `ArtifactCommand` content are not quoted (p2b open item).
2. Test files and `.ts` source-text laws still assert old shapes (see p2b/p2c reports plus `🧪️tests/🔬️app-child-member-registry`, `🧪️tests/🧩️composition`).
3. `Emit::close_child_demands` is `&mut self` because `ChildEmitPreparation::retirement_demands(&mut self)` (p1 preparation file) is `&mut`; p3/r11-store asked for `&self`: needs p1 to make the cause-text read non-mutating, then flip mine.
4. External artifact crates (writer `return_child_one`, editors' `close_child_one(items, bytes)`, ...) still call the 676 shapes.

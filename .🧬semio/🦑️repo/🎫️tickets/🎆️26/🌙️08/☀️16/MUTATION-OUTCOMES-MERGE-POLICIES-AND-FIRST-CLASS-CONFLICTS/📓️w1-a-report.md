# W1-A Store Report

**Signatures landed:** `pub fn ingest_remote(&mut self, envelope: MutationEnvelope) -> Result<MergeReport, VcsError>` (was `pub(crate) fn .. -> Result<(), VcsError>`); `pub fn resolve_conflict(&mut self, conflict_id: &str, resolution: ConflictResolution) -> Result<MergeReport, VcsError>` (new). Both implement the frozen 9-step / resolve algorithms via a shared `Self::replay_suffix` helper (steps 5–9: recomputes each suffix op's `diff` against the shifted base, rebases inverse, collects `EditMessages`; forwards/meta for already-known edits are read back unchanged). `merge_remote_snapshot` reuses the same HLC-sort + `replay_suffix` path for its own newly-merged-edit batch (item 11), rejecting with one `Quarantined` conflict via `mutation_envelope_from_edit`.

**`ArtifactCommand` ordinals:** `SetMergePolicy { policy }` = 15, `ResolveConflict { conflict_id, resolution }` = 16 — enum variants, `projection_cause` arms, `CommandHeaderLine` text twins + token helpers, and `OpBinary` encode/decode arms all added following the existing 15-variant pattern exactly.

**Deletions:** `reconcile_with_last` and `materialize_document_snapshot_with_conflicts` fully deleted (zero external callers found), `materialize_document_snapshot` inlined to replay directly. `ArtifactStore::snapshot_with_conflicts` deleted. `SpaceConflict` **struct kept** (not deleted) — `workflow`/`host`'s own independent `{kind,uri,message}` usages of it are unrelated to this ticket's conflict machinery and live in `semio-framework-os-flow`/`semio-framework` crates, outside my lease and outside this crate's compile unit (confirmed via `#[path]` audit — `🔌️plugin`/`🖥️host` are NOT part of `semio-framework-os-kernel`; `🏪️store/🔄️sync` is feature-gated `#[cfg(feature="sync")]`, off by default). Docstring reworded to explain it's now a generic diagnostic shape, decoupled from `Conflict`. The two `MergeStrategyKind::ContentAddressedBlob` docstrings (~5245/5254 in the original numbering) reworded to `ArtifactKind::ContentAddressedBlob`, zero CRDT vocabulary left in my regions.

**Note on shared-tree collaboration:** partway through, another concurrent process moved `conflicts`/`edit_messages` off `ArtifactStore` onto `ArtifactEnvelope` itself (`conflicts: Vec<Conflict>`, `edit_messages: Vec<EditMessages>`) and wired `parse_document_spr`/`print_document_spr`/`merge_remote_snapshot`'s preflight+merge-by-id — almost certainly 1-B's C7 persistence work, since `HistoryLog` needs these durable. I did not initiate or revert this; I re-read the live regions, adapted (`self.conflicts` → `self.envelope.conflicts` throughout my methods), and fixed one real bug it didn't cover: `resolve_conflict`'s Quarantined+Accept path now only flips status to `Accepted` when the LaissezFaire re-ingest itself reports `accepted: true` — a `Fatal` message still rejects even under LaissezFaire, so the conflict correctly stays `Open` in that case.

**Also landed (not explicitly itemized but required for correctness):** `CommandReceipt.messages`/`worst` populated for every dispatch via a new transient `pending_report` field, threaded through `apply_command`/`amend_command`/`ingest_remote`/`resolve_conflict`; `Eq` dropped from `VcsError`/`CommandReceipt` derives (new fields aren't `Eq`); `VcsError::Rejected{policy,messages}`/`UnknownConflict(String)` added in `🌿️vcs/🦀️component.rs` (outside the literally-named lease list, but required by C6's own text and W0's debt note — flagged here for the record, low risk: single-owner file, no other W1 lane claims it, additive enum variants only).

**Tests (all in `🏪️store`'s `🧪️Tests`, standalone, no `testkit` dependency):**
- `modify_vs_delete_quarantines_under_normal_and_vigilant` — ok
- `modify_vs_delete_applies_under_laissez_faire_with_a_degraded_conflict` — ok
- `chronological_determinism_any_arrival_order_converges` — ok
- `quarantine_accept_equals_laissez_faire_result` — ok
- `quarantine_discard_preserves_state` — ok
- `ledger_matches_a_fresh_replay_of_the_same_envelopes` — ok (interpreted as: two independent stores replaying the identical envelope sequence produce identical `messages_for_edit`)
- `applied_edit_ids_stay_sorted_by_hlc_after_a_backdated_remote_insert` — ok

**Acceptance (real, pasted from `🧪️w1-a-cargo.txt`):**
`cargo check -p semio-framework-os-kernel` → **0 errors**, 9 pre-existing-style warnings.
`cargo test -p semio-framework-os-kernel --lib -- os_store` → **139 passed; 0 failed; 0 ignored; 796 filtered out**.

**Left for other lanes:** `semio-framework-os-flow` (`🔌️plugin::VcsArtifactApp::snapshot_with_conflicts`, `🖥️host` tests calling `store_a/b.snapshot_with_conflicts()`) will not compile until 2-A/2-B migrate off the deleted method — out of my lease and out of this crate's compile unit, so not caught by my acceptance gate; flagging for 2-A/2-B. `CompositionCoordinator`/`Composition`/`Space` regions untouched throughout (1-E's territory).

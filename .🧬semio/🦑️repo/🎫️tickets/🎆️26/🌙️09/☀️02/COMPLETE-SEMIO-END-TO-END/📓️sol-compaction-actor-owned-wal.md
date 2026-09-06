# Actor-Owned WAL Compaction

Status: source-complete for the bounded document-actor integration and its retained VCS shutdown success path on 2026-09-05. The registered language-neutral source oracle is green. The latest native attempt remains red at the actor law and the repair has not yet received a native rerun.

## Implemented boundary

- `ArtifactMessage::Compact` is a command-priority mailbox message. `ArtifactEngine` executes it with its existing mutable `ArtifactWal`; neither the live path nor the database facade attempts to acquire a second writer permit.
- The retained compaction path derives the document and authoritative active segment from that WAL. Its only destructive WAL capability is `ArtifactWal::delete_compacted_sealed_segment`, which rejects the active segment, verifies the target is sealed, and stamps deletion with the WAL's private retained writer.
- The compaction lease is acquired before snapshot-floor discovery, committed-WAL horizon scanning, or WAL deletion. Every ordinary success, cancellation, and error result attempts lease release before returning the engine to the actor.
- Document-scoped compaction never deletes global CAS payloads. The prior document-local candidate/live scan was removed because it could not observe another document's live reference. Payload reclamation remains deferred until storage owns a global reference authority.
- `Database::compact_document` and `ArtifactHandle::compact` now queue through the live document actor. The standalone retained compaction machinery remains available only for its retained-operation tests and opens/closes its own WAL before and after its lease-owned work.
- Test-only `Compactor` and all raw compaction fixtures now use explicit writer permits. Fixtures release those permits before opening or reopening an `ArtifactWal`, and actor test segment-list result owners are explicitly retired.

## Laws

The strict neutral committed-effects fixture and schema now also fix the actor boundary: command priority, retained-WAL writer authority, WAL-derived active segment, lease-before-effects order, and queued-submit behavior.

The registered native group contains:

- `db_compact::tests::compaction_applies_only_committed_frontier_snapshot_and_payload_effects`
- `db_compact::tests::document_compaction_retains_shared_and_private_cas_without_global_reference_authority`
- `db_engine::vcs_integration::retained_tests::vcs_store_keeps_exact_history_owners_through_changes_checkpoint_and_bounded_close`
- `db_engine::tests::compact_document_uses_live_actor_writer_and_restores_submits`

The new actor law creates enough bounded public commands to rotate the real WAL, proves an independent writer conflicts before, during the post-compaction state, and after cancellation, publishes a snapshot floor, compacts a sealed predecessor without deleting the actor's active segment or any CAS payload, and proves a later command is accepted by the same actor. A separate two-document law gives document A one shared and one private CAS reference, gives document B the shared reference, compacts A, and requires both payloads to remain.

## Receipt and nonclaims

```text
NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-os-kernel:wal-committed-compaction-check --skip-nx-cache
session 76244, exit 0
wal-committed-compaction-independent-oracle: abort effects excluded, global payloads retained, header-only highest preserved
```

The current registered source receipt is GREEN (`wal-committed-compaction-independent-oracle: abort effects excluded, global payloads retained, header-only highest preserved`, Nx exit 0). This is not a Rust compile or runtime verdict.

The root-owned `DOCr0L/00` native attempt compiled and passed the first two compaction laws, then the actor law aborted. Its first panic was `LimitExceeded("wal source backing capacity")`: the encoded command length remained within the 64-KiB test limit, but the encoder's `Vec` allocation capacity exceeded it. The later Store destructor panic was unwind fallout. The private Artifact WAL admission now converts an in-limit, over-allocated encoded `Vec` through an exact boxed slice before retained admission; `WalBytes` itself keeps its strict backing-capacity rejection.

VCS shutdown now owns one Store close step at a time and reinserts the exact Store into its cell on Pending, Blocked, or Err. The graph drain revisits Pending/Blocked cells, and `Database::shutdown` invokes it only when the database is the sole `VersionGraphs` owner. Shared graph ownership is deliberately not torn down: if an externally retained document actor still owns the graph, shutdown skips graph retirement. The live actor law drops its storage/handle clones and explicitly calls Database shutdown, while the focused VCS law performs nine changes, a checkpoint, and bounded terminal close.

The consuming Database shutdown nonclaim is resolved at source level by the borrowed retained shutdown state machine documented in `📓️sol-database-retained-shutdown.md`; its separate native laws remain pending. The generic actor future still closes rather than restores on panic. Payload GC remains deferred. No durable Map publication or retained Home/WGPU qualification is implied.

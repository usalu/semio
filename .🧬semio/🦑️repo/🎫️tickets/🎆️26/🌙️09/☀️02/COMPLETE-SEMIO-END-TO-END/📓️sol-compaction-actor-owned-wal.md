# Actor-Owned WAL Compaction

Status: source-complete for the bounded document-actor integration on 2026-09-05. The registered language-neutral source oracle is green. Rust compilation and the two registered native laws remain queued behind the shared DB migration and an explicit build-cache handoff.

## Implemented boundary

- `ArtifactMessage::Compact` is a command-priority mailbox message. `ArtifactEngine` executes it with its existing mutable `ArtifactWal`; neither the live path nor the database facade attempts to acquire a second writer permit.
- The retained compaction path derives the document and authoritative active segment from that WAL. Its only destructive WAL capability is `ArtifactWal::delete_compacted_sealed_segment`, which rejects the active segment, verifies the target is sealed, and stamps deletion with the WAL's private retained writer.
- The compaction lease is acquired before snapshot-floor discovery, committed-WAL horizon scanning, payload tracing, or deletion. Every ordinary success, cancellation, and error result attempts lease release before returning the engine to the actor.
- `Database::compact_document` and `ArtifactHandle::compact` now queue through the live document actor. The standalone retained compaction machinery remains available only for its retained-operation tests and opens/closes its own WAL before and after its lease-owned work.
- Test-only `Compactor` and all raw compaction fixtures now use explicit writer permits. Fixtures release those permits before opening or reopening an `ArtifactWal`, and actor test segment-list result owners are explicitly retired.

## Laws

The strict neutral committed-effects fixture and schema now also fix the actor boundary: command priority, retained-WAL writer authority, WAL-derived active segment, lease-before-effects order, and queued-submit behavior.

The registered native group contains:

- `db_compact::tests::compaction_applies_only_committed_frontier_snapshot_and_payload_effects`
- `db_engine::tests::compact_document_uses_live_actor_writer_and_restores_submits`

The new actor law creates enough bounded public commands to rotate the real WAL, proves an independent writer conflicts before, during the post-compaction state, and after cancellation, publishes a snapshot floor, compacts a sealed predecessor without deleting the actor's active segment, and proves a later command is accepted by the same actor.

## Receipt and nonclaims

```text
NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-os-kernel:wal-committed-compaction-check --skip-nx-cache
session 48472, exit 0
wal-committed-compaction-independent-oracle: abort effects excluded, committed effects retained, header-only highest preserved
```

This is an AJV/source oracle, not a Rust compile or native runtime receipt. The neutral fixture states queued-submit behavior, while the current native law proves pre/post command acceptance and cancellation restoration; it does not deterministically park a concurrent submit mid-compaction. The generic actor future still terminally closes the authority on a panic instead of restoring its engine, so no panic-restoration claim is made. No durable Map publication or retained Home/WGPU qualification is implied by this slice.

# Hub Checkpoint Projections

## Scope

This P2-B packet implements the append-only checkpoint publication and retention contract for the hub directory. It deliberately does not implement the plugin catalog/blob-store adapter (P2-A2), bootstrap WebSocket production (P2-C), physical pruning (P2-D), or MCP.

Configured durable directory backends are SQLite, PostgreSQL, and Neo4j. There is no selectable filesystem directory backend. `MemoryArtifactProjection` remains the dependency-free in-memory projection/oracle used for invariant and atomic-fold laws.

## Contract

- `artifact.checkpoint-published` and `artifact.retention-advanced` are public directory events with structural `(spaceId, documentId)` scope.
- Public checkpoint events and read models contain SHA-256 integrity metadata and byte lengths, but never private storage locators or `storageKey`.
- Checkpoint publication is absent from client `DirectoryCommand` and from the retention-only server command enum. Only `DirectoryService::publish_verified_artifact_checkpoint(System, ArtifactCheckpoint)` may publish.
- `HubVerifiedCheckpointPublisher` implements P2-A2's `VerifiedCheckpointPublisher` and maps failures through the shared fixed 4096-byte diagnostic boundary.
- A backend-only append seam commits the public event, append-only private authority journal input, public projection, and derived private locator projection in one transaction. Generic event append rejects public checkpoint events.
- The private journal survives space deletion so full-log replay can reconstruct the pre-delete checkpoint before replaying deletion. The derived private projection is removed by deletion and recreated only during replay.
- Same checkpoint id plus identical public and private metadata is idempotent. Any altered public metadata or private locator conflicts.
- Descriptor digest, canonical checkpoint identity, active parent, strictly advancing frontier, unique active head, lineage membership, monotonic retention floor, and floor-not-ahead-of-active are checked before append and represented by backend constraints/indexes.
- Public scalar values are limited to the JSON/TypeScript exact-integer maximum `2^53-1`. Private locators are limited to 4096 UTF-8 bytes.
- Lineage reads cap at 16,384, event pages at 10,000, and full replay/read materialization at 1,000,000. Exact max and max+1 laws are fixture-backed.

## Backend atomicity

SQLite uses one `BEGIN IMMEDIATE` transaction. PostgreSQL uses one SQL transaction and a singleton head row incremented transactionally; the event primary key is plain `BIGINT`, not `BIGSERIAL`, so rollback cannot consume sequence authority. Neo4j uses one `Txn` and its singleton counter node. Each backend stores public checkpoint payload separately from the private authority journal and projection.

Rebuilds count and refuse over-limit logs before deleting projections. They then replay fixed 512-event pages cursoring by the last observed event sequence, check cancellation and report monotonic progress after each event, reconstruct private locators from the append-only journal, and commit only on complete success.

The REST read-model and directory WebSocket replay callers use one shared bounded paging helper. A replay backend/limit failure closes the socket with code 1011 and reason `directory_replay_failed`; it is never converted to an empty successful replay. The helper still materializes a bounded suffix and has no progress/cancellation surface, so it is not claimed as the final long-log P2-C transport design.

## Language-neutral oracle

`🌎️hub/📇️directory/🧪️tests/🔣️artifact-checkpoint-projection.json` contains two non-ASCII checkpoint lineage entries, retention metadata, and the fixed caps. The Rust projection laws consume it. A permanent Vitest uses AJV against the public directory JSON schema and independently reconstructs both canonical checkpoint identities with Node `crypto` SHA-256. It also proves that adding `storageKey` makes the public event invalid.

## Evidence

- `bun nx run os-hub-ts:test --skip-nx-cache -- -t 'neutral checkpoint event'`: passed, 1 passed / 4 skipped, 2.76 seconds. AJV public event/storage-key rejection, Node identity oracle, and neutral cap vectors passed.
- The obsolete pre-simplification Cargo/rustc processes were terminated by their exact owned process IDs. After P2-A2 removed the unrelated heavyweight development graph, `cargo test --manifest-path 🌎️hub/📦️packages/🦀️rust/Cargo.toml --all-features artifact_checkpoint_publication_is_atomic_bounded_idempotent_and_replayable --no-run` passed. Both the library and binary test targets compiled, including SQLite, PostgreSQL, and Neo4j sources.
- `directory::tests::artifact_checkpoint_publication_is_atomic_bounded_idempotent_and_replayable`: passed, 1 passed / 0 failed. This is the focused SQLite runtime proof for system-only publication, public/private atomicity, exact idempotency and altered-metadata conflict, storage-key-free serialization, lineage, retention, append failure, cancellation rollback, and replay.
- `directory::tests::sqlite_restart_and_rebuild_restore_exact_private_authority_locators`: passed, 1 passed / 0 failed in 0.14 seconds.
- `directory::tests::artifact_public_scalars_and_private_locators_obey_exact_max_plus_one_laws`: passed, 1 passed / 0 failed in 0.01 seconds.
- `directory::tests::memory_projection_is_atomic_and_fixed_caps_reject_max_plus_one`: passed, 1 passed / 0 failed in 0.01 seconds.
- `artifact_authority::tests::authority_diagnostics_are_utf8_safe_and_fixed_bounded_before_retention`: passed, 1 passed / 0 failed in 0.01 seconds, covering the shared error mapper used by `HubVerifiedCheckpointPublisher`.
- `tests::complete_directory_event_reads_cross_the_fixed_page_boundary_without_gaps`: passed, 1 passed / 0 failed in 0.01 seconds. The synthetic page source records the exact capped requests `(0, 10_000)` then `(10_000, 10_000)` and returns all 10,001 events in order.
- PostgreSQL and Neo4j opt-in runtime integration could not be run: `docker info --format '{{.ServerVersion}}'` failed because `/Users/ueli/.docker/run/docker.sock` does not exist. Their production sources compile under `--all-features`; no live-database result is claimed.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/{🦀️.rs,🟦️.ts,🔣️.json}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/{🦀️.rs,🟦️.ts}`
- `🌎️hub/📇️directory/🦀️.rs`
- `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs`
- `🌎️hub/📇️directory/🐘️postgres/🦀️.rs`
- `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs`
- `🌎️hub/📇️directory/🧪️tests/🔣️artifact-checkpoint-projection.json`
- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`
- `🌎️hub/📦️packages/🟦️typescript/{package.json,🧪️index.test.ts}` and `bun.lock`

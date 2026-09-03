# Hub Authority Integration Review

Date: 2026-09-03  
Ticket: `26/09/02/COMPLETE-SEMIO-END-TO-END`  
Scope: coordinator review of concurrent P2-A2/P2-B implementation and the next P2-C seam

## Outcome

The public checkpoint model is correctly storage-key-free, and the immutable DB payload adapter correctly distinguishes public SHA-256 integrity from the DB payload store's private BLAKE3 address. Five integration invariants still have to be closed before P2-C can treat publication as durable authority:

1. Plugin and package identity cannot be collapsed. `PluginGraph::manifest` is keyed by plugin id, while `PackageRef` carries package id/hash and `DocumentOwner` deliberately stores distinct `plugin_id` and `package_id`. The adapter needs an explicit live plugin-to-package binding and independent mismatch laws for both identities.
2. Publication has one irreversible commit point. No cancellation/deadline check may turn a successful durable publisher commit into an error afterward; final progress is best-effort or part of the committed result.
3. The public directory event cannot replay the DB-private BLAKE3 locators by itself. A private append-only authority journal must be committed with the public checkpoint event and must rebuild the exact pack/SPR locator projection without exposing either locator through the directory wire.
4. Shared JSON/TypeScript integers are capped at `2^53 - 1`; Rust must reject larger frontier/time/length values before SQLite/PostgreSQL/Neo4j `i64` conversion. Unchecked `as i64` conversion is not a contract.
5. Controlled projection rebuild must count/refuse before retaining the event log and must replay in fixed pages with cancellation/progress at bounded intervals. Loading the whole log before applying the one-million-event ceiling defeats the bound.

## Source Evidence

- `PluginGraph` stores manifests in `BTreeMap<String, PluginManifest>` under `manifest.plugin_id` and resolves `manifest(plugin_id)`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:6200-6338`.
- `PackageRef` contains only `package: PackageId` and `hash: PackageHash`: the same host file at `:505-508`.
- The canonical P2-A1 descriptor fixture uses different plugin and package strings, so equality is not an admissible hidden rule.
- The concurrent adapter currently looked up `graph.manifest(&package.package.0)` and required `manifest.plugin_id == package.package.0`; its catalog test varied package hash/version/kind/schema/codec hash but not plugin/package independently.
- `CheckpointPublicationOrchestrator::publish_candidate` currently awaited `publisher.publish` and then called fallible `context.report(Published)`, allowing cancellation after publication to return an error.
- `PublishedArtifactBlob` has only `sha256` and `byte_length`; `PublishedArtifactCheckpoint` contains no `storage_key`: `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:435-455`.
- SQLite/PostgreSQL/Neo4j checkpoint projections currently persist only the public hashes/lengths and public serialized payload. No replay source retains the private BLAKE3 locator.
- The shared JSON schema caps relevant numeric values at `9007199254740991`, while projection code currently casts Rust `u64` values with `as i64`.
- Each backend's controlled rebuild currently collects all directory events into `Vec<DirectoryEvent>` before calling `checkpoint_projection_rebuild`, which is where the one-million-event ceiling and cancellation are checked.
- The generated plugin registry is a real catalog source and includes plugin id, crate path, WASM output and, where generated, SHA-256 hashes. The existing run host also resolves committed package descriptors and builds a real `PluginGraph`; however the current tree has no demonstrated hub boot path that loads every registered descriptor/codec. P2-C must not claim a complete trusted catalog from the DAG-only adapter test.

## P2-C Entry Conditions

P2-C may begin once focused evidence proves:

- exact distinct plugin/package/hash binding through a production codec;
- pack-stage, SPR-stage, both readbacks, cancellation, and publisher failure never expose a public checkpoint;
- success cannot be reported as failure after the commit point;
- public serialization contains neither `storageKey` nor a private locator;
- restart/rebuild restores the private locator journal and public active checkpoint consistently;
- SQLite runtime passes the atomic commit/restart laws; PostgreSQL/Neo4j compile and report real-service runtime as unavailable rather than green-skipped when infrastructure is absent.

The next independent audit should census the generated registry, committed descriptors, executable registered codecs, and package hashes to define the honest headless hub catalog boot path. P2-C then owns live `HubState` wiring, existing artifact-bootstrap frame production, selection/barrier/tail ordering, reauthorization during transfer, and explicit `rebootstrap-required` close behavior on both lagged broadcast loops.

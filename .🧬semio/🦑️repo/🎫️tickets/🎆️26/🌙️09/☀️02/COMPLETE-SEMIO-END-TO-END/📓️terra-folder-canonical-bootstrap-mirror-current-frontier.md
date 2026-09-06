# Folder Canonical Bootstrap Mirror Frontier

Read-only audit on 2026-09-06. No route, source, or test was changed or run.

## Actual endpoint and current boundary

The browser worker maps a folder binding directly to:

```
GET|PUT /semio-backbone?uri=folder://<binding.path>&documentId=<document id>
```

at [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1540) and [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1648). The server is the dev Vite plugin, not Hub or DB: [vite-plugins.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🔌️vite-plugins.ts:226). A folder request opens `<folder>/.semio/documents.db`; the current PUT decodes the bundle then performs an unconditional SQLite upsert into `document` ([lines 127-151](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🔌️vite-plugins.ts:127)). It has no owner, expected generation, descriptor/checkpoint binding, body limit, or final-publication fence.

The native folder actor is not a reusable replacement for this endpoint. `FolderEventLogStorage` is an append-only native process owner with only a same-process mutex ([sync.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:4132)), and `FolderTextStorage` directly overwrites several files ([lines 4403-4442](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:4403)). Neither carries a browser lease or can fence the Bun SQLite route. The closest first-party *semantic* reference is `DbIoTask::CatalogCas`, which checks an `EpochFence`, writes a staged temp file, fsyncs, renames, and syncs the parent directory ([storage.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:8128) and [lines 7830-7861](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:7830)). It is a different Rust DB backend and must not be imported or treated as a folder-route capability.

## Smallest coherent owner-fenced mirror

Keep the generic `GET|PUT /semio-backbone` path unchanged for ordinary pure-folder documents and existing local snapshots. Add a **folder-only, canonical-bootstrap-specific** sibling under the same Vite plugin; do not accept `file://`, `remote://`, a caller-supplied arbitrary destination, a raw Hub lease, or a general write grant.

The route needs three server-owned operations, all keyed by the already-declared `(folder URI, documentId)`:

1. `reserve`: the Vite process transactionally increments a per-document `epoch`, mints an opaque 32-byte capability itself, and records the immutable expected `(schema, descriptorDigestV1, aggregateSha256, full baseline frontier)`. A new reserve atomically retires the prior epoch.
2. `stage`: accepts one bounded document-pack bundle only with the capability and epoch. Decode exact framing, reject trailing/invalid lengths, cap at `ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES` (64 MiB; the actual browser bootstrap ceiling in [replication.ts](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🟦️.ts:790)), and recompute the SHA-256 aggregate over `pack || spr`. Store it only as a non-current stage after exact expected aggregate equality.
3. `publish` / `retire`: each is one SQLite transaction guarded by `(document_id, epoch, capability, state)`. `publish` makes the staged pair visible only if the reservation is still current; `retire` makes that epoch invisible and removes its staged payload. A stale publish must return conflict, never overwrite a newer epoch.

Use new tables rather than altering the generic `document` row: `canonical_bootstrap_owner(document_id PRIMARY KEY, epoch, capability, state, expected metadata)` and `canonical_bootstrap_stage(document_id, epoch, pack, spr, aggregate)`. Folder GET resolves the current published canonical row first. A reserved or retired canonical row must not fall back to an older generic row: otherwise a just-retired stale pair remains observable. A later ordinary local snapshot can explicitly retire this canonical mirror before using the legacy generic write path; that preserves mixed folder/Hub operation instead of silently disabling the folder binding.

The browser retains only the server-minted capability inside `DocumentArtifactBootstrapOwner`; it never posts it to UI/plugin code. Capture performs `reserve`, asserts the local owner before and after it, and retires the exact capability on a stale result. The final `publish` request is made only while that owner is current. Every local transition that invalidates this owner must first make the server epoch non-current (or reserve its successor) before exposing the new runtime as active; merely aborting a fetch or asserting after PUT is not a final fence. This is the necessary cross-await ordering that the present direct PUT cannot supply.

## First executable corpus

Add a schema-first `canonical-bootstrap-folder-mirror/v1` corpus under the OS folder fixtures and execute it against the real `bun:sqlite` helpers in [the existing dev test runner](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:6668), which already creates isolated temporary SQLite files through `backboneDbHandleFor`. Register it through the existing `@semio-tech/framework-os-dev:test` target ([project.json](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json:42)).

Required rows:

- A reserves, stages and publishes an exact validated pair; GET yields A.
- B reserves before A publishes; late A publish conflicts and GET never yields A; B stages/publishes and GET yields B.
- A has published, then B reserves; GET does not return A during B reservation; A stage/publish/retire cannot affect B.
- wrong capability, wrong epoch, mismatching aggregate, malformed/trailing bundle, and max-plus-one body leave no visible stage or row.
- client invalidation during stage or publish retires the exact capability; its late server completion cannot become current.
- a pure-folder generic PUT/GET remains unchanged; a mixed binding performs explicit canonical retirement before the next generic local snapshot.

This is a Vite/Bun local-mirror durability slice only. It does not publish a Hub checkpoint, authorize document access, or replace the Hub lease/descriptor checks already added to the browser bootstrap owner.

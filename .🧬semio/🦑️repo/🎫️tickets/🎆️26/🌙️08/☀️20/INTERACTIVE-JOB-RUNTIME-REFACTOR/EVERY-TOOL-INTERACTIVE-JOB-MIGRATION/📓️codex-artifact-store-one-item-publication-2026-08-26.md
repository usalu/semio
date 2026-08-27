# Artifact Store One-Item Publication

Date: 2026-08-27  
Status: durable and ephemeral lifecycle laws pass; production libraries compile natively and for `wasm32-wasip2`.

## Outcome

The mounted typed-command publisher no longer invokes `ArtifactStore::apply_one` for document,
config, or draft output. Each durable mutation now requires an explicit app-owned
`ArtifactStoreOneItemPreparationFactory`, transfers its exact mutation/base owners into a retained
`ArtifactStoreOneItemPreparation`, and remains mounted as `pending_artifact_publication` through
bounded preparation, fixed authority validation, atomic move-only root installation, result-page
retry/ACK, and incremental close.

Generic `Mutation::inverse`, `Mutation::diff`, diff apply, mutation encoding, whole-snapshot clone,
backbone pumping, and outbound encoding are never invoked by `begin_apply_one` or
`advance_apply_one`. A domain must prebuild the semantic `Edit`, post-snapshot `Arc`, clock, and
digest. The Store mints an immutable `ArtifactStoreOneItemLiveAuthority` from its exact live cursor,
revision, generation, actor, and clock; it validates the domain result against that authority and
owns persisted cursor publication. Domains can no longer fabricate or copy cursor base IDs.
Absence of domain preparation authority rejects admission and returns the exact mutation and
description owners.

## Capacity And Publication Laws

- The retained store initializer already installs capacity for all 64 fixed history entries in its
  applied/redo identity and revision ledgers.
- One-item publication does not reserve or grow those vectors. It checks for already-installed
  spare capacity and fails closed at saturation without changing generation, revision, or roots.
- Store initialization installs fixed applied/redo ledgers for both history and the persisted cursor.
  Publication moves the prepared cursor ledger, appends exactly one prepared ID, and never asks a
  domain to reconstruct history.
- Generation and the full 32-byte base revision are checked at admission and before every turn.
- Publication is a separate phase and installs an already-built `Arc<P>` by move.
- The store result remains `AwaitingAck`; retry is capped at two. ACK transitions into incremental
  close, and the shell becomes terminal only after the domain owner reports terminal emptiness.
- Close grants at most one item to the domain owner and preserves the byte grant. No vector clear or
  bulk owner drop exists in the publication close body.

## Ephemeral And Route Boundaries

- Non-document history lanes are rejected until they have a retained side-lane-map builder.
- An attached backbone is rejected until it has retained inbound/outbound page factories.
- Presence and transient use retained preparation/publication owners, generation checks, retry/ACK,
  displaced-root retirement factories, bounded close, and terminal-empty witnesses.
- No existing app receives publication authority from the global Store type alone. Each app must
  override the exact lane factory and Presence/Transient local-root retirement methods.
- `ArtifactEditor` and `ArtifactViewer` expose the applicable preparation hooks and
  `EditorApp`/`ViewerApp` forward each exact owner.
- Each app-owned tool declares an exact publication-lane contract. Activation caches all five
  preparation factories once; absent declared-lane authority rejects before command admission,
  and an emitted undeclared lane is rejected.
- Before every pending publication advance, the publisher revalidates the document revision and
  operation generation. A stale turn begins bounded close without advancing the lane.

## Fixture And Oracle

`artifact-store-one-item-publication-v1.schema.json` defines the language-neutral contract.
`🧪️artifact-store-one-item-publication-v1.json` covers empty, single, maximum,
maximum-plus-one item, maximum-plus-one byte, stale generation, stale revision, cancellation,
retry/ACK, retry exhaustion, interrupted close, and terminal-empty outcomes.

The focused Rust law parses the fixture through third-party `serde_json`, independently computes
admission/freshness/retry outcomes, anchors the history capacity to the fixed VCS ledger, and rejects
generic reducers, allocator growth, whole-root cloning, and bulk close shortcuts in the production
advance/close bodies.

## Verification State

Exact commands and outcomes are in `🧪️codex-one-item-publication-validation-2026-08-27.txt`.

- Both component files were formatted.
- Both strict Draft-2020 fixtures pass third-party Ajv 2020 strict validation.
- Nx native host check, direct native plugin check, and direct `wasm32-wasip2` plugin check pass.
- The four cfg(test) DummyApp call sites that produced eight E0277 diagnostics now consume the
  synchronous `app.snapshot()` result directly. Plugin lib-test compilation succeeds past them.
- The hostile document-stale-between-ephemeral-turns law executes and passes: one test passed,
  407 filtered out in the latest source state.
- The 38 shared Store lib-test compile errors were repaired in their test/runtime support regions.
  The Presence/Transient retained publication law now executes and passes, including Transient
  publish/receipt/ACK/root identity and displaced-root retirement after bounded close.
- Durable single retry/ACK, stale/saturation/cancel, and Drop terminal-empty witness laws execute and
  pass. The production-source anchor and a broader transaction cache law also execute and pass.
- The live-authority API is covered by real Store initialization, publication, ACK/cancel, and close;
  no demo-only injected base-ID path remains.
- Live authority and prepared-envelope fields are sealed. Apps read authority through immutable
  getters and call `prepared_edit_digest` or the preferred `prepare_one_item` constructor. The
  Store recomputes that same canonical digest before publication, so an altered candidate digest
  fails without changing root, generation, revision, cursor, or history. Apps do not depend on
  `CursorRevisionAccumulator` or `semio-framework-hash`.
- Final direct native and `wasm32-wasip2` plugin checks pass after the production API change.

## Changed Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `artifact-store-one-item-publication-v1.schema.json`
- `🧪️artifact-store-one-item-publication-v1.json`
- `artifact-ephemeral-one-item-publication-v1.schema.json`
- `🧪️artifact-ephemeral-one-item-publication-v1.json`
- `🧪️codex-one-item-publication-validation-2026-08-27.txt`
- this report

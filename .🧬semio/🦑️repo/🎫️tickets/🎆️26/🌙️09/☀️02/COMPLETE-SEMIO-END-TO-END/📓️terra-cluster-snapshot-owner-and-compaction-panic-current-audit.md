# Cluster Snapshot Owner Transfer And Compaction Panic Audit

## Scope and evidence

Read-only audit on 2026-09-05. No build was started. The snapshot path is current source in [cluster `🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🌐️cluster/🦀️.rs:206). The compaction observation is the existing native receipt `🗑️generated/exact-cargo-laws-2V4Mai/00`: its first two selected laws passed, and its third exited `SIGABRT`.

## Snapshot replication: current result

The new success ordering is correct and materially better than source-to-follower transfer:

1. `replicate_document` selects the snapshot plan at [cluster `🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🌐️cluster/🦀️.rs:245).
2. `replication_snapshot_input` copies the read result into a newly reserved page owner and drains the source result before returning the input ([`🦀️.rs:255-264`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🌐️cluster/🦀️.rs:255)).
3. Only then does it call follower `write_generation` ([`🦀️.rs:246-248`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🌐️cluster/🦀️.rs:246)).

The registered Rust law uses 36,000 bytes and verifies distinct source/copy operations, exact resulting bytes, final read closure, and follower writer reacquisition ([`🦀️.rs:674-716`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🌐️cluster/🦀️.rs:674)). `DbIoPages::close_step` returns pages, then shell credit, then result handback before its terminal witness ([storage `🦀️.rs:1609-1629`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:1609)). Thus the all-success path has the intended independent-owner and no-data-loss property.

### P0 — copy failure/cancellation loses the exact independent writer into generic deferred retirement

`db_io_copy_page_owner` returns a borrowing `DbIoPageOwnerCopy`, whose only error result is `DbError` ([storage `🦀️.rs:827-831`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:827), [`:1096-1099`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:1096), [`:1174-1204`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:1174)). If `write_fragment` or `seal_retained_step` fails, or the helper future is dropped after a partial copy, its writer is only handled by `DbIoPageWriter::Drop`, which parks it in the global lost-owner queues ([storage `🦀️.rs:770-793`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:770)). The source is then closed by the helper on ordinary error, but the exact copy writer is not returned to the caller or retired by that same transfer state machine. A cancellation while the async helper is suspended has the same issue for both the source `DbIoPages` and the writer; `DbIoPages::Drop` also falls back to global lost-owner parking ([storage `🦀️.rs:1633-1656`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:1633)).

This is not a demonstrated byte-loss path in the concrete backends, but it fails the requested *exact-owner* failure/cancellation guarantee and can leave retirement dependent on unrelated maintenance fairness/pressure.

### Smallest correction

Do not make the borrowing helper's `Drop` the replication cleanup contract. Add a private, owned `ReplicationSnapshotInput` transfer cursor in `db_cluster`, holding `source: Option<DbIoPages>`, `copy: Option<DbIoPageWriter>`, cursor/phase, and a retained cleanup phase. It should:

1. pre-reserve the copy writer before touching the source;
2. copy one source page per poll and preserve the writer on every error;
3. after copy sealing, drive source `close_step` to its exact terminal witness before exposing `DbIoPages` to the follower;
4. expose `cancel/close_step` that drains copy then source with fixed progress, rather than relying on their `Drop` fallbacks;
5. return either `Ready { input }` or one retained failure owner that must be retried/closed. No self-referential borrowed future is needed.

The current `SnapshotStorage::write_generation` consumes `DbIoPages` and returns only `DbError` ([storage `🦀️.rs:4710-4718`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4710)). `MemoryStorage`/`FsStorage` happen to route through `DbIoTaskOperation::finish`, which waits task retirement on fault, but that is an implementation detail, not a trait ownership contract. The clean schema is a retained write result: rejection before task admission returns the untouched `DbIoPages`; an admitted post-I/O fault returns a dedicated retained operation owner with `retry/close_step`. This prevents a future storage implementor from silently delegating input disposal to Drop.

### Required first laws

Extend the neutral `replication` fixture/schema at [fixture `🔣️.json:5`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🧪️fixtures/🔣️.json:5) and [schema `🧬️.schema.json:9`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🧪️fixtures/🧬️.schema.json:9), then add native laws alongside the current transfer law:

1. **copy fault after first page:** follower write is never invoked; exact copy writer and source result become terminal through the transfer owner; follower snapshot remains absent.
2. **cancel while copy is partial:** close opportunities deterministically drain both owners; all page/task credits return; no generic lost-owner queue is required for the test to finish.
3. **source-close fault after a successful copy:** copied input is terminally retired, follower write is never invoked, and the source close owner remains explicit/retryable.
4. **destination rejection:** no success `SnapshotTransferred`; destination returns an explicit retained write failure or unadmitted input; after its documented close/retry, source/copy/destination owners are all terminal and existing follower bytes are unchanged.
5. **36,000-byte success:** retain the existing byte-exact test, but record the source operation, copy operation, source terminal witness, destination admission, and destination terminal witness in the native test itself.

The current Bun source gate only makes a synthetic `Buffer` copy and compares two fresh `Symbol` values ([Rust package `📜️script.ts:81-92`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:81)); it cannot establish actual `DbIoPages` operation identity or source retirement. It is useful fixture validation, not runtime proof for this boundary.

## Compaction receipt: immediate blocker, owner WGPU

`exact-cargo-laws-2V4Mai/00` passed:

- `db_compact::tests::compaction_applies_only_committed_frontier_snapshot_and_payload_effects`
- `db_compact::tests::document_compaction_retains_shared_and_private_cas_without_global_reference_authority`

It then aborted in `db_engine::tests::compact_document_uses_live_actor_writer_and_restores_submits`, before this law can establish the live-actor compaction behavior. The first reported error is:

```
Internal("vcs: validation failed: edit history insertion requires its exact mutation retirement factory")
```

The guard is real: `ArtifactStore::reserve_edit_history_slot` rejects a missing mutation retirement factory at [Store `🦀️.rs:14593-14603`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:14593). Unwinding then drops a nonterminal VCS `ArtifactStore`, producing a second destructor panic/abort. Therefore this receipt is **not** evidence for or against compaction's data mutation: live-actor compaction is currently unqualified.

`VcsVersionGraph::store` already installs the correct catalog on its build branch ([engine `🦀️.rs:4673-4680`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:4673)). The first correction must trace why a later dispatch observes it absent; do not weaken Store's factory requirement. Independently, [`ArtifactCodec::apply_ops_binary_impl`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:9549) creates and dispatches an `ArtifactStore` without `install_member_store_owners_exact(P::member_store_owners())`. That is a definite separate product-code factory-loss path, although this audit cannot prove it is the receipt's route.

### Bounded WGPU repair and qualification packet

1. Add a direct retained VCS lifecycle law that performs the large sequence used by the failing live-actor law (nine changes plus checkpoint/snapshot phase), asserting each lease reacquires a store with exact member owners. The current `vcs_store_installs_exact_history_owners_before_first_mutation_and_closes_bounded` covers only the first mutation.
2. Route every `ArtifactStore::new` that can dispatch through the one exact owner catalog installation before its first dispatch, including the codec thunk above.
3. On any VCS dispatch failure, make the test/graph close the store through bounded `SpaceMember::close_owned_step` before returning the error, so failure reports its primary `DbError` rather than an abort from `ArtifactStore::Drop`.
4. Rerun the existing live-actor law. It must reach all assertions in [engine `🦀️.rs:11660-11696`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:11660): held actor permit, safe deletion of sealed predecessors, same actor writer restored, cancelled maintenance leaves later submit possible.

I sent the receipt diagnosis and these bounded VCS facts to `/root/complete_wgpu_home`; that agent owns the correction. The current compaction panic-recovery laws at [compact `🦀️.rs:2723-2806`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:2723) are separately useful but do not cover this Store lifecycle failure.

## Qualification status

- Snapshot success transfer: source-backed and covered by a registered Rust law; not re-run in this audit.
- Snapshot exact failure/cancellation ownership: blocked by the P0 retained transfer contract above.
- Live actor compaction: blocked before assertions by the captured VCS/Store lifecycle abort.

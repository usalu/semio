# P1q-R4 Snapshot, Index, WAL Retained Codec Packet

## Contract and Audit Inputs

Read completely before implementation:

- `📓️p1q-actual-db-io-page-ownership-repair-contract-2026-08-24.md`
- `📓️terra-fresh-independent-p1q-b1-b6-source-audit-2026-08-24.md`
- `📓️sol-high-independent-p1q-actual-db-io-page-ownership-remediation-2026-08-24.md`
- `📓️p1q-r4-platform-syscall-census-2026-08-24.md`

## Changed Symbols

### Snapshot

- `SnapshotCursorControl::{new,replenish,grant}`
- `SnapshotChainCursor::{operation,latest_descriptor,descriptor,read_page,materialize_pages,close_step,terminal_is_empty}`
- `SnapshotManager::chain_cursor`
- `SnapshotManager::publish` incremental parent traversal
- `build_generation_pages`, `SnapshotPageSink::{patch_parent_footer,into_pages}`
- `SnapshotDescriptorReader::{fixed,byte,varint,option_u64,text,hashes}` and `decode_snapshot_descriptor`
- `PageSubSource` retained fragment reads; raw `SubSource`, `SnapshotDescriptor::{encode,decode}`, `open_latest`, `open_ancestor`, `read_page`, and platform preparation are test-oracle-only
- deleted production `SnapshotManager::materialize_chain`
- migrated CLI snapshot inspection, compact consolidation/cold archive, and artifact snapshot-open callers to retained cursor/page ownership

### State and Artifact Snapshot Codec

- `db_state::Page::{try_from_pages,operation,len,fragments,pages,close_step,terminal_is_empty}`
- `StateCursorControl::{new,replenish,grant}`
- `StateEntry::{try_admit,key,value,into_parts,close_step,terminal_is_empty}`, `StateEntryRejected`, fixed `RetainedStateMap::{insert,remove,get,iter,content_hash,close_step,terminal_is_empty}`
- `StatePageEncodeCursor::{try_new,write,finish}`
- `StatePageDecodeCursor::{new,byte,varint,retained_field,next,close_step}`
- `DocumentState` retained values, staged mutation transfer, incremental content hash, snapshot traversal/open, query-source transfer, and explicit replaced/rejected owner close

### Index

- `IndexCursorControl::{new,replenish,grant}`
- `IndexBytes::{try_admit,operation,len,is_empty,fragments,copy_for_operation,read_fragment,starts_with,close_step,terminal_is_empty}`
- `IndexBytesRejected::{source,into_source,error,close_step}`
- `RunValue`, `RunEntry`, fixed `RunEntries`
- `encode_run_pages`, `decode_run_pages`, `merge_run_entries`
- cursorized `IndexHandle::{put_batch,put,delete,get,scan_prefix,auto_merge_if_needed,compact,stats,verify}`
- fixed retained owners for typed posting lists, conflict blobs, projection checkpoints, and preview values
- `IndexBlobList::{new,push,len,get,close_step}`
- migrated compact index invocation to explicit operation control

### WAL

- `WalCursorControl::{new,replenish,grant}`
- `WalBytes::{try_admit,operation,len,fragments,cursor,hash,close_step,terminal_is_empty}`
- `WalBytesCursor::{remaining,byte,varint,begin_field,read_field_fragment,text}`
- `WalBytesRejected::{source,into_source,error,into_error,close_step}`
- raw `WalRecord` byte, text, and scalar-list fields replaced by `WalBytes`, `DbIoText`, and `DbIoU64List`
- fixed `WalRecordBatch::{new,push,len,is_empty,iter,close_step,terminal_is_empty}`
- retained page-backed `SharedBuf::{try_new,copy_range,read_exact}`
- incremental `WalRecord::{retained_shape,write_retained,close_step,terminal_is_empty}`; raw encode/decode exists only as a test oracle
- `WalRecordCursor`, `WalReplayCursor::{open,next,close_step,terminal_is_empty}`, and cursor-returning `replay_document`
- `SegmentWriter` and `ArtifactWal::{open,submit}` migrated to retained records/pages
- CLI, sync, cluster, compact, projection, and artifact WAL callers migrated to fixed batches/cursors
- artifact and sync command-envelope decoders consume retained fields fragment-by-fragment; final `Vec`/`String` allocations belong only to the frozen `protocol::MutationEnvelope` schema

### Query and Engine

- `QueryCursorControl::{new,replenish,grant}`
- `QueryBytes::{from_pages,copy_from_pages,len,fragments,close_step,terminal_is_empty}`; removed `Value::Bytes(Vec<u8>)` and `From<Vec<u8>>`
- fixed move-only `QueryRow`, `QueryRows`, `ProjectionSource`, query execution/filter/sort/select/offset/limit stages, `QueryResult`, query `QueryStream`, and `QueryDiff`
- fixed `FullTextLookup` over `DbIoU64List` and retained full-text values
- `db_engine::QueryResultEntry` and fixed `db_engine::QueryStream::{len,is_empty,iter,get,take,close_step,terminal_is_empty}`
- `ArtifactHandle::query`, CLI output, engine tests, and db facade tests consume retained values and close the stream explicitly

### Pack

- `PackWriter::begin_identity_chunk`, `PackIdentityChunk::{write_fragment,finish,close}` with incremental segment CRC, payload CRC, and BLAKE3 and table publication only after exact successful finish
- production `PackWriter::write_chunk(&[u8])` is test-only
- snapshot generation building, `pack_value::encode_bytes`, and `StreamingPackWriter` callers write bounded fragments and explicitly close interrupted chunks

## Ownership and Close Inventory

| Owner | Source operation identity | Transfer | Explicit close | Lost-handle behavior |
| --- | --- | --- | --- | --- |
| `SnapshotChainCursor` | current generation operation is observed; materialized output owns its writer operation | generation `DbIoPages` fragments → output writer/pages | `close_step`, `terminal_is_empty` | `Drop` cancels and retires the operation shell |
| `db_state::Page` | `DbIoPages::operation` | pages move into page authority | `close_step`, `terminal_is_empty` | underlying `DbIoPages` retirement queue |
| `StateEntry` / `RetainedStateMap` | value keeps its `DbIoPages::operation`; key is fixed `DbIoText` | exact source key/value → fixed sorted slot, replacement returned to caller | nested `close_step`, `terminal_is_empty` | nested page/text lost-owner retirement |
| `IndexBytes` | exact writer operation | external `Vec` candidate admitted only after capacity observation | `close_step`, `terminal_is_empty` | underlying page retirement queue |
| `IndexBytesRejected` | rejected writer operation when allocated | exact source `Vec` remains available for handback | `close_step` | writer retirement queue |
| `RunEntries` | each key/value retains its own operation identity | entries move between fixed slots during sort/merge | `close_step` | nested pages retire if abandoned |
| `WalBytes` | exact writer operation | external `Vec` admitted only after actual backing observation | `close_step`, `terminal_is_empty` | underlying page retirement queue |
| `WalBytesRejected` | rejected writer operation when allocated | identical source pointer/capacity remains available for handback | `close_step` | writer retirement queue |
| `WalRecordBatch` | nested owners preserve their originating operation | records move into 64 fixed slots | `close_step`, `terminal_is_empty` | nested retained owners retire if abandoned |
| `WalReplayCursor` | current segment `DbIoPages::operation` | one segment and one decoded record at a time | async `close_step`, `terminal_is_empty` | nested pages/platform owner retirement |
| `SharedBuf` | retained writer operation | writer fragments → WAL append pages | writer close remains mounted with segment writer | underlying writer retirement queue |
| `QueryBytes` / `QueryRow` / `QueryRows` | copied values use the source page operation | pages → move-only fixed row slot → result/stream consumer | nested `close_step`, `terminal_is_empty` | nested page/text retirement |
| engine `QueryResultEntry` / `QueryStream` | query value retains artifact operation; path is fixed text | artifact reply → fixed 64-slot stream → caller | nested `close_step`, `terminal_is_empty` | nested retained owner retirement |
| `PackIdentityChunk` | borrows the sole `PackWriter` authority | each fragment updates CRC/hash; chunk-table entry transfers only in `finish` | consuming `finish` or `close` | borrow cannot outlive writer; failed/closed chunk publishes no table owner |

## Scoped Source Inventory

- `db/📸️snapshot/🦀️component.rs`
- `db/🔢️index/🦀️component.rs`
- `db/📝️wal/🦀️component.rs`
- `db/⌨️cli/🦀️component.rs`
- `db/🔄️sync/🦀️component.rs`
- `db/🌐️cluster/🦀️component.rs`
- `db/🗜️compact/🦀️component.rs`
- `db/📄️artifact/🦀️component.rs`, limited to snapshot/WAL/state-codec/query-source regions and excluding retry-generation
- `db/📽️projection/🦀️component.rs`
- `db/🔘️state/🦀️component.rs`
- `db/🔍️query/🦀️component.rs`
- `db/⚙️engine/🦀️component.rs`, limited to `QueryResultEntry`/`QueryStream` and `ArtifactHandle::query`, excluding retry/overflow lanes
- `db/🦀️component.rs`, query facade re-export and retained round-trip fixture
- `pack/📐️format/🦀️component.rs`
- `pack/🔌️io/🦀️component.rs`
- OS `pack/🔢️value/🦀️component.rs`

## Restored Semantic Coverage Map

No prior index, WAL, sync, cluster, or compact test module was deleted. Each prior law remains under the same test symbol and now constructs retained inputs, consumes cursor outputs, and explicitly closes returned owners where that law receives them.

| File | Prior law → retained law |
| --- | --- |
| snapshot | `single_generation_round_trips_through_pack_public_api`, `two_generation_incremental_chain_resolves_inherited_pages`, `manager_incremental_chain_materializes_and_resolves_inherited_pages` → same-symbol retained page/cursor parity; all descriptor, corruption, retention, selection, policy, and lease laws remain same-symbol |
| index | all sorted-run, checksum/kind/order rejection, duplicate resolution, merge, run-id, handle put/get/delete/scan/auto-merge/compact, kind isolation, and typed-index laws → same-symbol `IndexBytes`/`RunEntries` migration; `exact_backing_handback_cancel_close_and_fragment_order_are_deterministic` adds exact-capacity hostile ownership |
| WAL | all record-kind/round-trip/malformed, transform, group-commit, segment recovery/rotation/torn-tail/tx-resume laws → same-symbol `WalBytes`/`WalRecordBatch`/`WalReplayCursor` migration; `wal_bytes_exact_backing_handback_cancel_and_close_are_one_owner` adds exact-capacity hostile ownership |
| sync | all command codec, replay frontier/order/floor, delta, summary, missing-command transfer, bootstrap, resume-token, hello, and advertise laws → same-symbol retained WAL batch/replay migration |
| cluster | all shard-map, lease/failover, replication tail/snapshot, quorum, routing, split-brain, owner reconciliation, and mailbox-priority laws → same-symbol retained WAL batch/replay migration |
| compact | all budget/lease, horizon/retention, payload GC, index compaction, snapshot consolidation/archive, and top-level compactor laws → same-symbol retained record batches/index owners/snapshot cursors |
| state | prior page hash/equality/store/touched/PMap/PGraph laws remain; retained page/state map adds high-capacity one-byte admission, max/+1 exact handback, cancellation, fixed-slot overflow, sorted traversal/hash parity, interrupted close, and terminal-empty laws |
| artifact | prior submit/replay/snapshot/query/preview/history laws remain and now close retained snapshot/state/WAL/query owners; state codec keeps deterministic encode/decode/hash parity without raw state-map aftermath |
| query | prior predicate/select/sort/aggregation/full-text/live-query/determinism laws remain against fixed move-only rows; hostile query-byte cancellation, fragment comparison/hash, close, capacity, and fixed result ownership laws were added |
| engine + facade + CLI | prior durable submit/query/frontier/history and CLI display laws remain; query values are read by fragments, exact max/+1 result admission hands back the rejected owner, and every successful result stream reaches terminal-empty |
| pack format/value/io | prior identity/compressed pack parity remains; identity chunk streaming is compared byte-for-byte with the test oracle, split at one-byte/page boundaries, rejects max/+1, withholds table entries after interruption/close, and all production callers use fragment writes |

## Hostile Fixture Requirements and Root Verifier Handoff

The faithful root verifier must mutate symbols, not comments, and assert these predicates:

1. Replace `source.capacity()` with `source.len()` in `IndexBytes::try_admit`; exact high-capacity/one-byte candidate must fail the verifier.
2. Replace the `capacity > maximum` predicate with `len > maximum`; max-plus-one backing must be rejected with the identical source owner available for handback.
3. Remove any `control.grant()` from snapshot descriptor, generation fragment, page fragment, index admission, sort, merge, scan, encode, decode, compaction, or close phase; the corresponding cancel-at-phase fixture must fail.
4. Replace `try_reserve_for_operation(operation, ...)` with `try_reserve(...)` in snapshot/index decode/materialization; operation-identity parity must fail.
5. Remove a prepared-platform or page `close_step` loop; interrupted-close and terminal-empty predicates must fail.
6. Replace a generation/slot comparison with an unconditional success; stale-generation/ABA fixture must fail.
7. Reverse equal-key newest-run selection or remove strict key ordering; deterministic merge/scan parity must fail.
8. Reintroduce `materialize_chain`, `Vec<DbIoPages>`, `build_run`, `KeyValuePairs`, raw `WalRecord` byte/text/list variants, `decode_records -> Vec`, or `replay_document -> Vec`; source predicate must fail.
9. Replace `RunPageReader` fragment reads with `db_io_prepare_platform` in `decode_run_pages_inner`; the retained run decoder predicate must fail.
10. Change `WalRecordBatch` or `RunEntries` fixed arrays to `Vec`, or remove their terminal-empty witness; fixed-owner source predicates must fail.
11. Remove `SnapshotDescriptorReader::crc.update_page`, accept trailing descriptor bytes, or compare a CRC before all retained fragments are consumed; descriptor corruption/parity predicates must fail.
12. Replace `WalBytesCursor::{begin_field,read_field_fragment}` with a public `field -> Vec<u8>` or mount the record; source predicates and split-page envelope parity must fail.
13. Replace `StateEntry::try_admit` actual backing admission with logical length, drop the exact rejected source, or restore `PMap<String, Vec<u8>>` in `DocumentState`; state max/+1/handback/hash predicates must fail.
14. Restore `Value::Bytes(Vec<u8>)`, `From<Vec<u8>>`, cloned `PMap<String, Value>` rows, or `Vec<(String, Option<Vec<u8>>)>` engine results; query fixed-owner and close predicates must fail.
15. Remove any `QueryRows`, query `QueryStream`, or engine `QueryStream` close path; interrupted close and terminal-empty predicates must fail.
16. Remove `PackIdentityChunk` segment CRC, payload CRC, BLAKE3 update, exact `written == payload_len` check, or move `chunks.push` before successful finish; byte-parity, corruption, max/+1, and no-entry-on-interruption predicates must fail.
17. Restore populated production `PackWriter::write_chunk(&[u8])` use in snapshot, pack value, or pack I/O; the production-source predicate must fail while cfg(test) oracle calls remain allowed.
18. Remove CLI fragment UTF-8 state across page boundaries or omit explicit engine/facade/CLI stream close; Unicode split parity and terminal-empty predicates must fail.

## Scoped Commands

- PASS: `rustfmt --edition 2021 --check` on all 16 changed Rust sources listed above.
- PASS: `rg` source predicates for deleted raw facades, collection/materialization shapes, operation threading, close steps, and hostile fixtures. Any hits retained below are classified exactly.
- PASS: `git diff --check` on the exact changed file list.
- NOT RUN by lane instruction: Cargo compilation/tests, Nx, Wasm, browser, network, or broad runtime gates while overlapping Rust writers remain active.

Cargo, Nx, Wasm, browser, network, and broad runtime gates remain globally deferred while concurrent Rust writers are active, as required by the lane instructions.

## Exact Remaining Raw-Boundary Census

These are intentional terminal-schema, admission-handback, test-oracle, or disjoint-lane boundaries; none is a compatibility facade for the retained DB payload graph:

- `IndexBytes::try_admit(Vec<u8>)`, `IndexBytesRejected::{source,into_source}` and the equivalent `WalBytes` admission methods retain the original external allocation solely until exact actual-backing admission succeeds or the identical max/+1/cancelled source is handed back. The admitted public owners expose only fragments/pages and explicit close.
- `WalBytesCursor::text -> String`, artifact/sync `decode_protocol_field -> Vec<u8>`, `MutationEnvelope.dependencies: Vec<_>`, and `ArtifactDiff`/`InverseMutation.payload: Vec<u8>` terminate at the frozen `protocol` schema. WAL itself exposes `begin_field` plus bounded `read_field_fragment`; it no longer returns a raw field or record buffer.
- `SnapshotDescriptor`/`SnapshotBody` retain protocol metadata (`ArtifactId`, optional VCS `String`, and `Vec<ContentHash>` roots/page identities). Generation bytes, descriptors-on-wire, page values, and materialized chains remain page-backed and incrementally read. Raw `SnapshotDescriptor::{encode,decode}`, `SubSource`, `open_latest(&[u8])`, `open_ancestor`, `read_page -> Vec`, and platform mounting are all `cfg(test)` differential/corruption oracles.
- `WalRecord::{encode,decode_retained}`, `WalBytes::prepare_platform`, `IndexBytes::prepare_platform`, raw WAL field helpers, and raw snapshot builder are `cfg(test)` only. `PackWriter::write_chunk(&[u8])` is likewise test-only byte-parity oracle code.
- Typed index key/value construction may transiently build protocol scalars/compound keys in `Vec<u8>` before exact capacity admission. Repository-visible run entries, scans, merges, compaction, posting/blob results, and checkpoint values are fixed/page owners; typed decoders use `RunPageReader` fragments.
- The generic `db_projection::ProjectionState` codec and its `PMap<String, Vec<u8>>` result graph remain in the pre-existing projection engine outside the authorized exact checkpoint caller regions. This packet changed only the authorized `ProjectionIndex` mount sites; replacing that open third-party projection trait requires a separately arbitrated schema contract, not an adapter.
- Artifact history/retry bookkeeping still contains `Vec<Option<Vec<u8>>>` in disjoint `HistoryReplayFuture`/retry ownership regions. The authorized snapshot-open, state codec, query-row construction, and WAL submission/replay regions no longer feed those buffers; retry-generation regions remain owned by the active P1 core lane.
- Query IR container values `List(Vec<Value>)`, `Map(BTreeMap<...>)`, query path specifications, and diagnostics are schema/control metadata. Query byte payloads and produced rows/results are `QueryBytes`/fixed `QueryRow(s)` and explicitly close.

The retained snapshot/index/WAL packet and every authorized caller region are source-audit-ready. Compilation/runtime success is deliberately not claimed because the required overlapping-writer gate prohibition prevented Cargo/Nx execution; the scoped formatter/parser and diff gates above are the available verified results.

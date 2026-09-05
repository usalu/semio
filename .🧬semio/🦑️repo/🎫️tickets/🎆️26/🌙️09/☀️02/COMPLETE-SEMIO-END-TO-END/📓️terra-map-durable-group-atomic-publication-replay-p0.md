# Map Durable Three-Member Publication And Replay — Current Implementation Frontier

Audit date: 2026-09-05. This is read-only. No build or runtime was launched.

## Decision

The smallest coherent next slice is a **fixed, owned three-member durable group**: one Map parent plus its already-declared `drawing` and `value` children. It needs one fsynced, canonical group-journal decision and a shared staged *complete store root* for all three members. Do not route this through `TransactionCoordinator`, sequential `ReadDocument`/`ReadChildren`, or three independent document WAL writes: each permits a real partial state.

The repository has useful in-memory preparation machinery, but no durable group journal, replay owner, group-wide snapshot root, or production persistence hand-off. In particular, it is not accurate to call the current prepared-history mechanism durable or to claim a Map write works end-to-end.

## Current exact seam map

| Concern | Current owner and exact evidence | Reuse / gap |
| --- | --- | --- |
| Map work is exactly parent + drawing + value | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:45-56,92-164` | `GisMapCreateRegionGroupWorkV1` contains the three mutations/inverses and rejects an image member. It is typed preparation only; it has no store, journal, or persistence call. |
| Stable Map membership corpus | `✏️s/🔌️plugins/🌍️gis/🧪️fixtures/🧩️map-create-region-group/🔣️.json:1-49`, `🧬️.schema.json:1-101` | Reuse the fixed member names, image-free rule, and 64 KiB Map-work cap. The corpus does not describe crash points or durable outcomes. |
| Object-safe member preparation | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17510-17525,17695-17788` | `SpaceMember::{begin_one_item_wire_publication,prepare_one_item_publication,abort_one_item_publication}` gives exact member ownership, fixed admission, freshness checks, history-slot reservation, and cancellation. It deliberately refuses ordinary publication once group reservations exist (`17722-17725`). |
| Prepared candidate contents | `…/🏪️store/🦀️.rs:13067-13246,15268-15355,15358-15486` | A candidate has a store-minted immutable authority, typed edit, post snapshot, inverse data, and revision inputs. The current public erased seam cannot serialize/recover that authority or stage it as a group document root. |
| Existing group visibility | `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:188-245,452-535`; `…/🏪️store/🦀️.rs:2144-2196,2399-2424` | `ArtifactGroupVisibilityOwner` correctly supplies one pending/committed/aborted bit. History and cursor can select staged state from a captured decision. This is the substrate for the live flip. |
| Current live-root hole | `…/🏪️store/🦀️.rs:14312-14344,14409-14418` | `snapshot`, `snapshot_ref`, `snapshot_read`, and `snapshot_root` directly select `self.current`; they do not consult group visibility. No staged envelope/current/generation/revision root exists. A commit that changes three `current`s sequentially tears live Map rendering even if history/cursor use the shared bit. |
| Existing reservation law | `…/🏪️store/🦀️.rs:22671-22749` | Reuse as the pre-durability law. It proves two members reserve history and abort cleanly, but intentionally proves no committed group publication. |
| Existing composite coordinator is unsafe for this goal | `…/🏪️store/🦀️.rs:19389-19565` | It applies each child and then parent, stamps metadata after each dispatch, and compensates with reverse sequential undo. That is best-effort rollback, not atomic visibility or durable replay. |
| Current composed-child persistence boundary | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20324-20361,25174-25190,32304-32336` | Child envelopes are independently read and independently loaded. A parent document packet explicitly carries no children. This must not be used as the durable group write/restart protocol. |
| WAL primitive | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:423-445,536-720,1363-1522,1732-1765`; `…/🗄️storage/🦀️.rs:4561-4594` | A `WalRecordBatch` has 64 slots. One inline `WalRecord::Event` inside `ArtifactWal::submit(..., Fsync, ...)` is a single fsynced WAL transaction and returns `committed=true`. `WalStorage` itself is strictly per document/segment. |
| Existing DB replay is not group replay | `…/🛢️db/🗿️artifact/🦀️.rs:1153-1176,1242-1289,1468-1540` | `ArtifactEngine` owns one document/WAL and only applies `Command` and `Frontier` during replay; `Event` is ignored. Its current `submit` mutates one materialized engine before/around its own WAL operation. It cannot be the Map three-store coordinator without a new owner. |
| Crash infrastructure | `…/🛢️db/🧪️testkit/🦀️.rs:259-274,489-562,669-752` | Reuse `FaultStorage`, its append/torn/fsync controls, and the after-every-write harness shape. It currently asserts a one-document command prefix only. |
| Dependency direction | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml:42-61`; `…/🛢️db/📦️packages/🦀️rust/Cargo.toml:32-51` | DB depends on the OS kernel, not conversely. The durable-group value schema and store-facing sink trait belong in the kernel/store; the WAL journal implementation belongs in DB. |

## Smallest public contracts

Keep the new public surface narrow and fixed at three members; do not expose a generic `Vec` transaction API before the Map path works.

1. In `🏪️store`, add schema-owned values named along these lines:

   - `DurableOwnedGroupV1`: `schema`, `group_id`, `parent`, `drawing`, `value`, `owner_parent_ref`, and a canonical group commitment.
   - `DurableOwnedGroupMemberV1`: fixed `slot` enum (`parent`, `drawing`, `value`), exact `ArtifactRef`, expected generation/revision, immutable live authority (`operation`, actor, sequence, HLC, group id), the original typed mutation wire, a canonical **prepared-outcome recovery payload** (edit including inverse/metadata plus post-snapshot/root encoding), edit digest, post-snapshot pack digest, and post generation/revision.
   - `DurableOwnedGroupReceiptV1`: canonical anchor id, WAL transaction/segment receipt, and group commitment.

   The member record must persist the **store-minted authority** and the exact prepared outcome, not merely the input wire. Recreating it with a new clock after restart changes the edit, and `ArtifactStoreOneItemPreparationFactory` is app-owned arbitrary code (`🏪️store/🦀️.rs:13228-13246`), so rerunning it is not a valid durable-recovery premise. `ArtifactStoreOneItemPrepared` itself is pointer-sealed and contains the edit, post root, clock, actor/id state, and private seal (`13067-13202`); it has no current codec. Add one narrow, member-installed durable prepared-outcome encoder/decoder bridge, with Store alone reconstituting its private seal from the persisted authority after decoding. The post-snapshot digest remains the Store's corruption oracle. Map's present work is bounded and already emits its parent/drawing/value mutations plus inverses as canonical JSON-sized data (`GIS …/💡️inferences/🦀️.rs:155-164`), but that does not by itself encode every Store root. Keep the 64 KiB total group bound and use an inline WAL event; do not introduce a CAS payload whose ordering would add a second durability decision.

2. Add one store-owned `DurableOwnedGroupCoordinator` that accepts exactly `(&mut parent, &mut drawing, &mut value)` and a kernel-defined `DurableOwnedGroupSink`.

   Its phases are: validate declared ownership/fixed slots → begin and prepare all three retained publications → capture canonical commitments → stage all three complete roots behind one `ArtifactGroupVisibilityOwner` → call `sink.commit_fsync(record)` → only on its receipt flip the shared visibility bit → bounded adoption/retirement. On any pre-receipt failure it aborts every staged owner; after a successful receipt it must finish adoption/recovery, never compensate with user-visible undo.

3. Extend `ArtifactStore` internally with a staged **complete document root**, not another history-only side table. It must cover the envelope/history/cursor, `current`, generation, content revision, edit sequence, HLC, applied/redo ids, revision accumulator, DAG, tail-undo state, and the associated retirements. Every direct live read listed at `14312-14344`, plus erased snapshot reads and identity reads, selects through the shared captured group decision. Provide one group-read capture for a caller that needs a coherent parent+drawing+value observation; it is the cross-store companion to `ArtifactEnvelopeOwners::capture_read`.

4. Add an explicit recovery constructor on the retained one-item path. It consumes a journal member's persisted authority plus decoded prepared-outcome payload, revalidates the exact base generation/revision and fixed slot/ref, reconstitutes the private Store seal, and compares the canonical commitment before staging. It must not route through ordinary `begin_one_item_wire_publication`, because that mints a fresh clock/sequence authority. This constructor is the only new Store entry allowed to decode the member-installed outcome codec; ordinary live preparation remains unchanged.

This retains the existing object-safe `SpaceMember` boundary. The coordinator needs only a small erased `stage/recover durable group publication` addition alongside the current prepare/abort trio, rather than leaking Map, Drawing, or Value types into the kernel.

## DB/WAL owner implementation

Add a DB-side `DurableOwnedGroupJournal` owner, backed by one **canonical anchor WAL** derived from the parent reference and the fixed `parent+drawing+value/v1` schema. The journal, not `ArtifactEngine`, owns the anchor `ArtifactWal`; otherwise two independently-open `ArtifactWal`s could append to the same active segment.

Its three methods are sufficient:

1. `open_or_recover(anchor, storage, now)` opens the anchor WAL and replays only completed transaction bodies.
2. `commit_fsync(record, now)` canonical-encodes the bounded record into one `WalRecord::Event`, puts it in a one-record `WalRecordBatch`, calls `ArtifactWal::submit` with `DurabilityClass::Fsync`, and rejects a non-committed receipt.
3. `replay_committed_groups(...)` recognizes only the journal schema, validates the transaction's `TxBegin`/`TxCommit` id and `record_count`, validates membership/commitment/limits, and yields an idempotent record to the store recovery constructor.

`WalReplayCursor` yields individual records, and `ArtifactEngine::open` currently ignores `Event`; therefore the journal must buffer at most the current transaction (the existing batch bound is 64) and release its event only after the matching `TxCommit`. Do not infer a durable group from a bare `Event`, and do not make three child WAL `Fsync` calls the commit rule.

The anchor must be discoverable before opening either child. For this bounded owned Map slice, use the parent reference plus the fixed schema as the canonical locator and require a child to validate its `OwnerRef`/slot against that recovered record. A private parent-only event with no deterministic child lookup is insufficient: direct child restoration would otherwise miss a committed group.

The current plugin persistence hand-off must gain one composite load/read boundary that carries the anchor record and all three envelope states from one group-read capture. Calling `child_packs()` followed by `ReadDocument`, or replaying `LoadChildren` one entry at a time, remains explicitly non-durable for this feature.

## Blocking restart-safety correction: active-WAL reopen currently destroys the anchor

The parent-anchor design cannot yet use an `ArtifactWal` receipt as a restart-safe durable decision. `ArtifactWal::open` reads and recovers the active highest segment (`🛢️db/📝️wal/🦀️.rs:1679-1682`), then **deletes that segment** and creates a fresh segment at the same index (`1692-1693`). It replays trusted records into a new `SprWriter` (`1694-1710`) and only later commits/appends/fsyncs those reconstructed bytes (`1715-1716`). A crash after `delete_segment`, or after empty creation/partial reconstruction but before the final fsync, can erase a previously fsynced `TxBegin` + `Event` + `TxCommit` anchor. A recovered Map group could therefore disappear after its caller already received a receipt.

This is not a theoretical torn-tail path: even a clean, fully committed active segment takes the delete/rewrite branch. The existing `WalRecoveryReport` only exposes segment/record/torn-tail counts (`1216-1224`); it is not a witness that the replacement itself survived. The stronger `WalSegmentChain` verifier does compute a fully verified committed sequence, predecessor offset, and tip internally (`1237-1263,1287-1354`), but `ArtifactWal::open` currently uses the protocol recovery report and record replay before the destructive operation.

`WalStorage` has no atomic replace/rename/rotate primitive. It promises one highest unsealed active segment (`🗄️storage/🦀️.rs:4561-4569`), append and `sync` whose documented promise is only “everything appended” (`4571-4579`), active-only `truncate_tail` (`4596-4600`), and independently callable `create`, `seal`, and idempotent `delete` (`4567-4583,4602-4604`). It therefore cannot make delete+create+rewrite a single durable transition. Existing storage laws only check the in-process result of truncate after an earlier sync (`…/🗄️storage/🦀️.rs:9009-9042`). `FaultStorage` deliberately injects faults only into append/sync/CAS and passes create/truncate/delete directly to its inner backend (`…/🧪️testkit/🦀️.rs:254-286,331-387`), so the present crash harness has no reopen-lifecycle coverage.

### Preferred repair: validated protocol writer resume plus tail-only recovery

Prefer a protocol-owned `SprWriter` resume constructor and tail-only recovery over porting a new atomic replacement transaction across Memory, FS, SQLite, Postgres, Neo4j, and Fault storage. The existing writer state shows the exact minimum it must restore: `running_chain_hash`, a pending hasher seeded with that hash, zeroed pending record length/count, `next_commit_seq`, and `last_commit_offset` (`🧰️framework/🔨️modules/📡️replication/📐️format/🦀️.rs:403-410,513-529`). It must not write another header.

The public recovery data is close but insufficient as-is:

| Current public value | Exact fields | Gap for a safe generic writer resume |
| --- | --- | --- |
| `protocol::format::RecoveryReport` (`📡️replication/📐️format/🦀️.rs:543-552`) | `records_recovered`, `bytes_recovered`, `last_commit_seq`, `last_commit_offset`, `torn_tail_bytes` | Has the trusted boundary and predecessor offset, but no tail chain hash or validated writer capability. |
| native SPR `ResumeState` (`🛍️products/💻️os/🔨️modules/📡️spr/🔌️io/🦀️.rs:31-60`) | `end_offset`, `last_commit_seq`, `chain_hash` | Has the hash, but omits `last_commit_offset`, header/write flags, and is reconstructible public data rather than an opaque validation witness. |
| `SprWriter` (same replication format path above) | private sink, chain hash, pending hasher, pending byte/count, next sequence, optional previous commit offset | No current `resume`; `begin` always writes a new header (`452-465`). |

Make the resume state opaque/protocol-owned and mint it only after validation of the header, flags/version, every trusted frame CRC, every commit link/sequence, and final chain hash. It needs at least: trusted end offset, last commit offset (or an explicit no-commit marker), next commit sequence, final chain hash, and the validated original write options/header identity. `SprWriter::resume(sink, validated_state)` must require the sink to be positioned at exactly that trusted end, seed the pending hasher with the stored tip, set pending counts to zero, and reject forged/mismatched state. The DB may use its existing `WalSegmentChain` to supply the WAL-specific full-chain validation, but that type is private DB machinery; the resumable state/constructor belongs in the protocol module so it cannot be recreated incorrectly by each consumer.

Then change `ArtifactWal::open` to: validate the active committed prefix → if `torn_tail_bytes != 0`, truncate **only** to `bytes_recovered` and durably confirm that operation → restore a `SegmentWriter` over the retained prefix with `SprWriter::resume` → append future suffixes normally. It must never delete/recreate an active segment during recovery. `SegmentWriter` currently holds an empty `SharedBuf` mirror and assumes `flushed_len` starts at zero (`…/📝️wal/🦀️.rs:1525-1587`), so the implementation also needs either a bounded retained-prefix `SharedBuf` initializer or a sink that presents the stored prefix's logical offset while buffering only new suffix bytes. Do not hide this ownership/offset requirement behind a fictitious zero-copy resume.

The storage contract needs one accompanying clarification/operation: after recovery tail truncation, `Fsync` must durably establish the new active length, not merely bytes appended afterwards. The smallest compatible expression is to define `sync(document,index,Fsync)` as forcing all completed mutations of that active segment, including a preceding `truncate_tail`; if a backend cannot make that claim, add a tail-specific durable confirmation rather than pretending current `sync` covers it. This still preserves the pre-existing committed prefix if a crash occurs before the confirmation; reopening can rediscover and truncate the same tail. It is materially safer and narrower than an atomic-replace port.

An immutable-closed/new-active alternative would require a new atomic storage rotation/metadata durability contract: create successor, durably link it to the predecessor, then seal predecessor under one recoverable linearization rule. Existing independent create/seal calls neither provide that nor preserve the “exactly one active” invariant across a crash. Do not select it for the first Map durable-group slice.

## Independent SPR `resume_verified` boundary review

### Verdict

The proposed protocol boundary is sound **if it is an owned `VerifiedSprSpan` hand-off and the DB adds the binding/semantic checks below**. `RetainedSprVerification::finish` is a materially better source than the former generic recovery report: `VerifiedSprSpan` has a private constructor and exposes exactly `end`, `sequence`, `commit_offset`, `frames`, `tail`, and `chain` (`🧰️framework/🔨️modules/📡️replication/📐️format/🔎️verification/🦀️.rs:40-58,130-137`). The scanner checks the fixed retained header and, for every trusted commit, CRC/framing, commit sequence, previous offset, record byte/count, and the hash-chain tip (`186-247`). Its explicit contract correctly says that the caller retains input ownership and that a span is not input authority (`1-8,40-41,63-64`).

`SprWriter::resume_verified(sink, span)` should consume (not borrow) the span, first require `sink.position() == span.end()`, and only then build:

| Writer field | Required resumed value | Source |
| --- | --- | --- |
| `running_chain_hash` | `*span.chain()` | last verified committed tip, or the verified header hash for sequence zero |
| `pending_chain_hasher` | a fresh hasher seeded once with that same chain | a new commit covers only newly written records |
| `pending_records_len` / `pending_record_count` | `0` / `0` | the span ends at a commit boundary |
| `next_commit_seq` | `span.sequence().checked_add(1)` | reject overflow before a record/header byte is written |
| `last_commit_offset` | `None` when `sequence == 0`; otherwise `Some(span.commit_offset())` | preserves first-commit `prev=0` semantics without manufacturing a predecessor |

This exactly matches the private writer state and commit behavior at `📡️replication/📐️format/🦀️.rs:403-410,513-529`. It must write **no header**, accept a nonzero `span.tail()` (because the supplied sink contains only `[0, span.end())`), and leave the sink untouched on bad position/overflow. The existing `commit` also increments `next_commit_seq` with unchecked `+= 1` (`528`); replace that with a checked transition while changing this area, even though the retained verifier's finite record limit makes the overflow unconstructible from its ordinary corpus.

Source recheck after the proposed protocol patch landed: `SprWriter::resume_verified` now follows the owned-span, pre-write position, checked-next-sequence, chain-seeding, zero-pending, and sequence-zero/`None` rules (`📡️replication/📐️format/🦀️.rs:467-488`). The new neutral retained test covers header-only and torn-prefix cuts, exact prefix retention, offset/chain continuation, and unequal-length sink refusal (`📡️replication/📐️format/🔎️verification/🦀️.rs:284-321`; its corpus at `…/🧫️fixture/🔣️.json:22-27`). It does not substitute for the DB source-binding/lifecycle laws below. The pre-existing unchecked increment remains at current line `551` and is still worth closing in the same protocol slice.

### Required source binding and WAL validation

`sink.position == span.end` proves only length. A generic `PackSink` cannot read or hash its existing prefix (`📡️replication/🚰️source/🦀️.rs:58-68`), so it cannot distinguish “verified prefix A” from a different prefix B of equal length. Resuming A's chain into B would make the next append locally succeed and only fail when a later reader verifies the chain. Therefore the protocol API must document that it is **not storage authority**, while the DB must establish this stronger construction invariant:

1. retain the exact `DbIoPages` used to feed `RetainedSprVerification`;
2. copy only its verified `[0, span.end())` bytes into `SharedBuf` before closing that page owner; and
3. construct `SprWriter::resume_verified` from that buffer and the consumed span.

If the DB ever needs a second read (for example, a chunked source), it must re-run retained verification over the copied `SharedBuf` and compare `end`, sequence, commit offset, and chain before resuming. Do not carry a naked span across a storage read. `SharedBuf` currently has only an empty constructor and a suffix copier (`🛢️db/📝️wal/🦀️.rs:856-933`), so it needs a private bounded verified-prefix loader; the loader must prove its final position is `span.end()`.

The retained verifier is deliberately framing-only: it does not admit/interpret WAL record kinds, document id, segment index, or the prior segment tip. Those checks remain in `WalSegmentChain` (`🛢️db/📝️wal/🦀️.rs:1237-1354`), and must run for **every nonempty committed segment**, carrying the verified tip into the next segment. Do not replace that chain with the retained scanner. Before either checker, require the listed segment indices to be contiguous from their retained first index—the same compacted-boundary rule `WalReplayCursor` already enforces (`1403-1421`)—rather than trusting `last_index + 1` after a gap.

There is also a header-profile gap. The DB writer creates only `required_flags = REQUIRED_HASH_CHAIN`, `optional_flags = 0` (`935-942`), whereas the retained verifier checks magic/version/required flags/reserved bytes/CRC but not optional flags (`📡️replication/📐️format/🔎️verification/🦀️.rs:186-197`). Read the same retained header and reject a WAL whose optional flags differ from the DB segment profile; retain the existing `WalSegmentChain` check for the required flags and minor version (`📝️wal/🦀️.rs:1255-1263`).

### Correct `ArtifactWal::open` shape and edge states

For each sealed segment, retained verification must have `tail == 0`; then WAL semantic/cross-segment validation must succeed. For the active segment, validate the full input first, copy its verified prefix through the bound loader, and, only if `tail != 0`, call `truncate_tail(document, index, span.end())` followed by a durability-confirming `sync(Fsync)`. Create the resumed `SegmentWriter` with `flushed_len = span.end()`, `pending_records = 0`, and `oldest_pending_at_ms = None`; a committed prefix is neither pending nor a suffix to append again. This replaces the current destructive delete/create/replay path at `📝️wal/🦀️.rs:1679-1716`.

Two pre-commit creation cases need an explicit non-destructive branch:

- A physical zero-length active segment cannot produce a `VerifiedSprSpan` at all (the scanner rejects an incomplete header). Initialize that already-created segment with a fresh header plus its required `WAL_SEGMENT_HEADER`, commit, and fsync; do not call `create_segment` again.
- A header-only or torn-create segment produces `sequence == 0`, `end == HEADER_SIZE`, and its verified header chain. Truncate its tail if present, resume at the header, then append/commit exactly one `WAL_SEGMENT_HEADER` using the verified previous segment tip. `WalSegmentChain` correctly rejects this state as not yet a committed WAL segment (`1287-1295`), so the opener must recognize this limited pre-genesis state before invoking it.

This closes the explicitly documented current crash-harness hole for a failed genesis write (`🧪️testkit/🦀️.rs:527-533`). It also means the opener must derive `next_tx_id` only from fully WAL-validated records and use checked increment, rather than preserve the present `saturating_add` followed by unchecked `submit` increment (`📝️wal/🦀️.rs:1701-1705,1737-1740`).

### Rotation crash: a valid WAL can temporarily have no active segment

There is one further lifecycle case outside a clean/torn active resume. Rotation first fsyncs the committed old segment, obtains its tip, **seals that highest segment**, and only then calls `SegmentWriter::begin` to create the successor (`🛢️db/📝️wal/🦀️.rs:1768-1779`). A crash between `seal` and successor creation leaves the valid committed highest segment sealed and leaves **no active segment**. The current `ArtifactWal::open` has no status query: it treats the highest index as resumable, so the next submit would eventually try to append to a sealed segment. It must neither probe by mutation nor delete/recreate that committed highest segment.

The smallest coherent storage addition is a read-only, per-segment state query, for example `WalSegmentState::{Active, Sealed}` plus `WalStorage::segment_state(document,index)`. Existing implementations already retain this datum: memory has `MemWalSegment.sealed` (`🗄️storage/🦀️.rs:5429-5434,5852-5869`), FS has the `.sealed` marker (`6533,6611-6612,6854-6885`), SQLite/Postgres store a `sealed` column, and Neo4j returns it in its segment row (`🗄️storage/🪶️sqlite/🦀️.rs:23,225-257`; `🗄️storage/🐘️postgres/🦀️.rs:38,229-266`; `🗄️storage/🌐️neo4j/🦀️.rs:224-238,422-424`). It requires one new DB I/O task/result plumbing and delegation through `DbBackend`/`FaultStorage`, but no new storage transaction or destructive compatibility path.

On reopen, if the fully verified highest segment reports `Sealed`, require `tail == 0`, take its already WAL-validated final tip, then create `index + 1` as a new active segment with that tip and commit/fsync its `WAL_SEGMENT_HEADER`. If the old highest reports `Active`, take the resume/tail-only path above. The new trait documentation must allow a crash-recovered WAL to have zero active segments until `ArtifactWal::open` repairs it; the present “exactly one … active” wording (`🗄️storage/🦀️.rs:4561-4569`) is too strong across the existing rotate order. The result also composes with a second crash during successor initialization: `Active,len=0` and header-only recovery are handled by the pre-commit branches above.

`seal` itself currently has no explicit metadata-durability promise, and FS creates the marker without a directory-sync (`🗄️storage/🦀️.rs:6872-6885`). The recovery rule should be based on the state actually observed after restart: if the seal survived, make a successor; if it did not, safely resume the old committed active segment. In both cases, the old fsynced anchor remains intact. Do not pretend `sync` of the segment data certifies the marker state.

### Bounds and cancellation that the patch must preserve

The retained verifier default allows 64 MiB and 8,192 frames (`📡️replication/📐️format/🔎️verification/🦀️.rs:15-26`), neither of which is automatically a WAL limit. The DB fixed page buffer has 64 × 16 KiB = 1 MiB capacity (`🗄️storage/🦀️.rs:71-75`; `SharedBuf::try_new` at `📝️wal/🦀️.rs:875-880`), while storage currently caps an individual read/retained WAL length at 496 KiB (`🗄️storage/🦀️.rs:63-72,5849-5862,7260-7266`). The existing default rotation threshold is 512 KiB (`📝️wal/🦀️.rs:1616-1621`), so a normal append can hit the storage cap before the after-submit rotation condition. This is an existing bound mismatch; the resume work must not enlarge it silently. Either rotate/preflight before the common backend ceiling or make the retained scan/copy genuinely range-streaming with a separately bounded active-buffer contract.

At the current 496 KiB ceiling, legal minimum-sized frames can exceed the retained verifier's default 8,192-record cap (507,904 / 11 > 46,000). Configure a DB-specific retained record bound derived from the accepted active-segment byte bound; do not make a valid dense WAL unreopenable merely because it contains many small transactions. The scanner consumes caller fuel synchronously one byte at a time (`117-127`), so `ArtifactWal::open` must feed bounded page fragments, call `WalCursorControl::grant` between fragments, yield between copies/scans, and propagate cancellation/deadline without returning a live writer. Current open creates an inaccessible false cancellation flag and a fixed 30-second/1,000,000-fuel control (`📝️wal/🦀️.rs:1694-1697`); it has bounded work but not caller-observable cancellation.

### First resume-specific executable laws

1. **`spr_writer_resume_verified_matches_uninterrupted_bytes_for_every_verified_commit_and_torn_prefix`** — use the neutral retained fixture (`📡️replication/📐️format/🔎️verification/🧫️fixture/🔣️.json`) at every prefix/fuel grant, copy only `span.end`, resume, append a known record/commit, and compare byte-for-byte with an uninterrupted writer. Assert sequence, previous commit offset, and chain continuity.
2. **`spr_writer_resume_verified_rejects_wrong_sink_position_or_sequence_overflow_without_writing`** — shorter/longer sinks and an internal maximum-sequence span must fail before mutation. A header-only verified span must yield the first commit with sequence one and previous offset zero.
3. **`artifact_wal_open_binds_verified_pages_to_the_resumed_sink`** — prove clean and torn active reopen leaves the committed prefix byte-identical, appends only a later transaction suffix, and cannot substitute a same-length post-verification source. A private page-owner recovery constructor is the expected way to make the negative substitution unrepresentable.
4. **`artifact_wal_open_rejects_semantically_invalid_or_gapped_retained_segments`** — retain framing-valid streams with wrong WAL document/index/previous tip/optional flags and a missing segment index; no live writer or receipt is returned. This proves retained verification is not being mistaken for WAL admission.
5. **`artifact_wal_reopen_after_every_recovery_mutation_preserves_prior_fsynced_anchor`** — inject failure/crash before and after tail truncate, its durability confirmation, prefix copy, and first post-resume append; reopen has the prior anchor exactly once or fails closed before issuing a new receipt. Include zero-length and header-only failed-create inputs.
6. **`artifact_wal_reopen_after_seal_before_successor_creates_one_linked_active_segment`** — crash after the old highest segment is sealed and before successor creation. Reopen must preserve all committed records, query `Sealed` without probing append/truncate, create exactly `last + 1` carrying the old verified tip, and accept one subsequent transaction. Repeat around successor creation/header commit to cover the zero-length/header-only branches.

## Schema-first neutral crash corpus

Add the schema before implementation at:

`🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️durable-owned-group-v1/🧬️schema.json`

and the corpus at:

`🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️durable-owned-group-v1/🔣️.json`.

Use schema id `semio.os.durable-owned-group/v1`. Require exactly three members in canonical slot order, the Map dialect/ref coordinates, unique ids, expected root identities, wire and post-state commitment hashes, `groupId`, and expected visibility after recovery. Cases should be declarative, not hand-written Rust crash timing:

| Corpus case | Injected boundary | Required recovered observable state |
| --- | --- | --- |
| `before-anchor` | after all three preparations/stages, before journal append | all three old; no journal decision |
| `torn-anchor` | every nonempty prefix of the anchor append | all three old; torn tail reported/truncated |
| `reopen-anchor` | crash at every active-WAL recovery lifecycle boundary after a previously fsynced anchor | anchor record remains discoverable; all three new exactly once, never a disappeared receipt |
| `after-fsync-before-live-flip` | journal receipt returned, process dies before visibility bit | all three new after replay; exactly one group id/edit per member |
| `after-live-flip-before-adoption` | committed bit set, process dies while roots are still staged | all three new after replay; no duplicate history or retirement leak |
| `tampered-or-stale-member` | mismatched slot/ref/base revision/authority/wire/post digest | reject before anchor; all three old |

The fixture should also contain expected pre/post parent region, drawing node, and value list observations copied from the existing Map corpus, and `imageTouched: false`. This binds the crash oracle to actual Map semantics rather than a Demo-only counter.

## First executable laws

1. **`map_durable_group_live_read_never_observes_a_mixed_parent_drawing_value_tuple`** — stage a real Map triple, force a read at each publication/adoption turn, and assert only the all-before or all-after tuple from one group-read capture. This fails against current `snapshot()` at `14315-14344`.
2. **`map_durable_group_fsync_receipt_is_the_only_commit_point`** — a pre-receipt append/sync failure aborts all three reservations and leaves no history, cursor, snapshot, or emitted journal group; successful receipt permits the shared flip exactly once.
3. **`map_durable_group_replays_after_every_anchor_write_and_reopen_boundary`** — adapt `CrashHarness`/`FaultStorage` so every anchor write/torn prefix **and every active-WAL recovery lifecycle boundary after a prior fsynced anchor** reopens parent plus both children; expect exactly all-before before durable receipt and exactly all-after after it. The lifecycle injector must cover delete/create/truncate if they exist, not only append/sync.
4. **`map_durable_group_recovery_rejects_forged_or_stale_member_commitment_without_partial_apply`** — mutate each of slot/ref/base generation/revision/authority/wire/post pack hash. Reopen must reject the whole record and materialize neither child nor parent transition.
5. **`map_durable_group_round_trip_restores_parent_and_two_child_envelopes_from_one_record`** — kill after the fsync receipt, reload via the new composite boundary, and compare all three envelope packs, group ids, inverses, and Map rendered inputs with the successful pre-crash run.

## Reuse sequence and nonclaims

Begin with the existing two-member reservation law at `22671-22749`, lift it to the actual fixed three Map members, then add the group-root staging law before any WAL code. Reuse the WAL's single-record transaction/fsync implementation and the fault harness, but do not reuse `ArtifactEngine::submit` or `TransactionCoordinator::dispatch_group` as an atomic coordinator.

This report makes no claim that the Map is persisted, replayed, rendered, or browser-activated today. No source was modified and no build was run for this audit.

## Current Fixed-Three Coordinator And Sink Audit (2026-09-05)

This addendum is a read-only review of the newly added Store-only coordinator at `🏪️store/🧩️composition/🗄️durable-group/🦀️.rs`. Its sole sink is still the in-module `FakeJournalSink` (`2203-2213`); `rg` found no production `DurableOwnedGroupJournalSinkV1` implementation. Consequently, the source proves neither a WAL append nor a persisted Map decision yet.

### P0: `begin_commit` has an unrepresentable uncertain-after-begin result

`DurableOwnedGroupJournalSinkV1::begin_commit` consumes the canonical pack and returns either a journal owner or `DurableOwnedGroupJournalBeginRejectedV1` (`267-270`). In `StartingJournal`, any `Err` restores the pack and calls `begin_abort` (`827-837`). The type cannot prove that a sink which returns `Err` did no append, enqueue, or writer-permit mutation. A real sink that has written `TxBegin`/the decision and then loses the response would make the coordinator retire three candidates even though a subsequent recovery may find the decision.

Keep `begin_commit` an infallible, non-I/O handoff that transfers the decision and an already-held writer capability into the retained journal owner. The first operation that may have touched the journal must be `DurableOwnedGroupJournalCommitV1::advance`; its `Err` already leaves the coordinator in `Journal` (`844-846`) with the same owner for retry. If construction itself must be fallible, replace the current rejected result with a three-way transfer type which distinguishes `NotStarted { decision_pack }` from `Started { journal }`; only the former may abort.

The first sink test must have `begin_commit` record an intentional durable-start marker and then report an error. It must prove that no abort/retirement happens unless it returns an explicit `Absent` from the retained owner; then retry the same owner to a receipt. The existing `ErrorThenCommit` test begins too late—it covers only `journal.advance` (`2507-2540`).

### P0: the coordinator cannot be dropped or abandoned after it stages a store

Staging mutates each Store before a journal owner exists (`307-418`). Any stage failure sets `AbortingValue`, but returns `Err` to the caller (`794-824`); an `advance` error from the journal likewise preserves `Journal` (`845`). There is no mounted operation/retirement owner which retains the coordinator and its three Stores through that error. Dropping the bare coordinator leaves a `durable_group_root`, VCS suffix, cursor group, retained reservations, and private prepared owners in the Stores; later mutations fail via `ensure_durable_group_idle` (`🏪️store/🦀️.rs:14073-14077`) and Store/ledger drop assert instead of releasing them (`16804-16829`; `🌿️vcs/🦀️.rs:661-665`). This is a real lifecycle hole, not a normal request-cancellation result.

The production entry point must be one retained Map composition operation that owns the three Store owners, this coordinator, and the journal/writer permit. Its error result is a retryable observation, never permission to drop that operation. Before a durable receipt it may request cancellation, but must continue `advance` through the three abort phases and journal close. After receipt it may not cancel and must continue publish/adopt/ack/close. Do not try to solve this with `Drop`: the required Store and journal calls are bounded work and need their owner, grant, and scheduler.

Required law: inject a stage failure after parent staging, and separately an `advance` error after the journal attempted work; drop the request-facing future/handle, drive the mounted owner, and assert terminal emptiness of coordinator, all three Stores, VCS/cursors, reservation queues, and the writer permit. The operation must respectively show all-old after trusted absence and all-new after a receipt.

### P0: composite capture admits a torn three-store root

`capture_store_owned_three_snapshot` selects one available root and reuses its visibility (`635-658`). `captured_store_snapshot` permits a missing root while that visibility is committed, falling back to that Store's ordinary current (`620-633`). A malformed/recovery-partial state with a committed parent/drawing root and no value root therefore returns a mixed tuple without error. Normal phases happen to avoid it, but recovery and future sink integration must fail closed rather than turn a partial publication into an observable Map.

Once any root supplies the selected visibility, require every one of parent, drawing, and value to carry the exact same `Arc` until the coordinator has cleared all three roots. If roots are absent for all three, read the ordinary settled state. Any absent/different root in the middle is `InvalidFrontier`/retry, not fallback. Add the direct hostile law: committed shared parent+drawing roots with a missing value root must reject; repeat for a different visibility and for one adopted plus two unadopted roots. The normal phase test remains the all-old/all-new oracle.

### P1: publication and selector obligations need to be explicit

The shared `ArtifactGroupVisibility` flips before the three independent lease registries are published (`851-890`). Normal Store `snapshot_read` publishes its own current authority on demand (`🏪️store/🦀️.rs:14352-14363`), but `SpaceMember::snapshot_read_erased` only issues a lease and does not publish it (`17834-17837`). Thus during `PublishingParentLease`, `PublishingDrawingLease`, or `PublishingValueLease`, erased composition reads can carry a new staged owner while `commit_authority_matches` still names the prior generation/revision. This is not covered by the current composite snapshot law.

Either make erased reads publish the selected Store generation/revision before issuing their lease, or prohibit individual erased reads in the Map composition path and issue one triple read owner after all three publication slots are live. Add one law at every publication phase: a typed and erased read must either validate the exact selected generation/revision or be rejected/blocked; it must never return new bytes with a stale authority. Direct `snapshot`, `snapshot_ref`, `snapshot_owner`, `generation`, `content_revision`, `artifact_revision`, projection stamp, applied/redo ids, and `envelope_json` are already individually group-aware through the root or retained VCS decision (`🏪️store/🦀️.rs:14069-14103,14171-14177,14342-14367`; `🌿️vcs/🦀️.rs:582-648,899-900`). The Map runtime must nevertheless use `capture_store_owned_three_snapshot`, not three arbitrary individual reads, for a coherent parent+drawing+value render.

### P1: close is invoked twice on the `Absent` path

On `Absent`, `Journal` calls `journal.begin_close()` (`847-849`), then the abort path calls it again in `AbortingParent` (`930-935`). The trait does not declare `begin_close` idempotent (`259-265`); only the fake happens to accept duplication. Add `journal_close_started: bool` and one `start_journal_close` helper, or make idempotence a stated, tested port guarantee. Add a fake which faults/counts a second call; an `Absent` cancellation must close exactly once and restore all old roots.

### P1: scheduler byte credit is not enforced at sink admission

`ArtifactStoreOneItemGrant::permits_one` checks only nonzero bytes (`🏪️store/🦀️.rs:13044-13052`). The coordinator can synchronously pass a decision of up to 491,520 bytes to `begin_commit` under a one-byte grant (`678-703,778-839`), and `begin_commit` can decode/allocate it (as the fake does at `2204-2211`). The real writer sink must preflight and charge the canonical decision's exact byte length before construction, returning `Blocked` without moving the pack or creating a transaction if insufficient. Add grants at `len - 1`, `len`, and one byte; prove no `begin_commit`/writer mutation on the first and exactly one retained attempt on the latter two.

### Required staged-read and sink corpus

1. All stages `StagingParent`, `StagingDrawing`, `StagingValue`, and `StartingJournal` expose all-old through the triple capture, ordinary Store snapshot/root/revision, and envelope history/cursor reads; journal is untouched before the final stage.
2. After the first uncertain journal error, the same journal/writer owner, decision bytes, and writer permit remain retained; cancel only reaches abort after its explicit `Absent` result.
3. Receipt hash mismatch, a journal for a different parent document, and a missing/different group root each fail closed before any visibility flip; a committed receipt permits exactly the single shared visibility flip.
4. Every publication and adoption/clear turn yields either the complete old triple or the complete new triple from the triple capture, while no individual erased lease can claim stale authority for new bytes.
5. `Absent`, receipt/ack, retryable close error, and cancellation at each pre-receipt turn terminate every group root, VCS suffix, cursor root, prepared owner, displaced-retirement reservation, journal owner, and writer permit exactly once.

No source was modified and no build was run for this addendum.

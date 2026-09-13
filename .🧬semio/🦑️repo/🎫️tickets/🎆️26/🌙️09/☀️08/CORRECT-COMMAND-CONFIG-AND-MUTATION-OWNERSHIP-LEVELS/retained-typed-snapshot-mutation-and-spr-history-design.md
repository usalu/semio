# Retained Typed Snapshot, Mutation and SPR History Design

## Decision

Implement the remaining typed persistence work as two sequential ownership slices with one shared contract. First, make the Generation2d and Generation3d typed snapshot candidates and mutation builders physically admitted, owner-preserving and exactly closable. Then replace the current batch-like `RetainedHistoryDecode` and member-open history copy/replay with a retained SPR owner that feeds those admitted typed builders. This order gives SPR replay a real destination contract and avoids transferring decoded operations into another untracked `Vec`/`String` graph.

This is the next source-first plan. Its schema law is present as `retainedTypedPersistence` in the Pack schema and fixture with status `next-slice-contract`; it does not assert that the Rust implementation exists. The neutral Ajv 2020 and fast-json-patch law passed in `🗑️generated/retained-pack-typed-persistence-neutral-2.log`.

The full future scope remains required: archive ingress, recursive member publication, exact Store document/config/draft capability composition, command/history mutation ownership and live state handoff must use these owners. This plan does not rename or widen the common Store factory while the capability-level split remains under separate review.

## Current Source Evidence

### Typed snapshot candidates

`Generation2dMountedTypedSnapshotOwner` and `Generation3dMountedTypedSnapshotOwner` synchronously reserve three `Vec` stacks in their constructors. During token replay they synchronously reserve each `String`, table row vector, presence vector, sequence, JSON/DSL list and dictionary-entry list. They also push into final fixture widgets, synapses, ordered maps and Generation lists whose backing is absent from the mounted allocation ledger. Representative Generation2d sites are snapshot binary lines 152-179, 260-270, 526-530, 621-651 and 811-831; Generation3d has the matching owner at lines 171-198 and matching collection transitions.

Both typed owners close through `close_step() -> bool`. They pop logical frames, clear optional state and drop their vectors and strings without a byte grant. The mounted session therefore accounts source, catalog, value and inflater while `typed` remains a separate unreported heap graph.

The completed `Generation2dSnapshot` and `Generation3dSnapshot` are handed out as plain values. The existing Flow retirement recursively moves strings and collections, but several branches count string length or one logical item and then let vector capacity drop synchronously. It is not an exact actual-capacity retirement boundary. A retained candidate cannot lose its physical ledger at this handoff.

### Typed mutation builders

`Generation2dRetainedMutationOwner` and `Generation3dRetainedMutationOwner` have the same hidden three-stack construction and build strings, row/presence vectors, nested neural dictionaries and JSON/DSL collections through ordinary `try_reserve_exact`. Representative Generation2d ownership begins at mutation binary lines 458-544; synchronous collection sites continue through the container handlers and string transitions. Generation3d matches this layout and has its typed logical close at lines 1685-1705.

The mutation session correctly propagates the already-accepted record-body allocation and release, but its typed owner still closes through a Boolean and reports no physical bytes. A successfully decoded mutation is returned as a plain enum whose nested backing is later passed to a retirement factory without the builder's allocation ledger.

Mutation replay and fresh-snapshot initialization later allocate another graph while copying strings, JSON values, trees, ports, previews, widgets, synapses and Generation rows. Generation3d representative copy sites are mutation binary lines 2532-2750. Those copies are part of the typed mutation/application ownership boundary and cannot be called retained merely because the wire body was retained.

### SPR history

`RetainedHistoryDecode` in `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs:1449` owns an ordinary `HistoryLog`, `DictReader` and `Vec<String>` edit-id index. `HistoryLog` and every edit/change/checkpoint/alternative/conflict contain nested `String`, `Vec` and payload-byte owners. `step` is bounded by bytes and record count, but each `decode_*` call can allocate a complete record and append it without a caller-visible allocation demand. Validation constructs owned `String` diagnostics and repeatedly scans previously materialized lists.

The member-open operation in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs` first creates `Vec::with_capacity(total)` for the entire verified history span, copies every byte, then asks `RetainedHistoryDecode` to materialize the second graph. Replay clones strings, payloads, edit metadata, changes, checkpoints, alternatives, conflicts and composition pins into the Store envelope. `RetireHistoryBytes` truncates length and then drops the retained capacity without an exact physical release. The close path has the same logical-length accounting gap.

The existing `RetainedSprVerification` is a useful allocation-free framing and hash-chain gate. It does not own the semantic history graph and should remain the first stage of the new cursor.

## Shared Ownership Contract

Add reusable retained contiguous sequence, UTF-8 and presence-bit owners beside the existing framework value/PagedList primitives. Each owner has a finite logical item or byte ceiling and a separate finite actual allocation ceiling. Constructors allocate nothing. A pending typed token or SPR field cannot be consumed until the exact next request is granted. Actual capacity is observed after allocation; allocator rejection and overgrant become the sticky first compact fault while any real backing remains available for close.

The common surface is:

- next allocation demand for the currently blocking owner;
- exact reserve with unchanged owner returned on refusal/fault;
- actual allocated-byte ledger;
- next physical release demand only after earlier logical owners are empty;
- bounded close with separate logical items and physical bytes;
- terminal witness requiring no pending event, no logical item, zero capacity, zero allocated bytes and no untransferred value.

Use PagedList where stable-address, incremental page ownership is useful. Use a retained contiguous owner where the final domain representation requires contiguous `Vec` or UTF-8 storage. Do not convert a retained PagedList into a fresh untracked Vec at completion. Arithmetic for `count * size_of::<T>()`, UTF-8 byte sums, nested aggregate ceilings and platform indices must be checked before allocation.

## Slice A: Typed Candidates and Mutation Builders

### Admission

Make each Generation typed constructor allocation-free. Replace eager stack reserve with the same pre-admitted finite frame rule used by `RetainedValueCursor`. For every container count, string length, symbol expansion, JSON/DSL collection and table presence vector, retain the triggering token until its specific collection owner has backing. Query and reserve must route through the same deterministic order.

The aggregate ceiling covers all simultaneously owned typed backing: container stacks, active scalar/string owner, row and presence collections, nested dynamic values, final candidate collections and any owner metadata. A maximum single demand is separate from the aggregate total. An allocator overgrant cannot leave parsing admissible simply because the logical slot exists.

### Owner-preserving handoff

Replace plain `take_candidate`/`take_mutation` results with an admitted owner carrying the value and exact physical ledger. Handoff is atomic: either the mounted cursor retains and closes the complete owner, or a matching snapshot/mutation retirement authority accepts both value and ledger in one operation. No API may extract a plain value while silently discarding the ledger.

After handoff, the receiving Store/domain retirement owner must calculate and release the same actual capacities. Final live snapshot and mutation fields remain the allocation owners; the ledger is evidence and a ceiling, not a substitute for owning the backing. Existing Flow/playbook/neural retirement paths therefore need the narrow capacity corrections exercised by the two Generation candidates. A `String` release reports its capacity, and a `Vec<T>` reports `capacity * size_of::<T>()`, only when that backing is actually dropped after its logical elements have transferred or retired.

### Mutation application

Decoded mutation construction and later application/fresh initialization are one ownership chain. Copy helpers must become resumable admitted copy owners, or application must move already-owned fields when the mutation semantics permit it. A replay step cannot clone a whole nested value under one logical grant. Displaced state stays in an explicit retirement owner until exact close.

Slice A acceptance covers both Generation snapshot sessions and both mutation sessions, including success handoff, rejection, cancellation and displaced-value retirement. It does not yet cover SPR ingestion.

## Slice B: Retained SPR and History Replay

Build a `RetainedSprHistoryCursor` around the accepted verifier and retained semantic owners. Avoid the full-span `history_bytes: Vec<u8>` copy. The member witness should stream verified bytes or retained pages into a bounded frame cursor. If an unavoidable contiguous frame is needed, it must be a separately admitted record-body owner limited by `frame_body_bytes`, never an unreported whole-file copy.

Represent the dictionary through the shared retained symbol/scalar owner and store edit-id ordinal entries as checked spans or indices rather than cloned strings. Decode one record incrementally into retained field owners. Publish the record into retained history collections only after its framing, CRC/hash-chain position, required fields and references are valid. A partially decoded record remains one governed owner and is closable without publication.

History collections for edits, changes, checkpoints, alternatives, conflicts, cursor lists, composition pins, messages and payload bytes use the shared admitted owners. Their independent logical limits include record count, cumulative UTF-8, cumulative payload bytes, nesting/count bounds and total actual allocation bytes. `RetainedSprLimits::records` and `frame_body_bytes` remain grammar limits; neither is a physical grant.

Replay transfers owned strings and payloads where possible and feeds each binary operation to the retained typed mutation builder. It must not call the cold whole-operation decoder and then clone the result into another vector. Store envelope collections reserve through their existing bounded owners only after an exact caller grant. The history cursor retains source/history owners until each typed operation, metadata entry and reference has either transferred or retired.

Member-open propagates the current blocking allocation demand and physical release through its existing step context/retirement boundary. Work fuel counts one admitted allocation, decoded record, transferred logical item or close opportunity; it does not saturating-charge physical bytes as work. Cancellation preserves the first fault and closes active typed mutation, current record, history collections, dictionary/index, frame backing and source pages in dependency order.

Slice B acceptance includes the normal persisted document hydration path and rejected history path. Archive/member recursion and the Store capability split remain the following integration slice, using this owner rather than a batch fallback.

## Schema-First Laws

The checked-in neutral contract requires allocation-free construction, allocation-before-ingress, separate logical/UTF-8/payload/actual-byte limits, the complete typed owner inventory, atomic owner-and-ledger handoff, record-atomic non-batch SPR decode, zero/subexact refusal, sticky first fault, deterministic close order and an all-zero terminal witness. Ajv 2020 validates the schema and fast-json-patch independently applies the terminal transition.

Add native laws in this order:

1. Each typed constructor starts at zero allocation. Zero and subexact frame grants leave the first token unchanged; exact grants expose actual capacities.
2. A string split across UTF-8 bytes, a symbol expansion, a list crossing one physical page and nested JSON/DSL map/list owners all block before allocation and resume without duplicate publication.
3. Every Generation2d and Generation3d snapshot collection kind reaches the exact cold semantic value. Every mutation variant reaches the exact cold semantic mutation. The accepted cold encoders remain the byte oracle; no compatibility decoder is introduced.
4. Cancellation at stack initialization 0/1/all, partial scalar, partially filled row/presence collection, nested value and ready-before-handoff closes with accumulated actual release equal to admitted allocation.
5. Successful handoff transfers value and ledger together. Rejected handoff retains both. The receiving retirement owner releases the same capacities exactly once.
6. A multi-page SPR dictionary, edit-id table, edit payload, metadata/messages, changes, checkpoints, alternatives, conflicts and composition pins decode incrementally after exact grants.
7. Malformed framing, invalid UTF-8, duplicate ids, invalid references and an oversized record after real allocation preserve the first compact fault, reject later ingress and close every lower owner.
8. The retained SPR result matches the existing cold `HistoryReader`/history decoder, while the neutral Ajv/fast-json-patch and platform UTF-8 checks remain the independent cross-language oracle. Existing hash/CRC implementation tests stay enabled.
9. The real member-open hydration succeeds and cancellation at verification, record decode, typed operation replay and history retirement reports exact aggregate physical bytes with no whole-file synchronous drop.
10. Cold `PackFile`, codec 1, ordinary history APIs and current file formats stay enabled throughout.

## Source Order and Coordination

Implement Slice A first in the shared framework value owner primitives and the four Generation binary files. Keep the public switch atomic across both snapshot and mutation callers. Rerun the accepted Pack source/catalog/value/inflater filters after adding the shared owner type, then run both Generation mounted and mutation filters plus focused retirement laws.

After Slice A acceptance, implement Slice B in SPR history and member-open operation. Do not begin with a Store-wide factory rename. First make the history cursor and its direct member-open caller coherent, then run the retained SPR verification/history laws, the two Generation typed replay laws and the real member-open cancellation route.

Expected source inventory:

- `🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs` and a colocated retained contiguous/UTF-8 owner module;
- Generation2d and Generation3d snapshot `💾️binary/🦀️.rs` plus retained-mounted laws;
- Generation2d and Generation3d mutation `💾️binary/🦀️.rs` plus mutation ownership laws;
- Flow, neural and playbook retained retirement modules reached by those exact typed values;
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs` and retained history laws;
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs` and its member-open laws;
- Pack schema, fixture and language-neutral law already carrying the planned contract.

## Acceptance Boundary

Slice A may be described only as retained typed candidate and mutation ownership for the two Generation routes after exact handoff and retirement tests pass. Slice B may be described only as retained SPR/history ownership for the real member-open route after source, semantic collections, replay and cleanup all pass. Complete recursive document persistence additionally requires archive/member ingress, recursive publication and the correct document/config/draft capability composition. No synchronous fallback or codec-1 refusal is acceptable in any stage.

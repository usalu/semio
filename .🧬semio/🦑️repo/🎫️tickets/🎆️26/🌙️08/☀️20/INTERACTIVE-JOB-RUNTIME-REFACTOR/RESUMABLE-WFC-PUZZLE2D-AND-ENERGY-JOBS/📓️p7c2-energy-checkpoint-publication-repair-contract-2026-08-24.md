# P7c2 Energy Checkpoint and Publication Repair Contract

Date: 2026-08-24  
Owner: next Sol High source executor after P7c1 acceptance  
Status: prepared, not accepted  

## Packet Boundary

This packet replaces whole-state Energy checkpoint, restore, preview, output, and terminal
publication with retained fixed/page-backed cursors and explicit bounded channels. It starts only
after P7c1 has made every numerical/model traversal a bounded microcursor. P7c3 remains responsible
for the product-mounted action/session, live revision/generation binding, visible provisional
quality UI, and document/window/application close integration.

No Energy-owned pool, scheduler, thread, blocking wait, recursive close, unbounded channel,
third-party runtime dependency, permanent script, compatibility route, or alternate batch solver is
permitted. `Engine::run` must drive the same retained job and terminal protocol.

## Exact Current Blockers

### Checkpoint build and terminal state

- `EnergyJob::checkpoint` calls `encode_state(false)` in one granted turn.
- `encode_state` clones weather, precompute, precomputed model, complete simulation state, warmup
  maps, run-hour iterator, active timestep work, time series, meters, orders, full temperature
  history, preview, result, sizing/final metrics, and the growing output buffer. It then performs a
  whole `serde_json::to_vec` and extends another dynamic `Vec`.
- the `Complete` arm repeats the same full clone/serialization with `encode_state(true)` while also
  handing back the whole commit buffer.
- neither logical record count nor a successful serialization proves actual retained backing
  capacity, simultaneous source/candidate ownership, or bounded close.

### Restore

- `EnergyJob::from_checkpoint` receives borrowed bytes, synchronously strips a header, performs one
  complete `serde_json::from_slice`, validates identity only after allocation, and constructs the
  full job in one call.
- malformed, oversized, stale, and allocation-failing input has no retained rejected owner, retry
  token, cursor, cancel path, or exact page handback.
- restore can reconstitute dynamic graphs whose observed capacities exceed any later numerical or
  channel limit.

### Preview

- `publish_preview` allocates three dynamic zone arrays, collects and sorts every zone ID, scans all
  zone state, clones the typed preview, and encodes the temperature vector into a second dynamic
  buffer in one turn.
- the preview object contains full-zone heating/cooling arrays that the wire encoder silently does
  not encode, so typed and byte views do not have one authoritative schema.
- there is no fixed latest-wins slot, item/byte cap, replacement retirement cursor, or terminal
  ownership rule.

### Checkpoint and commit channels

- `StepOutcome::CheckpointReady` and `StepOutcome::Complete` directly expose newly allocated
  vectors. There is no lossless bounded queue authority for checkpoints/commits and no retryable
  terminal `take`/`resume`/`close` state.
- output encoding is record-cursorized only partially: section headers, dynamic strings,
  `format!("{:?}", unit)`, capacity growth, and final buffer handoff are not pre-admitted.
- progress, fault, checkpoint, and commit payloads do not share a process-wide aggregate byte cap.

## Required Owned Protocol

### Schema-first wire format

Define one Energy-owned binary schema version for checkpoint, preview, fault, and commit packets.
Every section must carry checked lengths and explicit identity: operation, canonical base revision,
generation, seed, quality tier, numerical stage, publication sequence, and chronological progress.
The same schema definition must drive native encode/decode and both Wasm targets; handwritten Rust
and TypeScript surfaces must stay consistent through permanent schema fixtures.

Do not serialize implementation layout, standard-library map layout, `Instant`, pointers, Arc
counts, allocator capacity, or transient worker/channel handles. Do not use serde/JSON for the
production mounted checkpoint or publication route.

### Exact page and aggregate admission

Introduce fixed-capacity page arenas for checkpoint build, restore input, preview, lossless
checkpoint queue, lossless terminal commit, and faults. Admission must census actual observed input
backing and the maximum simultaneous candidate/build/queue/retirement working set before transfer.
Use checked item, page, operation-byte, and process-aggregate byte limits.

MAX must succeed. Every independent MAX+1 dimension must return the exact original owner with
pointer/identity preserved and a retry token. Credits remain charged until the corresponding page,
queue slot, replacement, or rejected owner is explicitly retired. Logical `len`, estimated JSON
size, decorative token pages, and early credit release are forbidden.

### Retained checkpoint builder

Replace `encode_state` with a persistent section/subsection/record/byte cursor. Each turn may census,
copy, or encode at most one admitted scalar, fixed byte fragment, or page transition. Nested Energy
owners must be enumerated explicitly, including:

- weather and precompute state;
- zones, surfaces, CTF history, delivered energy, battery/plant state;
- active P7c1 timestep/zone/system/secondary/warmup/finalization cursors;
- chronological run-hour state and deterministic RNG state;
- previous-temperature/load maps;
- meters, series names, timestamps, values, history, summaries, sizing, environmental,
  resilience, economics, result builders, and output-encoding cursors;
- quality tier, stage, checkpoint-due state, and publication sequences.

A checkpoint becomes visible atomically only after every page is complete and the source identity
is revalidated. An in-progress builder is retained across yields/cancellation. Periodic checkpoints
and the terminal state must use the same builder and exact format.

### Retained restore

Replace `from_checkpoint` with a construction authority that owns the exact input pages and advances
through header validation, section framing, per-record validation, admitted backing construction,
and atomic job handoff. Check operation/base-revision/generation/seed before allocating body pages.
Reject duplicate/unknown/out-of-order sections, integer overflow, invalid enum values, noncanonical
ordering, truncated bodies, trailing bytes, per-field caps, aggregate caps, and schema mismatch.

On any fault or cancellation, retain the exact input plus partially built output and retire both
through bounded close. A valid restored job must resume from the exact microcursor and produce the
same subsequent checkpoint/commit bytes as uninterrupted execution.

### Bounded preview authority

Define a deliberately small, visibly provisional progress projection rather than publishing every
zone. It must include sequence, tier, stage, warmup/timestep progress, warmup convergence, facility
totals, and a fixed maximum number of representative/changed zone or surface samples. The exact
selection order must be deterministic and documented.

Build the projection through a retained cursor into one pre-admitted fixed/page-backed slot. The
process owns a latest-wins slot per live Energy operation: replacement may occur only after the old
preview is moved into its bounded retirement cursor. Typed and wire views must be generated from one
owned packet; no silently omitted fields are allowed.

### Lossless checkpoint and terminal queues

Checkpoint and terminal commit packets are not coalescible. Give each operation a fixed lossless
queue with explicit capacity and process aggregate byte authority. When the queue is full, the job
must yield with the exact completed packet retained; it must not drop, overwrite, duplicate, or
rebuild it. Consumer `take` either transfers the exact packet or leaves it untouched.

Terminal state must distinguish `Completed`, `Cancelled`, `Faulted`, and `Closing`, all tagged by
generation. Provide explicit `take`, retry/resume where semantically valid, and `close_step` APIs.
No production helper may synchronously drain until terminal.

### Prepared commit packet

The final commit must be one atomic prepared packet whose bytes are already fully materialized and
admitted before terminal publication. Terminal exposure must perform no allocation, formatting,
serialization, model traversal, numerical work, or GPU/UI work. The commit state and output must be
freshness-tagged and transferred exactly once.

## Bounded Close and Failure Semantics

Every builder, decoder, page chain, queue entry, preview replacement, fault, rejected input, result,
and terminal packet must expose a persistent close cursor. One close grant may retire at most one
semantic owner or one admitted backing page and return only its exact real credit.

Ordinary `Drop`, `clear`, `truncate`, whole map/vector replacement, panic unwind, generation
supersession, queue replacement, and handle/session loss must not recursively drop admitted graphs.
A handle lost during partial `Closing` must durably requeue the same generation and cursor. Closing
must make progress without a mounted UI consumer and must never reset a cursor or double-return
credit.

## Hostile Permanent Fixtures and Mutations

Add permanent fixtures proving:

1. exact MAX item/page/op/aggregate admission succeeds and each MAX+1 dimension rejects before
   ownership transfer;
2. rejected checkpoint/restore/preview/commit owners preserve exact pointer identity and retry;
3. one-fuel checkpoint build and restore advance at most one record/fragment/page transition;
4. cancellation, malformed input, panic, and generation supersession at every section reach
   bounded terminal-empty close;
5. checkpoint queue saturation preserves every lossless packet in order while preview replacement
   keeps only the newest generation-valid packet;
6. dropped consumer/producer/terminal handles during partial close requeue the same cursor once;
7. uninterrupted and restored execution produce byte-identical subsequent checkpoints and commit;
8. 1/2/4/default workers produce deterministic checkpoint ordering and terminal bytes;
9. stale preview/checkpoint/commit packets cannot publish or consume capacity belonging to a reused
   slot/generation;
10. no mounted Energy step contains serde/JSON, whole clone/collect/sort/encode/decode, recursive
    close, production terminal drain, or unbounded allocation/channel.

Mutations must independently remove/bypass identity checks, each cap, checked arithmetic, section
validation, one-unit cursor advance, latest-wins replacement retirement, lossless saturation
handling, atomic terminal handoff, close requeue, and credit handback. The focused mutation target
must kill every mutation.

## Acceptance Evidence

P7c2 source acceptance requires an independent Terra read-only audit of the final diff, exact
producer/consumer caller census, retained-owner inventory, and source mutation inventory. Broad
builds remain forbidden while overlapping Rust packets are active. The later serialized immutable
tree owner must capture:

- focused debug/release and strict-warning builds through repository Bun/Nx commands;
- real process WorkerPool replay at 1, 2, 4, and default workers;
- MAX/MAX+1, queue saturation, retry, malformed input, cancellation, panic/stuck-job, freshness,
  and close-drain evidence;
- allocation-pressure and process-credit return evidence;
- checkpoint/preview/commit max and p99 step timing below 8 ms, first substantive preview below
  50 ms, and active cadence below 33 ms;
- native, `wasm32-unknown-unknown`, and `wasm32-wasip2` protocol parity;
- deterministic replay and numerical/result parity across uninterrupted/restored executions.

Passing P7c2 does not close P7c or Phase 7. P7c3, P7b, and the final Phase 7 executable matrix remain
required.

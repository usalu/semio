# Scheduled Checkpoint And Tail-Barrier Blueprint

Date: 2026-09-04  
Scope: current-tree, read-only audit of the path from a durable document edit to a selected canonical artifact pair and an exact-tail reconnect. No build, launcher, runtime, or product/test source was changed for this report. “Source-backed” means only that the cited bytes exist; it is not process evidence.

## Decision

**RED — the required two-user reconnect/restart journey has no production producer or server join.** The repository has three useful, bounded primitives:

1. checkpoint construction and atomic CAS publication;
2. authenticated, fully materialized P4-B active-pair delivery; and
3. browser/native consumers that enforce an exact `required_tail_frontier` barrier.

They are disconnected. The hub constructs `HubArtifactAuthority` and
`CheckpointPublicationOrchestrator`, stores both as underscore-prefixed unused
state, and starts only a CAS sweeper. It never turns a durable accepted
document mutation into `CheckpointRequest` / `AcceptedArtifactOperation`,
never schedules publication or retention, and never calls
`VerifiedRebootstrapSource::load` from the document WebSocket path. That path
still uses the database `hello`; its stale result is a database-private
`Snapshot`, which both browser and native artifact clients reject by design.

P4-B remains **accepted only for its bounded active-pair HTTP response**; its
header carries a checkpoint baseline, not a currentness or required-tail
claim. P4-C remains a bounded MCP cache/mount boundary, not a proof of a
current document stream. Neither closes this RED.

## Current End-To-End Map

| Stage | Current evidence | Classification |
| --- | --- | --- |
| Durable document command | `handle_client_frame` persists accepted commands through `handle.submit(..., Fsync)` and then fan-outs acknowledgements/commands at [`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2836-2902`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2836). | Durable command log, but no checkpoint producer. |
| Codec/catalog readiness | Startup calls `linked_native_codec_bindings()` before configuring authority at [`bin.rs:5320-5339`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:5320); the supplier is still `Vec::new()` at [`:393`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:393). | RED prerequisite: no honest live codec/catalog selection. |
| Checkpoint authority | `CheckpointRequest` owns descriptor, scope, parent, base frontier, input pair, and ordered accepted operations at [`🌎️hub/🗿️artifact-authority/🦀️.rs:57-72`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🦀️.rs:57). Its publisher reserves reachability before blobs, verifies staged readback, then atomically publishes at [`:435-490`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🦀️.rs:435). | Correct primitive when called. |
| Producer/schedule | All `CheckpointRequest { ... }` / `AcceptedArtifactOperation { ... }` construction found by this audit is in authority tests, beginning at [`artifact-authority/🦀️.rs:680-687`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🦀️.rs:680). Hub state retains `_artifact_authority` / `_artifact_publication` at [`bin.rs:1343-1348`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1343) after construction at [`:5339-5364`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:5339). | RED. No mutation-to-checkpoint ownership or retry/recovery driver. |
| CAS maintenance | The only periodic hub task is `ArtifactCasMaintenanceSupervisor`, which sweeps orphan/leased CAS work at [`bin.rs:229-340`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:229). | Correct cleanup; not a checkpoint or retention scheduler. |
| Directory persistence | Verified publication is private-locator plus sanitised public event, with a serialized service seam at [`🌎️hub/📇️directory/🦀️.rs:1120-1135,1641-1655`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1120); SQLite deactivates the old head and projects the new one transactionally at [`🪶️sqlite/🦀️.rs:669-693,1411-1470`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:669). | Source-backed atomic publication/recovery primitive. |
| Retention | `AdvanceRetention` validates active lineage and monotonic floors at [`📇️directory/🦀️.rs:1440-1476`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1440); production uses found here are tests at [`:3483-3486,3854-3858`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:3483). | RED: no policy/scheduler invokes it. |
| P4-B selected pair | `VerifiedActiveCheckpointPairReader` verifies scope, descriptor digest, active private/public equality, lengths, blobs, and aggregate. `VerifiedRebootstrapSource::load` can bind that pair to a passed required tail at [`🌎️hub/🛰️lag-rebootstrap/🦀️.rs:211-255,314-367`](/Users/ueli/Documents/semio/🌎️hub/🛰️lag-rebootstrap/🦀️.rs:211). | Reader is source-backed; `load` has no production caller. |
| Authenticated HTTP pair | Exact route is `/spaces/{space_id}/documents/{document_id}/active-checkpoint/pair` at [`bin.rs:2407-2494,5165-5168`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2407), with bounded materialization and reauthorization. | Accepted P4-B scope; no tail/currentness claim. |
| Socket bootstrap/resync | `document_ws_v1` calls `state.db.hello` at [`bin.rs:2683,3029`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2683), not `rebootstrap.load`. Database stale hello selects `Bootstrap::Snapshot` at [`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:2480-2496`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:2480). | RED: stale/restart artifact client receives a deliberately forbidden representation. |
| Socket lag | On broadcast lag the hub sends only `RebootstrapRequired` then closes at [`bin.rs:2734-2757,3289-3310`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2734). | Notification/control exists; reconnect still returns to DB hello, so no recovery. |
| Browser restore | Browser requires `required_tail_frontier == Welcome.server_frontier`, installs pair first, rejects `Snapshot`, and rejects commands before completion at [`🟦️backbone-worker.ts:1162-1168,1196-1206,1231-1298`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:1162). | Correct consumer state machine, but no P4-B/server producer journey. |
| Native restore | Native has equivalent exact-frontier/epoch gates, clears terminal socket epochs, and rejects database private snapshots at [`🏪️store/🔄️sync/🦀️.rs:577-579,1922-1953,1963-1975,2325-2437`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:577). | Correct consumer state machine, but no P4-B/server producer journey. |
| MCP | The accepted P4-C pair cache/mount invalidates on relevant directory stream loss/events, but does not bind an exact tail/currentness stream. | Honest nonclaim; not a two-user reconnect consumer. |

## The Decisive Boundary

The server must own one atomic **checkpoint-and-tail selection boundary**. It
cannot let a client fetch P4-B separately, then call a database hello and
assume the two observations describe the same history. Nor can it expose a
database snapshot as an artifact seed.

For one `(space_id, document_id)`, the server has to:

1. acquire a document-stream fence before observing the durable head;
2. select and retain one verified active checkpoint identity under the
   directory/CAS read protocol;
3. capture one exact `required_tail_frontier` and the complete ordered tail
   from that checkpoint baseline through that frontier while buffering later
   live commands;
4. authorize before every externally observable stage; and
5. emit `Welcome::ArtifactBootstrap`, pair chunks, tail through the captured
   frontier, then `Session`, before releasing buffered live commands.

The pair and tail selection must fail closed if the selected checkpoint,
descriptor, authority, or cancellation generation changes. It may retry from
fresh selection; it may not substitute a DB snapshot or mix a pair from one
checkpoint with a tail from another.

`canonical_pair_header_payload` deliberately carries only the baseline
checkpoint identity/frontier at [`lag-rebootstrap/🦀️.rs:428-458`](/Users/ueli/Documents/semio/🌎️hub/🛰️lag-rebootstrap/🦀️.rs:428). The new required-tail value is therefore a **transient authenticated document-socket field**, not a P4-B header revision, URL query, directory event, cache key, or catalogue property.

## Schema, Event, And Privacy Boundary

Keep P4-B’s public wire and `ArtifactCheckpointPublished` unchanged. Add a
private, durable authority record rather than publishing raw operation data or
CAS locators:

```text
ScheduledCheckpointJobV1
  job_id, scope, descriptor_digest_v1
  parent_checkpoint_id, source_frontier
  lease_epoch, state, attempt, run_after_ms, deadline_ms, cancelled_at_ms

CheckpointTailBarrierV1                 // socket-local, never a directory event
  scope, selected_checkpoint_id, descriptor_digest_v1
  baseline_frontier, required_tail_frontier, stream_epoch
```

The durable request contains no public `pack`/`spr`, storage key, raw command
payload, receipt, SocketGrant, or user capability. `ArtifactCheckpointPublished`
and `ArtifactRetentionAdvanced` remain the public, sanitized directory
projections; `published_artifact_checkpoint` already strips the private
storage key at [`🌎️hub/📇️directory/🦀️.rs:1040-1053`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1040). Pair bytes are sent only on the already authenticated socket/HTTP body,
bounded and wiped after decode; they never enter logs, readiness, URLs,
telemetry, browser worker/plugin ports, or MCP resource enumeration.

The user’s role is rechecked as part of the socket’s D1/SocketGrant authority
before selection, before chunks, before tail completion, and before `Session`.
Public checkpoint discoverability never implies pair or document-tail access.

## Dependency-Ordered Implementation Packets

### P0 — Real codec/catalog closure

First implement the D0 prerequisite documented in
[`📓️terra-d0-codec-open-plan-blueprint.md`](./📓️terra-d0-codec-open-plan-blueprint.md):
derive at least one `NativeCodecBinding` from an exact verified descriptor/
build receipt and make configured hub readiness nonempty. A scheduler must not
materialize a client-selected schema or a generated marketplace row. Missing,
extra, duplicate, zero-hash, and factory/descriptor mismatch fail startup
closed.

### P1 — Durable checkpoint-request capture and scheduler

Own a serialized per-scope job store and clock. Only after the document
command’s Fsync acceptance may it enqueue/coalesce a request. The scheduler
reads a coherent descriptor, prior active checkpoint, input pair, and ordered
accepted operations through one captured frontier; it invokes the existing
authority and publisher with a deadline/cancellation/progress owner.

One `(scope, source_frontier, descriptor_digest)` has one live lease/winner.
Duplicate enqueue joins it; stale retry, changed descriptor, changed parent,
or cancelled/deadline-expired work cannot publish. Persist before each
externally meaningful transition so a restart resumes or abandons safely; a
publish commit is the sole active-head transition. The user request/caller is
not the unbounded work owner.

### P2 — Retention policy after P1 publication

After P1 is durable, drive the existing `AdvanceRetention` command from a
deterministic policy under the same per-scope authority serialisation. Advance
only after a newer verified active head is committed; retain the selected
active pair until no read lease/barrier references it. A failed release leaves
the old retained floor and retries; it never claims a newer floor or deletes
the active pair. CAS sweeping stays an independent cleanup implementation.

### P3 — Authenticated socket checkpoint-tail barrier

Replace the stale DB-snapshot branch for artifact clients with a server
`CheckpointTailBarrierV1` actor. It owns the document stream fence, verified
checkpoint selection, CAS read lease/materialization, exact tail, and bounded
pending-live queue. It uses the existing `VerifiedRebootstrapSource` only
after extending it with a selection-bound read API; its current `load` makes a
fresh active read and therefore cannot itself prove that selection stays fixed
across tail capture.

It sends no `Session`, Ack permission, or local outbox replay opportunity until
the exact tail is installed. Revocation, expiry, receiver drop, auth failure,
descriptor/catalog mismatch, pair read integrity failure, queue/byte cap,
deadline, or selected-head race cancels one transfer and closes/retries with no
partial live state. `RebootstrapRequired` becomes an instruction to enter this
path, not merely to reconnect to `db.hello`.

### P4 — Browser and native restore wiring

Preserve the existing equality barrier; it is desirable that the native
`frontier_reaches` predicate is equality, not an ordinal-only comparison.
Wire the shipped browser worker and native store to the P3 socket flow with
the full D0 runtime key. Their existing terminal clear/late-connect close
rules remain mandatory. Local mutations replay only after pair, exact tail,
and matching `Session`; failure clears bootstrap, socket actor confirmation,
and stale outbox generation without touching another space’s same-ID document.

### P5 — MCP explicit nonclaim or headless tail actor

Do not promote P4-C to currentness. Either leave MCP as the existing bounded
pair cache/mount implementation, or add an explicit headless D1+P3 consumer
with the same full scope, selected checkpoint, tail barrier, cancellation, and
typed result boundary. It must never surface raw pair bytes as a generic MCP
resource.

### P6 — Cross-process two-user acceptance

Only after P0–P4 (and P5 if MCP claims current document state), run one actual
hub process with its real persistent directory/CAS roots, two independently
authenticated clients, real generated browser worker, native direct child, and
optional headless actor. This is the first honest “restart/reconnect
collaboration” acceptance point.

**Handoff order:** `P0 → P1 → P2/P3 → P4(browser + native) → P5(optional
MCP) → P6`. P2 may be implemented after P1 in parallel with P3, but P3 must
not rely on retention until the selection/read lease law exists.

## Required Invariants And Hostile Laws

The shared neutral fixture must encode the schema/framing independently of
Rust or TypeScript production decoders. Use fixture-owned SHA-256/BLAKE3 and
strict UTF-8/reference parsing, as P4-B’s Node/AJV oracle does, and compare
the expected wire trace rather than importing a production serializer.

Positive trace: user A Fsyncs mutation `m1`; P1 publishes a verified pair at
baseline `B`; A adds `m2`; user B connects and receives pair `B`, tail through
exact frontier `T`, `Session`, then a later live `m3`. B must not observe `m3`
before `T`, and both users converge after B reconnects.

At minimum, add independent positive/negative vectors for:

- concurrent same-scope enqueue/retry: one publication/winner, monotonic
  parent/frontier, no duplicate active event;
- CAS reservation expiry/cancel before blob write, between two blobs, after
  staging/before publish, and restart after each durable job transition;
- active-head/descriptor advance or retention release racing pair selection:
  one internally consistent pair+tail or retry, never a mixed lineage;
- tail before pair completion, omitted/reordered/duplicated command, wrong
  document/frontier/hash, `T+1` live before `T`, and database `Snapshot` on an
  artifact path;
- role revocation at admission, metadata, each pair chunk, tail, and Session;
  receiver cancellation/deadline/late successful socket gets one close and no
  receipt/grant/actor/outbox use;
- two spaces with the same document ID, two actors, reconnect after hub restart
  (including `close_all_sync_sessions`), and stale resume token/session actor;
- retention never drops active/selected blobs; public directory and
  `RebootstrapRequired` traces disclose no storage locator/raw pair/receipt;
- all cardinality, bytes, chunk, operations, queue, job, retry, deadline, and
  progress-report limits; no timer spin under cancellation or unavailable CAS.

The fixture should explicitly prove the P4-B nonclaim: its canonical header
has the baseline but no `required_tail_frontier`; that value first appears in
the authenticated socket `ArtifactBootstrap` trace.

## Gates And Acceptance

Existing registered checks are useful but insufficient:

- `bun nx run os-hub:canonical-pair-check --skip-nx-cache` is the accepted
  bounded P4-B reader/route/oracle/all-feature check; launch registration is
  [`.vscode/launch.json:4477-4481`](/Users/ueli/Documents/semio/.vscode/launch.json:4477).
- `bun nx run @semio-tech/framework-os-mcp-rs:canonical-pair-check --skip-nx-cache`
  checks P4-C’s separate bounded cache/mount path.
- `os-hub:open-plan-check`, `os-hub:open-plan-server-check`, and
  `os-hub:browser-document-open-check` are registered D1-focused checks, but
  none schedules a checkpoint or drives pair-to-tail recovery.

Add a single launch-registered, owning-`📜️script.ts` target only after P0:
`os-hub:scheduled-checkpoint-tail-barrier-check`. It must first list and
require exactly one fully-qualified law per suffix, then exact-run:

1. P1 durable job/capture/restart/cancellation tests;
2. P2 directory/CAS publication and retention/read-lease tests;
3. P3 actual authenticated WebSocket two-user barrier, lag, revocation, and
   hub-restart tests against real CAS/directory roots;
4. the language-neutral fixture/oracle; and
5. the required feature-complete hub check.

Extend the existing browser and native document-open gates only after P3 to
exercise their shipped worker/direct-child paths. An unavailable codec,
generated worker, real hub, or direct child is a terminal/inconclusive failure,
never a skip/pass. Record actual process results separately from source review.

## Explicit Nonclaims Until P6

Do not claim that a durable command is checkpointed, that CAS sweeping creates
checkpoints, that P4-B means “latest document,” that `RebootstrapRequired`
repairs a connection, that a client can use a DB snapshot as an artifact
bootstrap, that P4-C provides headless currentness, or that source/gate
registration is runtime proof. The current `features.rebootstrap` readiness
bit reflects the available control/route machinery, not this complete
checkpoint-to-tail collaboration journey.

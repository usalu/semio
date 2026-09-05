# First AI-Over-MAP Proposal Slice

Current-source implementation blueprint, 2026-09-04. This is a read-only
audit: no build, route, browser, socket, or process journey was run.

## Decision

Implement one **server-owned, deterministic GIS Map proposal job** after the
trusted `stdio+gis` installation and retained document-open work are real. An
active Author may create, reconnect to, and cancel an owner-private job that
offers one typed `CreateRegion` proposal. It never mutates the map.

The public approval request is part of the same schema family, but an approval
cannot publish a document until the separate prepared composition-publication
transaction is available. This gives the required rule a hard boundary:
**there is no map mutation before an explicit typed approval, and there is no
approval-to-mutation claim before atomic publication exists.**

This is a deterministic local GIS analysis, not a remote/paid model provider,
generic tool runner, MCP command, or WGPU rendering claim.

## Current evidence and gaps

| Boundary | Current source | Consequence |
| --- | --- | --- |
| Typed MAP analysis | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:195-292` declares `s.gis.gismap.inference`; `infer_gis_map_controlled` decodes a map, checks allocation/work/depth, checkpoints each value, and returns deterministic bytes. | Reuse as the sole P0 executor, but only after a server-owned selected binding has materialized the map. |
| Typed offer | `…/🧬️schema/💡️inferences/🦀️.rs:36-52` turns that result into one `GisMapMutation::CreateRegion` and explicitly does not apply it. | Correct P0 payload. It needs a server-built typed base and a durable private proposal encoding. |
| Client surface | `…/✏️editor/🦀️.rs:223-238` has fourteen `Gis2dCommand` variants; the action decoder at `:756-800` recognizes only those variants; `render` at `:820-829` has no job panel. | No request, progress, cancellation, proposal review, approval, or EN/DE accessibility state exists. Do not represent an ordinary map mutation as a job action. |
| Trusted executable selection | `🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:24-73` links GIS codec receipts, while `✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🦀️.rs:31-104` exposes Map and Terrain **document codecs**. | Codec selection is not a selected inference executor. The global framework inference registry at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:1394-1484` is process-global and request-keyed, so it is not P0 authority. |
| Server identity | `🌎️hub/💡️inference/🧬️schema/🦀️.rs:21-87` has bounded client intent and a serializable identity containing session, scope, descriptor/catalog/package and frontier facts. `…/📇️catalog/🦀️.rs:47-76` constructs it from verified catalog, session, descriptor, frontier, and server-held input. | Strong foundation, but the identity currently calls component SHA-256 `package_hash`, lacks BLAKE3 and parent dialect, and has no executable-binding identity. Rename/add fields source-first; do not infer them from a client plan. |
| Current authorization | `…/🛂️authorization/🦀️.rs:7-30` rechecks the original session/user/generation, scope and live Author role before and after the directory read. | Reuse at submit, claim, every progress checkpoint, read, cancel and approval. It returns `()`, not a retained submission grant. |
| Durable job ledger | `…/🪶️sqlite/🦀️.rs:8-47` has an immediate SQLite job transaction, immutable events, private input/result/proposal bytes, and an approval outbox. `accept` searches global `request_id` at `:129-149`; events are max five and unique by kind at `:27-34`; `start` has no worker lease at `:152-187`. | It cannot provide scoped idempotency, durable cursor/progress, exactly-one executor, restart recovery, or a routed job. |
| Approval and proof | `InferenceApprovalRequestV1` at `…/🧬️schema/✅️approval/🦀️.rs:7-24` admits only job and proposal digest. `prepare_approval` at `…/🪶️sqlite/🦀️.rs:219-251` validates a canonical command, and WAL witness/reconciliation exist at `…/🧾️wal/🦀️.rs:24-64` and ledger `:270-287`. | Keep the small approval DTO. The command must be server-built, not received from a UI. The witness reconciles *after* Fsync; it is not a cross-store transaction. |
| Socket/document write | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2818-2838` submits caller envelopes with Fsync; `handle_client_frame` at `:2861-2923` accepts only normal commands, frontier, preview, presence and credit. Router lines `5138-5170` have no inference route. | Keep private job transport off `ServerFrame::Commands` and document fanout. There is no job runtime in `HubState` (`:1345-1419`) and readiness pins inference to false (`:1691-1733`). |
| Composition publication | `VcsArtifactApp::commit_child_member` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19956-20094` suspends between owner/graph/map/root publication. GIS re-mints content-derived drawing/value children at `…/gismap/🦀️.rs:81-105,161-171`. | Approval cannot claim atomic parent/member/graph mutation. It depends on the prepared publication packet documented in `📓️terra-composition-publication-atomicity-blueprint.md`. |

The existing `GisInferenceLedgerCheckScript` declares its own limit at
`🌎️hub/📦️packages/🦀️rust/📜️script.ts:3785-3819`: it proves foundation
laws and explicitly makes no route/GIS-approval acceptance claim.

## P0 scope and lifecycle

### 1. Server-selected binding and materialization

Create a private `VerifiedGisMapProposalBindingV1`, stored in `HubState` only
when startup has atomically accepted the server-owned trusted GIS package. It
contains no client bytes and no general registry lookup. Its constructor takes
the exact selected trusted package/descriptor and retains:

* descriptor SHA-256, component SHA-256, component BLAKE3, catalog generation,
  plugin/package/version, exact `s.gis.gismap` artifact/document/inference
  schemas and versions;
* complete `parentDialect` (`artifactKind`, `standard`, `subset`), exact
  selected surface and grant policy; and
* the one compiled `ArtifactInferenceService` function pointer whose metadata
  equals those facts, plus a first-party typed GIS Map materializer.

The materializer must obtain the current map through the retained selected
document-open/member path, at one exact server frontier, and return an owned
canonical map pack plus its typed `GisMapSnapshot` under bounded retirement.
It must not use a browser pack, raw plan receipt, global
`artifact_inference_service`, or generic `ArtifactCodec::apply_ops_binary` as
proof of a terminal typed owner. The latter is a generic parse/reset/dispatch
bridge (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:9508-9580`),
not the retained selected-document authority this path requires.

Replace ambiguous `InferenceIdentityV1.package_hash` with explicit immutable
identity fields and add the selected parent dialect. The persisted identity
must at least include the exact descriptor SHA-256, component SHA-256,
component BLAKE3, catalog generation, package/version, GIS metadata,
parent dialect, full `DocumentScope`, original user/session/authorization
generation, and the materialized frontier/input hash. Greenfield code should
rename rather than preserve the old field.

### 2. Closed public intent; all authority server stamped

Reuse `InferenceRequestV1` as the only P0 submit body:

```text
schema = semio.hub.inference-request/v1
version = 1
requestId = lowercase-hex-32
serviceId = s.gis.gismap.inference
policyVersion = 1
lifetimeMs = 1..120000
```

The route path supplies `spaceId` and `documentId`; session, user,
authorization generation, descriptor, catalog, package, dialect, surface,
grant, server wall-clock deadline, typed map base, command actor, and frontier
are derived on the server. None are accepted in JSON. The client cannot select
an executor, payload, provider URL, model, tool, artifact, mutation, actor,
or base revision.

Use the following private state machine. `stale` is a terminal job state, not
only a proposal state, so it can explain authority/frontier/catalog loss
without falsely reporting provider failure.

```text
accepted -> claimed(runEpoch, leaseUntil) -> running -> offered
   |                |                       |         |
   +--cancelled-----+--cancelled/stale-------+         +--cancelled/stale
                                                      |
                                                      +-- approved only in P1
```

`accepted`, `running`, `offered`, `failed`, `cancelled`, and `stale` are
durable facts. Only the current claim epoch may append progress or transition
terminally. First terminal wins.

### 3. Ledger changes: scoped idempotency, cursor, progress, and claim

Replace the global `UNIQUE(request_id)` at
`🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:11-47` with an immutable idempotency key:

```text
(space_id, document_id, user_id, session_id, authorization_generation, request_id)
```

with its identity digest. A retry bearing exactly the same digest returns the
same receipt. A retry under that same key with a different catalog, frontier,
or input digest returns a bounded conflict. A different scope/principal may
reuse the 32-hex request ID without seeing or conflicting with the first job.
The deterministic job id derives from this scope key plus identity digest.

Replace the five-row, unique-kind event table with append-only
`InferenceJobEventV1` rows:

```text
jobId, cursor(u64), runEpoch(u64), kind,
completedWork(u64), totalWork(u64), atMs(u64)
```

`cursor` is per job and strictly contiguous; `0 <= completed <= total <=
65536`; there are at most 64 events/job, with one accepted, claim/running,
zero or more progress, and one terminal/offered row. Keep event data free of
input bytes, proposal bytes, prompts, capabilities, actor/session identifiers,
or provider diagnostics. The owner-private read view may expose a bounded typed
`GisMapProposalSummaryV1` (counts and finite bounds) plus the proposal digest;
it must not expose the captured source pack.

Add a durable execution claim table or columns:

```text
job_id, run_epoch, lease_until_ms, cancel_requested, claimed_at_ms
```

Claim is one `BEGIN IMMEDIATE` conditional update: only accepted/running,
unexpired, non-cancelled rows with expired/no claim may move to a new run epoch.
Progress, success, and failure include that epoch in their `WHERE` condition.
`cancel` durably sets `cancel_requested`, signals only a matching in-memory
control, and prevents later output; the claimant resolves it once as
`cancelled`. No callback from a superseded epoch may append progress or offer a
proposal.

The supervisor owns a fixed number of claims, creates an
`InferenceOperationControlV1` from the **server-stamped** deadline and work
budget, and drives `infer_gis_map_controlled` with checkpoints. At each
checkpoint and before writing an offer it rechecks:

1. operation cancellation/deadline;
2. `check_live_inference_author` against the original session/user/generation
   and exact scope;
3. trusted binding identity/generation and package facts; and
4. exact live document frontier and materialized input hash.

Loss of any recheck writes `stale` or `cancelled`, wipes private input/result/
proposal, and never emits a document frame. A directory `Unavailable` is the
only retryable condition; it must retain the job but not start a parallel run.

On restart, the supervisor takes an expired claim only if the row remains
unexpired and all the same live checks and exact retained-map materialization
succeed. It then deterministically reexecutes the stored canonical input under
a new epoch. Otherwise it terminals stale/cancelled and wipes bytes. It never
trusts a persisted bearer or resumes against a different frontier.

### 4. Private transport and UI contract

Do not add inference to the document WebSocket or `ServerFrame::Commands`.
Those frames are document/presence fanout, and private job state would leak to
collaborators. Add an authenticated, owner-private HTTP family beside the
existing document routes in `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs`:

```text
POST /spaces/{spaceId}/documents/{documentId}/inference/gis-map/jobs
GET  /spaces/{spaceId}/documents/{documentId}/inference/gis-map/jobs/{jobId}?after=<cursor>&limit=<1..32>
POST /spaces/{spaceId}/documents/{documentId}/inference/gis-map/jobs/{jobId}/cancel
POST /spaces/{spaceId}/documents/{documentId}/inference/gis-map/jobs/{jobId}/approval   [P1 only]
```

Every handler authenticates first, derives a fresh `InferenceReaderV1`, runs
the live Author check, and uses a uniform bounded denial for missing, foreign,
Viewer, Share, revoked, stale-generation, expired, or cross-space jobs. Reads
are cursor-paged and reconnect by last exclusive cursor. Cancellation is
idempotent. Neither the request nor event page is broadcast to peer B,
directory streams, preview, presence, or MCP.

At the GIS surface, add an **ephemeral host side-effect port**, not a
`GisMapMutation` or shared document/config field. Its closed states are:

```text
idle | submitting | running(completed,total) | offered(summary,digest)
| cancelling | cancelled | stale | failed(code)
```

The port may issue only submit/read/cancel using an already verified client
execution-target lease. It stores job id/cursor locally, clears it when the
lease/session changes, and does not persist it into the map. Add explicit EN
and DE labels, an accessible busy/progress value, an announced terminal error,
and a labelled cancel control. There is no approval control in P0. WGPU render
and browser map activation remain RED until the execution-target lease packet
lands; a route/UI fixture is not a rendering claim.

## Explicit P1: typed approval and publication

The existing `InferenceApprovalRequestV1` is the right external shape: only
job id and offered proposal digest. On receipt, a private committer must:

1. reauthenticate and require the exact original live Author; acquire the
   same per-full-document submission guard used by a document commit;
2. revalidate binding, complete identity, lease, catalog generation, exact
   frontier, and offered proposal under that guard;
3. decode the stored proposal into `GisMapInference`, reconstruct a typed map
   base from the retained document authority, and call `bounds_proposal`;
4. build one canonical `CreateRegion` envelope and inverse server-side. Actor,
   mutation id, HLC, scope, dependencies and schemas are all derived, never
   received; and
5. wait for the prepared `parent + members + graph + root` publication boundary
   to make the normal Fsync command and composition state visible together.

Only after the real Fsync result may `InferenceWalVerifierV1` produce the
existing witness and `reconcile_committed_approval` mark the ledger approved.
After a crash in that interval, retry witness/reconciliation only—never submit
again. One ordinary document command fanout follows durable publication; peer
B sees that ordinary mutation once, but never A's job history or proposal.

This is blocked by the current async/non-atomic `commit_child_member` sequence
and GIS's content-derived child handles. Do not weaken the approval rule by
using `submit_commands` alone or an outbox success as publication proof.

## Dependency-ordered implementation packets

1. **Trusted materialization prerequisite.** Finish trusted GIS installation,
   immutable execution-target lease, and retained selected GIS Map
   materialization. Verify descriptor/component/dialect/generation before typed
   snapshot decode and terminally retire that snapshot. No job route yet.
2. **Schema and ledger.** Extend identity, scoped idempotency, event cursor,
   claim/cancel/recovery state, private proposal summary, and independent
   language-neutral corpus. Existing ledger methods are replaced rather than
   compatibility-wrapped.
3. **Private binding and supervisor.** Construct `VerifiedGisMapProposalBindingV1`
   at server startup from the trusted selected package and bind only the
   deterministic `infer_gis_map_controlled`. Mount it with the ledger and
   fixed-capacity supervisor in `HubState`; readiness remains false until all
   three plus materialization are live.
4. **Authenticated owner-private routes.** Add submit/read/cancel routes and
   no document WebSocket frames. Reuse directory/session authority and add a
   focused route/runtime gate.
5. **GIS side-effect UI.** Add request/cancel/progress/reconnect rendering only
   behind the verified lease. This is separately browser/native tested and does
   not claim WGPU renderer acceptance.
6. **P1 publication.** Only after the prepared composition transaction and
   stable GIS member model are proven, add the approval committer, Fsync/WAL
   reconciliation, normal fanout, and two-user mutation law.

## Required proof packet

### Neutral cross-language fixture

Add `🌎️hub/🧪️fixtures/🗺️gis-map-proposal-v1/{🔣️.json,🧬️.schema.json}` and
an independent Bun/AJV/Node model in the existing
`🌎️hub/📦️packages/🦀️rust/📜️script.ts`. The fixture uses canonical fixed
strings/bytes and hashes; it contains no bearer, pack, provider secret, storage
path, or model endpoint.

It must cover:

* exact request, scoped idempotency and identity/binding/frontier inputs;
* accepted → running/progress → offered with one deterministic `CreateRegion`
  digest, no map command; cancellation at every provider checkpoint; deadline,
  provider failure, zero/overflow work, malformed request/cursor, and event
  cap; and
* same-key retry, same-key/different-identity conflict, cross-scope reuse,
  Viewer/Share denial, cross-space/document denial, revoked/rotated session,
  descriptor/component/catalog/dialect mismatch, stale frontier, restart
  lease handoff, duplicate completion, owner-only read, and peer redaction.

For P1, add separate pending vectors for approval replay, concurrent document
write/frontier advance, pre-Fsync cancellation, post-Fsync crash, and
composition publication refusal. They must remain red/selected separately until
the publication dependency exists.

### Focused native and socket laws

Register source-owned `gis-map-proposal-oracle` and
`gis-map-proposal-check` in the hub `📜️script.ts`, exposed through
`🌎️hub/📦️packages/🦀️rust/📋️project.json`, with exact-one FQN preflight for
every law. Add the launch entry to its seed and regenerate the launch artifact;
do not hand-edit generated launch JSON. Intended command after implementation:

```text
bun nx run os-hub:gis-map-proposal-check --skip-nx-cache
```

The exact native laws should prove:

1. trusted selected GIS Map materialization invokes only the deterministic
   service, emits monotonic bounded owner-private progress, and offers one
   typed proposal without a `MutationEnvelope`, graph change, or fanout;
2. concurrent identical submissions produce one row/claim/run; same request id
   in another scope neither conflicts nor reveals the original job;
3. cancel, deadline, session revoke, catalog/dialect rotation, frontier change,
   executor error and stale run epoch each yield one terminal event, wipe bytes,
   and prevent later progress/result;
4. restart either obtains one new durable lease and reexecutes exact input or
   terminals stale/cancelled; it never runs two executors; and
5. the authenticated route proves cross-space, Viewer, stale-socket,
   cursor-replay and owner-read denials, while a peer socket observes no
   inference frame/event.

The protected process/browser law launches a local SQLite hub with a real
trusted GIS selection, Author A and collaborator B. It drives A's verified
surface action, reconnects from a durable cursor, cancels and receives an
accessible EN then DE state, and proves B receives no job/proposal. It also
exercises revocation, stale socket, provider failure and restart. It is not
accepted until the actual browser/native lease consumer exists; until then a
route-level process law is the honest P0 ceiling.

## Acceptance boundary

No current end-to-end AI/MAP execution is accepted. The existing GIS service,
private ledger, approval DTO, WAL proof, catalog identity and codec receipts
are foundations only. P0 becomes accepted only with the neutral oracle, exact
native/runtime gate, and owner-private authenticated process journey against
one trusted GIS binding. P1 additionally requires the prepared composition
publication law before it may claim approval-to-map mutation or collaborator
visibility. Remote models, generic tool execution, MCP inference, automatic
approval, WGPU rendering, and long-offline replay are outside this blueprint.

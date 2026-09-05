# Terra AI-Over-MAP End-To-End Runtime Audit

Current-source audit, 2026-09-04. This report is deliberately read-only: no
build, route, browser journey, or hub process was run for this audit.

## Verdict

**RED.** The tree has three useful but disconnected foundations:

1. the GIS artifact has a deterministic, caller-driven bounds calculation;
2. the hub has private SQLite ledger, catalog-identity, command-decoding, WAL
   witness, and author-check primitives; and
3. the hub now statically links GIS *document codec* receipts.

There is no authenticated MAP inference UI action, hub request/stream/cancel
route, inference runtime in `HubState`, model/provider executor, durable
progress/restart worker, typed approval/apply route, or client-visible result.
No current test demonstrates the requested two-user runtime journey. The
linked GIS provider must not be represented as either AI activation or approved
MAP mutation publication.

The narrowest honest P0 after the member-opener and Flow bootstrap work is a
**private, server-owned deterministic MAP proposal job**: Author A can submit,
cancel, reconnect to, and read a bounded `CreateRegion` *proposal* bound to a
verified GIS catalog selection and an exact document frontier. It deliberately
does **not** mutate the document or notify collaborator B. Typed approval and
atomic map/child publication are the next P1 because the current composition
publication boundary is not atomic.

## Current trace and evidence

| Boundary | Current source | Classification |
| --- | --- | --- |
| OS MAP UI intent | [`Gis2dCommand`](../../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:223) declares fourteen closed editor commands and the bridge decodes the same set at [line 765](../../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:765). None requests analysis, shows job progress/result, cancels a job, or approves a proposal. | **RED** |
| Local MAP calculation | [`infer_gis_map_controlled`](../../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:255) accepts caller-supplied input, budget, cancellation callback, and produces deterministic output; [`bounds_proposal`](../../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:38) builds a `CreateRegion` proposal without applying it. | **Source/local test only; not a provider or server executor** |
| GIS codec linkage | The hub provider set includes a `gis` entry at [`native-openable-provider`](../../../../../../../../🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:24), and [`preview_gis_bindings`](../../../../../../../../🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:52) consumes exactly the GIS codec receipts. | **Document-codec selection only** |
| Trusted catalog startup | [`configured_artifact_authority`](../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:380) returns no authority unless both server environment inputs are present. Linked receipts alone do not create a trusted catalog row, open target, or inference binding. | **Server bootstrap prerequisite** |
| Hub inference schema/ledger | [`InferenceIdentityV1`](../../../../../../../../🌎️hub/💡️inference/🧬️schema/🦀️.rs:44), [`InferenceJobLedgerV1`](../../../../../../../../🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:50), exact canonical command decoding, and a private approval outbox exist. | **Foundation only; no hub consumer** |
| Authentication/authorization | [`check_live_inference_author`](../../../../../../../../🌎️hub/💡️inference/🛂️authorization/🦀️.rs:7) rechecks active Author membership, session generation, scope, expiry, cancellation, and time after the directory read, but returns `()` rather than a retained submit grant. | **Reusable guard, not a route/worker authority** |
| Catalog-to-inference identity | [`identity_from_verified_catalog`](../../../../../../../../🌎️hub/💡️inference/📇️catalog/🦀️.rs:47) validates a caller-provided materialized input against catalog/descriptor/service facts, then returns identity only. | **Source-only identity, no executor binding** |
| Client protocol | [`ClientFrame`](../../../../../../../../🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs:48) and [`ServerFrame`](../../../../../../../../🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs:519) contain socket/document, command, preview, presence, and credit frames—not inference request, progress, result, cancel, or approval frames. | **RED** |
| Normal mutation relay | [`handle_client_frame`](../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2861) admits direct socket mutation envelopes; [`submit_commands`](../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2818) persists/fans them out. Neither accepts a job/proposal nor shares a transaction with the inference outbox. | **Not an inference or atomic composite-publication path** |
| Hub readiness | [`hub_readiness`](../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1696) hard-codes `features.inference: false`. | **RED** |

The registered [`GisInferenceLedgerCheckScript`](../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts:3785) is correctly limited by its own terminal message to “no route/GIS-approval acceptance.” Its SQLite/WAL/catalog/author/GIS-provider-selection laws are useful foundation evidence, not an end-to-end MAP acceptance.

### What is real today, and what it does not prove

`InferenceRequestV1` is closed to the GIS service, bounds request bytes and
lifetime, and `InferenceIdentityV1` carries user, session, authorization
generation, space/document, descriptor/catalog/package, and frontier facts
([schema](../../../../../../../../🌎️hub/💡️inference/🧬️schema/🦀️.rs:16)).
`InferenceReaderV1` also requires the exact user/session/generation/scope
([ledger](../../../../../../../../🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:62)). Those
are good stale-session isolation inputs. They are not currently read by a
router, worker, or UI.

The ledger uses immediate SQLite transactions, first-terminal state changes,
and an approval outbox. It has no durable progress payload/event cursor,
worker-lease claim, restart reclaim procedure, executor registration, or
route. Its request-id uniqueness is global: `accept` searches only
`WHERE request_id=?1` ([line 138](../../../../../../../../🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:138)). A different scoped caller reusing a request id gets a conflict;
that is neither correct scoped idempotency nor safe capacity isolation.

No model/provider integration is present. The only executable MAP implementation
is the deterministic local `ArtifactInferenceService`. P0 must name it “MAP
analysis/proposal,” not claim remote/model AI. A later provider interface must
remain server-owned and never accept provider URLs, credentials, arbitrary tool
names, or raw document bytes from a browser.

## Decisive runtime REDs

1. **No user action or observable result.** The closed GIS action catalog has
   no inference/cancel/approval command. There is therefore no accessible EN/DE
   status, progress, error, cancel, or private result state in either desktop
   or browser surface.
2. **No authenticated transport.** There is no HTTP router endpoint or
   WebSocket frame for job submission, event replay, cancellation, result read,
   or approval. Existing document socket credentials must not be copied into a
   background worker; the worker needs its own rechecks.
3. **No server-selected executable binding.** The current GIS catalog link
   proves codec factories only. `identity_from_verified_catalog` validates
   identity but produces no selected `ArtifactInferenceService`/executor and
   has no production caller.
4. **No durable worker semantics.** `InferenceOperationControlV1` is local
   atomics, and the ledger has no persisted progress, claim/lease, resume, or
   one-executor proof. A restart can leave accepted/running rows without a
   truthful resumption rule.
5. **No safe publication.** The proposal is not a server-built command.
   Existing direct envelope submission is outside the ledger transaction.
   Further, each GIS map mutation regenerates derived drawing/value children
   from content ([map snapshot helper](../../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:84)); it does not establish stable member identity or atomic parent/member/graph publication. P0 must stop before apply.
6. **No multiuser privacy contract.** Normal `ServerFrame::Commands` fanout is
   appropriate only after a durable document mutation. Sending private job
   status/result there would disclose Author A’s activity and result to peers.
   B may observe only the later normal mutation in P1.

## Narrowest dependency-ordered P0

### P0.0 — prerequisites and non-goals

Wait for the generic public member-opener plus Flow bootstrap work to provide a
real retained selected-document/materialization boundary. That work does not
activate GIS inference. Do not make P0 depend on a browser-supplied pack,
synthetic catalog row, a generic `ArtifactHandle::submit`, or a content-derived
child id.

P0 creates a private durable proposal only. It has no document mutation,
approval button, collaborator broadcast, remote model, or claim of complete
artifact/child publication. Those are P1.

### P0.1 — schema-first server-owned binding and job/event stream

Add a closed `VerifiedGisInferenceBindingV1` behind the trusted catalog
authority. It is constructed only from the exact selected GIS descriptor,
component hash/package/version, artifact/document schema, declared GIS service,
catalog generation, and retained materialized map input. It owns the exact
deterministic executor selection; clients submit only the closed
`InferenceRequestV1` intent.

Extend the ledger schema with bounded, append-only job events carrying a
monotonic cursor and a bounded progress payload, plus an execution claim
`(job_id, run_generation, lease_until)`. Define scoped idempotency by exact
identity/reader scope and request id, rather than the current global request-id
unique key. Terminal events are immutable and there is exactly one terminal
state. A recovery claim must either continue the same deterministic input or
terminally fail/cancel; it must never execute two providers for one job.

The private hub runtime owns a fixed-capacity supervisor. For each checkpoint it
checks cancellation/deadline, executes the selected deterministic GIS service,
appends bounded progress, then rechecks all of: current session generation,
Author role, scope, descriptor digest, catalog generation/package, and
frontier. A changed/revoked fact records `stale`/`cancelled` and wipes private
input/result/proposal rather than applying anything. `check_live_inference_author`
is the recheck primitive, not a retained grant or substitute for the per-scope
submission gate.

### P0.2 — owner-private HTTP/event transport and UI

Use a separate authenticated hub route family, not a new unfiltered document
fanout frame:

* `POST` accepts one bounded GIS inference request for the exact space/document;
* `GET` returns an owner-private bounded event page after a cursor, which is a
  reconnectable stream; and
* `POST cancel` is idempotent and gives no provider continuation after the
  cancellation checkpoint wins.

The handler derives principal, `DocumentScope`, session generation, and Author
role from the server session/directory—never from client body fields. Every
read reuses exact `InferenceReaderV1` scope matching and live-session recheck.
Cross-space, Viewer/Share, stale session, catalog/frontier mismatch, and
expired request return the same bounded denial shape without revealing job
existence. Jobs/results must never appear in `ServerFrame::Commands`, presence,
preview, or public directory events.

Add only two GIS UI actions: request analysis and cancel. Their local ephemeral
state is job id/cursor/status/progress/private proposal summary, not a shared
document config field. Both need explicit English and German labels and semantic
busy/error/cancel affordances. The reconnecting UI asks the owner-private event
route from its last cursor. It does not render an approval or mutate a map in
P0.

## P1, explicitly outside P0: typed approval and publication

The existing public approval DTO is appropriately small, but the implementation
must make its command server-built from `(verified job, verified proposal,
current typed map base)`, not accept raw client command bytes. The approval
worker must acquire the same exact document submission authority as normal
socket commands, revalidate the complete identity/frontier after acquiring it,
then make one all-or-nothing durable transaction across map parent, child/member
graph, command/WAL, job approval event, and outbox reconciliation. It must
release one ordinary document command fanout only after commit.

This must wait for the active composition transaction/stable-child work. The
current `submit_commands` path is not proof of that atomicity. When P1 lands,
Author A and collaborator B may observe the normal committed map mutation once;
only A may see the private inference result/proposal history.

## Required proof packet

### Neutral language-agnostic oracle

Add a schema and fixture such as `gis-map-inference-runtime-v1`, evaluated by
independent Bun/Node/AJV code and the first-party Rust decoder. The fixture must
carry no bearer, raw MAP pack, storage path, provider secret, or prompt. It
defines:

* exact scope, authenticated actor/session/generation, descriptor/catalog/
  package/service identity, frontier, request/input hash, event cursor, lease,
  progress, terminal, and reader visibility fields;
* accepted request, deterministic progress sequence, success/proposal,
  cancellation at each provider checkpoint, deterministic provider failure,
  deadline, and restart/lease recovery vectors;
* Author success, Viewer/Share denial, cross-space and foreign-document denial,
  stale/revoked generation, descriptor/catalog/package/service/frontier mismatch,
  duplicate request (same identity idempotent; distinct scope non-interfering),
  over-capacity/oversize/cursor replay, and no-progress-after-terminal vectors;
* owner-private reads and a peer-redaction assertion; and
* P1-only records proving stale approval and concurrent normal document edit
  cause no partial map/child/ledger publication.

### Exact native P0 laws

After implementation, register focused laws through the hub `📜️script.ts`,
`project.json`, and the launch **seed** (then generate the launch artifact):

1. real SQLite directory + trusted GIS catalog selection + server materialized
   MAP invokes the deterministic executor, records bounded progress and one
   proposal, and preserves owner-only reads;
2. cancellation, deadline, provider failure, session revoke, catalog rotation,
   and frontier change each yield one terminal event, no post-terminal progress,
   and wiped private buffers;
3. two concurrent same-scope submissions have one durable idempotent job; a
   matching request id from a different scoped actor cannot conflict or learn
   the first job;
4. restart while leased cannot double-execute and resumes or terminals by the
   defined deterministic recovery policy; and
5. no `MutationEnvelope`, `ServerFrame::Commands`, map snapshot, child graph,
   or directory peer event changes on every P0 terminal path.

The existing ledger check remains a prerequisite, not a substitute for these
route/runtime laws.

### Protected two-user process/browser law

Only after P0, launch a local SQLite hub configured with a real trusted GIS
bundle/profile and a selected GIS descriptor. Bootstrap Author A and
collaborator B in the same space/document. Drive the actual MAP UI action (or
the exact public route while the UI is being wired), verify A’s localized,
accessible progress/result and cancellation/reconnect cursor, and prove B sees
neither job nor result. Exercise revoked A, stale socket/session, provider
failure, and hub restart. The P1 follow-up adds an explicit approval: both
clients see exactly one normal document mutation after durability, while B still
cannot read the job/proposal stream. This is a new journey gate; current
`secure-local-smoke`, admin journey, GIS codec tests, and ledger check do not
cover it.

## Acceptance boundary

No current runtime acceptance is claimed. P0 is accepted only when the neutral
oracle, focused native route/SQLite laws, and protected two-user process/browser
journey all execute against the same server-selected GIS catalog. P1 acceptance
additionally requires the composition transaction proof. A remote/paid model
provider, generic tool execution, automatic approval, and long-offline replay
remain outside both packets.

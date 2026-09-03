# AI Map Inference: Smallest Complete Vertical Slice Blueprint

## Decision

Build one **server-owned, durable, private GIS inference job**. A user with an already authenticated document binding can request the current `s.gismap` inference, see bounded progress, cancel it, and later read its immutable result. The hub, not a browser, native shell, MCP process, or plugin, owns authorization, the snapshot, execution admission, durable state, and result bytes.

This is deliberately an *inference-only* slice. It does **not** apply an AI mutation. A collaborator is deliberately unable to observe another member's job, progress, result, or cancellation. The later proposal/approval/apply/undo slice is the first point at which collaborators observe a committed document edit. That is the privacy boundary required by the acceptance matrix; making a private inference job visible merely to satisfy "collaboration" would be an authority leak.

Current-tree evidence supports this boundary:

| Fact | Evidence | Consequence |
| --- | --- | --- |
| GIS has one executable typed service | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:199-309,374-382` | Reuse its `s.gis.gismap.inference` semantics and geo vectors; do not invent an AI calculation. |
| The service rejects invalid cancellation ids, cache mode and budget exhaustion before decode | same file `:219-282` | The hub must pass a fresh opaque cancellation identity, a fixed cold cache mode, and bounded budgets. |
| Plugin-host `ArtifactInferenceRouter` validates route identity, dependency graph, freshness and guest echo | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:6838-6995,7078-7103` | Reuse behind a narrow hub executor port; never expose the router as an MCP or UI capability. |
| Hub has the dependency but no inference route or executor | `🌎️hub/📦️packages/🦀️rust/Cargo.toml:31-44`; no `ArtifactInferenceRouter` occurrence under `🌎️hub/**`; router is only the routes in `📦️bin.rs:5067-5098` | This is an actual implementation partition, not a wiring toggle. |
| Hub readiness explicitly says inference is false | `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1702-1744` | Readiness cannot be claimed until a trusted executor is installed and self-checked. |
| Verified server-side checkpoint bytes already exist | `🌎️hub/🛰️lag-rebootstrap/🦀️.rs:202-221,304-373` | The executor can read the verified active `pack`/`spr` pair itself; no client ever uploads map bytes. |
| A trusted catalogue currently has no linked native codec binding | `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:379-395` | The provider/catalog binding is a prerequisite; no implicit local GIS import may bypass it. |
| MCP discovery is real but execution intentionally fails closed | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:1-24,300-332` | Replace the gap only after a remote hub job client exists; do not call the headless workspace's local channel. |

## Slice contract, before Rust or TypeScript

Add schema-first files under a new domain-neutral hub inference module, for example:

```text
🌎️hub/💡️inference/
  🧬️schema/🔖️1/🔣️.json
  🧪️fixtures/🧬️inference-job-v1/🔣️.json
  🧪️fixtures/🧬️inference-job-v1/🧪️oracle/🟦️.ts
  🦀️.rs                         # codec, decider, projection and service
  🗄️store/🦀️.rs                 # backend-neutral event/projection port
```

The neutral JSON schema must be strict (`additionalProperties: false` everywhere), carry a literal `schema: "semio.hub.inference-job/v1"`, cap all UTF-8 identifiers, and reject control characters. It contains the following public *owner-only* view:

```text
InferenceJobRequestV1
  requestId: UUIDv7 / idempotency key, 1..64 ASCII bytes
  inferenceSchema: "s.gis.gismap.inference"
  expectedCheckpointId: 32-byte digest encoding

InferenceJobViewV1
  jobId: UUIDv7
  inferenceSchema, algorithmVersion, policyVersion
  checkpointId, baselineFrontier, descriptorDigestV1
  state: queued | running | succeeded | cancelled | failed | stale
  progress: { completedUnits, totalUnits }       // bounded, nondecreasing
  result: absent | { digest, byteLength, validity, quality, complete }
  terminalEventSeq

InferenceJobCancelV1
  requestId: UUIDv7 / idempotency key
  jobId: UUIDv7

InferenceJobEventV1
  eventSeq, jobId, occurredAtMs, kind, payload
```

`requestId` is a client idempotency key, never a cancellation secret. `jobId` and server-minted `cancellationId` are distinct. No wire type includes bearer material, socket grants, receipt proof, actor id, authorization generation, raw `pack`/`spr`, plugin component bytes, capability policy bytes, or a caller-supplied user/space/role.

### Fixed V1 bounds

The schema and server decider must state constants, not caller-selected limits:

| Value | V1 rule |
| --- | --- |
| Per principal / document live jobs | one; another request with the same idempotency key returns the original view, another key is `409 busy` |
| Stored event count per job | 32 total; terminal compaction retains `requested` plus the terminal event and projection |
| Progress records | at most 16, monotonic and coalesced; never hold arbitrary diagnostics |
| Result bytes | the exact GIS service allocation result cap, plus a hub result cap no larger than the provider's cap |
| Result lifetime | bounded retention; result is CAS-addressed by digest and delete/revoke makes the view unavailable |
| Request and cancel bodies | small strict JSON, independently bounded at the Axum route layer |
| Operation deadline | fixed server deadline propagated through `RebootstrapContext` and the provider operation context |

The initial implementation supports only the literal GIS schema/version tuple declared by `gis_map_inference_service` (`owner=gis`, artifact `s.gismap`, `s.gis.gismap`, document `gis.map`, inference `s.gis.gismap.inference`, all v1) at `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:201-216`. It does not accept a generic plugin id, schema string, model name, arbitrary policy, URI, prompt, cache mode, or previous-state payload.

## Durable CQRS/event-sourcing design

Use a dedicated aggregate, not `DirectoryCommand` or `DirectoryEventBody`. The directory event stream is global/member-filtered and its only current public variants end at artifact retention (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:145-213`); placing private result/progress there leaks it through directory replay.

### Commands, events, projections

```text
Commands (only the authenticated hub route constructs these)
  Request { authenticated_subject, bound_document, request_id,
            expected_checkpoint_id, literal_gis_schema }
  MarkRunning { job_id, worker_lease }
  AdvanceProgress { job_id, worker_lease, completed, total }
  Succeed { job_id, worker_lease, result_cas_ref, exact metadata }
  Cancel { authenticated_subject, request_id, job_id }
  Fail | MarkStale { job_id, worker_lease, bounded code }

Events
  inference.requested
  inference.started
  inference.progressed
  inference.succeeded | inference.cancelled | inference.failed | inference.stale

Projections
  inference_job_by_id             # owner, scope, lifecycle, checkpoint binding
  inference_request_dedup         # (owner, scope, request_id) -> job id + body digest
  inference_live_lease            # at most one active row per (owner, scope)
  inference_result_ref            # digest/length only; raw bytes live in protected CAS
```

The `Request` decider, dedup row, first event, and live lease must commit atomically. Same key plus same canonical body returns the original job; same key plus a distinct body is conflict. Two different simultaneous keys cannot create two jobs because the `(owner, scope)` unique/live lease is checked in the same transaction. The worker lease is monotonically minted from the event sequence; no late worker may write a terminal state after cancellation/revocation/restart.

Every transition appends an event and updates its projection in one backend transaction. This follows the existing directory invariant—backend `append_events` assigns a dense sequence and applies the projection before commit (`🌎️hub/📇️directory/🦀️.rs:2105-2124`)—but is a distinct private log/table family. Implement the same port for FS/SQLite/Postgres/Neo4j as the hub already selects independently for document storage (`📦️bin.rs:5105-5147`); do not hide persistent job state in `HubState`, `ShardedMap`, the MCP `HandleTable`, or a process-global job map.

On process restart, recover `queued`/`running` jobs by appending `stale` unless the retained event/projection establishes an unexpired recoverable lease and exact checkpoint binding. V1 may choose fail-closed restart semantics; it may not resume with a new snapshot under the old job id. Cancellation is terminal, idempotent, deletes the live lease, wipes in-memory provider input/output, and causes a late successful provider return to be discarded and wiped.

### Authoritative snapshot and stale law

The hub route derives `DocumentScope`, identity, role, and authorization generation from its authenticated connection. It performs all of these before appending `inference.requested` and again before terminal publication:

1. require document read authority; require the descriptor to match the open binding;
2. load `VerifiedRebootstrapSource::active_pair`, which verifies scope, descriptor digest, active checkpoint, `pack`, `spr`, and aggregate digest (`🌎️hub/🛰️lag-rebootstrap/🦀️.rs:314-373`);
3. require `selection.active_checkpoint_id == expectedCheckpointId` and capture `baselineFrontier` and descriptor digest in the first event;
4. invoke the provider only on the retained verified `pair.pack` bytes; never on a UI, MCP, URL, environment, or WebSocket payload;
5. just before `succeeded`, reread authorization, active checkpoint, descriptor digest, and frontier. A mismatch yields `stale`, no result projection, no event-visible payload, and zeroization of candidate bytes.

The current canonical pair endpoint already demonstrates the required repeated authorization/checkpoint discipline (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2407-2493`). It is not itself the inference API: it is HTTP projection of a canonical pair and only asserts a checkpoint baseline, not inference result currentness.

## Provider boundary and readiness

Introduce a narrow hub-only port rather than reuse generic `ArtifactInferenceRouter` directly:

```rust
trait TrustedInferenceExecutor: Send + Sync {
    fn readiness(&self) -> TrustedInferenceReadinessV1;
    async fn execute_gis_v1(
        &self,
        selection: VerifiedInferenceSelectionV1,
        pair: VerifiedActiveCheckpointPair,
        cancellation: &InferenceCancellation,
        progress: &dyn InferenceProgressSink,
    ) -> Result<VerifiedInferenceOutputV1, InferenceExecutorError>;
}
```

`VerifiedInferenceSelectionV1` is server-created from the verified catalog/descriptor and contains the literal GIS route identity, component digest, descriptor digest, active checkpoint id, frontier, fixed policy digest, budget constants, and server cancellation id. It does not deserialize from JSON.

The executor may internally initialise `ArtifactInferenceRouter`/a trusted plugin instance because that router already rejects route conflicts, dependency cycles, duplicate live cancellation identities, stale route commits, and non-echoed guest results (`🧰️framework/…/🔌️plugin/🖥️host/🦀️.rs:6849-6995,7078-7103`). It must do so only after the catalogue has verified the exact selected component and descriptor. `NativeCodecBinding` is codec-only (`🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:142-156`), so do not pretend that `linked_native_codec_bindings()` supplies an executable GIS provider. Add a separate verified executable/inference binding to the trusted-catalog loader, atomically installed with the selected catalogue generation.

Provider readiness is true only if all of these are true:

- verified trusted catalogue is loaded;
- its selected GIS executable binding, descriptor digest, service tuple and component digest agree;
- a fresh restricted plugin executor is creatable;
- the executor's fixed budgets/timeout and zeroization hooks are installed;
- the service's own deterministic GIS vector passes at startup or deployment gate.

Until then return `503 inference-unavailable`, retain `HubFeatureReadinessV1.inference=false`, and do not append a requested event. Populate the readiness field only after an actual executor is carried in `HubState`; today it is hardcoded false at `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1702-1744` and the state only retains `_artifact_authority`/`openable_catalog` at `:1340-1404`.

## End-to-end surface flows

### GIS browser/native UI

Extend the GIS action declaration and retained command mapping, not an ad-hoc React button:

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:219-237` — add `requestInference` and `cancelInference` to the schema-first `Gis2dCommand` action macro.
- The same file `:259-280,619-654,756-800,849+` — add them to retained tool ids/contracts, job factory, `build_tool_job`, `command_from_action`, app action definitions, and the all-actions coverage law. They must produce no document mutation.
- Keep `Gis2dConfigDelta` for presentation only. Do not persist job state, raw results, or authority in editor config (`…/🎚️config/🧬️schema/🔺️diff/🦀️.rs:12+`). The host maintains a bounded per-window `InferencePresentation` keyed by opaque `jobId`; it is cleared on close, cancellation, session epoch change, or any event that invalidates the document binding.

Do **not** use either existing generic kernel effect:

- `Effect::HttpRequest` exposes URL, headers and body to guest code (`🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:561-574`); it would invite bearer/header handling and client-supplied map bytes.
- `Effect::SpawnJob` is the local plugin worker protocol (`:625-633`) and is implemented as local guest work by the host/shard runtime. It is not a durable hub job.

Add three narrow capability-scoped effects and matching request outcomes: `RequestDocumentInference { req, inference_schema }`, `ReadDocumentInference { req, job_id, after_event_seq }`, and `CancelDocumentInference { req, job_id }`. They contain neither scope nor credentials; the shell/native host derives the current verified document binding. Their request outcome is a strict owner view. `Read` is a bounded long-poll or one bounded status page, not an unbounded stream; the host repeats it only while the same mounted session/epoch owns the job and cancels the outstanding request on unmount.

The effect change must cross every ABI boundary in one commit: kernel `Effect`; `🔌️plugin/🧬️schema/📜️.wit`; WIT/Rust conversions in `🔌️plugin/⚛️reactor/🦀️.rs:2536-2555` and `🔌️plugin/🖥️host/🦀️.rs:2615-2625`; browser wire mapping `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts:159-195`; and the browser shell dispatcher `…/ShellHost/🟦️.tsx:3030+`. The wire bridge currently drops every unrecognised effect with a debug warning, so adding only Rust would make browser GIS silently fail (`wire-turn.ts:192-194`). Native/WGPU needs the same protected host broker rather than a direct WebSocket/HTTP call.

The broker uses the current D1 authenticated directory transport/socket lifecycle to call only the new hub REST endpoints. It neither reads `fd3` nor sees a raw capability. Its cancellation owner closes a late successful long-poll/connection and ignores a response whose binding generation or view epoch changed.

### Hub REST boundary

Add strict routes beside the document routes in `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:5067-5098`:

```text
POST   /spaces/{space}/documents/{document}/inferences/gis/v1
GET    /spaces/{space}/documents/{document}/inferences/{job_id}?afterEventSeq=N
POST   /spaces/{space}/documents/{document}/inferences/{job_id}/cancel
GET    /spaces/{space}/documents/{document}/inferences/{job_id}/result
```

All are authenticated, exact-path (no query authority except bounded numeric cursor), strict JSON where applicable, have independent body/read deadlines, and derive principal/role from `bearer(headers)` plus server lookup. `GET result` revalidates membership and active checkpoint; it serves exact result bytes only to the original owner in V1. Return `404` for an inaccessible job rather than revealing its existence. The result is `private, no-store`, with no result URL logged or placed in a document/directory event.

The route is a separate command boundary. It must not be accepted over document WebSocket frames, directory WebSocket frames, admin APIs, extension asset routes, MCP stdio, or the canonical-pair route. It must not make `document_ws_v1` readiness/session activation conditional on inference. Existing hub router and `HubFeatureReadinessV1` show there is no accidental inference admission today.

### MCP endpoint

Keep `inference_list` as declaration discovery. Replace the fail-closed `inference_get` execution gap only with a `HubInferenceClient` that uses the authenticated hub workspace binding—not `HeadlessWorkspace`'s local `ArtifactChannel`, `open_probes`, or `HandleTable`.

Extend `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs`:

- replace `inference_get`'s current `channel.not-wired` branch at `:300-315` with owner-only job status/result retrieval;
- add `inference_start` and `inference_cancel`, with literal GIS selector validation and `jobId` only;
- update strict schemas at `:192-267`; request body must not accept artifact bytes, principal, scope, policy, or capability;
- `--folder` mode remains `PluginUnavailable` rather than falling back to a local, non-durable execution;
- remove or defer the current `InferenceJobPayload` seam (`:185-189`), which has no durable authority.

The client belongs next to `NativeHubBindingDriver`/the authenticated remote driver and is injected only when `HeadlessWorkspace::open_hub` installs its protected credential and grant source (`🏠️workspace/🦀️.rs:1155-1177,1226-1242`). The existing `principal`/`scopes` workspace fields are not an authority source for this endpoint; the hub derives them. MCP cancellation must cancel its outgoing request and issue the durable cancel only if it still owns the exact returned `jobId` and binding generation.

### Collaboration visibility

V1's positive collaboration law is **non-disclosure**: a second authenticated member may continue editing/reconnecting the same document, but cannot enumerate, poll, cancel, or read the owner's inference job. The document socket continues to fan out committed commands/ephemeral presence only; it gets no `inference.*` frame. `HubState` already describes this fanout as a relay of committed document frames, not a new durable ordering authority (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1366-1374`).

The next vertical slice, not this one, writes an explicit `gis.inference-proposal-created` artifact-domain event and then an approved document mutation. Only that committed mutation and its inverse become collaborator-visible. No generic JSON patch or auto-apply route is permitted.

## Language-neutral fixture/oracle

Create one fixture with a fixed `nowMs`, two users, two document bindings, one exact GIS checkpoint, deterministic `pack`/`spr`, and a compact sequence of requests/events. The independent Bun oracle must use AJV for schema strictness and WebCrypto/SHA-256 for record/digest checks; it must not import Rust codecs, scripts, or source snippets.

Required vectors:

1. request -> queued -> running -> two monotonic progress records -> succeeded; output digest/bytes equal the existing GIS geo fixture result;
2. repeated same `(owner, scope, requestId, bodyDigest)` returns one job/event stream; changed body with same key conflicts; concurrent distinct keys give one winner;
3. no raw `pack`, `spr`, bearer, socket grant, actor, policy or result appears in event, owner status, collaborator status, diagnostic, or audit vector;
4. negative: stale checkpoint, revoked member, descriptor/catalog mismatch, malformed id/schema/cursor, excess body, exhausted live slot, unknown job, and unready provider all fail before executor admission;
5. cancellation before start, during progress, and after a forced late provider result each produces exactly one terminal state and no result;
6. checkpoint/frontier changes during execution produce `stale`, never a result; a new request may then use the fresh checkpoint;
7. member B has no observable job result/progress and cannot cancel A; the document's normal edit/presence trace remains valid;
8. restart recovery does not resume old bytes under a new active checkpoint;
9. MCP and GIS/browser host forms produce the same canonical request and owner view while MCP folder mode stays fail-closed.

Keep the existing GIS semantic oracle authoritative for geographic values: its test already independently compares bounds against `geo::BoundingRect` and stable payload repetition (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:445-489`). The new oracle tests job protocol/authority, not duplicate GIS geometry implementation.

## Registered gates and launch entries

Add an `inference-job-check` command to `🌎️hub/📦️packages/🦀️rust/📜️script.ts`, then register the corresponding `os-hub:inference-job-check` target in `🌎️hub/📦️packages/🦀️rust/📋️project.json`, following `OpenPlanCheckScript`'s exact-one pattern at `📜️script.ts:2636-2678`:

1. run the independent Bun fixture oracle;
2. list each Rust law and require exactly one full test name before running it `--exact --test-threads=1`;
3. run only the narrow hub/MCP/GIS feature/package checks needed by this slice;
4. run a process-backed secure hub + native/MCP smoke only after the narrow laws pass; distinguish it from compilation.

The initial exact Rust-law suffixes should cover: `inference_job_request_is_single_winner_durable_and_idempotent`; `inference_job_uses_verified_active_pair_and_rejects_stale_or_revoked_authority`; `inference_job_cancel_and_late_success_are_exactly_once_and_wiped`; `inference_job_result_is_owner_only_and_bounded`; and `mcp_inference_job_uses_authenticated_hub_binding_not_local_channel`. The script must reject zero or multiple list matches, exactly as current P4 gates do.

Add a launch entry immediately after the current document-open/canonical-pair server gates in `.vscode/launch.json` (current ordering `411.108`–`411.12` at `:4433-4474`):

```text
name:    ⚖️gate💡️gis-inference-job🛡️server
command: bun nx run os-hub:inference-job-check --skip-nx-cache
group:   4_gate
order:   411.13
```

Then add one explicit secure smoke launch/compound that starts hub plus native GIS and secure MCP, invokes the exact same fixture job through both entry points, and proves no pre-Session/plugin activation action is required. It must not be folded into `dev-secure-suite` until the standalone gate is non-vacuous. Existing secure MCP launch is `🛠️dev🔐️os-secure-mcp🌉️stdio` at `.vscode/launch.json:4548-4560`; current compound multi-user hub capability is at `:7820-7826` and is the natural later collaborator non-disclosure smoke host.

The GIS plugin's current package test script ignores its input segments and runs the whole package; correct that before claiming a permanent exact GIS law (`✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts:7-10`).

## Safe implementation order

1. **Contract and oracle partition.** Add neutral schema/fixture/oracle and the exact GIS service metadata adapter. No routes, no UI.
2. **Hub aggregate partition.** Add durable event store, decider/projections, exact bounds/idempotency/cancellation/stale laws, and a test executor. It stays unreachable from HTTP.
3. **Trusted executor partition.** Extend trusted-catalog executable binding, install the provider in `HubState`, use `VerifiedRebootstrapSource::active_pair`, wire provider readiness, and prove source/result zeroization.
4. **Protected hub/MCP partition.** Add strict REST routes and the MCP remote client/tools. Retain folder-mode fail-closed behavior and test authority/revocation.
5. **OS effect/UI partition.** Add the narrow effect ABI across Rust/WIT/TS/native/browser and GIS request/progress/cancel presentation. Prove unmount/cancel/binding-generation races.
6. **Registered gate/launch partition.** Add exact-one combined gate, launch entry, then process-backed native and MCP smoke. Only this terminal phase may change the slice from source-complete to accepted.

## Most actionable next implementation partition

Start with **partition 1: `InferenceJobV1` contract + neutral oracle + hub decider in-memory tests**. It fixes the authority and state vocabulary before any cross-process wiring, creates an exact target for every later transport, and exposes the provider catalog prerequisite without letting a UI/MCP implementation accidentally execute local inference. The next owner should not start the GIS button or replace MCP's `channel.not-wired` response until this partition's request/cancel/stale/non-disclosure vectors have one canonical shape.

## Audit status

This is a source-validated blueprint only. No build, Nx target, launch configuration, or runtime journey was run for it.

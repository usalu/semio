# Hub Inference-Job Coordinator Audit

Read-only source audit, 2026-09-03. No build, test, or runtime process was run. Line references describe the current shared tree, not a claim that its adjacent tests currently pass.

## Decision and first deterministic blocker

**Do not connect MCP to `ArtifactInferenceRouter` directly.** Add a hub-owned, durable `InferenceJobV1` coordinator that obtains an authority-verified pair, derives all execution identity from the authenticated session and trusted catalog, and is the sole writer of inference job events. The local GIS implementation is real; it is not a hub service.

The first deterministic call-chain failure is `inference_get`: the MCP facet looks up a declaration and then deliberately returns retryable `PLUGIN_UNAVAILABLE` with `channel.not-wired`. `ArtifactChannel` has no inference command, and the headless workspace is a different activation/process from the `run` host. This is explicit in [MCP inference](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:1), [lines 135-189](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:135), and the capability handler around [lines 260-320](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:260). The hub has no alternate inference endpoint: its complete Axum route table ends with document status, share, blob, and WebSocket routes at [lines 2076-2104](../../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2076), while readiness explicitly reports `inference: false` at [lines 552-580](../../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs:552).

### Severity / blocker matrix

| Severity | Current evidence | Consequence / required ordering |
|---|---|---|
| Critical | `inference_get` is declaration-only and returns `channel.not-wired`; no hub route exists. | No authenticated MCP inference job can be submitted. Implement the coordinator command port before exposing an execution tool. |
| Critical | A hub workspace returns that canonical artifact bodies, schema, and validation remain unavailable until P4-B ([workspace lines 1279-1298](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1279), [1716-1728](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1716)). | A job cannot obtain a trusted cold input pair. P4-B plus P2-D canonical pair read is a hard prerequisite. |
| Critical | `linked_native_codec_bindings()` returns `Vec::new()` ([hub bin lines 184-201](../../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs:184)); the loader requires an exact binding for every selected artifact kind ([trusted catalog lines 296-320](../../../../../../../../../../🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:296)). | A configured trusted GIS catalog cannot activate its codec today. Land the linked native-codec/openable catalog provider before admitting jobs. |
| High | The generic MCP `JobRegistry` is a process-wide in-memory map, deliberately workspace-independent ([UI lines 8-12](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🖥️ui/🦀️.rs:8), [144-308](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🖥️ui/🦀️.rs:144), [536-588](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🖥️ui/🦀️.rs:536)). | Jobs leak across workspaces in a process, disappear at restart, and have no subject/session/revocation check. It must not be the durable job store. |
| High | `WasmtimeNodeHost` loads locally named component paths and committed descriptors; it hashes WASM then uses `PackageId(plugin_id)` ([run lines 1664-1713](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs:1664), [1788-1800](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs:1788)). | A hub executor cannot trust client/plugin labels or local registry residue. It needs an authority-issued, catalog-generation-bound execution ticket. |
| High | The MCP action adapter's handles, idempotency, audit sink, and undo token are fresh in-process values ([workspace lines 1449-1474](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1449)); its undo is best-effort local fan-out ([dispatch lines 835-864](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🦀️.rs:835)). | It is a useful semantic precedent, not durable shared approval/apply/undo authority. |
| High | Hub authentication already derives a user/session/authorization generation from a bearer and checks the current role ([hub bin lines 435-478](../../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs:435)); document WebSocket is document-scoped. | Coordinator ingress must reuse this server-side derivation and revalidate it, never accept actor, role, package, blob locator, or frontier from MCP. |
| Medium | GIS inference is bounded deterministic snapshot analysis: count fields and bounds are defined at [GIS lines 14-53](../../../../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:14). | It proves a local service, not a mutation producer. Never manufacture a generic JSON patch or auto-apply a bounds result. |
| Medium | Native/store sync has document bootstrap progress and cancellation ([sync lines 491-496](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:491)); no hub inference job projection or client observer was found. | Add a job-observation port only after durable job reads exist. Collaborators observe the approved domain edit, not another user's raw inference/progress. |

## What is reusable, and what is not

| Seam | Source-backed behavior | Coordinator use |
|---|---|---|
| GIS `ArtifactInferenceService` | `GisMapInference::infer` is deterministic and derives counts/bounds from a snapshot ([lines 22-36](../../../../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:22)). | Reuse only behind the trusted executor; serialize a typed result under its declared schema. |
| Host inference router | Enforces contributor/dependency ownership, conflicting routes, DAG validation, recursive dependency evaluation, live revision/generation checks, and exact result echo ([host lines 5854-5993](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:5854), [6083-6107](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:6083)). | Reuse as internal executor mechanics; coordinator supplies canonical input and independently enforces durable freshness/auth. |
| `run` host | Registers declared inference roster from committed descriptors with `ArtifactInferenceRouter`, subject to dependency-first loading ([lines 1689-1721](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs:1689), [1788-1799](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs:1788)). Its per-turn limits are fuel 10M, deadline 5s, effects 256, patch 1 MiB, frames 256 ([lines 1461-1466](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs:1461)). | Define a narrow `TrustedInferenceExecutor` port; do not export the mutable router or activate a second local host from MCP. |
| Plugin reactor infer job | Begins cancellation, publishes preview/progress, and runs a guest job ([reactor infer lines 178-205](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️.rs:178)). | Adapt its cancellation/progress contract to the coordinator; it has no durable identity or hub scope itself. |
| Artifact authority and directory checkpoint projection | Public checkpoint projection removes opaque storage locators and binds descriptor digest, pair hashes, aggregate and frontier ([directory lines 865-923](../../../../../../../../../../🌎️hub/📇️directory/🦀️.rs:865)); private checkpoint recovery remains authority-only. | Resolve the exact pair server-side. Never send a private locator, generic blob hash URL, or CAS chunk address to MCP/executor callers. |
| Trusted catalog | Bundle loader verifies component SHA-256/BLAKE3, descriptor SHA-256/decode, dependency closure and explicit codec binding ([trusted catalog lines 245-315](../../../../../../../../../../🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:245)). | Persist the verified catalog generation and exact plugin/package/component/descriptor identity in the job binding. |
| Mutation/undo | Adapter rechecks an expected revision before transaction commit and can require approval ([dispatch lines 615-717](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🦀️.rs:615)). | Retain the prepare/recheck/approve idea, but replace ephemeral handles/audit/undo with hub events and a server-issued application id. |

The existing directory has the appropriate projection pattern: immutable document descriptors, verified checkpoint append, active checkpoint query, retention lineage, and SQLite/Postgres/Neo4j implementations ([directory lines 945-1048](../../../../../../../../../../🌎️hub/📇️directory/🦀️.rs:945), [1513-1518](../../../../../../../../../../🌎️hub/📇️directory/🦀️.rs:1513)). It has no inference-job event or projection. The separate chunk CAS is the correct input/result substrate—not generic DB payloads ([chunk CAS line 91](../../../../../../../../../../🌎️hub/🗿️artifact-authority/🗂️chunk-cas/🦀️.rs:91)).

## Target contract: schema-first `InferenceJobV1`

Add JSON Schema plus neutral fixtures before Rust/TypeScript types. A single request is an append-only command that returns a stable `jobId`; reads are projections, never client-mutated records.

```text
InferenceJobRequestedV1
  commandId, idempotencyKey, requestedInferenceSchema, canonicalPolicy,
  requestedBudget { inputBytes, outputBytes, deadlineMs, maxProgressEvents }
  -- caller may name only the artifact in its already-bound workspace.

InferenceJobV1 (server-derived and durable)
  jobId, requestDigest, state, attempt, progressSequence,
  subject { userId, originalSessionId, originalAuthorizationGeneration },
  scope { spaceId, documentId },
  descriptorDigest, catalogGeneration,
  executable { pluginId, packageId, componentSha256, componentBlake3,
               descriptorSha256, artifactKind, artifactSchema, packSchemaHash,
               inferenceSchema/version, algorithmVersion, policyVersion },
  input { checkpointId, packSha256, sprSha256, aggregateSha256,
          baselineFrontier, requiredTailFrontier, pairDigest },
  result { schema, sha256, byteLength, sourcePairDigest, sourceFrontier,
           diagnosticsDigest } | failure { stableCode, redactedDetail },
  proposal { proposalDigest, mutationId, expectedFrontier } | applied { applicationId, editId, resultingFrontier }
```

`jobId` is server-minted. `requestDigest` is canonical schema bytes over the subject, scope, verified executable, pair identity, canonical policy, budget and idempotency key—never a private locator, bearer, or raw client actor. Limits must be constants, checked before pair materialization: maximum request/policy/result/diagnostic bytes, one job per command, bounded dependency depth, max attempts, monotonically increasing `progressSequence`, and an absolute deadline no greater than the executor budget. Result bytes reside in an authority-owned, ref-counted chunk-CAS object keyed by result SHA-256; the projection exposes decoded/limited content only to a currently authorized owner, never a storage key.

States are `Requested → Admitted → Queued → Running → {Succeeded | Failed | Cancelled | Stale | Expired}`. `Succeeded → ProposalPrepared → {ApprovalPending | Rejected | ApplyQueued} → {Applied | ApplyFailed | Stale}` is separate from computation. Terminal states do not regress. A cancellation command first appends `CancellationRequested`; workers cooperate at bounded checkpoints and append `Cancelled` only after releasing transient input/result leases.

### Authority, privacy, and freshness rules

1. The authenticated MCP carrier supplies only a verified session capability. The hub derives subject, session id, authorization generation, current role, space and document; MCP must not send actor, role, package/plugin id, descriptor digest, checkpoint, frontier, catalog generation, blob hash, locator, or a WASM path.
2. At admission the coordinator requires the immutable document descriptor, a P4-B canonical `(pack,spr)` pair, active checkpoint, required tail frontier, and a verified openable-catalog row. It computes the binding from those server objects and admits only an advertised inference capability.
3. Revalidate original-session revocation/generation and current membership at dispatch, result publication, `job.get`, cancellation, proposal, approval, apply and undo. A revoked/removed owner cancels an unstarted/running job and denies output; a later session may inspect a historical job only if it authenticates as the same subject with the current explicit `inference.read` capability. Share/public/spectator access is read-only and cannot request, observe private job content, approve, apply, or undo.
4. Before publishing a result and again before proposal/apply, compare descriptor digest, catalog generation, component/descriptor hashes, checkpoint pair digest and required tail frontier to current authoritative values. Any mismatch appends `ResultRejectedStale`/`Stale`; it never creates a mutation. Rebootstrap, catalog reload, descriptor change, checkpoint advance, retention floor advance, membership revoke, or session generation change invalidates relevant cached projections.
5. A job result/progress stream is private to the submitting subject/session authority (and redacted audit/admin projection). It is **not** document presence, preview, or a collaboration broadcast. Other collaborators receive only the ordinary approved document mutation and later undo on the existing document command lane.
6. The coordinator sends canonical pair bytes to one trusted executor by an in-process/private port, with deadline/cancel control; no generic `GET /blobs/{hash}`, private locator, chunk probe, or caller-selected WASM is part of the protocol.

### CQRS/event and recovery model

Add `InferenceJobEventV1` variants to the hub's append-only directory/event projection family (or one dedicated append-only job stream with identical backend transaction/rebuild semantics): `Requested`, `Admitted`, `Queued`, `Started`, `Progressed`, `CancellationRequested`, `Cancelled`, `ResultProduced`, `ResultRejectedStale`, `ProposalPrepared`, `ApprovalResolved`, `ApplyQueued`, `Applied`, `UndoQueued`, `Undone`, `Failed`, `Expired`. Each carries bounded redacted metadata, event sequence, server correlation id, and no secret/pair locator.

The durable projection has unique `(scope, requestDigest)`, `jobId`, request-owner, state, lease expiry/attempt, source binding, and result CAS reference. Worker claim is compare-and-set `Queued → Running` with a short lease; only the lease holder can append progress or terminal state. On restart, an expired `Running` lease becomes `Queued` only if its source binding and authorization still validate; otherwise it becomes `Stale`/`Expired`. The same request digest returns the existing job, so retries cannot run twice. CAS result publication uses staged object + integrity verification + durable reference before `ResultProduced`; an unreferenced staged object is swept after lease/retention grace.

Approval/apply is a separate saga because directory events and the document database are distinct stores. `ApplyQueued` persists a deterministic `applicationId` and expected frontier. The hub submits one typed, GIS-owner-approved mutation envelope stamped with that id and the server-derived actor; its document-side idempotency makes recovery retry-safe. The coordinator appends `Applied` only after observing that exact edit id/resulting frontier. A crash between the two is recovered by querying the document mutation/application id, never by replaying a client request. `UndoQueued`/`Undone` follow the same deterministic application identity and typed inverse; they do not reuse the MCP `undo_` handle. If a GIS inference supplies no declared `GisInferenceProposalV1` mutation capability—as the current counts/bounds service does—it terminates at `Succeeded` and offers no apply action.

## Bounded implementation packets

1. **P5-I0 — prerequisites, no job endpoint.** Finish authenticated MCP carrier/first-frame migration, P4-B exact canonical pair read, P2-D chunk CAS, authoritative open-plan/catalog capability projection, and generated `linked_native_codec_bindings`. Keep readiness `inference: false` until all are live. Touch the current trusted catalog/authority and [hub startup](../../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs:184) seams only.
2. **P5-I1 — contracts and persistence.** Add `🌎️hub/💡️inference/🧬️schema/…/🔣️.json`, neutral fixtures, `InferenceJobCommand/Event/V1` types, and directory projection ports/SQLite/Postgres/Neo4j parity. Add bounded replay/rebuild and request-digest uniqueness; no executor yet.
3. **P5-I2 — admission and trusted executor port.** Add `HubInferenceCoordinator` to `🌎️hub`, taking authenticated session context, directory, canonical pair authority, openable catalog and `TrustedInferenceExecutor`. It derives the binding, creates job events, verifies limits, leases, and calls the already-loaded host router through an internal ticket. Do not add an `ArtifactChannel` infer variant for hub execution.
4. **P5-I3 — result, stale, cancel, audit.** Implement result CAS staging/ref accounting, bounded private progress read, cancellation/deadline, restart reclaim, session/revocation/catalog/rebootstrap invalidation, and redacted audit events. Replace inference use of process-wide `JobRegistry`; retain it only for unrelated local UI jobs.
5. **P5-I4 — proposal/approval/apply/undo.** Add a GIS-owned `GisInferenceProposalV1` declaration plus typed mutation/inverse if a product mutation is desired. Add hub approvals and deterministic apply/undo saga. No generic patch, automatic mutation, local `ActionAdapter` handle, or actor supplied by MCP.
6. **P5-I5 — MCP and observers.** Replace `inference_get` execution gap with `inference_request`, `inference_get`, `inference_cancel`, `inference_proposal`, `inference_approve`, and `inference_apply` bindings to the authenticated hub client. Add private job status/progress to React/native once the secure carrier/open plan is present; map surfaces show only committed document changes. Wire EN/DE accessible status/cancel controls in a later focused UI packet.

## Required neutral and independent-oracle tests

| Oracle / fixture | Must prove |
|---|---|
| Language-neutral `inference-job-v1` fixture consumed by Rust, TypeScript and a tiny independent parser | Canonical request/event encoding, request/result digests, state transitions, size/deadline bounds, redaction, and no locator/bearer/client actor field. |
| Independent GIS oracle using the existing third-party `geo` test dependency | A known map snapshot's bounds/counts equal `GisMapInference`; tampered result schema/hash/echo is rejected. The third party is test-only, never runtime authority. |
| Deterministic fake authority + fake trusted executor | Descriptor/pair/catalog/frontier mismatch, result corruption, duplicate idempotency key, cancellation at every checkpoint, expired lease/restart, and CAS orphan sweep/ref retention race. |
| Independent real WebSocket + MCP stdio harness | Two isolated local-bootstrap credentials: A requests job, B cannot get/progress/cancel/result; B sees only A's approved map edit and undo. Revoke A before result/apply; verify terminal denial/socket close and no edit. Retry/reconnect/rebootstrap must yield `Stale`, never apply stale work. |
| SQLite mandatory; PostgreSQL/Neo4j optional parity environments | Event append/projection rebuild/restart, unique request digest and lease CAS give identical terminal projection. Do not call an unavailable Docker backend a passing gate. |

## Focused commands after the packets land

These are existing targets; they were **not run** for this audit.

```sh
bun nx run @semio-tech/gis-plugin:test-quick
bun nx run @semio-tech/framework-plugin-host:test-quick
bun nx run @semio-tech/framework-os-mcp-rs:test-quick
bun nx run os-hub:test-quick
bun nx run os-hub:secure-local-smoke
```

The two-user/MCP oracle belongs in a new bounded `os-hub` test target or its existing script router, started through the protected local bootstrap rather than a copied environment bearer. PostgreSQL/Neo4j parity remains opt-in when their real backend/Docker prerequisites are available. The current `os-hub` script explicitly says its all-feature test may require Docker for PostgreSQL ([script lines 465-480](../../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts:465)); it is not a zero-touch replacement for the SQLite gate.

## Exit criteria

The feature is ready only when a real authenticated MCP session can request a GIS inference against an authority-read canonical pair; hub restart preserves/reconciles exactly one job; revocation, rebootstrap, descriptor/catalog/checkpoint/frontier changes deny stale results; an approved typed mutation and its typed undo are durable document events; and a second authorized collaborator observes those mutations but cannot observe or control another user's raw job. Until then, the honest user-facing state is declared local inference with execution unavailable.

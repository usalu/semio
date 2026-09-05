# Terra AI-Over-Map Durable Integration Blueprint

Date: 2026-09-04  
Scope: current-source, read-only audit and implementation blueprint for an authenticated, durable map-inference path. No source/test code, build, launcher, model call, or runtime gate was run. “Source-backed” below is not a runtime result.

## Verdict

**RED — no durable AI-over-map integration exists.** The repository currently has a strict, deterministic GIS *inference* calculation. It has no map inference UI command, hub admission route, model/provider implementation, durable job/event/result projection, or collaborator/reconnect view. Calling the existing computation “AI” would overclaim: it returns map feature counts and geographic bounds, not a generated model response.

The smallest honest first feature is a **private, server-owned durable GIS inference job** using the fixed local `s.gis.gismap.inference` algorithm. It deliberately does not add a remote LLM/model provider or mutate the map. A model-backed provider is a later, separately audited implementation of the same internal executor port—not an implementation detail the UI, MCP, or document socket may select.

## Current End-To-End Trace

| Boundary | Current source-backed fact | Classification / break |
| --- | --- | --- |
| Map UI command | The closed `Gis2dCommand` list has 14 edit/view/locale/source commands and no inference/request/cancel/progress/result action ([`gismap editor/🦀️.rs:221-246,259-273`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:221)). React/WGPU action decoding maps only that closed list ([`:752-797`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:752)). | **RED:** no user request, progress, cancellation, result or error ownership. |
| Plugin capability policy | GIS declares `documents.write` and `shell.navigate`, but no inference capability ([`✏️s/🔌️plugins/🌍️gis/🦀️.rs:26-70`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🦀️.rs:26)). | **RED:** current plugin capability cannot authorize a remote/costly provider or a private result read. |
| Deterministic computation | `gis_map_inference_service` fixes owner/schema/algorithm/policy versions and computes an encoded `GisMapInference` from a supplied snapshot ([`gismap/🦀️.rs:199-309`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:199)). It bounds allocation/work/depth, rejects empty cancellation id and incremental cache mode before decode. | **Source-only local reusable calculation**, not an authenticated job, cancellation capability, model call, or persistent result. |
| Schema/result | The output schema is strict and contains only counts plus optional bounds ([`🧬️schema/💡️inferences/🔣️.json:1-38`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔣️.json:1)). | Accepted typed deterministic result shape; it contains neither prompt, provider, provenance, job identity nor mutation proposal. |
| Plugin host executor | `PluginInstanceHandle::infer` turns a raw request into a `semio.infer` cold guest job ([`🔌️plugin host/🦀️.rs:4795-4916`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:4795)). `ArtifactInferenceRouter` validates declared owner/dependency topology, guest echo and a process-local revision/generation fence ([`:6826-7103`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:6826)). | Reusable executor mechanics only. Its raw wire request and `cancellation_id` are not a hub subject, scope, durable lease, checkpoint frontier or revocation authority. |
| Loaded plugin path | `WasmtimeNodeHost` owns an inference router and registers descriptor inferences after local component/descriptor loading ([`🏃️run/🦀️.rs:1411-1500,1660-1810`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs:1411)). | Separate local `run` process; no hub route or durable authority may treat it as a multi-user service. |
| MCP | `inference_get` discovers a declared service but returns retryable `channel.not-wired` for execution ([`🌉️mcp/💡️inference/🦀️.rs:138-190,300-332`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:138)). | Correct current fail-closed behavior; MCP cannot submit, read, cancel or apply an inference. |
| MCP generic jobs | `job_get`/`job_cancel` use one process-global in-memory `HashMap`, with cooperative cancel and arbitrary JSON result ([`🌉️mcp/🖥️ui/🦀️.rs:183-310,536-590`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🖥️ui/🦀️.rs:183)). | **Privacy/durability RED:** no subject/workspace/scope/revocation filter, restart persistence, lease, bounded event history or collaborator rule. Never use it as the hub AI job store. |
| Hub/provider | The full hub route table has no inference endpoint ([`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:5067-5099`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:5067)). Readiness hard-codes `features.inference: false` ([`:1691-1744`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1691)). A source census found no OpenAI/Anthropic/Ollama/chat-completion/model-provider implementation under hub or GIS. | **RED:** no authenticated admission, provider credential boundary, rate/budget/deadline policy, or readiness. |
| Durable CQRS and reconnect | The existing directory event/command schema ends in space/member/document/checkpoint/retention events and has no inference command/event ([`📇️directory schema/🦀️.rs:145-240`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:145)). `DirectoryService` serializes append and broadcasts event/presence streams ([`🌎️hub/📇️directory/🦀️.rs:1519-1720`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1519)). | **RED:** there is no private job aggregate/projection to rebuild after restart or reconnect. Do not put private input/progress/result in the member-filtered public directory stream. |
| Verified document identity | Document socket admission revalidates descriptor/catalog/checkpoint and plan authority before consuming its grant ([`📦️bin.rs:2600-2710`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2600)). Hub D0 remains unavailable because `linked_native_codec_bindings()` returns an empty vector ([`📦️bin.rs:379-395`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:379)). | Valuable authority precedent, but map inference cannot currently establish a verified installed GIS executable/catalog target. |

## Security And Consistency REDs

1. **Unscoped raw executor.** `ArtifactInferenceRouter` accepts plugin-style raw bytes, including policy, canonical payload and cancellation ID. Passing browser/MCP values through it would let a client choose input, budget/revision identity, algorithm path or cancellation collision. The hub must derive all of those from authoritative server records.
2. **No provider boundary.** There is no current remote model integration. Adding a direct UI/MCP HTTP credential, environment token, arbitrary URL/model name, or a generic `model` string would leak provider authority and permit exfiltration of map contents. The first packet must have no external model credential at all.
3. **No private durable state.** The MCP singleton is process-wide and `DirectoryEventBody` is visible through directory replay. Either would leak one user's inference result/progress or lose it on restart. Private jobs need their own tenant-filtered durable event log/projection.
4. **No stale-result law.** The host's revision/generation fence is per loaded runtime, not an authoritative `(space,document,descriptor,catalog,checkpoint,pair,frontier)` binding. A map edit, rebootstrap, catalog change, role revoke or reconnect can otherwise publish obsolete output.
5. **No collaboration policy.** Presence/document broadcast is not permission to see private analysis. A collaborator should observe only a separately approved durable GIS mutation, never another user’s raw result/progress/input. The initial query-only result is owner-private.
6. **No result/CAS ownership.** There is no inference result ref counted against retention, no result digest/size validation, and no resume/retry lease. Raw map pair locators/chunk keys must never reach UI/MCP/provider payloads.

## Smallest Honest Implementation Packet

### First ship: `GisMapInferenceJobV1`, deterministic and private

This packet consumes the existing deterministic `s.gis.gismap.inference` service, but only behind a hub-private `TrustedMapInferenceExecutorV1` port. Its V1 implementation is fixed to `gis/deterministic/v1`; no caller selects provider, prompt, endpoint, model, policy, cache mode, component path, plugin ID, document bytes or frontier.

**Prerequisite:** do not expose an endpoint until D0 supplies an exact GIS native provider/catalog binding and the hub can resolve a verified `(plugin, package, component hashes, descriptor hash, artifact schema, inference schema)` for `gis.map`. The current empty hub native-binding vector makes this an explicit dependency, not an import-to-make-it-work shortcut.

The only client request is a strict bounded request under the already-authoritative scope path:

```text
POST /spaces/{spaceId}/documents/{documentId}/inference-jobs
GisMapInferenceJobRequestV1 { requestId }

POST /.../inference-jobs/{jobId}/cancel
GisMapInferenceJobCancelV1 { requestId }

GET /.../inference-jobs/{jobId}
```

The server derives authenticated subject/session generation, current role, exact document descriptor, verified catalog generation/executable, active canonical pack/spr pair, pair digest, baseline and required-tail frontier, fixed policy/budgets/deadline, cancellation capability and request digest. Do not accept any equivalent field from the request body.

The first UI action is one GIS editor command/effect, `request-map-inference`; it receives only a typed private job view. It must show bounded progress/status and a cancel affordance, clear state on document/scope/generation change, unmount, route close and authority revocation, and never put job data in presence/interaction state. EN and DE labels are part of the UI packet. It must not be represented as a normal document mutation or silently apply the result.

### CQRS boundary

Use a dedicated durable `InferenceJob` aggregate beside hub directory/artifact authority—not `DirectoryCommand`, generic MCP `JobRegistry`, websocket presence, or a document mutation stream. Its tenant-filtered event ledger must use the same SQLite/Postgres/Neo4j transaction/rebuild discipline as directory projections, while its read API applies owner/current-authorization checks before returning any view.

```text
Command (hub-created only)
  Request(subject, scope, requestId, verifiedSourceBinding)
  Claim(jobId, leaseId, deadline)
  Progress(jobId, leaseId, sequence, completed, total)
  Cancel(subject, jobId, cancelRequestId)
  Finish(jobId, leaseId, resultRef | stableFailure)

Events
  Requested, Admitted, Queued, Started, Progressed,
  CancellationRequested, Cancelled, Succeeded, Stale, Failed, Expired

Projections
  jobs_by_id (owner/scope/state/source binding/terminal sequence)
  request_dedup ((owner,scope,requestId) -> canonical request digest + jobId)
  active_lease ((owner,scope) -> jobId/lease/expiry)
  result_ref (CAS digest, byte length, source binding, retention)
```

Rules:

- `Request` dedup record, one live `(owner,scope)` lease and `Requested` event commit in one transaction. Same key + same canonical request returns the original job; same key + different digest conflicts. Two concurrent distinct keys cannot both start work.
- Worker `Queued → Started` and every progress/terminal append requires the current lease. Progress is sequence-monotonic, coalesced, bounded (for example max 16 entries/job); terminal states never regress.
- The executor receives server-owned canonical pair bytes via an internal port and checks cancellation/deadline at bounded stages. A pending cancel becomes terminal immediately; a running cancel becomes `Cancelled` only after the executor stops and releases input/result ownership.
- Result bytes are staged in authority-owned CAS, hash/size/schema-validated, durably referenced, then and only then `Succeeded` is appended. Unreferenced staged bytes are swept after a bounded lease/retention grace. The public projection exposes no CAS key or locator.
- At dispatch, completion, read and any future apply, revalidate current membership/session generation, descriptor, catalog generation, exact executable identity, canonical pair digest and required-tail frontier. Any mismatch yields one durable `Stale`/`Expired` terminal, never a current result or mutation.
- Restart examines expired leases: retry once only after source/auth revalidation; otherwise mark stale/expired. Reconnect reads the durable owner projection; it does not replay local UI/MCP memory.

### Provider boundary

`TrustedMapInferenceExecutorV1` is an internal hub trait with a single V1 implementation for the exact deterministic GIS service. Its input is a server-built opaque binding and canonical pair, its output a bounded typed GIS result plus stable diagnostics code. It cannot access request headers, raw session credentials, a client URL, local plugin registry cache, arbitrary Wasm path, or external network provider credential.

A later model-backed implementation must be a distinct provider identity/version with server-held secret provisioning, explicit egress/data-classification policy, request/result byte caps, deterministic cancellation/deadline, audit digest (not prompts/map bytes), and exact model/executable provenance in the source binding. It must first pass all V1 privacy/stale/revoke laws; the UI/MCP schema stays provider-agnostic.

## Neutral Fixtures And Independent Oracle

Create `inference-job-v1` strict JSON schema/vector fixtures before implementation. They contain only public IDs, canonical digest inputs, event transition expectations, bounded progress and redacted result metadata—no bearer, socket grant, actor ID, raw pack/spr, CAS locator, model secret, prompt, or provider request body.

The separate Bun/AJV/WebCrypto oracle must independently validate strict JSON, UTF-8 and count bounds, canonical request/result digests, idempotency outcomes, state transitions, progress monotonicity and redaction. It must not call Rust deciders or import the plugin host. Pair it with the existing GIS `geo::BoundingRect` vector oracle at [`gismap/🦀️.rs:450-525`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:450), but do not mistake that local geometry check for an authorization/durability oracle.

Mandatory hostile vectors:

- missing/extra/overlong fields, malformed request ID/digest, duplicate key with different body, and two simultaneous different request IDs;
- nonmember/viewer/revoked/changed-generation submission, read, cancel and result retrieval; second principal denied every private job field;
- forged component/catalog/descriptor/pair/frontier/result hash, wrong schema/version, oversized result/diagnostics, and stale descriptor/catalog/checkpoint after `Queued` and `Running`;
- cancel before claim, during execution and after terminal; late executor success after cancel/revoke must not publish;
- worker crash/lease expiry/restart and reconnect; exactly one terminal event, no double execution/result reference, no orphan CAS reference;
- progress sequence rewind/overflow/too-many updates; and
- proof that an ordinary approved document mutation is visible to collaborator B, while A’s job/progress/result is not.

## Exact Gates

Current registrations are inadequate and were not run:

| Existing command | Current coverage / limit |
| --- | --- |
| `bun nx run @semio-tech/gis-plugin:test-quick` | The target merely passes `test quick`, but its router ignores segments and runs the GIS Cargo package ([`📋️project.json:8-47`, `📜️script.ts:7-14`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📋️project.json:8)). It covers the local service/vectors, not hub admission or persistence. |
| `bun nx run os-hub:open-plan-check --skip-nx-cache` | Verifies D0/D1 plan scope only ([`🌎️hub/.../📋️project.json:111-117`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📋️project.json:111)). It has no inference aggregate/provider journey. |
| `bun nx run os-hub:secure-local-smoke --skip-nx-cache` | Current secured local startup/smoke; no inference endpoint or readiness is registered ([`📋️project.json:164-177`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📋️project.json:164)). |
| MCP Rust tests / `job_get` | Validate a local process-global UI job seam and a negative `inference_get`, not remote durable inference. |

Add permanent hub-script targets (in the existing hub `📜️script.ts`, exposed through `project.json`) and adjacent launch entries:

1. `os-hub:gis-inference-job-check`: exact-one FQN preflight then Rust laws for aggregate/lease/CAS/stale/revocation, plus the Bun neutral oracle. Launch name `⚖️gate💡️gis-inference-job🌎️hub`, ordered beside existing `open-plan-check`.
2. `os-hub:gis-inference-journey-check`: uncached protected-local SQLite process with two independently authenticated users, real GIS deterministic executor, restart/reconnect and raw protocol observer. Launch name `⚖️journey💡️gis-inference🌎️hub`.
3. Fold the successful journey into `os-hub:secure-local-smoke` only after it is independently non-vacuous. PostgreSQL/Neo4j projection parity is separate and must fail honestly when its real environment is absent.

Acceptance requires all three sequences to select/execute their exact cases, plus the isolated native GIS provider/D0 readiness gate. No Cargo compile, registry generation, static inference descriptor, or MCP `job_get` result may stand in for the two-user durable journey.

## Acceptance And Nonclaims

The first accepted scope is: an authorized map owner requests one fixed deterministic GIS inference over a hub-resolved verified current map pair; receives an owner-private durable result or typed terminal state; cancellation, revocation, descriptor/catalog/checkpoint/frontier change and restart cannot publish a stale result; reconnect reconstructs the same terminal projection; and another collaborator sees no private job data.

It does not accept model-backed AI, remote provider credentials, prompts, arbitrary plugins/models, automatic mutation, proposal/approval/undo, client-selected map bytes, MCP local handle state, browser/WGPU rendering, or all-catalog activation. A later GIS proposal/apply packet may make a specifically approved, expected-frontier typed document mutation visible to collaborators; that is the first legitimate shared result.

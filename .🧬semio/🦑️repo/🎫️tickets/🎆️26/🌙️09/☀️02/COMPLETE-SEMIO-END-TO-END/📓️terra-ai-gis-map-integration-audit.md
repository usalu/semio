# Terra Audit — AI/GIS Map Integration

Date: 2026-09-03  
Scope: read-only source audit after the current P2/P4 changes. No build, launch, network call, or test was run. Line anchors below are the inspected working-tree snapshot and can move with concurrent edits.

## Verdict

`s.gis.gismap.inference` is a real, deterministic, bounded **local plugin service** with vectors and an independent `geo::BoundingRect` oracle. It is neither a hub inference service nor an MCP executable capability.

The first deterministic end-to-end blocker is earlier than GIS execution: a hub-bound `HeadlessWorkspace` accepts caller-provided principal/scopes/token, reads its plugin catalog from the local repository, and cannot enumerate or read a cold hub artifact. Even if P4 lands verified metadata/pair access, the MCP wire has no `artifact-infer` command and the `inference_get` facet deliberately returns `PLUGIN_UNAVAILABLE / channel.not-wired`. `job_get` and `job_cancel` remain declared stubs.

A second independent high-severity launch blocker exists in the new catalog path: `configured_artifact_authority` passes `linked_native_codec_bindings() == Vec::new()` to `TrustedCatalogLoader`; the loader rejects a selected profile without an executable codec. The resulting authority is stored as `_artifact_authority` and has no request handler. Therefore the current opt-in trusted-catalog startup neither establishes a usable GIS executor nor exposes an authority-backed endpoint.

## What is demonstrably present

| Segment | Evidence | Current status | Boundary / limitation |
| --- | --- | --- | --- |
| Authenticated hub documents | Hub resolves bearer session to current space role, and document status returns `headSeq`, `commitSeq`, and `epoch`: `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:415-450,511-522`. | Partial | `POST /auth/sessions` still mints a bearer after an arbitrary email/upsert (`:554-564`); do not treat its token as production identity until the session-security packet lands. |
| Hub metadata / live directory | Directory WS revalidates its caller and filters space membership on each outbound frame: `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1395-1444,1483-1511`. | Real for the directory plane | MCP does not call these routes today. P4 verified descriptor/pair access is therefore a prerequisite, not evidence of current MCP discovery. |
| Trusted catalog boot seam | Optional loader verifies a selected bundle under fixed startup context, then creates `ValidatingCanonicalArtifactAuthority`: `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:147-160,2070-2099`; verified catalog promises dependency-first packages: `🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:212-250`. | Startup seam only | `linked_native_codec_bindings()` is empty (`📦️bin.rs:162-164`), loader requires an executable codec (`trusted-catalog/🦀️.rs:341-344`), and `HubState._artifact_authority` is not consumed (`📦️bin.rs:229-246`). |
| GIS declaration and service | `2d.map` / `gis.map` artifact declaration at `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:15-23,180-196`; inference metadata is `s.gismap` / `s.gis.gismap.inference` and calls `infer_gis_map` (`:199-217`); declaration registers it (`:376-381`). | Real local implementation | It is not a registered native hub codec or a hub route. |
| GIS computation | Canonical snapshot decode, budget/depth enforcement, typed result encode, exact/complete output (`gismap/🦀️.rs:219-309`). | Real local implementation | Only `Cold` and `Bypass` caching are admitted; `Incremental` is rejected (`:229-231`). It provides no job progress stream and the cancellation identifier is an admission field, not an externally callable cancellation operation. |
| Independent oracle | Language-neutral vectors run the service twice, decode the canonical result, and compare bounds to `geo::BoundingRect`: `gismap/🦀️.rs:388-487`; invalid payload/budget/cache/cancellation cases are tested at `:493-525`. | Strong local proof | This establishes computation correctness, not hub auth, remote bytes, routing, revision freshness, or collaboration visibility. |
| Plugin-host router | `ArtifactInferenceRouter` binds owner/kind/schema/versions/algorithm/policy/revision/generation/payload/dependencies/budgets/cancellation/cache in `🔌️plugin/🖥️host/🦀️.rs:5772-5841`; register validates descriptor/dependency graph (`:5854-5897`); infer recursively resolves dependencies and checks exact result echo (`:5904-5993,6083-6180`). | Reusable production seam | Request/response routing structs are private to host and the only live-freshness setter is not wired from hub/MCP. Router return is one terminal async result: no MCP job state/progress/cancel port. |
| Run-process assembly | `🏃️run` owns router (`🏃️run/🦀️.rs:1408-1412,1496-1500,1640-1642`). Recursive load decodes a committed descriptor, loads dependencies first, instantiates one plugin service instance, and registers its inference roster (`:1672-1720,1788-1799`). | Real run-process assembly | It is a distinct process from MCP. It fails loudly when a committed descriptor is absent; MCP cannot reuse this router merely by reading descriptor metadata. |
| MCP discovery | Static union of local registry descriptors at `🌉️mcp/💡️inference/🦀️.rs:13-24,74-103`; artifact matching at `:105-131`; `inference_list` and `inference_get` are registered (`:249-267,300-331`). | Honest metadata discovery | MCP uses local repo registry/descriptor paths, not hub authority. Input has only `artifactId` and `inferenceSchema` (`:212-225`), with no revision/frontier/identity/budget/policy pair. |
| MCP execution/jobs | File itself states `ArtifactChannel` has only history/pure-command/transaction frames and no infer (`🌉️mcp/💡️inference/🦀️.rs:1-24,135-190`); `inference_get` returns retryable `channel.not-wired` (`:155-161,300-316`). `job_get`/`job_cancel` are stub names (`🌉️mcp/🦀️.rs:282-283`). | Not implemented | `InferenceJobPayload` is an inert prospective shape. `HandleTable` is process-memory only (`🎫️handles/🦀️.rs:152-233`), so it cannot make jobs restart-safe or collaborator-visible. |
| MCP workspace / caller | Hub origin simply carries `base_url`, `space_id`, and optional raw token (`🏠️workspace/🦀️.rs:419-450`); workspace retains caller-provided principal/scopes and a locally minted session (`:1131-1184`). | Configuration exists | No authenticated `me`/role/capability discovery. CLI also accepts raw `--principal` / `--scopes` and token-shaped arguments (`📦️bin.rs:1-107`). |
| Cold artifact bytes | `workspace_artifact_ids` and `read_artifact_bytes` can read live probes/folder storage but do not enumerate/read a hub-origin cold artifact (`🏠️workspace/🦀️.rs:1194-1235`). | Folder/probe proof only | Blocks trusted input acquisition, source schema resolution, and map result visibility for a hub document. |
| Typed mutation, revision, undo, local audit | Generic action invoke approval-gates and compares revision just before commit (`🌉️mcp/🔀️dispatch/🦀️.rs:592-717`); it mints local undo, and undo/redo fans out transaction commands (`:835-863`). Agent audit is append-only local JSONL, with redaction (`📒️audit/🦀️.rs:1-6,55-81,151-195`). | Useful local protocol seams | The present channel cannot execute a real GIS capability end to end, local handles/audit do not survive restart or become collaborator-visible hub events, and inference has no typed apply capability. |

## Actual current path

```text
Hub bearer / space / document
   └─ MCP --hub --space --token → WorkspaceOrigin::Hub (configuration only)
       ├─ local repo registry + local committed descriptor → inference_list (metadata only)
       └─ no remote document list/read + no authenticated actor binding

GIS committed descriptor → run::load_runtime_recursive → PluginInstanceHandle
   └─ ArtifactInferenceRouter → guest infer → terminal typed bytes
       └─ no hub transport, no MCP ArtifactChannel variant, no job/event projection
```

The MCP module documents this process separation explicitly. Its `HeadlessWorkspace` activates its own Wasmtime instance; `run` owns the router that can dispatch plugin guest inference. Substituting MCP's synthetic probe/channel tests for the run process would prove only a local test DAG, not a hub execution path.

## Failure and authority matrix

| Severity | Failure / abuse | Deterministic evidence | Required rule |
| --- | --- | --- | --- |
| High | A remote caller selects a principal/scopes or reads another space by treating CLI configuration as authorization. | Workspace accepts supplied identity/scopes; it does not call authenticated hub discovery. | Bind each MCP workspace to an authenticated hub session, resolved user, exact space membership/role/capability epoch. Never accept actor, role, or scope from MCP arguments for hub mode. Revalidate before submit, result read, apply, and subscription delivery. |
| High | A GIS inference runs on unknown, stale, or substituted bytes. | MCP input has no artifact pair, identity digest, revision, generation, or frontier; cold hub bytes return none. | Hub supplies an authorized immutable input pair plus descriptor/catalog/package/wasm identity and an authoritative frontier. The executor verifies all of them before dispatch. |
| High | Trusted catalog is assumed to make GIS executable. | Hub has empty linked native bindings; the verified authority is unused state. | Do not advertise execute. Register an explicit trusted GIS executable binding and consume the authority in the execution boundary, or keep runtime result unavailable. |
| High | A result is applied after another collaborator edits the map. | Router has revision/generation checks, but no hub caller updates it; MCP has no frontier in request/result. | Capture one exact input frontier. Publish `current` only if current hub frontier equals captured frontier. Any change makes the candidate `STALE`; stale output can be historical/auditable but cannot be read as current or applied. Apply must repeat an atomic compare-and-commit at the hub. |
| High | Share-token/public viewer invokes inference or receives bytes/results intended for members. | Hub distinguishes session/share/public for document auth; blob authorization rejects share widening (`📦️bin.rs:415-450`). MCP bypasses that hub plane. | Separate result authorization from document visibility: require active authenticated member permission for infer/result/typed apply; share/public may receive no input pack, job status, progress, result, or audit detail unless an explicit policy declares a redacted read-only projection. |
| Medium | Cancellation/progress lie or job resurrects on process restart. | GIS validates an identifier only; router is terminal-return; MCP jobs are stubs and handles are in-memory. | A persisted hub job projection owns a cancellation token, monotonic event sequence, deadline/budgets, terminal reason, and restart reconciliation. Cancellation is idempotent; no `cancelled` result may later become current/successful. |
| Medium | An AI output becomes an implicit document mutation. | Existing generic action path has approval/revision/undo, but inference does not create a typed proposal/apply capability. | Keep inference query-only. Define one GIS-owned typed mutation from a validated proposal schema; prepare renders bounded diff, explicit approval signs the exact proposal + frontier, and apply uses atomic expected-frontier. No generic JSON patch. |
| Medium | Collaborators disagree about results or miss state after reconnect. | Hub document fan-out is present; inference has no hub event/projection. | Persist job/result lifecycle as space/document-scoped events and expose filtered projection + WS replay. Ephemeral progress is best effort and must be reconstructible to a bounded terminal/status projection after reconnect. |
| Medium | Local audit/undo is represented as collaborative durable history. | MCP audit is local JSONL and handle table is RAM. | Emit authoritative hub audit events for requested/started/progress/terminal/read/approved/applied/cancelled/stale decisions. Persist a typed inverse/transaction reference only on successful application; authorize undo independently and compare frontier again. |
| Low | Product prompt/tool list implies more than it executes. | `inference_get` is registered, but its documented result is `channel.not-wired`; tests assert the typed gap (`💡️inference/🦀️.rs:455-570`). | Preserve that error until the transport lands; update any prompt/end-to-end assertion that calls it a computed result. This is expectation drift, not a GIS algorithm defect. |

## Non-negotiable binding and stale-result contract

Define a schema-first `InferenceInputBinding` and make it part of every job, progress/result event, audit record, and apply request. It must contain:

- `spaceId`, `artifactId`, authenticated `subjectUserId`, authorization capability/role epoch, and an opaque server-issued binding id;
- exact artifact pair identifiers/digests, document codec/schema version, selected catalog profile, `pluginId`, package/wasm hash, descriptor hash, inference/algorithm/policy versions, and dependency result digests;
- full hub frontier: document identity, epoch, head edit ordinal/id, commit sequence, chain hash, plus router revision/generation derived from that same read;
- canonical input digest/length, bounded policy digest, cache mode, fixed resource budget/deadline, cancellation id, and request idempotency key.

The hub, not MCP, mints the binding after authorization and after fetching the verified P4 pair. The run/executor verifies identity and descriptor compatibility before it sees payload bytes. The host's current exact-echo checks are reusable, but only after the hub maps its frontier to router revision/generation and advances/cancels the route when document state changes.

State rules:

1. `QUEUED → RUNNING → {SUCCEEDED_CURRENT, STALE, FAILED, CANCELLED}` is monotonic; progress is sequenced and bounded.
2. A result can be `SUCCEEDED_CURRENT` only when the same authoritative frontier remains current at publication. If it changes during work, record `STALE` with the captured/observed frontier; retain only an authorized historical audit projection, never a current inferred field.
3. A read of a stored result rechecks active session, membership/capability epoch, space/document binding, descriptor/package identity policy, and result visibility. Revoke/kick/role loss ends delivery and denies apply.
4. `apply` accepts a server-issued proposal id, exact result digest, approval bound to that digest, and the captured frontier. It atomically compares current frontier and commits a GIS typed operation. Conflict returns no partial mutation and requires re-inference/review. Successful apply emits document event(s), audit event, and a typed undo reference.
5. Cache reuse is valid only for byte-identical input binding, exact executable identity, policy/budgets, and dependency digests. GIS map itself rejects incremental mode, so v1 permits only `Cold`/`Bypass`; no cache key may silently downgrade a changed frontier into a hit.

## Smallest dependency-ordered production packet

This deliberately starts after P4's promised verified metadata/pair access; it does not depend on unfinished loader internals beyond consuming their stable verified pair/identity boundary.

1. **Close the authenticated workspace boundary (blocking, parallel to loader work).**
   - Change targets: `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`, `🌎️hub/📇️directory/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️bin.rs`, `🌉️mcp/🏠️workspace/🦀️.rs`.
   - Land the session-security replacement before remote MCP. Add an authenticated workspace/bootstrap endpoint that returns only current caller/space grants and P4-filtered document metadata; remove hub-mode trust in CLI principal/scopes. Explicit loopback-only dev bootstrap may create a developer identity, but must be disabled/not routeable on non-loopback binds.
   - Exit: two credentials cannot cross-read a private space; revoked credential loses workspace, result, and apply access on the next request/stream frame.

2. **Expose a minimal verified artifact-read port (P4 integration, no inference yet).**
   - Change targets: `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`, `🌎️hub/🗿️artifact-authority/**`, `🌉️mcp/🏠️workspace/🦀️.rs`, `🌉️mcp/🗿️artifact/🦀️.rs`.
   - Implement `list/read` through active membership and a server-minted binding; return descriptor/pair/frontier identities, never arbitrary blob hash access. Replace hub `workspace_artifact_ids`/`read_artifact_bytes` `None` behavior with this port. Keep share/public responses redacted/denied by default.
   - Exit: MCP discovery reads actual authorized cold `gis.map` identity and bytes; wrong-space, revoked, stale-pair, and missing artifact cases are distinguishable typed failures without data leakage.

3. **Define the inference job protocol and a real channel, schema-first.**
   - Change targets: `🌉️mcp/🧬️schema/🦀️.rs`, `🌉️mcp/💡️inference/🦀️.rs`, `🌉️mcp/🧭️protocol/🦀️.rs`, `🌉️mcp/🎫️handles/🦀️.rs`, `🌉️mcp/📒️audit/🦀️.rs`.
   - Replace the prospective `InferenceJobPayload` with versioned job/event/result schemas and add `artifact-infer`, `job_get`, `job_cancel`, and result-resource semantics. Do not use process-local `HandleTable` as durable job authority; it may hold a client continuation only.
   - Keep `inference_list` descriptor-only. `inference_get` must require server binding and return a job or an authorized existing result; never fabricate result bytes from metadata.

4. **Add one hub execution coordinator, then adapt existing run router.**
   - Change targets: `🌎️hub/📦️packages/🦀️rust/📦️bin.rs` plus a new hub inference module, `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs`.
   - The coordinator alone owns admission, queue, worker/budget/deadline/cancellation, lifecycle persistence, result publication, and frontier checks. It uses a narrow public adapter around `ArtifactInferenceRouter`, rather than exposing host-private wire structs directly to MCP. Register descriptor roster only after trusted artifact/package association passes; dependency-first `run::load_runtime_recursive` is reusable.
   - Initially admit only the GIS service identity and `Cold`/`Bypass`; link a real trusted executable binding. Do not claim hub startup support merely because catalog metadata loaded.

5. **Publish durable result/audit projections and collaborator observation.**
   - Change targets: hub inference module, hub WS/REST routing in `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`, `🌉️mcp/💡️inference/🦀️.rs`, `🌉️mcp/📒️audit/🦀️.rs`.
   - Persist job lifecycle and filtered result/audit events in the authoritative space/document lane. Directory/document WS can carry a subscription notification, but result reads must remain authorization-gated and replayable. Treat progress as ephemeral and rate/budget bounded.

6. **Add explicit GIS proposal/apply and undo.**
   - Change targets: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs`, its GIS mutation/schema modules, `🌉️mcp/🔀️dispatch/🦀️.rs`, and hub coordinator/audit projection.
   - Translate only validated `GisMapInference`/proposal values into a GIS-owned typed operation. Reuse current prepare/approval/revision/undo structure only after the channel binds it to hub transactions. Do not mutate map snapshot from arbitrary inference bytes or generic JSON patch.

## Required tests and neutral oracles

| Layer | Required proof | Independent oracle |
| --- | --- | --- |
| GIS service | Keep the existing language-neutral fixtures, canonical wire round trip, fixed budget/cancellation cases, and repeated-result equality. | Existing `geo::BoundingRect` independently computes bounds (`gismap/🦀️.rs:446-487`). Add the protocol vectors outside implementation-language test fixtures so a second implementation consumes the same cases. |
| Trusted execution identity | Swap one package/wasm/descriptor/pair digest at a time; assert no guest invocation and no candidate/result. | Independently compute SHA-256 from fixture bytes and validate wire schema using a non-Rust JSON Schema implementation. |
| Auth/space isolation | Two authenticated users, two spaces, member/share/public/revoked/kicked states; assert list/read/infer/job/result/apply event visibility. | A minimal raw HTTP+WebSocket client independent of `DirectoryClient`; compare denied responses/body lengths and assert no event crosses spaces. |
| Frontier/staleness | Start delayed inference, submit another user's map edit, then release inference. Assert exactly one `STALE` result, no apply, and no current resource. Repeat with a same-frontier run. | A deterministic external scheduler/test clock plus independently calculated frontier/chain-hash fixture. |
| Cancellation/restart | Cancel queued/running jobs; kill/restart coordinator after each terminal transition; reconnect another collaborator. | Persisted event-log replay checker that reconstructs job state from events and rejects illegal transitions/duplicate terminal events. |
| Typed apply/undo/audit | Approved proposal applies only once at exact frontier; reject altered result/proposal/approval and conflict; undo is separately authorized and audited. | A language-neutral event-sequence fixture checked by a second reducer, plus direct canonical map snapshot decode before/after. |

## Focused commands after implementation

Do not run these concurrently with unrelated Cargo work. They are proposed verification commands, not run by this audit.

```sh
bun nx run @semio-tech/gis-plugin:test -- --exact language_neutral_vectors_match_geo_bounding_rect_oracle_and_stable_payload
bun nx run @semio-tech/framework-os-mcp-rs:test-quick -- inference
bun nx run @semio-tech/framework-os-mcp:test-quick -- end-to-end.test.ts
bun nx run os-hub-ts:test-quick -- index.test.ts
HUB_E2E=1 bun nx run os-hub-ts:test -- index.test.ts
```

For the final real-process slice, start the hub and MCP only through registered launch configurations (`🛠️dev🌉️os-mcp🧵️stdio` / `🛠️dev🌉️os-mcp🌐️http` and the hub launcher), then use two independent raw WebSocket clients against an isolated temporary data directory. That run needs a built hub binary and a usable configured storage backend; PostgreSQL/Neo4j feature paths may additionally require their real services. It must also use a non-dev authentication path once the session-security replacement lands.

## Honest exit criteria

The feature is not complete when MCP lists GIS declarations, when the local GIS vector test passes, or when a trusted catalog loads. It is complete only when a genuinely authenticated member can discover one authorized cold map, request bounded inference over an identity-bound pair, observe sequenced progress/terminal result with a collaborator, experience stale/cancel/revocation behavior correctly, explicitly approve one typed mutation, and see durable hub audit/undo behavior after reconnect/restart—while a non-member, share token, revoked session, stale candidate, wrong package digest, and wrong-space client cannot obtain or apply the result.

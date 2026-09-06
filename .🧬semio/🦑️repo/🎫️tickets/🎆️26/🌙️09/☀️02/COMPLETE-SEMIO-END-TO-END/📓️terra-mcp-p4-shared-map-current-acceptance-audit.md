# Terra P4 MCP Shared-Map Acceptance Audit

Status: current-source, read-only audit on 2026-09-06. No build, process gate, browser, or external model request was run.

The P4 exit in [master plan](📋️master-plan.md:158) is specific: a real authenticated `semio-os-mcp` client changes a Map, sees the result in the Shell and a second collaborator, and undoes it without crossing a session or space. The existing Shell/closed-actor work is necessary, but it is not a substitute for that MCP journey.

## Current acceptance matrix

| P4 requirement | Current source evidence | Current status |
| --- | --- | --- |
| MCP child is authenticated and reconnects as the same user with a new socket session | Hub’s `proveMcpWorkspaceProcess` starts a credential-FD child, opens a Hub document, kicks its connection, requires a new socket-grant selector/session, and verifies a non-regressing snapshot ([Hub script](../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts:1154)). | Partial source/process harness; it uses a generated probe document, not GIS Map. It was not run in this audit. |
| Every transport connection has Hub authority | The direct child reads the protected `mcp` credential before argv; `run_stdio` then makes a Hub workspace. However, its local `AgentPrincipal` is still constructed from CLI `--scopes` ([MCP](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:698)). Hub routes re-check membership/authorisation, so this is not evidence of Hub privilege elevation; it is two ununified policy planes. | Partial. The P4 gate must exercise Hub denial/revocation, not infer it from local scope parsing. |
| Capability discovery is live and trusted | `build_catalog` always scans the repository plugin registry/descriptor files ([MCP](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:172)); inference roster discovery independently calls `find_repo_root`, `load_plugin_registry`, and `load_package_descriptor` ([inference](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:93)). | Blocked for P4: this is an installed/repository catalog, not the authenticated Hub catalog, plan, or verified descriptor selected for the document. |
| Cold shared Map inspection | A Hub-bound workspace returns an explicit retryable `PLUGIN_UNAVAILABLE` for known artifact bytes: canonical bytes are unavailable until P4-B ([workspace](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1328)). The current canonical-pair helper only yields binding/frontier identity. | Blocked. There is no MCP tool that returns authenticated, canonical Map context from cold Hub state. |
| Typed proposal, polling/progress, cancellation | The four Hub-backed inference tools exist: submit, events, cancel, approve ([inference](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:1245)); their job handles bind user, space, document, session, and authority generation. | Source-present but not an end-to-end P4 result. A deterministic provider is acceptable for this tier; an external LLM is neither required nor sufficient. |
| Approval makes the three-member Map change durable | Hub startup constructs `HubInferenceRuntimeV1` with `UnavailableGisMapApprovalCommitterV1` ([Hub bin](../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6604)). The committer always returns `Unavailable` ([runtime](../../../../../../../🌎️hub/💡️inference/🏃️runtime/🦀️.rs:132)). Child-bearing Maps are refused before even consulting a committer. | Blocked. `inference_ready = inference_runtime.is_some()` ([Hub bin](../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6612)) currently means proposals can be served, not that Map approval can apply. |
| Undo of approved Map group | `inference_approve` returns the inference approval receipt only ([inference](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:1377)); `history_undo` consumes an unrelated local `ActionAdapter` `undoToken` ([MCP](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:447)). | Blocked. No committed Map-group witness is translated into a subject-scoped undo capability. |
| Attached Shell focus/reveal | Secure stdio deliberately passes `bridge=None` ([MCP](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:698)); UI tools return unavailable without the HTTP bridge. | Blocked for attached tier. The bridge is not yet a document/Map-actor control plane. |
| Second collaborator observes and restart/reconnect preserves one action | Existing Hub tests cover author/spectator denial and ledger restart, but expected Map approval remains unavailable ([Hub bin](../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7436)). The process script explicitly labels the complete GIS scenario as designed/not run. | Blocked. No shared Map mutation exists to observe. |

## Material current traps

### 1. Local catalog discovery can be stale or different from the authenticated document selection

The registry module is intentionally a repository-root scanner ([registry](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📇️registry/🦀️.rs:1)). It is honest installed discovery, but violates P4’s “live catalog” criterion when MCP is attached to a Hub document: a local descriptor can advertise a GIS inference service that the current trusted Hub generation did not select, or omit one that it did. Do **not** repair this by hardcoding `s.gis.gismap`; those constants merely move the same mismatch.

The existing authenticated descriptor snapshot is the right authority. Add a Hub-backed catalog capability to `HeadlessWorkspace` that returns only the verified selected package/component/descriptor identities from its ready snapshot, and make both `tools/list` contribution projection and `declared_inferences_for_workspace` consume that capability in Hub mode. Folder/bare modes may retain their installed registry. Discovery must not reread descriptor paths after the authenticated selection.

### 2. Submit is allowed despite unavailable cold Map binding

`inference_submit_handler` deliberately records `baseBindingUnavailable` and submits anyway ([inference](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:1274)). That is safe only as a proposal-only tier because the Hub re-derives its own authoritative state. It cannot be presented as P4’s requested “inspect the active map context”.

Keep proposal-only operation explicit. For the P4 full tier add a distinct authenticated Map-context/inspection operation backed by the verified canonical pair and current per-document owner. It must return a bounded, typed projection plus its exact document scope, component/descriptor binding, and frontier—not raw arbitrary artifact bytes and not a local probe snapshot. If the pair/owner cannot be mounted, return retryable unavailable and do not claim cold inspection.

### 3. Production startup advertises proposal availability while every real Map approval is unavailable

The unavailable committer is consciously fail-closed, which is correct. The missing slice is not an AI-provider integration: it is a production per-document three-Store ownership bridge that implements `GisMapApprovalCommitterV1`.

Its smallest coherent contract is:

1. Resolve the current authenticated document/verified GIS selection inside the Hub owner; never accept a caller-provided durable decision, raw WAL record, Store seal, or Map child identity.
2. Retain the exact parent/drawing/value Store owners and prepared mutations through preflight, the committed event/witness, and the single terminal acknowledgement. The existing Store/DB durable-group work may be used only through its consuming owner/witness API.
3. Return `applied: true` only after the same committed three-member witness has been observed and the document sync owner can publish the resulting frontier. On cancellation/fault retain the owner for close/retry; do not produce an undo capability.
4. Keep `UnavailableGisMapApprovalCommitterV1` when this owner is not mounted. Split Hub readiness into proposal capability and `mapApproval` capability, rather than treating runtime construction as approval readiness.

This avoids fabricating a generic `ArtifactHandle::submit` path and preserves the intended three-member atomicity boundary.

### 4. There is no undo authority for an inference group

Once the committer exists, its committed witness needs to yield a new opaque, subject- and document-scoped history-group handle. Carry that handle in the approval result only after commit, and route `history_undo` to the same retained per-document group owner. The existing `ActionAdapter` token is not a substitute: it has neither the Map group witness nor the Hub document/space binding. A peer can observe the inverse through normal document sync but must not receive the author’s private job/preview/undo handle.

## Bounded P4 implementation and acceptance gate

The next implementation should be the real `GisMapApprovalCommitterV1` mount plus its authenticated cold inspection projection, not an external model provider and not another Shell-only panel. Extend the existing `proveMcpWorkspaceProcess` harness rather than create a second test topology:

1. Materialise a trusted GIS profile and create one actual parent+drawing+value Map in an authenticated space through the normal Hub document owner.
2. Start two independently credentialed MCP children, A (author) and B (authorised collaborator), plus the attached Shell when the dedicated Worker is available. Assert each child gets distinct session/socket selectors and that a viewer/cross-space session is denied.
3. From A, list capabilities and inspect the Map. Assert every advertised GIS capability and descriptor/component hash equals the Hub’s live verified selection; no local-only registry row is visible. Assert the typed projection is bound to the current frontier.
4. Submit a deterministic typed inference; poll progress; submit/cancel a separate offer; reconnect A; then approve exactly one proposal. Require a non-public committed three-member witness before `applied:true` and an opaque author-only history handle are returned.
5. Prove B receives only the committed document update in the Shell and through its authorised document view—not A’s job, preview, or undo handle. Restart/reopen Hub state and show the action exactly once.
6. Undo as A via the committed group handle. Require B to observe the inverse group and reject stale, cross-session, and cross-space handles.

Existing source selectors to extend are `os-hub:mcp-workspace-process-check` / `proveMcpWorkspaceProcess`, MCP `inference-bridge-process-check`, and Hub `gis-map-proposal-process-check`; the present GIS process script is intentionally not an execution claim. This is a single vertical acceptance scenario once the committer and canonical context bridge are real.

## Qualification boundary

The direct-child process harness, typed job API, and deterministic proposal path are useful foundations. They do **not** establish P4. No audit evidence supports a current claim of live trusted Map discovery, cold shared Map inspection, durable three-member approval, author-safe undo, attached focus/reveal, or second-collaborator observation.

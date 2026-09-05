# AI-Over-Map Current-Tree Delta Audit

**Scope:** read-only source audit on 2026-09-04. This refreshes the earlier AI/GIS reports only where the live tree changed their conclusion. No command was run and no runtime result is claimed.

## Verdict

**RED — local deterministic GIS inference is real, but no authenticated, durable, collaborative inference journey exists.** The sole material advance is a nonempty linked *stdio* codec provider. It does not admit GIS, does not expose a GIS executable, and must not be used as evidence of map inference readiness.

## Current end-to-end boundary

| Segment | Current evidence | Classification |
| --- | --- | --- |
| GIS descriptor and executor | `gismap/🦀️.rs:201-216` constructs an executable `ArtifactInferenceService`; `:374-382` includes it in the declaration. `:219-308` rejects empty cancellation ids, zero/excess budgets, incremental mode, and payload/work/depth excess before decoding, then produces a deterministic typed result. | **Source-only local executor** |
| Actual inference meaning | `.../schema/💡️inferences/🦀️.rs:14-52` defines counts and geographic bounds only. It has no model prompt, provider identity, proposal, mutation, or approval. | **Accepted narrow deterministic calculation; not AI action** |
| GIS UI action | `.../✏️editor/🦀️.rs:219-247` has exactly 14 commands, none for inference request, progress, cancel, result, or approval. The app's existing mutation bridge cannot initiate an inference. | **RED** |
| Hub catalog admission | `hub/📦️packages/🦀️rust/📦️bin.rs:394-396,5336` now obtains bindings from `NativeOpenableCatalogProviderV1`. But `artifact-authority/.../native-openable-provider/🦀️.rs:8-11,20-22,37-46` fixes the provider to 26 `stdio` receipts and rejects every other plugin/package. GIS has no linked native codec or executable receipt at this boundary. | **RED for GIS; supersedes the old “empty bindings” observation only** |
| Hub service/readiness/route | `hub/.../📦️bin.rs:1703-1744` publishes `features.inference: false`; its complete route table at `5154-5190` has no inference request, status, cancel, result, or approval endpoint. | **RED** |
| Job/cancellation authority | The reusable host request is only `{policy,budgets,cancellation_id,previous_state,cache_mode,canonical_payload,dependencies}` at `framework/.../plugin/🦀️.rs:1290-1299`; the service registry is explicitly process-local at `1382-1407`. It carries no authenticated subject, space/document id, descriptor/package hash, session epoch, frontier, deadline, cancellation authority, progress sink, or durable job id. A nonempty string is an identity check, not cancellation. | **RED** |
| MCP/headless consumption | `mcp/💡️inference/🦀️.rs:1-24,77-103` does honest static descriptor discovery. `:300-315` returns retryable `channel.not-wired` for any executable service; `:169-189` defines an inert future job payload only. `job_get`/`job_cancel` remain declared stubs (`mcp/🦀️.rs:282-283`). | **Fail-closed discovery only** |
| Typed mutation and approval | `gismap/.../schema/🧬️mutations/🦀️.rs:17-43` has 12 ordinary feature mutations only. There is no proposed-inference result, immutable input/result binding, approval/rejection event, or approved apply command. | **RED** |
| Collaboration projection | The durable directory vocabulary ends at user/space/member/document/checkpoint/retention events (`directory/🧬️schema/🦀️.rs:152-177`), so it cannot project job lifecycle, private result visibility, approval, or applied result to a second collaborator. | **RED** |

The local GIS law and the separate neutral vector are useful inputs: `gismap/🦀️.rs:432-525` registers declaration/vector/bounds tests, including a Geo-backed bounds comparison. They neither start a hub nor establish a client/session/space ownership chain.

## Current deltas from earlier AI audits

1. The earlier assertion that the GIS declaration has no executable service is superseded: `declaration()` now includes `inference_services([gis_map_inference_service()])` at `gismap/🦀️.rs:374-382`.
2. The earlier assertion that hub startup passes an empty native binding vector is superseded: `linked_native_codec_bindings()` now calls the V1 provider at `hub/📦️packages/🦀️rust/📦️bin.rs:394-396`.
3. Neither change closes the journey. The hub provider is deliberately `stdio/native-codecs/v1`, and the hub still advertises inference as false and owns no inference route. The MCP `channel.not-wired` behavior remains the honest result.

## Smallest dependency-ordered implementation packet

The first vertical slice must be a **server-authoritative, deterministic GIS bounds job**. It must not claim an LLM/model-provider integration and it must not let a UI or MCP caller send a raw canonical snapshot, arbitrary service identity, or a generic JSON patch.

1. **GIS admission identity — prerequisite.** Define one exact linked GIS codec/executor receipt backed by the checked-in `s.gismap` factory and immutable plugin/package/descriptor/component/schema hashes. Extend catalog publication by an explicitly named GIS provider (or a versioned multi-provider aggregate); do not weaken `NativeOpenableCatalogProviderV1`'s `stdio` identity check. Refuse missing/extra/duplicate receipts, mismatched factory output, descriptor/component/pack hash mismatch, and partial publication. This makes only the fixed GIS service admissible.
2. **Durable hub job contract.** Add schema-first request, progress, terminal result, and cancellation/approval event types. The server derives `{subject, session generation, space, document, descriptor/package/codec identity, input frontier}` from the authenticated document route; the client supplies only an opaque request id, supported inference id, bounded policy, and deadline class. Persist an idempotency digest over those server-bound fields. The directory/document projector, not the process-local registry, owns queued/running/cancel-requested/succeeded/failed/cancelled states, immutable result hash, visibility, and terminal time.
3. **Atomic executor and cancellation.** On one document/job transaction or actor serial lane, recheck membership, descriptor identity, and frontier before admission; reserve capacity; persist `queued`; then execute the single admitted GIS service. A cancellation capability must be server-minted, scoped to `(job, subject, session generation)`, single-use or generation-fenced, and observed before execution and before terminal publication. A stale/revoked/expired session must produce no result and no retry as another principal. Bound request, decoded snapshot, output, diagnostics, queue length, job lifetime, and retained result count/bytes.
4. **Private result then explicit approved apply.** Project a completed deterministic result only to the requester by default. Define a distinct typed `GisInferenceProposalV1` whose input/result hash and base frontier are immutable. A member's explicit `approve` produces a single document event containing a schema-validated, finite `GisMapMutation` list; reject stale frontier, result/hash substitution, duplicate approval, unauthorised approver, and cancellation/terminal races. Do not add a generic patch mutation. Only this event reaches collaborators and normal undo/history.
5. **UI and MCP consumers last.** Add GIS request/progress/cancel/reveal/approve commands to the shared app command contract, with accessible EN/DE labels and terminal cleanup on unmount/reconnect. The UI/MCP carries opaque job/proposal handles only. Wire MCP execution to the authenticated remote workspace after the hub endpoint exists; preserve today's `channel.not-wired` for every tier not bound to that endpoint.

Steps 1 and 2 can be implemented in parallel only once their shared receipt and job schemas are frozen. Steps 3–5 are ordered: no executable RPC before durable authority/cancellation, and no collaborative mutation before explicit approval.

## Neutral fixtures and registered gates required for this packet

- Add a language-neutral `gis-inference-job/v1` corpus: valid deterministic bounds; unknown service; wrong descriptor/package/component/pack hash; raw snapshot substitution; foreign space/document; oversized policy/input/output/diagnostic; zero/deadline overflow; stale frontier; duplicate idempotency request; two concurrent requests; cancel-before-start; cancel-vs-terminal; revoked session; result/proposal hash substitution; duplicate/foreign/stale approval; two-user projection and reconnect. A separate implementation must validate the corpus and reproduce the GIS bounds result (the current Geo vector is a useful but insufficient seed).
- Register `bun nx run os-hub:gis-inference-job-check --skip-nx-cache`. Its `📜️script.ts` must first run the neutral oracle, enumerate exactly one fully qualified Rust law for each named case, exact-run each law, run the relevant hub all-feature check, and then run a real two-authenticated-client SQLite process journey. A compile/no-run or one-user in-memory test is not acceptance.
- Keep `bun nx run @semio-tech/gis-plugin:test-quick --skip-nx-cache` as a local regression command only: current `gis/.../📜️script.ts:7-10` ignores selectors and runs the entire package, so it is not an exact-law job gate.
- Keep `bun nx run os-hub:native-openable-catalog-provider-check --skip-nx-cache` as **stdio-only** evidence. Its own script says this at `hub/.../📜️script.ts:2900-2910`; it cannot qualify GIS admission.
- Add an authenticated remote-MCP counterpart only after the endpoint exists, under `@semio-tech/framework-os-mcp-rs` rather than re-labelling descriptor discovery as execution. It must demonstrate no raw session/capability/result bytes in argv, environment, log, or tool result outside the authorized response.

## Explicit nonclaims

This packet does not activate every plugin, invoke an external model, make results public, repair snapshot bootstrap/tail recovery, or establish generic arbitrary artifact inference. It proves one statically admitted `s.gismap` deterministic inference from an authenticated current document, plus an optional explicitly approved typed collaborative mutation.

## Current acceptance revalidation — authenticated space-scoped MAP journey

**Verdict: BLOCKED at discovery and again at execution.** The current tree has a bounded local
geometric calculation, and it has an authenticated MCP *directory* binding, but it has no path that
turns an authenticated space member's GIS document into an inference request. No command was run in
this read-only revalidation; nothing below is runtime acceptance evidence.

| Required transition | Current live entrypoint | Current result |
| --- | --- | --- |
| Map user intent | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/…/✏️editor/🦀️.rs:219-247` defines the complete 14-variant `Gis2dCommand` surface; its action bridge is exhaustively matched at `:756-800`. | **RED.** There is no request/progress/cancel/result/approve inference command, action, or state. The cancellation implementation at `:390-495` belongs to envelope loading, not inference. |
| Descriptor/capability discovery | The source declaration really adds `gis_map_inference_service()` at `…/🗺️gismap/🦀️.rs:375-382`; MCP discovery, however, reads committed owner `🔣️.json` via `load_package_descriptor` in `🌉️mcp/🏠️workspace/🦀️.rs:147-161` and extracts only its `contributions.inference_services`/artifact-contribution rows in `💡️inference/🦀️.rs:74-102`. | **RED — newly material source drift.** The current committed `✏️s/🔌️plugins/🌍️gis/🔣️.json:24-160` contains only `contributions.panels`; it has no inference or artifact-contribution field. Therefore even a catalog that names GIS yields no declared GIS inference through this MCP mechanism. This supersedes the narrower prior characterization of static discovery as a usable GIS path. |
| Authenticated space binding | `HeadlessWorkspace::open_hub` accepts the protected `LocalHubCredential`, verifies the selected origin, and installs the `DirectoryClient`/grant source at `🏠️workspace/🦀️.rs:1255-1265`. `NativeHubBindingDriver::connect` makes the authenticated directory client and fences the initial directory dial with cancellation plus authority generation at `🏠️workspace/🔗️remote/🦀️.rs:481-539,676-690`. | **Source-only, bounded descriptor authority.** It authenticates the selected directory scope; it is not an inference authorization or execution lease. |
| Bind a GIS document to discovery | `read_artifact_resource` accepts a hub document id but returns a schema only for the MCP-owned open probe; an arbitrary plugin document returns retryable `PLUGIN_UNAVAILABLE` at `🏠️workspace/🦀️.rs:1779-1812`. The test names the same boundary at `:2158-2172`. | **RED.** `declared_inferences_for_artifact` first calls that schema resource (`💡️inference/🦀️.rs:105-131`), so a live `s.gis.gismap` document cannot even reach its schema/roster match. |
| MCP tool call | Root registry registers `inference_list` and `inference_get` with only `workspace`, not the per-call `AgentPrincipal`, at `🌉️mcp/🦀️.rs:545-547`. `inference_get_handler` obtains an item then unconditionally returns `channel.not-wired` at `💡️inference/🦀️.rs:300-315`; the resource path has the same terminal at `:339-368`. | **Fail-closed.** The current MCP tools can honestly report the missing binding/route, but cannot mint a job, carry a caller/session generation, or execute a GIS service. |
| Model/provider transport, cancellation and progress | The local request is a synchronous function pointer whose complete input is only policy, budgets, opaque cancellation string, old state, cache mode, bytes, and dependencies (`🔌️plugin/🦀️.rs:1290-1343`). `infer_gis_map` checks only that the string is nonempty, decodes and computes synchronously (`…/🗺️gismap/🦀️.rs:219-308`). | **RED.** There is no provider/model identity, authenticated subject/space/document/descriptor/frontier binding, deadline/cancellation observation, progress sink, or job/result identity. The calculation is deterministic bounds/counts, not a model transport. |
| Hub admission, durable result and collaboration | The hub marks `features.inference: false` at `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1685-1729`; its complete router has no inference request/status/cancel/result/approval route at `:5133-5169`. The native provider is still selected through `linked_native_codec_bindings` (`:394-396,5315-5350`), with no GIS inference registration. | **RED.** No durable job/event/projection, private progress/result visibility, proposal/approval, typed GIS apply, sync broadcast, undo, or reconnect recovery exists. |

The one positive local gate is deliberately narrow: GIS has a language-neutral bounds fixture and
Geo comparison in `…/🗺️gismap/🦀️.rs:445-489`, plus malformed/budget/empty-string tests at
`:492-525`. Its sole Nx-facing test router is package-wide
`@semio-tech/gis-plugin:test-quick` (`📦️packages/🦀️rust/📋️project.json:8-38` and
`📜️script.ts:7-10`); it discards selectors and has no exact inference, hub, two-user, or process
gate. MCP's registered `test-quick` likewise has no AI-specific exact selector; its only exact
compound gate is `canonical-pair-check` (`🌉️mcp/📦️packages/🦀️rust/📜️script.ts:80-108`). The
existing GIS React/WASM/native launch entries (`.vscode/launch.json:657-704`) render a playground;
they do not attach a hub or exercise inference.

### One dependency-ordered implementation packet

Implement a **hub-authoritative deterministic GIS inference job**, not an external-model feature:

1. Make the generated, immutable GIS package descriptor agree with the source declaration before
   any tool registration. The receipt must carry the exact GIS inference service metadata and be
   verified with the selected package/component/descriptor/schema hashes; reject missing, duplicate,
   stale, or substituted entries. Do not make MCP scrape source as a fallback.
2. Add schema-first `InferenceJobV1` command/events/projection under the hub's authoritative
   directory/document vocabulary. The request supplies only a bounded opaque idempotency key,
   declared service id and deadline class. From the authenticated document route the hub derives
   subject, session/authorization generation, space/document scope, immutable descriptor and
   selected catalog generation, plus exact base frontier. Hold admission and terminal publication on
   the document/job serialization boundary; a stale/revoked generation, changed descriptor, or
   changed frontier must publish neither a result nor a retry under another principal.
3. Register exactly the receipt-admitted GIS executor in the hub, reserve bounded queue/work/output
   capacity, persist `queued` before compute, and persist private progress/terminal result or
   cancellation before notifying clients. A server-minted cancellation capability is scoped to
   `(job, subject, authorization-generation)`, rechecked before execution and terminal commit. The
   current synchronous local string is not a cancellation capability and must not cross this route.
4. Only after that route exists, bind MCP and the map command surface to opaque job handles. Keep
   results requester-private. A separate subsequent packet defines an immutable
   `{job,resultHash,baseFrontier}` GIS proposal and explicit member approval that commits one
   existing typed GIS mutation/inverse; never auto-apply or accept a generic patch. That document
   event is the sole collaborator-visible/undoable output.

The acceptance gate must be newly registered as one non-cached Nx command, e.g.
`os-hub:gis-inference-job-check`: validate a language-neutral corpus in an independent
implementation; list and exact-run one Rust law per named case; then run two authenticated SQLite
clients against the real hub. Positive case: member A submits the current GIS document, observes
bounded progress and private result, explicitly approves, and B receives exactly one typed document
event and can undo it. Hostile cases: stale or foreign space/document, stale descriptor/catalog or
frontier, substituted raw payload/result hash, duplicate idempotency, capacity limit, cancel-before-
start, cancel-vs-terminal, session/member revocation, reconnect, and B attempting to read A's
private result or approve it. This packet intentionally does not claim an LLM/provider, generic
artifact inference, or a full catalog boot.

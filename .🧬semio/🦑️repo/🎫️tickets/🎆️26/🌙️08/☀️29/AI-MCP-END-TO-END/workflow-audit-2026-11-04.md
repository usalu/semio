# Workflow Audit — 2026-11-04

## Scope and Evidence Limits

Read-only source audit of the ticket semio-smoke.ts, the native-binary end-to-end suite, runOsMcpClientJourney, and selected-target channel metadata. No native execution was performed by this auditor. Source checks and declared tests do not establish native readiness, successful workflows, or successful shell interaction.

## Readiness and Discovery

The smoke harness correctly rejects non-JSON or invalid JSON-RPC stdout, rejects outstanding requests on exit, bounds discovery, and checks native server version differs from warming. Its ordinary mode only performs initialize: that can succeed against the warming facade and is not native evidence. Ready mode checks only a subset of tool names, not workflows.

Native mcp-server/🦀️.rs emits server/discover with resultType=complete, protocolVersion, capabilities and serverInfo at the result root. InitializeResultSchema validates the common fields but does not assert discovery resultType. This is an incomplete discovery check, rather than a demonstrated nesting mismatch. Add an independent discovery contract asserting complete, supported selected protocol and the expected server, then test ready/warming/error shapes against it.

The native suite openServer asserts only matching id and absent error. A malformed empty discovery result could pass that opener. It uses 90 seconds for catalog discovery, while the smoke allows up to 30 minutes and defaults to 10 minutes. The suite relies on the enclosing test gate to budget catalog work appropriately; actual build/discovery duration is still evidence to obtain.

## No-Catalog Artifact Roundtrip

The native end-to-end fixture uses the genuine gateway-owned os.agent.probe/v1 kind. Its JSON initial object is legitimate for that probe; there is no demonstrated invalid fixture. AJV and the first-party validator check the workflow and response shapes. The test creates, opens, validates, snapshots, reads resource/history, rejects duplicate creation, closes, launches a new process and compares persisted canonical pack/spr payloads.

Remaining holes: equality of bytes across restart does not independently prove the initial title/values survived encoding, and the history assertion only checks appliedEditIds is an array. Decode the canonical pair through an independent reader and verify the initial semantic value plus nonempty edit history. This test covers the gateway probe, not a real plugin artifact mutation.

The no-catalog launch omits --catalog but inherits process.env through spawnRawMcp. SEMIO_PLUGIN_CATALOG can therefore leak into it. Explicitly clear that variable in a no-catalog subject and assert plugin capability absence. Existing tests claiming that default openServer compiles the installed plugin registry conflict with the current explicit-catalog boundary: a truthy catalogHash also exists for an empty/plugin-free catalog and cannot prove installed-plugin discovery.

## Public Client Journey

The current journey supplies locale=en to context_resolve. Its handle invocation uses preparedActionHandle and the current response preparedHandle, not a proven obsolete action identifier.

The journey creates a real plugin artifact and checks mutation snapshots and revision identities, undo/redo, transaction rollback, exported bytes and subscriptions. It does not pass target or context on prepare, so it does not establish new selected-instance ownership or retained-locale behavior.

Inference cancellation is sent only after awaiting the inference_run response (lines 824–829), so that notification cannot prove interruption of active inference. job_cancel explicitly accepts PRECONDITION_FAILED, which can pass for an already finished job. inference_run acceptance only checks isError!=true; job progress only requires a string status. Require actual terminal success, a typed nonempty output semantically linked to the input artifact, then separately cancel a demonstrably active job and assert its terminal canceled result without committed effects.

The journey launches --no-bridge and requires channel=headless. It provides no successful UI focus, reveal, navigation or mounted-window history evidence. The native end-to-end UI tests establish typed missing-shell refusal only.

## Smallest Clean Selected-Identity Fix

ActionAdapter.target_instance normalizes catalog ownership, calls select_target returning Result<(), Fault>, then stores the caller selector clone. ShellArtifactChannel resolves exactly one genuine census entry but only retains its instance id in bound and the original selector in targets. Thus instanceId-only selection loses the resolved artifactRef; artifact_instance later cannot match that selection. Parsing serialized key strings would compound the wrong abstraction.

Change the internal channel selection result to a typed resolved selection record: genuine instance identity, plugin/app owner, canonical artifact reference and genuine selected document/revision identity obtained from the selected owner. The shell fills ownership from its single authenticated census entry and obtains the revision from that selected guest ReadHistory; the headless route fills it from its actual registered document binding. If revision acquisition remains in the existing prepare pipeline, return the resolved owner record first and complete its revision from that exact exchange before exposing the prepared handle. Persist this resolved record separately from caller selectors, preserving the opaque adapter slot and immutable prepared revision. artifact_instance reads resolved metadata. Never fabricate a revision from selector strings, plugin names or a guessed genesis document.

Deduplicate artifact lookup by genuine selected identity, rather than treating two selector aliases of one guest as two distinct documents. Continue refusing genuinely different guests for the same artifact unless an explicit selector disambiguates them. Add a shared neutral case for instance-only selection, two aliases of one guest, two genuinely different guests, unknown census owner, and revision change after prepare; run it through actual native channel selection and an independent relational oracle.

## Required Genuine Acceptance Evidence

1. Complete the real native binary build and tests, capture exact executable identity and actual schema emission/mirror validation; then run configured semio MCP to native discovery readiness.
2. Run isolated no-catalog probe persistence with environment cleared and independently verify decoded content after restart.
3. Run real catalog-backed plugin artifact create/open/read, explicitly targeted mutation, output/command result validation, semantic snapshot mutation, undo/redo and durable restart.
4. Run inference with real declared typed input/output and independent output interpretation, plus active cancellation and progress.
5. Attach the actual shell with multiple instances; resolve selected owner/revision, invoke an action against exactly one instance, inspect retained context, focus/reveal/navigate the real window, and prove history affects that instance only.
6. Record failures as failures. Surface census, schema tests, source tests, refusal paths and warming initialize are not substitutes for the successful native workflows above.

## Concrete Follow-up References

The discovery contract is authored in 🧭️protocol/🎯️mcp-server/🦀️.rs:225, with its native shape test in 🧭️protocol/🧪️tests/🔬️quick/🦀️.rs:120. The generated public MCP schema has no resultType/discovery export. The smallest harness assertion supplements InitializeResultSchema with resultType=complete, a supported protocol, serverInfo.name=semio and a nonempty version differing from warming.

Real inference request and result authority: ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🔨️modules/🏠️host/💡️inferences/🦀️.rs exports BITMAP_INFERENCE_REQUEST_SCHEMA and BITMAP_INFERENCE_CONTRACT. Service s.wfc.bitmap.solve uses payload schema s.wfc.bitmap.inference.request.v1, exactly one document canonical pack/spr pair or snapshot, and optional checkpoint bytes. Required artifact binding names document, encoding INFERENCE_ARTIFACT_PACK_BASE64. The public caller should name a genuine artifactId so gateway binding supplies those bytes. Result authority is the bitmap standard v1 any schema inferences JSON; progress unit cells; commit action pin-solution. Existing native unit tests beside the host inference file, lines 290–320, cover published binding and settled pair semantics. A successful small solve and a separately pending, bounded large solve are needed; cancellation after a completed request is insufficient.

Authored project IDs confirmed: @semio-tech/framework-os-mcp:test-canonical-stdio; @semio-tech/s-dev-composition:mcp-client-e2e and :generate; @semio-tech/trinity-jack-shell:run; @semio-tech/framework-os-host-rs:wasm; @semio-tech/framework-os-dev:serve-hold. These name entry routes, not evidence they were run. Artifact package @semio-tech/note-note-rs:build is an artifact crate build and must not be mislabeled a plugin component wasm rebuild. Root note/cad directories have no authored project.json. The current configured catalog folder is absent in the audited working state; its composition generate target must publish it before a configured catalog journey can succeed.

## Current Component Producer and Attached UI Targets

The authored producer is repo run @semio-tech/framework-os-dev:plugin note --catalog <published-catalog-root> (or cad/wfc as the one authored filter), not the artifact package build. The project forwards all arguments; the dev script removes the explicit catalog option into the catalog environment, then PluginBuildScript in 🔌️plugin/🏗️build/🏃️execution/🟦️.ts resolves selected plugin IDs and calls buildComponentClosureV1 with facet browser. The plan uses wasm32-wasip2; its filter explicitly recognizes a bare plugin ID such as note. This is a source-confirmed route, not a completed current guest publication. Current public result payload changes require a genuinely rebuilt and re-described guest before workflow acceptance.

Authored real interaction targets: @semio-tech/framework-os-dev:time-travel forwards verify time-travel with renderer react/wgpu; :two-human-cross-undo forwards verify two-human --journey cross-undo; :parity-journey requires explicit renderer paired/react/wgpu, locale en/de and output directory, with renderer server URLs. Keep output under this ticket generated directory. These harness routes do not themselves establish MCP attached focus/reveal success. The deployment rebuild contribution still names activate-s-react-prototype, but that exact target is absent from the inspected authored framework-os-dev project; do not substitute the stale chain for current target ownership.

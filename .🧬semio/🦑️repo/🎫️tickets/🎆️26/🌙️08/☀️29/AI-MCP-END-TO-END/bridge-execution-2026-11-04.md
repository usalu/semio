# Live Shell Artifact Route

## Current Verified Outcome

The production route now executes instance-owned actions, owner-qualified app/mode/plugin commands through the actual typed command port, and **genuine bounded media exports**. Export cancellation reaches the exact native submitted generation through `cancelMediaExport`; the route observes terminal status and never re-runs one-shot `MediaOut` to obtain metadata. The guest retains and encodes its own descriptor from the real completed export metadata before donating the output. Status wire version 24 carries those opaque descriptor bytes across Rust and TypeScript unchanged.

The latest executed Bun production-route and wire suite passed **24 tests, 0 failures, 196 assertions in 956 ms**. It covers AJV plan/schema agreement, shared Rust/TypeScript wire bytes above the JavaScript safe-number range, addressed-editor and declared-pane dispatch, full capability ownership and dotted IDs, cross-instance transactions, malformed plans, actual command-port encoding with unchanged responses and causal authority, app/mode/plugin command ownership, action/command cancellation, export success, export cancellation before/after submission and during chunk collection, rejection of every foreign export-handle coordinate while cancelling only the owned generation, valid work past 8200 pending polls, late cancellation and the actual operation deadline.

Export polling uses a schema-owned five-minute elapsed deadline and ten-millisecond poll cadence, independently of the output chunk/page cap. A monotonic clock and yield port allow deterministic tests; chunk collection retains the existing 8192-page bound. Seventy committed native binary descriptor companions were also updated through the real Pack codec and its `packUInt` mint after verifying exact original canonical roundtrip; 71 binary descriptors were examined and no failures remained.

The declared channel-version census passed for **35 consumers, pin 24, zero findings**. Stale consumer entries were removed, the moved framework channel constant received its own source-owner contribution, skew vectors now use previous/future versions 23/25, and the canonical lease descriptor was decoded/re-encoded through the real Pack codec. The descriptor remained 431 bytes; its SHA256 is `3243a4fa913cbff9f2d800abbd7721052015ce6af9223fcb520e1a89fa178e51`. Its dependent receipt and browser-actor digest bindings were updated. The standalone lease corpus has an opaque catalog identity rather than a complete trusted-profile source bundle; that fixture identity was rotated with a domain-separated digest of its new descriptor, without claiming to recompute a live trusted-profile generation. Deliberately malformed dependency vectors retained their duplicate/trailing-byte structure while updating only the encoded channel scalar.

Strict actual TypeScript checking executed through `repo tools exec tsc` without `--light`, with explicit route, host hookup, frame codec and codec-suite files. It exited 1: the broad import closure has unresolved workspace packages and peer errors. Its one owned diagnostic, an optional cancel port invoked inside a callback, was fixed by capturing the narrowed port. A second strict source check targets only the route and wire codec using matching ESNext/DOM libraries; it is pending. Temporary logs are under `🗑️generated/bridge-tsc.txt` and `🗑️generated/bridge-focused-tsc.txt`. Earlier local MCP bridge-check/typecheck/native-test runs were cancelled after remaining in repository planning/frontier work; they provide **no test-pass evidence**. The native executor is running the coherent frozen Rust wire/producer changes through the real Cargo ingress.

Native WGPU relay remains explicitly disabled and responds with a correlated refusal when no artifact owner is mounted. This slice does not claim native visible-session artifact/action parity. ReadDocument/ReadHistory retain their finite one-shot protocol; no per-command abort was invented. Live byte-progress streaming is not defined by the gateway's terminal AppCommand/AppFrames exchange; export work itself advances through bounded native progress/status slices and yields between turns.

## Export Continuation Files

Beyond the initial file list below, the export continuation updates the shared wire-contract status type, TypeScript and Rust frame codecs, Rust channel version/pin and wire fixture/schema, native funded media-result metadata publication, native export status producer, typed status fixture consumers, source-owner channel consumer declarations, and current-version descriptor/schema assets. The exact version asset paths and consumer fixture paths are listed in the later audit sections. No migration or compatibility runtime was added.

## Implemented

The real ShellHost hook now installs `createAgentArtifactRouteV1`, extracted into the adjacent owned agent artifact route module. It still calls the shell input funnel rather than a separate plugin mutation path.

Deferred plans carry plugin, app and instance identity. Prepare validates the complete plan and refuses foreign owners, undeclared actions, malformed JSON, extra document operations and config/draft operations. Transaction keys include the owning plugin/app/instance, preventing cross-instance commit or rollback and allowing separate instances to reuse a gateway transaction identifier. Duplicate prepare cannot overwrite an existing plan. Commit consumes a plan once and revalidates its owner/action.

Agent dispatch now selects a window owned by the addressed instance. Previously the active window's provenance caused `resolveDispatchTarget` to override the requested controller with the focused spawned editor, even when the MCP request addressed a different editor. Superseded input no longer produces a successful commit acknowledgment.

Cancellation now uses the shell's existing `ActionDispatchLifecycleV1.started` observer and its admitted `ActionOperationControlV1.cancel` port. In-flight actions are addressed by instance and request sequence. A cancellation arriving before admission is retained and delivered when the real operation control arrives; foreign-instance cancellation cannot touch it, and completed actions are removed. The original action returns a cancellation fault rather than a successful transaction acknowledgment after cancellation. Unsupported read/export cancellation explicitly reports the missing cancellable action.

Pure-command responses now explicitly warn that the live route prepares a deferred action and domain validation occurs at commit; non-object action arguments are rejected immediately.

## Test Contract

`ShellHost/🌉️agent-artifact-route/🧪️fixtures/🔣️.json` defines a language-neutral deferred plan schema, valid/invalid plan corpus and cross-instance transaction journey. The existing AgentBridge artifact-route suite drives the actual extracted production route and compares structural plan validation with independently compiled AJV JSON Schema validation.

Cases cover wrong-instance preparation/commit/rollback, same-id transactions in separate instances, duplicate prepare, replay, malformed/extra operations, undeclared action, focused-other-editor dispatch, target closure and superseded action outcome.

The neutral cancellation fixture also drives a before-admission cancellation test, including foreign-instance isolation and no cancellation replay after terminal settlement.

## Validation Status

Executed the co-located Bun neutral contract suite against the real production route via `repo tools exec --light bun -- test ./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🌉️agent-artifact-route/🧪️tests/🟦️.test.ts`: **5 passed, 0 failed, 41 assertions**, 253 ms. This shares the plan/journey/cancellation JSON fixture and independently compiled AJV schema oracle with the Vitest route suite. It verifies actual production callback arguments and cancellation controls without mounting React or invoking Vitest. The existing owned `agent-bridge-check` target now runs this suite before its Vitest tests.

The proper local repo MCP `@semio-tech/framework-renderer-react:agent-bridge-check` run was started after the test/fixture edit and before implementation, but remained at `boot.loaded` for over 140 seconds. Direct raw Vitest and direct project script attempts were rejected by the repository's required policy/parent-invocation authority; no bypass was introduced. Final run results will be appended here.

The proper `@semio-tech/framework-renderer-react:typecheck` target was also submitted through local repo MCP. Root confirmed the shared delay is the existing parser-source frontier and full workspace custody scan, prior to compilation.

The native `@semio-tech/os-renderer-wgpu-agent-elements-rs:test` target was submitted through the same local repo MCP after adding the native missing-route response and its shared fixture tests. It is also pending repository boot/frontier work.

## Native Carrier Audit and Fix

The native WGPU transport announces `BridgeFlags::default()` in `agent-bridge-door/🦀️.rs`; `relay_app_commands` is false. Its outgoing codec intentionally omitted ShellState, ShellStatePatch, Instances and AppFrames. Its shell pump consumes focus/reveal chrome commands only. Consequently the native carrier currently does not execute gateway artifact/action commands against its visible session, and successful headless gateway tests cannot establish native live UI mutation parity.

The accidental silent-drop branch for incoming `AppCommand` was replaced with an explicit `plugin.unavailable` AppFrames response on the exact instance and correlation. The native AppFrames encoder/decoder now participates in the existing canonical shared wire fixture replay. A new neutral missing-route fixture is asserted by Rust and the TypeScript answer handler; the missing-route message is identical across implementations.

Relay remains false because the native shell still needs a real session census and owned artifact/action/history adapter before it can honestly claim live relay capability. No flag was enabled without an implementation. Complete native live mutation parity is not established by this change.

## Files Changed

Paths below are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/`:

- `🧱️elements/🏛️ShellHost/🏗️dock-assembly/🪟️panel-reveal-and-tool-tabs/🟦️.ts`: installs the extracted live route.
- `🧱️elements/🏛️ShellHost/🌉️agent-artifact-route/🟦️.ts`: owned route implementation, validation, instance-scoped transactions, target window selection and actual operation cancellation.
- `🧱️elements/🏛️ShellHost/🌉️agent-artifact-route/🧪️fixtures/🔣️.json`: neutral plan schema/corpus and transaction/cancellation laws.
- `🧱️elements/🏛️ShellHost/🌉️agent-artifact-route/🧪️tests/🟦️.test.ts`: executed Bun production-route laws.
- `🧱️elements/🔗️AgentBridge/🧪️tests/🚏️artifact-routes/🟦️.ts`: Vitest production-route, cancellation, schema-oracle and native refusal parity regressions.
- `🧱️elements/🔗️AgentBridge/🧪️tests/🧫️fixtures/🗿️app-command-refusal/🔣️.json`: neutral missing-route response shared by Rust/TypeScript.
- `🧱️elements/🔗️AgentBridge/🧪️tests/🔬️wgpu-unit/🦀️.rs`: AppFrames canonical wire replay and correlated refusal regressions.
- `🧱️elements/🔗️AgentBridge/🎯️targets/🧊️wgpu/🧠️bridge-state-and-dialer/🦀️.rs`: explicit artifact route refusal.
- `🧱️elements/🔗️AgentBridge/🎯️targets/🧊️wgpu/📨️wire-and-gateway-messages/🦀️.rs`: native AppFrames codec.
- `🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts`: registers the Bun production-route laws with the existing owned agent bridge check.

Retained ticket input: `extract-agent-route.ts`. Retained report: this file. No AGENTS files, modifying Git commands or worktrees were used. No generated output files were added by this slice.

## Remaining Semantics

The existing live shell protocol uses a deferred plan rather than a true plugin dry run. This change validates plan ownership/shape/action declarations; it does not claim to validate domain arguments before dispatch. History and export ports expose promises without an abort argument. Typed action cancellation now reaches the existing owned cancel control, but immediate actions without that control cannot be interrupted after they commit. A request to cancel an unknown/completed/read/export operation is explicitly refused rather than presented as an abort.

## Independent API Review Addendum

The new native workspace validation calls match the public codec APIs: `document_kind_codec` is async and returns `Result<Option<DocumentKindCodec>, DocumentCodecRegistryError>`; `DocumentKindCodec::print_mirror` is public async and returns `Result<ArtifactTextFiles, VcsError>`; `GuestRuntimes::codec_print_mirror` accepts the exact compiled/schema/pack/spr/budget arguments used by the fallback. The plugin channel's `runtime` and `compiled` fields are public. The pair validator remains crate-private and root tests now use the public codec rather than violating facade privacy. No concrete compile mismatch was found in these call sites.

Native validation's activation scope checks bracket the codec call; cancellation stops caller waiting through the activation-job runner and prevents subsequent phases, but does not interrupt the finite codec pass. The owned interpreter has a `codec_print_mirror_observed` primitive; the `GuestRuntimes` facade does not expose its observed variant. This is phase-boundary cancellation, not mid-codec interruption.

The new repository root dev birth/composition functions were reviewed without edits. Their prefix/help guard, exact catalog target validation, deduplicated descendant declarations, resource identity fence and null-only fallback to the existing test birth line up with their imported APIs. No concrete mismatch was found.

`PluginWasmHandle.readAppDocumentPack` and `.readHistory` delegate to one-shot channel document/history reads, with no per-command abort protocol. Racing their promises would only discard a result and is not cancellation. `exportAppMedia` similarly delegates to one-shot `MediaOut`.

There is a genuine bounded media-export protocol on the same handle: `readAppDocumentIdentity`, `submitMediaExport`, `pollMediaExport`, `cancelMediaExport` and `takeMediaExportChunk`. It binds the exact instance/document/revision and returns a generation-owned handle. However its completion status exposes only MIME type and byte count, while MCP's current `Exported` frame requires the guest's full opaque `MediaArtifactDescriptor`. The owned native export result retains schema/media type but the bounded status protocol omits that metadata. Repeating one-shot `MediaOut` just to fetch the descriptor would repeat expensive work and undermine cancellation; synthesizing a descriptor would invent guest-owned schema/opaque metadata. Exact integration requires carrying the real descriptor over the bounded export protocol or explicitly redesigning the agent export metadata contract. Current AppCommand/AppFrames is also one terminal request/reply; no intermediate live export-progress frame is defined.


## Channel Version 24 Assets

- ✏️s\🔌️plugins\🪵️sourcing\🧩️extensions\🪵️beams\🧩️component\🔣️.json
- 🌎️hub\💡️inference\🧬️schema\🟦️.ts
- 🌎️hub\💡️inference\🧬️schema\🔣️.json
- ✏️s\🔌️plugins\🪵️sourcing\🧩️extensions\🪟️windows\🧩️component\🔣️.json
- ✏️s\🔌️plugins\🪵️sourcing\🧩️extensions\🧱️slabs\🧩️component\🔣️.json
- 🌎️hub\🧪️tests\🧫️fixtures\🧊️gis-map-frozen-binding-v1\🔣️.json
- 🌎️hub\🧩️compositions\🪵️sourcing\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\📏️layout\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🎬️sequence\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🪐️space\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\📋️forms\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🌊️flow\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🎪️demonstrator\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\💡️reasoning\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🌀️procedural\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🎥️shooting\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\💠️lowpoly\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🎞️animate\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🏭️process\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🀄️wfc\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\➗️mathematical\🧩️component\🔣️.json
- 🌎️hub\🗿️artifact-authority\🔏️trusted-catalog\🧪️tests\🧫️fixtures\🧱️generation-stage\🔣️.json
- 🌎️hub\🗿️artifact-authority\🔏️trusted-catalog\🧪️tests\🧫️fixtures\🧬️stdio-gis-bootstrap\🔣️.json
- 🌎️hub\🧩️compositions\🏛️architect\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🌿️vcs\🧩️component\🔣️.json
- 🌎️hub\🗿️artifact-authority\🔏️trusted-catalog\🧪️tests\🧫️fixtures\👥️two-package\🔣️.json
- 🌎️hub\🧩️compositions\✒️writer\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🏙️bim\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🌍️gis\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🏗️fem\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🖨️raster\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🔋️energy\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\📜️imperative\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\📖️playbook\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🖍️draw\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\📸️remodel\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🕸️dag\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\📕️norm\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🔱️trinity\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\📐️cad\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🧱️block\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🧩️puzzle\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🗒️note\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🗄️stdio\🧩️extensions\🧿️semio\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🗄️stdio\🧩️extensions\🛠️cad\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🗄️stdio\🧩️extensions\🖼️image\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🗄️stdio\🧩️extensions\🔺️mesh\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🗄️stdio\🧩️extensions\🔢️binary\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🗄️stdio\🧩️extensions\📘️pdf\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🗄️stdio\🧩️extensions\💼️office\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🗄️stdio\🧩️extensions\🏠️bim\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🗄️stdio\🧩️extensions\🎵️media\🧩️component\🔣️.json
- 🌎️hub\🧩️compositions\🗄️stdio\🧩️component\🔣️.json
- ✏️s\🔌️plugins\🏭️process\🧩️extensions\🪵️wood\🧩️component\🔣️.json
- ✏️s\🔌️plugins\🏭️process\🧩️extensions\🧱️concrete\🧩️component\🔣️.json
- ✏️s\🔌️plugins\🏭️process\🧩️extensions\🤖️robotic\🧩️component\🔣️.json
- ✏️s\🔌️plugins\🏭️process\🧩️extensions\🔩️metal\🧩️component\🔣️.json
- 🧰️framework\🛍️products\💻️os\🧪️tests\🧫️fixtures\📇️directory\🧭️document-open-plan-v1.json
- 🧰️framework\🛍️products\💻️os\🧪️tests\🧫️fixtures\📇️directory\🌐️browser-document-open-v1.json
- 🧰️framework\🛍️products\💻️os\🧪️testing\⚖️scale\🦀️.rs
- 🧰️framework\🛍️products\💻️os\🔨️modules\🧑‍💻dev\🧪️tests\🧫️fixtures\🚀️local-hub.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\🧑‍💻dev\🔖️channel-version\📌️pin\🔣️.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\🚪️io\🪪️bindings\🏛️catalog\🧬️schema\🔣️.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\🚪️io\🪪️bindings\🏛️catalog\🧪️tests\🧫️fixtures\🔣️.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\🔌️plugin\🧠️interpreter\🏃️runtime\🏗️startup\🧪️tests\🧫️fixtures\🔣️.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\🔌️plugin\🖨️describe\🧪️tests\🧫️fixtures\🧊️fresh-staging\🔣️.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\🔌️plugin\🖥️host\🧪️testing\🧩️component\🔣️.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\🔌️plugin\📇️registry\🧪️tests\🧫️fixtures\🧬️catalog-publication\🔣️.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\🔌️plugin\📇️registry\🛂️descriptor-verification\🧬️schema\🔣️.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\🔌️plugin\📇️registry\🛂️descriptor-verification\🧪️tests\🧫️fixtures\🔣️.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\🔌️plugin\📇️registry\🔎️discovery\🧪️tests\🧫️fixtures\🔣️.json
- ✏️s\🔌️plugins\🌊️flow\🧩️extensions\🧮️math\🧩️component\🔣️.json
- ✏️s\🔌️plugins\🌊️flow\🧩️extensions\🧠️logic\🧩️component\🔣️.json
- ✏️s\🔌️plugins\🌊️flow\🧩️extensions\🖍️draw\🧩️component\🔣️.json
- ✏️s\🔌️plugins\🌊️flow\🧩️extensions\🔤️primitive\🧩️component\🔣️.json
- ✏️s\🔌️plugins\🌊️flow\🧩️extensions\📝️text\🧩️component\🔣️.json
- ✏️s\🔌️plugins\🌊️flow\🧩️extensions\📖️dictionary\🧩️component\🔣️.json
- ✏️s\🔌️plugins\🌊️flow\🧩️extensions\📐️brep\🧩️component\🔣️.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\🔌️plugin\🌐️browser-bundle\🧾️describe\🧪️tests\🧫️fixtures\🔣️.json
- ✏️s\🔌️plugins\🌊️flow\🧩️extensions\📃️list\🧩️component\🔣️.json
- ✏️s\🔌️plugins\🌊️flow\🧩️extensions\🏗️bim\🧩️component\🔣️.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\🔌️plugin\🌐️browser-bundle\🎯️action-handoff\🧬️schema\🔣️.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\🔌️plugin\🌐️browser-bundle\🎯️action-handoff\🧪️tests\🧫️fixtures\🔣️.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\📺️renderer\🧑‍🎨engine\🧪️tests\🧫️fixtures\🧊️wgpu\⌨️native-entrypoint\🚪️headless\🔣️.json
- ✏️s\🔌️plugins\📜️imperative\🧩️extensions\🧮️math\🧩️component\🔣️.json
- ✏️s\🔌️plugins\📜️imperative\🧩️extensions\🧠️logic\🧩️component\🔣️.json
- ✏️s\🔌️plugins\📜️imperative\🧩️extensions\📣️effect\🧩️component\🔣️.json
- ✏️s\🔌️plugins\📜️imperative\🧩️extensions\📝️text\🧩️component\🔣️.json
- ✏️s\🔌️plugins\📜️imperative\🧩️extensions\🎮️control\🧩️component\🔣️.json
- ✏️s\🔌️plugins\📐️cad\🧩️extensions\🔥️aec-building-energy\🧩️component\🔣️.json
- ✏️s\🔌️plugins\📖️playbook\🧩️extensions\🌀️procedural\🧩️component\🔣️.json
- ✏️s\🔌️plugins\📐️cad\🧩️extensions\📐️spatial-shape\🧩️component\🔣️.json
- ✏️s\🔌️plugins\📐️cad\🧩️extensions\🏢️aec-building\🧩️component\🔣️.json
- ✏️s\🔌️plugins\📐️cad\🧩️extensions\🏛️aec-building-structure\🧩️component\🔣️.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\📡️spr\🧵️channel\🧪️tests\🧫️fixtures\🪪️document-identity-wire\🔣️.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\📡️spr\🧵️channel\🧪️tests\🧫️fixtures\🎬️media-export-wire\🔣️.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\📡️spr\🧵️channel\🦀️.rs
- 🧰️framework\🛍️products\💻️os\🔨️modules\🌉️mcp\💡️inference\🧪️tests\🧫️fixtures\🧩️neutral-descriptors\🔣️.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\🌉️mcp\🏠️workspace\🧬️schema\🔣️.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\🌉️mcp\🏠️workspace\🧪️tests\🧫️fixtures\🔐️hub-live-catalog\🔣️.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\📇️directory\🧬️schema\🔣️.json
- 🧰️framework\🛍️products\💻️os\🔨️modules\📇️directory\🧪️tests\🧫️fixtures\🔏️document-execution-target-lease-v1\🔣️.json
- 🧰️framework\🔨️modules\🧾️app-channel\🔢️request-sequence\🟦️.ts
- 🧰️framework\🔨️modules\📇️directory\🧬️schema\📂️document-open\🟦️.ts
- 🧰️framework\🔨️modules\🛂️manifest\🧪️tests\🧫️fixtures\📦️catalog-package\🔣️.json
- 🧰️framework\🔨️modules\🌐️browser-bundle\🎯️action-handoff\🟦️.ts

## Bounded Status Consumer Fixtures

- 🧰️framework/🛍️products/💻️os/🧪️tests\🧾️app-channel-codec\🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer\🧑‍🎨engine\🧪️tests\🎬️wgpu-browser-media\🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer\🧑‍🎨engine\🧪️tests\📦️document-pack-wire-adapter\🟦️.tsx
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer\🧑‍🎨engine\🧱️elements\🎬️MediaTransportHost\🧪️tests\♻️lifecycle\🟦️.tsx
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer\🧑‍🎨engine\🧱️elements\🎬️MediaTransportHost\🧪️tests\♻️lifecycle\🌐️browser\🟦️.tsx

## Remaining Version 23 Literal Audit
[
  {
    "path": "🌎️hub\\🗿️artifact-authority\\🔏️trusted-catalog\\🧪️tests\\🧫️fixtures\\🔗️compiled-dependencies\\🔣️.json",
    "values": [
      {
        "index": 4471,
        "length": 52,
        "valueStart": 4507,
        "value": 23,
        "encoded": true
      },
      {
        "index": 4950,
        "length": 52,
        "valueStart": 4986,
        "value": 23,
        "encoded": true
      },
      {
        "index": 5443,
        "length": 52,
        "valueStart": 5479,
        "value": 23,
        "encoded": true
      },
      {
        "index": 5930,
        "length": 52,
        "valueStart": 5966,
        "value": 23,
        "encoded": true
      }
    ]
  },
  {
    "path": "🧰️framework\\🛍️products\\💻️os\\🔨️modules\\📇️directory\\🧪️tests\\🧫️fixtures\\🔏️document-execution-target-lease-v1\\🔣️.json",
    "values": [
      {
        "index": 11400,
        "length": 52,
        "valueStart": 11436,
        "value": 23,
        "encoded": true
      }
    ]
  }
]

## OS Consumer Census
{
  "pin": 24,
  "findings": [
    {
      "path": "🧰️framework/🛍️products/💻️os/🟦️.ts",
      "problem": "occurrences",
      "detail": "declares 1 version literal(s), holds 0"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️schema/🔣️.json",
      "problem": "occurrences",
      "detail": "declares 1 version literal(s), holds 0"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🧫️fixtures/🧬️catalog-publication/🔣️.json",
      "problem": "hostile",
      "detail": "declares 2 hostile literal(s), holds 3: a literal equal to a hostile value is a stale pin or an undeclared vector"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/🚀️local-hub.json",
      "problem": "occurrences",
      "detail": "declares 6 version literal(s), holds 8"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json",
      "problem": "drift",
      "detail": "1 literal(s) at 23, the pin is 24; derived values to recompute by the owner: the lease's catalog generation and its Pack-encoded descriptorHex"
    }
  ]
}

## Final Declared Consumer Census
{
  "generation": "a112b0d7cb64926cdf204fd33648799bfa9bfbf5b059d236f6806becf682086d",
  "consumers": 35,
  "pin": 24,
  "findings": []
}

## Canonical Binary Descriptor Assets


{"failures":[]}

## Canonical Native Integer Descriptor Assets


{"scanned":71,"failures":[]}

## Native Tagged Integer Descriptor Updates


{"scanned":71,"failures":[{"path":"✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/✒️writer/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/➗️mathematical/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🀄️wfc/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🌀️procedural/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🌊️flow/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🌍️gis/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🌿️vcs/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🎞️animate/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🎥️shooting/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🎪️demonstrator/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🎬️sequence/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🏗️fem/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🏙️bim/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🏛️architect/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🏭️process/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/💡️reasoning/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/📋️forms/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/📏️layout/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/📐️cad/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/📕️norm/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/📖️playbook/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/📜️imperative/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/📸️remodel/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🔋️energy/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🔱️trinity/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🕸️dag/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🖍️draw/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🖨️raster/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🗄️stdio/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/🎵️media/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/🏠️bim/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/💼️office/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/📘️pdf/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/🔢️binary/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/🔺️mesh/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/🖼️image/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/🛠️cad/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/🧿️semio/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🗒️note/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🧱️block/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🪐️space/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🌎️hub/🧩️compositions/🪵️sourcing/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"},{"path":"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🛂️.descriptor.semio","error":"Error: PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint"}]}

## Canonical Owned Pack Descriptor Updates

- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/🧩️component/🛂️.descriptor.semio
- ✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/✒️writer/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/➗️mathematical/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🀄️wfc/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🌀️procedural/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🌊️flow/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🌍️gis/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🌿️vcs/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🎞️animate/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🎥️shooting/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🎪️demonstrator/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🎬️sequence/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🏗️fem/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🏙️bim/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🏛️architect/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🏭️process/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/💡️reasoning/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/📋️forms/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/📏️layout/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/📐️cad/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/📕️norm/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/📖️playbook/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/📜️imperative/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/📸️remodel/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🔋️energy/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🔱️trinity/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🕸️dag/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🖍️draw/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🖨️raster/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🗄️stdio/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/🎵️media/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/🏠️bim/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/💼️office/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/📘️pdf/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/🔢️binary/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/🔺️mesh/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/🖼️image/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/🛠️cad/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/🧿️semio/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🗒️note/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🧱️block/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🪐️space/🧩️component/🛂️.descriptor.semio
- 🌎️hub/🧩️compositions/🪵️sourcing/🧩️component/🛂️.descriptor.semio
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🛂️.descriptor.semio
{"scanned":71,"failures":[]}

## Executed Broad Strict TSC Result
{
  "diagnostics": 611,
  "owned": [
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🌉️agent-artifact-route/🟦️.ts(181,95): error TS2722: Cannot invoke an object which is possibly 'undefined'.\r"
  ],
  "missing": [
    "'@semio-tech/ui/modules/dock-skeleton'",
    "'@semio-tech/ui/scene/components'",
    "'@semio-tech/framework-interaction/schema'",
    "'@semio-tech/framework-manifest/schema'",
    "'@semio-tech/framework-manifest'",
    "'@semio-tech/framework-manifest/generated/ui-axes'",
    "'@semio-tech/ui-contract/retained/instance'",
    "'@semio-tech/ui-contract/retained/operations/wire/pages'",
    "'@semio-tech/ui-contract/retained/resident'",
    "'@semio-tech/ui-contract/retained/host/return-content'",
    "'@semio-tech/ui-contract/retained/host/input'",
    "'@semio-tech/framework-effect'"
  ]
}


## Command Ownership and Final Source Verification Addendum

The actual catalog compiler publishes app commands as plugin.app.cmd.command, mode commands as plugin.app.mode.mode.command, and plugin commands as plugin.cmd.command (catalog-compile Rust lines121–148). capability-builders projects resolve_command_audience; these are not blanket human-only declarations. The previous shell route accepted only app actions and therefore rejected these advertised command capabilities. The neutral fixture now carries commandCases with full expected CommandInvocation addresses, including three foreign-owner refusals. Native implementation ownership is delegated to server_execution using that same fixture.

The staged schema now explicitly carries invocationKind and modeId alongside plugin/app/instance/window-kind identity. Prepare and commit recheck the current declarations. Published window actions retain their declared window kind instead of inheriting another pane. Full capability identity is compared against declared IDs, preserving dotted action IDs and rejecting foreign same-leaf IDs.

The input ledger has an optional owned commandAddress. Commands bypass shell action intercepts and action-only browser-actor handoff, pass declaration and viewer checks, and enter the existing program-dispatch operation lifecycle. program-invocation calls the actual handleCommand port using the owned address, unchanged arguments/view/causal authority, returning the original response to existing history/effects/source-revision/work/settlement code. It never translates commands into action invocations. Typed-operation cancellation removes commandAddress so the real cancellation action reaches the action port.

Additional edited sources: ShellHost/📨️program-invocation/🟦️.ts (new), ShellHost/🎯️input-ledger/🟦️.ts, ShellHost/🏁️operation-settle/🟦️.ts, ShellHost/🖱️shell-actions/🧭️input-funnel/🟦️.ts, ShellHost/🖱️shell-actions/🧭️input-funnel/🚀️program-dispatch/🟦️.ts, and the route/schema/fixture/Bun tests.

Test-first command regression initially failed2 tests (advertised command rejected and staged schema mismatch); after integration the exact production Bun suite passed20 tests153 assertions. The suite is registered before the existing Vitest phase in AgentBridgeCheckScript. No raw compiler/test runner bypass is used.

Actual TypeScript checking used the public ingress repo tools exec tsc, without --light. The first broad route/host/frame/codec source check completed exit1 with611 diagnostics in the existing import closure; one owned optional cancelMediaExport diagnostic was fixed by capturing the guarded function. A subsequent focused route+frame-codec check completed exit1 with48 dependency-resolution/type diagnostics and zero diagnostics in those owned files. The final command route/program-dispatch check completed exit1 with604 dependency-closure diagnostics and zero diagnostics in agent-artifact-route, program-invocation, program-dispatch, input-funnel, input-ledger, or operation-settle. Missing framework-onboarding, framework-effect, framework-manifest aliases and unrelated closure errors remain; these are not reported as passing typechecks.

Exact final typecheck ingress: repo tools exec tsc -- --noEmit --strict --skipLibCheck --module esnext --moduleResolution bundler --target es2022 --lib esnext,dom,dom.iterable --allowImportingTsExtensions --resolveJsonModule --esModuleInterop --jsx react-jsx [ShellHost/🌉️agent-artifact-route/🟦️.ts] [ShellHost/🖱️shell-actions/🧭️input-funnel/🚀️program-dispatch/🟦️.ts]. Logs remain only in ticket generated output pending parent cleanup. Native compilation is owned by parent/server_execution and is not claimed complete here. WGPU relay stays honestly false because it has no genuine session census and addressed command queue.

The final additional test calls the actual typed-operation cancellation factory on a command and invokes the production program-input selector. It proves commandAddress is removed, the real action cancellation port is used, and original window provenance is preserved. Final focused Bun result:21 pass0 fail156 assertions951ms.


## Actual Command Result Carriage

The private MCP port Emit and TransactionCommitted variants now carry output:Option<serde_json::Value>. This does not alter the AppChannel binary ABI or channel version24. The version1 shell JSON payload adds optional output on those two replies, with both codecs preserving absence versus explicit JSON null. The owned schema is framework/modules/mcp/shell-channel/schema (emoji source path below); the private shell payload previously had fixtures but no owned output-bearing schema. It validates the existing frame fields plus arbitrary JSON guest output. Public untrusted envelopes and prepared-record retention are owned by parent, not synthesized in these carriers.

The native actual PureCommand Emit match decodes the guest-owned output Pack through store::pack_rt::decode_wire_value and the existing firstparty DslValue→serde_json conversion. It does not fabricate a guest result. Native committed frames carry None unless the native implementation actually retained a corresponding result; the parent dispatcher can retain real pure/nonapplying output for readonly no-op invocation.

The React bridge still performs no applying invocation during PureCommand or transactionPrepare. The real onAction/handleCommand callback executes at commit. InputOutcome.applied now optionally preserves actual guest response.output after the normal operation settle; the shell commit frame carries it without assigning trust. programInputResultV1 never adds output to refused/superseded outcomes and distinguishes undefined from null. Existing effects, history patching and source work ownership remain on the same dispatch path.

The new regression first failed because committed output disappeared. The final production Bun suite passes23 tests175 assertions998ms, including actual committed result without early execution, shared Rust/TS payload equality, AJV schema validation, explicit null/absent behavior and successful-only result retention. The Rust shell-channel quick test consumes the same5 result fixture rows; execution is pending parent/native focused compilation.

Additional edited files for this result lane:
- 🧰️framework/🔨️modules/🤖️mcp/🐚️channel/🧬️schema/🔣️.json (new)
- 🧰️framework/🔨️modules/🤖️mcp/🐚️channel/🟦️.ts
- OS/mcp/🔀️dispatch/🔌️channel-port/🦀️.rs: enum fields, testkit optional result outputs and force_result_output(instance,preview,committed)
- OS/mcp/🏠️workspace/🧩️artifact-app-frames/🦀️.rs: real guest output decoder
- OS/mcp/🐚️channel/🦀️.rs and 🧪️tests/🔬️quick/🦀️.rs
- OS/mcp/🐚️channel/🧪️tests/🧫️fixtures/📦️command-results/🔣️.json (new shared neutral corpus)
- OS/mcp/🏠️workspace/🔀️routing-and-shell-routed/🦀️.rs, 🤝️hub-relay/🦀️.rs, and quick hub-relay-and-workspace-guards test: preserve result while decorating relay status
- ShellHost/🎯️input-ledger/🟦️.ts, 📨️program-invocation/🟦️.ts, 🖱️shell-actions/🧭️input-funnel/🚀️program-dispatch/🟦️.ts, 🌉️agent-artifact-route/🟦️.ts and its Bun tests

The public nonlight tsc result-lane check is session75140 (updated route/program-dispatch/shell JSON codec roots), output only under ticket generated. No clean typecheck or native test pass is implied until their actual completion.

Result-lane tsc session75140 completed exit1 with604 dependency-closure diagnostics, zero in the owned route/program-invocation/program-dispatch/input-ledger/operation-settle/shell JSON codec files. This is executed evidence of no owned diagnostic, not a passing workspace typecheck. The native relay quick test also now asserts the actual guest JSON survives relay-status decoration.


## Independent Public Result Review

Reviewed parent changes to reports, capability-output.json, untrusted provenance/constructors, public dispatcher preparation/invocation/replay, and command-output.json. No concrete Rust API mismatch or foundation→schema compiled dependency cycle was found by source review. The foundation report schema_with helper includes the owned JSON file rather than importing the schema crate; its extracted Envelope has no dangling external refs. The include_str relative path resolves from reports to the sibling untrusted-content-schemas directory. Serde internally tagged UntrustedAuthors::Unattributed matches the closed {kind:unattributed} schema; source/notice values match their narrowed public contract. Preview and commit paths use real output, retain the prepared envelope, prefer actual committed output, and keep no-op readonly calls out of TransactionPrepare/Commit. Idempotency encloses invoke_uncached so the no-change early return still reaches the same replay store.

An independent executed Bun/AJV + node:crypto oracle checked all3 parent command-output fixture rows against the owned capability-output Envelope and SHA256 of recursively sorted JSON (matching serde_json object ordering): readonly-text, committed-result, readonly-boolean all schema-valid with matching hashes. This is a schema/hash oracle, not execution of the pending native dispatcher regression.

Outstanding public mirrors at review time: mcp/🧬️schema/🔣️.json lacks InvocationReport.untrusted, has PreparedActionReport.preview:{}, lacks capability-output source and unattributed author; mcp/🧬️schema/🟦️.ts has the same stale generated export types. Parent was notified; no edits were made to those owned public mirrors.

Private null output remains distinct from missing at both shell codecs. Public untrusted_capability_output deliberately collapses raw JSON null to nullable no-envelope; false/string/object are enveloped. This follows the parent nullable-result design but means explicit null is not a separately attributed public payload.

The existing completion contract exposes actual PluginOperationCompletion.terminalOutput. The frontend program dispatcher now forwards this for admitted typed jobs instead of the initial operationId/generation token; synchronous commands retain immediate response.output. A job with no terminal completion does not fabricate a final result. Executed production tests now pass23 tests179 assertions849ms, including terminal value/null/missing and synchronous result selection.


## Public Mirror Refresh and Semantic Verification

Parent authorized refresh of the public output mirrors after the review. Updated mcp/🧬️schema/🔣️.json using17 targeted jsonc-parser AST edits, preserving all22 existing numeric float spellings (including0.0) and leaving unrelated source text untouched. InvocationReport.untrusted now has the nullable capability envelope and is required; PreparedActionReport.preview is closed over opsCount/warnings/untrusted; every inlined generic untrusted notice/provenance mirror plus standalone provenance now admits capability-output and unattributed. No new compiled crate dependency was introduced.

Regenerated mcp/🧬️schema/🟦️.ts using the exact existing pure renderSchemaTypescript/renderType/GENERATED_PRELUDE source from mcp/📦️packages/🦀️rust/📜️script.ts. The renderer was read and evaluated in memory through Bun TypeScript transpilation, not reimplemented or copied into a ticket source tree. The retained ticket input refresh-mcp-output-mirrors.ts records the AST edits and invocation. Actual native semio-os-mcp schemas byte-for-byte emitter comparison remains pending the binary build; only semantic/source-renderer equivalence is claimed here.

Executed parent source law at mcp/🧬️schema/🧪️tests/🧷️capability-output/🟦️.ts:2pass0fail19 assertions465ms, reversing the expected stale-mirror red test. Independently resolved all74 public exports through strict AJV draft07 without error. Additional production route tests compare the regenerated firstparty schema walker with AJV for nullable output, properly enveloped false/object content, rejected raw false/missing output, and closed prepared previews. Latest route suite:24pass0fail196 assertions956ms.

# Native MCP Execution Repairs

## Changes

- Artifact reads, native probe creation, snapshots, and validation no longer require an unrelated plugin-owned capability. Real plugin artifact creation still resolves an installed owner, and media export requires its owner.
- Artifact validation now reads the live canonical pack and SPR pair, resolves its declared schema, and runs the exact document owner codec validation fence. Linked/component DocumentKindCodec.print_mirror explicitly defines rejection as a pair that is not a document of its kind. If no registered codec exists, the owning component receives codec_print_mirror. Unknown artifacts fail NOT_FOUND; absent owner codecs remain explicit PLUGIN_UNAVAILABLE.
- Validation returns artifactId, schema, validation=canonical-pair, valid, and untrusted owner diagnostics. It claims canonical pair validity, not semantic completeness. The direct tool runs as a cancellable progress job and the validation resource uses the same path.
- The schema mirror now describes this validation result.
- Native CLI admission validates the original host root policy before serving, preventing later wake-driver expect panics for zero identity/fuel/deadline policies.
- Stale native tests now pass a shared, explicitly authored host policy fixture to mandatory constructors and CLI admission. The independent serde_json decoder cross-checks the owned policy parser.
- Language-neutral canonical-pair cases cover valid JSON, malformed input and a nonfinite number. The owned canonical pair decoder is compared with the independent serde_json decoder. Native artifact tests cover successful creation/open/validation with no plugins.

## Verification

All new include_str fixture paths were checked and resolve. Required repository MCP native test target @semio-tech/framework-os-mcp-rs:test was started through the ticket repo-mcp.ts wrapper with agent semio-mcp-native. Test result is pending repository graph/build completion. The parent agent owns the main native build and independent real-binary workflow verification.

## Remaining Boundaries

The domain-neutral binary installs no product-specific remote inference protocols by design; local inference routes already execute installed component declarations. Hub artifact creation still requires the hub document-create authority. Validation uses the existing canonical pair codec fence rather than introducing a fabricated wire validate command.

## Icon Contract Compiler Admission Repair

The first cargo-ingress check/test attempt reached source admission and failed before compiler dispatch: semio-framework-icon-contract exhausted its finite 32-file roster on a stale generated identity file that still included 250 SVGs. Its current owner generator already publishes SVGs only through the runtime icon pack. Synchronized only the generated Rust identity file with that existing generator; no source limits were raised or bypassed.

Verification through the same authored Rust closure reader and owner projection equality:

```json
{
  "ownerProjectionMatches": true,
  "files": 3,
  "bytes": 64545,
  "activeLines": 2131,
  "inputs": [
    {
      "path": "🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🪪️icon-name/🦀️.rs",
      "bytes": 63325,
      "activeLines": 2101
    },
    {
      "path": "🧰️framework/🔨️modules/🖼️assets/🔣️icons/🧬️schema/🦀️.rs",
      "bytes": 278,
      "activeLines": 10
    },
    {
      "path": "🧰️framework/🔨️modules/🖼️assets/🔣️icons/🧬️schema/🧾️value/🦀️.rs",
      "bytes": 942,
      "activeLines": 20
    }
  ]
}
```

## Edited Paths

All MCP paths are relative to framework OS modules/mcp.

- bootstrap Rust CLI policy reader and bin-quick tests; root quick options tests.
- workspace gateway-backend artifact-creation-and-routing implementation; headless-workspace-state resource validation.
- artifact tools implementation and quick tests.
- artifact-tool-schemas Rust definition and module JSON schema mirror (only ArtifactValidateOutput changed; unrelated original schema bytes preserved).
- workspace quick tests for folder/probe resources, capability routing, backbone egress and authenticated Hub workspace; workspace long tests.
- inference quick and binding-cancellation tests; context quick tests: explicit mandatory host policy constructor arguments.
- new fixtures: MCP tests/fixtures/host-driver-policy.json; host-policy-admission.json; workspace tests/fixtures/canonical-pair-validation.json.
- framework assets/icons/generated/icon-name Rust identity projection regenerated exactly from its current owner generator to repair compiler input roster.

Focused cargo-ingress validation test was rerun after the stale icon source correction. Result remains pending. No compiler/test pass is claimed.

## Schema Projection Oracle

Regenerated the TypeScript projection with the existing owner renderer against the updated JSON document. Actual native Rust emitter comparison remains pending compilation. The owned TypeScript validator and independent Ajv agree on all five validation output cases, and Ajv resolves all74 exports.

```json
{
  "exports": 74,
  "cases": [
    {
      "id": "valid-pair",
      "accepted": true
    },
    {
      "id": "invalid-pair",
      "accepted": true
    },
    {
      "id": "missing-schema",
      "accepted": false
    },
    {
      "id": "wrong-validation",
      "accepted": false
    },
    {
      "id": "unexpected-field",
      "accepted": false
    }
  ],
  "ownedAjvAgree": true
}
```

## Independent API Review and Cancellation Boundary

The bridge agent independently checked the registered codec return types, public print_mirror async signature, and GuestRuntimes print-mirror call signature. These match the new validation call sites. The direct tool cancels the caller wait immediately through the existing activation worker selector. Validation checks its scope before and after decoding; an already running owner codec completes its bounded pass before the worker observes cancellation. Mid-codec interpreter interruption is not claimed.

## Compiler Type Inference Blocker

The parent native check reached the Rust compiler and failed with E0283 in store sync-native folder persistence: four literal messages called .into() before a helper that itself accepts impl Into<String>. Removed only those redundant conversions so Rust infers the literal input directly. No behavior or diagnostic text changed. Native test/check retry remains required.

## Test Composition Repair

Inference-tools unit tests inject neutral package descriptors into their workspace dependency. The workspace hook was previously cfg(test)-only, so it disappeared when the workspace was a dependency of another tested crate. Added an explicit workspace-host testkit feature and enabled it only from the inference-tools dev-dependency. The field, initializer and discovery branch share the same gate. Production defaults do not enable the fixture hook. Published artifact_validate capability text now describes canonical owner-codec validation in English and German.

Cancelled obsolete planning-only test invocation and stale pre-fix filtered compiler run; started a fresh current-tree focused cargo test with MCP PID31084 (session7364). Awaiting actual terminal evidence.

## Mounted Instance Source Admission Repair

Fresh current-tree test reached source admission and found plugin-instance-app at263486 bytes,1342 above its unchanged262144-byte contract (16files,3137active lines). Replaced obsolete migration narrative and repeated long rustdoc with concise contracts, retained current ownership/lifetime details in the owning instance README, and grouped consecutive imports by owner. No executable statements or resource limits changed.

The same authored Source closure reader now reports:

```json
{
  "files": 16,
  "bytes": 254563,
  "activeLines": 3090,
  "headroomBytes": 7581
}
```

## Current-Tree Test Attempts

After bridge wire24 declarations became coherent, the focused cargo test was retried. One attempt failed before compiler/source admission on a transient Windows EPERM while reading a live lease queue row (structured result mcp-37412.json under generated). Retried once through the same owned cargo ingress without bypassing admission or deleting lease files. No native test pass is claimed yet.

After bridge native metadata coherence, current instance closure remains within the original contract:

```json
{
  "files": 16,
  "bytes": 254618,
  "activeLines": 3091,
  "headroomBytes": 7526
}
```

## Native Compiler Progress And Optional Cache Publication

The current owned native test reached real Cargo compilation after finite source preparation. Its authoritative cargo phase ledger for PID 38120 recorded inputs ready at 633128 ms, graph ready at 723915 ms, all 450 authored source units admitted at 750035 ms (2912 files, 34016409 bytes), capacity ready at 895989 ms, and compiler child start at 897832 ms. Before that child existed, the immutable Bun loader used CPU rather than waiting on a nested resource grant: the existing doctor OS command port observed process CPU increasing from 1106.56 to 1108.02 seconds over approximately 1.45 seconds. The Windows process-list parser currently reports CPU percent as zero, so that field cannot establish idleness. No resource cycle, violation or unresolved nested waiter was observed. The current test subsequently compiled native dependencies; its terminal result remains pending.

The optional Windows rustc wrapper publication warning had two concrete causes. Bun compile appends .exe when the requested temporary path ends in .tmp, so chmod addressed a path that was never emitted. Also the opportunistic directory collector could remove another producer staged file, and deleting older published digests could remove an executable still used by another Cargo invocation. Publication now stages with .tmp.exe, only cleans its own stage on failure, and retains peer published immutable generations. No cache limits or fallback policy changed.

A new language-neutral JSON fixture and JSON Schema describe interleaved producers. The implementation is checked against an independent SQLite model, with Ajv validating the fixture. The existing actual Windows executable test compares compiler argument transport with an independent Python oracle, explicitly configured for UTF-8. The complete focused Windows wrapper suite passed through repo: 5 passed, 1 preexisting platform skip, 0 failed, 41 assertions. This includes the actual native wrapper compile and quoted Unicode argument transport, immutable-generation retention, fallback in both languages, POSIX naming, and producer interleaving.

The first current-tree filtered native test exceeded the ticket MCP client 30-minute request timer while compiling manifest-app-surface. This is a client timeout, not a passing native test or compiler diagnostic. The same owned cargo test was relaunched with a finite 60-minute client timeout through repo; its reusable input is native-canonical-pair-test.json. The ticket wrapper now supports SEMIO_TICKET_TIMEOUT_MS while retaining the original 30-minute default. No native source changed between these two runs.

## Native Declared Command Parity

The catalog advertises app commands as plugin.app.cmd.command, mode commands as plugin.app.mode.mode.command, and plugin commands as plugin.cmd.command. The previous native facade built only ActionInvocation, and the plugin instance PureCommand decoder also accepted only ActionInvocation. Both owners now accept genuine CommandInvocation. Declaration projection compares complete catalog identities rather than splitting dotted app IDs. Shared browser/native commandCases contain three declared owner addresses and three foreign owner refusals; a new explicit commandDeclarations roster makes authority independent of expected answers. The native test compares its real declaration address projection and owned Pack decoder with independent serde_json-decoded expected addresses from the same corpus that the browser implementation executes. Native execution of that new test is pending compilation.

App and mode commands resolve their actual command registry, validate execution classification, construct the same original typed command as native UI command dispatch, then share the existing nonapplying preview completion and retained cancellation lease. Their operations still enter the ordinary prepare/commit/undo history transaction exactly once. A plugin-level command invokes the real PluginProgram.handle_plugin_command. Output and diagnostics without mutation, requested effect, event, history or UI work are admitted as an actual read-only computation. Work requiring those publication lanes returns plugin.unavailable with the remedy to attach the live shell command executor; no empty transaction claims to have committed effects. The public actual-result carrier is coordinated with the root agent and bridge executor.

PureCommand previously always constructed ActionMeta with no view state, while addressed actions require explicit language and terminology. It now reads the genuine retained instance surface context when one exists. The schema owner is framework/manifest/view-model/ViewModel, whose locale and terminology are nonoptional. The native AppCommand::PureCommand wire carries command and head only, unlike the existing full invocation event which separately carries view_state. No invented default or ABI field was introduced: actions without retained caller context and modes without their active retained context return retryable plugin.unavailable with a live surface remedy. A bare headless app command that does not depend on surface context may use its genuine typed producer. Attached frontend commands use the existing owned command funnel, which supplies real view context and settles effects/history/lifecycle.

Command parity edits belong to workspace/plugin-artifact-channel/inference-route-and-actions, workspace/artifact-app-frames, plugin/instance/command-routes/staging/actions, and plugin/reactor/runtime/exchange/plugin-exchange. The shared ShellHost agent-artifact-route fixture gained only commandDeclarations; workspace plugin-capability-routing quick tests consume it. A plugin global read-only result also refuses nonempty inverse-history groups, preserving the transaction boundary.

## Cargo Response-File Boundary

The owned focused native test completed with a concrete infrastructure error: ENAMETOOLONG uv_spawn while the optional unit-cache wrapper forwarded the semio-framework facade compile. Source admission passed; Cargo failed before that rustc instance started, so this is not native source or test success. The wrapper parsed Cargo response files for cache identity, then mistakenly forwarded the expanded arguments, exceeding the Windows process command limit. It now keeps expanded arguments only for identity/admission and forwards the original Cargo arguments, including its live response-file reference, unchanged. No new response file, bypass or cache limit change was introduced.

A neutral JSON fixture plus JSON Schema creates 2000 quoted Unicode arguments beyond 32768 characters. The compiled native wrapper test reproduced the same ENAMETOOLONG before the fix; afterward the complete Windows wrapper suite passed6tests with1preexisting skip and48assertions. The independent Python argparse fromfile parser agrees with our own argument expansion. The final complete focused rerun including Ajv fixture-schema validation passed6tests,1preexisting skip,0failures,49assertions. The failed native capture was mcp-30768.json; raw generated capture remains under ticket generated until parent cleanup.

The root MCP package explicitly mounts workspace quick tests, while the workspace-host package mounts the long tests. The focused canonical-pair native target therefore remains semio-framework-os-mcp --lib. Its current-tree retry runs through owned cargo ingress with a60-minute client timeout and fixed original response-file forwarding; terminal native result is pending.

After public result and command additions, the real authored readRustSourceClosureV1 reader measured the root MCP lib-test source with native Windows x86_64/test cfg:22files,256677bytes,3657active lines. The unchanged boundary is32files/262144bytes/4000lines, leaving5467bytes and343lines. The shared neutral command corpus is included. No source-budget expansion or additional cleanup was needed.

## Explicit Pure Invocation Context

The opaque AppChannel PureCommand.command payload now requires schema-owned PureCommandRequest with PureInvocation action/command variant and optional actual ViewModel. Raw addressed invocation fallback was removed. Its JSON Schema lives in kernel/schema/pure-command and references the existing manifest/view-context schema, which requires locale and terminology explicitly. Rust types and TypeScript projection metadata were added, together with explicit framework facade reexports. The payload variants deliberately use manifest-address ActionInvocation/CommandInvocation: kernel identity has different execution-envelope types with the same short names, so those must not be substituted. Kernel declares the existing firstparty manifest-address dependency.

PureCommandRequest.bind_context is the production admission used by the genuine instance preview dispatcher. Explicit context wins; otherwise only the authoritative retained caller surface is used. Action mode and window instance come from that actual context, with specific window-kind ownership checked against its roster or active window kind. Mode commands require their actual active mode. Missing context or inactive mode returns plugin.unavailable; foreign window kind returns input.invalid. No English locale, manifest default mode or synthetic window instance is invented.

The language-neutral corpus exercises explicit-context precedence, actual retained window resolution, missing caller context, foreign window kind and inactive command mode. Strict Ajv validates the request schema, and an independent SQLite relational oracle resolves the expected context/ownership outcomes. The two Bun tests pass13assertions. The corresponding primary Rust JSON/Pack/serde and production bind_context laws are mounted in the actual root MCP quick suite; their native execution remains pending.

An underlying kernel/instance-command/runtime-exchange cargo check stopped before compiler after16seconds on an owned separated capture policy admission, not Rust source. Its complete generated capture was mcp-20572.json. Current workspace-selection and compiler-configuration capture bounds statically fit the16MiB/65536line guard, so a structured nonsecret cause was added to that existing refusal to identify the actual offending call, preserving every condition and limit. A fresh owned check is running.

Root source boundary was exceeded after combined public target/context changes. The target owner compacted obsolete root docs; this native owner additionally removed1462bytes of obsolete history from plugin capability routing test docs, preserving all tests, fixtures and source custody. No limits or fixture inclusion were bypassed. The reusable ticket native-source-measure.ts invokes the actual authored source-closure reader for future measurements.

## Current Native Compiler Diagnostics

The underlying check reached actual compiler and completed with four E0277 errors: addressed manifest ActionInvocation and CommandInvocation intentionally do not implement external serde runtime traits. The new pure envelope had unnecessarily derived those traits. Production envelope types now use only firstparty ToValue/FromValue, with deny_unknown_fields on the envelope. The independent serde admission DTOs are confined to tests, and the address oracle compares independent JSON values with actual owned value projection and Pack roundtrip. No serde implementation or external runtime dependency was added to addressed manifest types. The complete failed capture was mcp-31196.json under generated and will be removed with ticket generated outputs.

The full current-tree root MCP lib suite is running through owned cargo with native-root-test.json and a finite60-minute client timeout. It includes canonical-pair validation, real context/address binding, public targeting and folder canonical-event CAS/undo/redo/schema-owner laws. No root test pass is claimed before its actual result. The final source measurement after necessary compiler fixes and concise bootstrap docs was24files260965bytes3599active lines, leaving1179bytes/401lines within unchanged262144/32/4000 limits.

The native context foreign-window fault now maps input.invalid to public INPUT_INVALID instead of falling through to INTERNAL; the existing context law asserts that exact boundary. Additional static review corrected missing PluginArtifactChannel/private Fault imports in the staged quick tests. All tests/fixtures remained included in authored source custody.

## Inference Request Data and Native Funding (2026-11-05)

The public client supplies exact finite requested work, retained-policy, duration, cancellation, revision and documentGeneration data. Private RetainedTurnInput, operation identity, source epoch, clock binding and deadline were removed together from parser, committed fixtures, schemas and projections; raw turn input is refused. Native monotonic time and a freshly issued operation driver supply the actual Source authority. Document generation remains model freshness data and is distinct from the new operation generation.

Both canonical CLI transports require an explicit --inference-operation-policy JSON path. The strict native parser accepts only scope per-operation plus the separately declared cleanup reserve; unknown policy/cleanup/retained fields, missing axes and nonpositive fuel/time are refused. Direct artifact-only workspaces retain an optional configuration slot; inference without it returns a typed missing-plugin admission refusal with a configuration/live-shell remedy. The generic MCP port carries only firstparty cleanup data, projected to the host reserve at the native boundary. No default reserve or client-created clock authority exists.

Inference actor startup now uses the existing real begin/advance/take original actor APIs with actual synchronous driver admission, execution receipts and retained context. Startup and transfer keep actual opening/instance owners outside the callback result so settlement failure cannot drop them. Each inference uses a separate call driver after component activation, propagates that funding through the genuine router and awaits funded close. A failed close retains the exact source plus body and cleanup failures in bounded channel refusal custody; it never reports cleanup success. Final error-propagating server shutdown traversal remains coordinated with the root owner.

Executed current firstparty/third-party tests: 5 Bun tests pass, 52 assertions, covering DATA input with Ajv plus independent SQLite exact-integer bounds and explicit finite host/cleanup policies. Native Rust policy-decoder, command/context and durable-folder laws are authored but no native pass is claimed. Full root suite session70721 / immutable PID23924 is genuinely compiling dependency crates after Source admission (last observed3112 seconds overall); separate private folder-law session74426 remains in resource-directory admission. Neither run has reached a terminal result at this entry.

A schema census found the old raw-turn scope-export roster and repaired it to policy-only. The JSON mirror also lacked the existing HubInferenceApprovalRequestV1 registry export before this change; the root owns restoring that genuine export and byte-exact emitter verification.

### Native Run Boundary and Poll Audit

The full root --lib client session70721 ended at its explicit60minute ticket-client deadline, exit1, while dependency compilation was progressing. Ledger PID23924 reached real Cargo child admission at1,987,351ms; it never emitted a child terminal before client cleanup. A following owned doctor census proved PID23924, Cargo23328 and its wrapper/scheduler children had drained. This is neither a Rust failure diagnosis nor a native test pass. A later distinct wake operation-law session3600 uses a120minute client allowance; actual Original policy budgets are unchanged. Store-law session74426 remains live and untouched.

The phased Owned instance cannot use legacy execute_actor_turn: original_core is retained and actor is absent, and the legacy operation port explicitly refuses this combination. The root now owns a genuine generic original byte-unit call machine; this agent owns the poll-specific framing. The added controlled poll encoder reuses OwnedPollInput and the existing firstparty controlled JSON encoder. One borrowed ordinary event remains on the stack; command and cold-document pages preserve their existing validation. The neutral fixture plus independent Ajv/SQLite oracle executes1pass6assertions; matching Rust producer laws are authored pending compiler admission.

A second genuine guest gap was located: poll_owned retains TurnResult in ORIGINAL_OWNED_OUTPUT and serializes it, but no current producer takes or retires that static output. outputs_held consequently refuses the next poll and actor close. Freeing serialized output bytes alone does not release that distinct original TurnResult. The root was sent the exact owning paths and asked to coordinate an actual bounded output retirement/ack producer rather than claim successful empty transport.

### Resident Turn Output and Current Native Verification

The canonical owned ABI now includes CloseTurnOutput (30 exports total), taking the existing OriginalJobBudget JSON and returning the resident fixed SAC2 close receipt. The actual guest root deallocator recognizes fixed receipt loans and never frees their static storage. The close producer operates on a shared OriginalTurnOutput owner, available to native guest-contract tests as well as wasm exports. It binds the actual caller identity/clock/epoch, retains denied and foreign close attempts, and clears ownership only after the actual typed descendants and final owner metadata close.

TurnResult and its status/presence/ingress descendants now have firstparty controlled retirement. UiTurnPatches retains each existing reservation and quotes the actual next UiPendingPatch physical release before advancing its existing typed cursor. No resident owner is replaced by an empty success. The neutral corpus includes a real nonempty UI patch page, a Unicode effect, denial, foreign caller and funded close. Independent Ajv/SQLite oracle:1pass6assertions after the foreign row addition (the prior3row run was1pass5assertions). The Rust law observes per-step actual allocations/releases against Source receipts, denial with no physical work, and cancellation followed by paid original retirement; actual native execution remains pending.

Actual store-law session74426 also ended at the old60minute client deadline before compilation, during Source admission unit170/235 (UI contract,1106 files and13.61MB accumulated across the closure). The original owned lineage drained; a reused PID19908 belonged to another parent and was not acted on. New actual isolated verification sessions use120minute client allowances:3600 wake operation laws,12882 original byte-unit laws,67933 resident-output laws. All are live and no Rust/compiler success is claimed.

The canonical shared channel test fixture used obsolete scalar arities. It now matches the real20/19/5/22 argument counts and offers a real poll-byte echo helper for the root generic byte-unit laws. This repairs fixture admission without relaxing the real ABI roster.

## Controlled UI native receiving checkpoint

The native common operation policy parser now uses HostOperationPolicy and the required --operation-policy path, with closed cleanup and transit records; its host cap projection passes both explicit reserves. Caller limits remain guest logical DATA. Host seals a genuine guest subloan after startup and must rewrite the actual request wire from that loan, without native parallel cap arithmetic.

Transparent wrapper NativeReceive derive, direct bounded UTF8 UiText receiving, PagedList-backed UiFixedList receiving, and real bounded UiFixedMap/UiFixedBytes construction/retirement adapters are authored. Fixed map input keys are sorted through the existing owner insertion API with a bounded worst-case move quote. Byte backing keeps its physical allocation through vector-to-box transfer by initializing spare capacity before the exact no-resize handoff. Generic sequence receiving exposes allocation-free push/finalization quote hooks; existing receivers retain their original quotes. These native additions are not yet compiler-verified.

The text receiver corpus reuses the neutral UI foundation UTF8 vectors and a finite five-axis/work stock with cancellation prefixes. Independent Ajv + SQLite UTF8 byte/JSON oracle ran successfully: 1 pass, 19 assertions. Native law checks each denied/prefix/cancelled close, source identity, exact observed heap receipts and finite currency; it remains pending a real native compiler result. The full typed TurnResult/UI arena receiving cascade and genuine poll adapter are still incomplete; no headless discard or raw serde fallback has been introduced.

## Actual wake compile diagnostic recovery

The wake operation native request (session3600, immutable48104) reached a real cargo child at4653913ms, returned child/error at5209623ms, and completed consumption at5249438ms. Outer MCP response remained pending. Read-only owned compiler-unit fingerprints recovered one real blocking compiler diagnostic: E0599 in job/context/encoding.rs:14, NativeEncodeControl::new_scoped_source absent from compiled encoding API. Fingerprint owner: fixed/m/u/l/p5c360c/b/prototype/build/semio-framework-job-context/a65b8313690ec0e2/fingerprint/output-lib-semio_framework_job_context. No other error diagnostics with source spans appeared among that publication roster after1793839300000ms. Root source-encoding owner has been notified. This is a dependency compiler failure, not a wake law result.

## Lifetime arena and recursive UI receiving ports

UiArenaOpening now admits a stable header and four actual array backings through allocation/one-row initialization/sealing phases. Opaque UiArenaBacking retains all actual allocations; partial opening and final backing each close explicitly under their real grants. UiArenaRegistration uniquely owns that backing and publishes only a mutex-protected borrow pointer; close refuses active borrows/descendants, freezes new borrows, audits one row or handback word per funded step, detaches, then releases exact physical backings/header. Explicit paid recover_borrow_step retains the same owner and epoch. No Job types are required by these UI boundary APIs. The old cold global has not yet been cut over: MCP process-lifetime Source boot and guest actual admit-context boot must be coherent first. It is not a completed runtime integration.

NativeReceive and true controlled RetireOwned are authored for all handwritten base UI owners: UiValue/UiList/UiMap/UiText/UiFixedList/UiFixedMap/UiFixedBytes. Nonempty recursive collections require an already initialized registered lifetime arena; scalar and empty values need no arena. The canonical sequence trait now takes a mutable pending donor in push_step, allowing mutex contention to yield with exactly that donor retained. Vec, PagedList, PagedBytes and both UI collection families were updated together; no synchronous unsupported push stub or compatibility method remains.

Independent neutral oracles currently pass: bounded text1/19assertions; fixed collections1/13; arena ownership1/7; recursive values1/18. Native finite-wallet/heap/denial/cancellation laws are authored, including scalar/empty recursive values without global arena birth, and are pending actual compiler/test completion. Concrete UI struct/enum receiving projections, canonical global registry boot cutover, full TurnResult typed receiver and genuine poll execution remain incomplete.

Source bounds: NativeOwned cfgtest32files261246bytes1197activeLines after5940bytes of prose contracts moved to its owning README with source link (all executable operations, tests and assertions preserved). UIvalues cfgtest25files230508bytes3467activeLines before the final independent numeric oracle adjustment. Bounds remain unchanged.

Core native session91770 ultimately returned actual innercargo101 after a55.999second compiler child, outerMCP duration5529054ms; response generated/mcp-13688.json. Four dependency compiler errors were in nativeencoding: scoped allocation closure Send mismatch and three observer callback/work-port lifetime errors. Root encoding owner received exact rendered diagnostics; Core own laws did not run. NewUI session98225 uses current source, the same bounded repository policy, deadline tracing only, and a120min client timeout.

## Source-native UI arena lifetime — 2026-11-05

Added UiArenaLifetime alongside the canonical arena owner, using the existing framework Job dependency. Its unopened input allocates nothing and contains no UiArenaOpening. admit_source delegates to the actual Original native recipient before advance_on_source may construct the opening or allocate any backing. Actual partial opening, taken backing and unique registration stay inside that recipient. Registration and physical cleanup advance through OriginalNativeRetirement under the same genuine Source; active descendants still block detachment. No OS host dependency or per-call global arena fabrication was added.

The neutral readiness corpus is independently evaluated by strict Ajv and SQLite: 1 test, 6 assertions passed. The authored native law checks zero-capacity admission refusal without allocation, foreign Source refusal, prefix cancellation, exact observed heap receipts, publication and complete physical cleanup. It has not yet compiled or passed. UI values current test closure is 28 files, 239579 bytes, 3529 active lines, within unchanged limits. The current actual native owner test (session 98225, wrapper 20140) remains live around 48 minutes, now cargo planning capture; no compiler result. The old LazyLock global is deliberately not cut over until actual host and guest Source bootstrap callers are coherent and verified.

The common MCP operation DTO now requires infrastructure separately from transit and cleanup. The native inference owner passes all three explicit policy projections to operation_caps. Root owns administrative parser/config validation.

Kernel receiver cascade now includes four retained-turn records, canonical decimal lifecycle and patch receipts, cold-pair frontier/cursor/applied/status, command page cursor/status, JobCheckpoint and TurnStatus. JobCheckpoint also receives genuine owned retirement needed by the existing TurnStatus retirement derive. Full TurnResult and custom UI/effect receipt coverage remain incomplete; no readiness claim follows from these ports.

### UI native owner invocation terminal and correction

Session 98225 completed after 51m0.799s with ok:false (generated/mcp-44100.json). Actual Cargo planning refused testing UI contract text because it is not a root workspace member; no Rust compiler or law result exists. Root Cargo.toml does not list text, whereas framework/Cargo.toml does and the package explicitly points to that framework workspace. The corrected owned request therefore supplies the actual text manifest and selects text, values and presentation within that owning workspace. This is caller workspace selection, with no infrastructure/boundary change. Before retry, doctor process facts returned no old PID 20140/44100 or direct children.

### Complete UI special receiving owners

Mounted the actual UiPatchOps NativeReceive and owned retirement after the parent completed UiPatchOp concrete receiving. Directory allocation, each separate operation page and donor placement use existing storage admission APIs. Physical allocation refusals preserve actual backing and exact allocation bytes. Owned retirement preserves all actual operation pages and an in-progress typed retirement cursor, including its detached UiValueRetirement; no reset or fabricated empty cursor occurs. UiPublicationRevision uses a checked borrowed decimal scalar port with bounded 20-character validation, preserving 0 and the full u64 maximum.

Kernel UiTurnPatches now receives the full fixed page through the standard original sequence machine. It reserves one actual static retirement-arena candidate per funded step, yields without consuming its donor on contention, and includes an unpublished reserved slot in its real owner prefix so cancellation retires its key. A dedicated native law covers denied grants, unpublished reservation, controlled closure and observed heap receipts; native execution remains pending. Neutral strict Ajv plus SQLite reservation corpus passed 1 test with 11 assertions. Kernel UI test source closure is 8 files, 91786 bytes, 1629 lines; UI values is 28 files, 239877 bytes, 3530 lines, both within unchanged limits.

Corrected actual owner-manifest native UI call is live session 98721, wrapper 48940, and selects text, values and presentation with a finite 120-minute client timeout and unchanged Original process policy. No result has been claimed.

### Canonical wide handle and channel 25 checkpoint

ArtifactHandle and WindowHandle now share canonical decimal-string serde, value admission and bounded native scalar receiving through the complete u128 range. Numeric input, leading zeroes, signs, whitespace and overflow refuse. The exact-range JSONSchema and independent SQLite string/integer-domain oracle pass alongside the production TypeScript parser: 1 test, 45 assertions. A Rust serde/value/native agreement law is authored, uncompiled.

Actual WIT effects now carry string window/document identities, removing the previous as-u64 truncation from guest bridge, both host conversion families and direct artifact read/write imports. Renderer close-window decoding validates and retains the canonical string; its public Effect field is string. One shared channel 24 to 25 landing covers these WIT fields and the guest owner single ActorContextBudget Admit/Close change. Canonical declared consumer generation has run; derived fixture owner work recomputed descriptor Pack fields through the actual Pack scalar codec, exact host fixture descriptor bytes, bootstrap profile generation, frozen GIS digest and linked proposal/job references, directory catalog encodings and descriptor/catalog lease hashes. Hostile raw duplicate/trailing layouts and deliberate incompatible protocol vectors remain intact.

Census identified five genuinely unregistered existing OS consumer owners, now declared with exact occurrences and preserved hostile version22 for descriptor verification. The publication future-incompatible vector moved25 to26 so live25 cannot become an indistinguishable hostile vector. Final fresh declared census/generation is running (session18799), no zero-findings claim yet. Source fixture regeneration exposed missing internal @semio-tech/framework-hash Node resolution and an unavailable generated dev-catalog import; fixture-resolution.json maps the exact declared hash package source, and the unrelated generated module is avoided by evaluating its existing closed pure codec-projector source, not replacing its implementation. No migration script is retained.

Whole TurnResult receiving and Effect-related native DTO ports are now authored, preserving the full output graph rather than dropping UI/effects. Their native compilation and actual KernelPoll integration remain pending. Corrected UI native test98721 remains active with no current compiler result.


## Unique Output Receiving Source Boundary

The generic first-party `semio-framework-pack-json-output-receiving` owner now retains the grammar cursor, original DSL source admission, NativeReceivingOriginal typed receiver, unique capture and paid parser cleanup without producing an unmetered host JSON copy. `OriginalJsonOutputSource<T>` couples this receiver to its actual `OriginalNativeDecodeSource`; receive and take return the complete raw producer outcome plus receiving refusal. The containing Source-native app state must retain both failures and the actual guest UnitFrontier/resident-output owner until genuine handback. `close_owned_step` provides the same Source-funded controlled retirement for a captured typed output before closing the decoder. No generic receiver method acknowledges guest memory or fabricates CloseActorOutput success. Native laws remain uncompiled.

The prior PackJson source closure independently exceeded the unchanged262144-byte ceiling:25files275713bytes3414activeLines. Relocating only complete-line older comments and doc narratives into its owning README reduced the exact closure to25files250812bytes3139activeLines; executable statements, fixtures and assertions are unchanged. The separate unique owner was3files21541bytes72activeLines before the Source wrapper addition.

Focused native UI run98721 terminated before the compiler after1h0m38.926s. Actual retained response is generated/mcp-52916.json. Its admitted workspace-manifest extent changed from127459 to127578bytes during capture, correctly refusing concurrent-source admission; this is no native test pass.


The Source wrapper has a native law against every neutral JSON row and cancellation prefix, including actual Serde output comparison, physical heap receipt conservation and captured-output retirement through the same live decoder before Source return. Strict Ajv plus SQLite neutral output suite reran:1pass11assertions. New actual owner-manifest native call77566 (wrapper57212) selects text, values, presentation, complete UI contract and unique JSON output owner with --lib and120-minute client delivery allowance. Prior wrapper52916/48940 process lineages were absent before submission. Native outcome pending; no success inferred from Source admission or client allowance.


PackJson source-only policy and physical receipt laws reran after documentation relocation:8tests69assertions passed. The unique output Source wrapper closure parses through the real owned source selector at4files28407bytes104activeLines. Source ready-transfer demands now include the actual retained DSL source transfer header before reducing its child grant. No limit was raised. Actual native77566 remains in bounded preparation/resource retirement; results still pending.


## Actor Input Wire Disposition

The actual WIT reactor ingress record now has a shared first-party Rust representation in actor retained-context/input: OriginalActorInputDisposition and OriginalActorInputAdmissionReply. Readiness remains independent of Borrowed, Retained or Returned ownership; malformed returned-wire combinations remain representable raw evidence and validation borrows rather than destroys them. Returned requires the actual vector in this WIT owned-wire record. The distinct Owned Core136-byte InputAdmission borrow witness never uses or validates this vector disposition: its actual caller already retains the wire. Strict Ajv and SQLite agree on7neutral combinations (1pass14assertions); Rust law repeats both ready states and exact Serde roundtrips without dropping a returned wire. Native actor law pending. Actual context closure11files69207bytes486activeLines.

# Sequence Retained Action Cohort

Date: 2026-08-27  
Phase: `EVERY-TOOL-INTERACTIVE-JOB-MIGRATION`  
Scope: `✏️s/🔌️plugins/🎬️sequence` only, plus this ticket evidence  
Compiler/Nx/rustfmt lease: unavailable while stdio work is incomplete; this pass is source-only

## Result

The complete live Sequence action surface is now retained: 17 routes total, 17 `Migrated`, 0 `BatchOnlyPendingRewrite`. The official source verifier admits all seventeen routes, reports no Sequence route in `remainingCommands`, `scanThenMonolithRows`, or failures, and reports no remaining Sequence process-global payload-store candidate.

Retained routes, exact Config lane:

1. `setViewport`
2. `setOrientation`
3. `stop`
4. `setLocale`
5. `run`

Retained routes, exact Artifact lane:

1. `addStep`
2. `addStepToSlot`
3. `addStepDropped`
4. `removeStep`
5. `deleteSelection`
6. `moveStep`
7. `connectSteps`
8. `disconnectSteps`
9. `setStepParams`
10. `setStepCollapsed`
11. `reorganize`
12. `nodeGraphEdit`

No compatibility alias, identity lookup, alternate publication path, or pre-job payload cache was added.

## Exact Ownership Repairs

### Artifact scene owner

The former process-global `SEQUENCE_SCRATCH: RefCell<HashMap<String, SequenceWorkingScene>>` was removed. `SequenceWorkingScene` now lives on its exact `ArtifactChild` through `with_local_owner(Arc<SequenceWorkingScene>)`; reads use `local_owner::<SequenceWorkingScene>()`. Schema constructors, snapshots, mutation fixtures, and the external mutation test attach or preserve that same exact child-owned state.

This removes document-id identity lookup, cross-instance collision, stale cache retention, and process-global payload ownership.

The pure content bridge helpers (`sequence_content_snapshot_from_working`, handle minting, exact-owner reads/writes, `diff_replace_content`, and snapshot fixture conversion) were also made synchronous. Their callers already consume immediate values; retaining artificial futures would make the new synchronous Store preparation authority impossible and would preserve pre-job async staging for pure in-memory work.

### WASM bridge owner

The former thread-local `BRIDGE` and `RETAINED` payload stores were replaced by `SequenceBridgeOwner`. `sequence_bridge_create` returns an exact heap owner; every send, poll, close, and terminal query requires its pointer; `sequence_bridge_destroy` refuses non-terminal retirement. The JavaScript host creates one owner per host instance and destroys it only after terminal emptiness. Browser, protocol-oracle, and host mocks were updated to the same ABI.

## Retained State Machine

`SequenceRetainedConfigWork` has two bounded semantic units:

1. localized preparation progress (`en` and `de`),
2. exact typed command dispatch and Config-only publication.

It owns an operation-scoped workspace digest, cursor, replay target, completion flag, and close state. Its 24-byte `SRC1` checkpoint validates magic, reserved bytes, cursor bound, and exact operation identity before replay. Cancellation is provided by the retained command wrapper; replay emits a distinct localized replay stage; abandon-close releases replay state through bounded close steps and exposes terminal emptiness.

`SequenceConfigStorePreparationFactory` supplies the exact Config lane preparation authority. It validates lane, operation, generation, base revision, actor/description bounds, and one-item work. Its 65,536-byte cap admits the bounded `run` result while retaining the scalar routes' 4,096-byte wire proofs. It prepares an exact-base inverse snapshot and returns base/prepared/authority ownership incrementally during close.

`SequenceArtifactStorePreparationFactory` supplies the exact Artifact lane preparation authority for the ten retained edit routes. It admits one typed semantic mutation at a time, rejects the unregistered `DuplicateStep` variant, requires the exact operation/generation/base revision, caps steps and edges at 256 each, and bounds pre/post serialized scene ownership at 65,536 bytes. It prepares typed inverses for create/delete/move/params/collapse/connect/disconnect and returns prepared, mutation, base, and authority owners incrementally during abandon-close.

`SequenceRetainedArtifactWork` bypasses the former whole-host rebuild/diff reducers. It emits the existing typed mutation vocabulary directly after bounded exact-child scene inspection. Add routes mint a deterministic next id, dropped-add preserves expanded-control default-slot behavior, remove/delete-selection preserve direct control-child removal, connect preserves slot/cycle/fan-out/incoming-rewire behavior, and invalid/no-op commands publish an empty exact-lane emit. The scene and selection caps make each state-machine poll fixed; the declared maximum is 2,000 microseconds, below the 8,000-microsecond interactive ceiling.

### Persistent composite routes

`SequencePersistentWork` owns a route-specific workspace, operation/generation-derived identity, semantic progress cursor, replay target, completion state, and bounded close state. Its 24-byte `SRP1` checkpoint contains only the exact owner identity and semantic progress. Restore reconstructs fresh route-owned scratch and replays the actual deterministic state transitions to the checkpoint instead of restoring an alias or cached payload. Completion validates the exact Artifact or Config publication lane and the 65,536-byte output envelope immediately before exposure. Close requires both a nonzero item and byte grant and retires one owned item per call.

`reorganize` is a persistent layered-layout reducer. It initializes one node depth per poll, relaxes one edge per poll with explicit pass/edge cursors, and plans one typed `MoveStep` mutation per poll. At the 256-node/256-edge envelope the largest honest progress trace is 66,048 units, below the declared 66,049-unit cap. Orientation remains customizable through `SequenceConfig`; every preview has English and German text.

`nodeGraphEdit` separates bounded decode, one sub-operation per poll, one fixture step or edge transfer per poll, one selected/nested step traversal or removal per poll, and one base/target step or edge diff per poll. Kind or slot replacement uses typed delete/create operations and records recreated identities so cascaded edges are republished exactly. Composite output is byte-counted before Artifact publication. Operation, fixture, selection frontier, base, target, deleted/recreated identities, and mutations are all operation-owned scratch and retire item by item.

`run` is a persistent nested interpreter rather than a whole-path dispatch. Every frame incrementally scans one scene step or edge, resolves one head, walks one graph edge, or emits one deterministic remainder item per poll. Execution advances one step or one frame transition per poll. Ordinary operators run as single-step imperative-engine oracle calls; `if`, `repeat`, and `while` use explicit nested frames, retain graph order, publish localized progress, preserve repeat indices and error-local frame halt, and enforce depth 64 plus repeat/while/effect envelopes of 256. The final `RunResult` is serialized only after the bounded traversal and publishes exactly one Config mutation. The persistent routes declare 7,500 microseconds per poll, below the strict 8,000-microsecond ceiling.

## Language-Neutral and Hostile Laws

Added Draft 2020-12 schema and data fixture:

- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧬️schema/🧵️retained-actions.json`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️fixtures/🧵️retained-actions.json`

The fixture fixes owner, controller, document schema, all 17 route dispositions, exact publication lanes, retained bounds, checkpoint/replay/close requirements, English/German localization, keyboard reachability, cancellability, progress announcements, orientation/locale/viewport customization, and three persistent algorithm scenarios.

The Ajv third-party oracle rejects seven hostile mutations: unknown identity cache, invalid Migrated classification, false BatchOnly retained contract, raw-byte plus-one, semantic-unit plus-one, monolingual configuration, and an 8,001-microsecond poll. It source-checks all 17 classifications, all 17 exact publication lanes and bounded proofs, both Store authorities, progress/replay/checkpoint/restore/close witnesses, and the reorganize, composite-graph, nested-order, exact-lane, and byte-aware retirement cursors. Dagre 0.8.5 independently reproduces the language-neutral chain layout, graphlib independently reproduces the run order, and `fast-deep-equal` checks all three algorithm outputs including the typed node-graph replacement mutation sequence.

## Source-Only Validation

1. `bun .../🧵️retained-actions.test.js` — PASS: Ajv 2020 + Dagre 0.8.5 + graphlib, `routes=17`, `migrated=17`, `pending=0`, `hostileLaws=7`, `persistentScenarios=3`, `maximumStepMicros=7500`, locales `en,de`.
2. `bun .../🧪️sequence-host.test.js` — PASS: bounded pages/bytes/events/in-flight, exact retained retry/acknowledgement, cancellation, playback close, terminal empty.
3. `bun .../🧪️sequence-protocol-oracle.test.js` — PASS: Ajv protocol and semantic output equal.
4. `bun .../🧪️sequence-browser-consumer.test.js` — PASS: public entry, exact instantiation/session ownership, terminal empty.
5. `bun ./📜️script.ts verify interactivity tool-jobs --self-test` — PASS: `self-tests=486 clean`.
6. Official R3 JSON: `📊️sol-sequence-retained-tool-jobs-r3-2026-08-27.json`. Repository-wide command exits nonzero on unrelated fleet debt. Concurrent census: 773 rows, 253 bounded, 154 BatchOnly, 528 remaining, 42 factories, 222 registrations. Sequence slice: 17 accepted retained routes, 3 explicit factories, 0 remaining, 0 scan-then-monolith, 0 global payload candidates, 0 failures.
7. Sequence source scan for `thread_local!`, mutable static locks, `HashMap<String, SequenceWorkingScene>`, `SEQUENCE_SCRATCH`, old cache helper, and old child-and-cache helper — no matches.

## Pending Validation and Blocker

Cargo, Nx, and rustfmt were intentionally not run while stdio work remains incomplete and the exclusive validation lease is unavailable. Rust compile, Rust unit coverage, rustfmt, and the Sequence Nx target remain pending until the root grants that lease. The source verifier and JavaScript/Ajv runtime checks above are clean; this report does not claim Rust compilation.

The ticket is not closed here because repo MCP ticket tools are unavailable. There are no pending Sequence BatchOnly routes.

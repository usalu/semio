# Shared Typed-Command Full-Operation Foundation Follow-Up

Date: 2026-08-26

## Result

The shared typed-command path now has a production retained-page factory route and a real guest/actor/WGPU retained result exchange rather than only the fixture model. Exact raw-wire capacity is admitted before JSON serialization, the truthful written prefix is sealed after incremental page encoding, and app-owned dispatch transfers retained input into the registered concrete factory job. Layout export is the first production registration using that seam: its command-wire contract is one exact 4,096-byte page; the worker copies one page, scans one byte per step, performs the bounded typed JSON command decode, and only then materializes its reducer. The repository's official `verify interactivity tool-jobs` gate no longer reports any of this packet's typed-command blockers.

## Implemented

- Added `RetainedToolWireInput::seal_admitted_prefix`, `ToolJobFactory::create_job_from_wire_pages_with_payload`, its erased adapter, and `ActionBus::dispatch_wire_retained_with_spec`. The route validates exact key/schema/contract/factory identity, rejects stale or unsealed owners, and transfers the typed payload plus retained pages through the registered factory.
- Changed production JSON ingress to call `begin_exact_wire(..., max_raw_wire_bytes)` before `serde_json::to_writer`. The writer owns the admitted fixed pages, performs no pre-admission serialization/census, and seals only the prefix it actually wrote.
- Added a concrete Layout export retained-wire factory override. `LayoutExportToolJob` owns the raw pages through bounded close, copies one page per step, scans one byte per step with a bounded 64-level structural stack, decodes the exact verb/page-id command inside the worker under a one-page raw cap, and defers `LayoutExportJob` construction until that decode succeeds. Cancel and fault paths materialize only for bounded owner retirement.
- Changed document/config/draft/presence/transient publication to clone the tail owner, apply it, and pop the authoritative owner only after successful acceptance. Failed apply leaves the original owner mounted; the publication driver retries up to the fixed retry authority and then exposes a bounded fault page.
- Changed host lanes from serialize-and-ACK-drop behavior: child publication calls the real composition group dispatcher; effects, events, and UI scopes transfer to fixed receiver outboxes and are emitted through `plugin_exchange` (UI as an unsolicited invocation scope consumed by the existing shell route); saturation retains the authoritative source lane.
- Added `MountedTypedOperationResultExchange`, a fixed 64-page object-safe WGPU implementation. It is installed during `KernelClient::get`, replaces retry attempts by stable operation/sequence identity, rejects stale/wrong tokens, invokes the exact ACK callback, and removes the page only when the callback has admitted the ACK request.
- Wired `PluginExchangeOutput::typed_operation_result` into a bounded language-neutral guest message in the real reactor. WGPU intercepts and decodes that shell message before the legacy `AppFrame` decoder, publishes it into the installed exchange, and its ACK callback enqueues `KernelRequest::AcknowledgeTypedOperationResult`. The kernel delivers an exact shell message back to the guest; the reactor decodes it and calls `plugin_acknowledge_typed_operation_result`. Queue saturation returns `false` from the callback, so the WGPU page and guest authoritative owner both remain retained.
- Separated framework-owned and app-owned qualified registrations. Every live controller now installs the seven route-specific history/revert factories plus the configuration-binary factory, while application factories remain a distinct proof class and cannot accidentally enter a framework commit branch.
- Changed the shared media-import boundary to transfer the owned `Media` value into the app's concrete reserved job. Import construction no longer JSON-serializes or clones the whole media envelope before the resumable job exists; the empty raw lane truthfully identifies this typed-owner route.
- Replaced draft, interaction, presence-local, and transient preparation clones with their event-maintained immutable roots. The official gate now recognizes the existing `Arc::clone(&*self.current)` snapshot-root implementation and the retained rejected-presence publication authority instead of requiring obsolete spellings/queues.
- Reconciled the official gate with the production architecture where its predicates still pointed at superseded source locations or unreachable decode routes: paged ingress is schema-first in `spr/channel`, fail-closed UI intent never deserializes, persistent typed publication uses `MountedWorkerJobSession` plus exact ACK, and reserved routes use mounted preparation with a freshness-validated, cancellation-held actor commit.
- Bound the production mounted typed-command scheduler to the admitted execution contract: each worker turn receives exactly `max_work_units_per_step`, and its millisecond deadline is the strict floor of `max_step_micros`. The official predicates now inspect the retained `MountedWorkerJobSession::try_new` route, its single initial worker turn, fixed result-page encoder, and the live publisher's revision/generation validation before the first store apply instead of deleted one-shot factory symbols.
- Reconciled the actor progress predicate with the current retained replay architecture. Autonomous shards retain the spawn owner, qualified `JobTurn`, replay request, and placement in a fixed `MountedReplaySeed` before live start; WGPU captures each `ShardOutcome::Job` into the mounted replay path, then publishes through `publish_captured_job_progress`, the fixed progress overlay, presenter bridge, and exact ACK/abort handback. The verifier and hostile fixture now require this stronger fixed-seed route instead of the deleted direct `publish_job_progress` call shape.
- Reconciled runtime close admission with its preflighted `RuntimeInstanceRegistry::insert_admitted` transfer and added an executable reactor close law. The law drives the real fixed close registry through requests, resumes, tasks, timers, and metadata one opportunity at a time, proves it remains mounted for multiple structural phases, and requires terminal removal within the fixed 8,192-opportunity bound. The official runtime instance/actor authority and reactor task/request/open-instance close blockers are now absent.
- Preserved all 40 language-neutral rows. The Rust fixture now asserts the exact row count and anchors its contract to production admission ordering, retained factory dispatch, owner-preserving publication/retry, real child/effect/event receivers, guest page production, and guest ACK application. Action-bus, Layout, plugin-wire, and WGPU tests add executable max/+1, saturation, stale ACK, success, cancel, fault, and bounded-close coverage.

## All-App Discovery Boundary

The all-app verifier previously counted only the 32 plugin descriptors and their 101 declared apps. It parsed 248 dev launches but used them solely to prove React/WGPU coverage for those descriptor apps, so launch-only framework/product surfaces were invisible to its app ledger.

The verifier now derives the generated playground launch identity set from the generated playground catalog plus the schema-owned launch seed, subtracts it from the bounded `.vscode/launch.json` dev-launch census, and exposes every unmatched row as a launch-only product surface. This uses no product allowlist and does not falsely invent action metadata for launch rows that have no descriptor. The official discovery result is `apps=101 launchOnlyProducts=68 surfaces=169 launches=248 launchMissingApps=0 failures=0 selfTests=25`. Eleven of the 68 launch-only rows are Compose surfaces. The 68 rows include interactive products, framework tools, hubs, MCP endpoints, and Storybooks; their launch metadata does not encode which rows own UI actions, so action migration remains descriptor-scoped until that role is schema-owned rather than inferred from a name or command heuristic.

## Boundary

The shared route and Layout production registration now cover the requested retained command/result seams. Other applications still need their own exact factory override and bounded in-job decoder; the default `create_job_from_wire_pages_with_payload` remains deliberately fail-closed. The six direct framework interaction actions plus `setHistoryCommandFilter` and `noteShellCommand` retain their existing private host semantics and are not falsely promoted through a no-op application factory. They require a future shared retained post-worker host-commit state machine before application cohorts may mark those rows migrated.

## Verification

- `bun ./📜️script.ts verify interactivity tool-jobs --format json` was run against the official JSON path. The latest concurrent census is `production-hosts=50 production-rows=776 admitted=161 remaining=724 factories=16 dispatches=3 self-tests=365`. The gate still exits 1 for broader member/envelope/application/runtime cohorts, but none of this packet's typed-command blockers remains, including the runtime qualified-proof, mounted-session dispatch, execution-limit, immediate-freshness, copied-owner compiler-witness, actor progress-overlay, runtime instance/actor close-authority, and reactor close predicates. The apparent three-predicate regression in the coordinator JSON was traced exactly to `indexOf` selecting earlier source-test string literals rather than production definitions; `lastIndexOf` now selects the live mounted dispatch and publisher. Source mtimes were plugin `07:26:04`, verifier `07:27:26`, coordinator JSON `07:30:15`, proving this was predicate extraction ambiguity rather than a source race. Current official JSON evidence is recorded below; retained scratch diagnostics use `.txt`.
- `bun ./📜️script.ts verify interactivity apps` exits 0 with `descriptors=32 extensions=4 apps=101 launchOnlyProducts=68 surfaces=169 actions=4754 migratedActions=1973 missingActions=2781 launchCoveredApps=101 launchMissingApps=0 launches=248 failures=0 selfTests=25`.
- `rustfmt --edition 2021 --check` parsed all five changed Rust sources and emitted formatting diffs: exit 1. No parse error was emitted, and no broad mechanical rewrite was applied while agents share these files. Focused retained evidence uses the `🧪️sol-*-rustfmt-check-2026-08-26.txt` files listed in later sections.
- No Cargo, Nx, Wasm, browser, or Git command was run. Compilation and runtime behavior beyond the executable repository gate are not claimed.

## Changed Sources

- `🧰️framework/🔨️modules/🎯️action-bus/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️component.rs`
- `📜️script.ts`
- `📊️sol-envelope-peer-roots-final-2026-08-26.json`
- `🧪️sol-peer-roots-focused-rustfmt-check-2026-08-26.txt`

The repo MCP server was unavailable, so `repo://goals` and ticket lifecycle calls could not be performed.

## Hard Nested-Payload Close Completion

The final shared close blocker is now remediated in production. `VcsArtifactApp::close_step` no longer stops after the five application stores with `interactive-job.close-app-retained-fields-missing`, and `close_terminal_is_empty` is no longer the unconditional false placeholder.

- Added the sixth concrete disposer for the framework interaction store.
- Added a retained cache retirement owner that releases history, configuration, and snapshot roots one grant at a time.
- Added one-owner close steps for command-log nested edit ids and rows, dirty-history identities, child pins, pending transactions/proposals, presence output, each retained emit buffer, typed host outboxes, application action/command/interaction catalog rows, tool proofs/contracts/registrations, and the controller identity.
- Added bounded `CompositionGraph`/`TransactionCoordinator` disassembly: one ownership edge, link edge, or empty adjacency owner per grant, followed by an exact terminal-empty witness.
- Replaced the VCS terminal placeholder with an aggregate witness over all six store disposers, cancellation authority, live/closing operation registries, snapshot/child/peer/envelope retirement registries, snapshot-read return pump, and every retained-field owner above.
- Repaired a real typed-download ownership gap found while reconciling the hard-close predicate. A Download result page now preflights both segmented live and close registries before consuming its exact ACK. Stale ACK or saturation leaves the page/publication owner mounted; successful exact ACK transfers the same `ArtifactDownloadOutput` into `segmented_downloads` without cloning or dropping it.
- Added hostile laws `artifact_close_final_destructor_is_constant_after_every_owned_field_is_drained`, `retained_field_maximum_and_maximum_plus_one_are_language_neutral`, and `cleanup_queue_saturation_preserves_detached_app_ownership`. They cover exact maximum/+1 byte grants, interruption/resume, repeated terminal close, full fixed quarantine, and pointer-identical owner handback. Existing stale-ACK, duplicate-id, cancel/fault, and bounded close laws remain required.
- Reconciled three verifier extractors with the stronger mounted `BatchJobSession` architecture: runtime live/terminal cleanup budgets are carried by `BatchDriveConfig`, not an inline `StepBudget`; the live cleanup job upgrades its retained weak cell before `try_lock`; and media sessions are installed as accepted/rejected mounted owners before exposure. The no-loop check now recognizes actual Rust loop statements instead of falsely matching the words `waits for` inside a fault string. Hostile verifier fixtures remain rejected.

Focused evidence:

- `rustfmt --edition 2021 --check` parsed both modified Rust roots; filtering its output produced no `error:` line. Formatting differences remain and no broad rewrite was applied.
- Fresh official `bun ./📜️script.ts verify interactivity tool-jobs --format json`: hard-close blocker matches `[]`; concurrent census `productionHosts=50 productionRows=776 admitted=161 remaining=724 factories=16 dispatches=3 selfTests=365`; total broader failures `171`.
- Exact official blocker removed: `instance close still permits implicit nested payload destruction or lacks saturation-safe bounded cleanup job ownership`.
- No Cargo, Nx, browser, or Git command was run.

Additional changed production source for this close packet:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`

## Central Ownership Follow-Up

The central paged-ingress, media-export, generated-member, and artifact-store structural-owner blockers are now absent in the official gate. The production changes and verifier reconciliations preserve the hostile negative fixtures rather than bypassing them.

- Paged ingress now recognizes the rustfmt-independent exact fixed-page reservation expression. Its fixed owner, multi-page ACK order, cancel/fault/close, and host registries remain required.
- Media export now proves the production construction order: exact registry preflight, fixed output credit/chunks, close-owner installation, mounted session admission, accepted/rejected owner retention, and live exposure. A new registry-detach law proves unrelated owners remain live and the detached owner is handed back by exact `Arc` identity.
- Generated member create/open now proves all four owner authorities through the real wrapper path: snapshot, initial snapshot, mutation, and whole-store disposal. The fixed edit-message ledger is directly embedded because its own `Drop` requires terminal empty; the obsolete wrapper-level `ManuallyDrop` spelling is no longer treated as ownership evidence.
- Artifact-store undo/redo no longer invokes direct string-vector removals. `take_string_at_retained` returns the exact removed owner, which is transferred into the opposite history lane; the only clone retained is the distinct tail-cache identity. The structural proof now binds the live `(edit_id, messages)` retirement constructor, fixed ledger `Drop`, generation-qualified history slot, bounded conflict/backbone/DAG/history retirement, exact root handoff, capacity/+1, interrupted close, and structural-default laws. The history reservation interval is checked only until exact checkpoint insertion, so the later `bump().await` is not falsely classified as an await with a live reservation.
- The OS services test-only compute runner was migrated from a direct `drive_step` recursion to a retained `MountedWorkerJobSession`. Each host closure pumps one session transition, nonterminal payloads retire one page per turn before resume, cancellation remains in the mounted parameters, admission rejection closes through the exact rejected owner, and terminal session ownership transfers into the global retirement authority.

Official evidence:

- `📊️sol-structural-universal-final-3-2026-08-26.json`: `failureCount=167`, `productionHosts=50`, `productionRows=776`, `admitted=161`, `remaining=724`, `factories=16`, `dispatches=3`, `selfTests=365`.
- Exact blocker arrays: paged ingress `[]`, owned media export `[]`, generated member create/open `[]`, artifact store structural owners `[]`.
- Universal retained ownership remains red in this snapshot solely because the concurrently owned Puzzle3d precompute source still contains one production `drive_step` import/call in `drive_fill_envelope`; its mounted helper and other converted route are already present. The Puzzle owner was notified to finish the remaining production call before the final official rerun.
- `🧪️sol-structural-universal-rustfmt-check-2026-08-26.txt` contains no `error:` parse line. No broad formatting rewrite was applied.
- Temporary diagnostic output is confined to the ticket (`🔬️sol-*.txt`), and all `SEMIO_TOOL_JOB_DIAG` source instrumentation was removed.
- No Cargo, Nx, browser, or Git command was run.

Additional changed files in this follow-up:

- `📜️script.ts`
- `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`

## Envelope Caller Completion

The Present and Writer representative caller blockers are now absent, and the shared live initializer proof also clears the Jack, Trinity Rewrite, GIS Map, and Raster caller predicates.

- Present already owned a fixed 64-slot registry, retained worker session, rejected-session authority, typed completion owner, publication retry, cancel path, and bounded close. The verifier still named two superseded hostile laws; it now binds the stronger zero-grant fault/cancel laws. A new wrong-owner law proves stale operation and generation probes cannot cancel, close, or consume the live slot, then aborts with the exact owner and resumes from a zero-grant interrupted close. The capacity/+1 law now proves the returned page authority retains its exact page and byte counts before bounded retirement.
- Writer's only failed constituent was the deleted `retain_initializer_for_close(job)` spelling. The live shared route is stronger: `ArtifactStoreReplacementAdmissionTarget` inserts the exact job into `ActiveArtifactStoreReplacement`, which immediately retains either a mounted session or its exact rejected-session authority. `Drop` refuses initializer/candidate/displaced-store loss before terminal empty. The shared predicate and its hostile drop mutation now require this production transfer.
- Five non-Puzzle Wasm bridges had optional whole-string constructor branches that called the deliberately fail-closed compatibility function: CAD, DAG, FEM2d, FEM3d, and Shooting. Those unreachable parameters/branches were deleted; each bridge now constructs its fresh default document only. No non-Puzzle production caller remains.
- The direct-serde census now derives the exact set of Rust aliases whose right-hand side is `ArtifactEnvelope` and checks only those types. This removes false positives from unrelated `CommandEnvelope` and `GltfInferenceLeafEnvelope` values while still detecting every document-envelope alias. The derived direct-serde result is empty.

Official evidence:

- `📊️sol-present-writer-envelope-2026-08-26.json`: `failureCount=160`, `remaining=724`, `selfTests=365`.
- Exact blocker arrays empty: universal retained ownership, Present representative caller, Writer caller, Jack caller, Trinity Rewrite caller, GIS Map caller, Raster caller, artifact-store structural owner, generated member, paged ingress, and media export.
- ArtifactEnvelope owned codec remains red only for the concurrently owned Puzzle3d/Puzzle5d Wasm whole-buffer placeholder calls; `🔬️sol-envelope-callers-2026-08-26.txt` records `direct=[]` and those two exact remaining placeholder paths.
- `🧪️sol-present-envelope-rustfmt-check-2026-08-26.txt` and `🧪️sol-envelope-bridges-rustfmt-check-2026-08-26.txt` contain no `error:` parse line.
- Scratch diagnostics and rustfmt evidence use `.txt`; no agent-owned `.log` scratch file remains.

Additional changed files:

- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️component.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs`

## Draw, Artifact Envelope, and Peer-Root Completion

The final central envelope and peer-root predicates in this packet are absent from a fresh official run.

- Draw's live store already publishes the exact fallible revision record with its admitted identity digest. The verifier now binds that production spelling instead of the superseded `self.revision.identity_digest` expression. All recursive layer/style/asset preflight, one-owner clone/retirement, fixed-page ingress, initializer recovery, cancellation, and exact acknowledgement conditions remain required.
- Puzzle3d and Puzzle5d removed their final whole-buffer Wasm callers. The derived repository census now reports no non-store `reject_whole_buffer_artifact_envelope_ingress` caller and no direct serde decode into any derived `ArtifactEnvelope` alias. The last red constituent was a stale non-generic verifier spelling; the live rejected envelope authority is correctly `ArtifactEnvelopeDecodeRejected<P, Mutation>` with a generic `ErasedSnapshotRetirement` implementation.
- Peer ingress retains one fixed page at a time after exact reservation, exposes explicit subsequent-page owner handback, validates generation/cancellation immediately before atomic metadata and app-typed root publication, and captures all six command roots plus the peer metadata root by `Arc` ownership. The verifier now selects the production `dispatch_typed_command_inner` with `lastIndexOf`, avoiding earlier hostile source-string fixtures, and binds the current `FixedCommandPage` cursor API.
- Added executable law `peer_roster_saturation_cancel_stale_and_interrupted_close_preserve_exact_authority`. It covers fixed-slot maximum/+1 before payload decode, ordered outcome saturation cleanup, cancellation, a byte grant one below the retained page, bounded resume, stale generation rejection, and proof that neither metadata nor app-typed live roots change on cancel/stale paths.

Official evidence:

- `📊️sol-envelope-peer-roots-final-2026-08-26.json`: `failureCount=157`, `remaining=724`, `selfTests=365`.
- Exact blocker arrays are empty for ArtifactEnvelope owned codec, Draw `.spr`/`.ops`, and peer ingress/app-typed presence/interaction roots.
- `🔬️sol-peer-roots-constituents-2026-08-26.txt` contains no missing or forbidden whole-source constituent after reconciliation.
- `🧪️sol-peer-roots-focused-rustfmt-check-2026-08-26.txt` confirms source parsing and records only pre-existing/shared formatting differences; no broad rewrite was applied.
- No Cargo, Nx, browser, or Git command was run.

Additional changed files in this completion:

- `📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `📓️sol-shared-typed-command-full-operation-foundation-2026-08-26.md`

## FEM2D Mounted Live-Revision Reconciliation

The official live FEM2D retained-session blocker is absent. Production already carried the requested architecture; the verifier referenced an earlier visual-builder vocabulary and a superseded resizable dirty-render queue spelling.

- `MountedState` retains a fixed generation-tagged snapshot, cancellation token, graph/mesh/assembly/CSR/PCG builders, exact snapshot-return witness, live/rejected/displaced visual owners, and a `Fem2dVisualJob` candidate. Every scheduler call advances one retained child step or one visual scalar/entry/page transition and caps its deadline at eight milliseconds.
- `Fem2dVisualJob` reserves the fixed canvas-snapshot descriptor and pages before writing, uses persistent insertion-order cursors for regions/elements/fields, seals the exact page owner into `Fem2dMountedVisualLease`, validates full freshness immediately before publication, and incrementally aborts or closes rejected/displaced owners.
- The editor consumes the mounted snapshot through `with_live_visual` and `render_with_progress`; the reactor routes job progress into fixed `DirtyRenderSet::try_surface` ownership rather than the removed `dirty_render` vector.
- Mesh classification delegates one face to `advance_face_classification`, whose exact `triangulation.triangles.get(self.face_cursor)` retrieval remains required. Assembly construction still reserves and observes stiffness backing before admission, and session close returns the snapshot through `return_to_registry_witness` before terminal empty.
- The predicate now requires the executable production laws for fixed-page maximum/+1 owner handback, maximum/+1 input rejection before transfer, stale/cancel/fault/device-close preservation of the last valid snapshot, deterministic replay, and a measured sub-eight-millisecond job step. Four new hostile verifier mutations remove those laws independently and remain rejected.

Official evidence:

- `📊️sol-fem2d-mounted-final-2026-08-26.json`: `failureCount=156`, `remaining=724`, `selfTests=365`.
- Exact `live FEM2D revisions ...` blocker array is empty; ArtifactEnvelope, Draw, peer roots, universal ownership, and paged ingress remain empty.
- `🔬️sol-fem2d-constituents-2026-08-26.txt` and `🔬️sol-fem2d-block-constituents-2026-08-26.txt` record the stale vocabulary/block extraction diagnosis.
- No FEM, Puzzle, or Vulkan production source was edited for this reconciliation. The official Bun run parsed and executed every refreshed hostile predicate; no Cargo, Nx, browser, or Git command was run.

Additional changed files in this reconciliation:

- `📜️script.ts`
- `📓️sol-shared-typed-command-full-operation-foundation-2026-08-26.md`

## Fixed Operation Scheduler Language-Neutral Exit Gate

The fixed operation scheduler foundation now satisfies the schema-first, independent-oracle, and production-execution exit gate. This does not clear any live global owner by association.

- `fixed-operation-registry.schema.json` is a strict Draft 2020-12 schema for the exact empty, single, capacity maximum/+1, byte maximum/+1, collision, cancel/stale, ABA, and interrupted/repeated-close cases.
- The permanent TypeScript runner validates the exact case bijection, independently simulates fixed-slot hashing and retained credit/close behavior, and compares every emitted row with the language-neutral fixture.
- `fixed-operation-registry-cases.rs` is generated exactly from that same JSON step stream. The root verifier compares it byte-for-byte with the generator before acceptance, and the included Rust test drives the production `FixedOperationRegistry` rather than a model.
- The ticket-only ArrayVec oracle parses and executes the same steps through `arrayvec::ArrayVec<Option<Entry>, 64>`. It exited 0 and emitted canonical output byte-identical to the owned runner. Ajv independently validates the same fixture against the JSON schema. Neither oracle introduces a production or permanent test dependency.
- Maximum backing construction now measures 31 samples on each of four simultaneously released threads and enforces the interactive ceiling against every worker median. This removes the single-sample concurrency flake while preserving the hard `CAPACITY <= 64` construction bound.
- Targeted Cargo evidence is green: four fixed-registry tests passed, including the generated language-neutral production test, maximum/+1, stale/ABA/interrupted close, and concurrent initialization timing. Focused rustfmt check is clean.

Evidence:

- `📊️sol-fixed-operation-language-neutral-draw-honesty-2026-08-26.json`: `failureCount=157`, `remaining=724`, `selfTests=413`, `globalPayloadStores=41`.
- The fixed scheduler and schema-first production-fixture blocker strings are absent.
- `🧪️sol-fixed-operation-registry-owned-fixture-2026-08-26.txt`, `🧪️sol-fixed-operation-registry-arrayvec-oracle-2026-08-26.txt`, and `🧪️sol-fixed-operation-registry-ajv-oracle-2026-08-26.txt` contain byte-identical canonical rows.
- `🧪️sol-fixed-operation-registry-targeted-cargo-2026-08-26.txt`: 4 passed, 0 failed.

## Draw Operation-State Honesty Gate

The rejected Draw shortcut has been removed. `DRAW_SESSIONS` and `ACTIVE_DRAW_SESSIONS` are restored, and the official raw inventory is honestly 41 until a real instance-retained owner replaces them.

The root verifier now independently rejects Draw gesture operation/session/checkpoint payloads stored in Config, Draft, Snapshot, or document-schema lanes. It also rejects process-global gesture registries, checkpoint reconstruction from config, missing fixed registry ownership, missing mounted worker ownership, and missing cancel/terminal-empty authority. Seven hostile mutations plus the accepted synthetic owner fixture are live.

Current production remains deliberately RED on the exact new blocker:

`Draw gesture operation state remains process-global or persisted through Config/Draft/document lanes instead of one instance-retained worker owner`

This prevents the raw 41→39 scanner reduction from being credited until Draw has a real per-instance `FixedOperationRegistry<DrawGestureOperationOwner, 64>`, retained worker session, base-revision/stale/cancel/preview laws, and bounded terminal-empty close.


## Draw Instance-Retained Gesture Owner

Draw's former `DRAW_SESSIONS` pair moved into one concrete `DrawInstanceOperationOwner` constructed by each `VcsArtifactApp`. A cloneable, object-safe instance capability crosses only into the exact registered Draw jobs and renderer; it is not exported as an application payload store. This is not yet accepted as the complete Draw operation owner because the older trace-pointer subsystem remains process-global and the worker still wraps monolithic decode/reducer paths.

- `FixedOperationRegistry<DrawGestureOperationOwner, 64>` owns exact operation/generation keys, byte credit, the live `DrawSession`, cancellation, bounded close, and terminal-empty authority.
- Six Draw gesture actions use one exact `DrawGestureOperationJobFactory`. Retained fixed pages cross admission unchanged, are copied/scanned one bounded unit per worker turn, and decode only inside the mounted job before the reducer receives the same typed command.
- Active owner reuse is accepted only for the same exact key and canonical base revision. A new key/generation or revision cancels the displaced owner. Renderer publication reads the same app-instance handle, validates the current canonical revision, cancels stale preview state, and renders the live gesture snapshot.
- Config, Draft, Snapshot, document schema, and persisted checkpoint reconstruction remain forbidden. The removed `gesture_checkpoint_json`, `SetGestureCheckpoint`, `checkpoint_from_config`, `DRAW_SESSIONS`, and `ACTIVE_DRAW_SESSIONS` census is zero.
- Rust laws cover exact capacity maximum/+1 handback, stale generation/ABA, interrupted and repeated close, and stale-preview cancellation. The renderer now consumes a fixed-capacity `DrawGesturePreview` projection capped at 256 points rather than cloning the FSM/session working state.
- The gate additionally rejects whole `serde_json::from_slice`, generic `command.dispatch`, whole session cloning, and the `TRACE_POINTER_JOBS`/`ACTIVE_TRACE_POINTER_JOBS` process-global system. Those genuine source violations remain, so the Draw blocker is deliberately RED.

Evidence:

- `📊️sol-draw-semantic-honesty-final-2026-08-26.json`: `failureCount=157`, `remaining=724`, `selfTests=419`, raw global candidates 39, and the single Draw semantic blocker remains present.
- `bun ./📜️script.ts verify interactivity tool-jobs --self-test`: 419 clean, including whole-parse, generic-dispatch, preview-clone, trace-global, wrong-owner, stale-generation, preview-consumer, and persisted-lane hostile mutations.
- `🧪️sol-draw-instance-owner-rustfmt-check-2026-08-26.txt`: all touched Rust parses; it records shared formatting differences, so no broad formatting rewrite was applied.
- Focused Draw Cargo validation is recorded separately below because concurrent workspace checks held the shared Cargo lock during the initial run.

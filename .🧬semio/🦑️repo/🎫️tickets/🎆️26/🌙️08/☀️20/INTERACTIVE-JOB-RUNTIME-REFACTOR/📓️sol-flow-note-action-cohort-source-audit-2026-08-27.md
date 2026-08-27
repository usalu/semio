# Flow and Note Action Cohort Source Audit

## Scope and decision

Audited the canonical `FlowPlayApp` 37-row command macro and `NotePlayApp` 36-row command macro. No route currently owns the complete bounded decode → persistent semantic cursor → progress/cancel/freshness/ACK → exact lane preparation → root retirement → incremental-close lifecycle, so no route is honestly admissible as `Migrated`.

| Owner | Audited | Retained | BatchOnlyPendingRewrite | Framework-delegated | Blocking globals | Admitted scan-then-monolith |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `FlowPlayApp` | 37 | 0 | 37 | 0 | 2 | 0 |
| `NotePlayApp` | 36 | 0 | 36 | 1 (`setActiveUtility`, still explicitly fail-closed at the Note manifest) | 1 | 0 |
| Total | 73 | 0 | 73 | 1 | 3 | 0 |

Because the retained set is empty, neither owner declares `PUBLICATION_CONTRACTS`, a `ToolJobFactory`, an `ArtifactOwnedToolJobFactory`, a factory registration, a retained proof, or app-owned store preparation. Adding any of those without the missing runtime ownership would be a forged admission.

## Exact publication inventory

The schema-first fixtures group every route by its currently emitted completion lanes:

- Flow: Artifact 12; Config 18; Config + Child 1; HostOnly 6.
- Note: Artifact 26; Config 5; Artifact + Config 2; HostOnly 3.

`setActiveExample` and `setFixtureJson` are classified as Artifact because their host effects replace the document. `saveDownload` and `loadRequest` are HostOnly effects. `setActiveUtility` uses a framework reducer but is still a Note command row without a retained Note publication owner, so it is explicitly `BatchOnlyPendingRewrite`.

## Exact blockers and globals

- `PROCESS_FLOW_EVAL_SESSION` is a framework-process singleton reached through `with_process_flow_eval_session` from Flow handle, pending-effects, and render paths. It is outside this owner packet and prevents owner-local evaluation state.
- `NEXT_DUPLICATE_WIDGET_REQUEST` is a Flow plugin-process `AtomicU64`; duplicate progress generations are not an owner-local persistent cursor.
- `NEXT` in Note `create_note_id` is a plugin-process `AtomicU64`; durable generated model identifiers are not allocated from owner-local persisted state.

The official verifier's narrow global heuristic reports zero scoped Flow/Note payload stores, while the independent fixture/source scan intentionally records these three broader session/identity blockers. No route is admitted to a scan-then-monolith path; legacy reducers remain unreachable through the retained job registry.

## Schema and hostile laws

The shared JSON Schema and one fixture per owner enforce the exact route count, fail-closed status, declared lane vocabulary, unique routes, exclusive `HostOnly`, empty retained set, empty scan-then-monolith set, and explicit blockers. The Bun gate uses third-party Ajv plus an independent source oracle that parses both `app_commands!` rows and literal manifest annotations.

Hostile coverage rejects:

- changing a route to `Migrated` before a decoder/factory exists;
- injecting a forged retained proof;
- adding a retained fixture route;
- changing the exact census;
- mixing `HostOnly` with a store lane.

A Rust `serde_json` test independently parses both fixtures and checks owner, census, empty retained set, and route uniqueness. It is source-complete but deliberately not compiled in this packet because Store holds the exclusive compiler lease.

## Commands and results

1. `bun './✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/📜️script.ts' action-cohort-audit`
   - Green, exit 0.
   - `routes=73; retained=0; failclosed=73; frameworkDelegated=1; globals=3; scanThenMonolith=0; schema=Ajv; oracle=independent`.
   - Complete log: `🧪️sol-flow-note-action-cohort-ajv-source-r3.log`.
2. `bun './📜️script.ts' verify interactivity tool-jobs --format json --output '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📊️sol-official-tool-jobs-flow-note-source-r2.json'`
   - Repository-wide red, exit 1, for unrelated global Puzzle/importer/framework-reserved/remaining-command ledgers.
   - Scoped Flow/Note result: accepted 0, remaining 0, factories 0, verifier-global stores 0, scan-then-monolith 0.
   - Full log/report: `🧪️sol-official-tool-jobs-flow-note-source-r2.log`, `📊️sol-official-tool-jobs-flow-note-source-r2.json`.
   - Scoped extraction: `📊️sol-official-tool-jobs-flow-note-scoped-r2.json`.
3. `git diff --check -- '✏️s/🔌️plugins/🌊️flow' '✏️s/🔌️plugins/🗒️note'`
   - Green, exit 0.
   - Complete log: `🧪️sol-flow-note-action-cohort-diff-check-r1.log`.

No Cargo, Nx, rustc, or rustfmt command was run. Native compilation, the serde test execution, and rustfmt remain pending an explicit compiler-lease grant.

## Flow retained follow-up checkpoint

The Flow-only follow-up removes the two blockers previously counted for Flow: the duplicate continuation derives its request identity from the operation-owned checkpoint, and the evaluation session is held by `FlowInstanceOperationOwner` instead of `PROCESS_FLOW_EVAL_SESSION`. Note remains untouched and still owns the one process-global blocker counted by the shared fixture.

Six exact HostOnly routes now have one concrete `FlowHostEffectJobFactory`, exact owner/schema/controller proofs, one nonempty exclusive HostOnly publication contract per tool, byte-incremental wire activation, cancellation, freshness through the mounted operation, single-assignment completion, incremental raw-owner return, and terminal-empty witnesses:

| Routes | Lane | Retained microstate |
| --- | --- | --- |
| `contextMenuAt`, `openSpotlight`, `replaceImage` | HostOnly | `RawPage { page, byte, decoder } -> CompleteNoStoreOutput -> Close`; these are genuine no-ops in their current command definitions. |
| `flowEvalResolve` | HostOnly | `RawPage -> ResolveOneNodeCacheEntry -> EmitNextTick -> Close`, using the app-instance evaluation owner. |
| `evaluate` | HostOnly | `RawPage -> ArmInstanceEvaluation -> EmitNextTick -> Close`. |
| `flowEvalTick` | HostOnly | `RawPage -> AdvanceOneEvaluationBudget -> EmitExtensionOrNextTick -> Close`. |

The source-only Ajv/oracle gate is green at Flow retained 6 / fail-closed 31; combined with untouched Note it reports retained 6 / fail-closed 67 / framework-delegated 1 / blocking globals 1 / scan-then-monolith 0. Complete output is `🧪️sol-flow-hostonly-action-cohort-ajv-source-r1.log`.

## Flow Store-lane microstate plan

The remaining 31 routes deliberately remain `BatchOnlyPendingRewrite` until Store exposes the stable live publication-authority packet. The app must never invent cursor ids, HLC/actor, base generation/revision, or base counts. Every family below starts with the same retained `RawPage { page_index, byte_index, strict_decoder }`, admits bounded command collections before semantic work, echoes Store-minted authority metadata into app preparation, yields prepared semantic edits/post-root/digest only, waits for publication/ACK, and retires the pre-root, candidate values, decoder, raw pages, and completion handles one bounded close item at a time.

### Artifact preparation — 12 routes

| Route | Persistent semantic cursor and bounded unit |
| --- | --- |
| `addWidget` | `ValidateDescriptor -> ScanWidgetIds { index, suffix } -> BuildCreateWidget -> PrepareArtifact`; scan at most one admitted widget-id page per step and never call `FlowHost::add_widget`. |
| `removeWidget` | `FindWidget { index } -> ScanIncidentSynapses { index } -> BuildDisconnects -> BuildDelete -> PrepareArtifact`; one node/edge per step. |
| `deleteSelection` | `ReadBoundedSelection -> ScanSelectedNodes { selection_index, scene_index } -> ScanSelectedEdges -> BuildDeletes -> PrepareArtifact`; selection ids and output edits are fixed-capacity. |
| `disconnect` | `FindSynapse { index } -> BuildDisconnect -> PrepareArtifact`; one edge comparison per step. |
| `connectMediaPorts` | `FindSource -> FindTarget -> ValidatePorts -> CycleFrontier { edge_index, frontier_cursor, visited } -> FindOccupiedTargetPort -> ScanSynapseId { suffix, index } -> BuildEdits -> PrepareArtifact`; fixed frontier/visited/output caps, no `FlowHost::connect_ports`. |
| `moveMediaNode` | `FindNode { index } -> BuildMoveWidgets -> PrepareArtifact`; one node/layout lookup page, preserving the amend key. |
| `reorganize` | `BuildAdjacency { edge_index } -> LayerFrontier -> OrderLayer { layer, index } -> AssignPosition { node_index } -> BuildMoveWidgets -> PrepareArtifact`; persistent layout frontier and coordinate cursor, never `host.reorganize` followed by a whole diff. |
| `patchFlowWidgets` | `AdmitIds -> ScanWidgets { widget_index, selected_index } -> BuildReplaceWidget -> PrepareArtifact`; one widget/id comparison or one replacement per step, preserving the coalesce key without joining unbounded ids. |
| `renameFlowWidget` | `ValidateNewId -> CollisionScan { widget_index } -> RewriteWidget -> RewriteSynapses { edge_index } -> MoveLayout -> PrepareArtifact`; direct semantic edits, no cloned fixture plus `snapshot_operations`. |
| `nodeGraphEdit` | `Operation { operation_index, subcursor }`, where subcursor is incremental strict snapshot decode, bounded selected-delete scan, or the connect cycle cursor above; each sub-operation appends only admitted semantic edits. |
| `spotlightCommit` | Same closed `Operation { operation_index, subcursor }` state machine as `nodeGraphEdit`, with a distinct tool identity and proof. |
| `runExtensionAction` | `ResolveAutomation { registry_index } -> CheckEnabled { config_json_cursor } -> ReorganizeCursor` for the artifact-producing action; the evaluate action advances the instance evaluation cursor and produces no Store item. Static publication remains Artifact because HostOnly cannot be mixed with a Store lane. |

### Config preparation — 18 currently declared routes

| Routes | Persistent semantic cursor and bounded unit |
| --- | --- |
| `setContributions`, `nodeGraphViewport`, `setLodMode`, `setProximityDistance`, `setGridVisible`, `setGridSnapEnabled`, `setGridFactor`, `setCatalogueSections`, `setLocale` | `ValidateBoundedValue -> BuildOneConfigMutation -> PrepareConfig`; scalar/one bounded-string work only. |
| `focusSelection` | `ReadBoundedSelection -> ScanNodeBounds { selection_index, scene_index, min/max } -> BuildCamera -> PrepareConfig`; one selected-id/node comparison per step. |
| `setPreviewOff` | `AdmitIds -> ScanCurrentPreviewIds { current_index, request_index } -> BuildBoundedNextIds -> PrepareConfig`; no `contains` nested monolith and no unbounded replacement vector. |
| `toggleExtension` | `DecodeAutomationMap { raw_index, entry_count } -> FindOrInsertEntry -> EncodeMap { entry_index, output_chunk } -> PrepareConfig`; strict JSON/member/string caps. |
| `addGeneration`, `removeGeneration`, `selectGeneration`, `renameGeneration`, `updateGenerationValues` | `DecodeGeneration { byte/member/depth } -> BuildFormSpec { widget_index, question_index } -> ApplyOneGenerationAction -> PatchPreviewFixture { value_index, widget_index } -> EvaluatePreview { eval_cursor } -> EncodeGeneration { member_index, output_chunk } -> PrepareConfig`; no `handle_generation_action`/serialize/evaluate run-to-completion call. |
| `duplicateWidget` | Proposed end-to-end retained ownership changes this route to Config + Child: `ValidateIdentity -> SnapshotChildRevision -> SourceScan -> WidgetCollisionScan -> EdgeCollisionScan -> BuildChildEdits -> ClearProgress -> PrepareConfigAndChild`. The existing 64-row scan cursor and 4 KiB checkpoint envelope are reusable, but the effect-dispatched legacy continuation is not. |

### Config + Child preparation — 1 route

| Route | Persistent semantic cursor and bounded unit |
| --- | --- |
| `duplicateWidgetStep` | `ValidateCheckpointFields -> ValidateAppDocumentOperationGenerationBaseAndChildRevision -> SourceScan { index } -> WidgetCollisionScan { index, suffix } -> EdgeCollisionScan { index, suffix } -> BuildChildEdits -> ClearOrCancelProgress -> PrepareConfigAndChild`; activation must finish before child revision/read authority is touched. It shares the duplicate cursor implementation but has its own exact tool contract/proof. |

There are no Flow Presence, Draft, Transient, or import routes in this 37-row command cohort. Import/export reserved routes are a separate app surface and are not silently folded into this factory.

## Follow-up command and result

`bun './📜️script.ts' action-cohort-audit`, from `✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript`, exited 0:

`validated Flow/Note action cohort; routes=73; retained=6; failclosed=67; frameworkDelegated=1; globals=1; scanThenMonolith=0; schema=Ajv; oracle=independent`

No Cargo, Nx, rustc, rustfmt, or repository-wide official verifier was run during this source-only checkpoint because Store retains the compiler lease and its publication API is concurrently changing.

## Files

- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🌊️flow/🧪️action-cohort/🔣️schema.json`
- `✏️s/🔌️plugins/🌊️flow/🧪️action-cohort/🔣️component.json`
- `✏️s/🔌️plugins/🗒️note/🧪️action-cohort/🔣️component.json`

## Final Flow retained implementation checkpoint

The final honest Flow census is 21 retained and 16 fail-closed routes. The retained set contains 15 Store-lane routes and 6 HostOnly routes:

- Artifact: `removeWidget`, `deleteSelection`, `disconnect`, `moveMediaNode`, `patchFlowWidgets`.
- Config: `nodeGraphViewport`, `setLodMode`, `setProximityDistance`, `setGridVisible`, `setGridSnapEnabled`, `setGridFactor`, `setPreviewOff`, `setCatalogueSections`, `toggleExtension`, `setLocale`.
- HostOnly: `evaluate`, `contextMenuAt`, `openSpotlight`, `replaceImage`, `flowEvalTick`, `flowEvalResolve`.

The Store preparations use only the sealed live-authority path `authority.prepare_one_item(edit, Arc::new(post))`; Flow reads `prepared.edit_digest()` for its checkpoint and owns no digest function or fabricated cursor/base identifiers. Artifact and Config preparation factories directly build bounded semantic edits and post roots, preserve the exact Store authority, return the exact pre-root, and retire preparation owners incrementally. Direct Store work holds persistent semantic and scan cursors, a 17-byte replay checkpoint, fixed-capacity owned collections, incremental close, and a terminal-empty witness. The app-instance evaluation owner replaces the former process-global evaluation session, and duplicate request identity is operation-owned rather than an `AtomicU64` process global.

The exact source-visible proof macro contains 21 literal rows and the two registered factories each expose a nonempty exact `PUBLICATION_CONTRACTS` table. The official verifier report `📊️sol-flow-official-tool-jobs-r3.json` accepts all 21 Flow rows, finds both Flow contracts explicit, and reports no remaining Flow-owner row, Flow global payload store, or Flow scan-then-monolith row. Repository-wide verifier status remains red for other owners; the Flow-scoped result is clean.

The 16 routes left `BatchOnlyPendingRewrite` are not counted as retained: `addWidget`, `connectMediaPorts`, `reorganize`, `renameFlowWidget`, `nodeGraphEdit`, `spotlightCommit`, `runExtensionAction`, `setContributions`, `focusSelection`, `addGeneration`, `removeGeneration`, `selectGeneration`, `renameGeneration`, `updateGenerationValues`, `duplicateWidget`, and `duplicateWidgetStep`. Their fixture blockers name the missing persistent collision/cycle/layout/generation/child-publication cursors. In particular, `connectMediaPorts` remains fail-closed until capability validation, occupied-port replacement, cycle detection, and Store-safe synapse-id allocation are all resumable.

### Final validation

1. `bun './📜️script.ts' action-cohort-audit flow` from the Flow TypeScript package: exit 0; `routes=37; retained=21; failclosed=16; frameworkDelegated=0; globals=0; scanThenMonolith=0; schema=Ajv; oracle=independent`.
2. `bun './📜️script.ts' action-cohort-audit all`: exit 0 against the latest concurrent Note fixture; `routes=73; retained=30; failclosed=43; frameworkDelegated=1; globals=0; scanThenMonolith=0; schema=Ajv; oracle=independent`.
3. `rustfmt --edition 2021` on the two exact Flow Rust files: exit 0.
4. `git diff --check --` on the five exact Flow-owned production/fixture/oracle paths: exit 0.
5. Native `cargo check --locked -p semio-s-plugin-flow --lib --message-format short` with `CARGO_INCREMENTAL=0` and ticket-isolated `CARGO_TARGET_DIR`: exit 101 before Flow. The cold attempt first found a concurrent stdio glTF patch-marker parse error; after that owner repaired it, the warm attempt reached a different unfinished stdio PDF mutation leaf and failed because `🔤️embed-font-file/🦀️component.rs` does not exist. No Flow compiler diagnostic was reached.
6. Wasm32-wasip2: not run. Native was not green and the same transitive stdio mutation tree is incomplete, so the staged Wasm gate was not started.

Complete concise command/result evidence is saved in `🧪️sol-flow-action-cohort-final-source-r4.log` and `🧪️sol-flow-native-upstream-blockers-r2.log`. Native and Wasm are explicitly unverified; no green claim is made.

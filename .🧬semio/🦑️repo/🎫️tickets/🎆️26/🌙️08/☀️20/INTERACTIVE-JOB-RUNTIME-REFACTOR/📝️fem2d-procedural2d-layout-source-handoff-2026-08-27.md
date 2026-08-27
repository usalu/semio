# FEM2d, Procedural2d, and Layout Source Handoff

## Status and Exact Counts

This packet is **source-complete for the admitted bounded envelope**, not whole-app interactive completion. Rust compilation, Rust runtime tests, mounted interaction timing, and real Store publication execution were not run by this executor; the coordinator owns the compiler lease.

| Owner | Commands | Bounded First Step | Resumable Export | Batch Only | Forbidden |
| --- | ---: | ---: | ---: | ---: | ---: |
| FEM2d | 19 | 3 | 0 | 16 | 0 |
| Procedural2d | 21 | 7 | 0 | 13 | 1 |
| Layout | 20 | 9 | 4 | 7 | 0 |
| Total | 60 | 19 | 4 | 36 | 1 |

The focused live-source verifier finds 23 exact command proofs, zero proof catalog failures, and zero scan-then-monolith rows. Their publication lanes are exactly 13 Config and 10 HostOnly. Layout's additional reserved media-export factory has an exact HostOnly contract; it is not counted as one of the 60 app commands.

Evidence: [source evidence](./📊️fem2d-procedural2d-layout-source-evidence-2026-08-27.json).

## Implemented Source Changes

### Config Preparation

All three owners now supply concrete app-owned Config one-item preparation factories.

- Admission measures aggregate string lengths without scanning or copying a whole record.
- Config base, incoming scalar mutation, and prepared post-state each have a 128-byte aggregate text envelope.
- Actor and description admission are each at most 64 bytes.
- The fixed retained footprint and each advance/close byte grant are 4,096 bytes, matching the real production Store grant; no larger shared grant was introduced.
- Exactly one bounded Config semantic unit is prepared with an explicit field update, exact granular inverse, exact-base undo metadata, and Store-produced digest.
- Operation ID, generation, and base revision must exactly match the captured live Store authority before owner transfer.
- Zero/undersized grants, cancellation, and closing cannot execute preparation.
- Preparation checkpoints are stable after publication transfer and do not repeat the mutation.
- Close releases one retained owner per step, returns the captured SnapshotRead to its registry, and witnesses an empty terminal state.

This intentionally rejects larger Config values and larger existing Config roots. It is **not** a full-envelope resumable string/root implementation. A genuinely larger-envelope solution needs retained string/root preparation, hashing, and retirement that make progress in 4,096-byte units. That is the next packet, not a completed claim here.

### FEM2d

Migrated Config commands: setCamera, setResultDisplay, setLocale.

setAnalysisSettings is no longer mislabeled as a Config-ready route: it mutates the FEM document. setActiveExample remains BatchOnly because parsing and LoadDocument replacement are one-shot and mixed with Config reset. All other model mutations explicitly remain BatchOnly pending granular Artifact Store preparation. Mounted FEM numerical/render session code was preserved.

### Procedural2d

Migrated Config commands: nodeGraphViewport, setShowMode, generate.

Migrated HostOnly no-op commands: canvasPointerDown, canvasPointerMove, canvasPointerUp, canvasWheel.

Removed the production scan-then-monolith work/factory and its misleading resumable tests/proofs. No scan cursor now earns interactivity credit merely by delaying a whole-graph reducer. The retained reducer also rejects non-admitted IDs directly.

No process-global payload registry was introduced. Existing surrounding edits replaced evaluator access with per-call FlowEvalSession construction; evaluator continuity is still a blocker, not a claimed fix. setEvalOutputs and flowEvalTick therefore remain explicitly BatchOnly. setLocale retains its existing ForbiddenFromUi classification.

### Layout

Migrated bounded commands: setActivePage, focusPreflightIssue, engagementInput, canvasPointerUp, canvasDragOver, canvasDragLeave, setCamera, setLocale, engagementSubmit.

The existing exportPng, exportSvg, exportPdf, and exportPackage state machines now have exact HostOnly publication contracts and owner-local proof catalog wiring. Their numerical/export implementations were preserved. Oversized export command wire input is rejected before reserve/allocation. Layout's media-export factory also received an exact HostOnly contract.

The editor's build_tool_job hook is synchronous, matching ArtifactEditor. Export construction is delegated to a synchronous local helper. As explicitly requested by the coordinator, the Home and Animate editor build_tool_job declarations were also mechanically corrected from async fn to fn; no other edits were made to those two owners by this executor.

## Cross-File Factory Verification

The verifier now resolves a concrete cross-file factory through the owner's unique crate-qualified import and the plugin's explicit #[path] module declarations in its Rust glue. It checks the actual ToolJobFactory implementation, create_job, execution_contract, exact ArtifactOwnedToolJobFactory owner type, and owner-local live registration. Missing or ambiguous import/module targets are rejected. Existing owner/file/schema/Migrated bijection checks remain.

Layout's two concrete factory catalogs are joined by its real bounded_first_step_tool_proofs implementation. No fake sentinel factory name or marker-only exemption was added.

Language-neutral JSON fixtures cover one honest cross-file case and eight hostile substitutions. Three strict-schema hostile cases are also rejected. The new fixture schemas reject additional properties.

## Verification Actually Run

| Check | Result |
| --- | --- |
| Focused Bun owner-factory fixture checks | Passed: 12 cases |
| Focused Bun cohort route/publication checks | Passed: all 60 routes; 23 migrated, 36 batch, 1 forbidden |
| Config mutation/reverse/replay comparison with existing Immer | Passed: 11 mutation oracles |
| Config source-admission/authority/retirement and strict-schema hostile checks | Passed: 49 cases |
| bun ./📜️script.ts verify interactivity tool-jobs --self-test | Passed: 618 self-tests |
| Targeted git diff --check | Passed, exit 0 |
| Cargo / Nx / rustfmt / compiler / Rust tests | Not run; coordinator lease |

Three Rust preparation-law tests were added for exact maximum/maximum-plus-one admission, zero/undersized grants, cancellation, retained mutation ownership during blocked close, one-owner retirement, and terminal emptiness. They have **not** been compiled or executed by this executor.

The static fixture and Immer oracle are not evidence that the Rust publication path ran. Runtime verification remains required.

## Precise Remaining Route Blockers

### fem2d

- addNode — BatchOnlyPendingRewrite: Fem2dSnapshot owns deep Vec model roots; Artifact mutation inverse/diff/publication needs granular retained-root Store preparation.
- addBar — BatchOnlyPendingRewrite: Fem2dSnapshot owns deep Vec model roots; Artifact mutation inverse/diff/publication needs granular retained-root Store preparation.
- addBeam — BatchOnlyPendingRewrite: Fem2dSnapshot owns deep Vec model roots; Artifact mutation inverse/diff/publication needs granular retained-root Store preparation.
- addMaterial — BatchOnlyPendingRewrite: Fem2dSnapshot owns deep Vec model roots; Artifact mutation inverse/diff/publication needs granular retained-root Store preparation.
- addSection — BatchOnlyPendingRewrite: Fem2dSnapshot owns deep Vec model roots; Artifact mutation inverse/diff/publication needs granular retained-root Store preparation.
- addSupport — BatchOnlyPendingRewrite: Fem2dSnapshot owns deep Vec model roots; Artifact mutation inverse/diff/publication needs granular retained-root Store preparation.
- addNodalLoad — BatchOnlyPendingRewrite: Fem2dSnapshot owns deep Vec model roots; Artifact mutation inverse/diff/publication needs granular retained-root Store preparation.
- addMemberUdl — BatchOnlyPendingRewrite: Fem2dSnapshot owns deep Vec model roots; Artifact mutation inverse/diff/publication needs granular retained-root Store preparation.
- addAreaLoad — BatchOnlyPendingRewrite: Fem2dSnapshot owns deep Vec model roots; Artifact mutation inverse/diff/publication needs granular retained-root Store preparation.
- addRegion — BatchOnlyPendingRewrite: Fem2dSnapshot owns deep Vec model roots; Artifact mutation inverse/diff/publication needs granular retained-root Store preparation.
- addLoadCase — BatchOnlyPendingRewrite: Fem2dSnapshot owns deep Vec model roots; Artifact mutation inverse/diff/publication needs granular retained-root Store preparation.
- addCombination — BatchOnlyPendingRewrite: Fem2dSnapshot owns deep Vec model roots; Artifact mutation inverse/diff/publication needs granular retained-root Store preparation.
- setSelfWeight — BatchOnlyPendingRewrite: Fem2dSnapshot owns deep Vec model roots; Artifact mutation inverse/diff/publication needs granular retained-root Store preparation.
- setAnalysisSettings — BatchOnlyPendingRewrite: Fem2dSnapshot owns deep Vec model roots; Artifact mutation inverse/diff/publication needs granular retained-root Store preparation.
- removeSelection — BatchOnlyPendingRewrite: Fem2dSnapshot owns deep Vec model roots; Artifact mutation inverse/diff/publication needs granular retained-root Store preparation.
- setActiveExample — BatchOnlyPendingRewrite: Bundled example parsing and LoadDocument replacement are one-shot and mixed with Config reset; requires retained document replacement publication.

### procedural2d

- nodeGraphEdit — BatchOnlyPendingRewrite: Whole FlowFixture/graph clone and host operation diff remain monolithic; needs granular graph root and Artifact Store preparation.
- moveMediaNode — BatchOnlyPendingRewrite: Whole FlowFixture/graph clone and host operation diff remain monolithic; needs granular graph root and Artifact Store preparation.
- addWidget — BatchOnlyPendingRewrite: Whole FlowFixture/graph clone and host operation diff remain monolithic; needs granular graph root and Artifact Store preparation.
- removeWidget — BatchOnlyPendingRewrite: Whole FlowFixture/graph clone and host operation diff remain monolithic; needs granular graph root and Artifact Store preparation.
- connectMediaPorts — BatchOnlyPendingRewrite: Whole FlowFixture/graph clone and host operation diff remain monolithic; needs granular graph root and Artifact Store preparation.
- reorganize — BatchOnlyPendingRewrite: Whole FlowFixture/graph clone and host operation diff remain monolithic; needs granular graph root and Artifact Store preparation.
- addGeneration — BatchOnlyPendingRewrite: Generation projection and preview evaluate/clone the full generation state; needs retained generation/preview computation and exact mixed Artifact/Config preparation.
- removeGeneration — BatchOnlyPendingRewrite: Generation projection and preview evaluate/clone the full generation state; needs retained generation/preview computation and exact mixed Artifact/Config preparation.
- renameGeneration — BatchOnlyPendingRewrite: Generation projection and preview evaluate/clone the full generation state; needs retained generation/preview computation and exact mixed Artifact/Config preparation.
- updateGenerationValues — BatchOnlyPendingRewrite: Generation projection and preview evaluate/clone the full generation state; needs retained generation/preview computation and exact mixed Artifact/Config preparation.
- selectGeneration — BatchOnlyPendingRewrite: Generation projection and preview evaluate/clone the full generation state; needs retained generation/preview computation and exact mixed Artifact/Config preparation.
- setEvalOutputs — BatchOnlyPendingRewrite: FlowEvalSession is reconstructed per call; app-instance evaluator authority and retained evaluator state must own this route before it can be interactive.
- flowEvalTick — BatchOnlyPendingRewrite: FlowEvalSession is reconstructed per call; app-instance evaluator authority and retained evaluator state must own this route before it can be interactive.
- setLocale — ForbiddenFromUi: Existing locale command is intentionally not exposed as a Procedural2d UI route.

### layout

- addFrame — BatchOnlyPendingRewrite: LayoutSnapshot owns deep page/frame/style roots; mutation cloning/diff and Artifact Store publication require retained per-item preparation.
- addPage — BatchOnlyPendingRewrite: LayoutSnapshot owns deep page/frame/style roots; mutation cloning/diff and Artifact Store publication require retained per-item preparation.
- patchPage — BatchOnlyPendingRewrite: LayoutSnapshot owns deep page/frame/style roots; mutation cloning/diff and Artifact Store publication require retained per-item preparation.
- patchFrame — BatchOnlyPendingRewrite: LayoutSnapshot owns deep page/frame/style roots; mutation cloning/diff and Artifact Store publication require retained per-item preparation.
- canvasDrop — BatchOnlyPendingRewrite: LayoutSnapshot owns deep page/frame/style roots; mutation cloning/diff and Artifact Store publication require retained per-item preparation.
- canvasPointerDown — BatchOnlyPendingRewrite: Hit testing builds a full page display list before selection/hover effects; requires retained scene/hit-test cursor.
- canvasPointerMove — BatchOnlyPendingRewrite: Hit testing builds a full page display list before selection/hover effects; requires retained scene/hit-test cursor.

## Files Changed by This Executor

- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️component.rs
- 📜️script.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧬️schema/🔣️owner-factory-resolution.schema.json
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️fixtures/🔣️owner-factory-resolution.json
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧬️schema/🔣️scalar-config-cohort.schema.json
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️fixtures/🔣️scalar-config-cohort.json
- ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs
- ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📊️fem2d-procedural2d-layout-source-evidence-2026-08-27.json
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📝️fem2d-procedural2d-layout-source-handoff-2026-08-27.md

The root script and Home/Animate files already contain substantial concurrent changes by other executors. Their full git diff sizes must not be attributed to this packet. No modifying git commands were run. The existing ticket remains coordinator-owned.

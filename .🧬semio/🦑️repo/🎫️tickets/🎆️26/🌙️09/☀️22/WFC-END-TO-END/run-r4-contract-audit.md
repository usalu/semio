# Run R4 Current Contract Audit

Read-only source inspection, 2026-11-05. This snapshot is subject to concurrent edits. No build or browser gate was launched.

## Findings

The WFC 3d, grid3d and grid2d package manifests now declare `semio-plugin` and `semio-os-kernel`, but several live source paths still name `semio_framework_plugin` or `semio_framework_os_kernel`. The artifact roots inspected alias `semio_os_kernel` to `dsl`, `protocol`, `store`, and `vcs`, but do not define aliases for the old crate names. In particular:

- 3d viewer preview window uses the old plugin path at lines 60–66.
- Grid3d editor uses the old plugin path at lines 328–329; its fill tool uses it at lines 324, 352 and 622, and preview window at line 43. Grid3d transient uses the old kernel path at lines 3 and 56–57.
- Grid2d editor window uses the old kernel path at lines 14 and 430–431, and the old plugin path at lines 211 and 466–567.
- Retained-command and viewer tests for 2d and 3d, store fixture tests, grid2d and grid3d fill and preview tests also use the old plugin path.

Both implementation owners received these findings. Recheck the current files before editing because they are actively changing.

A `tomllib` inspection of the WFC package manifests with explicit `package.workspace` found no inherited dependency absent from the referenced workspace at the time of the scan. This verifies dependency-key presence only, not feature availability, path validity, target-specific dependencies, or successful compilation. Previous engine/oracle and bitmap root stale dependency findings had already been reported to owners and were being fixed concurrently.

## Browser scenario

`wfc-browser-cases.json` initializes all five variants to seed 4001; `wfc-browser-scenario.ts` changes it to 4002 and checks stale output disappears before a fresh fill, preserving extents. Canvas projections compare with the pre-solve digest; `previewDigest` waits for two equal screenshots. 3d projections require a positive solved count equal to the total. Source inspection alone cannot establish browser success.

`surfaceWitness` takes only one screenshot per reload witness, so transient rendering can affect pixel equality; it does not use the stable screenshot loop from `previewDigest`. The wfc3d status parser recognizes English strings only; verify its status contract against the locale exercised. The revision helper returns early if the seed form is missing or unsuccessful, so it should report the missing scenario explicitly if form failures do not already do so.

## Graph verification

`build/🕸️graph/🟦️.ts:794` documents a selected scope derived from the newest sealed generation, with fresh verification of its closure; generation source `🕰️generations/🟨️.mjs:500,603–614` describes verified reuse, missing-generation builds, shared builder custody and closure refresh. `🕰️generations/🔌️load/🟨️.mjs:293` distinguishes whole-workspace from selected roots. Advancing counters do not establish a hang. The counter alone does not prove whether the active invocation is whole-workspace or closure scoped, or whether it is reusing a peer builder. No policy-preserving narrower CLI invocation was established in this audit.

## Current Source Integrity Audit

A read-only scan of `✏️s/🔌️plugins/🀄️wfc` and `🌎️hub/🧩️compositions/🀄️wfc` on 2026-11-05 inspected 904 Rust, 157 TypeScript, 770 JSON and 71 TOML files, excluding build caches and intentional binary `.semio` resources. All inspected files decoded as UTF-8; no NUL bytes or standard line-start merge conflict markers were found. All JSON files parsed successfully. Every Cargo manifest parsed as TOML; each direct dependency path resolved to a Cargo.toml, and each workspace member glob matched existing Cargo manifests. These are source integrity checks, not compilation, semantic schema validation, or runtime proof.

Read-only Git queries of staged, unstaged and untracked paths under those two scopes found 38 changed paths at the time of the check. Their maximum absolute path length using the current `/home/deck/semio` root was 170 UTF-16 code units; none exceeded 256. This is a point-in-time result in a shared workspace and is sensitive to checkout-root length. No compiler, build or runtime gate was launched.

## Mounted Render And Fill Source Audit

Read-only inspection of current five editor paths: bitmap `artifact-editor::render_bodies` calls input/output rendering; grid2d `render_bodies` calls grid/preview; grid3d `render_bodies` calls grid/preview; wfc2d and wfc3d `render_body` call graph/preview. These preview paths construct scene data from snapshots, transient state and tool-run payloads. No synchronous cold solver invocation was found in these traced paths. The public headless `solve_with_job` functions exist in inference host modules; the only non-test-tree invocation found was explicitly under the wfc3d `transient/🧪️testing` fixture.

Each mounted editor registers a fill tool and its `build_tool_run_job` selects the fill tool for `Run` and constructs the corresponding `BitmapFillRunJob`, `Grid2dFillRunJob`, `Grid3dFillRunJob`, `Wfc2dFillRunJob`, or `Wfc3dFillRunJob`. Each job source publishes `ToolRunProgress`, tests its step context for cancellation, and admits a cancelled outcome. This is source evidence, not proof that the browser can observe pause, progress, or cancellation during a specific workload.

Revision and trace checks are present in the traced paint paths: bitmap compares live run base revision and transient source revision plus dimensions; grid2d compares the current revision to both live run identity and cached solve source; grid3d checks both live run identity and payload source and requires a done cached payload; wfc2d and wfc3d filter transient and live run by current revision and payload source. Grid2d, grid3d, wfc2d and wfc3d apply trace events only when `done` is false; terminal runs are excluded by their live payload readers. Bitmap output similarly excludes terminal live runs and validates current revision before rendering. No concrete stale-completed-trace or cold-solve-on-paint path was found in the inspected call graph. No tests or gates were launched.

## Registered Run Graph Scope Trace

`build/🧭️invocation/🕸️graph/🟦️.ts:14–22` derives selection from the original invocation arguments. `build/🕸️graph/🚪️admission/🟦️.ts:131–138` maps `run project:target` to a task selection. `build/🕸️graph/🟦️.ts:781–799` turns task projects into selected slice roots. `build/🕸️graph/🕰️generations/🔌️load/🟨️.mjs:299–308` passes those roots and the slice closure to `refreshClosure`; `🕰️generations/🟨️.mjs:603–614` passes the computed names to `refresh` as `only`. If no usable generation or complete closure exists, the initial `only` may be undefined and broader work is needed to establish the generation.

The `verification` counter is the number of suspect derived questions, not the number of graph projects: `🕰️generations/✂️cutoff/🟨️.mjs:84–100` reports `names.length`, while `🕰️generations/🟨️.mjs:390–392` constrains suspects to holders in the scoped owners plus environment. Therefore 2151 and 2141 by themselves do not establish whole-graph verification or a scope defect: a selected closure can hold many stale derived questions, and an unusable generation can require wider initialization. `🕰️generations/🧪️tests/🟦️.ts:191` covers verified closures without a builder, `🕰️generations/🧪️tests/🪞️plan-once/🟦️.ts:24` covers scoped verification, and `🕰️generations/✂️cutoff/🧪️tests/🏪️store/🟦️.ts` covers derived-fact scoping against an independent closure walk. No policy-preserving narrower run parameter or concrete defect was established. No gate was launched or interrupted.

## Gate Process Checkpoint

A read-only `/proc` command-line scan on 2026-11-05 observed live repository command fronts for browser bitmap time-travel, wfc-plugin, wfc-js, framework headless, WFC engine core, and each of the bitmap, grid2d, 2d, grid3d and 3d artifact Cargo gates. No requested front was absent at the snapshot. Process liveness is wait evidence only; it does not establish test completion, success, or active execution. No gate session was polled or interrupted.

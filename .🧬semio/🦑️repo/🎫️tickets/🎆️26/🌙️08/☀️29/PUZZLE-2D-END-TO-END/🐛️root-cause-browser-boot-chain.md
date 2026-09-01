# 🐛️ Browser boot chain — the six defects between "no plugins loaded" and a rendering 2D board

Traced live in the React OS dev server (`SEMIO_RENDERER=react`, port 6012) on 2026-08-30. Each defect
only became visible once the previous one was fixed, so they are listed in the order they surfaced.

## 1. Stale `VITE_SEMIO_APP_ID` in an already-running Vite

`🧑️‍💻️dev/🟦️component.ts` reads `import.meta.env.VITE_SEMIO_APP_ID ?? boot.defaultAppId`. The env var is
baked when `runViteBunxDev` spawns Vite, from `🔣️playgrounds.json`'s `app` column. The catalog had
already been regenerated with the new `s.puzzle.puzzle2d@1/*#editor` id, but the *running* server still
carried the pre-regeneration `s.puzzle2d@1/*#editor`, so `ShellHost` rejected the pinned app.

Not a code defect — but note that **editing any file inside the Vite config's import graph
(`🧵️shard-client.ts`, `🌐plugin-web-materialize.ts`, …) triggers a config restart that reliably wedges
in this repo**. Kill the chain and cold-start instead of waiting on the restart.

## 2. `PluginRuntime.createApp` bypassed the instance-lifecycle lease

`createApp` hand-built `{ kind: "instance-open", payload: { instance, appId, actor, config, assets,
capabilities, quotas } }` and pushed it through the plain `submitTurn` path. The guest's own wire
decoder requires `activation-generation` and `request-sequence` on that event, so every first turn died
with `actor-lifecycle.invalid-authority` before the guest ever ran.

The correct API already existed in the same module and was already covered by its tests
(`shardClient.captureInstanceLifecycle` → `submitPluginLifecycleTurn({ kind: "open" })` → `receipt-ack`);
only production `createApp` had never been migrated to it. Fixed by migrating it, tracking the lease per
instance, and clearing that map in `destroyApp`/`dispose`.

## 3. jco's `useDirectParams` rewrite had never run on the puzzle component

`semio_s_plugin_puzzle_component.js` carried `useDirectParams: true` on all seven `taskReturn` bindings.
The `poll` result exceeds the flat-return limit, so its discriminant was read from the return-area
*pointer* — `caseIdx: 15179872` against a two-case `result`, surfacing as
`undefined is not iterable`. `rewriteJcoAsyncResultLifting` (which exists precisely for this, and is
idempotent) flips exactly one of the seven to `false`. Re-running it fixed the lift.

## 4. The generated bridge did not unwrap jco's `option` representation

WIT declares `lifecycle-receipt: option<receipt>`. jco's flat-lift path yields `{ tag: "none" }` /
`{ tag: "some", val }`, but `pluginComponentBridgeSource`'s `lifecycleReceipt`/`uiPatchReceipt` readers
expected the bare value or `undefined`, so a legitimately absent receipt raised
`actor-lifecycle.receipt-required`. Added `unwrapOption` to the generator and applied it to
`lifecycleReceipt`, `uiPatchReceipt` and `nextWake`.

## 5. `patchAckEvents` dropped the UI-patch receipt

`patchAckEvents` emitted `{ surface, revision }` with no `receipt`, so the bridge's patch-ack branch
dereferenced `payload.receipt.lifetime` on `undefined`. The ack is only meaningful for the turn whose
`uiPatchReceipt` produced the patches, so the function now takes that turn and decodes its receipt.

## 6. `resolveActionArgDef` assumed every string arg carries `options`

The Rust wire type declares `options` as `#[serde(default, skip_serializing_if = "Vec::is_empty")]`, so
a free-text string argument legitimately serialises with no `options` field at all — 91 of them in the
puzzle descriptor. `ShellHelpers` called `def.schema.options.map(...)` unconditionally and took the whole
shell render down. Reads through `actionArgStringOptions` now.

## 7. Two puzzle-2D-specific id/classification defects

- `🌉️wasm/🟦️component.ts` still registered the board session factory under `s.puzzle2d@1/*#editor` /
  `…#viewer`, so `Board2dHost` threw *"The current app has no registered board session factory."*
- All 38 `action_interactive_job` rows in the 2D editor were `BatchOnlyPendingRewrite`, and **only
  `Migrated` is UI-dispatchable**. Every interactive action — `setActiveExample`, the whole `brush*`
  family, `setFillCount`, `deleteSelection`, `setCamera` — was rejected at dispatch with
  `interactive-job.not-ui-safe`. 2D was simply never migrated: 5D already marks the same class of
  interactive editor actions (`deleteSelection`, `duplicateSelection`, `setSelectionFlag`, the pointer
  actions) as `Migrated`. All 38 flipped to `Migrated`.

The classification is compiled into the guest, and `🔣️descriptor.json` is extracted by running the
transpiled component under node — so this one needs a real wasm rebuild to take effect.

## Tooling note

`grep` silently returns no matches on this repo's emoji-named sources (it treats them as binary).
Every content search in this investigation had to go through `python3`. `grep -c` reporting nothing
where the string is present is the tell.

## 8. The wasm-pkg dev stub swallowed a pkg that was present

`playgroundFlowWasmDevStubPlugin` substitutes a `wasmMissing` stub for any `/pkg/` import it cannot
find on disk. When the **browser requests the module directly**, Vite hands `resolveId` its own
`/@fs/<absolute path>` URL rather than the specifier the importer wrote — and
`resolve(repoRoot, "/@fs/…")` is that same non-existent path, so the check failed and the stub won even
though `pkg/semio_puzzle.js` was sitting right there.

Symptom: `BoardSession.prototype.free` was `undefined` in the browser (the stub's `BoardSession` only
has `lodScaleJson`), so `Board2dHost` died with `session?.free is not a function` and never painted.
Fixed by stripping the `/@fs` prefix before the existence checks. With the real 157 KB wasm-bindgen
module served, all three board panes (Overview / Detail / Selection) construct their WebGPU sessions
and paint.

## Verified in the browser so far

- Shell chrome, example selector, panels, and the three board panes render.
- Board WebGPU sessions construct and paint their grid.
- Remaining: every editor action is still refused with
  `interactive-job.not-ui-safe … BatchOnlyPendingRewrite` until the guest is rebuilt with the
  `Migrated` classifications from §7.

## 9. Puzzle 2D was never migrated to the typed tool-job protocol — the real remaining work

With the boot chain fixed, the app renders and every editor action reaches dispatch. There the last
gate closes: only `InteractiveJobClassification::Migrated` is UI-dispatchable, **and** a migrated verb
must resolve an exact app-owned proof through `qualified_tool_proof` — otherwise
`interactive-job.missing-factory`: *"typed command 'X' has no exact controller/owner/factory/tool
/schema proof"*.

The 3D and 5D editors each register an app tool factory
(`Puzzle3d/5dRetainedCommandJobFactory`), declare `PUZZLE3D/5D_RETAINED_TOOL_IDS`, and implement
`build_tool_job` mapping each tool id to a `PuzzleCommandWork`. **2D had none of it**:
`PUZZLE2D_RETAINED_TOOL_IDS` was `&[]` and neither `register_tool_job_factories` nor `build_tool_job`
existed — which is exactly what the blanket `BatchOnlyPendingRewrite` marker was recording.

So flipping all 38 rows to `Migrated` was necessary but not sufficient, and on its own it only trades
`interactive-job.not-ui-safe` for `interactive-job.missing-factory`.

### What this ticket implemented

2D already carried two finished `PuzzleCommandWork` implementations (`Puzzle2dActiveExampleWork`,
`Puzzle2dForceLayoutWork`) plus a `puzzle2d_retained_reduce`/`_extent` pair for `addNode` — all three
written and then never wired. Now added, mirroring 3D:

- `PUZZLE2D_RETAINED_TOOL_IDS = ["setActiveExample", "forceLayout", "addNode"]`
- `Puzzle2dRetainedCommandJobFactory` (`ToolJobFactory` + `ArtifactOwnedToolJobFactory`, with
  publication contracts on the `Artifact` lane)
- `bounded_first_step_tool_proofs!`, `register_tool_job_factories`, `build_tool_job` on
  `impl ArtifactEditor for Puzzle2dPlayApp`
- Classifications corrected to the honest state: `Migrated` for exactly those three, the other 35 back
  to `BatchOnlyPendingRewrite`.

### What remains

The other 35 interactive 2D actions — the whole `brush*` family, `setFillCount`, `applyBoardEvents`,
`deleteSelection`, `setCamera`, the `engagement*` and `setGrid*` group — still need a
`PuzzleCommandWork` each (or a shared scalar-config work like `Puzzle3dScalarConfigWork`) before they
can be marked `Migrated`. That is the substance of finishing puzzle 2D end to end, and every
verification round costs a full guest rebuild (~2 h: `semio_s_plugin_stdio` alone is ~65 min).

Note also that the currently published guest is a **debug** profile (97 MB core wasm). Its maintenance
step blows the 8 ms `INTERACTIVE_STEP_CEILING_US` and marks the instance
`RUNTIME_MAINTENANCE_FAULT` → *"runtime live cleanup faulted for instance 1"*. Rebuild with
`SEMIO_PLUGIN_PROFILE=wasm-release` (20 MB) for a representative run.

## 10. Brush and fill hinge on exactly one action: `applyBoardEvents`

Worth recording because it collapses the remaining work from "35 actions" to one.

Brush interaction is **client-side**: the wasm-bindgen `BoardSession` owns `brushOpenSlot`,
`brushCommitSlot`, `brushCycleCandidate`, `brushSetCandidateIndex`, and paints candidates and previews
itself. `Board2dHost/🟦️component.tsx:447` then commits the session's event queue to the plugin with a
single `dispatch("applyBoardEvents", { eventsJson })`, and `brushPlace` is one of the
`PUZZLE2D_FLUSH_NOW_EVENT_NAMES` (line 88) that forces that flush.

So the plugin-side `brush*` actions are not on the browser's interaction path at all — only
`applyBoardEvents` is. Migrating that one verb is what makes brush and fill commit end to end.

### Scale of that one migration

5D's equivalent, `Puzzle5dBoardEventsWork`, is a bounded incremental state machine: it streams the
events JSON byte-by-byte with its own depth/string/escape tracking, carries ~25 fields of cursor state,
and folds drag-moves, edges, fasteners, brush placement and camera into mutations — all inside the
7.5 ms / 4096-item `puzzle_command_contract()`. 2D's `apply_board_events::apply_board_events(ctx, args)`
today runs inside `handle`'s one-shot pipeline (build scene → sync host → act → `apply_host_events` →
`puzzle2d_document_delta_operations`) and cannot be dropped into a resumable work as-is.

### Repo-wide context

No puzzle editor has finished this migration: 5D is 9 migrated / 41 batch-only, 3D lists 4 retained
tool ids, and 2D is now 3 / 35. Repo-wide the split is 366 `Migrated` vs 444 `BatchOnlyPendingRewrite`.
Finishing 2D's brush/fill is a slice of that standing program, not a defect introduced here.

### `applyBoardEvents` migrated (written, not yet compiled)

Rather than port 5D's ~400-line streaming state machine, 2D's batch is small by construction — the
browser flushes a handful of events per interaction — so `applyBoardEvents` migrates as a bounded
one-shot `BoundedFirstStepCommandWork`:

- `puzzle2d_board_events_extent` parses `eventsJson`, refuses a batch over
  `PUZZLE2D_BOARD_EVENT_BATCH_LIMIT` (256) rather than truncating it, and reports the real event count.
- `puzzle2d_board_events_reduce` reruns exactly `handle`'s pipeline — `scene_for`, a fresh
  `BoardHost`, `sync_host_runtime_state`, `apply_board_events`, `apply_host_events`,
  `puzzle2d_document_delta_operations` — and emits the same artifact/config mutations and `ui_scope`.
  The one difference is `operation: None`: a retained work never receives the `ArtifactView` that
  `handle` reads `operation_optional()` from.

`PUZZLE2D_RETAINED_TOOL_IDS` is now
`["setActiveExample", "forceLayout", "addNode", "applyBoardEvents"]` (4 `Migrated`, 34 still
`BatchOnlyPendingRewrite`).

**Not verified.** `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` has not been green once
since 14:55 — every failure is in framework crates another session is editing live (store schema,
replication manifest, wgpu ui glue, store component, mesh-engine), none in a file this ticket touched.
Compile, then rebuild at `SEMIO_PLUGIN_PROFILE=wasm-release`, then drive brush/fill in the browser.

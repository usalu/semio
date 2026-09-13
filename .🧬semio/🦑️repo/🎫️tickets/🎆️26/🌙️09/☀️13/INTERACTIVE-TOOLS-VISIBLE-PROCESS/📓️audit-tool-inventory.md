# Audit — Tool Inventory: Visible Process & Cancel

Read-only survey for ticket `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS`. Goal being measured against: *"All
tools must be interactive and the user must see how the algorithms think by seeing the process. Every tool can
show progress (steps, percentage, status)."*

Method: grepped `ToolDefinition::new(`, `🛠️tools/*`, `🪛️utilities/*`, `🎮️commands/*` under every `s` plugin's
`editor`; for each hit, read the command/tool file and its backing session/job type to see whether it (a) has a
`Work`/`InteractiveJob` type that steps with a checkpoint/progress readout, (b) is a hidden-precompute-then-reveal
lane, or (c) is a synchronous one-shot reducer. Depth is full for puzzle (2d/5d/3d) and procedural
(generation2d/3d, assembly WFC) per the ticket's explicit ask; the other nine plugins (energy, fem, remodel,
lowpoly, raster, animate, vcs, sourcing, process) are surveyed at the depth needed to classify their named
algorithmic tools, not exhaustively file-by-file.

## 0. Finding: `🛠️tools/` as a directory concept barely exists

`grep -rl "ToolDefinition::new("` across all of `✏️s/🔌️plugins` returns exactly **two** files:

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`

Every other plugin exposes its algorithmic surfaces as `🪛️utilities/*` (mode-level picks, e.g. brush/volume-brush/
world-relocate) or bare `🎮️commands/*` (menu/keyboard/slider-driven verbs), not as a first-class "tool" struct.
So this audit treats "tool" per the ticket's own definition — *edit-mode tools, window utilities, commands that
run algorithms* — not literally `ToolDefinition::new(...)` call sites.

## 1. Reusable class-A patterns already in the codebase

Four places already do what the ticket wants. These are the templates to copy, not reinvent:

1. **Energy simulation** — `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs`.
   Four quality tiers, each rendered with stage name + `timestep/total (NN.N%)` + warmup hour + the actual
   intermediate result (`facility_electricity_kwh`) the tier already published — "the highest tier that actually
   published" is shown live, not just at the end. `aria-live=polite role=status busy=…`, full keyboard contract
   (`mod+enter` start, `mod+period` cancel, `mod+shift+enter` adopt) rendered as a help tree, not just bound.
   Backing session: `InteractiveJob` in `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs`
   (`EnergySimulationProjection`, `EnergySimulationStatus`, per-tier `step_one`), spawned via
   `Effect::SpawnJob { kind: ENERGY_SIMULATION_JOB_KIND, placement: JobPlacement::Isolated }`, cancelled by
   `cancel-energy-simulation` with `(job, operation, generation)` identity args so a stale cancel is a no-op.
2. **Remodel reconstruction** — `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs`
   renders `"Reconstruction: <Stage> (NN%) [- Error: …]"` plus an idle/running status line, sourced from
   `job.stage` / `job.progress_0_1` / `job.error` on the persisted `ReconstructionJob`. Driven by a
   generation-checked bounded continuation: `🏗️run-reconstruction/🦀️.rs` (`RECONSTRUCTION_STEP_BUDGET`,
   `MAX_RECONSTRUCTION_TICKS`, ten named `RequestedStage` variants from feature-extraction through texturing) plus
   `⏩️advance-reconstruction/🦀️.rs` (one generation-checked tick), `🛑️cancel-reconstruction`, `🔁️retry-stage`.
3. **Procedural node-graph preview eval** — shared `🧵️preview-eval` module
   (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs`), bound
   identically by editor and viewer (`🎮️commands/⏱️flow-eval-tick`, `🎮️commands/🛑️cancel-preview-eval` in both).
   Publishes a `phase`/`progress` status object every tick (`preview_window_status_json`, read in
   `🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs`), self-re-arms via `Effect` until done, and is cancellable — the
   comment there literally says this was retrofitted because generate-mode preview had "no status host at all…
   189s of `[data-status-json]`-less generate mode."
4. **Assembly WFC engine (procedural, wave-function-collapse inference)** —
   `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/💼️job/🦀️.rs`.
   This is the best mechanical template for puzzle-3d-fill's redesign: a named `WfcStage` enum
   (`InitializeDomains → FindMinimumEntropySlot → ChooseCandidate → PropagateCompatibilityEdge →
   DetectContradiction → BacktrackTrailEntry`), an `emit_preview` step that publishes up to
   `PREVIEW_ITEM_LIMIT=256` items at least every `PREVIEW_UNIT_INTERVAL=16` units or
   `PREVIEW_TIME_INTERVAL_MS=16`ms (`StepOutcome::PreviewReady`), plus `StepOutcome::{Yield, CheckpointReady,
   Cancelled, Fault, Complete}` and a real `progress() -> (usize, usize)`. This is exactly "stream every
   candidate the solver tries, with a bounded per-tick cadence" — the shape ticket 26/09/13 wants puzzle 3d fill
   to have.

## 2. Puzzle plugin (2d / 5d / 3d) — the ticket's named targets

| Tool / command | Files | Class | What the user sees today | What's needed |
|---|---|---|---|---|
| **2d fill** (`🛠️tools/🪣️fill`, `🎮️commands/🏁️fill-session-begin`, `👣️fill-session-step`, `🧹️fill-session-clear`, `🧮️set-fill-count`) | `◻️2d/…/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`; `◻️2d/…/🎚️config/🦀️.rs` (`Puzzle2dFillRuntime`, `Puzzle2dFillLifecycle`) | **B→A, best of the puzzle family** | Slider label already reads `"Progress: accepted/count"` or `"Result: N"`, driven by `fill_job_lifecycle` (`Capturing/Queued/Running/CheckpointReady/Applying/AwaitingAdoption/Closing/Completed/Faulted/Cancelled`); a Cancel toggle appears while running, a Retry toggle appears on Faulted/Cancelled. **Missing**: no per-object visual (no danger-colored/highlighted mesh per candidate) — only the aggregate counter moves. | Add a per-candidate visual event (object id + accepted/collision) alongside the existing counter; the lifecycle/job plumbing is already there, this is a render-layer addition, not a new job. |
| **3d fill** (`🛠️tools/🪣️fill`, `⏳️precompute/🪣️fill`, `🎮️commands/🪣️fill-build-tick`, `🌀️set-vortex-show`, `🔓️open-vortex-suggestions`/`🔒️close-vortex-suggestions`) | `🧊️3d/…/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`; `🧊️3d/…/⏳️precompute/🪣️fill/🦀️.rs` (`FillBuilder`, `FillPreviewJsonStep::Pending{progress, checkpoint}`); `🎮️commands/🪣️fill-build-tick/🦀️.rs` | **C — hidden precompute + reveal (THE ticket's named offender)** | The tool file's own doc comment: *"the slider's `reveal` key lets the viewport show/hide already-planned pieces client-side per drag value with zero WASM round trips."* A background `Effect::SpawnJob { kind: FILL_JOB_KIND, placement: JobPlacement::Isolated }` runs `FillBuilder` to completion silently; the slider only exposes a `ready`/`loading` extent and a cancel toggle with `"count / max_count planned"` text. Nothing shows *which* object was tried, whether it collided, or where. | This is the ticket's primary target — see §5, item 1. The job already steps (`FillPreviewJsonStep::Pending{progress,checkpoint}`, `StepOutcome::CheckpointReady`) and already tracks `placed_object.reveal_index` per accepted object — the missing piece is emitting a *rejected*-candidate event too (collision), not just accepted ones, and wiring both into a live per-object mesh state (danger-colored vs. highlighted) instead of a slider-driven reveal cutoff. |
| **3d brush** (`🪟️windows/🧊️main/🪛️utilities/🖌️brush`, `🎮️commands/⏱️suggestions-tick`, `🔓️open-vortex-suggestions`) | `🧊️main/🪛️utilities/🖌️brush/🦀️.rs`; `🎮️commands/⏱️suggestions-tick/🦀️.rs`; `🎮️commands/🔓️open-vortex-suggestions/🦀️.rs` | **C — hidden precompute** | Doc comment: *"hover a vortex, cycle the collision-free candidates the ENGINE PRECOMPUTED for it."* `suggestions_tick` runs up to 8 synchronous `refresh_brush_candidates` slices per 120ms host tick (bounded loop, inline in the command handler, not a spawned job) until candidates exist or give up; `open_vortex_suggestions` does the identical up-to-8-slice loop synchronously on open. The UI only ever sees the *outcome* (a `Select` list of ready candidate labels) — never the trial/rejection process. `[DEBUG] eprintln!` lines are the only trace of the search (`puzzle3d.brushPreview.cache … free=… pending=…`), meaning even a developer can't see it without console access, let alone the end user. | Publish a live "trying candidate K of N…" status while `unknown_pending` is true, and once `refresh_brush_candidates` starts finding free/blocked slots, stream them instead of gating the whole `Select` on completion. Also: **remove the `[DEBUG]` eprintln!s once the real progress surface exists** — they're temporary. |
| **3d volume-brush** (`🪟️windows/🧊️main/🪛️utilities/🧊️volume-brush`) | `🧊️main/🪛️utilities/🧊️volume-brush/🦀️.rs` | **E — pure UI** | Three voxel-dimension steppers (width/depth/height, 1–64). No algorithm runs here; painting a volume is an O(1) grid write. | None. |
| **3d transform / world-relocate** (`🪟️windows/🧊️main/🪛️utilities/🔄️transform`, `🚚️world-relocate`; `🎮️commands/🌍️world-relocate`, `🚀️translate-selection`) | `🎮️commands/🌍️world-relocate/🦀️.rs` (68 lines) | **D — synchronous one-shot, cheap** | `world_relocate` does one linear scan of `ctx.scene.fixture.objects` to find proximity attractions after a drop; no progress needed at today's object counts, but it's an unbounded `for other in &ctx.scene.fixture.objects` scan with no step budget — on a catalogued document (hundreds/thousands of objects, per `component-release`/`taxonomy` audits elsewhere in this repo) this could start costing visible frame time with zero cancel path. | Estimate cost against realistic document sizes before deciding; if it's provably sub-frame at max document size, leave as D. If not, bound it into the shared `PuzzleCommandWork` stepping trait already used elsewhere (`🎮️commands/🧵️retained/🦀️.rs`). |
| **5d fill** (`🎭️modes/✏️edit/☑️options/🪣️fill`, `🪟️windows/◻️2d/🪛️utilities/🪣️fill`, `🧠️precompute`) | `☑️options/🪣️fill/🦀️.rs`; `🪛️utilities/🪣️fill/🦀️.rs`; `🧠️precompute/🦀️.rs` (`Puzzle5dPrecomputeSession` wraps `Puzzle3dPrecomputeSession`) | **C — hidden precompute, WORSE than 3d's own fill UI** | The 5d fill-count slider (`☑️options/🪣️fill/🦀️.rs::fill_count_measure`) sets `ready: None, loading: None, waiting: None` — i.e. it doesn't even surface the `ready`/`loading` extent that 3d's own slider has, despite delegating to the exact same `Puzzle3dPrecomputeSession` engine underneath. | Wire the same `progress`/`ready`/`cancel` measures the 3d tool already computes (`precompute.fill_progress_summary()`) into 5d's slider — this is a UI-only gap, the session object already carries the data (`fill_progress_summary`, `cancel_fill_job_for` exist on the 3d session that 5d wraps). |
| **5d retarget/patch-grip/proximity-connect etc.** (`🎮️commands/*`) | 25+ files under `🖐️5d/…/🎮️commands/` | **D/E — small synchronous reducers** | Each is a single-shot mutation on selection/fasteners/kind weights; not algorithmic search, no progress applicable. | None flagged. |

## 3. Procedural (generation2d / generation3d / assembly)

| Tool / command | Files | Class | What the user sees today | What's needed |
|---|---|---|---|---|
| **Node-graph eval / flow-eval-tick** (generate + edit modes, both 2d and 3d) | `🧊️generation3d/…/🎮️commands/⏱️flow-eval-tick/🦀️.rs`; shared `🧊️generation3d/…/🧵️preview-eval/🦀️.rs`; `🌀️generation2d` mirrors the same shape under its own `🪟️windows/👁️preview`/`🕸️flow` | **A** | Phase/progress status object every tick, cancellable (`cancel-preview-eval` in both editor and viewer), self-re-arming. Already the pattern other tools should copy (see §1.3). | None structural; verify the generate-mode preview window (`🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs`) surfaces the SAME status host in every entry state — its own doc comment records a prior regression (189s with `hosts=[]`) so this is a known-fragile wire, worth a regression test if none exists yet. |
| **`node-graph-edit` command** (move/connect/disconnect/setFixture/deleteSelection) | `🧊️generation3d/…/🎮️commands/✏️node-graph-edit/🦀️.rs` | **D — synchronous, cheap** | Applies host-graph sub-operations (`host.move_widget`, `connect_ports`, `disconnect`, `replace_fixture`, `remove_widget`) directly, no job. These are direct-manipulation edits (drag/connect/delete), not "algorithms" in the ticket's sense — the actual algorithmic work is the eval tick that follows, already class A. | None. |
| **generations window** (`🪟️windows/🗂️generations`) | `🧊️generation3d/…/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs` (also `🌀️generation2d`'s sibling) | Not independently audited for progress fields this pass — flagged for the next audit pass since it was modified in the working tree at session start (see git status) alongside `👁️preview`/`📝️form`; likely consumes the same `preview-eval` status host. | — | Recommend the fill-in agent confirm it reads the same `phase`/`progress` status rather than re-deriving its own. |
| **form window** (`🪟️windows/📝️form`) | `🧊️generation3d/…/🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs` | Not independently audited this pass (no `progress`/`loading`/`status`/`cancel` grep hits found) — likely **E**, pure parameter form. | — | Confirm no long-running validation hides behind form submit. |
| **Assembly WFC solve** (procedural's wave-function-collapse inference engine) | `🧩️assembly/…/🧬️schema/💡️inferences/🧩️wfc-engine/💼️job/🦀️.rs`, `🏗️model/🦀️.rs`, `🗺️topology/🦀️.rs` | **A — best mechanical template in the repo** | Full `StepOutcome::{Yield, PreviewReady, CheckpointReady, Cancelled, Fault, Complete}` state machine, staged (`WfcStage`), previews streamed every 16 units/16ms, real `progress()`. Not itself flagged as a UI gap in this pass (its consuming window wasn't located under the `s`-plugin `editor` tree during this sweep — the engine may be consumed from a non-`s` host); worth a follow-up grep for `WfcPreview`/`WfcStage` consumers if the UI side isn't already wired to it. | If a consuming window exists but doesn't surface `WfcPreview`, that's a quick win — the engine already emits everything needed. |

## 4. Nine other plugins (energy, fem, remodel, lowpoly, raster, animate, vcs, sourcing, process)

| Plugin / tool | Files | Class | Notes |
|---|---|---|---|
| **Energy simulation** | `🔋️energy/…/🧵️simulation-session/🦀️.rs` + `…/⚡️simulation/🦀️.rs` window | **A** | Gold-standard exemplar, detailed in §1.1. Nothing to fix. |
| **FEM 2d/3d — mesh/graph domain solve** | `🏗️fem/…/◻️2d/…/✏️editor/🧵️session/🦀️.rs` | **A for the graph/mesh lane, C/D for the actual numeric solve** | The session is a real `InteractiveJob` with named progress tags (`fem2d.graph-admitted`, `fem2d.domain`, `fem2d.domain-complete`, `fem2d.mesh-preview`, `fem2d.mesh-complete`, then a `FemJobStage::Solve` stage) — so mesh-building already ticks with visible progress tags. **But** the Results window (`🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs`) calls `fem2d_solve_all(doc)` directly and synchronously to render results, bypassing the job/stage machinery entirely for the actual linear-solve step. That is a monolithic synchronous call sitting right next to a properly staged job — a prime restructuring target: route the results-window solve through the SAME `FemJobStage::Solve` stage the session already models instead of a direct function call. |
| **Remodel reconstruction pipeline** | `📸️remodel/…/🎮️commands/🏗️run-reconstruction`, `⏩️advance-reconstruction`, `🛑️cancel-reconstruction`, `🔁️retry-stage`; `📌️panels/🗿️artifact` | **A** | Second-best exemplar in the repo, detailed in §1.2. Ten named stages, generation-checked bounded continuation, textual `Stage (NN%)` readout with error surfacing. One soft gap: the panel's own comment admits `running` is derived from persisted stage only ("a synchronous run never leaves the document in a non-terminal stage… effectively always Idle once a run finishes") — meaning very fast runs never show as "running" even momentarily; low priority. |
| **Lowpoly mesh edit** (`extrude/inset/bevel/loopCut/subdivide/triangulate/mirror/decimate/flipFaces/merge/dissolve/snap/toggleSmooth`) | `💠️lowpoly/…/🎮️commands/🔷️mesh-edit/🦀️.rs` | **D — synchronous, shared reducer** | All twelve+ mesh operations, including `decimate`, run through one shared `mesh_edit(doc, config, ctx, closure)` helper that calls straight into `active_mesh_mut()` kernel methods and returns a `Patch` diff — single call, no job, no progress. For typical low-poly mesh sizes this is likely sub-frame; **decimate specifically is the one operation in this list whose cost scales with mesh complexity** and is worth a real timing measurement before ruling it "fine." |
| **Raster editor** | `🖨️raster/…/✏️editor/📌️panels/🎭️masks`, `🗿️artifact` | Not deeply audited — no `🎮️commands` files found under the expected edit-mode path (`✏️editor/🎭️modes/✏️edit/🎮️commands` is empty), meaning raster's edit surface is mutation-driven (mask/layer edits) rather than command/algorithm-driven at this layer. **E** for what was found. |
| **Animate (presentation) editor** | `🎞️animate/…/✏️editor/⚙️engine/🎥️video`, `🔤️text`, `🎬️scene`, `🎛️config` | Not deeply audited — same empty-`🎮️commands` pattern as raster; render/bake logic lives under `⚙️engine/` but no SpawnJob-based command surface was found in the edit-mode command directory. **Flagged, not classified** — if `⚙️engine/🎥️video` does real encoding work synchronously on a UI thread, that would be a D/C candidate; needs a follow-up read of `⚙️engine/🎥️video/🦀️.rs` specifically. |
| **VCS diff/merge** | `🌿️vcs/…/✏️editor/🎭️modes/✏️edit/🎮️commands` | **Directory is empty** — vcs's edit-mode algorithmic work (diff/merge) is declared under `🧬️schema/⚙️operations` and `🔺️diff`, i.e. as schema-level diff/inverse pairs, not editor commands. Likely **D/E** (git-like diffing at document scale is normally fast) but not confirmed by a timing measurement. |
| **Sourcing / curation** | `🪵️sourcing/…/✏️editor/🎭️modes/✏️edit/🎮️commands` | **Directory is empty**; only `🗿️set-artifact-json` exists at the editor root. Curation's actual crawl/fetch/import work (implied by the plugin name) was not located under the `s`-plugin editor tree in this pass — likely lives in an ingestion pipeline outside the scope grepped here. Flagged for a follow-up, not classified. |
| **Process3d** | `🏭️process/…/✏️editor/🎭️modes/✏️edit/🎮️commands` | **Directory is empty**, and `SpawnJob`/`InteractiveJob`/`Work` greps returned zero files anywhere in `🏭️process`. Whatever process-simulation/scheduling logic exists is either not yet built or lives entirely outside the `s`-plugin surface. Flagged for scoping, not classified. |

## 5. Top 10 priority list (after puzzle 3d fill)

Ordered by (ticket-declared priority × user-facing frequency × how cheap the fix is given existing plumbing).

1. **Puzzle 3d fill** (`⏳️precompute/🪣️fill/🦀️.rs`, `🎮️commands/🪣️fill-build-tick/🦀️.rs`, `🛠️tools/🪣️fill/🦀️.rs`) — *already in scope per the ticket description itself.* Job already steps and checkpoints (`FillPreviewJsonStep::Pending{progress,checkpoint}`); needs a rejected-candidate event alongside the existing accepted-object stream, and a render layer that shows danger-colored (collision) vs. highlighted (collision-free) mesh per attempt instead of a reveal-cutoff slider. **Work type exists — cheap.**
2. **Puzzle 3d brush / suggestions** (`🪟️windows/🧊️main/🪛️utilities/🖌️brush/🦀️.rs`, `🎮️commands/⏱️suggestions-tick/🦀️.rs`, `🔓️open-vortex-suggestions/🦀️.rs`) — same underlying precompute session as fill, same hidden-until-ready pattern, plus it's the one place doing synchronous multi-slice work *inline in the command handler* (up to 8 slices per call) rather than through a spawned job. **Work type exists (`Puzzle3dPrecomputeSession`) — needs the tick loop's intermediate `unknown_pending` state surfaced, cheap.**
3. **Puzzle 5d fill slider** (`🎭️modes/✏️edit/☑️options/🪣️fill/🦀️.rs`) — literally missing the `ready`/`loading`/cancel wiring that its own delegate (3d's `Puzzle3dPrecomputeSession`) already computes. **Pure UI wiring gap — cheapest fix on this list.**
4. **FEM results-window solve** (`🏗️fem/…/🪟️windows/📊️results/🦀️.rs` calling `fem2d_solve_all` directly) — a monolithic synchronous call sitting beside an already-staged `InteractiveJob` session in the same artifact. **Needs restructuring**: route through the existing `FemJobStage::Solve` stage instead of a bare function call.
5. **Lowpoly `decimate`** (`🎮️commands/🔷️mesh-edit/🦀️.rs`) — one of twelve reducers behind a shared synchronous `mesh_edit` helper; the one whose cost is not obviously O(1) in mesh size. **Needs a cost measurement first**, then either leave as D or extract it alone into a stepped `Work` type (the other eleven operations are fine as-is).
6. **Puzzle 3d world-relocate / attraction resolution** (`🎮️commands/🌍️world-relocate/🦀️.rs`) — unbounded linear scan over all document objects per drop, no step budget, no cancel. **Needs a cost measurement against catalogued-document object counts** (this repo has prior incidents of exactly this kind of unbounded-scan surprise at scale — see the `PuzzleCommandWork` step-budget infrastructure already built for other puzzle commands).
7. **Assembly WFC → its consuming UI** — confirm whether the `WfcPreview`/`WfcStage` stream the engine already emits is actually rendered anywhere; if a consumer exists but drops the preview events, wiring it up is nearly free (engine does all the hard work already). **Needs a locate-the-consumer pass**, not a rewrite.
8. **Procedural `generations` window** (`🧊️generation3d/…/🪟️windows/🗂️generations/🦀️.rs`, `🌀️generation2d`'s sibling) — currently modified in the working tree; confirm it reads the shared `preview-eval` phase/progress host rather than re-deriving its own status, given the documented prior regression (189s with no status host). **Needs a verification read, likely no code change.**
9. **Animate `⚙️engine/🎥️video`** — unclassified; if video encode/render runs synchronously off the render/bake path, it is a strong `D→C` candidate purely because of typical video-encode cost. **Needs a first read** before it can even be sized.
10. **Puzzle 3d volume-brush / transform utilities** — currently correctly class E (cheap, no algorithm), but worth a explicit regression guard (a test asserting they stay O(1)) so a future change to volume painting or gumball transforms doesn't quietly turn them into hidden-cost tools without anyone adding progress UI to match.

### Work-type-exists vs. needs-restructuring, at a glance

- **Cheap (Work/Job/session already steps — only the render/measure layer needs the missing event)**: puzzle 2d fill, puzzle 3d fill, puzzle 3d brush/suggestions, puzzle 5d fill, procedural node-graph eval (already class A), assembly WFC (already class A, if wired), energy simulation (already class A), remodel reconstruction (already class A).
- **Needs restructuring (currently one monolithic synchronous function, no stepping primitive at all)**: FEM results-window solve (`fem2d_solve_all`), lowpoly `decimate` (shares a synchronous reducer with 11 other ops — would need to be pulled out on its own), puzzle 3d world-relocate's attraction scan, animate's video engine (pending a read).

## 6. Answers to the ticket's explicit questions (§4/§5)

- **Puzzle 2d fill**: already shows an aggregate accepted/total counter with cancel/retry — best UX of the puzzle family, but still aggregate-only, no per-object trial visual.
- **Puzzle 5d fill**: shows nothing — slider has no `ready`/`loading`/cancel despite delegating to the same engine 3d already instruments.
- **Puzzle 3d brush, volume-brush, suggestions tick, vortex suggestions**: brush/suggestions are hidden-precompute-then-reveal (candidates appear only once fully resolved, with only `[DEBUG]` eprintln! traces visible to a developer); volume-brush is a pure O(1) UI utility needing nothing.
- **Transform / world-relocate**: synchronous, unbounded in the number of objects scanned, no progress infrastructure at all — currently "fine" only because documents are small in practice.
- **Procedural generation2d/3d node-graph eval (`flow-eval-tick`)**: already class A — phase/progress, cancel, shared editor+viewer. The best-instrumented tool in the entire audited surface besides energy/remodel/WFC.
- **Energy, fem, remodel, lowpoly, raster, animate, vcs, sourcing, process**: see §4 table — energy and remodel are exemplars; fem is half-exemplar/half-monolithic in the SAME artifact; lowpoly's decimate is the one synchronous reducer worth timing; raster/animate/vcs/sourcing/process either have no algorithmic command surface at the `s`-plugin editor layer in this sweep (raster, vcs, sourcing, process) or weren't reached this pass (animate's video engine) — flagged for a follow-up rather than guessed at.

## Files referenced (non-exhaustive index of what was read)

- `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs` — shared `PuzzleCommandWork`/`InteractiveJob` trait puzzle 2d/3d/5d commands ride on.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/{🎭️modes/✏️edit/🛠️tools/🪣️fill,🎮️commands/{🏁️fill-session-begin,👣️fill-session-step,🧹️fill-session-clear},🎚️config}/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/{🎭️modes/✏️edit/🛠️tools/🪣️fill,⏳️precompute/🪣️fill,🎮️commands/{🪣️fill-build-tick,⏱️suggestions-tick,🔓️open-vortex-suggestions,🔒️close-vortex-suggestions,🌍️world-relocate,🚀️translate-selection},🪟️windows/🧊️main/🪛️utilities/{🖌️brush,🧊️volume-brush,🚚️world-relocate}}/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/{🧠️precompute,🎭️modes/✏️edit/☑️options/🪣️fill,🎭️modes/✏️edit/🪟️windows/◻️2d/🪛️utilities/🪣️fill}/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/{🧵️preview-eval,✏️editor/🎮️commands/{⏱️flow-eval-tick,✏️node-graph-edit},✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview}/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/💼️job/🦀️.rs`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/{🧵️simulation-session,✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation}/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/{🧵️session,🎭️modes/✏️edit/🪟️windows/📊️results}/🦀️.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/{🎮️commands/{🏗️run-reconstruction,⏩️advance-reconstruction},📌️panels/🗿️artifact}/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔷️mesh-edit/🦀️.rs`

## Scope not covered this pass (flag, don't guess)

- Raster's `⚙️engine`-level pixel/render pipeline internals, animate's `⚙️engine/🎥️video` encode path, vcs's `⚙️operations`/`🔺️diff` cost at large-repo scale, sourcing's actual ingestion pipeline, and process3d's scheduling logic — none of these had an algorithmic command surface at the `s`-plugin editor-command layer this sweep grepped; they need a targeted follow-up (grep their `⚙️engine`/`🧬️schema` layers directly) rather than being classified from absence of evidence.
- `➗️mathematical`, `🌊️flow`, `🌍️gis`, `🏛️architect`, `📐️cad`, `📕️norm` (32 SpawnJob-bearing files — the largest count after puzzle/procedural, unexamined here), `🔱️trinity`, `🖍️draw`, `🗒️note`, `🧱️block`, `🪐️space` were not opened at all — the SpawnJob-file-count table in §method is a starting point for whoever picks these up next, especially `📕️norm` given its outsized job-file count.

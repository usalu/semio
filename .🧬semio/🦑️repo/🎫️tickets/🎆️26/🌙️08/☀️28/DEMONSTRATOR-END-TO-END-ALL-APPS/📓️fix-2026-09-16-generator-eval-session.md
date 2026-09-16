# Generator (`generation3d`) preview evaluation — trace, verdict, new laws (2026-09-16)

Scope: the demonstrator pane `generator` → `s.procedural.generation3d@1/*#editor`, crate
`semio-s-artifact-procedural-generation3d`
(`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d`).

Root paths below are relative to `/Users/ueli/Documents/semio`; `EDIT/` abbreviates
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any`.

## 1. Verdict first

**The gap `📓️app-generator.md` §3 records is CLOSED in the current code, and the closure is now
proven natively.** There is no fresh-per-render `FlowEvalSession` anywhere in the live path, the
evaluation already is a progress/cancel-capable `previewEval` tool run, and the retained session
already lives in app instance state. Two peer tickets moved this ground after the audit was written
(`🎆️26/🌙️09/☀️14/PROCEDURAL-3D-FLOW-WINDOW-ARTIFACT-TREE`,
`🎆️26/🌙️09/☀️15/PROCEDURAL-3D-HISTORY-CAMERA-SPURIOUS`), plus the whole
`26/09/09/PROCEDURAL-3D-END-TO-END` chain the source comments cite.

So this packet made **no production change**: it added the two laws that state the boot contract
(§4) and corrected the two stale "KNOWN GAP" claims that were still steering readers (§5). What is
left is browser verification on :6029 (§7).

## 2. The evaluation chain, file:line

### 2.1 The retained session

- `EDIT/✏️editor/🦀️.rs:112-241` — `Generation3dInstanceOperationOwner`, the ONE
  `eval_session: Option<FlowEvalSession>` (`:120`) per app instance, built once at `:133`.
  - `:136` `with_session`, `:144` `with_session_waking` (a fold that changed the session wakes the
    live run job), `:152-176` the three `owe_attached_previews*` bridges, `:179`
    `preview_eval_parts` (the `PreviewEvalRunOwner` impl the run job reaches the session through),
    `:190-221` the explicit close ladder (`FlowEvalSession` rejects a live drop).
  - `:225-240` `with_scratch_session` is the ownerless FALLBACK only — used by the marks-free
    `render`/`context_menu` delegates and the pure-reduce path, never by a live window.

### 2.2 Who starts the run

- `EDIT/✏️editor/🦀️.rs:2178-2189` `pending_effects` → `preview_eval_run_effects`.
- `EDIT/🧵️preview-eval/⏯️tool-run/🦀️.rs:350-418` `preview_eval_run_effects` — retains the per-window
  latches against the attached roster, records the debt a history verb created
  (`applied_document_edits_digest`, `EDIT/🧵️preview-eval/🦀️.rs:196-205`), and asks for exactly one
  `toolRunStart` / `toolRunFinalize` under a one-request latch.
- A gesture that moved the document carries the start on its OWN emit instead of waiting for a poll:
  `EDIT/🧵️preview-eval/🦀️.rs:243-275` (`owe_attached_previews_for_mutations` /
  `owe_attached_previews_carrying`).
- Boot works because a window that has never ticked OWES its first tick:
  `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:3499` `window_tick_owed` answers `true`
  for `None`. Latch vocabulary: `:3481` arm, `:3516` begin, `:3531` outcome, `:3549` retain,
  `:3605` armed.
- Gate: `EDIT/🧵️preview-eval/🦀️.rs:165` `may_rearm` — a graph whose operator kinds no contributed
  extension serves is NOT startable, and only `setContributions` moves it. That route owes the
  previews itself at `EDIT/✏️editor/🦀️.rs:1275-1288`.

### 2.3 Who drives it

- `EDIT/✏️editor/🦀️.rs:2193-2199` `build_tool_run_job` → `PreviewEvalRunJob`
  (`EDIT/🧵️preview-eval/⏯️tool-run/🦀️.rs:436+`). One fuel unit = one `flowEvalTick` hop handed to the
  host through the `ToolRunJobPort`; scheduling law at `⏯️tool-run/🦀️.rs:251-260`.
- Hop: `EDIT/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs:57-92` `evaluate` — advances the retained
  session, parks `evaluate`/`tessellate` extension invocations, returns a `FlowEvalPublication`.
  `:106-120` `continue_inline` runs the next wave inside a just-folded answer's own turn when the
  turn budget allows.
- Answers fold back through the retained command route at `EDIT/✏️editor/🦀️.rs:683-701`
  (`FlowEvalTick` / `FlowEvalResolve` / `FlowTessellateResolve`, all under `with_session_waking`), and
  a `Changed` publication is written into the addressed window's retained transient by
  `generation3d_addressed_preview_eval` (`:278-291`).
- Cancel/abort: `EDIT/✏️editor/🎮️commands/🔓️flow-eval-release/🦀️.rs:16-18`.

### 2.4 Who paints it

- `EDIT/✏️editor/🦀️.rs:2210-2229` `render_with_request_context` — reads `preview_eval_text` off the
  preview window's transient and renders under the RETAINED session. This is the path
  `PluginApp::render` takes for every live window.
- `EDIT/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:79-103` — `preview_payload(eval_json, …,
  Some(session), marks)` → `meshesJson` / `instancesJson` / fit / status into the `World3d` scene.
- `EDIT/✏️editor/🦀️.rs:2882-2890` `preview_payload` still short-circuits on an EMPTY evaluation — that
  is the correct law (a render must never evaluate), not the defect.
- `EDIT/✏️editor/🦀️.rs:2203-2205` is the marks-free delegate the framework still offers; it uses a
  scratch session and `None` eval text. **This is the function the audit measured.** It is not on any
  live window path.

### 2.5 The demonstrator's boot gesture

- `♻️mit-bestand/🧺️demonstrator/🪧️brand.ts:506` — `ENTWERFEN_MIT_BESTAND_GENERATOR_BRAND.defaults =
  { exampleId: "hexagonal-mushroom-column" }` (so NOT `sphere-cut-with-torus`, whose preview freeze is
  a separate known hang).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:9891-9904` —
  the once-per-instance boot effect dispatches `setActiveExample` with that id.
- `EDIT/✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs:51-71` — replaces the whole document; the
  mutations it emits are what makes the retained-command route owe every attached preview and carry the
  run start.
- Registry row: `🧰️framework/…/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts:55` (variant
  `generator`, plugin `demonstrator`, app `s.procedural.generation3d@1/*#editor`, react port 6027).
  `✏️s/🔌️plugins/🎪️demonstrator/🪪️manifest/🎪️demonstrator/🦀️.rs:52-57` depends on `procedural`, which is
  the plugin that declares the `brep` flow extension — so the pane does get contributable operators.

## 3. Why the audit read it as empty

`📓️app-generator.md` §3 measured `Procedural3dPlayApp::render` (today `EDIT/✏️editor/🦀️.rs:2203`), the
owner-free delegate, and generalised it to the live window. The live window has gone through
`render_with_request_context` + the retained owner since the `PROCEDURAL-3D-END-TO-END` wave. The
audit's other §3 claim — "each `flowEvalTick` dispatch builds its own throwaway session" — is also
gone: `EDIT/✏️editor/🦀️.rs:691-694` folds every hop into the retained one.

The one structural claim of the audit that still holds is §6 (selection-dependent UI: `render` and
`context_menu` receive no `InteractionView` at the FRAMEWORK entry points) — but
`render_with_request_context` does receive one and `PreviewInteractionMarks::from_interaction`
(`EDIT/✏️editor/🦀️.rs:2223`) threads it, so the live preview does paint hover/selection. Out of scope
here.

## 4. What was added (test-only)

`EDIT/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs`

- `:6` — `use semio_framework_plugin::PluginApp;` (the trait `handle_action` hangs off).
- `:55-84` (law at `:68`) — **`the_demonstrator_boot_example_renders_a_non_empty_preview_scene`**. Replays the
  generator pane's boot exactly: shell-dispatched `setActiveExample("hexagonal-mushroom-column")`
  under the FLOW window's `ViewModel`, settle, then `context::drive_preview_run` (the real served loop
  — `pending_effects`, framework run actions, every `Effect::DispatchAction` redispatched as the typed
  command, every extension invocation answered by the in-process brep kernel). Asserts the run
  actually evaluated `procedural-preview`, then decodes the RENDERED preview body and asserts
  `meshesJson != "[]"` and `instancesJson != "[]"`.
- `:86-105` (law at `:92`) — **`the_preview_render_reads_the_retained_evaluation_instead_of_recomputing_it`**. States
  the other half of the requirement: a render taken BEFORE the run's first hop paints `"[]"` (it does
  not evaluate on its own), and two renders after the run are byte-identical in both lanes. A surface
  that re-evaluated per render would rebuild geometry a user had just cancelled.

Both reuse the file's existing `preview_scene` decoder (`decode_fixture_scene_with_lanes`), so they
grade the same paged-lane payload a React/wgpu host reads.

## 5. Stale-claim corrections (non-code)

`♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts`

- `:14-16` header — "Generator's edit-mode preview is still expected to fail" → used to be expected to
  fail; the guest half is fixed and proven natively; the assertion now grades the SERVED path only.
- `:157` the `procedural-preview` `note` — rewritten the same way, naming the new law.

`expectContent: true` is unchanged on that row: the assertion itself was always correct.

## 6. Commands and outcomes

All prefixed `DEVELOPER_DIR=/Library/Developer/CommandLineTools RUSTFLAGS=-Awarnings
CARGO_TERM_QUIET=true CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_BUILD_JOBS=4`, run from the repo root.

| command | outcome |
|---|---|
| `cargo check --target wasm32-wasip2 -p semio-s-artifact-procedural-generation3d --features component-app-assembly` | clean, exit 0 (log: `🗑️generated/generator-eval-wasm-check.txt`) |
| `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- modes::edit::windows::preview::tests --test-threads=1` | **4 passed, 0 failed**, 478 filtered (log: `🗑️generated/generator-eval-test.txt`) |
| same, filtered to the boot law with `--nocapture` | `[DEBUG] demonstrator boot run: hops=1 windows=["procedural-preview"] answered=2 state=Some("finalized")` (log: `🗑️generated/generator-eval-boot-nocapture.txt`) |

Counts: the edit-preview window module held **2** laws before and holds **4** after; the lib's test
total moved 480 → 482. No pre-existing failure was observed in the module. The first attempt without
`--features component-app-assembly` fails to compile (`crate::editor` / `crate::viewer` are gated) —
that feature is mandatory for this crate, same as the puzzle-3d crate.

## 7. What remains — browser verification (:6029)

The native proof covers the guest: gesture → run → hops → extension answers → retained session →
published transient → rendered scene with meshes and instances. It cannot cover the host half:

1. **`setContributions` actually lands in the pane.** `may_rearm` refuses to start the run on a graph
   whose `brep.*` / `math.*` kinds no contributed extension serves. On :6029 confirm the console shows
   a `contributions …` line for the `generator` shell BEFORE judging the preview, and that the preview
   status pill is not `flow.extension-not-contributed`.
2. **`pending_effects` is actually polled.** The host polls once per `refreshUi` and refreshes on
   ACTIVITY (see the comment block at `EDIT/🧵️preview-eval/🦀️.rs:234-242`). A boot that goes quiet
   before the first poll never reads the debt. Look for one `toolRunStart` followed by `flowEvalTick`
   hops for `procedural-preview`.
3. **The pane's `ViewModel` carries `window_instances`.** `attached_preview_windows`
   (`EDIT/🧵️preview-eval/🦀️.rs:288-290`) returns empty without them, and an empty roster arms nothing
   at all — this is the memory note "Action View State Needs Window Instances" and it is the single
   most likely served-only failure mode left.
4. The acceptance spec's own oracle: `[data-shell-id="generator"]` →
   `framework.window.proceduralPreview` → `data-meshes-json` / `data-instances-json` non-empty.

If (1)–(3) all check out and the surface is still empty, the next suspect is the render host, not the
guest: compare `data-meshes-json` against the status pill's `PreviewStatusDebug` counts, which
`preview_window_status_json` (`EDIT/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:87`) already
stamps onto the same scene.

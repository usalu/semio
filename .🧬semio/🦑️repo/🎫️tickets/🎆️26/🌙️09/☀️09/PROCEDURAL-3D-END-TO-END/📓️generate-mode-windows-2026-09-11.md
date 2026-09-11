# Generation3d Generate-Mode Windows Audit — 2026-09-11

Read-only audit of the **generate** play mode for `s.procedural.generation3d@1/*#editor`. Code on disk wins over older ticket reports where they disagree. Runtime notes cite boot probes through 2026-09-11 evening (`📓️status.md` §2026-09-11 evening).

---

## Executive summary

| User expectation (generate mode) | Source reality | Runtime (last verified) |
|---|---|---|
| Click **Generate** → three-pane layout (Generations / Form / Preview) | Manifest + layout authored; shell switches mode via `applyModeChange` | **Fixed after boot #13** once window bodies render (~9 s); boot #13 itself still showed edit dock tabs only (contributions blockade) |
| Generations list: add / select / rename / remove | Real `generation_tree` UI + four migrated actions | List chrome renders when bodies arrive; preview still empty |
| Form: sliders/fields for selected generation | Real `generation_form` when a generation is selected; hint otherwise | Form body renders; changing values does not produce 3D output |
| Preview: tessellated 3D of patched generation | Render calls shared `preview_payload`, but eval path is **not** the edit-mode `flowEvalTick` chain | **Broken:** `meshes=0`, preview faults on `unknown kind: brep.curve.polygon` / missing contributions upstream |

Generate mode is **not a separate app** — it is a second mode on the same editor app with three window kinds registered in the manifest. All generation commands are app-scoped; there is no `enterGenerate` command (contrast generation2d, which has `🧬️enter-generate`).

---

## 1. Window inventory

Generate mode declares **exactly three** editor windows (no fourth pane, no panels). Edit-mode-only windows (Flow, side panels) are absent from the generate layout.

### 1.1 Mode shell

| Item | Path | Notes |
|---|---|---|
| Mode definition | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🦀️.rs` | id `generate`, layout id `generation3d-generate`, icon `sparkles` |
| Layout | same file `layout()` | Row split `[22%, 43%, 35%]` → Generations / Form / Preview |
| Mode registration | `✏️editor/🦀️.rs` `create_generation3d_app()` ~1794–1804 | `.mode_def(generate::definition())`, `.mode_layout("generate", …)`, `.named_layout(generate::layout())` |
| Mode-local commands dir | `…/🧬️generate/🎮️commands/📌️.empty.md` | **Empty** — no generate-only commands |
| Mode-local config / presence / transient | `…/🧬️generate/{🎚️config,👥️presence,🫧️transient}/📌️.empty.md` | **Empty placeholders** |

### 1.2 Generations window

| | |
|---|---|
| **Window kind id** | `generation3d-generations` |
| **Body key** | `procedural.play.generations` |
| **Surface** | `Canvas2d` (semantic tree, not a canvas engine) |
| **Rust definition + render** | `…/🪟️windows/🗂️generations/🦀️.rs` |
| **TS view-model twin** | `…/🗂️generations/🟦️.ts` (`Generation3dGenerationsViewModel`) |
| **Render implementation** | `generation_tree(GENERATION_3D_PLAY_APP_ID, "procedural3d-play-generate", …)` in `✏️s/🔌️plugins/🌀️procedural/🫀️core/🖼️semantic-ui/🦀️.rs:56–112` |
| **Window-owned actions** | `addGeneration`, `selectGeneration`, `renameGeneration`, `removeGeneration` (`✏️editor/🦀️.rs:1915`) |
| **Subdirs (actions/config/…)** | All `📌️.empty.md` — actions live at app level, not per-window files |
| **Body wiring** | `generation3d_render_body` match arm `generations::GENERATION_3D_PLAY_BODY_GENERATIONS` (`✏️editor/🦀️.rs:244`) |
| **Content verdict** | **Real body** — tree with row actions and empty-state placeholder |
| **Tests** | `…/🧪️tests/🗂️generations/🔬️unit/🦀️.rs` — `generate_mode_renders_surfaces` (asserts `addGeneration` in projection) |
| | `…/🎮️commands/➕️add-generation/🧪️tests/🔬️unit/🦀️.rs` — undo round-trip, render surfaces, select does not mutate document |

### 1.3 Form window

| | |
|---|---|
| **Window kind id** | `generation3d-generate-form` |
| **Body key** | `procedural.play.generate-form` |
| **Surface** | `Canvas2d` |
| **Rust definition + render** | `…/🪟️windows/📝️form/🦀️.rs` |
| **TS view-model twin** | `…/📝️form/🟦️.ts` (`Generation3dGenerateFormViewModel`) |
| **Render implementation** | `flow_fixture_to_form_spec` → `generation_form(…, "updateGenerationValues", …)` (`form/🦀️.rs:37–43`, `semantic-ui/🦀️.rs:131–200`) |
| **Window-owned actions** | `updateGenerationValues` only (`✏️editor/🦀️.rs:1916`) |
| **Hint when no selection** | `labels.generate_hint` — EN: *"Add a generation to edit input values."* (`🗣️terminology/🦀️.rs`) |
| **Body wiring** | `form::GENERATION_3D_PLAY_BODY_GENERATE_FORM` (`✏️editor/🦀️.rs:245`) |
| **Content verdict** | **Real body** when a generation is selected; **hint-only chrome** otherwise |
| **Tests** | `…/🧪️tests/📝️form/🔬️unit/🦀️.rs` — `generate_form_hints_without_a_selected_generation` |

### 1.4 Preview window

| | |
|---|---|
| **Window kind id** | `generation3d-generate-preview` |
| **Body key** | `procedural.play.generate-preview` |
| **Surface id** | `procedural.play.generate-preview` |
| **Surface kind** | `World3d` (fallback `TextEditor` when no meshes) |
| **Rust definition + render** | `…/🪟️windows/👁️preview/🦀️.rs` |
| **TS view-model twin** | `…/👁️preview/🟦️.ts` (`Generation3dGeneratePreviewViewModel`) |
| **Window-owned actions** | `setCamera`, `setShowMode`, `toggleSun`, `setSunAzimuth`, `setSunElevation`, `setSunIntensity` — **no** gumball transforms (`✏️editor/🦀️.rs:1917–1924`) |
| **Interactions** | `graph` domain registered (`✏️editor/🦀️.rs:1950`) but preview has no graph widgets to pick |
| **Measures** | Same show-mode + sun measures as edit preview (`generate_preview::window_measures` → `edit_preview::preview_window_measures`) |
| **Render path** | Selected generation → `generation_fixture_for` (patch fixture values) → `preview_payload(eval, patched_fixture, cfg, **None**, marks)` → `World3dScene` or hint `TextEditorScene` |
| **Body wiring** | `generate_preview::GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW` (`✏️editor/🦀️.rs:246`); reads `transient.snapshot.generation_preview_text` (`✏️editor/🦀️.rs:1693`) |
| **Content verdict** | **Chrome + fallback hint wired**; **3D content broken** for brep examples (see §4–§5) |
| **Tests** | `…/🧪️tests/👁️preview/🔬️unit/🦀️.rs` — hint text when unevaluated; **no mesh assertion** |
| **Subdirs** | All `📌️.empty.md` under preview (`🎬️actions`, `🫧️transient`, etc.) — no window-local transient owner (unlike edit preview’s `Generation3dPreviewWindowTransientOwner`) |

### 1.5 Shared generation command module

| | |
|---|---|
| **Path** | `…/✏️editor/🎮️commands/🧬️generation/🦀️.rs` |
| **Role** | Maps `addGeneration` / `removeGeneration` / `renameGeneration` / `updateGenerationValues` / `selectGeneration` → artifact + config mutations; builds `preview_fixture` via `generation_fixture_for` when a generation is selected |
| **Used by** | `Generation3dPreviewCommandWork` (retained job path) and direct `handle()` on add-generation command |

### 1.6 App-level transient for generate preview

| | |
|---|---|
| **Schema** | `…/✏️editor/🫧️transient/🦀️.rs` — field `generation_preview_text: Option<String>` |
| **Mutation** | `SetGenerationPreview` — `…/🫧️transient/🧬️schema/🧬️mutations/👁️set-generation-preview/🦀️.rs` |
| **Written by** | `Generation3dPreviewCommandWork::complete` (`✏️editor/🦀️.rs:366–369`) after synchronous `FlowEvalSession::tick` loop |
| **Read by** | `render_with_request_context` → generate preview render (`✏️editor/🦀️.rs:1693`) |

---

## 2. Entering generate mode from the playground

### 2.1 URL and serve target

| Entry | Value |
|---|---|
| React dev URL | `http://127.0.0.1:6018/?plugin=generation3d` |
| Serve script | ticket `📜️serve-generation3d-react.sh` → `bun ./📜️script.ts serve generation3d react dev` |
| wgpu variant | port **6118**, same `?plugin=generation3d` query |
| **No URL parameter for mode** | Mode is session state (`viewState.activeModeId`), not query-string driven |

### 2.2 Navbar controls

| Control | DOM id | Handler |
|---|---|---|
| Example picker | `playground.navbar.fixture.trigger` | Dispatches `setActiveExample` with chosen example id (8 examples in manifest) |
| Edit mode button | `playground.navbar.modes.edit` | `applyModeChange("edit")` |
| Generate mode button | `playground.navbar.modes.generate` | `applyModeChange("generate")` |

Implementation: `🏛️ShellHost/🟦️.tsx` `modeSwitcherElement` (~8248–8267) → `applyModeChange` (~6775–6799):

1. Clears active tool / per-window utility.
2. Sets `viewState.activeModeId`.
3. Calls `resolveLayoutForMode(app, modeId)` (`🎠️kernel/🟦️.ts`) — for generate, resolves `namedLayouts` entry `generation3d-generate`.
4. Seeds shell layout via `applyFrameworkLayoutSeed` and `refreshUi({ kind: "full" })`.

Default mode on boot: **edit** (`default_mode_id(edit::GENERATION_3D_PLAY_MODE_EDIT)`).

### 2.3 Commands

There is **no** `enterGenerate` / `setPlayMode` plugin command. Mode switching is **shell-only** (framework session reducer). All generation **data** commands remain available regardless of mode but only matter when generate windows are mounted:

- `addGeneration`, `removeGeneration`, `renameGeneration`, `updateGenerationValues`, `selectGeneration` — palette + generations tree / form bindings.

Internal runtime chain (not user menu): `flowEvalTick`, `flowEvalResolve`, `flowTessellateResolve`, `setContributions` — scoped to **edit** preview window only (see §4).

### 2.4 Historical runtime note (boot #13)

`📓️boot-13-2026-09-10.md` §2.2: clicking Generate left the **edit** dock tabs (`procedural-main`, `procedural-preview`) because window UI had not arrived yet (contributions blockade). That symptom is **infrastructure**, not missing manifest entries. After the window-bodies fix (`📓️window-bodies-and-eval-dispatch-2026-09-10.md`), status log records generate mode mounting all three bodies.

---

## 3. Body content vs chrome-only

| Window | Expected UX | Wired in Rust | Runtime status (2026-09-11) |
|---|---|---|---|
| **Generations** | List of generations + add/remove/rename/select | Full semantic tree | **Works** once bodies render; actions dispatch through retained `Generation3dPreviewCommandWork` / generation handlers |
| **Form** | Playbook-driven controls for selected generation | Full form or hint | **Works** for chrome; value changes persist to artifact but do not yield preview meshes |
| **Preview** | Shaded 3D world of evaluated generation | World3d when `preview_payload` returns meshes; else TextEditor hint | **Chrome only / faulted** — hint or empty world; `meshes=0` on probes |

### 3.1 Empty taxonomy placeholders (not user-visible)

Every generate window carries standard subfolders (`🎚️config`, `👥️presence`, `🎬️actions`, `☑️options`, `🪛️utilities`, `🫧️transient`) containing only `📌️.empty.md`. This is **schema taxonomy**, not missing UI — real actions are declared on the app manifest and referenced via `window_kind_action_refs`.

### 3.2 Panels in generate mode

Artifact / Catalogue / Inspection panels remain in the navbar (framework-injected). They are **not** part of the generate layout and behave like edit mode — document tree, operator catalogue, inspection of graph selection (often empty in generate mode since there is no flow window).

---

## 4. Preview pipeline: generate vs edit

### 4.1 What the user should see (generate preview)

1. Select (or add) a generation in the Generations pane.
2. Form shows patched input values (`generation_fixture_for` applies `FormGeneration.values` onto matching widgets in the base fixture).
3. Preview evaluates the **patched fixture** and tessellates geometry-bearing outputs.
4. Orbit / show-mode / sun controls adjust the world (same measures as edit preview).

### 4.2 Edit-mode preview path (working design reference)

| Hop | Location |
|---|---|
| Arm tick | `pending_effects` → `flow_eval_tick::rearm` for each attached **`procedural-preview`** window (`✏️editor/🦀️.rs:1656–1668`, `generation3d_preview_window_ids`) |
| Tick work | `Generation3dFlowEvalWindowWork` + `flow_eval_tick::evaluate` (`⏱️flow-eval-tick/🦀️.rs:49–76`) |
| Extension | `ExtensionInvocation` → host → `flowEvalResolve` |
| Tessellate | `preview_tessellate_invocations` → `flowTessellateResolve` |
| Publish eval | `Generation3dPreviewWindowTransientOwner` on **window** transient |
| Render | `edit_preview::render` → `preview_payload(…, **Some(session)**, marks)` + `status_json` |

Tick addressing tests: `…/🧪️tests/🔬️tick-addressing/🦀️.rs` (includes end-to-end mesh test for **edit** preview only).

### 4.3 Generate-mode preview path (actual)

| Hop | Location | Gap |
|---|---|---|
| Trigger | User generation command → `Generation3dPreviewCommandWork` (`GENERATION3D_PREVIEW_TOOL_IDS`, `✏️editor/🦀️.rs:291, 1368–1369`) | OK |
| Eval | `FlowHost` + `FlowEvalSession::tick` in a **tight loop** (`✏️editor/🦀️.rs:413–427`) | **No `ExtensionInvocation`**, no `flowEvalResolve`, no `flowTessellateResolve` |
| Publish | `SetGenerationPreview { preview_text }` on **app** transient | Different store than edit preview |
| Render | `generate_preview::render` → `preview_payload(..., **None**, marks)` (`preview/🦀️.rs:65`) | Session tessellation cache bypassed; still calls `mesh_data_for_preview_handle` synchronously **if** eval JSON contains handles |
| `flowEvalTick` | `retained_window_transient_target` + `Generation3dFlowEvalWindowWork` hard-code **`procedural-preview`** window kind (`✏️editor/🦀️.rs:512–514, 1332`) | Generate preview window **never** receives tick chain |
| `pending_effects` | Only arms ticks for edit preview window ids | Generate preview never auto-evaluates on pending nodes |

**Conclusion:** Generate preview uses the same **`preview_payload` / tessellation helper** as edit mode at render time, but **not** the same **`flowEvalTick` → extension → resolve → tessellate** evaluation chain. For brep-heavy example graphs, synchronous `session.tick` inside `Generation3dPreviewCommandWork` stops at pending extension nodes — eval JSON lacks handles → `meshes_json == "[]"` → fallback hint TextEditor surface (`preview/🦀️.rs:71–74`).

### 4.4 Upstream blockers affecting both modes (runtime)

Even if generate preview adopted the edit chain, current playground runtime still reports (`📓️status.md` 2026-09-11 evening):

- `setContributions` may settle from scoped example graph but preview faults **`unknown kind: brep.curve.polygon`**
- `invokeExtension` → `extension.requester-unavailable` for math/brep evaluate
- `ReadDocument` returning genesis envelope without post-`setActiveExample` ops graph (host scoping skip) — see `📓️window-bodies-and-eval-dispatch-2026-09-10.md` §4.1

These are **shared infrastructure** defects, not generate-window-specific, but they block the user-visible end state for preview in **both** modes today.

---

## 5. Tests vs runtime evidence gaps

### 5.1 Tests that exist (generate-related)

| Test file | What it proves | What it does **not** prove |
|---|---|---|
| `…/🧬️generate/🧪️tests/🔬️unit/🦀️.rs` | Layout JSON contains all three window kind ids | Runtime mode switch, DOM layout |
| `…/🗂️generations/…/🦀️.rs` | Generations body includes `addGeneration` action binding | Tree interaction, preview update |
| `…/📝️form/…/🦀️.rs` | Hint when no generation selected | Form controls with real fixture widgets |
| `…/👁️preview/…/🦀️.rs` | TextEditor hint contains *"evaluate a generation"* | World3d meshes, extension round-trip |
| `…/➕️add-generation/…/🦀️.rs` | Undo/redo, render surfaces string, select idempotence | Preview eval after add |
| `…/🧬️schema/🧬️mutations/*generation*` | Mutation algebra for create/rename/change/delete | UI or preview |
| `…/🔬️tick-addressing/🦀️.rs` | Edit preview tick addressing + **mesh** after `setActiveExample` | **Generate preview** (not in fixture roster) |

**Count:** 4 async tests under `modes/🧬️generate/🧪️tests/**` plus 3 in add-generation command module — all **projection / mutation** level.

### 5.2 Missing test coverage (generate mode E2E)

| Gap | Risk |
|---|---|
| No browser probe step for `#playground.navbar.modes.generate` | Regressions like boot #13 §2.2 undetected |
| No test that `applyModeChange` layout exposes three generate window ids in DOM | Shell/layout integration |
| No test dispatching generation commands **in generate mode view context** with mesh assertion | Core product gap |
| No language-agnostic fixture for generate preview mesh oracle | Violates test-driven rule for feature |
| No cucumber/feature file referencing generate mode | E2E harness only covers geometry examples, IO, mutations — not generate UX |
| `🔍️browser-probe.ts` steps: `example,hover,select,orbit` only | Assumes edit preview canvas; fails with *"no preview canvas"* when generate mode active |

### 5.3 Runtime evidence on record

| Probe / boot | Generate-specific observation |
|---|---|
| Boot #13 | Generate click did not swap dock tabs (bodies blockade) |
| Post window-bodies fix | Three canvases / ten window nodes at ~9 s; **generate bodies mount** per status |
| bodies-3/4 / hold-260 | `meshes=0`, `invokeExtension=0`, contributions skip / fault |
| Evening 2026-09-11 | Scoped pack lands; preview faults on missing brep kind |

**No recorded runtime trace** shows a successful generate-mode preview mesh for any of the eight examples.

---

## 6. Punchlist — code changes for user-seat generate mode

Ordered for dependency. No patches here; implementation lanes should own files as in `📓️gap-inventory-2026-09-10.md` Lane C plus shared infrastructure.

### 6.1 Shell / playground verification

1. Extend `🔍️browser-probe.ts` with `--steps=generate,add-generation,form-edit,preview-orbit` (click `#playground.navbar.modes.generate`, wait for `generation3d-generations` / `generation3d-generate-preview` window ids, assert canvas count ≥ 1).
2. Add launch.json regression entry mirroring probe (devs use launch, not CLI).

### 6.2 Unblock shared eval infrastructure (prerequisite for any preview)

3. Fix live document projection for host `ReadDocument` / contributions scoping so post-`setActiveExample` fixture ops expose brep/math neuron kinds (`📓️window-bodies-and-eval-dispatch-2026-09-10.md` §4.1).
4. Resolve `unknown kind: brep.curve.polygon` / `extension.requester-unavailable` so `setContributions` + `invokeExtension` complete (`📓️status.md` evening).
5. Restage wasm after framework + plugin fixes; verify edit preview meshes first (existing tick-addressing test + browser probe).

### 6.3 Wire generate preview to the same eval chain as edit preview

6. **Option A (recommended):** Register `generation3d-generate-preview` in `generation3d_preview_window_ids` and `retained_window_transient_target`; add `Generation3dPreviewWindowTransientOwner` (or reuse with window-kind discriminator) for generate preview; arm `flowEvalTick` when generate preview is attached and fixture has pending nodes.
7. **Option B:** Refactor `Generation3dPreviewCommandWork` to delegate to `flow_eval_tick::evaluate` + extension/tessellate resolves instead of raw `session.tick` loop (`✏️editor/🦀️.rs:421–427`) — must emit same `ExtensionInvocation` pattern as `⏱️flow-eval-tick/🦀️.rs`.
8. Pass **`Some(session)`** from a retained eval session into `generate_preview::render` (or shared instance owner) so `preview_tessellate_invocations` cache is used (`preview/🦀️.rs:65`).
9. Re-arm generate preview tick after `updateGenerationValues` / `selectGeneration` (mirror `set_active_example::rearm_attached_previews`).

### 6.4 Generate preview UX parity

10. Add gumball actions to `window_kind_action_refs` for `generation3d-generate-preview` if transform utilities are required (currently omitted vs edit preview).
11. Add `status_json` / fault surface to generate preview World3d scene (edit preview injects debug + cancel — `edit_preview/🦀️.rs:72–91`).
12. Consider `translateSelection`/`rotateSelection`/`scaleSelection` only if generate preview instances should be manipulable (product decision).

### 6.5 Tests (must land with fixes)

13. Async integration test: dispatch `AddGeneration` + `SelectGeneration` under generate mode view → drain ticks → assert `World3dScene` mesh count ≥ 1 (mirror `set_active_example_drives_the_self_dispatched_tick_chain_to_a_rendered_mesh` but for generate window ids).
14. Extend `tick-addressing.json` fixture with `generation3d-generate-preview` attached window cases.
15. Language-agnostic JSON oracle: patched generation fixture → eval → expected mesh count / volume band (reuse example-geometry harness patterns).
16. Browser probe green path documented in ticket checklist.

### 6.6 Cleanup / hardening

17. `generation_fixture_for` clones fixture — render already calls `retire_cold()` (`preview/🦀️.rs:66`); add test that repeated preview renders do not leak (ticket comment at preview/🦀️.rs:58–62).
18. Remove or implement empty generate mode command folder (`🎮️commands/📌️.empty.md`) — if mode entry stays shell-only, document in manifest that no `enterGenerate` is intentional (generation2d divergence).

---

## Appendix A — File path quick reference

```
…/✏️editor/🎭️modes/🧬️generate/
  🦀️.rs                          # mode + layout
  🪟️windows/🗂️generations/🦀️.rs
  🪟️windows/🗂️generations/🟦️.ts
  🪟️windows/📝️form/🦀️.rs
  🪟️windows/📝️form/🟦️.ts
  🪟️windows/👁️preview/🦀️.rs
  🪟️windows/👁️preview/🟦️.ts
  🧪️tests/{🔬️unit,🗂️generations,📝️form,👁️preview}/🦀️.rs

…/✏️editor/🎮️commands/🧬️generation/🦀️.rs
…/✏️editor/🎮️commands/{➕️add-generation,🎯️select-generation,🎚️update-generation-values,…}/🦀️.rs
…/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs   # edit-preview eval chain only today

…/✏️editor/🦀️.rs                 # render dispatch, PreviewCommandWork, preview_window_ids
…/✏️editor/🫧️transient/…          # generation_preview_text

✏️s/🔌️plugins/🌀️procedural/🫀️core/🖼️semantic-ui/🦀️.rs  # generation_tree, generation_form

🧰️framework/…/🏛️ShellHost/🟦️.tsx   # mode switch, window UI delivery
🧰️framework/…/🎠️kernel/🟦️.ts      # resolveLayoutForMode
```

---

## Appendix B — Related ticket reports

- `📓️gap-inventory-2026-09-10.md` §1.2, §6 Lane C — prior generate-mode gap analysis (still accurate on eval chain)
- `📓️boot-13-2026-09-10.md` §2.2 — generate mode never exercised before bodies fix
- `📓️window-bodies-and-eval-dispatch-2026-09-10.md` — bodies + contributions scoping
- `📓️status.md` — runtime timeline through 2026-09-11 evening

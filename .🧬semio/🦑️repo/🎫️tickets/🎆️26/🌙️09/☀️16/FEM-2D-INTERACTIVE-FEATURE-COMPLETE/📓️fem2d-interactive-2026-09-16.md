# 🏗️ Fem2d interactive feature completeness — report (2026-09-16)

Coordinator: session ⚪34c772d7 (Fable 5.1). Fleet: 5 Opus execution slices (A interaction, B artifact tree, C mutations, D inspector, E animation) + 1 Opus fix (combination guard) + 1 Opus rewrite (inspector form layout); 4 Sonnet explorations + 2 Sonnet audits. Ticket bookkeeping on disk (repo MCP down). Reports per slice: `📓️w-{a,b,c,d,e}-*.md`, `📓️fix-2026-09-16-combination-self-reference.md`; explorations `📓️explore-2026-09-16-*.md`; plan `📓️plan.md`; log `📓️status.md`.

## What fem2d can do now (React lane, port 6086)

- **Artifact tree** (`📌️panels/🗿️artifact`): nine sections — Nodes, Elements, Regions, Supports, Load Cases (with their loads nested), Combinations (terms nested), Materials, Sections, Analysis — with document counts, human labels (`n1 · (0.00, −4.00)`, `e3 · Beam n1 → n2`, `Dead Load · 1 Load · Self Weight`, `ULS · 1.35 dead + 1.5 live`), dangling-reference dimming, max-min-fair paging of the one-page row budget, `+N` continuation rows. A row click is a framework `interactionSelect` pick; rows author one argument map each (row actions moved to the inspector after they starved the shared arena page).
- **Selection** is the framework-owned `fem2d` interaction domain (nine granularities, raw entity ids): tree picks, viewport picks (`canvasPointerDown` hit-testing in canvas pixels: node → support → load → element → region; empty click clears), hover, `sel-*`/`hov-*` overlay layers in both canvases, delete/backspace on the live selection, `focusEntity` framing.
- **Inspector** (`📌️panels/🔍️inspection`): a form per entity kind — number inputs, sliders, selects, toggles bound `Trigger::Change` → nine `patch*` commands (`{id, field, value}`) that emit whole-record `replace-*` mutations (or `change-load-case-name` / `-self-weight`); Focus/Delete buttons; the owning case's loads and a combination's terms as pick rows; document summary when nothing is selected.
- **Mutations** (slice C): `replace-node`, `replace-load`, `change-load-case-name`, `replace-combination` on every schema surface (leaf rs/diff/inverse/json/descriptor, 19 scenarios + fixtures, aggregate rs/ts/json, subset feature/rs/py twins, oracles, generator, taxonomy + schema catalog); the combination guards now refuse a self-referencing term.
- **Deformation animation** (slice E): `Fem2dResultsWindowConfig.animation { phase, playing, speed, loopMode, waveform, reverse }`; a `DispatchAction` clock (`resultAnimationTick`, 33 ms, one clock per window, inert when stopped, Once/PingPong/Loop, Ramp/Sine), a results cache keyed by (instance, revision) so ticks never re-solve, the deformed shape / reactions / moment diagrams / mode shapes scaled by one amplitude, a `phase 0.45 · ▶ 0.5 Hz` caption; the **Results panel** (`📌️panels/📊️results`): source/mode/mode-index, phase slider, Step −/+, Play/Pause, speed, loop, waveform, analysis settings — every control tagged with its `windowId`; playback publications coalesce into one history edit.

## Verified

| Gate | Result |
|---|---|
| `cargo check … --features component-app-assembly --tests` | green (`📜️check-fem2d.sh`) |
| feature-enabled nextest, whole crate (runs 12 / 14) | 1238 / 1240 · 1240 / 1243 — see "Known gaps" (convergence law; engine wall-clock tests red only under fleet load) |
| `fem-plugin:test` / `fem-2d-rs:test` / `fem-js:test` | 5/5 · 1038/1038 · 6/6 |
| `fem-plugin:describe` + `plugin-registry:generate` | green (descriptor pair regenerated with the 15 new tools) |
| `component-dev` (wasm32-wasip2) via `activate-fem2d-react-dev` | green ×5 |
| Browser (built-in pane + playwright probes, `🗑️generated/fem2d-*`) | boot clean; tree lists the demo; `n1`/`s1` picks mark the row, paint the halo in both canvases and open the inspector; Delete removes `s1` (results window reports the mechanism); empty-canvas click clears; Play from the panel arms the clock (tick dispatches observed), the results window animates (caption phase 0.45 → 0.94, reactions change), the focused panel reads live (`Pause`, phase 0.30 → 0.45 across ticks); after the form-layout rewrite the inspector shows `Node / ID / X / Y` number inputs — typing X = 1.5 + Enter dispatches `patchNode`, the tree reads `n1 · (1.50, −4.00)`, the model canvas moves the node and the results window re-solves; History → Undo restores all three; on the final build the `mod+z` chord (now declared) undoes the edit too |

## Framework findings (not fem2d's to fix)

1. The fem artifact crates gate their editors behind `component-app-assembly`; `fem-2d-rs:test` runs without it, so the editor tests had never run (memory `project-fem2d-editor-feature-gated-tests`).
2. A peer's 12:06 commit (f66680537c) requires every store to be closed before drop and `bind_instance_id` before typed dispatch; the fem2d harness now wraps its fixture app (bound, settle-after-dispatch, self-closing). Registry-less `paired_apps`/`assert_two_instances_converge` fail the proof-catalog join for apps with tool proofs, and `paired_registered_apps` refuses `attach_backbone` — `two_instances_converge_on_disjoint_edits` stays red as the canonical law.
3. A dock panel projection captures only the FOCUSED window's config, so the results panel shows defaults until the Results pane is focused (unfocused mode: bare Play/Pause toggle + hint). Window-config publications refresh only the guest-declared `ui_scope` (now `Partial { results window, results panel }`).
4. The React tree renderer drops non-tree children, so editable controls must live in `ui::section`/`field` layouts (inspector rewritten).
5. The one-page `UiValue` argument arena is shared by every panel; two row actions per tree row exhausted it (cad's incident) — tree rows now author their pick only.
6. `mesh_job_large_boundary_never_runs_to_completion_in_one_step` (engine wall-clock budget) is red under fleet load and green alone.

## Known gaps / follow-ups

- `two_instances_converge_on_disjoint_edits` (framework convergence law) — red for every app with tool proofs.
- Playback re-arm delay is clamped to ~1 tick/s in hidden panes (host); the transport is usable but coarse there.
- `SelectionMethod::Rectangle` is declared but not implemented for the canvas; duplicate area-load glyphs on one region; hover is emitted per pointer sample.
- The carrier fixture generator's output format drifted before this ticket (compact vs pretty JSON) — regenerating the whole corpus is its own change.
- `focusEntity` from the inspector dispatches cleanly but the camera move was not visually confirmed (the Canvas2d host owns pan state).

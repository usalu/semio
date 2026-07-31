---
name: Playground Window Mode Completeness
overview: "Bring every playground's windows and modes up to the same completeness bar already met by puzzle/2d, puzzle/3d, puzzle/5d and shooting: every `WindowKindRuntime` gets a populated `measures` rail (window options) and a `WindowEngagement` (commands), and every `ModeRuntime` gets a populated `tools` footer toolbar — then add a runtime auditor so this stays enforced."
todos:
 - id: phase1-tools
   content: "Add missing Mode.tools footer toolbars: flow (both modes), semios, trinity/jack, trinity/rewrite, mathematical/dag"
   status: completed
 - id: phase2-editors
   content: Add window measures+engagement to draw, forms, raster (all windows currently bare)
   status: completed
 - id: phase2-semios
   content: Add window measures+engagement to all 4 semios windows
   status: completed
 - id: phase2-trinity
   content: Add window measures+engagement to trinity/jack (Jack Query, Results) and trinity/rewrite (LHS/RHS/Jack/Parameters, plus engagement on Before/After)
   status: completed
 - id: phase2-generate
   content: Add measures+engagement to generate-mode windows in flow, procedural/2d, procedural/3d
   status: completed
 - id: phase2-writer
   content: Replace writer's empty measures [] with real editor-settings measures
   status: completed
 - id: phase3-auditor
   content: Extend framework/product/playground/core/script.ts with an audit command asserting measures/engagement/tools completeness across all playgrounds; wire into nx/launch.json
   status: in_progress
 - id: ticketing
   content: Open/reopen one ticket per technology cluster per repo MCP workflow, in the worst-first order above
   status: completed
isProject: false
---

# Playground Window & Mode Completeness Pass

## Ground truth (verified by reading code, not assumed)

"Window" = `WindowKindRuntime` (`framework/product/playground/core/index.ts:362`), built from `(id, label, bodyKey, iconId?, measures?, engagement?, templates?)`. "Window options" = `measures: WindowMeasure[]` (the options rail, tree-composable via `WindowMeasureGroup`). "Commands" = `engagement?: WindowEngagement` (floating command input/controls, `framework/product/playground/core/index.ts:290`). "Mode" = `BaseModeRuntime` (`framework/core/index.ts:1137`); "tools in the footer" = `Mode.tools: AppTools`, rendered as the footer toolbar by the playground renderer.

I re-verified every playground directly via `new WindowKindRuntime(...)` and `.tools =` call sites (not just the earlier survey) so the worklist below is grounded in real line numbers. I also disproved an earlier "missing rs/react layer" theory — every technology already has full `core/play/react/rs` coverage (flow's Rust lives in `flow/core/`, dag's at `mathematical/graph/port/directed/dag/`, trinity splits into `jack/core+lsp+shell` + shared `trinity-react`, `wires` intentionally reuses `puzzle/2d`'s complete runtime, `reasoning/mindmap` is documented `kind: library` with no playground expected). So this pass is scoped purely to window/mode completeness — no new Rust/React packages needed.

Already fully complete (reference implementations): `puzzle/2d`, `puzzle/3d`, `puzzle/5d`, `shooting` (measures + engagement + tools on every window/mode).

## Phase 1 — Add missing footer toolbars (`Mode.tools`)

No `.tools =` assignment exists at all in these files today:

- [flow/play/index.ts](flow/play/index.ts) — `rebuildShellMode()` (~744) and `rebuildGenerateMode()` (~750) set `windowKinds` but never `this.mainMode.tools` / `this.generateMode.tools`.
- [semios/play/index.ts](semios/play/index.ts) — 4 windows (Media Graph, App Host, Launcher, History), no toolbar.
- [trinity/jack/play/index.ts](trinity/jack/play/index.ts) — no toolbar across Nakagin Graph / Jack Query / Results.
- [trinity/rewrite/play/index.ts](trinity/rewrite/play/index.ts) — no toolbar across LHS/RHS/Jack/Parameters/Before/After.
- [mathematical/graph/port/directed/dag/play/index.ts](mathematical/graph/port/directed/dag/play/index.ts) — has measures+engagement but no toolbar.

For each: add a `build<Tech>PlayToolbarTools(...)` function (same pattern as `buildFormsPlayToolbarTools`, `buildWriterPlayToolbarTools`, `buildMapPlayToolbarTools`) built from commands the controller already exposes (selection/undo-redo/run/reset/etc.), assign it in the mode-rebuild method, and register a `ToolLeaf`/`ToolNode` per real action — no placeholder/no-op buttons.

## Phase 2 — Add missing window `measures` + `engagement`

Fully bare windows (`new WindowKindRuntime(id, label, bodyKey)` — zero measures, zero engagement):

- [draw/play/index.ts:1029-1030](draw/play/index.ts) — Canvas + Navigator.
- [forms/play/index.ts:696-697](forms/play/index.ts) — Edit + Try.
- [raster/play/index.ts:746-747](raster/play/index.ts) — Composite + Navigator.
- [semios/play/index.ts:167-170](semios/play/index.ts) — all 4 windows.
- [trinity/jack/play/index.ts:322-323](trinity/jack/play/index.ts) — Jack Query + Results (Nakagin Graph already has measures, needs engagement).
- [trinity/rewrite/play/index.ts:391-394](trinity/rewrite/play/index.ts) — LHS/RHS/Jack/Parameters (Before/After already have a LOD measure, need engagement too).
- Generate-mode windows: [flow/play/index.ts:754](flow/play/index.ts), [procedural/2d/play/index.ts:1025](procedural/2d/play/index.ts), [procedural/3d/play/index.ts:1454](procedural/3d/play/index.ts).
- [writer/play/index.ts:548](writer/play/index.ts) — explicit `measures: []` (has engagement already; needs real measures, e.g. font size/line numbers/tab size, mirroring the already-existing `EditorSettings` WASM surface used by `editor-mode-check.mjs`).

For each window, derive the measure/engagement content from state the controller/canvas already manages (e.g. draw: stroke/fill/zoom-level toggles as measures, tool-args as engagement; forms: field-validation/preview-mode toggles; semios: per-window layout/zoom controls; trinity/jack Results: wrap/auto-scroll toggle; trinity/rewrite LHS/RHS/Jack: LOD measure matching Before/After, engagement for "apply rule"). Follow the `WindowMeasureGroup` composable-tree pattern from `.cursor/plans/composable_window_option_trees_afd215c8.plan.md` when a window needs more than 1-2 flat controls.

## Phase 3 — Runtime completeness auditor (prevents regression)

Extend [framework/product/playground/core/script.ts](framework/product/playground/core/script.ts) with a new `audit` subcommand that imports every known playground's `build<Tech>PlayAppRuntime`/controller, walks `mode.windowKinds` and asserts `measures.length > 0` and `engagement != null` for every window (with a small, explicit allowlist for legitimately measure-less windows, e.g. pure hosts), and asserts `mode.tools` is defined with at least one non-separator leaf. Register it as an `nx` target the same way other `script.ts` commands are wired into `project.json`/`launch.json`, per repo conventions. Run this after every phase-1/phase-2 fix as the "definition of done" instead of eyeballing each file.

## Ticketing

This spans many independent technologies — work it as one ticket per technology (or logical cluster), not one giant ticket, following existing precedent (`draw_technology_completeness_pass` reopened `DRAW-VECTOR-TECHNOLOGY`; forms reopened `FORMS-TECHNOLOGY-AND-GENERATE-MODE`). Before opening each: read `repo://goals`, check for an existing open/closed ticket covering that technology and `ticket_reopen` if found, otherwise `ticket_open`. Suggested order (worst-first, matches the audit above):

1. `flow` + `procedural/2d` + `procedural/3d` (shared generate-mode gap, likely one ticket)
2. `semios`
3. `trinity/jack` + `trinity/rewrite` (shared trinity toolbar patterns, likely one ticket)
4. `mathematical/dag` (tools only — smallest fix)
5. `draw`, `forms`, `raster` (each already has tools; needs measures+engagement only)
6. `writer` (measures only)
7. Land the Phase 3 auditor last (or first, as a TDD guardrail, if preferred at execution time), then re-verify all technologies pass it.

## Verification

Per-technology: re-run that technology's existing `core`/`react`/`play` test suites (`nx test`), then runtime-verify in the actual playground (per repo rules — no claiming a feature works without confirming runtime behavior), extending each technology's existing ticket-folder check scripts (e.g. reuse `DRAW-VECTOR-TECHNOLOGY/runtime-check.mjs`-style scripts) rather than creating new ones. Finish each ticket by running the new Phase 3 auditor and `ticket_close` with the full file list.

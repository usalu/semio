---
name: True Full-LOC S Parity
overview: Port the old S studio to the pure-Rust program framework at full fidelity and full scale (~100k LOC, matching the deleted code), so the new S looks, behaves, and measures like the old one, verified end-to-end in the browser.
todos:
 - id: baseline-audit
   content: Extract old-S reference sources and behavior checklist into ticket folder; set up old-to-new LOC accounting
   status: completed
 - id: shell-full-port
   content: Full port of platform+playground renderers (8k LOC) into os-shell/ui-interpreter and platform+playground core runtimes (5.4k) into framework/core/rs
   status: completed
 - id: os-core-full
   content: Complete framework/product/os/core/rs to full os-core JS coverage (backbones, media export, catalog, alternatives)
   status: completed
 - id: s-program-exact
   content: "Line-for-line S program port: SPlayController commands, applySOsUri, export download, exact S_PLAY_LAYOUT, all old vitest behaviors as Rust tests"
   status: completed
 - id: demo-tech-full
   content: Full ports of draw, writer, raster, note plugins (~15k LOC) including all old react surface behavior as Rust scenes
   status: completed
 - id: graph-tech-full
   content: Full ports of flow, dag, sequence, imperative, trinity-rewrite, mathematical dsl (~15k LOC)
   status: completed
 - id: tech-2d-full
   content: Full ports of puzzle2d, gis2d, procedural2d, layout, reasoning-wires, forms, vcs (~38k LOC)
   status: completed
 - id: tech-3d-full
   content: Full ports of cad, puzzle3d, puzzle5d, procedural3d, lowpoly, shooting (~53k LOC)
   status: completed
 - id: presentation-trinity
   content: Full ports of presentation and trinity/jack plugins (~17k LOC)
   status: completed
 - id: e2e-loc-audit
   content: Browser E2E on 6066 across all 25 programs with [DEBUG] runtime confirmation; final LOC audit vs old in ticket folder
   status: completed
isProject: false
---

# True Full-LOC S Parity Port

## Current gap (measured)

The deleted code at `f8376e848` totals ~110k lines; the current Rust/renderer port totals ~10.7k. The existing port is a structural approximation, not the demanded line-for-line behavioral port. Old sizes (reference via `git show f8376e848:<path>` only):

- Framework: platform renderer 5,880 · platform core 3,804 · playground renderer 2,207 · playground core 1,566 · os core 3,095 · framework core js 2,020 · presentation renderer 11,515 · presentation core 4,440
- S itself: `s/core/js/index.ts` 1,579 (still on disk) · `s/react/index.tsx` 521
- Tech packages (core+internal+react): puzzle2d 22,560 · puzzle3d 22,379 · cad 14,553 · puzzle5d 8,162 · flow 7,516 · procedural3d 5,070 · gis2d 3,850 · writer 3,908 · raster 3,931 · draw 4,133 · lowpoly 3,256 · forms 3,917 · sequence 2,610 · procedural2d 3,248 · note 3,145 · dag 2,220 · layout 2,500 · shooting 2,471 · trinity 2,087+1,008 · reasoning/vcs/rest ~3k

`ui/react/index.tsx` (25,455 lines) still exists and is the visual foundation — not ported, only consumed.

## Approach

Port each old file 1:1 into its new home, preserving every behavior, command, layout constant, inspector field, and scene detail. New homes: tech logic/scenes go into `<tech>/program/rs/lib.rs` (+ existing `<tech>/rs` domain crates), shell/React goes into [framework/renderer/react/os-shell.tsx](framework/renderer/react/os-shell.tsx) and [framework/renderer/react/components/](framework/renderer/react/components/), OS/platform runtime goes into [framework/core/rs](framework/core/rs) and [framework/product/os/core/rs](framework/product/os/core/rs). Extend existing files with regions; no new files outside the package structure and ticket folder `26/07/04/RUST-PLUGIN-FRAMEWORK-MIGRATION`.

Each phase ends with: `cargo test` for the crates touched, program rebuild, and a browser check on port 6066 with `[DEBUG]` console logs confirming runtime behavior. No phase is "done" without the runtime confirmation.

## Phase 1 — Parity baseline and audit harness

- Extract the old-S behavior spec: dump `f8376e848` platform/playground/os/s sources into the ticket folder as reference notes; derive a behavior checklist (every command, panel, window body, keybinding, URI route).
- Add a LOC accounting script section to the ticket folder mapping old file → new file → target/current LOC, updated per phase.

## Phase 2 — Framework shell full port (~12k LOC React)

Port `platform/renderer/react/index.tsx` (5,880) + `playground/renderer/react/index.tsx` (2,207) fully into [framework/renderer/react/os-shell.tsx](framework/renderer/react/os-shell.tsx) and the ui-interpreter/component files: ProductShell with windowMeasuresToGolden, tab stacks, engagement rails, surface bindings, useUIHistory, useCommandHotkey, PanelToggleGroup, theme/compact/expertise, footer rows, drag-and-drop trees, every renderUiControl branch. Port `platform/core/js/index.ts` (3,804) + `playground/core/js/index.ts` (1,566) runtime semantics into [framework/core/rs](framework/core/rs) (UiNode vocabulary completion, WindowKind/Mode runtimes, VFS surfaces, layouts).

## Phase 3 — OS core completion (~3k LOC Rust)

Finish [framework/product/os/core/rs](framework/product/os/core/rs) to full `os/core/js/index.ts` (3,095) coverage: every backbone, media export handler with coverage assertion, program/resource descriptor, parameter compatibility rule, studio catalog operation, OsStore alternative/checkpoint materialization path.

## Phase 4 — S program exact port (~2.1k LOC Rust)

Line-for-line port of [s/core/js/index.ts](s/core/js/index.ts) (1,579) + old `s/react/index.tsx` (521) into [s/program/rs/lib.rs](s/program/rs/lib.rs): SPlayController command-for-command, applySOsUri routing, media export download, compose-sketchpad program alignment, engagement, exact `S_PLAY_LAYOUT`, all 20 vitest behaviors as Rust tests.

## Phase 5 — Demo-studio tech plugins full port (~15k LOC Rust)

Full ports (core + internal + react surface behavior as Rust scene builders) for the demo studio apps: draw (4,133), writer (3,908), raster (3,931), note (3,145). Every tool, layer operation, inspector field, canvas interaction command from the old react packages.

## Phase 6 — Graph-family tech plugins (~15k LOC Rust)

flow (7,516: components/sources/sinks/channels per flow AGENTS spec), dag (2,220), sequence (2,610), imperative, trinity-rewrite (1,008), mathematical dsl — node-graph scenes with exact port/channel semantics.

## Phase 7 — 2D-family tech plugins (~38k LOC Rust)

puzzle2d (22,560), gis2d (3,850), procedural2d (3,248), layout (2,500), reasoning-wires, forms (3,917), vcs (1,021) — canvas-2d/table scenes with full old feature sets.

## Phase 8 — 3D-family tech plugins (~53k LOC Rust)

cad (14,553), puzzle3d (22,379), puzzle5d (8,162), procedural3d (5,070), lowpoly (3,256), shooting (2,471) — world-3d scenes: meshes, instances, cameras, gizmos, per-mode interactions.

## Phase 9 — Presentation and trinity (~17k LOC Rust)

presentation (15,955), trinity/jack (2,087) as full plugins.

## Phase 10 — End-to-end verification and LOC audit

- Browser run `SEMIO_PLUGIN=s` on 6066 (launch config `dev s`): home → open studio → spawn each of the 25 programs → edit documents → parameters/bindings → checkpoint → export, each step confirmed by `[DEBUG]` console logs and screenshots vs old-S reference.
- Final LOC audit in the ticket folder: new total must land in the same order as the old (~100k), with the old→new file map complete.
- All Rust test suites and the renderer vitest suite green.

## Constraints

- Readonly git only (`git show f8376e848:<path>`); no checkout/stash/commit.
- `bun` + `nx`, scripts only in `script.ts`, existing launch.json configs.
- Extend existing files with regions; temp artifacts only in the ticket folder.

## Risk

This is a ~100k-line port — a multi-session effort. Phases are ordered so the app stays bootable and each batch is independently verifiable; the LOC audit keeps progress honest against the "same size as before" requirement.

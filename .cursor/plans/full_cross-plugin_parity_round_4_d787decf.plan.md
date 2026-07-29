---
name: Full Cross-Plugin Parity Round 4
overview: A fresh, verified re-audit (the prior restore_app_parity_post-migration plan was stale — puzzle3d/lowpoly/raster are already fully fixed) found confirmed, specific parity gaps across 16 of the remaining 19 non-S plugins. This plan fixes them in severity-ordered waves, starting with a cross-cutting node-graph command-contract bug that silently breaks canvas interaction in 5 plugins at once.
todos: []
isProject: false
---

# Full Cross-Plugin Parity — Round 4

## Why another round

Previous rounds fixed S studio and shell-wide regressions. The existing `.cursor/plans/restore_app_parity_post-migration_e255a2e1.plan.md` claimed 12 "Tier 1" apps were severely broken, but a fresh verification pass shows **puzzle3d, lowpoly, and raster are already fully ported** (real GLB meshes/brush-fill, real halfedge paint/mesh-edit, real pixel decode/composite) — evidently fixed by concurrent work outside this thread. That plan is stale. This plan replaces it with a fully re-verified gap list across every remaining program, using the pre-migration commit `32693795d4d761d347338fa010fed19ab714ef2d` (last commit before old TS `core/js`/`react` files were deleted) as ground truth.

## Cross-cutting bug found: node-graph canvas commands don't match across 5 plugins

`framework/renderer/react/components/node-graph-host.tsx` always dispatches a fixed contract from `framework/renderer/react/types.ts:258-264`:

```258:264:framework/renderer/react/types.ts
export const nodeGraphCommands = {
	select: "nodeGraphSelect",
	hover: "nodeGraphHover",
	edit: "nodeGraphEdit",
	viewport: "nodeGraphViewport",
	spotlightCommit: "spotlightCommit",
} as const;
```

Only `s/program/rs/lib.rs` and `flow/program/rs/lib.rs` implement handlers for these exact names. The other 5 plugins that mount `node-graph-host` instead implement legacy/custom names (`setSelection`, `selectNode`, `graphPointerDown`) that the host never sends, so **clicking/hovering/dragging nodes on the canvas is silently a no-operation** in:

- `trinity/jack/program/rs/lib.rs:700` (`"setSelection"`), `:864` (`"graphPointerDown"`)
- `trinity/rewrite/program/rs/lib.rs:712` (`"setSelection"`), `:769` (`"graphPointerDown"`)
- `mathematical/graph/port/directed/dag/program/rs/lib.rs:650` (`"setSelection" | "selectNode"`), `:671` (`"graphPointerDown"`)
- `sequence/program/rs/lib.rs:512`, `:516`
- `procedural/3d/program/rs/lib.rs:691`, `:848`

**Fix:** add `"nodeGraphSelect"`, `"nodeGraphHover"`, `"nodeGraphEdit"`, `"nodeGraphViewport"` as the primary handled names in all 5 plugins' `handle_command` match arms (keeping/removing legacy names per program as appropriate), matching the pattern already correct in `s`/`flow`. Two plugin-specific follow-ons surfaced by the audit: `dag` also needs `"deleteSelection"` handling (Delete/Backspace on the canvas currently dispatches this, but only `removeNode` exists, which nothing calls), and `dag`'s `render_main_graph` needs `selection_json` wired into the scene (unlike trinity-rewrite, which at least plumbs it) so tree-selected nodes highlight on the canvas.

## Confirmed gaps by app (post fresh audit)

**Already fully fixed (no action needed):** puzzle3d, lowpoly, raster, dag's `renameDagNode`/node-kind coverage/undo-redo, gis2d's camera/tiles/routes, flow's evaluation pipeline, procedural3d's brep evaluation, sequence's nested slot/collapse editing, presentation's frame/example/patch commands, note's patch/undo/duplicate/delete, vcs's checkout/checkpoint, puzzle2d's edges/wires/handles rendering and inspector patch.

**Tier 1 — broken core interaction:**

- **trinity (jack)**: bespoke WebGPU `TrinityCanvas` (LOD/force-layout) downgraded to generic node-graph _and_ clicks/drags dead (command mismatch above).
- **trinity-rewrite**: LHS/RHS are read-only graphs missing semantic node-kind vocabulary (`match`/`where`/`set`/`parameter`/`create`/`delete`/`merge`); cross-panel hover/select bridge is dead code; command mismatch above.
- **procedural2d**: the actual node-graph editor window was replaced by a duplicate canvas — no `addWidget`/`moveMediaNode`/`removeWidget`, so the flow graph cannot be edited from the UI at all.
- **cad**: no transform gumball UI (`grep -i gumball` = 0 matches; only headless `translateSelection`/`rotateSelection`/`scaleSelection`), no quad multi-pane view (single `CAD_PLAY_WINDOW_COMPOSITE` vs old 4-pane shape/building/energy/structure-classic), no undo/redo (dead `vcs::{Operation, OperationDiff}` import, `cad/program/rs/lib.rs:20`), typology mesh geometry hardcoded per-typology (`cad/program/rs/lib.rs:73-80`) instead of derived from the object's authored dimensions.
- **draw**: canvas renders bounding-box-only stubs, no real path/fill/boolean geometry, no interactive drawing tools.
- **dag/sequence/procedural3d canvas click/select**: covered by the cross-cutting fix above.

**Tier 2 — degraded but functional:**

- **puzzle5d** (vs. fully-ported puzzle3d reference): missing `deleteSelection`, `setFixtureJson`, `worldVortexHover/Select`, `worldRelocate`, `setBrushPlacementOverlapBudget`, `setObjectKindWeight`/`setVortexKindWeight`; grips/fasteners never rendered or selectable in either 2D or 3D view (`puzzle/5d/program/rs/lib.rs:279-301`, `:350-378`).
- **puzzle2d**: 3-pane LOD architecture (overview/detail/selection) collapsed to one pane; engagement REPL input/candidate-cycling control/fill-slider all stripped (`puzzle2d_engagement`, `puzzle/2d/program/rs/lib.rs:440-489` leaves `input`/`control`/`controls` all `None`); suggestion-offset and kind-weight sliders have working handlers but no UI ever calls them.
- **gis2d**: most of `MapHost`'s capability (render mode, vector style, LOD, feature hit-testing/selection, route editing) never invoked from the program.
- **flow**: ~18 of ~28 old commands ported; missing LOD/proximity/catalogue/extension/generation commands (`setLodMode`, `setProximityDistance`, `setCatalogueSections`, `toggleExtension`, `runExtensionCommand`, etc.).
- **writer**: `formatDocument` is a no-operation; completions/lint run against an empty graph (schema-blind).
- **shooting**: icon window shows a hardcoded placeholder PNG; export produces a generic title-card SVG, not a real render of the model/icon.
- **layout**: inspector only edits bounds/name/size (missing fill/stroke/story-content/margins/columns/link-path); `exportPng`/`exportPdf`/`exportPackage` are stubbed despite real export code existing in `layout/rs/export.rs`.
- **reasoning/wires**: edges/relationships render, but node/wire dragging and live force-layout (inherited from puzzle2d) never made it into the port — canvas is static/click-only.
- **forms**: "Try" tab is a static table, not a live form preview; `required` flag is faked via a runtime map wiped by nearly every edit.
- **imperative**: `addStepAt`/`removeStepAt`/`setStepParamsAt` hardcode `PathRef::default()` so nested control-body targeting never actually reaches the right step; run output truncated to 80 chars in a fake table row.
- **vcs**: projection JSON text editor accepts typed input that's silently discarded (no `"edit"` handler); backbone sync exists at the OS level but isn't exercised by the vcs-play demo.

**Tier 3 — minor, isolated fixes:**

- **sequence**: footer toolbar (Run/Stop/Reorganize/orientation toggle) unwired.
- **procedural3d**: gumball drag transforms only write ephemeral runtime state (not persisted to the flow graph); no undo/redo/delete-widget.
- **presentation**: canvas never draws the actual source image behind crop tiles, only labeled boxes.
- **note**: arrow-key nudge is a no-operation (keybinding builder can't pass `dx`/`dy` args).

## Execution waves

Each unit of work happens inside its own ticket per repo rules (`ticket_open`, associated with the migration goal), temporary artifacts in the ticket folder, closed with a full summary of files touched.

1. **Wave 1 — cross-cutting node-graph command fix** (trinity/jack, trinity/rewrite, dag, sequence, procedural3d): restore the standard `nodeGraphSelect`/`Hover`/`Edit`/`Viewport` contract. Highest leverage, unblocks canvas interaction in 5 apps at once.
2. **Wave 2 — Tier 1 remaining** (trinity canvas/vocabulary restoration, procedural2d graph-editor window, cad gumball/quad-pane/undo/typology geometry, draw real path/fill geometry + tools).
3. **Wave 3 — Tier 2** (puzzle5d command parity with puzzle3d, puzzle2d LOD panes/engagement UI, gis2d MapHost wiring, flow remaining commands, writer format/schema-aware completions, shooting real render, layout inspector+export wiring, wires drag/force-layout, forms live preview/required-flag fix, imperative PathRef targeting fix, vcs projection edit handler).
4. **Wave 4 — Tier 3 cleanup** (sequence footer toolbar, procedural3d gumball persistence + undo/redo, presentation source-image rendering, note nudge fix).

Each wave ends with `cargo test`/`cargo check --target wasm32-unknown-unknown` for touched crates and a targeted manual/E2E smoke check before moving to the next wave.

## Todos

</plan>
<todos>
[{"id":"wave1-nodegraph-contract","content":"Fix node-graph command contract mismatch in trinity/jack, trinity/rewrite, dag, sequence, procedural3d to handle nodeGraphSelect/Hover/Edit/Viewport"},{"id":"wave2-trinity-canvas","content":"Trinity (jack): restore bespoke canvas capability (LOD/force-layout) or at minimum verify generic node-graph now correctly drives selection/query-result rendering after Wave 1 fix"},{"id":"wave2-trinity-rewrite-graphs","content":"Trinity-rewrite: make LHS/RHS pattern graphs editable with full semantic node-kind vocabulary; restore cross-panel hover/select bridge"},{"id":"wave2-procedural2d-editor","content":"Procedural2d: restore real node-graph editor window (addWidget/moveMediaNode/removeWidget) instead of duplicate canvas"},{"id":"wave2-cad-gumball-panes","content":"Cad: add transform gumball UI, restore quad multi-pane view (shape/building/energy/structure-classic), wire undo/redo via vcs, derive typology geometry from authored object dimensions instead of hardcoded per-typology constants"},{"id":"wave2-draw-geometry","content":"Draw: render real path/fill/boolean geometry and restore interactive drawing tools instead of bounding-box stubs"},{"id":"wave3-puzzle5d-parity","content":"Puzzle5d: port missing commands from puzzle3d (deleteSelection, setFixtureJson, worldVortexHover/Select, worldRelocate, overlap budget, kind weights); render/select grips and fasteners in 2D and 3D views"},{"id":"wave3-puzzle2d-lod-engagement","content":"Puzzle2d: restore multi-pane LOD (overview/detail/selection), engagement REPL input/candidate control/fill-slider, wire suggestion-offset and kind-weight sliders into UI"},{"id":"wave3-gis2d-maphost","content":"Gis2d: wire remaining MapHost capability (render mode, vector style, LOD, feature hit-testing/selection, route editing)"},{"id":"wave3-flow-commands","content":"Flow: port remaining LOD/proximity/catalogue/extension/generation commands"},{"id":"wave3-writer-lsp","content":"Writer: implement real formatDocument; make completions/lint schema-aware instead of running against an empty graph"},{"id":"wave3-shooting-render","content":"Shooting: render real icon/SVG preview instead of placeholder PNG; make export produce a real render, not a generic title-card"},{"id":"wave3-layout-inspector-export","content":"Layout: widen editable inspector (fill/stroke/story-content/margins/columns/link-path); wire exportPng/exportPdf/exportPackage to existing layout/rs/export.rs"},{"id":"wave3-wires-interaction","content":"Reasoning/wires: restore node/wire dragging and live force-layout"},{"id":"wave3-forms-preview","content":"Forms: make Try tab a live form preview; fix required flag to persist correctly instead of being wiped by edits"},{"id":"wave3-imperative-pathref","content":"Imperative: fix addStepAt/removeStepAt/setStepParamsAt to resolve real PathRef instead of PathRef::default(); show full run output instead of 80-char truncation"},{"id":"wave3-vcs-edit","content":"Vcs: implement projection editor 'edit' command handler so typed changes persist"},{"id":"wave4-sequence-toolbar","content":"Sequence: wire footer toolbar (Run/Stop/Reorganize/orientation toggle)"},{"id":"wave4-procedural3d-gumball","content":"Procedural3d: persist gumball transforms into the flow graph; add undo/redo and delete-widget commands"},{"id":"wave4-presentation-image","content":"Presentation: render actual source image behind crop tiles"},{"id":"wave4-note-nudge","content":"Note: fix arrow-key nudge to pass dx/dy args correctly"},{"id":"verify-wave-all","content":"Run cargo test/check and E2E smoke checks after each wave; final full-suite verification"}]
</todos>

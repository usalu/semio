---
name: Restore App Parity Post-Migration
overview: Restore full pre-migration functional parity (React renderer) across all 25 program apps by wiring each Rust WASM program's `render`/`handle_command` to the still-present domain engines and porting the dropped command/interaction surfaces, prioritized by severity, verified with a strengthened functional E2E suite.
todos: []
isProject: false
---

# Restore App Parity Post-Migration

## Root cause (applies to nearly every app)

The Rust Plugin Framework Migration (`.repo/🎫/26/07/04/RUST-PLUGIN-FRAMEWORK-MIGRATION`) replaced each app's old TypeScript `PlayShellController` (rich commands, real canvas hosts, domain engines) with a new `<domain>/program/rs/lib.rs` that only implements a **scaffold**: declarative panels, a handful of CRUD commands, and a simplified scene (`canvas-2d`/`node-graph`/`world-3d`/`table`) that frequently **hardcodes placeholder geometry** (`"box"`, fixed rects, empty pixel buffers) instead of resolving real fixture data. Critically, the actual domain engines from before the migration are often **still present on disk but never called** by the new plugin:

| Engine (still exists)                                | Consumed by new plugin?                                 |
| ---------------------------------------------------- | ------------------------------------------------------- |
| `puzzle/3d/rs` (brush/fill precompute)               | No                                                      |
| `lowpoly/core/rs` (halfedge mesh + paint)            | No                                                      |
| `gis/2d/rs` (MapHost, tiles/routes)                  | No                                                      |
| `cad/kernel`, `cad/renderer`                         | No                                                      |
| `flow/core/rs` (`FlowSession` eval/worker)           | No (program reimplements a subset)                       |
| `sequence/core/rs`, `dag/.../rs`, `trinity/*/engine` | Partially (compiled-DAG only, not full command surface) |

This is the pattern to fix everywhere: **wire the plugin's `handle_command`/`render` to the existing engine**, port the dropped command handlers, and stop emitting placeholder data when real fixture/engine data exists. This is a native Rust re-port, not an adapter/shim (per repo "no compatibility layers" rule) — each plugin's internal structs gain the real fields/logic directly.

## Severity triage (from the 5 parallel audits)

**Tier 1 — Severe (core interaction/domain broken, user-visible immediately)**

- `puzzle3d`: every object renders as a gray box (ignores `meshUrl`/orientation/kind catalog); brush/fill entirely unported (`puzzle/3d/program/rs/lib.rs`, precompute engine `puzzle/3d/rs/lib.rs` unwired)
- `lowpoly`: paint completely non-functional (no paint commands at all); mesh editing commands (`extrude`/`inset`/`bevel`/...) are no-operations; real halfedge mesh from `lowpoly/core/rs` ignored; default fixture emptied
- `flow`: no evaluation/compute pipeline (~90% of old `FlowSession` API dropped); undo/redo dead; catalogue trimmed to 5 widgets
- `sequence`: no nested control/slot/collapse support; no `setStepParams`/`removeStep`
- `procedural3d`: no brep evaluation, hardcoded box preview mesh, examples not registered
- `procedural2d`: wrong domain model entirely (revision counter instead of flow fixture); canvas is two fixed rects
- `gis2d`: not actually a map (hardcoded camera, no tiles/positions/routes, example fixture unused)
- `cad`: no real geometry/BREP kernel wired; typology mesh hardcoded to box/cylinder; play fixtures unused
- `puzzle5d`: brush/fill dropped (same class as puzzle3d); hardcoded box meshes; fixture fields (`grips`, `3d`, fasteners) silently dropped
- `puzzle2d`: ~45+ commands dropped, no tools/LOD/engagement, edges/wires never reach canvas
- `trinity` (jack): Trinity canvas downgraded to generic node-graph, no VCS/LOD, graph query results not rendered
- `trinity-rewrite`: LHS/RHS pattern graphs reverted to raw JSON text editors, cross-panel hover/select bridge gone

**Tier 2 — Major (large functional gaps, less immediately obvious)**

- `dag`: `renameDagNode` referenced but unimplemented (dead UI), no delete/disconnect, only 4/12+ node kinds, no undo
- `raster`: composite view renders **empty pixels** (fixture image data never decoded), no paint/selection tools, masks/filters missing
- `writer`: LSP entirely absent (diagnostics/completions/format/lint all no-operations), AST↔editor sync broken
- `shooting`: icon window renders blank raster; model window ignores GLB URLs (hardcoded box); no export commands
- `layout`: read-only inspector, no export pipeline (PNG/SVG/PDF), canvas is bounding-box stub (no fill/stroke/story content), pointer interaction stubbed
- `reasoning-wires`: entire inherited Puzzle2d engine dropped; relationships/edges never rendered

**Tier 3 — Moderate (structure present, editing/interaction incomplete)**

- `draw`: canvas renders bounding boxes only (not real path/fill/boolean geometry), no pointer tools, no import/export
- `note`: no patch/drag-reorder/undo/nudge, read-only inspector, missing snap/grid fields
- `forms`: no example fixture loading, table-only UI instead of live builder/try-preview, several question kinds unsupported
- `imperative`: nested control body editing (`addStepAt`/`removeStepAt`) missing, undo dead, run output discarded
- `vcs`: no checkout-checkpoint, read-only projection editor, backbone sync unused
- `presentation`: missing `setFrame`/`setActiveExample`/single-tile patch commands, canvas is geometry-only stub

**Tier 4 — Mostly fine**

- `s` (S Studio): studio command surface is essentially complete (navbar handled by separate `restore_old_s_navbar_parity` plan); only minor gaps (`compiledDagEngagementSubmit`, single example id)

**Cross-cutting**

- All plugins register `mod+z`/`mod+shift+z` keybindings but most never implement `undo`/`redo` in `handle_command` — dead shortcuts everywhere.
- `connectMediaPorts` exists in flow/dag/sequence plugins but the WGPU node-graph renderer never dispatches it (out of scope per React-only decision, but confirms React is the right renderer target).
- E2E (`.repo/🎫/26/07/05/SUPPORT-REACT-AND-WGPU-RENDERERS-IN-PLAYGROUNDS/verify-react-playgrounds-e2e.ts`) only asserts "shell visible" / canvas >8px — it cannot and did not catch any of the above, which is why migration "closed" with 25/25 green.

## Execution approach

Per repo rules, each unit of work happens inside its own ticket (`ticket_open`, associated with the appropriate goal from `repo://goals`), with temporary artifacts kept in the ticket folder, and closed with a full summary. Given the scale (24 of 25 apps need real engineering, several requiring a full domain-engine wiring), this proceeds in **priority waves**, each independently shippable and verifiable:

1. **Wave 0 — Verification infrastructure** (do first, unblocks trustworthy signal for every later wave)
   - Extend `verify-react-playgrounds-e2e.ts` per-program with real functional assertions instead of "canvas painted": e.g. non-placeholder pixel/geometry signature checks for 3D/raster/map/canvas apps, a scripted command-dispatch smoke test per app (exercise 3-5 of its real commands and assert document/render state actually changed), and console zero-warnings.
   - This is what prevents another "25/25 pass" false-green.

2. **Wave 1 — Tier 1 apps** (puzzle3d, lowpoly, flow, sequence, procedural3d, procedural2d, gis2d, cad, puzzle5d, puzzle2d, trinity, trinity-rewrite): wire each program to its existing domain engine, port dropped command handlers, replace placeholder scene data with real fixture-driven geometry. Puzzle3d gets the explicitly-requested **full port**: real per-object GLB meshes + orientation from `meshUrl`/kind catalog, and complete brush/fill (placement, candidate cycling, fill slider, voxel paint) wired through the existing `puzzle/3d/rs` `Puzzle3dPrecomputeSession`.

3. **Wave 2 — Tier 2 apps** (dag, raster, writer, shooting, layout, reasoning-wires): fix the specific dead/broken paths identified (raster pixel decode, writer LSP wiring, dag rename bug + node kinds, shooting GLB/icon render, layout export + editable inspector, wires edge rendering).

4. **Wave 3 — Tier 3 apps** (draw, note, forms, imperative, vcs, presentation): restore real canvas geometry rendering, editable inspectors, undo/redo, and the specific dropped commands cataloged above.

5. **Wave 4 — Cross-cutting cleanup**: implement `undo`/`redo` uniformly wherever the keybinding exists but the handler doesn't; re-verify full 25-program suite end to end.

Each wave ends with: `cargo test` for touched crates, program WASM rebuild, and a run of the strengthened E2E suite (Wave 0's output) before moving to the next wave.

## Todos

</plan>
<todos>
[{"id":"wave0-e2e-infra","content":"Strengthen verify-react-playgrounds-e2e.ts with real functional/pixel/command-dispatch assertions per app (not just canvas-painted)"},{"id":"puzzle3d-full-port","content":"Puzzle3d: real GLB meshes+orientation from fixture/kind-catalog, full brush/fill tool port wired through puzzle/3d/rs precompute engine"},{"id":"lowpoly-paint-port","content":"Lowpoly: port paint commands (stroke/fill/layers) and mesh-edit commands (extrude/inset/bevel/etc.) by wiring lowpoly/core/rs halfedge+paint engine"},{"id":"flow-eval-port","content":"Flow: wire program to FlowSession evaluate/worker pipeline, restore widget catalogue, undo/redo, cluster/connect commands"},{"id":"sequence-nested-port","content":"Sequence: restore nested control/slot/collapse support, setStepParams, removeStep, disconnect, reorganize"},{"id":"procedural3d-brep-port","content":"Procedural3d: wire real brep evaluation/preview meshes, register example fixtures, restore selection/gumball/persistence commands"},{"id":"procedural2d-flow-port","content":"Procedural2d: replace revision-counter stub with real flow fixture model, restore generate mode and preview pipeline"},{"id":"gis2d-map-port","content":"Gis2d: wire real MapHost (tiles/positions/routes) from gis/2d/rs, register example fixture, fix undo/redo dispatch"},{"id":"cad-kernel-port","content":"Cad: wire cad/kernel BREP geometry and play fixtures, restore transform gumball and multi-pane quad"},{"id":"puzzle5d-brush-port","content":"Puzzle5d: restore brush/fill tools, real 3D meshes/orientation, 2D shape rendering, dropped fixture fields (grips/3d/fasteners)"},{"id":"puzzle2d-engine-port","content":"Puzzle2d: restore dropped command surface (tools/LOD/engagement/inspector patch), render edges/wires/handles"},{"id":"trinity-jack-port","content":"Trinity Jack: restore Trinity canvas (LOD/ports/force-layout), VCS undo/redo, graph query result rendering"},{"id":"trinity-rewrite-port","content":"Trinity Rewrite: restore visual LHS/RHS pattern graphs and cross-panel hover/select bridge"},{"id":"dag-fixes","content":"Dag: fix renameDagNode handler, add missing node kinds, delete/disconnect, undo/redo"},{"id":"raster-pixel-port","content":"Raster: decode+composite real pixel data, restore paint/selection tools, masks/filters, patchLayer(s)"},{"id":"writer-lsp-port","content":"Writer: wire LSP diagnostics/completions/format/lint, restore AST\u2194editor sync"},{"id":"shooting-render-port","content":"Shooting: load real GLB assets for model view, render real icon/SVG previews, restore export commands"},{"id":"layout-export-port","content":"Layout: restore editable inspector (patchPage/patchFrame), export pipeline, real canvas fill/stroke/story rendering, pointer interaction"},{"id":"wires-engine-port","content":"Reasoning-wires: restore inherited puzzle2d engine surface, render relationships/edges"},{"id":"tier3-fixes","content":"Draw/Note/Forms/Imperative/Vcs/Presentation: restore real canvas geometry, editable inspectors, dropped commands per audit"},{"id":"undo-redo-sweep","content":"Cross-cutting: implement undo/redo handlers wherever keybinding exists but handler is missing"},{"id":"final-e2e-verify","content":"Run full strengthened 25-program E2E suite and cargo test workspace-wide; confirm no regressions"}]

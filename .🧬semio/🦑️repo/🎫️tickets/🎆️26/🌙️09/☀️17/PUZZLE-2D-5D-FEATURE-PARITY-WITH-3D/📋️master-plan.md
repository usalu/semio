# 📋️ Master plan — puzzle 2d + 5d parity with 3d (2026-09-17)

Inputs: `📓️E1`…`📓️E10` (this folder). Reference feature set = E1 (3d, 64 app verbs + clipboard + framework generics).

## Scope decisions (coordinator, opinionated)
- Dimension-neutral 3d features are OWED by both 2d and 5d. 5d additionally owes every 3d-pane feature (its 🧊️3d window is a World3d surface): sun, LOD, projection, grip show/direction, target volumes + volume brush, world relocate, proximity radius, chunk size, contact tolerance.
- 2d has no 3d camera: sun / projection / depth LOD / voxel dims / mesh registration stay OUT. The 2d analogue of a target volume is a **target region** (axis-aligned rectangle constraining fill) with an **area brush** utility — OWED.
- 2d gumball: 3d exposes move + rotate (scale deliberately absent). 2d owes a rotate handle in the board engine (move is the native drag).
- Clipboard: 3d owns `Puzzle3dClipboardJob` since 09-13 → 2d and 5d owe `copy`/`cut`/`paste`.

## Wave 1 — execution fleet (Opus 5), all launched together
| id | artifact | slice | owns |
|---|---|---|---|
| 5A1 | 5d | migrate WindowConfig/Config-lane `BatchOnlyPendingRewrite` verbs (cameras, grid, LOD, sun, suggestion offset, contact tolerance, kind weights) | their command files + registry rows |
| 5A2 | 5d | migrate Artifact-lane verbs (add*, fastener CRUD, patch*, transform, setActiveExample, applyBoardEvents, worldRelocate, proximityConnect, registerBrushMesh, selectSameKind, focusSelection) | their command files + registry rows |
| 5B | 5d | Fill as a first-class Tool (`🛠️tools/🪣️fill`), host verbs proof block (`setActiveTool`/`setActiveUtility`), `engagementRepeatLast`, abort, brush placement story | tools dir, build_tool_run_job |
| 5C | 5d | panels: per-entity inspector (InteractionView threading), settings panel, kind-row inference fallback (catalogue + fill), display labels + numbering, outliner hide/lock | `📌️panels/*` |
| 5D | 5d | export/import/open-import fixture, clipboard job, add-part dialog with live kinds, context menu audit | new command dirs |
| 5E | 5d | window options: grid group, selectable kinds, grip show/direction, projection, LOD trio, hover threading in both panes, cross-pane selection law | `☑️options/*`, windows |
| 5G | 5d | target volumes + volume brush (mutations, schema, fixtures, second implementation, commands, utility, fill bridge arm) | `🧬️schema/🧬️mutations/*target-volume*` … |
| 5F | 5d | boot chain (Nx targets, port, launch seed), serve supervisor, Playwright battery | ticket folder + os-dev (after E4) |
| 2A | 2d | battery failures (E7): engagement text normalizer, fill = one history edit, blank-pane descriptor sync recurrence | framework Action line, fill commit, board host |
| 2B | 2d | hover paint, suggestions popup family reachable from the context menu with hover-preview, shift+tab back-cycle | modes/edit, brush commands, Board2dHost popup |
| 2C | 2d | outliner hide/lock, display labels + numbering, grid visible, selectable-kind filter, settings steppers, add-node dialog, `engagementRepeatLast`, tool-switch state clearing | panels, options |
| 2D | 2d | clipboard job, `createEdge` verb + engagement `connect`, proximity auto-connect on drop + radius/overlap settings | commands |
| 2E | 2d | board engine rotate handle (gumball) + `Board2dHost` wiring + probe vitals (`data-window-instance-id`, guest selection, status, interaction) | `♾️infinite/🎲️board`, Board2dHost |
| 2F | 2d | target regions + area brush (mutations, schema, fixtures, python second implementation, third-party oracle, commands, utility, fill constraint) | schema/mutations |
| 2G | 2d | battery expansion to the full 3d matrix (E6 §2) + probe fixes (window-scoped Actions, fault regex) | 2d ticket probe |

## Build discipline
Executors: source edits + `CARGO_INCREMENTAL=0 cargo check -p <crate> --features component-app-assembly` (native, then `--target wasm32-wasip2`) and filtered unit tests, FOREGROUND only. Main session alone runs Nx activate / component builds / serves / batteries.

## Wave 2 — integrate (main): compile all three crates, descriptor regen, publication-authority audit, activate 2d/3d/5d, batteries, fix loop (fresh Opus slices per red lane).
## Wave 3 — Sonnet audits of the landed diff (lanes, registries, silent no-ops, AGENTS.md conformance), final batteries, ticket close.

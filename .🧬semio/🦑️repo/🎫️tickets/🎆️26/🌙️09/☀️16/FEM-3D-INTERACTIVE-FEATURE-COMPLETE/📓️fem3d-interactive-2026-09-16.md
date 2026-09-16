# 🏗️ Fem 3D interactive feature complete — report (2026-09-16)

Session ⚪9dc2b27f (Opus 5), single session, no fleet. Repo MCP down all day → ticket bookkeeping on disk. Blueprint: `FEM-2D-INTERACTIVE-FEATURE-COMPLETE` + `FEM-2D-TRANSFORM-GUMBALL` (same day).

## What landed

- **Schema / engine.** `FemSolid.axis: FemAxis` (X/Y/Z extrusion; `to_world`/`from_world`), so a wall is drawn in ELEVATION with window/door holes and a gable roof is its chevron section swept along the ridge. `⚙️engine/🧊️3d/🕸️meshing` rewritten around a document-wide `NodeMerger` (mesh points within 1e-6 m of a registered node reuse its id → touching solids share interface nodes and solve as one structure), `mesh_solids`/`resolve_geometry`, area loads on upward faces. `crate::mesh::triangulate` lattice at `max_edge/√2` with exact edge subdivision so conformity is a drawing rule (0.5 m module ↔ `mesh_size = √2/2`).
- **Four mutation kinds** `replace-node`, `replace-load`, `change-load-case-name`, `replace-combination` — leaf + diff + inverse + schema facets, and the whole oracle/harness protocol: 22 committed scenario bundles with per-scenario Rust tests, subset + `✳️any` differential harnesses (Rust/feature/Python), oracle catalogs/manifests, JSON-carrier corpus (26 kinds, reader re-measured 26/26 both ways), PyNite mutate-then-solve benchmark (4 after-models + references), schema catalog. Generators: `🐍️fem3d-mutation-vectors.py`, `🐍️fem3d-mutation-harnesses.py` (idempotent).
- **Editor** on the framework interaction domain (`fem3d`, nine granularities, raw entity ids): shared `🎬️scene` node (box/cone kinds + solid surface meshes, every instance carries `interactionGranularityId`, load glyphs redirect onto their load), model/results windows with persisted configs (camera, result display, playback, gumball flags), 36 commands (9 `patch*` → replace mutations, `setResultAnimation`/`resultAnimationTick`, `focusEntity`, `translate/rotate/scaleSelection` coalesced per drag, `setTransformGumballFlag`, `transformBegin/End` host brackets), three panels (paged artifact tree, inspector forms per kind, results transport/display/analysis), transform utility with options rail, keybindings.
- **Framework `World3dHost`.** Instance pick/hover/marquee resolve `interactionGranularityId` per record (a member selects an `element`, a load glyph its `load`); `WorldSelectionRecord.gumballLiveDispatch` → incremental `translate/rotate/scaleSelection` dispatch mid-drag (latest pose wins, one in flight, tail delta + `transformEnd` on release), the layer skips its local preview so instances draw the document.
- **Framework plugin lane.** The retained window-config lane now carries `coalesce_key` (`window:{id}:{key}`) like the non-retained path; maintenance stage 25 drains the config-lane stores' (app config, draft, every window-config partition) displaced-owner queues. Without both, a results-window animation died after ~2 s (64-item ledger) and, once coalesced, after ~340 frames (1 024-slot retirement queue).
- **Solver.** `sparse::ldlt_factor` is an envelope (skyline) LDLT bit-identical to the checkpointed `LdltJob`; `analyses::DofMap` is hash-indexed, `positions_of` goes through a node-slot index. House solve: 101 s → 9 s (debug build; assembly stepping 3.5 s, LDLT 3.8 s).
- **House example** `📚️examples/🏠️house` (`build()` generator, asset regenerated with `SEMIO_FEM3D_WRITE_HOUSE_ASSET=1`): 6 × 8 m raft, ground + attic slabs, four masonry walls with windows and a door, C24 gable roof, 63 raft supports, dead/live/snow cases, ULS/SLS. 6 408 tets / 1 939 nodes, ONE connected structure (union-find law), balanced reactions, slabs and ridge sag as expected, ULS superposes linearly.

## Verification

| Gate | Result |
| --- | --- |
| `📜️nextest-fem3d-editor.sh` (feature-enabled, `--skip long::`) | run 8: 1111/1113 → fixes → runs 12/13/19 green on the touched subsets; final full run in `🗑️generated/nextest-fem3d-editor-final.txt` |
| fem-2d `sparse::` + `analyses::` (envelope LDLT, DofMap) | 85/86 fundamental (the `long::` subspace test passes under `--profile long`) |
| framework plugin `window_config` / `publication_retirement` | 7/7 |
| renderer react vitest `engine-contract` (new instance-target laws) | 3/3 (filtered) |
| `fem-plugin:describe` + `plugin-registry:generate` | green ×2 (36 tools, house example) |
| `activate-fem3d-react-dev` (wasm32-wasip2) | green ×4 |
| PyNite / SciPy / scikit-fem oracle references | `🔨️run-fem3d-oracle.py`: 33 + 5 + 1 scenarios, old 29 unchanged to 1e-9, 4 new committed |
| Browser (react lane 6087, `🐍️fem3d-panels-probe.mjs` + built-in pane) | boot clean; tree pick → `selectedIds:["n00_l1"]`; inspector auto-opens, Z = 3.05/3.4 → `patchNode`, tree/model/results update (`results solve #k` increments); Transform arms the gumball (`gumballActive`, `gumballTarget`, options rail toggles reach `setTransformGumballFlag`); a drag dispatches incremental `rotateSelection` steps mid-drag; Play animates the results window (`phase 0.53 → 0.22 → 0.01` across 11 s, no ledger fault); House loads and solves in ~11 s in wasm |

## Known limits (recorded, not hidden)

1. The mounted live-visual numerical child refuses the demo at the 4 KiB `MOUNTED_OWNER_PAGE_BYTES` (48 analysis nodes × 6 dofs × 32 B) with the engine's `Singular` wording — pinned by `fem3d_production_numerical_child_refuses_the_demo_at_the_mounted_owner_page`; the results window solves through the synchronous path. Framework cap (memory `project-fem-mounted-session-runtime-laws`).
2. Gumball drags were verified in the built-in pane; the headless probe cannot aim at a 3D handle (no world→screen projection), so `translateSelection` is proven headless through the inspector `patchNode` path and the framework unit laws.
3. House solve in wasm ≈ 11 s per revision; a live drag on the house re-solves per step at that cost. Next hotspot is the assembly job's paged stepping (3.5 s debug), not the factorization.
4. `bun ./📜️script.ts typecheck` of the renderer react package has 849 pre-existing errors (peer churn); none in the touched hunks.

## Framework findings

- Instance picks used the literal `"object"` granularity (gis/energy windows declaring another `domainGranularityId` were mis-picked) — fixed per record.
- `transformBegin`/`transformEnd` must be declared by every app that arms the host gumball (the shell refuses undeclared window actions).
- Retained window-config lane dropped `coalesce_key`; no maintenance stage drained config-lane displaced owners.

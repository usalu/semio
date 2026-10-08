# 📓️ BIM Plugin — Coordination Log

## Wave R1 — exploration (Haiku), complete 2026-10-08
`r1-explore-{plugin-anatomy,artifact-mutations,inferences,editor-viewer,tests-tooling,io-persistence,concurrent-state,aec-geometry}.md`.
Decisions: plugin `🏙️bim`, artifact `🏢️model`, crate `semio-s-artifact-bim-model`, hub `semio-hub-bim`; template = shooting;
derived values only via `ModelInference` (`InferredField` DAG storey → wall → opening); geometry via framework crates
(`semio-framework-geometry`, `-2d`, `-3d`, `-mesh-engine`), missing domain-neutral primitives added to the framework;
existing `🌊️flow/🧩️extensions/🏗️bim` and `📐️cad/🧩️extensions/🏢️aec-building` left untouched (follow-up: consolidate).

## Wave F — foundation (Sonnet), launched 2026-10-08
| Label | Scope | Report |
|---|---|---|
| f1-foundation | plugin skeleton, artifact crate, snapshot, diff algebra, golden leaves, storey/wall inferences | `r3-exec-f1-foundation.md`, `r3-golden-leaf.md` |
| r-registration | hub composition, bridge, taxonomy, launch entries, plugin oracle manifest | `r3-exec-r-registration.md` |
| g-geometry | framework geometry inventory + missing primitives | `r3-geometry-inventory.md`, `r3-geometry-api.md`, `r3-exec-g-geometry.md` |
| o-oracles | python env (ifcopenshell, shapely), oracle recipe, first inference oracle | `r3-oracle-pattern.md`, `r3-exec-o-oracles.md` |

Recipes (Haiku, parallel): `r3-recipe-ui.md`, `r3-recipe-io.md`.
Build gate: `🚦️gate.sh` (Windows; tasklist-based, 4 slots, < 12 rustc).

## Wave F results
- f1-foundation: DONE — crate builds native + wasm32-wasip2, 217/217 lib tests; recipe `r3-golden-leaf.md`; datum rule implemented.
- g-geometry: DONE — bulge segments, loops, triangulation, TriMesh builders (walls with openings, curved walls, sweeps), plane section in `semio-framework-geometry`; region booleans + offset in `semio-framework-2d::regions`; oracles kurbo/parry3d/three/shapely green. Follow-up: register vitest/pytest oracles as nx targets.
- o-oracles: DONE — ifcopenshell 0.8.4.post1 + shapely 2.1.2 in .venv (additive) and declared as oracle host packages; recipe `r3-oracle-pattern.md`; cases 🪜️infer-bim-1-levels-and-wall-heights, 🧊️infer-bim-1-wall-solids green. Follow-up: `setup deps` should install the test group (`uv sync --group test --frozen`).
- r-registration: running.
- Build infra: shared cargo cache deadlocks → gate now assigns 6 warm per-slot build dirs `⚡️cache/cargo/{build,target}-bim-<slot>` (new layout + fine-grain locking off).
- f1 touched `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (`close_typed_operation_cursor` field) because HEAD did not compile.

## Wave M / I / U / X — launched 2026-10-08 (19 Sonnet agents)
m-materials-layers, m-profiles-openings-types, m-context, m-walls, m-frame, m-horizontal, m-openings-stairs,
m-railings-spaces, m-multi-data, i-walls, i-openings, i-solids-walls, i-solids-rest, i-spaces-quantities,
i-plan-diagnostics, u-editor, u-viewer, x-ifc, x-examples. Pending launch: u-tools (after M), x-gltf-svg (after solids).

## r-registration result
DONE (partial): hub `semio-hub-bim` (ports 6302/6402) compiles native + wasm, descriptor generated, bridge lists mutations,
taxonomy rows added, hub bootstrap/test-budget/nextest/ownership rows. Blocked/unverified: hub tests (stdio-contract
mid-conversion by DIFF-ONLY ticket: `ApplyCapability` missing), registry `check` red repo-wide (67 stale descriptors,
pre-existing), Nx graph duplicate project (dashboard fixture). launch.json files were deleted by the dashboard migration →
playground registration is now `🚀️playgrounds.json` (generated) → dashboard `playground:bim` → Nx `dev-bim-react-dev` on
`@semio-tech/framework-os-dev`. TODO (wave R2): regenerate the playground catalog including bim without dropping other
plugins; verify `semio run playground:bim --detach --wait-ready`; run hub tests once stdio compiles.

## Wave M / I results (2026-10-08)
All 9 M slices DONE (≈100 leaves, sum-law tests, fixtures, en+de, x-semio-ui). I: storey-levels, wall-layout (joins),
curtain-layout, opening-frames, element-solids (walls/curtain/fillers + columns/beams/slabs/roofs/stairs/railings), bodies,
spaces, quantities, stair-runs, plan-linework, diagnostics DONE with shapely/ifcopenshell/three oracles.
Full lib at last count: 3110 passed / 15 failed (editor/viewer/ifc in flight).
Out-of-scope edits by the BIM fleet (owners to review): `🧰️framework/.../🔌️plugin/🦀️.rs` (close_typed_operation_cursor),
`💻️os/🎚️config/🧬️schema/🧬️mutations/*` (Set* variants), `🗄️stdio/📇️registry/🧬️contract` (3 apply_diff),
`🗄️stdio/🗿️artifacts/{💾️binary,🔤️txt,📐️step,🏗️ifc}` (~30 apply_diff call sites).
Fix-wave items so far: remove `rehost-opening` (move-opening re-hosts with validation; split-wall inverse → move-opening);
oracle generator regex broken (`r3-f1-gen-oracle.ts`); verify delete-storey cascade covers all collections; wall-layout
O(n²) `layout_of`; straight-skeleton roofs for concave/curved footprints; curtain panels cut by hosted openings; stair
stringers/nosings + railing profiles as authored parameters; run feature/oracle generators last; platform subject/parity
roles for all infer cases; registry regeneration (playground:bim); hub tests.

## Disk incident (2026-10-08 ~11:20)
C: hit 0 bytes free (whole fleet ENOSPC). BIM build slots had grown to 56 GB (6 × 6.5–14 GB). Deleted slots 4–6, gate capped
at 3 slots (slots 4–6 lock dirs kept so stale gate loops cannot reuse them) → 23 GB free. Other tickets' `target-fleet-*`
dirs not touched.

## Audits
- `r5-audit-inferences.md`: 29 findings (9 P1). Core claims hold (no derived state stored, dep_input honest, parametric
  tests). P1s = duplicated derivations (layout/host/top/validity recomputed in consumers), silent hole loss + quantities
  mismatch, no production per-entity cache, three inference access paths. Plan: single model-graph InferredField with typed
  node keys (storey → wall layout → host → opening → solids/spaces/plan) so consumers get real cross-entity parents; one
  shared session for editor/viewer/export.
- `r5-audit-mutations.md`: R8–R16 clean on 88 leaves; 88/88 sum-law; catalogue complete (+place-elements, +rehost-opening).
  High: element ids unique only per collection (cross-kind collisions); single deletes orphan properties/classifications.
  Medium: 17 docstrings with literal `\u{…}` escapes; 14 set-* leaves include unchanged fields; 8 whole-list replaces
  (layer stacks/phases/paths/loops — semantically owned lists, review); `PropertyValue` untyped vs design typed enum.

## x-ifc: DONE — IFC2x3 export (ifcopenshell-validated, volumes agree) + import, round trip byte-stable. Open: `io()` not
read by `declaration()`; quantities from layouts not `🧮️quantities`.

## Wave U / X / Z1 results
- x-examples DONE (house 30 walls/35 openings/12 spaces, office 96 columns/152 beams/8 curtain walls; both replay through
  mutations: 157 / 448 ops). x-gltf-svg DONE (three GLTFLoader + lxml/shapely oracles) but with LOCAL writers → z-codecs.
- u-viewer DONE (+ shared `🖌️render` module). u-tools DONE (all tools + hotkeys, parametric e2e test). Both blocked in
  mounted-app tests by a close stall (terminal-empty witness never reached once a window instance is rendered/addressed;
  affects editor and viewer). u-editor still debugging it.
- z-mutations DONE: cross-kind id uniqueness, data cleanup on deletes, rehost-opening removed, sparse set kinds, typed
  PropertyValue, generator fixes, no engine calls in diffs. Lib: 3511 passed / 15 failed (editor close stall).

## Wave Z2 — launched
z-graph (inference model-graph + incremental session), z-codecs (stdio json/xml migration, glTF/SVG on stdio writers,
io() in declaration(), IFC quantities from inference), z-registration (playground catalog, dashboard run, hub tests,
bridge, taxonomy, platform subject/parity, mutation oracle, geometry oracle nx targets, python test group),
z-depth (stair/railing/opening/storey authored params, straight skeleton in framework, beam top_offset).
Next: z-consumers (editor/viewer/export on the model session, section inference) after z-graph; close-stall root cause.

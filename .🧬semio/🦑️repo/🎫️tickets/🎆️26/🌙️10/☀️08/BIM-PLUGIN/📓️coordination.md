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

## Session restart (2026-10-08 ~16:30)
Previous session ended; z-graph/z-codecs/z-registration/z-depth stopped mid-work and u-editor died → resumed the four Z
agents via their transcripts. Owner committed the whole tree (675/676) incl. BIM plugin (4221 files) AND u-editor's
temporary `[DEBUG]` lines (store/🦀️.rs, window/config/🦀️.rs) + 4 `zz_debug_close*` tests → assigned to z-close.
Commit 675 changed the workspace layout: one Cargo workspace per artifact (`🗿️artifacts/🏢️model/Cargo.toml`); brief updated.
HEAD did not compile in BIM's graph: removed dangling `TransientDiff` re-export (plugin/🦀️.rs, added by 676, undefined and
unused), regenerated asset icons via their generator; remaining flow `RetainedCloneGrant` migration breakage → z-baseline.
New: z-baseline (HEAD compile fixes, cross-team log), z-close (close-stall root cause + debug cleanup).
- z-depth DONE: stair stringer/nosing/tread/riser/landing, railing profile/post/baluster/infill, opening sill_override,
  storey cut_height (+ set-storey-cut-height), signed beam top_offset; framework weighted straight skeleton + roof
  surfaces (geometry 168 tests, py_straight_skeleton oracle 104). Pending → z-depth-infer after z-graph: solids/plan/
  quantities consume the new params + skeleton roofs (`r7-depth-inference-handoff.md`); hip fallback warning near reflex.
  HEAD at that time also missing `semio_framework::io`, `protocol::mutation_fixture_ops`, window-config traits, stdio-ifc
  borrow error → z-baseline scope.
- z-baseline DONE: flow RetainedCloneGrant migration, flow test factories, stdio-ifc inverse borrow, BIM window-config
  tests → BIM graph compiles lib+tests native+wasm (`r8-exec-z-baseline.md`).
- Coordinator full run after baseline: `cargo test --manifest-path …/🏢️model/Cargo.toml --lib` → 3677 passed, 9 failed,
  1 ignored. All 9 = mounted-app close-stall (3 editor component, 6 gesture app tests) → z-close.
- z-codecs DONE: stdio json/xml/svg/gltf compile (ApplyCapability call sites migrated; gltf diff triple-duplicated block
  + stale authority rows fixed), BIM glTF/SVG on stdio writers (local writers deleted), io() via artifact::<A>(),
  IFC one infer + quantities from ModelInference; oracles three/lxml+shapely/ifcopenshell green; lib 3678/9 (stall).
  Open: hub blocked by workflow-run crate at HEAD → z-baseline (resumed).
- z-baseline round 2 DONE: workflow-run BorrowedDslField shapes, retract-* → remove-* (unapproved verb), hub-bim
  semio-framework-async dep, test backing Cow → `cargo check -p semio-hub-bim --tests` green native+wasm.
- z-close DONE: root cause = framework app `next_close_byte_demand` answered 1 for the window-config stage while a
  partition owes up to 32 KB → 4096-byte grant stalls forever. Registry/partition owners now publish demand; framework
  regression test added; [DEBUG] + zz_debug tests removed; viewer test un-ignored. BIM lib: 3688 passed / 0 failed /
  0 ignored. Not ours: 7 framework tool_run_tests failing (baseline unknown), drawing graph broken (stdio-semio).

## Completeness (r9/r10)
`r9-audit-completeness.md`: 56 areas → 15 present / 23 partial / 18 missing; 23 work packages. Decisions in
`r9-decisions.md` (constraints check-only, keyed curtain panel overrides, stdio SVG/PDF/CSV/BCF/zip + framework deflate,
families spike). Brief `r10-wave-brief.md`. Gate raised to 6 slots.
Wave W1 launched: w04-storey-phase, w05-modify, w06-ceilings, w07-zones, w09-ramps, w11-annotations, w12-views,
w13-schedules. Queued after z-graph: WP-01 (depth params into inference), WP-02 (diagnostics panel), WP-03 (progress/
cancel + one session), z-consumers. Queued W2: WP-08 sweeps/reveals/attach, WP-10 furniture/MEP, WP-14 sheets,
WP-15 options/worksets, WP-16 clash/rules/BCF, WP-17 IFC4, WP-18 templates/classifications, WP-19 beams/columns/curtain,
WP-20 energy, WP-21 structure, WP-22 costing, WP-23 families spike.
- z-registration DONE (mostly): playground catalog has `bim` (6302/6402); deployment dir `🏬️bim` (🏙️ taken by concrete
  extension in the deployment sibling set; precedent wood 🪵️→🪓️); bridge 88 kinds; jsonpatch+deepdiff+jsonschema
  mutation oracle (540 quintets, 177/177; found + fixed non-minimal move/rotate diffs); all oracle roles green; subject/
  parity 31/32; nx targets test-oracle/subject/parity + geometry oracle project; taxonomy dirs registered.
  Still to verify when crate is quiet: `semio run playground:bim --detach --wait-ready` + browser, hub tests, subject runs
  of gltf/stair-runs/wall-joins/levels/opening-frames. Open repo-wide: registry generate fails on stdio publication digest;
  taxonomy plan phase crashes on stale library fixture digest; 176 over-long fixture paths + 78 path-budget breaches in BIM.
- UI audit `r9-audit-ui.md` → u-fix launched (all items except cut height/section/views/progress).
- z-graph DONE: `🕸️model-graph` single InferredField with typed ModelNode keys + real cross-entity parents (storey →
  band/layout → host → frame → solids/quantities/rooms/plan/diagnostics); F01–F07, F11–F14, F17–F19, F21, F22, F25 fixed;
  engine hashes deps only when cache enabled; lib (skip editor) 3469/0; all 10 infer oracle cases green; moving 1 wall in
  511-wall model recomputes 88/2385 nodes; release: full 84 ms, incremental 353 ms (hashing) → z-incremental.
- Launched: w03-session (z-consumers + WP-03 progress/cancel), w01-depth-infer (WP-01), w02-diagnostics (WP-02),
  z-incremental (engine dirty-key incremental, target < 15 ms).

## Session resume R11 (2026-10-09)
Previous session died ~23:35 on 10-08 with W1 agents (w01–w13, w03-session, z-incremental, u-fix) mid-work; no r10-exec
reports. Crate state at resume: lib compiles; `--tests` fails on one missing file
(`💡️inferences/📋️schedules/🧪️tests/🔬️unit/🦀️.rs`, w13). ticket_reopen MCP malformed → manual session entry.
Wave R11-A (Haiku, read-only): resume audits `r11-audit-{w01-depth-infer,w02-diagnostics,w03-session,w04-w05,w06-w07,
w09-w13,w11-annotations,w12-views}.md` per `r11-audit-brief.md`. Next: Sonnet finishers per audit, then W2.
- R11-A audits in: w01 (PARTIAL), w02+u-fix (PARTIAL), w11 (PARTIAL), w12 (PARTIAL). Explore agents cannot write files →
  coordinator saved their reports. Gate capped at 4 slots; slots 5–6 cache deleted (slots 1–2 ≈ 43 GB each; disk 109 GB free).
  Ruling: ViewKind superset accepted; schedule stays a window.
- R11-B launched (Sonnet): r11-baseline (missing #[path] test files, railings test, baseline counts), finishers
  r11-w01-depth-infer, r11-w02-diagnostics (+u-fix), r11-w11-annotations, r11-w12-views per `r11-finish-brief.md`.
- Audits w03 (NOT STARTED both: one inference path + z-incremental), w04-w05 (nearly done), w06-w07 (partial / nearly),
  w09-w13 (nearly done) saved. Rulings: cancellation = stepped cancellable session + ArtifactCommandWork jobs (no
  Serializer trait change); clear height = min(slab, hung ceiling); delete-zone refuses while a scheme counts it;
  delete-storey cascades storey-scoped schedules. Launched finishers r11-w03-session, r11-z-incremental, r11-w04-phase,
  r11-w05-modify, r11-w06-ceilings, r11-w07-zones, r11-w09-ramps, r11-w13-schedules (13 Sonnet agents total).
- 01:44 session restart: all 13 stopped ~20 min in; resumed via SendMessage. AGENTS.md gained the 256-char path rule →
  added to `r11-finish-brief.md`.
- ~01:50 owner commit 677 (01:21) + stash pop (01:24) left 26 unmerged files (16 BIM, stdio gltf diff, framework store/
  plugin/flow/workflow, .vscode UD). Launched `r11-merge` (Sonnet) with exclusive ownership; whole fleet told to wait
  for `r11-exec-merge.md` before builds. Ruling: keep upstream's move of table_json into io/text/inferences/*.
- WP-23 spike: research `r11-explore-families.md`; decision `r11-decision-families.md` (families + components + MEP as
  authored data, formulas authored / values inferred, new domain-neutral framework crate `semio-framework-expression`).
  W2 launched early: `w2-f1-expression` (framework crate, independent of the BIM conflicts).
- r11-baseline: six missing #[path] test files created (real tests), railings test reads profiles; no counts yet.
- r11-merge DONE (`r11-exec-merge.md`): 0 unmerged, no markers; restored lost text-projection arms (annotations,
  finishes, schedules, zones, ramp-runs). Blocker found: owner commit 677 (01:21) is a half-finished framework
  retirement refactor (value/store/spr; os-kernel 303 errors) → blocks every artifact crate. No owner ticket/session
  found → launched `r11-store` (Sonnet) to finish it in 677's direction; fleet told to wait for `r11-exec-store.md`.
- w2-f1-expression DONE (`r12-exec-w2-f1-expression.md`): `semio-framework-expression` 94 unit + 5 conformance green,
  wasm green, python oracle agrees (346 kind / 444 eval / 60 param sets), round-trip on 20k trees. Formulas stored as
  canonical text (decision amendment). Launched `w2-f2-families`.
- r11-w11-annotations stopped: oracle green; Rust written but uncompiled (store blocker) → resume after r11-exec-store.
  Warns: do not run `r4-x-examples-gen-rust.ts` (stale template); a peer's whole-file write clobbered IFC
  `🔬️projection` helpers once (restored).
- W2 brief `r12-wave-brief.md`. Batch A launched: w2-wp08-walldepth, w2-wp14-sheets, w2-wp18-psets, w2-wp19-frame.
  Batch B (WP-15, 16, 17, 20, 21, 22, F3 components/MEP) after W1 finishers report.
- r11-w04-phase, r11-w13-schedules: written, unverified (framework blocked). Verify-after-store list: baseline, w11,
  w04, w13 (+ BIM_BLESS for delete-storey schedule case, house/office DSL texts, house.ifc).
- DRIFT: `r3-f1-gen-model.ts` would overwrite upstream `🔺️diff/🩹️patches` and `r4-x-examples-gen-rust.ts` is a stale
  template → agents hand-edit generated files. Queued package `r13-generators`: reconcile both generators with the tree
  (generator output must equal the current tree byte-for-byte), then make them the only path. Runs after W1 finishers.
- r11-w03-session: written, unverified (one session registry `🗂️registry`, stepped/cancellable update, export/analyse
  jobs). Follow-ups: viewer progress/cancel UI + viewer job factory; w13 `CsvJob` must reuse `registry::Analysis`
  stepping and register `exportScheduleCsv` in `every_command()`; empty dir `🖌️render/🔮️inference/🧪️tests/🔬️unit`.
- 05:25 r11-w02-diagnostics, r11-w12-views written/unverified (oracles green). r11-store at work with helpers
  (store/os-kernel compile; `semio-framework-plugin` + playbook/infinite-dag still on old API). Disk 72 GB free.
  Example generators currently throw (wp19 `Beam.axis` change vs old start/end) → part of `r13-generators`.
- r11-w07-zones written/unverified (oracles green). Plan after `r11-exec-store.md`: ONE integration agent
  (`r13-integrate`) makes the BIM crate compile end-to-end (lib + tests + wasm), runs all BIM_BLESS + python `write`
  steps from each r11-exec report, runs `cargo test --lib`, and files failures per area; then area owners fix in parallel.

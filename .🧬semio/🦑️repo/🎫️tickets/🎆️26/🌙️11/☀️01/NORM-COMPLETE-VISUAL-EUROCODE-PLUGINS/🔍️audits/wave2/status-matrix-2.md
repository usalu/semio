# 📋 Status Matrix 2 (Eurocode norm plugins, as of 2026-11-03 00:04 CET)

Read-only audit of EN 1990 to EN 1999 plus the shared crates. No build was run, no git write was made, and no source file was edited. Two builds were still running at the snapshot (`build-en1992/16.txt`, `build-en1998/12.txt`), so their status is open.

Sources:
- Wave-2 reports: newest per agent and version (`en1990-v3`, `en1991-v3`, `en1992-v3`, `en1993-v4`, `en1994-v3`, `en1995-v3`, `en1996-v3`, `en1997-v3`, `en1998-v3`, `en1999-v4`, `catalogue-coverage-2`, `editability-2`, `i18n-2`, `ndp-v3`, `live-scene`, `scene-v5`, `scene-1991`, `oracle-wiring-v2`, `shared-v3`, `build-doctor`, `ingress-blocker`).
- Build logs: `🗑️generated/build-en199x/latest.md` and the newest `<n>.txt` per project (ANSI stripped), `integrate/3.txt`, `mechanics/1.txt`, `fixture-sweep/1.txt`, and `build-runner/runner.log`.
- Grep and find checks in `✏️s/🔌️plugins/📕️norm`.
- Morning baseline: `🔍️audits/status-matrix.md`.

Method notes:
- "Unverified" means the value or code has never run through the compiler or a test.
- Counts from older reports are kept only where no later report replaces them. The `(v2)` tag marks a count from `catalogue-coverage-2` (20:08). Later per-artifact reports traced more ids, and those are noted in the text.
- The registry count is 2762 rows. `ndp-v3` said 2754, so the registry grew after that report.

## 1. Build state (newest log per project)

Classes: `rustc` means the log reached the compiler and produced `error[E…]`. `cargo-pre` means failure in cargo or manifest resolution before rustc. `admission` means the repo source gate refused the build before cargo. `infra` means OOM, SIGKILL, ingress, or cache. `running` means the build had not finished at the snapshot.

| Project | Newest run | Result | Furthest stage | Class | Detail |
|---|---|---|---|---|---|
| `norm-en1990-rs` | build run 14 (23:46) | exit 1 | rustc | rustc (1 error counted, `E0063`) | Source admission passed (469/469). Error: missing field `factor_overrides` in initializer of `En1990Snapshot` (snapshot sqlite io). |
| `norm-en1991-rs` | build run 14 (23:47) | exit 1 | rustc | rustc (`E0080`) | Source admission passed (469/469). Const-eval panic: "borrowed schema declares an unknown unit" in the snapshot crate. |
| `norm-en1992-rs` | build run 16 (00:03) | running | graph/cargo | in progress | Run 15 failed at workspace-member manifest load (`mechanics` missing from workspace deps). Run 16 was in graph inference (30 of 99 projects, 43 failed, 16 restarts). |
| `norm-en1993-rs` | build run 11 (21:34) | exit 1 | cargo-pre | cargo-pre | `failed to get semio-s-plugin-norm-ndp-en1990`. Stale: no run since the `ndp-model` authority fix. |
| `norm-en1994-rs` | build run 11 (22:14) | exit 1 | cargo-pre | cargo-pre | `failed to get semio-s-plugin-norm-ndp-model`. Stale. |
| `norm-en1995-rs` | build run 17 (22:10) | exit 1 | cargo-pre | cargo-pre | `failed to get semio-s-plugin-norm-ndp-en1990`. Stale. Run 16 was SIGTERM (143). |
| `norm-en1996-rs` | build run 12 (23:25) | exit 1 | cargo-pre | cargo-pre | `Invalid Cargo workspace member patterns`. Latest run. Earlier runs were OOM-SIGKILL in graph-plan. |
| `norm-en1997-rs` | build run 14 (22:30) | exit 1 | cargo-pre | cargo-pre | `Cargo dependency lacks authored member authority` (ndp-model Cargo.toml). Stale. |
| `norm-en1998-rs` | build run 11 (22:29), run 12 started 23:48 | run 11 exit 1; run 12 running | cargo-pre (run 11) | cargo-pre; run 12 in progress | Run 11 hit the same ndp-model authority error. |
| `norm-en1999-rs` | build run 10 (22:45) | exit 1 | cargo-pre | cargo-pre | Same ndp-model authority error. Build 2 failed with "Admitted Cargo artifact plan Source changed" (a peer saved a file during compile). Build 3 restarted. Stale. |
| `norm-artifact-contract-rs` (integrate) | run 3 (22:48) | exit 1 | cargo-pre | cargo-pre | `failed to load manifest for workspace member` (ndp package). |
| `s-plugin-norm-mechanics` (mechanics) | run 1 (14:40) | exit 0 | build complete | green | Only green build in the tree. Predates the wave-2 edits. |
| `s-fixture-sweep-rs` | run 1 (23:23) | exit 1 | nx invocation | infra or unknown | Log ends in `Owned Nx invocation failed (1)`. No rustc output. |

Shared-crate gates (from the build logs):

| Crate | Gate result | Evidence |
|---|---|---|
| `structural-scene` | passes admission | `Source admitted: 469/469 semio-s-plugin-norm-structural-scene-meshing` (en1990, en1991). |
| `app-surface` | refused by byte budget (earlier runs) | `Rust crate Source extent exceeded`: `diagrams2d/canvas-lowering/🦀️.rs` 32979 B > 29401 B budget (en1990 run 4). Also `Rust crate Source byte limit exceeded` (en1994 run 3). Passes in the en1990/en1991 runs of 23:46 and 23:47. |
| `app-surface` module | ambiguous module | `Missing or ambiguous Rust module: store_ownership_macros` (en1991 run 3, en1999 build 1). |
| `ndp-model` / `ndp-enXXXX` | authority fixed by hand | `build-doctor.md`: `..` count off by one, fixed. `check-cargo-authority.py` reports 0 errors. Runs after the fix are not in the logs yet. |

Infrastructure failures seen in the logs (not code):
- `build:graph-plan` SIGKILL or OOM: en1990 (run 1), en1992 (run 2), en1996 (runs 1 and 2), en1999 (run 1). Graph budget is 1346 MB (`memory above the budget of 1346 MB`).
- `Original compiler ingress requires the repository launcher` on `cargo -vV`. This is the `:check` route only (`ingress-blocker.md`). The `:build` route passes this point.
- `NX Exact live repository partition cache owner required` (en1991 run 1, en1993 run 1).
- `Bun installation owner is a symlink: CLAUDE.md` (en1990 check 0).
- `build:source-admission` stalled for 1310 s in en1990/14 and 1010 s in en1991/14 before it admitted the sources.

## 2. Matrix per artifact

Abbreviations: `set` = leaves with a typed set-field kind (editability-2). `handle` = leaves with a 3D handle write. `cat-only` = spec ids present only in the catalogue (catalogue-coverage-2, v2). Unless marked "(later)", counts come from the v2 report.

### 2a. Catalogue coverage, editability, and 3D

| Artifact | Spec ids | In document | Cat-only | Missing (v2) | Later trace | Editable leaves set / total | Handle leaves | Non-editable content | Collections w/o insert | 3D scene entities (live wiring) | Diagrams (named new) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| EN 1990 | 29 | 5 | 24 | 0 | `SPEC_TRACE` maps 26 ids (en1990-v3) | 88 / 101 | 16 (15 if `betaComputed` dropped) | 0 | 2 | Pilot: `SceneKit`, `Out::scoped`, `live_scene_of` (en1990-v3, shared-v3) | `envelope-and-targets` (beta targets, RC1 to RC3) |
| EN 1991 | 60 | 10 | 49 | 1 (`E13.13`) | none | 104 / 175 | 53 (17 handle-only) | 39 | 8 | 14 projection units plus placeholder entities (`live-scene`); row-keyed scene (`scene-1991`) | bridge wind map, slender element, roof-step drift |
| EN 1992 | 156 | 3 | 153 | 0 | `reach.rs` routes all 174 registry entries (en1992-v3); catalogue panels only, document cases not plumbed | 111 / 125 | 21 | 1 (`members[].tightness`) | 0 | members, anchors; `stored_placements` (`live-scene`) | `creep-curve`, `fatigue-range`, `strut-and-tie`, `prestress-loss` |
| EN 1993 | 142 | 0 | 0 | 87 (v2) | all 87 traced: 81 checked or kernel, 6 open (en1993-v4) | 461 / 495 | 55 | 0 | 0 | 21 collections (`rows_of`, `Out::own`) | annex diagrams, plate frames |
| EN 1994 | 54 | 42 | 0 | 3 (v2) | C-02, C-50, C-51, C-54 tagged in code (en1994-v3); C-48 and C-54 open per morning | 102 / 109 | 26 | 0 | 3 | beams, columns, slabs; mode track | `slip-distribution`, fire temperature plate |
| EN 1995 | 67 | 1 | 66 | 0 | `TRACE` maps 68 ids, all in document or traced (en1995-v3) | 67 / 75 | 21 | 0 | 2 | members, connections, cases; fire track | Johansen `failure-modes` (one per mode) |
| EN 1996 | 37 | 0 | 31 | 0 (6 function-only) | 11 ids also in document (morning) | 100 / 115 | 35 | 0 | 2 | walls (per-wall unit budget) | `yield-lines`, `arching` |
| EN 1997 | 42 | 26 | 16 | 0 | none | 127 / 146 | 30 | 0 | 2 | 11 collections, `unit-groundwater`, `unit-settlement` | n/r (settlement interaction in checks) |
| EN 1998 | 26 | 0 | 0 | 0 (11 function-only, 15 clause-cited) | SEI ids in `SPEC_TRACE` (en1998-v3) | 173 / 189 | 25 | 0 | 3 | 9 collections; seismic joints; mode and phase tracks | `soil-profile`, N2 capacity |
| EN 1999 | 42 | 16 | 17 (v2 +1, morning) | 2 | `catalogue::ENTRIES` (17 rows) | 134 / 153 | 39 | 1 (`members[].bucklingLengthT`) | 12 | 7 collections; fire, fatigue, sheets, shells; mode track | `sn:<id>` S-N, `fire-field:<id>` |

Notes:
- Editability per artifact is from `editability-2`, using the snapshot schema facet. Its earlier generic fallbacks (`update(collection, row)`, `replace_row`, `update(collection, list)`) are gone. No fallback is reachable from the editor (`EDIT_RULES`).
- Dead code still present in en1998: `replace_all` (no callers) and `UpdateSite` (a whole-record kind not reached through `EDIT_RULES`). `editability-2` says to delete both.
- Collections without insert kind (editability-2): en1999 has 12, en1991 has 8, and en1990, en1994, en1995, en1996, en1997 and en1998 have 2 to 3 each (mostly `cases[].numbers` and `texts`, and `actions`).
- The live scene is wired in all ten artifacts (`live_scene_of` appears in 3 files per artifact, `SceneKit` in 1 file each), per grep. Compile status: none.

### 2b. Placement, drag, and live re-evaluation

| Artifact | Flat `placements` in snapshot (schema property) | Placement kinds (`change-placement-x|y|z|yaw`, insert, remove) | Drag pipeline | Incremental evaluation wired | Scene golden | Latency measured |
|---|---|---|---|---|---|---|
| All ten | yes (by schema walk, property present in all ten) | yes (scene-v5). Selector name differs: `id` (en1990/91/93/94/96/97/98) versus `placementId` (en1992/95/99); value field `newValue` in en1995 | Shared module `📍️placement-drag` exists (grep). Test `a_synthetic_placement_drag_previews_the_complete_row_commits_one_event_and_reloads_to_the_same_pose` exists (1 file). `HandleDragTool` pipeline per scene-v5. Not compiled. | `⚡️incremental-evaluation` exists. `IncrementalNormFamily` is referenced in 20 artifact files. Wired in the EN 1990 pilot. Per-artifact wiring is not measured. | None. No golden file found by name. `SEMIO_UPDATE_GOLDEN` is referenced in 12 files, and every report asks for one bless. | None. `repo bench` or p95 appears in 12 files, but no number has been produced. |

Open risks from scene-v5:
- Standing yaw of a row-less entity is 0, so the first drag resets a derived yaw.
- `dispatch_live_command` (dry run in `lane_events`) still errors on placement leaves that have no row. The result is harmless.

### 2c. Oracles and Rust wiring

| Artifact | Third-party oracle (per wave-2 and morning reports) | Third-party fixture entries (`*third-party*.json`) | Independent-only | Oracle registry exposed (`oracle_wiring`) | Rust adapters (oracle-wiring-v2) |
|---|---|---|---|---|---|
| EN 1990 | scipy (morning) | 21 | n/a | yes (all ten) | n/r |
| EN 1991 | scipy, sympy (en1991-v3) | 59 | n/a | yes | 3 cases added |
| EN 1992 | none validates PR-05 or elastic bar stresses (en1992-v3). Morning: stdlib only | 80 (including 35 + 37 in `third-party*.json`) | n/a | yes | 47 adapters |
| EN 1993 | numpy, scipy, PyNite (morning) | 47 | n/a | yes | 27 adapters |
| EN 1994 | structuralcodes, PyNite, numpy (en1994-v3) | 19 | n/a | yes | n/r |
| EN 1995 | PyNite, sectionproperties (en1995-v3). Morning: stdlib only | 24 | n/a | yes | n/r |
| EN 1996 | scipy, shapely, numpy (en1996-v3) | 9 | n/a | yes | n/r |
| EN 1997 | scipy `dblquad` (en1997-v3) | 40 | n/a | yes | n/r |
| EN 1998 | scipy, groundhog (en1998-v3) | 17 | n/a | yes | n/r |
| EN 1999 | python compliance oracle (stdlib, en1999-v4). Morning: stdlib only | 20 | n/a | yes | n/r |

Notes:
- The fixture counts come from the JSON files named `*third-party*` under `oracle-cases`. They count rows. None of them has a recorded `ok` status, and none has been run in a green build.
- `oracle-wiring-v2` lists the "still unwired" items: en1993 (7 checks needing an FE torsion solver, shared with en1999 and en1994), en1990 (fatigue cut-off, exact beta, beam envelope), en1991 (tandem, LM71, drift), en1997 (anchor elongation, pile group, oedometer), en1995 T classes, and en1996.
- The en1992 known mismatches were routed to the oracle owner (oracle bugs: `t0_adj` omitted, deep-beam `N_Ed` unit). They are not fixed.

### 2d. NDP registry adoption and unverified values

| Artifact | Code references to NDP ids (grep `ndp_*`, `.ndp(`) | UnverifiedBasis or unverified references in code | Registry rows (code) | Rows unverified | Rows verified |
|---|---|---|---|---|---|
| EN 1990 | 20 | 13 | 206 | 206 | 0 |
| EN 1991 | 14 | 38 | 533 | 533 | 0 |
| EN 1992 | 77 (63 local keys, not in registry per morning; 12 rows added in `ndp-v3`) | 23 | 194 | 194 | 0 |
| EN 1993 | 49 | 42 | 328 | 328 | 0 |
| EN 1994 | 17 | 22 | 70 | 70 | 0 |
| EN 1995 | 1 | 4 | 721 | 532 | 189 |
| EN 1996 | 19 | 9 | 246 | 246 | 0 |
| EN 1997 | 3 | 11 | 116 | 116 | 0 |
| EN 1998 | 16 | 21 | 236 | 236 | 0 |
| EN 1999 | 1 | 22 | 112 | 112 | 0 |
| **Total** | 217 | 205 | **2762** | **2573** | **189** |

Adoption status:
- The registry is split into per-part crates (`semio-s-plugin-norm-ndp-enXXXX` and `ndp-model`), and 11 members were added to the norm `Cargo.toml`. The artifact `Cargo.toml` files do not depend on these crates directly. The artifacts go through the compliance facade (`ndp_cell`, `ndp_interpolate`).
- Open adoption gaps:
  - en1993 cites `v-258.1`, `v-376.1`, `v-806.1`, `v-881.1`, `.2`, `.3`, which are not registered (`ndp-v3`).
  - en1992 values equal the code's values, but they are unverified and the NA-DE edition is not checked.
  - en1995 has 17 constants "from memory" in the registry, and 189 rows that are verified.
- The "unverified-value count" column is 2573 registry rows. Constants that never entered the registry were not counted.

### 2e. i18n

| Artifact | Bilingual pairs | Empty EN or DE | Identical EN and DE (cognates) | EN-only UI literal candidates after pair filter | Snapshot leaves without field-meta row |
|---|---|---|---|---|---|
| EN 1990 | 441 | 0 | 6 | 0 | 0 |
| EN 1991 | 973 | 0 | 39 | 3 | 0 |
| EN 1992 | 365 | 0 | 6 | 22 | 0 |
| EN 1993 | 1484 | 0 | 6 | 55 | 1 (`materials[].rambergOsgoodN`) |
| EN 1994 | 385 | 0 | 6 | 0 | 0 |
| EN 1995 | 240 | 0 | 8 | 11 | 0 |
| EN 1996 | 667 | 0 | 6 | 4 | 0 |
| EN 1997 | 625 | 0 | 6 | 1 | 0 |
| EN 1998 | 638 | 0 | 8 | 13 | 0 |
| EN 1999 | 810 | 0 | 8 | 2 | 0 |
| Shared app-surface | 62 | 2 (placeholders) | 2 | 37 | n/a |
| Shared structural-scene | 14 | 6 (placeholders) | 0 | 3 | n/a |

Notes: `Locale::En` value-use in UI is 1 open (`scalar_text`), per `i18n-2`. `Locale::En` appears in 95 files, so the grep count does not measure value-use. No German-only candidates were re-verified. The language keyboard and screen-reader paths were not reviewed in this pass.

### 2f. Build outcome summary

| Artifact | Last build | Furthest stage | Open blockers |
|---|---|---|---|
| EN 1990 | exit 1 (run 14) | rustc (E0063) | Missing `factor_overrides` in the snapshot initializer. Scene golden needs one bless. |
| EN 1991 | exit 1 (run 14) | rustc (E0080) | Schema const-eval panic. The scene crate is at 3990 of 4000 lines (split needed). |
| EN 1992 | running (run 16) | graph inference | Run 15 manifest error (fixed by hand, not yet verified). |
| EN 1993 | exit 1 (run 11) | cargo-pre | ndp dependency error (fix applied after run). Source-boundary split of clauses (28 files). |
| EN 1994 | exit 1 (run 11) | cargo-pre | ndp-model dependency error (fix applied after run). `Cargo.lock` synchronise. |
| EN 1995 | exit 1 (run 17) | cargo-pre | ndp-en1990 dependency error. Schema split of the kernel was requested. |
| EN 1996 | exit 1 (run 12) | cargo-pre | Invalid workspace member patterns. Masonry crate near budget (3860 lines). |
| EN 1997 | exit 1 (run 14) | cargo-pre | ndp-model authority (fixed by hand). |
| EN 1998 | run 12 running | cargo-pre (run 11) | ndp-model authority (fixed by hand). |
| EN 1999 | exit 1 (run 10) | cargo-pre | ndp-model authority. Peer `store_ownership_macros` ambiguity in app-surface. |
| Shared (integrate) | exit 1 (run 3) | cargo-pre | Manifest load (ndp). |
| Shared (mechanics) | exit 0 (run 1) | complete | Predates wave 2. |

## 3. Definition of done (visual-editor-architecture.md section 10)

The spec file is at `📐️specs/visual-editor-architecture.md`, section 10 (lines 740 to 747).

| # | DoD item | Status | Evidence |
|---|---|---|---|
| 1 | Opening any artifact shows outliner, 3D viewport with utilisation colouring, typed properties, checks with bars and clause links, one diagram per governing check, no JSON text | Partial (source), unverified | Viewport, diagrams, properties and checks windows are mounted in all ten. Legacy `inputs`, `results` and `report` dirs are gone from en1990 to en1999 (grep: 0). `render_document_json`: 0 references. The outliner and checks bars have not been run. One diagram per governing check: not verified (named diagrams exist in each artifact, but no coverage count was produced). |
| 2 | Live drag preview under 12 ms p95, one undoable commit, checks under 400 ms p95 | Not met (not measured) | Drag path exists (`📍️placement-drag`, `HandleDragTool`, lane overlay). Incremental evaluation is wired in the EN 1990 pilot, and `IncrementalNormFamily` is referenced in 20 files. No latency number exists. `scene-1991` gives rebuild counts (for example, 1 to 9 rebuilt fragments per edit), not milliseconds. |
| 3 | Every fixture projects to a golden scene validated by independent oracles; incremental equals full | Not met | No golden file found by name. Blessing needs `SEMIO_UPDATE_GOLDEN=1` (12 references). Oracles are partial (section 2c). The incremental-equals-full law test exists (`shared-v3`: "law test is the guard") but has not run. |
| 4 | EN and DE complete; keyboard and screen-reader paths; CVD-safe palettes | Partial | 0 empty EN or DE texts. 152 EN-only UI candidates after pair filter (`i18n-2`). `a11y-devices.md` exists, its results are not re-read here. Palette verification: not done. |
| 5 | Desktop, mobile and tablet layouts from the same windows | Not verified | `DeviceClass::profile()` written (wp07, morning). No build, no screenshot, no test. |
| 6 | Viewer shows the same scene and checks read-only; no editor import in the viewer tree | Source-level only | Morning check found no editor `use` in viewer trees. Not re-run in this pass. |
| 7 | Obsolete code removed (`render_document_editor`, `NormFieldMeta`, global report cache, `document_revision_key`), no compatibility layers | Partial | `render_document_editor`: 0. `render_document_json`: 0. `document_revision_key`: 0 (removed since the morning). `report_cache`: 0 (removed). Legacy window dirs in EN 1990 to 1999: 0. Still present: `NormFieldMeta` in one stale file (`🪟️windows/📥️inputs/🦀️.rs`, app-surface). `SDK GAP` comments in 15 files. Dead `replace_all` and `UpdateSite` in en1998. |

Summary: none of the seven items is met. Items 1, 4, 6 and 7 are partly met in source, and no item has been verified by a build.

## 4. Top 15 remaining gaps (ranked by impact on: norm complete, 3D visual, real-time rich editors)

1. **No artifact has a green compile.** The furthest stage is rustc on en1990 (`E0063`) and en1991 (`E0080`). Every other artifact stops in cargo or manifest resolution, and en1992 and en1998 are still running. Every "done" claim in the wave-2 reports is unverified. (Blocks all three goals.)
2. **Shared `app-surface` gate.** The source-size budget (32979 B against 29401 B in `canvas-lowering`) and the ambiguous `store_ownership_macros` module refuse builds before cargo. Every artifact depends on it. The size gate needs a split, and the module ambiguity needs one owner (`foundation` and `prelude` both declare it). (Blocks real-time editors, which all depend on app-surface.)
3. **Manifest authority and workspace resolution.** `ndp-model` had a wrong `..` count. The fix was applied by hand after the last run of en1993 to en1999 and integrate. The fix is unverified because those runs are stale. (Blocks norm complete.)
4. **Build infrastructure.** Graph-plan OOM or SIGKILL under a 1346 MB budget (five runs lost: en1990 once, en1992 once, en1996 twice, en1999 once). `NX Exact live repository partition cache owner required`. A 1310 s stall in source admission. The `:check` route is blocked by the framework ingress (`ingress-blocker.md`). Only `:build` works. (Slows every goal.)
5. **Scene goldens absent.** No golden file exists. Each of the ten artifacts needs one `SEMIO_UPDATE_GOLDEN=1` bless after the first green build. DoD 3 cannot be met until then. (3D visual.)
6. **Latency gates unmeasured.** The <12 ms drag, <400 ms check and the incremental-equals-full law have no run and no bench number. The drag pipeline exists in source only. (Real-time editors.)
7. **Scene crate size budgets.** `en1991` scene is at 3990 of 4000 lines. `en1996` masonry is at 3860. `en1993` clauses span 28 files. Growth stops until the splits are done. (3D visual.)
8. **NDP registry: 2573 of 2762 rows unverified.** Values "from memory" in en1995 and en1992. Adoption is partial: en1993 cites four unregistered ids (`v-258.1`, `v-376.1`, `v-806.1`, `v-881.x`). Oracle-verified values are a fraction of the registry. (Norm complete.)
9. **Catalogue-only ids with no document path.** en1992 has 153 cat-only ids (its registry is routed only through the catalogue panels, and the document `cases` plumbing is not in the tree). en1995 has 66 (later `TRACE` covers them), en1990 has 24, en1999 has 17, en1997 has 16. (Norm complete.)
10. **Oracle coverage is weak in several artifacts.** en1992-v3 says no library validates PR-05 or the elastic bar stresses. The morning matrix recorded no third-party library for en1992 and en1999, and that was not re-checked here. en1992 and en1999 do have `*third-party*` fixture rows (80 and 20). The oracle fixture rows have no recorded green status. (Norm complete.)
11. **Editability gaps that block typed UI.** en1991: 39 non-editable content leaves and 17 handle-only leaves with no `SetFieldRule`. en1999: `members[].bucklingLengthT` has no kind, `bolts.boltsPerRow` has no rule, and 12 collections have no insert or remove kind (a regression since the earlier report). (Real-time rich editors.)
12. **Collections without insert or remove across artifacts.** en1991 has 8, and en1990, en1994 to en1998 have 2 or 3 each, mostly `cases[].numbers` and `texts`. Action rows have no insert kind. Row-level editing is therefore incomplete. (Real-time rich editors.)
13. **Placement semantics not verified.** Placement selectors differ across artifacts (`id` versus `placementId`). A row-less entity has standing yaw 0, so the first drag can reset a derived yaw. `dispatch_live_command` errors on placement leaves with no row. (Real-time rich editors, 3D visual.)
14. **i18n worklist.** 152 EN-only UI candidates after the pair filter (37 in shared app-surface, 55 in en1993, 22 in en1992, 13 in en1998, 11 in en1995). One open `Locale::En` value-use (`scalar_text`). German-only candidates unverified. (Norm complete, accessibility.)
15. **DoD 7 leftovers and dead code.** `NormFieldMeta` in one stale file, 15 files with `SDK GAP` comments, dead `replace_all` and `UpdateSite` in en1998. Viewport and device layouts (DoD 5) have no test. Desktop, mobile and tablet layouts are fixed fractions. (Real-time rich editors, 3D visual.)

## 5. Changes since the morning matrix

- Wave-2 reports now list `ndp-model` authority errors as fixed by hand, and the `check` route is documented as blocked by the framework ingress.
- Source admission now passes for `structural-scene` (469 of 469) and for `app-surface` in the newest en1990 and en1991 runs. The morning matrix said "Unknown project in the closure" for those crates.
- The first compiler diagnostics exist (en1990 `E0063`, en1991 `E0080`). The morning matrix had none.
- The legacy `inputs`, `results` and `report` window dirs are gone from EN 1990 to EN 1999.
- `document_revision_key` and the global report cache are gone from the source.
- The morning matrix's "kernel registry 520 entries, all unverified" is now 2762 entries, 2573 unverified.
- Catalogue coverage has moved since `catalogue-coverage-2` (20:08): later per-artifact traces cover en1990 (26 ids), en1993 (87 ids), en1995 (68 ids), en1998 (SEI ids) and en1994 (C-02, C-50, C-51, C-54).

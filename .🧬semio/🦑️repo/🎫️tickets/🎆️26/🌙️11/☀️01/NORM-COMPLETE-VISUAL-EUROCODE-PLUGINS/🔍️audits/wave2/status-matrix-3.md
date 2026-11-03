# 📊 Status Matrix 3 (Eurocode norm plugins EN 1990 to EN 1999, rebuilt from code, 2026-11-03)

Read-only audit. No source edit, no build, no git write. Counts come from `🗑️generated/status-matrix-3/matrix.py` (run with `python3 -I`), output in `🗑️generated/status-matrix-3/matrix.json`. Where a wave-2 report disagrees with the code, the code is the reference and the disagreement is listed in section 4.

Scope: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/{⚖️en1990,🏋️en1991,🏛️en1992,🔩️en1993,🧩️en1994,🪵️en1995,🪨️en1996,🌍️en1997,🫨️en1998,🪶️en1999}`, shared compliance `⚖️compliance`, NDP registry `⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json`, spec targets `📐️specs/*.md`.

Reports read: `status-matrix-2`, `claims-verified`, `laws-wiring`, `showcase-docs`, `doc-oracles-v2`, `catalogue-coverage-3`, `en1999-v6`, `kind-convention-v3`, `leaf-kinds`, `live-units`, `handle-laws`, `coordinator.md` (tail), and the newest build logs in `🗑️generated/build-en19xx/`.

Method caveats:
- "Trace kind" = `realisation.kind` in each `🧭️spec-trace/🔣️.json`.
- A check name or design-case entry counts as resolved when its literal appears in the artifact or in `⚖️compliance`. This shows the name exists. It does not prove the check runs.
- "3D for every collection" is a name match against 3D-side files (scene, handles, layout, placement). It is a heuristic, not proof that each collection emits a 3D entity.
- Leaves without a kind are approximate: scalar schema leaves minus `SetFieldRule` count. The rule count is not one-to-one with leaves, so negative values are clipped to 0.
- Shared spec docs (`en1990-en1991-target.md`, `en1997-en1998-target.md`, `en1994-en1999-target.md`) hold ids of several codes. Token counts for those codes carry cross-artifact noise, noted per row.

## 0. Headline

- No artifact has a green `check` or `test` in its newest build log. en1990 and en1991 reach rustc with errors (E0063, E0080). The other eight fail in cargo planning, the NX graph, OOM, or the NX daemon.
- Almost all visual, live, incremental and scene-fidelity work from wave 2 is marked "ready for rebuild" and was never compiled. Every wave-2 report says "nothing compiled or run".
- NDP registry: 2714 of 2903 rows (93.5 %) are `unverified`. The 189 verified rows all belong to EN 1995.
- Spec coverage on paper is high (665 ids in traces, 0 unresolved names). Coverage on the editor, 3D, and design-case side is thinner than the reports claim.
- Several S1 review findings listed as open in `requests.md` are already fixed in code, so the open list overstates the bug count (section 4).

## 1. Build state (newest log per project, `🗑️generated/build-en19xx/`)

| Project | Newest log | Result | Reason from the log |
|---|---|---|---|
| en1990 | 14.txt, 23:46 | exit 1, rustc | `error[E0063]` missing field `factor_overrides` in `En1990Snapshot` (sqlite io) |
| en1991 | 14.txt, 23:47 | exit 1, rustc | `error[E0080]` const-eval panic, "borrowed schema declares an unknown unit" |
| en1992 | 16.txt, 00:26 | exit 1 | graph: 43 of 99 inferences failed, 16 restarts |
| en1993 | 12.txt, 00:55 | exit 1 | NX daemon timeout on REQUEST_PROJECT_GRAPH (about 40 min graph-plan) |
| en1994 | 11.txt, 22:14 | exit 1 | cargo planning (`ndp-model` workspace path, fixed after this run) |
| en1995 | 17.txt, 22:10 | exit 1 | cargo planning (stale, `ndp-en1990` lookup) |
| en1996 | 12.txt, 23:25 | exit 1 | graph partly done, then failed; earlier runs SIGKILL |
| en1997 | 14.txt, 22:30 | exit 1 | cargo authority: `Cargo dependency lacks authored member authority` |
| en1998 | 12.txt, 00:08 | exit 1 | planner SIGKILL (OOM) |
| en1999 | 11.txt, 00:19 | exit 1 | 37 of 72 inferences failed; duplicate Build project name |

Shared gates: `structural-scene` passes admission (469/469 in en1990 and en1991). `app-surface` passed in the 23:46 and 23:47 runs. The Nx and graph infra is the main blocker: 40-minute graph-plan from crate-manifest churn, and a hard-coded 20-minute Nx timeout (`coordinator.md`, 01:15, "INFRA v3" open).

## 2. Matrix per artifact (counts from code)

### 2a. Spec ids by trace kind and trace integrity

| Code | Spec ids (trace) | check | design-case | data | not-modelled | Unresolved check or entry names | Doc ids absent from trace (heuristic) |
|---|---:|---:|---:|---:|---:|---:|---|
| EN 1990 | 29 | 29 | 0 | 0 | 0 | 0 | 1 (E90.13) |
| EN 1991 | 60 | 36 | 24 | 0 | 0 | 0 | 11 (E15.06, E16.04–06, E17.03–06, E17.08–10; same shared doc) |
| EN 1992 | 159 | 60 | 98 | 1 | 0 | 0 | 39 (includes `D-30`, `E-01…`; reconcile) |
| EN 1993 | 142 | 129 | 0 | 5 | 8 | 0 | 31 (`V-029`, `V-170`, `V-255…260`, `V-301…304`, …) |
| EN 1994 | 54 | 51 | 0 | 3 | 0 | 0 | 0 |
| EN 1995 | 68 | 30 | 38 | 0 | 0 | 0 | 4 (`T-36`, `T-67`, `T-68`, `T-110`) |
| EN 1996 | 37 | 0 | 37 | 0 | 0 | 0 | 2 (`M-18`, `M-95`) |
| EN 1997 | 42 | 0 | 42 | 0 | 0 | 0 | 96 (mostly EN 1998 ids in the shared doc; false positives) |
| EN 1998 | 32 | 28 | 0 | 4 | 0 | 0 | 17 (`BR-01…12`, `SEI-31/60/70`, `ST-01/03`, `TW-01`) |
| EN 1999 | 42 | 40 | 0 | 1 | 1 | 0 | 54 (EN 1994 `C-` ids in the shared doc; false positives) |
| **Total** | **665** | **403** | **239** | **14** | **9** | **0** | |

Not-modelled ids: EN 1993 `V-034`, `V-066`, `V-501`, `V-504` (rainflow only in fatpack oracle), `V-505`, `V-506` (fracture), `V-609` (advanced fire), `V-805`; EN 1999 `A-81`.

Data-only ids (kernel functions, no check of their own): EN 1992 `DI-01`; EN 1993 `V-001`, `V-014`, `V-015`, `V-366`, `V-601`; EN 1994 `C-11`, `C-17`, `C-45`; EN 1998 `SEI-03/04/05/42`; EN 1999 `A-01`.

Trace totals changed since matrix 2: EN 1992 156 to 159, EN 1995 67 to 68, EN 1998 26 to 32. Catalogue-coverage-3 used 655 ids from the specs. The trace now has 665. The difference needs reconciling.

### 2b. Typed entity collections, kinds, leaves, handles, diagrams

Collections = top-level snapshot arrays with `$ref` items (`top`), and all such arrays including nested (`all`). Kinds = entries of `KINDS` in `🧬️mutations/🦀️.rs`. Set rules = `SetFieldRule` occurrences. Leaves = scalar schema properties in snapshot `$defs` and top level.

| Code | Collections top / all | Kinds | Set rules | Scalar leaves | Leaves without rule (approx) | Handle fns in scene | Handle literal constructions | Diagram constructions (scene) | Diagram fns | Scene dirs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| EN 1990 | 13 / 15 | 128 | 103 | 115 | 12 | 19 | 1 | 19 | 9 | `scene` |
| EN 1991 | 16 / 20 | 197 | 166 | 183 | 17 | 7 | 4 | 18 | 8 | `scene`, `scene-members` |
| EN 1992 | 7 / 16 | 147 | 118 | 115 | 0 | 4 | 2 | 10 | 10 | `scene` |
| EN 1993 | 26 / 34 | 533 | 466 | 484 | 18 | 25 | 1 | 25 | 11 | `scene`, `scene-members` |
| EN 1994 | 4 / 7 | 116 | 103 | 95 | 0 | 13 | 2 | 21 | 4 | `scene` |
| EN 1995 | 9 / 15 | 145 | 120 | 132 | 12 | 19 | 2 | 23 | 14 | `scene` |
| EN 1996 | 5 / 17 | 141 | 112 | 117 | 5 | 17 | 1 | 8 | 5 | `scene` |
| EN 1997 | 13 / 19 | 166 | 133 | 150 | 17 | 9 | 2 | 10 | 6 | `scene` |
| EN 1998 | 14 / 20 | 247 | 214 | 226 | 12 | 8 | 2 | 28 | 12 | `scene`, `scene-structures` |
| EN 1999 | 14 / 19 | 195 | 158 | 135 | 0 | 19 | 0 | 16 | 7 | `scene` |
| **Total** | **121 / 182** | **2015** | **1693** | **1752** | **93** | | | **178** | | |

Handle and diagram counts are weak proxies. "Handle fns" counts functions with `handle` in the name under scene dirs. "Handle literal constructions" counts `Handle {` only. Handles are often built through helpers, so the two differ a lot (EN 1999: 0 literal, 19 fns). The v2 handle-leaf counts are kept in section 3 as the best available measure of handle-editable leaves.

Editable leaves set and total (v2, `editability-2`): EN 1990 88/101; EN 1991 104/175 (17 handle-only leaves with no rule, 39 non-editable content leaves); EN 1992 111/125; EN 1993 461/495; EN 1994 102/109; EN 1995 67/75; EN 1996 100/115; EN 1997 127/146; EN 1998 173/189; EN 1999 134/153 (`members[].bucklingLengthT` has no kind; 12 collections without insert). These were not re-run in this matrix.

### 2c. Capability checklist

Legend: yes = evidence in code, partial = evidence with a gap, no = not found, unverified = claimed in a report but not evidenced in code.

| Code | 3D for every collection (name heuristic) | Design cases in 3D (cases dir, 3D refs) | Live units with precise reads (read-tracker / `incremental_laws!` lines) | Document oracle with third-party libs | Showcase or demo in palette | Scene, diagram and handle law tests |
|---|---|---|---|---|---|---|
| EN 1990 | yes (13/13) | yes (`scene/cases`, 8 refs) | partial: 1 line, never compiled | yes (anastruct, Pynite, numpy, scipy) | yes (`full catalogue example` in new-document palette) + `demo-session` | yes (2 handle, 2 scene, 1 diagram) |
| EN 1991 | yes (16/16) | partial (`scene-members/cases`, 5 refs; 24 design-case rows) | partial: 1 line, never compiled; no handles dir in `scene` | yes (numpy, scipy only; check-outcome oracle) | partial: `demo-session` only | yes |
| EN 1992 | yes (7/7) | yes (`scene/cases`, 9 refs; 98 design-case rows) | partial: 1 line, never compiled | yes (concreteproperties, sectionproperties, structuralcodes, shapely, numpy, scipy, blueprints) | partial: `demo-session` only | yes |
| EN 1993 | yes (26/26) | no (no cases dir, 0 design-case rows) | partial: 6 lines (most complete), never compiled | yes (Pynite, sectionproperties, shapely, jsonschema, numpy, scipy; 15 oracle files) | yes (showcase fixtures, 19 files: frame portal, annex, special parts, welded variant) | yes (1 handle, 1 scene, 1 diagram, showcase test) |
| EN 1994 | yes (4/4) | no (0 design-case rows) | partial: 1 line, never compiled | yes (sectionproperties, Pynite, numpy, jsonschema) | yes (showcase, 5 files; showcase-docs) | yes |
| EN 1995 | yes (9/9) | yes (`scene/cases`, 44 refs; 38 design-case rows) | partial: 1 line, never compiled | yes (sectionproperties, Pynite, scipy) | partial: `demo-session` + 1 showcase reference | yes |
| EN 1996 | yes (5/5) | yes (`scene/cases`, 7 refs; 37 design-case rows, thin 3D) | partial: 1 line, never compiled | yes (sectionproperties, Pynite, numpy, scipy, shapely) | partial: `demo-session` only | yes |
| EN 1997 | yes (13/13) | partial (`scene/cases`, 4 refs; 42 design-case rows, thin 3D) | partial: 1 line, never compiled | yes (groundhog, numpy, scipy) | yes (showcase, 5 files; `demo` example) | yes |
| EN 1998 | yes (14/14) | no (0 design-case rows) | partial: 1 line, never compiled | yes (groundhog, numpy, scipy; 8 fixtures) | yes (`full showcase` palette test) | yes |
| EN 1999 | yes (14/14) | no (0 design-case rows) | partial: 1 line, never compiled | yes (sectionproperties, Pynite, numpy, scipy) | yes (showcase, 5 files) | yes |

Notes on the table:
- "precise reads" is the read-tracker in `⚖️compliance/⚡️incremental-evaluation`. The word "precise" appears in no code file. The `with_every_collection` fixture trick from `live-units.md` is the only evidence. Nothing has been measured, and the wave-2 reports say so.
- Design cases in 3D: the three codes with no design-case rows (EN 1993, EN 1994, EN 1998, EN 1999) have no design-case 3D either. For EN 1996 and EN 1997 the design-case 3D is evidenced by only 4 to 7 references.
- Showcase rows are from `showcase` and `palette` token counts plus the named editor examples. `demo-session` exists in all ten editors.
- Scene, diagram and handle laws: `assert_handle_laws`, `assert_scene_laws` and `assert_diagram_laws` have at least one call site in every artifact (`laws-wiring.md`). The tests have never run.

### 2d. Unverified NDP rows (shared registry)

| Family | Rows | Unverified | Verified |
|---|---:|---:|---:|
| EN 1990 | 260 | 260 | 0 |
| EN 1991 | 543 | 543 | 0 |
| EN 1992 | 194 | 194 | 0 |
| EN 1993 | 332 | 332 | 0 |
| EN 1994 | 70 | 70 | 0 |
| EN 1995 | 726 | 537 | 189 |
| EN 1996 | 288 | 288 | 0 |
| EN 1997 | 155 | 155 | 0 |
| EN 1998 | 205 | 205 | 0 |
| EN 1999 | 130 | 130 | 0 |
| **Total** | **2903** | **2714** | **189** |

The registry lists 2903 rows (`ndp-registry`, 2762 in that report). The `status` field is `unverified` or `verified`. The "EN 1995 unverified" note in `requests.md` is about its own rows. The registry is the source of truth.

Oracle hygiene found in this audit:
- `Command::new("python3")` still appears 13 times under `🗿️artifacts`. `requests.md` says the twelve spawns were switched.
- `[DEBUG]` `eprintln!` calls remain in `📇️registry/🧬️contract/🖥️app-surface/🧪️tests/⏱️latency-bench/🦀️.rs:102,105`. It is an untracked file, still in the tree.
- 7 directories ending in `-2` or `-3` under `🗿️artifacts`. The AGENTS rule forbids version-suffix names.
- Longest path is 166 bytes, under the 256 limit. No violation found.

## 3. Open correctness bugs from `📌️important/requests.md`

`requests.md` has 579 entries: 150 done and 429 open. About 78 open lines carry a bug marker (S1, S2, wrong, inverted, mismatch). Each S1-class item was checked against the code in this audit.

| Item | Request says | Code now | Status |
|---|---|---|---|
| EN 1992 `helpers-and-annex-parts` `asw` unit factor | `* 1e-3`, should be `* 1e3` | `asw_per_s * 1.0e3` | fixed, request still open |
| EN 1992 `shear_v_rd_c_n` `sigma_cp` | `-n_ed/(b d/0.9)`, should be `N_Ed/A_c` | `-n_ed / (a_c * 1e6)` | fixed, request still open |
| EN 1993 `joint-components` prying | `prying` inverted | `let prying = lb <= l_b_star;` | fixed, request still open; E10 245.96 kN test not verified |
| EN 1993 catalogue HEB 240 `it`, `iw` | It 103.7 cm4, Iw 4.76e-7 m6 | `it: 1.037e-6`, `iw: 4.762e-7` | fixed for HEB 240; HEB 260 to 300 not checked |
| EN 1995 `load-combinations` ContinuousTwoSpan | exact support values | `q l²/8 + 3 F l/16`, `5 q l/8 + 11 F/16` | fixed, request still open |
| EN 1997 `utilization()` capacity 0 | returns 0 (pass) | 0 only if demand is also 0, else 1e6 | fixed, request still open |
| EN 1999 `L_cr` double-k | (fixed in en1999 v6) | explicit lengths used as given | fixed per `coordinator.md`; no test read |
| EN 1992 creep / `5.10.decompression` sign | oracle and Rust disagree | sign fixed per `coordinator.md` (EN 1992 v5) | fixed, verify with oracle run |
| EN 1990 `gamma_G,sup/inf` by effect sign | chosen by action kind, not effect sign | combination generator takes direction (±1) and calls `gamma_g(sup, inf)` | unverified: needs a sign-case test |
| EN 1994 two-span coefficients and `Δσs` uncracked section | wrong coefficients | v7: all continuous internals (UDL and point forces at `xM`, cracked EI2 zones, patterns of EN 1990 6.4.3.2) now come from the shared mechanics `Analysis`, reproduce 9qL²/128, 3PL/16, 11P/16 and agree with PyNite (`e1_18_*`, `🧰️scripts/en1994-continuous-points-oracle.py`); `Δσs` is read on the cracked section (`fire-and-bridge`, test `two_span_cracked_alpha_st_is_the_cracked_ratio`) | fixed by reading, never compiled |
| EN 1996 `f_vk0` material independence and `f_vlt` | same value for all units | `f_bt,cal` now split 0.032/0.026/0.020 by unit group; `f_vk0` table not re-read | partly fixed, unverified |
| EN 1998 Rayleigh period | normalised shape, stiffness-independent | `rayleigh_period_shear_building` added (stiffness-based), but `t1_rayleigh` (displacement form) is still the one called at `schema/🦀️.rs:418` | open: check the caller |
| EN 1998 Table 4.2 `phi` | must be 1.0 for some categories | `phi_table_4_2(category, is_roof, correlated)` exists | unverified |
| EN 1999 `eta` on internal flanges | class 4 flange misclassified | `eta` used in fire-and-fatigue and sheets-and-shells; rule not re-read | unverified |
| structuralcodes creep-shrinkage `t0_adj`, `alpha_cement` | adapter ignores them | zero `t0_adj` hits in `🔮️oracles` and `⚖️compliance` | unknown: name may differ, or still open |
| EN 1994 three blessed cases without an evaluator (`Iw`) | no entry point | the three third-party cross-checks exist as scripts and stdlib oracle vectors; harness adapters are a shared request (`requests.md`, en1994 v7) | open (shared) |
| `[DEBUG]` guard in latency bench | remove or unprefix | still present, 2 lines | open |

Net: about 6 S1 items are closed in code but open in `requests.md`. About 6 are still unverified. Clean up the list before it is used as a bug backlog.

## 4. Reports that disagree with the code

- `status-matrix-2` (EN 1993 `it`, `iw`, prying, EN 1995 coefficients, EN 1997 utilisation, EN 1992 `asw`): fixed in code. The matrix still lists them as open.
- `catalogue-coverage-3` "untraced: none": the doc-token scan finds 11 EN 1991 ids (E15–E17) absent from the trace. Possibly cross-references, but the claim must be reconciled.
- `catalogue-coverage-3` "EN 1992 156 ids": trace has 159. "EN 1998 26": trace has 32.
- `oracle-wiring` claim of 12 `python3` spawns switched: 13 `Command::new("python3")` remain in `🗿️artifacts`.
- `laws-wiring` ("wired into ten scene tests"): call sites exist in all ten. The tests have never run, so "wired" is not "passing".
- `live-units` "precise" reads: the claim is about read-tracker coverage. It is not measured. The 12 ms drag and 400 ms check budgets from `status-matrix-2` are still "not measured".
- `claims-verified` item 1 ("`analyse_thin_walled` not used in en1993 or en1999"): `en1999-v6` says the production caller now exists in en1999 `part_1_1`. Not re-verified here.

## 5. Top 20 remaining gaps (ranked by user impact)

Owners are suggestions by role. Each gap has one concrete next action.

1. **No artifact is green.** Impact: nothing a user runs has been verified end to end. Next action: fix the two rustc errors in `build-en1990/14.txt` (E0063 `factor_overrides`) and `build-en1991/14.txt` (E0080 unknown unit), then run `:check` and `:test` for en1990 alone and get it green. Owner: en1990 and en1991 owners, with the integrator.
2. **Wave-2 visual, live, incremental and fidelity code never compiled.** Impact: most of the 3D and real-time editor claims are untested. Next action: run `:check` then `:test` in dependency order (en1990, then en1991 and en1992), and log the first compile errors per crate into `build-enXXXX/`. Owner: integrator.
3. **Build infrastructure cannot finish a graph.** Impact: no run finishes in reasonable time. Next action: implement INFRA v3 (incremental graph refresh instead of a full 4.6k-project inference; raise the hard-coded 20-minute Nx timeout or make it configurable), then measure one `:check` end to end. Owner: infra.
4. **Two repo MCP servers failed to connect (`repo`, `semio`).** Impact: ticket bookkeeping and the required `repo` commands are unavailable in this session. Next action: restart the `repo` MCP server and confirm `repo://goals` resolves; until then, record state in files. Owner: infra or the user.
5. **93.5 % of NDP rows are unverified (2714 of 2903).** Impact: every compliant verdict carries an unverified basis, so the headline result is weak. Next action: verify NA-DE values family by family, starting with EN 1990 (260 rows) and EN 1992 (194), against the NA text, and record the edition and locator in each row. Owner: compliance owner (shared NDP registry), with each family owner.
6. **Closed S1 bugs still open in `requests.md`.** Impact: the bug backlog is wrong, so work gets repeated or the fixes are not credited. Next action: write one regression test per item in section 3 (EN 1992 `asw`, `sigma_cp`; EN 1993 prying; EN 1995 two-span; EN 1997 capacity 0; EN 1993 HEB 240), then close the requests. Owner: review agent, with the four family owners.
7. **Still-open correctness candidates.** Impact: possibly unconservative results in EN 1990 (γ sign choice), EN 1994 (two-span coefficients, `Δσs`), EN 1996 (`f_vk0`), EN 1998 (Rayleigh caller, Table 4.2), EN 1999 (`η` on internal flanges). Next action: one sign or value test per item, with a hand-checked expected value. Owner: per-family owners.
8. **Oracle adapter bug (structuralcodes creep-shrinkage `t0_adj`) status unknown.** Impact: EN 1992 creep values may be off by about 1 %. Next action: grep the adapter under its real name (`alpha_cement`, `B.9`) and, if it is still missing, add `t0_adj` and re-bless the two cases. Owner: oracle-infra.
9. **Document oracles run in Python only.** Impact: the check-outcome oracles (`doc-oracles-v2`) were dry-run in Python and never through the Rust adapters. Next action: add one Rust test per artifact that spawns the python oracle through `python_oracle::run_ok` and compares to the blessed JSON. Owner: oracle-infra.
10. **Design cases are missing in 3D for EN 1993, EN 1994, EN 1998 and EN 1999.** Impact: the goal ("design cases in 3D") fails for four of ten codes, and EN 1996 and EN 1997 have thin 3D (4 to 7 references). Next action: add a typed design-case collection and a 3D case view for EN 1993 first, using the catalogue-bridge recipe. Owner: catalogue-bridge agent.
11. **Spec rows are not modelled or are untraced.** Impact: about 9 not-modelled ids (EN 1993 fracture, rainflow, advanced fire; EN 1999 `A-81`), plus up to 39 untraced EN 1992 doc ids, 31 EN 1993, and 17 EN 1998 (bridges, silos, towers). Next action: decide per id, either model it or mark it out of scope with a clause reason, and add it to the trace. Owner: spec owner per code.
12. **Handles and editable leaves are not fully wired.** Impact: about 93 scalar leaves have no `SetFieldRule` by approximation, and EN 1991 has 17 handle-only leaves with no rule. Next action: generate rules for the approximate gap per code, starting with EN 1991, EN 1993 and EN 1997. Owner: per-code editor owners.
13. **Live drag latency is not measured.** Impact: the 12 ms p95 drag and 400 ms check budgets are untested, so real-time claims are unproven. Next action: run the latency bench in one built artifact (EN 1990) and record p95 in a report. Owner: app-surface performance owner.
14. **3D coverage is a name match only.** Impact: 121 top-level typed collections are "referenced" in 3D code, but no test asserts each one emits a 3D entity. Next action: add a law test per artifact that fails when a snapshot collection has no 3D entity and no documented exclusion. Owner: scene owners.
15. **Showcase in the palette is explicit in three codes only.** Impact: EN 1991, 1992, 1995 and 1996 offer only `demo-session` in the new-document palette. Next action: add a full showcase document to each of these four editors' palette lists, with a palette test like the EN 1998 one. Owner: per-code editor owners.
16. **`[DEBUG]` and python3-spawn hygiene.** Impact: the repo rules forbid them. Next action: remove the two `[DEBUG]` `eprintln!` calls in the latency bench and switch the 13 `Command::new("python3")` spawns to `python_oracle::command()`. Owner: app-surface contract owner, with oracle-infra.
17. **Version-suffix directories.** Impact: 7 `-2` or `-3` dirs break the naming rule. Next action: rename them and update the references in one pass. Owner: integrator.
18. **Scene law tests have never run.** Impact: `assert_scene_laws`, `assert_diagram_laws` and `assert_handle_laws` are wired into every artifact but nothing has executed them, so a broken scene could pass a laws test by default. Next action: run `:test` once per artifact after gap 2 and fix the first failures (laws-wiring lists the likely ones). Owner: integrator, with scene owners.
19. **Document oracle coverage varies.** Impact: EN 1991 has numpy and scipy only, and EN 1996 and EN 1997 have three to five oracle files. Next action: add a third-party cross-check per code for the three most-used checks. Owner: oracle-infra, with family owners.
20. **Kind counts are not reconciled with leaves.** Impact: `leaves-without-kind` cannot be computed reliably from the code, so editability progress cannot be tracked. Next action: write a generator that prints, per code, the snapshot leaf paths with and without a `SetFieldRule` (replacing the approximation in this matrix). Owner: integrator, with the kind-convention owner.

## 6. Summary per code (one line each)

- EN 1990: most complete trace (29 checks), 3D and design cases present, en1990 build fails at rustc (E0063); 12 approximate leaves without rule; 260 NDP rows unverified.
- EN 1991: 36 checks and 24 design cases; 3D lives in two scene dirs; build fails at rustc (E0080); 17 approximate leaves without rule; 11 untraced doc ids; 543 NDP rows unverified.
- EN 1992: largest trace (159 ids, 98 design cases); 3D and cases present; 39 untraced doc ids to reconcile; `asw` and `sigma_cp` fixed in code; 194 NDP rows unverified; build graph fails.
- EN 1993: largest kinds count (533) and 26 typed collections; no design cases in 3D; 8 not-modelled; 31 untraced doc ids; prying and HEB 240 fixed; build fails in NX daemon.
- EN 1994: 4 typed collections, 51 checks, no design cases; v7: continuous statics on the shared solver, beam/slab action row types split (no dead `yM`), SQLite contract recomputed; three blessed oracle cases without a harness adapter (shared); never compiled (run 12 still verifying the graph at 01:49).
- EN 1995: 38 design cases with 44 refs in 3D; EN 1995 has the only verified NDP rows (189); two-span fixed; build fails in cargo planning.
- EN 1996: 37 design cases and thin 3D (7 refs); `f_vk0` unverified; build fails after graph partly done.
- EN 1997: 42 design cases, thin 3D (4 refs); utilisation capacity-0 fixed; cargo authority error in build.
- EN 1998: 28 checks, 4 data; 17 untraced ids (bridges, silos, towers); Rayleigh caller open; `Table 4.2` unverified; build OOM.
- EN 1999: 40 checks, 1 not-modelled; `L_cr` fixed; `eta` on flanges open; 37 inference failures in build.

## 7. Files

- Scripts: `🗑️generated/status-matrix-3/matrix.py` (run `python3 -I`).
- Output: `🗑️generated/status-matrix-3/matrix.json`.
- Previous matrix: `🔍️audits/wave2/status-matrix-2.md`.
- Requests: `📌️important/requests.md`.

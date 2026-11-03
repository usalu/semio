# 📊 Status Matrix 4 (Eurocode norm plugins EN 1990 to EN 1999, 2026-11-03)

Read-only audit. No source edit, no build, no git write.
Counts come from `🗑️generated/status-matrix-4/matrix.py` (run with `python3 -I`), output in `🗑️generated/status-matrix-4/matrix.json`. Exact leaf and handle numbers come from `🧰️scripts/kind-convention-v2/leaf_coverage.py` and `🧰️scripts/handle-paths-check.py`, run through the same script. Per-tool output is in `🗑️generated/status-matrix-4/leaf.txt`, `handles.txt`, `run.txt`.

Improvements over status-matrix-3:
- Leaf counts are exact (leaf_coverage), not the old approximation. The old `leaves_without_kind_approx` was always 0 and is removed.
- Handle write paths are exact (handle-paths-check), per code.
- NDP verified and unverified counts come from the registry `status` field, not a grep over artifact code.
- Design-case-in-3D is yes/no from design-case trace ids plus case-file references to scene, viewport, handle or diagram.
- Live units are counted by a stricter pattern (`live::`, `live_*`, `LiveView`, `SharedEvaluation`).
- Catalogue feature table rows are counted from the catalogue `.feature` files.
- Build status uses the last `run N` line of each `latest.md`.
- The final print line of matrix-3 was a no-op and is fixed.

## 0. Headline

- No EN 1990 to EN 1999 artifact has a green `check` or `test` in its newest build log. All ten newest numbered logs end in exit 1.
- Content leaves are fully covered by edit rules in all ten codes (0 content leaves without a kind). Handle write paths still uncovered in EN 1992 (9), EN 1996 (2) and EN 1995 (1).
- NDP registry: 258 of 2938 rows verified (8.8 %), 2680 unverified. 255 of the 258 verified rows are EN 1995 (timber classes from EN 338). Eight codes have zero verified rows.
- Design cases in 3D exist in EN 1991, 1992, 1995, 1996 and 1997. EN 1993, 1994, 1998 and 1999 have no design-case trace ids at all. EN 1990 has no design-case ids (combination explorer is in 3D from v6).
- Every code has third-party imports in its Python oracles, and every code has a demo-session example. Showcase fixtures exist only for EN 1993, 1994, 1998 and 1999.
- Law test files are present in all ten codes (6 to 13 per code). None has ever executed, because no build passed.
- Integrate: `norm-artifact-contract-rs:build` is green (run 4, exit 0, 01:42). Mechanics run 2 failed on framework churn after run 1 was green.

## 1. Build state

Newest numbered log per project in `🗑️generated/build-en19xx/`, plus the last `run N` line of `latest.md` (`run N` lines are the authoritative status).

| Code | Newest log | Last run line (latest.md) | Result |
|---|---|---|---|
| EN 1990 | 14.txt, 23:46 | run 14: exit 1, `error[E0063]` missing field `factor_overrides` in `En1990Snapshot` | Fixed in source per coordinator; not re-run |
| EN 1991 | 14.txt, 23:47 | run 14: exit 1, parse error (`bridge_lanes`) and `error[E0080]` unknown unit | Stale; not re-run |
| EN 1992 | 16.txt, 00:26 | run 16: exit 1, Nx invocation failed (124), OOM, re-queued | Infra; 43 of 99 graph projects failed |
| EN 1993 | 12.txt, 00:55 | run 12: exit 1, Nx invocation failed (1), OOM | Infra |
| EN 1994 | 12.txt, 01:54 | run 11: `ndp-model` dependency error; run 12 log still in graph verification (127 of 1071) when last written | Run 12 in progress or lost; no result |
| EN 1995 | 17.txt, 22:10 | run 17: `ndp-en1990` dependency error (cargo) | Stale |
| EN 1996 | 12.txt, 23:25 | run 12: Nx invocation failed (1), OOM | Infra |
| EN 1997 | 14.txt, 22:30 | run 14: Nx invocation failed (1) | Infra |
| EN 1998 | 12.txt, 00:08 | run 12: graph 25 of 52, OOM | Infra |
| EN 1999 | 11.txt, 00:19 | run 11: `Duplicate Build project semio-s-plugin-norm-ndp-en1990` (two `ndp` table paths) | Config, not rustc |

Other gates:
- `integrate/`: run 4 `norm-artifact-contract-rs:build` exit 0 (01:42, 4207 s). Shared contract crates compile. Run 3 failed on a manifest load of `ndp/📦️packages/🦀️rust`.
- `mechanics/`: run 1 exit 0 (0 rustc errors, 14:40). Run 2 (01:20) exit 1, `Compiler-captured admitted Source image identity required` (framework churn). The mechanics crate is not green at the current source.
- `build-runner/runner.log`: last event 01:42. Runner v2 serialises graph work. Build-owners sync passed at 00:34 (`build-owners-sync.txt`, added 102). Later sync failures are recorded in coordinator.md (INFRA v3, 01:45).

## 2. Matrix per code

### 2a. Spec ids by trace kind

`check` / `design-case` / `data` / `not-modelled` are from `🧭️spec-trace/🔣️.json`. "Absent" counts ids that look like the code's shape in the spec doc and are missing from the trace. These are candidates and need review (see gap 14).

| Code | spec ids | check | design-case | data | not-modelled | absent candidates |
|---|---|---|---|---|---|---|
| EN 1990 | 29 | 29 | 0 | 0 | 0 | 1 |
| EN 1991 | 71 | 36 | 29 | 0 | 6 | 0 (EN 1990 E90.xx cross-refs excluded) |
| EN 1992 | 159 | 60 | 98 | 1 | 0 | 39 |
| EN 1993 | 142 | 133 | 0 | 5 | 4 | 31 |
| EN 1994 | 54 | 51 | 0 | 3 | 0 | 0 |
| EN 1995 | 68 | 33 | 35 | 0 | 0 | 4 |
| EN 1996 | 37 | 31 | 6 | 0 | 0 | 2 |
| EN 1997 | 42 | 36 | 6 | 0 | 0 | 96 (regex noise likely) |
| EN 1998 | 32 | 28 | 0 | 4 | 0 | 17 |
| EN 1999 | 42 | 40 | 0 | 1 | 1 | 54 (C-01..C-54, unreviewed) |

status-matrix-3 had EN 1996 at 23 design-case and 14 check. The trace now has 6 and 31, after EN 1996 v6 switched 14 ids to check.

### 2b. Entity collections, leaves, handles, diagrams

Leaves are from leaf_coverage (`content` drops identity `id` leaves). "Set-field" is the number of SetFieldRule paths. Content leaves without a kind are 0 for all ten codes.

| Code | leaves | content | set-field | typed collections (top) | scene Handle{} / handle fns | scene Diagram2d{} / diagram fns | handle write paths uncovered |
|---|---|---|---|---|---|---|---|
| EN 1990 | 115 | 101 | 101 | 13 | 1 / 19 | 20 / 10 | 0 |
| EN 1991 | 184 | 166 | 166 | 16 | 4 / 7 | 19 / 8 | 0 |
| EN 1992 | 135 | 119 | 119 | 7 | 2 / 4 | 10 / 10 | 9 |
| EN 1993 | 511 | 477 | 477 | 26 | 3 / 28 | 25 / 11 | 0 |
| EN 1994 | 112 | 105 | 105 | 4 | 3 / 13 | 21 / 4 | 0 |
| EN 1995 | 171 | 153 | 153 | 12 | 2 / 19 | 23 / 14 | 1 |
| EN 1996 | 128 | 111 | 111 | 5 | 2 / 18 | 11 / 7 | 2 |
| EN 1997 | 151 | 132 | 132 | 13 | 2 / 9 | 10 / 6 | 0 |
| EN 1998 | 244 | 224 | 224 | 14 | 2 / 8 | 29 / 13 | 0 |
| EN 1999 | 182 | 162 | 162 | 15 | 0 / 21 | 16 / 7 | 0 |

Uncovered handle paths: `🗑️generated/status-matrix-4/handles.txt`.

### 2c. Capability checklist

| Code | Design case in 3D | Showcase fixture | Demo example | Catalogue dirs | Catalogue feature table rows | Third-party oracle libs (py files) | Live files | Law test files |
|---|---|---|---|---|---|---|---|---|
| EN 1990 | n/a (no design-case ids) | no | yes (demo-session) | 4 | 59 | Pynite, anastruct, numpy, scipy (2) | 11 | 8 |
| EN 1991 | yes (scene-members cases) | no | yes | 6 | 161 | numpy, scipy (2) | 11 | 6 |
| EN 1992 | yes (9 case refs, 98 design-case ids) | no | yes | 5 | 27 | 7 libs (2) | 11 | 10 |
| EN 1993 | no (0 design-case ids) | yes (`🎪️showcase`) | yes | 3 | 0 (no catalogue feature file) | 7 libs (16) | 11 | 7 |
| EN 1994 | n/a (no design-case ids) | yes | yes | 5 | 48 | 4 libs (7) | 11 | 10 |
| EN 1995 | yes (55 case refs) | no | yes | 4 | 75 | 5 libs (4) | 11 | 9 |
| EN 1996 | yes (7 case refs, 6 design cases) | no | yes | 6 | 75 | 5 libs (2) | 11 | 10 |
| EN 1997 | yes but thin (3 case refs, 6 design cases) | no | yes | 7 | 127 | groundhog, numpy, scipy (6) | 11 | 13 |
| EN 1998 | no (0 design-case ids) | yes | yes | 2 | 0 (no catalogue feature file) | groundhog, numpy, scipy (2) | 11 | 8 |
| EN 1999 | no (0 design-case ids) | yes | yes | 1 | 0 (no catalogue feature file) | 4 libs (2) | 11 | 10 |

"Showcase fixture" is a `showcase` or `demo` dir under the subset, excluding the demo-session editor example. Palette wiring was not measured in this pass (the grep in matrix.py does not cover palette lists).

Live files: 11 files per code reference `live::`, `live_*`, `LiveView`, `SharedEvaluation` or `LiveReport`. The count is equal for all ten codes because they share a template. Live latency is not measured (gap 11).

## 3. NDP registry

| Code | rows | verified | unverified |
|---|---|---|---|
| EN 1990 | 260 | 0 | 260 |
| EN 1991 | 543 | 0 | 543 |
| EN 1992 | 194 | 0 | 194 |
| EN 1993 | 332 | 3 | 329 |
| EN 1994 | 70 | 0 | 70 |
| EN 1995 | 735 | 255 | 480 |
| EN 1996 | 289 | 0 | 289 |
| EN 1997 | 163 | 0 | 163 |
| EN 1998 | 217 | 0 | 217 |
| EN 1999 | 135 | 0 | 135 |
| **Total** | **2938** | **258** | **2680** |

Source: `⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json`, field `status`. status-matrix-3 reported 189 verified of 2903 (2714 unverified). Later NDP-VERIFY, NDP-FOLLOWUP and NDP-EVIDENCE work changed the count, and the registry now holds 2938 rows.

## 4. Open correctness disagreements

Open items, as recorded in the current files. None is verified by a run.

1. `requests.md` (`📌️important/requests.md`): 471 open of 859 bullets. Open S1 items: 12. Open S2 items: 9. Open lines with any bug marker (S1, S2, wrong, inverted, mismatch, bug): 17. status-matrix-3 said about 78. That figure counted lines with markers in all states, so it overstated the open list.
2. Open S1 items, by code (from `requests.md`):
   - EN 1992: member shear `asw` m²/m to mm²/mm is `* 1e3` not `* 1e-3`; resistance file must use `min(V_Rd,s, V_Rd,max)`; Python mirror `flexural_resistance_nm` (`🐍️.py:242-263`) unverified.
   - EN 1993: prying `lb <= l_b_star` inverted (`🧮️clauses/🔗️joint-components:86`); tests and special-parts oracle must follow.
   - EN 1994: two-span coefficients (`⚖️checks/🦀️.rs:131,139`) and `Δσs`.
   - EN 1995: two-span load-combination coefficients (`⚖️timber/🧮️load-combinations:29-31`).
   - EN 1996: `f_vk0` material independence (`⚖️masonry/📏️normative-tables:204-225`).
   - EN 1997: `utilization()` returns 0 for capacity 0, passing (`📇️catalogue/🦀️.rs:140`); `gamma_phi`/`gamma_c` of M2 ignored in earth pressure.
   - EN 1998: Rayleigh T1 normalised shape (`BR98:24-34`); Table 4.2 phi.
   - EN 1999: web stress-gradient eta on internal flanges of RHS/box (`MC99:226-229`); HAZ plate check (`CC99:181`).
   - EN 1990: gamma_G,sup/inf chosen by action kind (sign-based fix per coordinator, unverified).
3. Oracle vs Rust disagreements (`catalogue-oracles.md`, `correctness-2.md`):
   - EN 1995 T-95 `f_d` (666.7 vs 400 N, DE). Annex-independent bracing divisor.
   - EN 1991 E14.09 interpolation, E4.01 class boundary, E17.10 fit, E2.13 band. EN 1991 fire-load fractile now read from registry (unverified rows).
   - EN 1996 pass-through limits.
   - EN 1992 `5.10.decompression`: no oracle row (sign issue). `7.2.sigma-s|sigma-c` assume current bar and centroid conventions.
   - EN 1992 creep `t0_adj` (structuralcodes adapter, `🔮️oracles/🏃️harness/🔌️adapters/structuralcodes/🐍️.py`): the string is present, but the value was not re-checked against the blessed cases.
4. Adjudicator (`requests.md`, "from: adjudicator"): announced for EN 1995, 1991, 1996, 1999 and the compliance owners. Editing the NDP registry rows. No closing report found. Open.
5. Requests that are marked fixed in code but unverified by any run: EN 1992 `asw`/`sigma_cp`, EN 1993 prying, EN 1995 two-span, EN 1997 capacity-0, EN 1993 HEB 240.

## 5. Top 15 remaining gaps (ranked by user impact)

1. **No norm plugin is green.** Impact: nothing a user runs has been verified end to end. All ten newest logs exit 1. EN 1990 and EN 1991 errors are from logs before fixes, and no rerun followed. Next action: run `:check` then `:test` for en1990 alone, then en1991, on a host with the Nx graph wait knob set; log to `build-en1990/` and `build-en1991/`.
2. **Build infrastructure blocks five codes and one config error blocks EN 1999.** Impact: EN 1992, 1993, 1996, 1997 and 1998 fail in Nx graph planning or OOM. EN 1994 and 1995 fail on stale `ndp` cargo dependencies. EN 1999 fails on a duplicate Build project for `ndp` en1990 (two table paths: `🔖️ndp/🗃️table/en1990` and `📜️annex-and-tables/🗃️table/en1990`). Next action: confirm the duplicate table directory is gone; then run `:check` for en1995 and en1999; take INFRA v3 measurements.
3. **Wave-2 visual, live and scene code has never compiled.** Impact: most 3D and real-time editor claims are unproven. Scene goldens are stale. Next action: after the first green `check` per code, run each scene crate's tests once with `SEMIO_UPDATE_GOLDEN=1` and commit the goldens.
4. **NDP registry is 91.2 % unverified, with eight codes at zero verified rows.** Impact: every compliant verdict carries an unverified basis. EN 1990 (260 rows) and EN 1991 (543 rows) gate the partial factors and combinations. Next action: verify EN 1990 rows first, against the DIN EN 1990/NA text, recording edition and locator in each row. Then EN 1991.
5. **Open S1 correctness items (12 in `requests.md`).** Impact: possibly unconservative results in EN 1990 (gamma sign), EN 1993 (prying), EN 1994 and EN 1995 (two-span coefficients), EN 1996 (`f_vk0`), EN 1997 (capacity 0), EN 1998 (Rayleigh period), EN 1999 (eta). Next action: one regression test per item with a hand-computed expected value; close the request only when a run passes.
6. **Oracle vs Rust disagreements have no closing adjudication.** Impact: the disagreements list (EN 1995 T-95, EN 1991 E14.09 and E4.01, EN 1996 pass-through, EN 1992 decompression sign) is not decided. Next action: finish the adjudicator pass: decide each by clause, fix the wrong side, and add one row per item to the oracle.
7. **No 3D design cases in EN 1993, 1994, 1998 and 1999; thin in EN 1997 and EN 1996.** Impact: the goal "design cases in 3D" does not hold for four codes. Those codes have no design-case trace ids at all. Next action: classify the spec ids for EN 1993 first (design situations to design-case, with a 3D case view using the EN 1991 `scene-members` pattern); record a clause reason for any id kept as check.
8. **Catalogue feature rows missing for EN 1993, 1998 and 1999, thin for EN 1992 (27 rows) and EN 1994 (48).** Impact: catalogue entries in those codes are not checked by a feature row. Next action: generate catalogue feature rows with the `catalogue-oracles` recipe. Reconcile row counts against catalogue entries for EN 1992 and EN 1994.
9. **Showcase fixtures exist in four codes only.** Impact: EN 1990, 1991, 1992, 1995, 1996 and 1997 have only the demo-session example. Next action: add a full showcase fixture per code (the EN 1993 `unreached_fragments` pattern) and a palette test. Palette wiring is not yet measured; check it first.
10. **Law tests have never executed.** Impact: `assert_scene_laws`, `assert_diagram_laws` and `assert_handle_laws` run in every code, but nothing has tested them, so a broken scene could pass by default. Next action: after gap 1, run `:test` with the filter `law` once per code and fix the first failures.
11. **Live latency is unmeasured.** Impact: the 12 ms p95 drag and 400 ms check budgets are unproven. The latency bench (`latency_bench`, EN 1990 editor) has never run in release. Next action: run it on a release profile in EN 1990 and record p95 to `🗑️generated/realtime-qa/`.
12. **Handle write paths are not covered by edit rules (12 paths).** Impact: direct manipulation of those handles cannot write through the edit rules. EN 1992 has 9, EN 1996 has 2, EN 1995 has 1. Next action: add one `SetFieldRule` per path from `🗑️generated/status-matrix-4/handles.txt`, then rerun handle-paths-check.
13. **Third-party oracle cross-check has not run through Rust.** Impact: the oracles exist for all ten codes with 2 to 16 Python files each, but the Rust tests that call them (for example `third_party_libraries_reproduce_the_blessed_oracle_cases`) have never run. Next action: after gap 1, run that test once for EN 1990 and EN 1993.
14. **Spec-trace integrity is unreviewed.** Impact: absent candidates are 39 (EN 1992), 31 (EN 1993), 54 (EN 1999, C-01..C-54), 96 (EN 1997, likely regex noise), 17 (EN 1998); not-modelled ids are 6 (EN 1991), 4 (EN 1993) and 1 (EN 1999). Next action: write one reconciliation rule per code (candidate id, trace status, clause reason), and mark each not-modelled id with its reason.
15. **Open requests backlog is stale.** Impact: 471 open requests of 859 bullets; the done items are not all reflected in code, so work repeats and fixes go uncredited. Next action: close each `requests.md` entry only with a test or run reference, starting with the 17 open lines that carry bug markers.

## 6. Notes on what changed from status-matrix-3

- Leaves: the approximation (about 93 without kinds) is gone. The exact figure is 0 content leaves without a kind in all ten codes. The coordinator's LEAVES-AND-HYGIENE entry (01:45) agrees.
- EN 1996 trace: 6 design-case ids now (matrix-3 said 23).
- NDP: 258 verified of 2938 now (matrix-3 said 189 of 2903).
- Open bug markers: 17 open lines, not 78.
- Handles: exact uncovered count is 12 paths (matrix-3 said about 17 handle-only leaves in EN 1991, which this measure does not reproduce; EN 1991 has 0 uncovered handle write paths).

## 7. Files

- Matrix script: `🗑️generated/status-matrix-4/matrix.py`
- Matrix output: `🗑️generated/status-matrix-4/matrix.json`
- Leaf and handle tool output: `🗑️generated/status-matrix-4/leaf.txt`, `handles.txt`
- Run log: `🗑️generated/status-matrix-4/run.txt`
- Previous matrix: `🔍️audits/wave2/status-matrix-3.md`
- Coordinator log: `🔍️audits/wave2/coordinator.md`

## 8. EN 1994 items closed or re-stated by en1994 v8 (read-only to the matrix script; never compiled)

- Open S1 (section 4, EN 1994 two-span coefficients and `Δσs`): fixed in v7 and covered by `e1_18_*` plus `two_span_cracked_alpha_st_is_the_cracked_ratio`; closes with the first green `:test` run.
- Gap 13 (third-party cross-check through Rust): the Rust subject adapter of the catalogue feature now answers E1-16..E1-20 (column fire cells, shear lag, girder details, composite-beam fire, fatigue); E1-14/E1-15 and `crack_band_k6` remain without adapter.
- Gap 8 (EN 1994 feature rows): 48 rows before, 56 now (E1-19 beam fire: steel, slab soffit, slab core, M_fi,Rd; E1-20 fatigue: stud endurance, stud damage, rebar damage, steel utilisation).
- `Iw` blessed cases: `ISection::iw()` exists; the harness adapter for `en1994.6.4.ltb-restrained-*` stays with the shared agent.
- Gap 7 (design cases in 3D): not done. EN 1994 has no design-case collection in the schema; adding one touches the snapshot in all twins and about 300 fixtures and needs the EN 1993 recipe first. Left open with this reason.
- Section class fillet `r` (doc-oracles request): not done, needs a new `SteelSection` leaf through the field adder.
- Visuals added: beam section temperature plates (steel, slab, reinforcement) with time slider, `fire-temperature:` and `fire-resistance:` diagrams, fatigue S-N and stud-row diagrams, governing stud row highlight in 3D. Values stay unverified (`ndp.en1994.1-2.*`, `ndp.en1994.2.rebar-fatigue-range`).

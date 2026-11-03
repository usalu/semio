# 🧭 Status Matrix 5 (NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS)

Read-only audit at 2026-11-03 03:10 CET. No source edits, no builds, no git writes. Raw output: `🗑️generated/status-matrix-5/` (`matrix.py` copied from status-matrix-4 and re-run into `matrix.json`, `run.txt`, `leaf-coverage.txt`, `handles.txt`, `spec-trace.txt`, `delta.py`). Baseline: `🗑️generated/status-matrix-4/matrix.json` (01:56).

## 0. Headline

- Nothing is compiled green for any EN family. The only green build since the fixes is `norm-artifact-contract-rs:build` (01:42) and mechanics (run 1). Every family's newest numbered build log predates most of the 02:00 to 03:07 source edits.
- Spec trace is consistent: the strict verifier passes 10 of 10 (`⚖️compliance/🧭️spec-trace/🧰️codegen/🐍️.py`, rc 0). Spec id counts are unchanged from matrix 4 (676 ids in total).
- NDP registry: 3053 rows (+109 since matrix 4), 258 verified (unchanged), 2795 unverified.
- Leaf coverage is not computable for EN 1992 to EN 1999 this run. `kind-convention-v2/leaf_coverage.py` crashes with `KeyError: 'ReliabilityClass'` on the en1992 snapshot (the dangling `$ref` came from a schema snapshot rewritten at 03:06). Matrix 5 keeps the matrix 4 values for those families, marked stale.
- Handle-path check now reports 0 uncovered in all ten families (matrix 4: 12). This is a checker fix (member-relative literals and entity source refs were false positives per HANDLE-RULES), not new handle write paths.

## 1. Per-code matrix, delta against status-matrix-4

Columns: spec ids (trace kinds), NDP verified / rows, content leaves (without kind), uncovered handle paths, design-case 3D (`cases` refs), live files, third-party oracle modules, catalogue dirs, showcase dirs, typed top-level collections. `m4 -> m5` where changed.

| Code | Spec ids (trace kinds, m5) | NDP verified / rows | Leaves (content, no-kind) | Handles uncovered | Design cases 3D | Live files | Oracles (3rd-party modules, py files) | Catalogue / showcase | Typed collections | Open S1 (per newest reports) |
|---|---|---|---|---|---|---|---|---|---|---|
| EN 1990 | 29 (check 29) | 0 / 260 -> 0 / 270 | 101 -> 103 (0) | 0 | n/a | 11 -> 13 | 4 (Pynite, anastruct, numpy, scipy), 2 | 4 / 3 | 13 -> 14 | gamma sign: span-arrangement envelope in code (en1990-v9), unrun |
| EN 1991 | 71 (check 46, design-case 24, not-modelled 1); m4 was 36 / 29 / 6 | 0 / 549 -> 0 / 591 | 166 -> 202 (0) | 0 | yes (scene-members) | 11 -> 12 | 2 (numpy, scipy), 2 | 6 / 3 | 16 -> 22 | none listed |
| EN 1992 | 159 (check 60, design-case 98, data 1) | 0 / 194 -> 0 / 208 | stale 119 (crash now) | 9 -> 0 | yes | 11 -> 12 | 7, 2 | 5 / 3 | 7 | asw `* 1e3` already correct (v10), unrun; E_cm now registry rows |
| EN 1993 | 142 (check 133, data 5, not-modelled 4) | 3 / 332 (unchanged) | stale 477 (crash now) | 0 | n/a (no cases) | 11 -> 12 | 7, 16 | 3 / 2 | 26 | prying `lb <= l_b*` fixed in source (status-matrix-3), unrun |
| EN 1994 | 54 (check 51, data 3) | 0 / 70 -> 0 / 101 | stale 105 (crash now) | 0 | n/a | 11 -> 12 | 4, 7 | 5 / 3 | 4 | two-span coefficients and Delta sigma fixed (v7), unrun |
| EN 1995 | 68 (check 34, design-case 34); m4 33 / 35 | 255 / 735 -> 255 / 743 | stale 153 (crash now) | 1 -> 0 | yes, but refs 54 -> 7 | 11 -> 12 | 5, 4 | 4 / 3 | 12 -> 13 | two-span: no change needed (span-check); sagging not computed by ContinuousTwoSpan |
| EN 1996 | 37 (check 37); m4 31 / 6 | 0 / 289 -> 0 / 294 | stale 111 (crash now) | 2 -> 0 | n/a (0 design cases) | 11 -> 12 | 5, 2 | 6 / 3 | 5 | f_vk0 already correct (v8), unrun |
| EN 1997 | 42 (check 42); m4 36 / 6 | 0 / 163 -> 0 / 162 | stale 132 (crash now) | 0 | n/a (0 design cases) | 11 -> 12 | 3, 6 | 7 / 7 | 13 | capacity-0 fixed (status-matrix-3); M2 gamma_phi/gamma_c in earth pressure: not confirmed in newest reports |
| EN 1998 | 32 (check 28, data 4) | 0 / 217 | stale 224 (crash now) | 0 | n/a | 11 -> 12 | 3, 2 | 2 / 3 | 14 -> 15 | Rayleigh T1 and Table 4.2 phi already correct (v10), unrun |
| EN 1999 | 42 (check 40, data 1, not-modelled 1) | 0 / 135 | stale 161 (crash now) | 0 | n/a | 11 -> 12 | 4, 2 | 1 / 2 | 15 | eta on internal flanges and HAZ plate: marked closed (v10), but SUMMARY still lists them open |

Trace-kind moves: design-case to check in EN 1996 (6), EN 1997 (6), EN 1991 (5 design-case to check, 5 not-modelled to check) and EN 1995 (1 design-case to check). EN 1997 and EN 1996 have no design-case rows left, so their "design cases 3D" is n/a.

Unchanged between matrix 4 and matrix 5: spec-trace unresolved check refs (0 in all ten); the heuristic list of spec doc ids absent from the trace (e.g. EN 1992 `D-30`, `E-01`; EN 1993 `V-029`; EN 1998 `BR-01`; these are regex matches and need per-family classification, not a finding yet).

Totals: NDP 3053 rows, 258 verified, 2795 unverified (868 awaiting an annex value). Leaves measured this run: EN 1990 103, EN 1991 202 content leaves, both with 0 without kind.

### 3D highlight and visual coverage (first capture in matrix 5, static, from `🔍️audits/wave2/visual-coverage.md` 02:57)

- 140 top-level collections: 3D 129 (92%), grip 94 (67%), diagram 46 (33%), insert palette 106, showcase 128.
- Check-family binding (literal proxy): 171 of 467 check ids bound in scene (37%). 128 of 222 families have no bound id. Per code: EN 1997 26 of 33 unbound, EN 1992 26 of 36, EN 1998 25 of 48, EN 1996 15 of 29, EN 1993 12 of 21, EN 1995 13 of 17, EN 1990 8 of 17. EN 1991 is fully bound via `report_owner`.
- Out of scope for the Eurocode goal (other plugins): DIN 18599, DIN 16798, DIN 4108 (no scene at all), VDI 3805 and ISO 16757 (no scene).

### Oracles and collection facts (matrix 5)

- Python oracle modules per family: EN 1990 2, EN 1991 2, EN 1992 2, EN 1993 16, EN 1994 7, EN 1995 4, EN 1996 2, EN 1997 6, EN 1998 2, EN 1999 2. Third-party imports across them: 44 module entries (Pynite, anastruct, sectionproperties, shapely, groundhog, rainflow, structuralcodes, concreteproperties, blueprints, jsonschema, numpy, scipy).
- Oracle cases: 2 to 7 third-party libraries per family. No Rust test has run against the oracles (see gap 6).

## 2. Build status (newest logs, as of 03:10)

| Family | Newest numbered log | Last recorded outcome | Note |
|---|---|---|---|
| EN 1990 | `build-en1990/15.txt` (02:36, running) | run 15 (`:build`) at 2040 s in `build:projects` at 03:10. Earlier: run 14 1 rustc error (E0063 missing `factor_overrides` in `En1990Snapshot`), run 13 "Native dependency has no Cargo manifest" for the NDP en1990 rust package | Only family with a current build |
| EN 1991 | `14.txt` (23:47) | run 14: parse error at `bridge_lanes` | Stale 3.5 h |
| EN 1992 | `16.txt` (00:26) | run 16: Nx OOM (exit 124), re-queued | Stale 10 h |
| EN 1993 | `12.txt` (00:55) | run 12: Nx OOM; run 11: failed to get NDP en1990 dep | Stale 10 h |
| EN 1994 | `12.txt` (02:32) | run 12: SIGTERM (exit 143) | Only the stop is recent |
| EN 1995 | `17.txt` (22:10) | run 17: failed to get NDP en1990 dep | Stale 13 h |
| EN 1996 | `12.txt` (23:25) | run 12: Nx OOM; run 11: 8 rustc E0063 (`placements` in `En1996Snapshot`) | Stale |
| EN 1997 | `14.txt` (22:30) | run 14: Nx failed; run 13: NDP en1990 dep | Stale 13 h |
| EN 1998 | `12.txt` (00:08) | run 12: graph 25/52, OOM re-queued | Stale |
| EN 1999 | `11.txt` (00:19) | run 11: duplicate build project `semio-s-plugin-norm-ndp-en1990`, OOM | Stale |
| Shared | `🗑️generated/integrate/4.txt` (01:42) | `norm-artifact-contract-rs:build` green, exit 0 (4207 s) | Mechanics: green run 1 (14:40), failed run 2 (01:20) |

Runner (`🗑️generated/build-runner/`): last log line 03:08 is a load gate ("host busy, load 38.79 on 12 CPUs, one build at a time"). `runner.lock` 02:37 is held; `runner.out` shows 5 "another runner holds the lock" lines and a `runner-next.sh` permission error. `runner-next.new` (03:11) is an in-progress rewrite, so the runner is not in a stable state. Common failures across families: OOM / SIGKILL from Nx, NDP dependency ownership, and rustc errors from the last edit wave (no family has a green `check` or `test`).

## 3. Top 15 remaining gaps against "norm complete, visual 3D, rich real-time editors"

Blocker key: [printed standards] = needs values from the printed EN or DE NA; [builds] = needs a green compile or test run; [schema generator] = needs SCHEMA-CODEGEN v2 or a generator fix; [none] = implementable now.

1. **No family is green on build, check or test.** The en1990 run 15 has been in `build:projects` for 34+ min. Next action: let run 15 finish or cancel it, then run the families serially (`:build`, then `:test`) from en1990 once the runner rewrite is stable. Blocker: [builds].
2. **Build graph ownership is broken.** Errors: "Native dependency has no Cargo manifest" (NDP en1990 rust package), "Duplicate Build project semio-s-plugin-norm-ndp-en1990" (paths in `⚖️compliance/📜️annex-and-tables/🔖️ndp/🗃️table/en1990` and `📦️packages`), "failed to get `semio-s-plugin-norm-ndp-en1990`" in 5 families. Next action: resolve one owner for the en1990 NDP package in `build-owners-sync.txt` and re-sync. Blocker: [builds].
3. **Infra OOM and runner state.** Nx SIGKILL and OOM in 6 families; host load 38 to 52 on 12 CPUs; `runner-next.sh` rewrite in progress with a stale lock. Next action: finish the runner rewrite, keep one build at a time, and bound Nx parallelism to 1 for each family. Blocker: [builds].
4. **Known rustc errors from the last edit wave.** EN 1990 E0063 `factor_overrides`; EN 1996 8 × E0063 `placements`; EN 1991 parse error at `bridge_lanes`; EN 1992 test-dependency identity error. Next action: fix these four in the owning artifact, then re-run that family's `:build`. Blocker: [builds] to confirm, [none] to fix.
5. **Source budget overflows block compile.** `CRATE-BUDGET F` is open for en1994-checks (4089 lines), en1991 checks, en1992 checks, en1996 diff-json-schema (95 %), en1997 mut-wall/pile, en1998 scene, structural-scene and app-surface-panels. The gate rejects a build on any crate over budget. Next action: finish the CRATE-BUDGET F splits in the owning crates before the next gate run. Blocker: [none] to split, [builds] to verify.
6. **S1 correctness fixes are unrun.** EN 1990 gamma sign (span-arrangement envelope), EN 1992 asw/min, EN 1993 prying, EN 1994 two-span coefficients and Delta sigma, EN 1995 two-span, EN 1996 f_vk0, EN 1997 capacity-0, EN 1998 Rayleigh/phi, EN 1999 eta and HAZ. The coordinator's "closed" claims rest on reading code and on the regression tests, not on a run. Next action: once the family builds, run each `review_s1_regressions`-style test per family and record the result in `status-matrix-6`. Blocker: [builds].
7. **Schema snapshot has a dangling `$ref`.** `en1992` snapshot (03:06) refers to `#/$defs/ReliabilityClass` with no definition. `leaf_coverage.py` now crashes for EN 1992 to EN 1999, so leaf coverage is unknown for 8 families. Next action: fix the snapshot `$defs` emission for enum refs in the schema generator (RC-THREADING / GAMMA-UNIFY touched it), re-run `leaf_coverage.py en1990 ... en1999`. Blocker: [schema generator].
8. **Typed refs and enums still in the generator.** SCHEMA-CODEGEN v2 is needed for the per-stud rows in EN 1992, the `concreteClass` enum leaf in EN 1997, the typed storey ref in EN 1998 and the number-typed `wallStorey` (f64-only field-adder). Next action: land SCHEMA-CODEGEN v2 with all-type tools, then convert these four. Blocker: [schema generator].
9. **NDP registry is mostly unverified.** 258 of 3053 rows are verified (8.5%); 2795 are unverified, 868 awaiting an annex value. EN 1990 and EN 1991 have zero library sources. Next action: transcribe values from the printed EN annexes and the DE NA for the rows with the most check sites (`🗑️generated`/`🔍️audits/wave2/unverified-register.md`, per-code worklist). Blocker: [printed standards].
10. **Printed-table gaps.** EN 1994 C-32, C-46, C-54 tables not transcribed; EN 1998 Table 9.3 masonry not available and Table 3.1 thresholds are unverified NDP rows; EN 1995 k_mod/k_def (117 rows, no source); EN 1997 33 GPa modulus is from memory. Next action: obtain the printed tables and cite them in `evidence`, or downgrade the rows to `awaiting values`. Blocker: [printed standards].
11. **Check families not highlighted in 3D.** About 128 of 222 families have no literal scene binding (EN 1997 26 of 33, EN 1992 26 of 36, EN 1998 25 of 48). Failures in these families stay in the checks table. Next action: CHECK-HIGHLIGHT per family: bind each check's `subject.entity_id` to a scene entity and add one runtime test per family that asserts the highlighted entity. Blocker: [none] to implement, [builds] to prove.
12. **Grips missing on 46 of 140 collections.** EN 1995 has 9 of 13 without grips (tapers, cutouts, fasteners, panels, floors, diaphragms, trusses, bracings, decks); EN 1993 has 11 of 26; placements have no grip in EN 1990, 1992, 1993, 1994, 1996, 1998. Next action: VISUAL-GRIPS, starting with the EN 1995 timber collections and the placement drag in the six codes. Blocker: [none].
13. **Diagrams missing on 94 of 140 collections.** EN 1998 13 of 14 collections have no diagram (seismic results only in the checks table); EN 1991 17 of 22; EN 1997 upliftCases, slopes, anchors, pileGroups, layerWaters. Next action: VISUAL-DIAGRAMS per family, storey drift and base shear first for EN 1998. Blocker: [none].
14. **EN 1995 case scene unification is partial.** 48 of 70 cases are on shared forms, 22 still on the entity engine (prisms, laminates, char, modes, assemblies). Literal 3D refs for cases dropped 54 to 7 in matrix 5, which may be a proxy artifact or a real loss. Next action: finish FORMS-V2 (migrate the 22 cases), then check the 3D refs with a scene test. Blocker: [builds] to verify the scene.
15. **Typed-enum fixtures contain guesses.** TYPED-ENUMS v2 restored 288 EN 1995 `strengthClass` fixtures "by most common pattern", and `snow_high` (EN 1990 A1.1, H > 1000 m) was dropped. Next action: TYPED-ENUMS v3: repair the fixtures from the source first, then convert the remaining 6 codes. Blocker: [none].

Excluded from the top 15 (out of scope or minor): DIN, VDI and ISO 3D gaps (other plugins); the coordinator and SUMMARY counts disagree (coordinator says 396 NDP rows verified, the registry says 258; SUMMARY says 12 open S1, open-items.md lists 8 S1 lines and requests.md 11). Reconcile these in `status-matrix-6`.

## 4. Blocker summary

- [builds]: gaps 1, 2, 3, 6, 14 (verification), 11 (proof).
- [printed standards]: gaps 9, 10.
- [schema generator]: gaps 7, 8.
- [none, implementable now]: gaps 4 (fixes), 5, 11 (implementation), 12, 13, 15.

## 5. Caveats

- All numbers are static (literal matches, registry reads and checker outputs). Nothing was compiled or run in this audit.
- Matrix 4 leaf values for EN 1992 to EN 1999 are carried forward and marked stale; they are not re-measured.
- Handle-path "0" reflects the corrected checker, not a new verification of handle write paths.
- Spec doc ids absent from the trace come from a shape regex and may include annex letters; they are not counted as gaps.

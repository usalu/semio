# 🧭 Status Matrix 6 (NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS)

Read-only audit at 2026-11-03 ~04:10 CET. No source edits, no builds, no git writes. Only reads, plus `--check`-style verifier runs (the spec-trace verifier was run without `--write`).

Raw output: `🗑️generated/status-matrix-6/` (`matrix.py` copied from status-matrix-5 and re-run into `matrix.json`, `run.txt`, `delta.txt`; `leaf-coverage.txt`, `handles.txt`, `spec-trace.txt` from the verifiers; `fixtures.txt` from the fixture checker).

Verifier tools used:
- Spec trace: `⚖️compliance/🧭️spec-trace/🧰️codegen/🐍️.py` run with `/home/deck/semio/.venv/bin/python -I` (Python 3.14.6, jsonschema 4.26.0). rc 0, 10 of 10 ok, 714 spec ids.
- Leaf coverage: `🧰️scripts/kind-convention-v2/leaf_coverage.py` (all ten families, rc 0).
- Handle paths: `🧰️scripts/handle-paths-check.py` (rc 0, 0 uncovered in all ten).
- Fixtures: `🧰️scripts/kind-convention-v2/verify_fixtures.py <family>` per family (see section 1).

## 0. Headline

- The `KeyError: 'ReliabilityClass'` crash from matrix 5 is gone. The en1992 snapshot now has no dangling `$ref` (checked: no `$defs` reference without a definition in any snapshot). Leaf coverage is measured for all ten families (matrix 5 carried stale values for EN 1992 to EN 1999).
- Nothing is green on build, check or test for any EN family. Matrix 5 said the same; the only build change is en1990 run 15 (cancelled at 03:20 in graph-plan, re-queued) and en1994 run 13 (running at 04:08).
- Fixtures regressed against the 04:04 `fixture-kinds.md` report. That report claims 0 failures in en1991, en1992, en1993 and en1995. The checker now reports 8, 2, 16 and 1 failures there. 25 mutation-fixture failures in total (en1994 2 included).
- The EN 1991 spec trace grew from 71 to 109 ids (10 now `not-modelled`, was 1).
- Spec ids total 714 (matrix 5: 676, all of the change is EN 1991).
- NDP registry: 3060 rows (+7), 258 verified (unchanged), 2802 unverified.

## 1. Per-code delta against status-matrix-5

Columns: spec ids (trace kinds), NDP verified / rows, content leaves (without kind), uncovered handle paths, live files, fixtures (scenarios, failures), newest numbered build log. `m5 -> m6` where changed. Full per-key diff: `🗑️generated/status-matrix-6/delta.txt`.

| Code | Spec ids (trace kinds) | NDP verified / rows | Leaves (content, no kind) | Handles uncovered | Live files | Fixtures (scenarios, failures) | Newest build log |
|---|---|---|---|---|---|---|---|
| EN 1990 | 29 (check 29), unchanged | 0 / 270, unchanged | 103 (0), unchanged | 0 | 13 -> 14 | 148, 0 (unchanged) | `15.txt` 03:20, run 15 OOM re-queued (m5: running at 03:10) |
| EN 1991 | 71 -> 109 (check 46 -> 63, design-case 24 -> 36, not-modelled 1 -> 10) | 0 / 591 -> 0 / 593 | 202 (0), unchanged | 0 | 12 -> 13 | 246, **8 (new; was 0)** | `14.txt` 23:47, stale (parse error at `bridge_lanes`) |
| EN 1992 | 159 (check 60, design-case 98, data 1), unchanged | 0 / 208 -> 0 / 212 | stale -> 119 (0): re-measured | 0 | 12 -> 13 | 154, **2 (new)** | `16.txt` 00:26, stale (Nx OOM) |
| EN 1993 | 142 (check 133, data 5, not-modelled 4), unchanged | 3 / 332, unchanged | stale -> 478 (0): re-measured | 0 | 12 -> 13 | 578, **16 (new)** | `12.txt` 00:55, stale (Nx OOM) |
| EN 1994 | 54 (check 51, data 3), unchanged | 0 / 101, unchanged | stale -> 109 (0): re-measured | 0 | 12 -> 13 | 126, **2 (new)** | `13.txt` 04:08, **running** (freeze on build-en1994 since 03:24) |
| EN 1995 | 68 (check 34, design-case 34), unchanged | 255 / 743, unchanged | stale -> 172 (0): re-measured | 0 | 12 -> 13 | 208, **1 (new)** | `17.txt` 22:10, stale (NDP en1990 dep) |
| EN 1996 | 37 (check 37), unchanged | 0 / 294, unchanged | stale -> 112 (0): re-measured | 0 | 12 -> 13 | 147, 0 | `12.txt` 23:25, stale (Nx OOM) |
| EN 1997 | 42 (check 42), unchanged | 0 / 162, unchanged | stale -> 157 (**1**: `piles[].concreteClass`) | 0 | 12 -> 13 | 204, 0 | `14.txt` 22:30, stale (Nx failed) |
| EN 1998 | 32 (check 28, data 4), unchanged | 0 / 217 -> 0 / 218 | stale -> 240 (**2**: `buildings[].members[].effectiveStiffnessBasis`, `buildings[].storeyStiffnessBasis`) | 0 | 12 -> 13 | 287 (m5 table 296), 0; **11 orphan fixture dirs** (no schema kind) | `12.txt` 00:08, stale (OOM) |
| EN 1999 | 42 (check 40, data 1, not-modelled 1), unchanged | 0 / 135, unchanged | stale -> 162 (0): re-measured | 0 | 12 -> 13 | 223, 0, no orphans | `11.txt` 00:19, stale (duplicate NDP en1990 build project) |

Totals: spec ids 714 (m5 676). NDP 3060 rows, 258 verified, 2802 unverified. Handle-path uncovered 0 everywhere (unchanged from matrix 5). Oracle third-party modules, catalogue and showcase counts: unchanged from matrix 5 within a few units (oracle py files 45, catalogue dirs 43, showcase dirs 32, typed top-level collections 134).

Spec-doc ids absent from the trace (regex, not gaps yet): EN 1997 96, EN 1999 54, EN 1992 39, EN 1993 31, EN 1998 17, EN 1995 4, EN 1996 2, EN 1990 1, EN 1991 0, EN 1994 0.

Caveats on the delta:
- The live-file counts moved from 11 to 12 in matrix 5 and to 13 here for all families. That is a `LIVE_RE` proxy shift from the shared scene code, not new live editing.
- The en1991 jump (71 to 109) is a trace rewrite (`en1991-v10`, `en1991-v11` at 03:22 and 03:36), not new checks in the code.

### 1a. Fixture checker (verify_fixtures.py) detail

Failures print before each family's summary line. Lines per family:

- EN 1991 (8): insert/remove of `derailment`, `falling`, `helicopter`, `imperfection`. Refused: "addresses None, which is not an ordered collection", or "addresses a collection this document does not carry". The kinds point at a collection the snapshot does not hold.
- EN 1992 (2): insert/remove `bar-layer`: "after-snapshot differs".
- EN 1993 (16): insert/remove of `cf-action`, `crane-action`, `fatigue-band`, `plated-action`, `tension-action`, `tower-action` (same collection-address refusal); `insert-joint-action` and `insert-pile-action` (after-snapshot differs); `remove-joint-action` (inverse refused: id `joint-w1` already held); `remove-pile-action` (inverse does not restore before).
- EN 1994 (2): `change-beam-steel-designation` and `change-reliability-class`, after-snapshot differs. The second is the annex `En`/`De` defect that `fixture-kinds.md` lists as open in the frozen artifact.
- EN 1995 (1): `change-deck-compression-c90` refused: "change-compression-c90 names 'compression-c90' which this document has no key for". This is the same fixture that the 03:40 engine change was meant to fix.

Likely cause, not confirmed: `🔮️oracles/🏃️execution/🐍️.py` is unchanged since 03:38, while `verify_fixtures.py` was modified at 04:06 (after the 04:04 report). Either the 04:06 checker edit or the 04:04 to 04:06 schema or engine edits by other owners changed how kinds resolve. Diagnose by diffing the checker against its 04:04 state before touching any fixture.

## 2. Build status (newest logs, as of ~04:10)

| Family | Newest numbered log | Last recorded outcome | Change vs m5 |
|---|---|---|---|
| EN 1990 | `15.txt` (03:20) | run 15 SIGTERM-cancelled in graph-plan 03:20, OOM re-queued, 0 rustc errors (build-fixer.md) | Changed: m5 said still running |
| EN 1991 | `14.txt` (23:47) | run 14: parse error at `bridge_lanes` | Same, stale |
| EN 1992 | `16.txt` (00:26) | run 16: Nx OOM (exit 124) | Same, stale |
| EN 1993 | `12.txt` (00:55) | run 12: Nx OOM | Same, stale |
| EN 1994 | `13.txt` (04:08) | run 13 in progress (`build:playground-native`, `build:playground-variants`) | Changed: running; freeze named build-en1994 since 03:24 |
| EN 1995 | `17.txt` (22:10) | run 17: NDP en1990 dep failure | Same, stale |
| EN 1996 | `12.txt` (23:25) | run 12: Nx OOM; earlier 8 × E0063 `placements` | Same, stale |
| EN 1997 | `14.txt` (22:30) | run 14: Nx failed | Same, stale |
| EN 1998 | `12.txt` (00:08) | run 12: graph 25/52, OOM | Same, stale |
| EN 1999 | `11.txt` (00:19) | run 11: duplicate build project `semio-s-plugin-norm-ndp-en1990`, OOM | Same, stale |
| Shared | `integrate/4.txt` (01:42) | `norm-artifact-contract-rs:build` green (matrix 5 says the same) | Same |

No family has a green `check` or `test` run. Newer edit waves (forms v2 to v4, schema codegen v2, rc threading v2, no-defaults v2, visual grips v2, shared v10 to v12, en1990 to en1999 v9 to v12) are all marked "UNCOMPILED", with "ready for rebuild" lines in each `latest.md`. The build queue is therefore behind the source by many waves.

## 3. Top 12 remaining gaps against the goal

Goal: norm complete, visual (3D where possible) with rich real-time editors, for the ten EN artifacts. Tags: `builds`, `printed standards`, `generator`, `implementable now`.

1. **No family is green on build, check or test.** Everything since 01:42 is uncompiled, and the en1994 freeze holds the queue. Tag: `builds`. Next action: once `build-en1994` run 13 ends, lift the freeze and run `:build` then `:test` serially per family, starting with en1990 (the newest log, and the most rustc-tested).
2. **25 fixture failures across EN 1991 to EN 1995 that the 04:04 report says are clean.** Tag: `implementable now`. Next action: diff `verify_fixtures.py` against its 04:04 state and the engine `🐍️.py` to find the change that makes collection-address kinds fail; then fix the fixtures (hand-edit, no migration) and re-run `verify_fixtures.py` per family.
3. **Schema-to-kind collection addressing for the EN 1993 action and band kinds and EN 1991 hazard kinds.** Their kinds point at collections that the snapshot does not hold ("addresses None"), or insert ids that already exist. Tag: `generator`. Next action: regenerate these kinds from the snapshot with the kind-convention v4 generator (`🧰️scripts/kind-convention-v4/taxonomy_generate.ts`) or correct the owner collection in the kind rule, then re-check with `verify_fixtures.py en1991 en1993`.
4. **NDP registry: 258 of 3060 rows verified (8.4%); 2802 unverified, 868 awaiting an annex value.** EN 1990 (270 rows) and EN 1991 (593 rows) have zero verified rows. Tag: `printed standards`. Next action: transcribe annex values from the printed EN and DE NA for EN 1990 and EN 1991 first, citing them in `evidence`.
5. **Build graph ownership and Nx OOM.** "Duplicate Build project semio-s-plugin-norm-ndp-en1990" (EN 1999), "failed to get `semio-s-plugin-norm-ndp-en1990`" (EN 1995, EN 1997), Nx OOM in six families. Tag: `builds`. Next action: give the en1990 NDP package one owner in `build-owners-sync` and run `verify build-owners sync`; bound Nx to `--parallel=1` per family.
6. **Known rustc errors from the last edit wave are unconfirmed.** EN 1990 E0063 `factor_overrides` (run 14), EN 1996 8 × E0063 `placements`, EN 1991 parse error at `bridge_lanes`. Newer runs show 0 rustc errors, but those runs stopped before rustc. Tag: `implementable now` (grep-check) plus `builds`. Next action: grep the snapshot constructors for `factor_overrides`, `placements` and `bridge_lanes`; fix any still present, then build that family.
7. **S1 correctness fixes unrun.** EN 1990 gamma sign, EN 1992 asw, EN 1993 prying, EN 1994 two-span and Delta sigma, EN 1995 two-span, EN 1996 f_vk0, EN 1997 capacity-0, EN 1998 Rayleigh and phi, EN 1999 eta and HAZ. Tag: `builds`. Next action: after gap 1, run each family's regression test for these items and record the outcome in `status-matrix-7`.
8. **EN 1991 spec trace: 10 `not-modelled` rows (was 1) and 38 new ids.** The rewrite (`en1991-v10`, `en1991-v11`) added ids without checks. Tag: `implementable now`. Next action: for each of the 10 `not-modelled` rows, either add a `check` realisation with a test or record the reason and the clause in the trace, then re-run the spec-trace verifier with the venv.
9. **Content leaves without a set-field kind: 3 (EN 1997 `piles[].concreteClass`, EN 1998 `buildings[].members[].effectiveStiffnessBasis`, `buildings[].storeyStiffnessBasis`).** Tag: `implementable now` for the two EN 1998 leaves (add set-field kinds and fixtures), `generator` for the EN 1997 enum leaf (needs SCHEMA-CODEGEN v2 output for the string enum). Next action: add the two EN 1998 kinds and re-run `leaf_coverage.py en1998`.
10. **EN 1998 has 11 fixture directories without a schema kind.** The `fixture-kinds.md` report claimed no orphans. Names: `change-assessment-building-id`, `change-building-eccentricity-ratio`, `change-building-multi-systems`, `change-building-simple-masonry`, `change-building-stiffness-basis`, `change-building-stiffness-ratio`, `change-foundation-building-id` and others. Tag: `implementable now`. Next action: rename each directory to its current kind by hand, or delete it if the kind is gone, then re-run `verify_fixtures.py en1998`.
11. **Typed-enum fixtures contain guesses.** EN 1995 `strengthClass` (288 fixtures restored "by most common pattern"); EN 1990 `snow_high` (A1.1, H > 1000 m) dropped. Tag: `implementable now` (repair from the source first); `printed standards` to confirm the values. Next action: TYPED-ENUMS v3, restore each fixture value from the source table and record the source in the fixture's `evidence`.
12. **Check highlight and visual gaps (matrix 5 gaps 11 to 13) are overstated by the literal proxy.** `check-highlight.md` found that almost every check already carries `subject.entity_id`; the real holes are typed (empty or collection-level subjects, and a few owners that are not scene entities). Visual grips and diagrams are also partly closed by `visual-grips-v2.md` and `check-highlight.md`. Tag: `implementable now`. Next action: close the typed holes listed in `check-highlight.md` and add one runtime test per family that asserts the highlighted entity; re-measure with a scene test once `builds` is green.

Excluded from the top 12 (noted for the next matrix): the coordinator says 396 NDP rows verified while the registry says 258; `open-items-sweep-v2.md` lists 7 open en1997 items and `open-items-sweep.md` lists the en1996 and en1999 items; the freeze file still names `build-en1994`.

## 4. Blocker summary

- `builds`: gaps 1, 5, 6 (confirm), 7.
- `printed standards`: gaps 4, 11 (confirm values).
- `generator`: gaps 3, 9 (EN 1997 enum leaf).
- `implementable now`: gaps 2, 6 (grep), 8, 9 (EN 1998 leaves), 10, 11 (repair), 12.

## 5. Caveats

- Spec-trace, leaf coverage, handle-path and fixture results are measured on the trees as they were at 04:04 to 04:08. Owners may still be editing.
- The fixture checker runs were read-only (no `--write` or fix flags used).
- Matrix 6 numbers for "live files" and "typed collections" are proxy counts and are not compared to a build.
- Nothing was compiled; "build status" is the newest log per family, not a fresh run.

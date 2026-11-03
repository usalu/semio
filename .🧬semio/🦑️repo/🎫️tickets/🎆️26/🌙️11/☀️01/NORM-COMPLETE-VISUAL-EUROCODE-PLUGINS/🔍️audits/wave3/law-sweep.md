# W3-LAW-SWEEP: law sweep log

Runner: `🗑️generated/law-sweep/run-law.sh`, wrapped in `build-runner/heavy.sh`, `nice -n 19`, `python -I` (`/home/deck/semio/.venv/bin/python`). Raw logs: `🗑️generated/law-sweep/pass1.*.log`.

## Pass 1 (2026-11-03 ~15:20, heavy.sh waited for the en1996 graph-plan to clear)

| Law / check | Result | Most likely responsible change |
|---|---|---|
| binary-tags | ok (2553 leaves) | - |
| check-stations | ok | - |
| handle-paths | ok | - |
| i18n-mutations | ok (3121 leaves) | - |
| i18n-sinks | FAIL: en1997 `piles/🦀️.rs` gate reason "raft_depth_factor must be below 1" has no coded rule | en1997 pile check (15:07) |
| kind-kebab | FAIL: 2 mismatches in en1994 (`change-stud-x-m` must be `change-stud-xm`, `change-stud-f-u-pa` must be `change-stud-fu-pa`) | en1994 studs conversion mutations (15:09) |
| ndp-adoption | FAIL: private NDP list `NDPS` remains in en1998 `assessment-state/🦀️.rs` | en1998 assessment-state (15:12) |
| ndp-registry-law | ok (3774 rows) | - |
| ndp-registry | ok (374 lookups, 843 literals) | - |
| ownership | ok (5 cases) | - |
| path-budget | FAIL: 32 paths over 240 bytes (longest 248), all en1997/en1998 `change-ground-investigation` and `change-seismic-slope-topographic-amplification` mutation fixtures | en1997/en1998 mutation fixture names |
| remedy-law | ok (510 sites, 0 findings) | - |
| source-budget | FAIL: en1994 `snapshot/packages/rust` at 133 % (191190 B, 7 files); 15 crates above 85 % | en1994 snapshot packages |
| utilization-boundary | ok (7 cases) | - |
| fixture en1990 | ok (148 scenarios) | - |
| fixture en1991 | ok (274) | - |
| fixture en1992 | ok (165) | - |
| fixture en1993 | FAIL 1/665: `change-joint-beam-web-thickness` refused, names `BeamWebThickness`, no such key | en1993 joint beam web mutation |
| fixture en1994 | FAIL 14/132: stud mutations (`change-beam-stud-*`, `change-stud-*`, `insert-stud`, `remove-beam`) after-snapshot mismatch or refused | en1994 studs[] conversion (requests.md, en1994 studs[] item) |
| fixture en1995 | ok (208) | - |
| fixture en1996 | ok (147) | - |
| fixture en1997 | ok (222) | - |
| fixture en1998 | ok (335) | - |
| fixture en1999 | ok (223) | - |
| schema twins check --strict | see pass 1 addendum below | - |

No trivial fixes were applied. Every failure is either a kind/key naming change, a stud-collection migration in en1994, or a gate-reason rule, and each needs a schema or generator change, so they were logged as `[open]` in `📌️important/requests.md`. No artifact was edited, and en1996 (the freeze holder) was not touched.

## Pass 1 addendum: `repo ./📜️script.ts schema twins check --strict` (exit 1, run after the en1995 build lease cleared)

- identical=7744, style=2, drift=44 (rust 8, typescript 14, graphql 11, protobuf 11; json style 2).
- Drift by artifact (twin files): en1999 12, en1991 13, en1994 11, en1996 4 (plus style on en1994 schema JSON and snapshot JSON).
- Likely responsible: twins not regenerated after the en1994 studs[] edits (15:09) and the en1991/en1999 wave-3 schema JSON edits. Not fixed here: twins are generated, never hand-edited, and this needs the schema-first generator run by the owner. Logged as `[open]` in `📌️important/requests.md`.
- Log: `🗑️generated/law-sweep/pass1.twins2.log`. The first heavy.sh attempt produced no output (see note below).

Note: the first heavy.sh call for the twins check returned exit 0 with only a wait line and no check output. A direct run with `repo` gave the result above (exit 1). Treat the heavy.sh run as not a valid pass.

## Pass 2 (2026-11-03 ~16:00-16:25, W3-LAW-SWEEP-2)

Light tests and fixture checkers ran directly under `nice -n 19` with `python -I` (not through `heavy.sh`, which queued behind graph-plan and the heavy slot for 10+ min per test). Raw logs and summary: `🗑️generated/law-sweep/pass2.*.log`, `pass2.lines`. The driver is `🗑️generated/law-sweep/pass2-all.sh`.

| Law / check | Pass 1 | Pass 2 | Change / likely owner |
|---|---|---|---|
| binary-tags | ok (2553) | ok (2584) | - |
| check-stations | ok | ok | - |
| handle-paths | ok | ok | - |
| i18n-mutations | ok | ok (3157) | - |
| i18n-sinks | FAIL en1997 piles | ok | turned green (en1997 gate reason rule landed) |
| kind-kebab | FAIL 2 (en1994) | ok (2582, 0 mismatches) | turned green |
| ndp-adoption | FAIL NDPS in en1998 | ok (3830 entries) | turned green |
| ndp-registry | ok | ok (392 lookups) | - |
| ndp-registry-law | ok | FAIL: `ndp.en1997.6.6.t90-time-factor` cites Terzaghi (not an EN 1997 document) | NEW red, en1997 owner (already in requests as w3-lawfix) |
| ownership | ok | ok | - |
| path-budget | FAIL 32 paths over 240 B | FAIL "walkers disagree" (2, then 1 on rerun; 0 on a direct set diff) | NEW, transient: the two walkers race with concurrent writes in the tree. Long-path check was not reached. Test robustness, not a path defect |
| remedy-law | ok (510 sites) | ok (519 sites) | - |
| source-budget | FAIL en1994 snapshot 133 % | ok (0 over boundary, 18 above 85 %) | turned green (warning band grew 15 to 18) |
| utilization-boundary | ok | ok | - |
| fixture en1990 | ok | ok (148) | - |
| fixture en1991 | ok | CRASH `KeyError: 'rule'` in shared `🔮️oracles/🏃️execution/🔧️verbs/🐍️.py` `declare_kinds` (edit row without `rule`) | NEW, leaf-kinds / en1991 rows |
| fixture en1992 | ok | FAIL 1/166: `change-member-bearing-width` refused (no such key) | NEW, en1992 owner |
| fixture en1993 | FAIL 1/665 | ok (666, 0 failures) | turned green |
| fixture en1994 | FAIL 14/132 | FAIL 3/133: `insert-stud` refused (no element), `remove-beam` after-snapshot differs x2 | narrowed 14 to 3. Studs: W3-EN1994-STUDS / W3-EN1994c cascade (already logged) |
| fixture en1995 | ok | ok (208) | - |
| fixture en1996 | ok | FAIL 1/148: `change-wall-web-spacing-m` refused (no such key) | NEW, en1996 owner (wall web spacing NDP and evolve spec) |
| fixture en1997 | ok | ok (222) | - |
| fixture en1998 | ok | ok (336) | - |
| fixture en1999 | ok | FAIL 1/224: `change-conn-load-eccentricity` refused (no such key) | NEW, en1999 owner |
| schema twins check --strict | exit 1, drift 44 | NOT RUN: queued in heavy.sh, killed by the 20-minute limit (exit 124, never started) | skipped per the 20-minute rule. Pass 1 drift (44) is unverified now |
| Cargo.toml package names | - | No duplicate package names among repo-owned crates (4372 manifests; duplicates only in `⚡️cache`, `.cache`, ticket copies, vendored/registry trees). Scope check only, not a build-graph proof | ok |

Scanners (run in pass 2, no pass 1 baseline): `🧰️scripts/kind-kebab/scan.py` 0 findings (green). `🧰️scripts/split-refs/scan.py` 26 crates, 106 `missing crate::` references (log `🗑️generated/law-sweep/pass2.scan-split-refs-scan.log`). `🧰️scripts/compile-review-q/scan.py` 574 `use-unresolved` and 39 `intra-unresolved` lines (log `pass2.scan-compile-review-q-2.log`; first run crashed on a file removed mid-scan, rerun OK). These scanners need a baseline from the owners before they count as red; they are logged as `[open]`.

New red items (one `[open]` line each in `📌️important/requests.md`): ndp-registry-law en1997 (duplicate of w3-lawfix line, kept for the sweep count), path-budget walker race, fixture en1991 KeyError, fixture en1992, fixture en1994 insert-stud, fixture en1996, fixture en1999. Still open from pass 1: fixture en1994 remove-beam (cascade), and the schema twins drift, which pass 2 could not re-measure.

No artifact, twin, or source file was edited. The freeze holder was not touched.

## Pass 3 (2026-11-03 ~16:55-17:10, W3-LAW-SWEEP-3)

Same method as pass 2: every law test and fixture checker one at a time under `nice -n 19` with `python -I`, the three scanners, and the duplicate-package-name check, all run by `🗑️generated/law-sweep/pass3-all.sh` (log lines in `pass3.lines`). The twins check ran through heavy.sh with a 30 min cap (`twins-pass3.sh`, log `pass3.twins.log`). No artifact, twin, or source file was edited. The freeze file was empty throughout.

| Law / check | Pass 2 | Pass 3 | Change / owner |
|---|---|---|---|
| binary-tags | ok (2584) | ok (2582) | - |
| check-stations | ok | ok | - |
| handle-paths | ok | ok | - |
| i18n-mutations | ok (3157) | ok (3155) | - |
| i18n-sinks | ok | ok | - |
| kind-kebab | ok (0) | ok (2580 leaves, 0 mismatches) | - |
| ndp-adoption | ok (3830) | ok (3836) | - |
| ndp-registry | ok | ok (392 lookups, 874 literals, 3624 unverified) | - |
| ndp-registry-law | FAIL en1997 t90 (Terzaghi) | ok (3836 rows) | turned green (en1997 fixed its t90 citation) |
| ownership | ok | ok | - |
| path-budget | FAIL walker race | ok (59288 paths, longest 240, 0 over) | turned green (race gone; 240 B boundary holds) |
| remedy-law | ok (519) | ok (519, 0 findings) | - |
| source-budget | ok (0 over, 18 above 85 %) | FAIL: 2 crates over the boundary, 16 above 85 % | NEW red: en1991 `scene-members` `packages/rust` 102 % (262970 B, 16 files); en1992 `scene` `packages/rust` 114 % (300075 B, 30 files). Owners: en1991 and en1992 (SCENE work in each) |
| utilization-boundary | ok (7) | ok (7) | - |
| fixture en1990 | ok (148) | ok (149 scenarios, 0 failures) | - |
| fixture en1991 | CRASH KeyError `rule` | FAIL: `ValueError` `insert-fatigue-model` declares an edit without a `rule` (stale taxonomy row, items `item`, `path`) | still red, different symptom. Owner: en1991 (shared taxonomy generator, LEAF-KINDS) |
| fixture en1992 | FAIL 1/166 `change-member-bearing-width` | ok (166, 0 failures) | turned green |
| fixture en1993 | ok | ok (663, 0 failures) | - |
| fixture en1994 | FAIL 3/133 (insert-stud, remove-beam x2) | ok (133, 0 failures) | turned green (studs cascade landed) |
| fixture en1995 | ok (208) | ok (208, 0 failures) | - |
| fixture en1996 | FAIL 1/148 `change-wall-web-spacing-m` | ok (148, 0 failures) | turned green |
| fixture en1997 | ok | ok (222) | - |
| fixture en1998 | ok | ok (336) | - |
| fixture en1999 | FAIL 1/224 `change-conn-load-eccentricity` | ok (224, 0 failures) | turned green |
| scan kind-kebab | 0 findings | 0 findings | - |
| scan split-refs | 26 crates, 106 `missing crate::` | 24 crates, 104 `missing crate::` | still red, improved. Owners by crate |
| scan compile-review-q | 574 use-unresolved, 39 intra-unresolved | 574 use-unresolved, 39 intra-unresolved (unchanged) | still red, no baseline yet. Owners by crate |
| Cargo package names | ok (scope check) | ok (0 duplicated among repo-owned manifests) | - |
| schema twins check --strict | NOT RUN (queued, 20 min limit) | see the twins addendum below | - |

Pass 2 red items that turned green: ndp-registry-law, path-budget, fixture en1992, en1994, en1996, en1999. Still red: fixture en1991 (symptom changed). Newly red: source-budget (en1991, en1992 scene packages/rust).

### Pass 3 twins addendum

Completed at ~17:05 with exit 1 and no drift count: the check crashed while resolving schemas (`[schema twins] En1993Mutation: unresolved reference .../en1993/mutation/change-plated-transverse-stiffener-inertia/schema.json`). It stopped before the identical/style/drift tally, so pass 1 drift (44) is still unverified. Log: `🗑️generated/law-sweep/pass3.twins.log`. Owner: en1993 (the mutation schema references a stiffener-inertia row that the index does not contain). Logged as `[open]` in `📌️important/requests.md` (tag w3-law-sweep-3).

# Norm scene sweep (2026-11-25)

## Dependencies and conditions (read first)
- The re-blessed en1997 scene golden depends on another agent's uncommitted change: `lateral_head_deflection(h, lambda, k, d, length)` (finite pile) in `en1997/…/🧬️schema/💡️inferences/⚖️checks/🌋️lateral-and-seismic-checks/🦀️.rs`. If the owner reverts it, the golden must be reverted to `🗑️generated/en1997-golden-before.json` (the new one is `en1997-golden-new.json`). Re-check once at the end against that file's then-current content. Checked at 11:23:55 (`date`): line 35 still reads `head_deflection: piles::lateral_head_deflection(h, lambda, k, d, pile.length)`, the function is `pub fn lateral_head_deflection(h: f64, lambda: f64, k_h: f64, d: f64, length: f64) -> f64` in `⚖️checks/📇️catalogue/🪵️piles/🦀️.rs` line 126, file mtime (`stat`) Nov 25 03:20:06.
- All results below are runs of the coordinator's private copy (debug mounts stubbed, plain dev profile, copy of 10:27 unless stated). I ran no build successfully myself: my own lane runs were refused (source boundary) or stopped at the foreign `mod dbg` mounts, which I did not touch.
- Wall-clock budget tests (en1994 live-law drag, din4108 thousand elements, din16798 thousand rooms) are not verdicts in the dev profile; no budget and no code was changed for them.

## Results actually observed
| family | observation | source |
|---|---|---|
| en1994 scene | 60 passed, 4 failed (drag budget = profile, `beams_are_coloured_per_station…`, `golden_scenes_match_the_committed_bytes` = first recording, `slip_arrows_follow…`); copy of 10:27, before the slip, per-station, fixture and probe-removal edits | `cp-scene-en1994.log` 11:21 |
| en1994 probe | handles 19 non-stud (9 base + 7 actions + 3 move grips); example `m_ed` 178757 < `m_pla` 662785 so `slip_inputs` peak 0; bridge `eta` 0 | `cp-scene-en1994-probe.log` |
| en1997 scene | 56 passed, 4 failed (2 without Python = copy; golden; handle at zero drag) | `cp-scene-en1997.log` |
| din4108 scene | 15 passed, 4 failed (oracle without Python; thousand-elements 75.8 s = profile; two real) | `cp-scene-din4108.log` |
| din16798 scene | 16 passed, 3 failed (thousand rooms = profile; two real) | `cp-scene-din16798.log` |
| en1991 / en1992 / en1996 scene | test build does not compile (norm test code, below) | `cp-scene-*.log` |
| vdi3805, iso16757, din18599, en1998, en1995 | chain3 `exit=1` each; not read here | `chain3-summary.txt` |
Not yet observed: any run containing the edits below (the copy is being re-synced), the checks/inferences/schema/examples crates of en1994, golden of bridge and showcase.

## Causes and fixes (all unverified until the re-synced run)
### en1994
1. Fire checks have subject entity `""` (insulation field); the scene looked them up by subject. Now `kit::fire_check`/`fire_binding` look up `<prefix><beam>` by id (rows with subject `""` reach every fragment, so incremental = full).
2. `non-degenerate` annotation: a `Dimension` with `from == to`. For beams without studs (the live-law clones: studs belong to `beam-B1`) the degree is `capacity / N_cf = 0`, a legitimate zero, so `Out::annotate` leaves such a dimension out. For the bridge fixture the zero was not legitimate (below).
3. Column buckling returned 0.5: `Props{g:1,a:1,iy:1,iz:1,j:1}` leaves a free torsion mode at `G·J/i_p² = 1/2` (`geometric()` adds `n·(iy+iz)/(a·l)`). `g` and `a` are now large, so only planar flexural buckling is critical; the test checks two lengths and two stiffnesses against `π²EI/L²` and that the first mode is a translation.
4. `layout_stamps` solved a column and a girder eigenproblem every drag step; both solvers are memoised by input bits. They live in the new module `🌀️eigen-modes` of the scene-base crate (carved out of `🎞️tracks` because the scene crate was 4034 of 4000 active lines; mechanics added to the base crate's dependencies).
5. Continuous-beam solver (`🧮️catalogue/📉️continuous`): midpoint integration over 400 cells has O(h²) error (`en1994-deflection-midpoint-bound.py`: 1.152e-7 at u=0.1, 9.375e-8 at u=0.25/0.75, ratio 4.000 per doubling of cells), so the oracle's 1e-12 could never hold. The solver now integrates κ and κ·x per cell with 2-point Gauss (exact for the quadratic moment), cells split at stiffness steps, and the oracle keeps 1e-12. Every continuous-beam check value shifts by about 1e-7 relative; assertions that pin them are listed below and have not run against it.
6. Slip arrows: the engine's plastic force `N_c,req` is 0 when the steel alone carries `M_Ed` (example: 178.8 vs 662.8 kNm), a wrong zero for a drawing of 26 placed studs. `slip_inputs` uses `η_req·N_cf` when `η_req > 0`, otherwise the elastic force `M_Ed·S/I` of the uncracked transformed section (integral of the elastic shear flow `V·S/I`, EN 1994-1-1 6.6.2; sub-clause number to be confirmed by the owner). Test `a_beam_the_steel_carries_alone_still_transfers…` recomputes it by parallel axes and checks the peak flow `4·N_c/L`.
7. Per-station: only sagging/hogging checks carry stations; the governing check of `continuous()` is `en1994.6.6.1.1.layout` (1.0, no stations). Stations are lifted to the worst station-less check of the member, so the member shows that value everywhere; the original assertion (per-station maximum = worst check of the subject) is restored.
8. Bridge fixture: `bridge_girder()` is a fatigue fixture whose 52 studs lay in 0.2–7.85 m of a 25 m span, i.e. none in the right half (η = 0 < η_min = 1; the 6.6.1.2 check reports it). Now 160 studs centred at 12.5 m, pitch 0.15 m (both halves). Other users of `En1994Snapshot::bridge_girder()` (expected to be affected): scene unit tests (lines 65, 117, 428, 469 golden bridge, 767, 785, 805, 825, 878), inferences unit tests (`🧪️tests/🔬️unit` 18, 30), inferences `🔬️compliance-report` (479, 536, 591, 743, 754, 758), editor `🧮️incremental` unit test (line 6). The examples crate and the hub use their own `composite_bridge_girder` source, not this function.
9. Stale tests, each with its document fact: single mode (example has `fire_rating: R60`, `fire_times` gives 30 and 60 min, `fire_track` adds track `fire`; now asserts no `mode` track); placements (id is `placements:document` in `📈️diagrams` line 723 and in en1992 `📈️diagrams` line 383; the test's own later assertions use it); handle counts (`handles::build` appends `move_handles` = `move-x`, `move-y`, `move-yaw`; probe lists 19 non-stud ids = 9 + 7 + 3; test now adds 3 per entity).
10. Golden: HEAD and index hold 0-byte goldens (building, bridge, showcase): the first run is a first recording, not a comparison. `golden-verify-en1994.py` derives ids and stud counts from the document rows. It passes on the first building recording (86 entities, 3 rows, 0 problems; `🗑️generated/en1994-golden-first/🏛️scene.json`), but that file predates the fixes; the golden to accept is the one from the re-synced run, and the golden test counts as unverified until then. The test stops after each first recording, so bridge and showcase need further runs.
11. Pinned continuous-beam values to rerun: `🧮️catalogue/📉️continuous/🧪️tests` (closed forms 1e-6…1e-9, PyNite deflections 1e-3), `🧮️catalogue/🧪️tests`, `📇️catalogue/🧪️tests`, `⚖️checks/🧪️tests/🎯️oracle-wiring`, `🧪️tests/🔬️compliance-report`, schema `⚖️compliance`, `🧮️catalogue-en1994`, `🎯️oracle-comparison-en1994-1`, `🚦️check-outcome-en1994-1`, `third-party-mechanics*.json`. Old/new values: not available yet.

### en1997
- Golden re-blessed: 85 differences, all `pile-P1` (84 `lateral-line` fields: x of the ribbon, `utilisation.ratio` 0.29266605 → 0.29277012, content hash; one `lateral-label` anchor x 0.36108 → 0.36096). Cause: the finite-pile head deflection above; the ratio moves +3.6e-4 and the line x values −3.6e-4. No other entity, annotation, layer, legend or bound differs (`golden-diff-en1997.py`).
- `handles_write_current_values_at_zero_drag`: not the shared kit. `Grip::build` rebases (`rebased` + `with_rest_in_limits`), so the drag parameter is the anchor coordinate; the last lines still passed a displacement 0.5, clamped to −0.1, writing 0.1. The test drags `rest_of(grip) + 0.5`.
- The two Python-oracle tests failed only for lack of an interpreter in the copy (fixed by the coordinator).

### din4108
- `the_cross_check_temperature_factor…`: the `fRsi` vectors carry `rLayers`, not `layers` (the oracle emits resistances); the test builds a one-layer element with that resistance. Fixture facts: `🔮️oracles/🌐️cross-checks/🐍️.py` line 98.
- `the_failing_check_colours_its_element`: undiagnosed. U-value subject is the element and `layers[2]` is the EPS, so a Fail is expected; the assertion message now prints the binding for the next run.
- Oracle test: no interpreter in the copy.

### din16798
- Handle test: not shared; the grip quantises with `round(10.0)`, so `value_at` rounds the stored 1008 to 1010 by design; the test reads `map.base + map.k · anchor.x` (the law "the affine rests on the stored value").
- `diagrams_obey_the_laws`: `Diagram2d::ids` joins series, marker and note ids in one namespace; the series `supplied` collided with the marker `supplied`; series is now `supplied-limit`.

### Compile errors of test code (norm side)
- en1991 unit test `points[1], points[2]` moved a non-`Copy` `Vec2` → `.clone()`.
- en1992: `ColourChannel` import (design-views), `Path3` is an enum (grade-families: `Path3::Polyline`), three calls shadowed by the local `built` (unit: `self::built`).
- en1996: `.detail` → `.accessible_detail`.

## Process facts
- I started two stray unlaned en1994 cargo processes (pids 69939 and 69973, `test -p semio-s-artifact-norm-en1994-scene`, from my `run-one-scene.sh` before the lane wrapper was in place) and killed them. My earlier unlaned run `sweep-en1994-1` made the memory warden cancel it and the chain's vdi3805 attempt (the chain retried). My lane runs `sweep-en1994-3` (refused: source boundary), `-4`, `-5`, `checks-1/2` did not reach compilation or were killed by another agent's pkill; the foreign debug mounts were not touched.
- chain3 failures of en1991, en1992, en1996, en1997, din4108, din16798 were framework breaks (`value-native-owned` signatures, `dbg` mounts), none in norm crates.

## Files changed (real tree)
- en1994 scene: `🧰️kit`, `🔥️fire-section`, `🎞️tracks`, new `🌀️eigen-modes`, `🧱️base/🦀️.rs` and its `Cargo.toml`, scene root `🦀️.rs`, `📈️diagrams`, `🌉️beams`, `🔮️oracles/🐍️.py` (net unchanged), `🧪️tests/🔬️unit`, golden file `🏛️scene/🔣️.json` (emptied, still 0 bytes); `🧬️schema/📸️snapshot` (`bridge_girder()`); checks crate `🧮️catalogue/📉️continuous/🦀️.rs`.
- en1991 `🧪️tests/🔬️unit`; en1992 `🧪️tests/🔬️unit`, `🔭️design-views`, `🧵️grade-families`; en1996 `🧪️tests/🔬️unit`.
- en1997 `🧪️tests/🔬️unit`, `🧪️tests/🧫️fixtures/🏛️scene/🔣️.json`.
- din4108 `🧪️tests/🔬️unit`; din16798 `📈️diagrams/🌬️ventilation`, `🧪️tests/🔬️unit`.
- Ticket folder: `run-one-scene.sh`, `run-lane-pkgs.sh`, `en1994-deflection-midpoint-bound.py`, `golden-verify-en1994.py`, `golden-diff-en1997.py`, this report; `🗑️generated` holds logs and the saved goldens.

## Unverified
Everything under "Causes and fixes" until the re-synced runs; the en1994 checks/inferences/schema/examples crates; the bridge and showcase goldens; the repo-profile verdict on the three time budgets; the din4108 failing-colour cause; vdi3805, iso16757, din18599, en1998, en1995 scene results.

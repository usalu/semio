# Tolerance v2 (wave 2)

Nothing compiled; Python via heavy.sh.

## 1. Scene tone and status through the shared limit rule
- Shared (`🎬️structural-scene/🧱️foundation/🦀️.rs`, re-exported as `scene::*`): `utilisation_tone(ratio)` (Error when `!norm::within_limit`, Warning above 0.8, Info) and `utilisation_status(ratio, near)` (Fail when `!within_limit`, Warning above `near`, Pass). Test `utilisation_tone_and_status_agree_with_the_shared_limit_rule` in `🎬️structural-scene/🧪️tests/🔬️unit/🦀️.rs` (sweeps 0.5 .. 1+1e-6 and asserts tone/status Fail iff `norm::limit_status` is Fail; 1+1e-12 is Warning, not Error).
- Replaced strict `ratio > 1.0` tone/status decisions with the shared helpers: every local `utilisation_tone`/`tone`/`tone_of_ratio` helper (en1991, en1993 x5, en1992 shear, en1990 diagrams, en1994 diagrams, en1995 diagrams + typed entities, en1996 diagrams, en1998 x4, en1999 diagrams) now delegates; inline decisions in en1990 members, en1992 fire-capacity / liquid-pressure / anchors, en1997 settlement / slopes, en1998 pier / shear / set markers, en1999 fire-diagrams required tone, web-panel dash and fire-and-fatigue status, en1994 kit `scaled`, en1996 kit `computed_binding`.
- Kept: geometry/force thresholds (`f_k_n.abs() > 1.0`), en1997 settlement angular `beta > limit` (limit-relative, not a utilisation), en1993 temperature tone `theta_end > theta_cr`.

## 2. Ad-hoc utilisation epsilons
- Rust production: en1991 `covered = selected_demand <= 1.0 + 1e-9` now `norm::within_limit`; tests in en1990/en1991/en1993 asserting `util <= 1.0 + 1e-6/1e-9` now use `within_limit`.
- Python: en1990 rail-and-overrides material rows and en1997 geotechnics self-check use `LIMIT.limit_status` (harness `📏️tolerance`, loaded by walking up to `🔮️oracles/🏃️harness`).
- Left: en1992 studs sentinel `1.0 + 1e-6` (deliberate just-failing utilisation of a misaligned layout, above the tolerance by design, identical in Rust and Python); en1993 `alpha >= 1 - 1e-12` classification geometry; en1997 `gamma_c > 1 + 1e-9` factor assertion; en1991 `loadedArea` input comparisons.

## 3. Mutate dry-run failures
- en1990 `mutate-change-reliability-action-dist`: fixture bug. The vector set the action distribution to `normal` on a document whose action distribution already was `normal` (the resistance twin goes lognormal to normal). Fixed the committed vector (`🧫️fixtures/🧬️mutations/🎲️change-reliability-action-dist/✅apply`): mutation `newActionDistribution` = `lognormal`, after-snapshot and diff patch `actionDistribution` = `lognormal`. No generator exists for these en1990 fixtures. Dry run en1990: mutate 280/0, check-outcome 88/0, oracle-comparison 4/0, round-trip 3/0.
- en1995 `change-deck-compression-c90` (mutate + inverse): handler wiring problem, not fixture. The kind writes `decks[].compressionPerpendicularPa` (declared as a `set-field` edit rule in the artifact's edit-rules and the taxonomy), but the noun `compression-c90` does not match the field, and the Python engine only knew declared rules through `verify_fixtures.py`; no adapter ever called `declare_kinds`. Fix: `🔮️oracles/🏃️execution/🐍️.py` `declare_edit_rules(subset)` loads the taxonomy rows of the subset's artifact (found by walking up to `🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1`) and calls `declare_kinds` in `build_adapter`.

## Verification
- Done: en1990 mutate 280/0, check-outcome 88/0, oracle-comparison 4/0, round-trip 3/0; en1995 mutate 415/0 (was 2 failures).
- Still queued behind other heavy.sh jobs when this was written (loop writing `/tmp/claude-1000/-home-deck-semio/84faf2e3-4721-4a15-8b57-5d44cdd1d7f5/tasks/bcwzr1wd1.output`): regression runs of the engine change on en1998, en1990 (re-run), en1994, en1992 mutate. The boundary test (`⚖️compliance/🧪️tests/🎚️utilization-boundary`) was not re-run; it is untouched by this change. Not compiled: the Rust edits (foundation helpers, scene call sites, test) are checked by reading only.
- Risk: `declare_edit_rules` now activates set-field declarations for every adapter that runs the shared engine; `verify_fixtures.py` (v4) was green for all ten artifacts with exactly these declarations.

- ready for rebuild: tolerance v2 (all ten builds en1990-en1999 latest.md appended)


## Regression runs of the engine change (final)
- mutate dry runs with `declare_edit_rules` (all declared rules): en1995 415/0, en1998 579/0, en1990 280/0, en1992 312 ok / 14 failed, en1994 249 ok / 1 failed.
- en1992 (stud-row insert/remove and `change-stud-row-diameter`) and en1994 (`change-beam-steel-designation`: the catalogue, `declare_catalogue`, is not wired into the adapter either) fail independently of the set-field change: restricting declarations to set-field only made en1992 worse (16 failed), so full declaration (14) stays. These are open items for the en1992/en1994 owners; the same wiring gap (adapter never calling `declare_catalogue`/section catalogue) likely explains en1994.

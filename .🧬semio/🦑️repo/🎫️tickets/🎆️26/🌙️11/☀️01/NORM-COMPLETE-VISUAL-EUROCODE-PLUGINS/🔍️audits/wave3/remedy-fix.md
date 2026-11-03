# Remedy fix (W3-REMEDY-FIX)

Base: `NORM` = `/home/deck/semio/✏️s/🔌️plugins/📕️norm`. Nothing was compiled or run (no cargo, no `repo run`). `freeze.txt` was empty for en1990, en1993, en1994, en1998 and en1999 during the edits; en1995 and en1996 were frozen (graph-plan since 17:02/17:03) when the new law test was inserted into their `✏️editor/🧪️tests/📚️assets/🦀️.rs` (17:07, one test appended; re-plan if the graph restarted). "- ready for rebuild: remedy-fix" is appended to `build-en1990 .. build-en1999/latest.md`.

## 1. Critical remedies (all three fixed)

| Site | Before | After |
|---|---|---|
| EN 1990 `en1990.fat.damage.{member}` (`📊️annex-analyses/🦀️.rs`) | `at_least` on `fatigueSpectra[first].cycles` with a stress value (`Δσ_C · D^(1/3)`), first bin | Governing bin = largest n/N (`governing_spectrum_bin`). Remedy 1: `at_least` on `fatigueSpectra[id=<governing>].detailCategory`, solved by bisection on D <= 1 with the other bins kept (`required_detail_category`). Remedy 2: `at_most` on the same bin's `.cycles` (`allowed_spectrum_cycles`, linear, 1e-9 margin). When neither is reachable (the other bins alone reach D = 1) an advisory. Check subject is the governing bin's `detailCategory`. |
| EN 1990 `en1990.annex-d.{id}.standard` | `at_most` on the text field `pairs` (wrote a number into the test pairs) | Subject `.required` (numeric): `at_most` r_d (rely on at most) plus an advisory "add test pairs and re-evaluate". Same for the non-standard series; its `sampleSize -> 3` remedy (fabricated n) is now an advisory too. |
| EN 1994 `en1994.5.4.2.analysis.{beam}` (`🏗️beams/🦀️.rs`) | `exactly(support, 0)` on the enum `SupportCondition` | `exactly` on `crackedLengthFraction` (to 0), plus a solved typed alternative on `adjacentSpanM` (`at_least` 0.6 l1 or `at_most` l1/0.6). While the fraction is 0 the 15 % rule is not used, so the check is NotApplicable (the remedy therefore clears it). |

Also in EN 1990: the FORM single-action `unwrap_or(mean * 2^40)` fallback is gone (unreachable target = advisory); the multi-action FORM remedy is now solved on the `form_multi` run by bisection (`required_resistance_mean_multi`) instead of the linear estimate, with an advisory when unreachable.

## 2. Shared solver (EN 1993 `🔭️special-parts-checks`, used by `🏛️annex-block-checks`, `🔩️joint-row-checks`)

- `grow(start, limit: Option<f64>, ok) -> Option<f64>` and `shrink(...)`: bisection (64 steps) on the check's own utilisation closure between `start` and the limit (field hard range / physical cap, else `SOLVER_REACH = 100 x start`); a 1e-6 margin is added only if still inside the limit and still `ok`. `None` when `ok` fails at the limit: no value is invented. The ad hoc `|| x > 60.0` stops were turned into the explicit limits (`Some(60.0)`, `Some(80.0)`, lip `0.6 b_f`).
- `solved([Option<Remedy>...])` drops unsolvable remedies. All 25 call sites were converted (cold-formed, plated, shells, joints, pin, tower, pile, crane web, annex blocks, joint rows).
- `emit` fallback: an empty remedy list now becomes `Remedy::advisory(subject, ...)` with a localized reason (EN + DE: "No value of a single field within its limits solves this check: revise several design inputs together."), never an applicable empty `one_of`.
- Note on "schema min/max": the snapshot JSON schemas carry almost no numeric limits (only `minimum 0` and the `u32`/`u8` maxima of counts) and the editors' field meta set `range` only for EN 1997; the limit parameter is therefore filled by the caller where a physical cap exists. The dynamic law checks the field meta range where one exists; schema types and count limits are enforced by the decode of the changed document.
- Also: sheeting restraint remedy is now solved (`C_D,C` from the series formula) instead of `2 x C_D,req`; toughness `one_of` with no option is an advisory; `assumedClass` remedy used invalid enum tokens (now `Rigid`, `Pinned`, `SemiRigid`).
- Tests: `solver_tests` in `special-parts-checks/🦀️.rs` (grow/shrink solve, limit, reach; `solved`), `spectrum_solvers_hold_their_bounds`, `fatigue_remedies_target_the_governing_bin_and_clear_the_damage_sum`, `test_standard_remedy_targets_the_numeric_required_value` (EN 1990 compliance-report test).

## 3. Strengthened remedy law

- Static (`⚖️compliance/🧪️tests/🩹️remedy-law/🐍️.py`, ran: 553 sites, 24 self-test cases, 1 finding): typed remedy on a text/enum field of the artifact snapshot schema, applicable empty `one_of`, solver fallback to an unreachable value (`unwrap_or(.. powi(NN))`). The one finding is not from this task (`en1998 🫧️liquefaction-checks:68` check `en1998.5.{}.spreading` without remedy, logged in requests.md).
- Dynamic (new, Rust): `remedy_law_findings` in `📇️registry/🧬️contract/🖥️app-surface/✏️edit-rules/🦀️.rs` and the test `every_palette_document_obeys_the_remedy_law` in all ten editors, over every palette document (examples and the full showcase) and the fixture-only assets. Per remedy it checks: EN and DE text; advisory = `applicable false`, no options; the target path exists; numeric bounds only on numeric (or bool) fields with a finite value; `OneOf` has options and every option applies; the changed document decodes (enum tokens, counts); the written value is inside the field meta range; after re-evaluation the check's utilisation or severity falls (a `Remove` and the fix of an `.input.` check may drop the check). It returns one message per violation (check id, remedy index, path, rule).
- Fixed by the static scan: en1998 `site.deGroundCombo` (numeric write into an enum, now `one_of` with token), en1999 outline skew (numeric `at_most` on the enum `kind`, now an advisory).
- Not yet seen: the dynamic law has not run. Expected findings are the fixed-factor remedies listed per owner in `📌️important/requests.md` (`[w3-remedy-fix]`).

## 4. Alternatives and text

- EN 1993 `en1993.1-5.5.panel`: besides the solved thickness, `cap` on `platedPanels[id].a` (transverse stiffener spacing, bisection on `panel_shear`) and on `.b` (web height h_w); both dropped when unreachable.
- EN 1991 `en1991.1-7.gas-explosion-domain` volume remedy: text now says that Annex D gives p_d only for the volume range, that the edit changes the modelled enclosure (subdivided or merged compartment), that it does not change the real room and that otherwise another method must be used (EN + DE, range values interpolated).

## Files

- `NORM/⚖️compliance/✅️check-results/🦀️.rs` (`Remedy::advisory`)
- `NORM/📇️registry/🧬️contract/🖥️app-surface/✏️edit-rules/🦀️.rs` (`remedy_law_findings`)
- `NORM/⚖️compliance/🧪️tests/🩹️remedy-law/{🐍️.py,🥒️.feature}`
- EN 1990: `…/⚖️checks/📈️reliability-math/🦀️.rs`, `…/📊️annex-analyses/🦀️.rs`, `…/🧪️tests/🔬️compliance-report/🦀️.rs`
- EN 1991: `…/🌪️wave3-checks/🦀️.rs`
- EN 1993: `…/🔭️special-parts-checks/🦀️.rs`, `…/🏛️annex-block-checks/🦀️.rs`, `…/🔩️joint-row-checks/🦀️.rs`
- EN 1994: `…/🧮️compliance/🏗️beams/🦀️.rs`
- EN 1998: `…/🌍️full-seismic-check/🦀️.rs`; EN 1999: `…/🔷️outline-checks/🦀️.rs`
- Ten editors: `✏️editor/🧪️tests/📚️assets/🦀️.rs`

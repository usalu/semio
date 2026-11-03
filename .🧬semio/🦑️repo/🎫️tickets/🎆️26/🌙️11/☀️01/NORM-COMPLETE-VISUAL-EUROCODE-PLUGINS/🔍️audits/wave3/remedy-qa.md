# Remedy QA (W3-REMEDY-QA, read-only)

Scope: the remedies offered by checks of EN 1990 to EN 1999. Nothing was edited, built or run. Remedies were audited by tracing each check's inputs in the Rust source and by reading how `apply-remedy` writes them back.

Base for all paths: `/home/deck/semio/✏️s/🔌️plugins/📕️norm` (short: `NORM`). Artifact folders are `NORM/🗿️artifacts/<code>/🏅️standards/🔖️1/🪆️subsets/✳️any/`, abbreviated `<code>/…`.

## How remedies are applied (what "effective" means here)

- `NORM/📇️registry/🧬️contract/🖥️app-surface/✏️edit-rules/🦀️.rs:407-465` (`remedy_resolution`): `AtLeast`, `AtMost`, `Exactly` write the raw SI `required` value into `target.path`, coerced by the **current type at that path**. A `String` target (enum or text) receives `format!("{}", required)`, so a numeric remedy on a text or enum field silently writes garbage (no type check).
- `OneOf` with empty `options` and `applicable: true` fails at apply time (`option 0 missing`). It is shown as actionable but cannot be applied.
- `Remove`/`Insert` are rejected by `remedy_edit`; only `apply_remedy_edit` handles them.
- The remedy law (`NORM/…/🧪️tests/🩹️remedy-law`, wave 2) checks only that a remedy exists. It does not apply the remedy, does not check the target type, does not check empty options, and does not re-evaluate. Every defect below passes that law.

## Census (non-test `.rs`, ctor = `Remedy::…(`, `.remedy(` = attachments; approximate, grep-based)

| Code | Ctor | `.remedy(` | Post-pass | Wave-3 modules read | Verdict |
|---|---|---|---|---|---|
| EN 1990 | 42 | 41 | 0 | annex-analyses (fatigue, FORM, multi-FORM, test series, test standard) | Poor: 3 critical sites |
| EN 1991 | 41 | 23 | 0 | wave3-checks (LM3, wind, cscd, vortex, explosion, thermal) | Mixed: vortex solve is correct; volume and fixed-option sites are wrong |
| EN 1992 | 23 | 17 | 34 | not read (minimal-remedies post-pass not audited) | Not audited |
| EN 1993 | 127 | 120 | 1 | special-parts (plated, emit fallback, grow), joint-row, toughness | Mixed: unreachable values, empty-option fallback, single-option panel fix |
| EN 1994 | 63 | 76 | 1 | beams (b_eff, class, support, cracked fraction) | Poor: one enum-field misuse, fixed step |
| EN 1995 | 12 | 14 | 2 | assessment-vocabulary `finish`, entities (creep, deck sizing) | Good core (solver), two heuristics |
| EN 1996 | 71 | 62 | 0 | not read | Not audited |
| EN 1997 | 57 | 55 | 0 | not read | Not audited |
| EN 1998 | 133 | 134 | 1 | three advisories (`applicable: false`), formulas not read | Partly audited, advisories fine |
| EN 1999 | 85 | 45 | 0 | connection-checks (bolts, plates, welds, distances) | Good: linear solves with `u` factor |

## Wrong-direction, wrong-field or ineffective remedies

Severity: C = corrupts data or wrong field; H = misleading or does not reach pass; M = weak or unsolved; L = cosmetic.

| # | Sev | Check id (pattern) | File:line | Problem |
|---|---|---|---|---|
| 1 | C | `en1990.fat.damage.{member}` | `NORM/⚖️en1990/…/📊️annex-analyses/🦀️.rs:17, 28-33` | Subject path is `fatigueSpectra[..].cycles`, but the value written is `detail_category · D^(1/3)` (a stress in Pa). The apply writes a stress into the cycle count. Also targets `bins.first()`, not the governing bin, so raising that bin may not lower D. |
| 2 | C | `en1990.annex-d.{id}.standard` | `…/annex-analyses/🦀️.rs:230, 249` | Subject is `testSeries[..].pairs` (a text field). `at_most` writes `format!("{}", design)` into it: the test-pair data is replaced by a number. |
| 3 | C | `en1994.5.4.2.analy…` (beams) | `NORM/🧩️en1994/…/🧮️compliance/🏗️beams/🦀️.rs:170` | `Remedy::exactly(bref(b,"support"), 0, 0, "Use an uncracked analysis")`. `support` is the enum `SupportCondition` (simply_supported / continuous_2_span). The apply writes `"0"` into it, an invalid enum. The remedy should target `crackedLengthFraction` (already done at line 202). |
| 4 | H | `en1990.annex-d.{id}` (test series) | `…/annex-analyses/🦀️.rs:179` | `at_most` lowers the **required** value to X_d. The check passes by weakening the requirement. "Test more specimens" is text only. Offer a typed `at_least` on sampleSize or an advisory instead. |
| 5 | H | `en1990.annex-d.{id}.sample` (Warning) | `…/annex-analyses/🦀️.rs:210` | `sampleSize → 3` with the same two-test mean and s. The input now describes data that do not exist (fabricated n). Replace with an advisory "perform one more test and enter it". |
| 6 | H | `en1990.annex-c.form.{id}` (single action) | `…/annex-analyses/🦀️.rs:71` | `required_resistance_mean(..).unwrap_or(mean · 2^40)`: an unreachable target becomes a remedy of about 1.1 × 10¹² × μ_R. |
| 7 | H | `en1990.annex-c.form.{id}` (multi, wave 3) | `…/annex-analyses/🦀️.rs:109-110` | Target solved by the linear estimate Δμ = Δβ·σ_total, not solved against `form_multi` β. Non-normal inputs can still fail. Only one remedy (μ_R) for three inputs (R, E1, E2). Solve by bisection on the FORM result and add a reduce-σ_E alternative. |
| 8 | H | `en1993.1-5.5.panel.{id}` (wave 3) | `NORM/🔩️en1993/…/⚖️checks/🔗️special-checks/🔭️special-parts-checks/🦀️.rs:545` (`raise_t`, defined at line 408) | Only the thickness is offered. k_τ depends on a/h_w and on the transverse stiffener inertia I_st, both cited in the explanation, and neither is a remedy. Thickness step is 5 % (`grow`), not solved. |
| 9 | H | all `raise_t` / `grow` users, EN 1993 | `…/special-parts-checks/🦀️.rs:60-70` (`grow`) | `grow` runs 240 steps of ×1.05 and returns the last x even when the target is unreachable (1.05²⁴⁰ ≈ 1.6·10⁵ × start). The remedy then asks for an absurd value. Return `Option` and emit an advisory when unreachable. |
| 10 | H | EN 1993 `emit` fallback | `…/special-parts-checks/🦀️.rs:47` | `Remedy::one_of(subject, Vec::new(), "Revise the design inputs.")` is `applicable: true` with no options. Apply fails with "option 0 missing". Set `applicable: false`. |
| 11 | H | `en1993.1-8.6.2.row.{id}` (wave 3 rows) | `NORM/🔩️en1993/…/🔗️special-checks/🔩️joint-row-checks/🦀️.rs:47-48` | `grow(tp, … || x > 60.0)`: at 60 mm it stops even if the row still fails, and the remedy says "at least t_req mm" anyway. Only the plate thickness is offered (bolt diameter, grade, column flange not offered). |
| 12 | M | `en1993.1-8.6.2.comp.{id}` | `…/joint-row-checks/🦀️.rs:78` | The only typed remedy reduces the applied joint moment (a load). The resistance fix (stiffener, thicker flange) is text only. Make the load reduction an advisory and add a resistance remedy. |
| 13 | M | `en1991.2.lm3-*`, `en1991.1-4.*`, `en1991.1-7.*`, `en1991.1-5.*` (raise assumed inputs) | `NORM/🏋️en1991/…/🌪️wave3-checks/🦀️.rs:86, 134, 215, 242, 281` | Direction is conservative and sign is correct, but these are input inflation, not structural fixes. Acceptable only with an advisory label. |
| 14 | M | `en1991.input.*` (non-positive input) | `…/wave3-checks/🦀️.rs:49, 119, 152, 204, 235, 275` | `positive()` offers fixed options "1", "10", "100" for lengths, pressures and factors. No solve, no physical meaning. Replace with an advisory. |
| 15 | M | `en1991.1-4.vortex-shedding` | `…/wave3-checks/🦀️.rs:183` | Width solve is correct (b ≥ onset·St/n₁). "Stiffen to raise n₁" is text only; `firstFrequency` is not a remedy target. Add a second typed remedy for n₁. |
| 16 | H | `en1991.1-7.gas-explosion` (volume domain) | `…/wave3-checks/🦀️.rs:257` | `Remedy::exactly(volume, clamped)`: changes the enclosure volume to the Annex D validity boundary to pass a validity check. Misleading. Replace with an advisory (other method, or subdivide the enclosure). |
| 17 | M | `en1994.beff` / spacing | `…/beams/🦀️.rs:141` | Raises spacing to `c.beff` (unverified half/full width convention). Add a check on the convention in the test oracle. |
| 18 | M | `en1994` class (web/flange) | `…/beams/🦀️.rs:154` | `× 1.2` fixed step. The class depends on c/t, so solve for t at the class limit. |
| 19 | M | `en1995.floor-creep` | `NORM/🪵️en1995/…/🧱️entities/🦀️.rs:813` | EI scaled linearly by w_fin/L. Ignores shear deflection, not iterated, and quantity kind is `Dimensionless` for N·m²/m. |
| 20 | M | `en1995.deck-sizing` | `…/entities/🦀️.rs:751` | Power law `current · u^exp` with fixed exponents, not re-evaluated. |
| 21 | L | EN 1993 stiffener / stiffener-inertia remedies | `…/special-parts-checks/🦀️.rs` (`Quantity::new(QuantityKind::Dimensionless, …)`) | m⁴ and N·m²/m quantities labelled dimensionless in the UI. |
| 22 | L | `en1998` advisories | `NORM/🫨️en1998/…/🏢️building-regularity-and-base-shear-checks/🦀️.rs:319`, `…/🌉️tower-modal/🦀️.rs:29`, `…/🌉️bridge-spectrum/🦀️.rs:77` | Empty options but `applicable: false` (correct). Fine. Listed to show the pattern is right. |
| 23 | L | `en1995` `finish` empty levers | `NORM/🪵️en1995/…/🧰️assessment-vocabulary/🦀️.rs:257-262` | `applicable: false` (correct). Fine. |

Cross-cutting (not a single site):

- `remedy_resolution` (edit-rules, lines 449-465) coerces the numeric `required` into a String target. Reject a numeric remedy on a non-numeric path.
- The remedy law should apply each remedy with `apply_remedy_edit`, re-evaluate, and assert pass (or documented cap). It should also reject `OneOf` with empty options and `applicable: true`.

## Quality of the sound remedies (kept as reference)

- EN 1991 vortex width: b ≥ 1.25 v_m · St / n₁ (solved, correct direction).
- EN 1995 `finish`: each lever is solved with `solve`, applicable only for float grain (`edit-rules` `applies_as_float`).
- EN 1999 bolt rows, plate thickness, weld throat/length, bolt diameter (√u): linear solves with a 1 % to 10 % margin.
- EN 1990 single-action FORM: `required_resistance_mean` is solved (line 71 is only the fallback).
- EN 1993 panel: thickness direction is correct (`v_b_rd` increases with t).

## Top 20 improvements, ranked

1. Fix `en1990` fatigue (`annex-analyses/🦀️.rs:17, 28-33`): target `detailCategory` of the **governing** bin (max damage contribution), not `cycles`, and not `bins.first()`.
2. Fix `en1990` test standard (`…:230, 249`): remove the write to `pairs`. Offer an advisory, or a numeric remedy on η_d or on the number of tests.
3. Fix `en1994` beams (`…:170`): target `crackedLengthFraction` with value 0 (or make it an advisory). Never write an enum with `exactly`.
4. Add a type guard in `remedy_resolution` (`edit-rules:449-465`): numeric remedies on non-numeric paths fail with a fault, not a silent write.
5. Remove the sampleSize=3 remedy (`…:210`); replace with an advisory.
6. Replace `unwrap_or(mean · 2^40)` (`…:71`) with an advisory "no solution within the FORM bounds".
7. Make `grow`/`shrink` return `Option` (`special-parts-checks/🦀️.rs:60-80`). When unreachable, emit an advisory, not a numeric remedy. Applies to items 8, 9, 11.
8. Set `applicable: false` on the EN 1993 fallback (`special-parts-checks/🦀️.rs:47`) or give it options.
9. Add a remedy-effectiveness test: apply every remedy and re-evaluate. Extend `remedy-law` to fail on empty `OneOf` with `applicable: true` and on wrong target types.
10. EN 1993 panel (`special-parts-checks/🦀️.rs:545`): add typed alternatives for panel length a and transverse stiffener I_st, each solved.
11. EN 1991 gas explosion volume (`wave3-checks/🦀️.rs:257`): replace the `exactly` with an advisory on method or subdivision.
12. EN 1990 multi FORM (`annex-analyses/🦀️.rs:109-110`): solve μ_R on `form_multi` β by bisection; add a σ_E-reduction alternative.
13. EN 1993 joint row (`joint-row-checks/🦀️.rs:47-48`): add typed bolt diameter/grade alternatives; when the 60 mm cap is hit, emit an advisory.
14. EN 1993 compression (`joint-row-checks/🦀️.rs:78`): make the moment reduction an advisory; add a resistance remedy (web stiffener or flange thickness, solved).
15. EN 1991 `positive()` (`wave3-checks/🦀️.rs:49` and its callers 119, 152, 204, 235, 275): replace fixed "1, 10, 100" with an advisory.
16. EN 1991 vortex (`wave3-checks/🦀️.rs:183`): add a typed firstFrequency remedy, n₁ = onset · St / b.
17. EN 1994 class (`beams/🦀️.rs:154`): solve t at the class limit instead of × 1.2.
18. EN 1990 test series (`annex-analyses/🦀️.rs:179`): add a typed "test more" alternative; mark the requirement-lowering option as advisory.
19. EN 1995 creep (`entities/🦀️.rs:813`): iterate with shear deflection, or label as first-order; use a stiffness quantity kind.
20. Quantity kinds (cross-cutting, EN 1993 stiffener inertia, EN 1995 EI): replace `Dimensionless` with the proper kinds so labels and units are right in both locales.

## Coverage and limits

- Read in depth: EN 1990 annex-analyses; EN 1991 wave3-checks (vortex, LM3, wind, cscd, explosion, thermal); EN 1993 special-parts (plated, emit, grow) and joint-row; EN 1994 beams (remedy sites); EN 1995 `finish` and entities (creep, deck); EN 1998 advisories; EN 1999 connection remedies (sampled).
- Not read: EN 1992 (minimal-remedies post-pass), EN 1996, EN 1997, the EN 1993 toughness and crane sites beyond the `one_of` census, and the EN 1998 formulas. Their verdicts are "not audited", not "good".
- The census counts are grep-based and approximate. Several remedies are built through helper functions (for example `lift`, `cap`, `raise_t`), so the per-code counts are lower bounds.
- Nothing was compiled or run. Items 1-3 and 5-6 are derived from the source paths (field name, value written, branch in `remedy_resolution`); they should be confirmed by one apply-and-evaluate test each before a fix.

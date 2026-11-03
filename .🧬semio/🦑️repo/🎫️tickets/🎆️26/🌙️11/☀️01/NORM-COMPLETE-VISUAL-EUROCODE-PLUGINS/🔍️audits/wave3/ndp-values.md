# W3-NDP-VALUES: missing annex values, runtime refusal, annex-blind reads

Date 2026-11-03. Scope: registry `⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json` (3866 rows now), the compliance NDP API, and the ndp-reading check code. No Rust was compiled (the runner does it): `- ready for rebuild: ndp-values` is appended to the `latest.md` of build-en1990 to build-en1999 (the compliance crate is shared by all). Python checks run green: codegen `--check` (3866 entries), `ndp-registry` (392 direct lookups, 878 id literals, 522 annex-blind reads proven equal), `ndp-registry-law`, `ndp-adoption` (164 annex-sensitive values), ndp fixture sync.

## 1. What happened at runtime before (read-only finding)

| Reader | Behaviour on a missing value |
|---|---|
| `ndp_number(id, annex)` (compliance, used by en1991..en1999) | `panic!("NDP … is not a registered number")` for an unset, text or unknown row |
| `ndp_common(id)` | the same panic, plus `assert!(en == de)` panic on divergence |
| en1990 `ndp(annex, id)`, `annex_gamma_m`, action-factors `registry()` | silent `NaN` |
| en1995 `gamma_m_for(..).unwrap_or(f64::NAN)` (12 sites) | silent `NaN` |
| en1997 `lateral_deflection_limit`, `seismic_sliding_factor`, Blum embedment | silent in-code fallback constant under the DE annex (rows had no DE value) |
| `ndp_number_opt` consumers (en1991 hipped, stage return, en1993 plates, en1999) | `None`, mostly handled with an explicit "awaiting values" check; this is the house pattern and stays |

So the answer to the first question: panic in the shared kernel, silent NaN in the helpers, silent fallback in three en1997 reads. None of these was a localized refusal.

## 2. Fix (kernel, with tests)

`⚖️compliance/📜️annex-and-tables/🦀️.rs`:
- `NdpGap { id, annex, reason }` with `NdpGapReason { Unregistered, NoValue, NotNumeric, Divergent }`, `NdpGap::notice()` returns the localized text: EN `NDP <id> has no value for annex EN`, DE `NDP <id> hat keinen Wert für den Anhang EN` (plus not registered, not a number, differs between the annexes).
- `ndp_value(id, annex) -> Result<f64, NdpGap>` (typed read, logs the read); `ndp_number_opt` = `ndp_value(..).ok()`.
- `ndp_number` and `ndp_common` never panic: they log the gap on the thread (`drain_ndp_gaps()`) and return NaN. `ndp_number_or_gap` is the `Option` flavour that logs the gap (for callers that turn `None` into NaN).
- `CheckBuilder::assemble` (`✅️check-results/🦀️.rs`) drains the gaps; any gap turns the check into `UnverifiedBasis` (a pass and a fail alike, never a number), appends `Refused: <notices>` / `Verweigert: <notices>` and cites the gap ids in `ndps`. NotApplicable checks keep their status and leave the gaps for the next check (fail-closed).
- Silent NaN readers now go through the logged path: en1990 `ndp` and `annex_gamma_m`, action-factors `registry`, en1995 `gamma_m_for`.
- Silent fallbacks removed: en1997 `LATERAL_LIMIT_FALLBACK_M`, `SEISMIC_SLIDING_FALLBACK`, `BLUM_FALLBACK` deleted; the three reads use `ndp_number`.

Tests: Rust `a_read_without_a_value_for_the_annex_is_refused_with_a_localized_notice_and_never_defaulted` (compliance unit tests; typed error and both texts, unknown/text/divergent reads, NaN plus logged gap, pass and fail both end `UnverifiedBasis`, drained with the check that took it). Language-agnostic: `🧪️tests/🔖️ndp-registry/🐍️.py` (`read_number`, `read_common`, `refusal`, asserted against the committed registry) and two new assertions in `🥒️.feature`. Not run until the rebuild: the Rust test.

## 3. Missing values: what the audit numbers really were

The audit counted every registry id string literal in check code. Of the ~255 "read ids with no value" almost all are not number reads:

| Class | Rows (read ids still without a value in at least one annex) | Meaning |
|---|---|---|
| citation of a rule or formula block | en1990 10, en1991 41, en1992 24, en1993 57, en1994 13, en1996 79, en1999 2 | provenance row of a clause with several constants (for example `ndp.en1996.m10.rho2`, `ndp.en1993.v-420.1`); the check cites it in `.ndps(..)`; no single number exists. They already end `UnverifiedBasis` on a pass. |
| annex-gated value read | en1991 10 (`alpha-a.de-*`, `alpha-a.en-*`, `vb0.zone*` read only under the annex that holds them), en1992 3 (`nu2-*`, `vrdcc-k`, DE only), en1995 2 (`k-cr`), en1996 4 (`fvlt.*`), en1998 1 (`en3.na.return-period`, `Option` read, EN recommended per limit state comes from `return-period.*`) | designed: the other annex has a different formula; reading them under the wrong annex would now be a refusal, not a panic |
| one-sided EN-only constants | en1991 4 (`stage-return.*`, `Option` read, "awaiting values" check), en1994 3 (stud spacing, edge distance, transverse reinforcement; code holds literals) | DE column missing |
| optional read | en1995 `fire.connection-reduced-load-k` (no value either annex; the outcome `connection_fire_time` is only produced when the case gives `k_fire`) | |

Full list with ids, clause, symbol, values, first use and class: `🔍️audits/wave3/ndp-values-missing.tsv` (254 rows). Per-code counts (read ids with a value missing in an annex, after this work): en1990 10, en1991 55, en1992 27, en1993 57, en1994 16, en1995 3, en1996 83, en1997 0, en1998 1, en1999 2.

### Values added (all `unverified`, evidence names the spec line; the spec is project-written, not the print)

| Row | EN / DE | Source line |
|---|---|---|
| `ndp.en1992.d-03.1` Dc,dur,gamma | 0 / 0 mm | `📐️specs/en1992-target.md:76` |
| `ndp.en1992.d-04.1` Dc,dev | 10 / 10 mm | `en1992-target.md:81` |
| `ndp.en1992.f-03.1` Delta sigma_Rsk(N*) straight bars | 162.5 / 162.5 MPa (DE has an open "phi <= 28?") | `en1992-target.md:333` |
| `ndp.en1993.v-026.1` k | 0.5 / 0.5 | `📐️specs/en1993-target.md:69` |
| `ndp.en1993.v-111.1` lambda_bar_LT,0 | 0.4 / 0.4 | `en1993-target.md:140` |
| `ndp.en1993.v-860.1` gamma_R | 1.5 / 1.5 | `en1993-target.md:292` |
| `ndp.en1993.v-880.1` gamma_M0 | 1.0 / 1.0 | `en1993-target.md:295` |
| `ndp.en1997.7.7.lateral-deflection-limit`, `9.blum-embedment-increase`, `9.seismic.gamma-sliding` | DE column set equal to the EN constant (0.010 m, 1.2, 1.0) | not a spec value: this is the constant the code already applied as fallback; now explicit, unverified, listed for the print (W3-V01) |

Everything else was left missing: the spec line either is absent ("wave2 ndp: value as used by the artifact", 740 rows) or bundles several constants in one row (for example `d-03.2` Table 4.4N matrix, `pr-01.1` k1 and k2, `m50.ties` EN 2.5 and DE 7). Script: `🧰️ndp/w3_ndp_values.py`; scanners `w3_ndp_values_{scan,kind,common,review,table}.py`.

Not changed on purpose: en1999 `ring-stiffener.required-inertia-factor` (check evaluates with margin 1.0 and says "awaiting verified margin"; documented in the row); en1999 web crippling coefficients (check reports awaiting); en1991 hipped roof cells (awaiting values check).

Print checklist: new section `5.4` (W3-V01 to W3-V08) in `🔍️audits/wave2/printed-standards-checklist.md`.

## 4. Annex-blind reads

`ndp_common`, `ndp_cell_common`, `normative_limit` and annex-less `ndp(id)` (en1991 helper) are used 143+ times. All ids they read were proven equal numbers in EN and DE by the registry, so none was replaced (no legitimate divergence exists today):
- Proof is now a test: `ndp-registry` collects every literal id, every template prefix (for example `ndp.en1991.1-4.zeta.`) and every table prefix of those calls and asserts EN and DE are equal numbers (522 rows). The first run found `ndp.en1996.limit.min-thickness` and `pier-area` (EN 0.100 m and 0.04 m2, DE 0.115 m and 0.10 m2); they are read through `normative_limit_for(.., annex)`, and the scan excludes the wrapper's own generic call (`WRAPPED_TABLES`).
- en1997 design sets (`a1 a2 a-acc m1 m2 r1..r4` times `rv rh re b s t st phi c cu gamma g q`) are equal in both annexes; the only divergent one, `r3.a` (EN 1.0, DE 1.1), is read with the annex.
- A future EN/DE divergence now produces a `Divergent` gap and a refused check instead of a panic, and the registry test fails first.

## 5. Open items for owners

- Rust compile and the new unit test: runner (`ndp-values`).
- en1994 stud rows: the constants are literals in the catalogue (`stud-edge-distance`, `stud-cross-spacing`, `min-transverse-reinforcement`); route through the rows once the DE value is read (W3-V05).
- en1995 `connection_fire_time` is silently absent without `k_fire`; an "awaiting value" outcome would be consistent with the house pattern.
- A flag on registry rows for "citation of a rule block" (no number intended) would let a law test separate them from real gaps; it needs a schema change of the NDP model and was not done here.

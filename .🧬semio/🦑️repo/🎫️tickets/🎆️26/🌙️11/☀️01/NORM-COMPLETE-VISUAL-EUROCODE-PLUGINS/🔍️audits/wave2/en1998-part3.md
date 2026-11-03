# EN 1998-3 assessment AS-01 to AS-10 (wave 2)

Nothing compiled or run (no cargo, rustc or `repo run`); checked by reading. Normative values come only from the spec table (3.3 of `📐️specs/en1997-en1998-target.md`); every new registry row is `unverified`.

## Trace (SEI table 33 to 62 ids; `spec-trace` codegen ok for all ten codes)
| Id | Realisation | Status before |
|---|---|---|
| AS-01 | Check `.analysisMethod`, `.capacity`; typed `KnowledgeLevel`; CF from rows `ndp.en1998.en3.cf.kl1..3` | typed enum and hard-coded CF existed, untraced |
| AS-02 | Check `.limitState`, `.capacity`; typed `LimitState`; T_R from rows `return-period.nc/sd/dl`, exponent row | hard-coded 2475/475/225, k = 3 |
| AS-03 | Check `.chordRotation.derivation`, `.cyclicShear`; `Mechanism::design_strength` (ductile f_m/CF, brittle f_m/(CF γ_m), γ_c and γ_s from the EN 1992 rows) | ductile part existed |
| AS-04, AS-05 | Check `.chordRotation.derivation` (+ `.chordRotation`); v16's kernel (θ_y with fibre-section φ_y, θ_um) | existed untraced; no duplication |
| AS-06 | New: `part_3_state::cyclic_shear_capacity`, check `.cyclicShear`, input `shearForceDemandN` | missing |
| AS-07 | Check `.chordRotation` (ductile D/C <= 1) + `.cyclicShear` (brittle E_d <= R_d); kernel `demand_capacity_ratio`, `verified`, `member_ratios` | partly (ductile) |
| AS-08 | New: typed kernel enum `AnalysisMethod` (LF, MRS, pushover, NLTH) gated by KL, check `.analysisMethod` (warning plus raise-KL remedy); N2 pushover already existed | missing gating |
| AS-09 | NotModelled with reason: the spec row has no equations or values (jackets, FRP, interface); remedies of the checks are the only retrofit aid | missing |
| AS-10 | Check `.limitState`: registry row `ndp.en1998.en3.na.return-period` (475 yr, NA-DE) reported under the German annex; capacity checks keep the EN periods (see open item) | missing |

## Changes
- Registry (`🧰️ndp/wave4_en1998_part3.py`, codegen run, `--check` ok): 20 rows `cf.kl1..3`, `return-period.nc/sd/dl/exponent`, `na.return-period`, `shear.{axial-limit, ductility-cap, ductility-slope, base, rho-min, slenderness-slope, slenderness-cap}`. `part_3::confidence_factor`, `limit_state_return_period_yr`, `limit_state_ag_factor` read them via `ndp_common` (signatures unchanged, no caller edits, values identical so no outcome change).
- Kernel `🧬️schema/📐️formulas/🏚️assessment-state/🦀️.rs` (mounted as `part_3_state`, re-exported by the schema crate): `AnalysisMethod`, `Mechanism`, `Basis`/`basis`, `ShearCoefficients`, `plastic_ductility`, `cyclic_shear_capacity`, `assessment_shear_capacity`, `MemberRatios`/`member_ratios`, `knowledge_level_token`. Unit tests with a hand-computed worked example (V_R = 222 975.46 N).
- Schema-first input `assessments[].shearForceDemandN` (default 0 = not given) through the field-adder (`specs/en1998-part3.json`): schema JSON, twins (generator run, strict check clean over all norms), model, diff, SQLite (sql, rs, ts, tests, fixture), field-meta EN + DE, 4 DSL assets, 623 mutation fixtures, 7 check-outcome fixtures, showcase (as-2 has V_Ed = 100 kN; showcase generator line updated, the file patched by hand because the generator fails on the stale `importanceClass` "iI").
- Mutation leaf `change-assessment-shear-force-demand-n` (script `🧰️scripts/en1998-part3/1-shear-kind.py`: crate `mut-assess-section`, root enum, KINDS, edit rule, demo cases, text codec, binary tag, harness, feature, oracle vectors, taxonomy, fixtures); root mutation twins regenerated.
- Checks (`🧬️schema/💡️inferences/🏚️assessment-checks`, mounted in `🧱️details`): `en1998.3.<id>.cyclicShear` (silent without V_Ed or section; stirrup-spacing bisection remedy, else limit on V_Ed), `.analysisMethod` (always, pass or warning), `.limitState` explanation now carries T_R, a_g factor, CF and the NA-DE period.
- 3D and diagrams: member colour field `assessment-dc` (governing D/C of chord rotation and shear, active channel; v16's `chord-rotation-utilisation` kept as fields[0]) with legend, member binding = the worse of the two checks, badge label `<id>:badge` ("KL2 · CF 1.20 · T_R 475 yr · D/C 0.50 · E/R 0.33"), grip `<id>:member:shear-demand` (SetFieldRule exists via the new leaf), diagram `assessment-dc:<id>` (module `🎬️scene/🏚️assessment`, D/C of both mechanisms vs the limit 1, both markers draggable onto `chordRotationDemandRad` and `shearForceDemandN`, k = capacity).
- Tests: kernel tests, scene unit test (grip, diagram, colour, badge), analysis-crate tests (markers and bindings), compliance tests (cyclic shear pass/fail with remedy that fixes it, KL effect on V_R, KL1 pushover warning, registry rows, limit state explanation), spec-trace union extended in the structures test, language-agnostic check-outcome rows pass-57 and fail-58 with two new fixtures and an extension of the independent Python oracle (`cyclic_shear_utilisation`, E_d/R_d for assessments without chord demand), numpy cross-check `🧰️scripts/en1998-part3/2-shear-oracle.py` (passes). No third-party library implements EN 1998-3.

## Gates
Strict twin check: drift 0 (5162 twins). schema-sync: 0 mismatches (2483 units). spec-trace codegen: all ten ok. handle-paths-check en1998: 0 uncovered; handle-edit-rules en1998: none missing. Source-boundary scan: no new OVER (en1998 diff and model-json-schema twins were already over). Fixture checker en1998: 301 scenarios, 6 failures, all `change-bridge-*` (another agent's `pierShearRdN` kind; none of mine). `verify build-owners sync`: see the end of this file.

## Open items and unverified
- Unverified: all `shear.*`, `cf.*`, `return-period.*` rows; the definition μ_pl = θ_Ed/θ_y − 1 (assumption); ρ_tot = (A_s1 + A_s2)/(b h) without web steel; V_w = ρ_w b z f_yw with z = 2d − h; x taken as the yield neutral axis depth; tension gives no axial term; CF applies to brittle strengths like to ductile ones except for DL.
- NA-DE T_R = 475 is recorded and displayed but not applied to the capacity checks (would change every DE assessment outcome and the goldens).
- The analysis method is derived (pushover when every storey has a shear resistance), not an input: the field-adder has no enum branch. NLTH has no assessment check.
- The cyclic shear oracle row needs a section and no chord demand (μ_pl = 0) because θ_y rests on v16's fibre-section kernel.
- Goldens (scene) and the checks counts need one re-bless; nothing was executed.

`verify build-owners sync`: passed (removed 0, added 0, renamed 0, relocated 0, repackaged 0).

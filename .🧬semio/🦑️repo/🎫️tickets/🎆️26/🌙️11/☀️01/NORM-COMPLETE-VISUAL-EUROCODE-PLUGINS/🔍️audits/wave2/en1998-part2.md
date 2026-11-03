# EN 1998-2 bridges (wave 2, PART2)

Nothing compiled or run (no cargo/rustc/`repo run`); checked by reading. Gates run: spec-trace codegen `--write` then verify (`en1998 62 ok`), `handle-paths-check.py en1998` (0 uncovered), `schema twins generate` then `check --strict --under en1998` (identical=588, drift 0; the repo-wide check showed 3 mutation-aggregate drifts caused by concurrent kind additions, a regenerate cleared them), `verify build-owners sync` passed. Source-boundary scan lists pre-existing OVER crates (diff, mechanics, ...), none of the new modules.

## Done
- BR-01: field `bridges[].importanceClass` (1..3, default 2), `part_2::bridge_gamma_i` (rows `en2.gamma-i.{i,ii,iii}` = 0.85/1.0/1.3), used by the bridge evaluation (`evaluate_bridges`), the spectrum analysis and the scene `bridge_view`; check `en1998.2.<b>.action.importance`.
- BR-02: field `verticalPeriodS` (0 = none), row `en2.spectrum.vertical-ratio` 0.7, `part_2::bridge_vertical_spectrum`; check `action.vertical` (F_v = G/g S_ve against G, no uplift); diagram `bridge-spectrum:<b>` (S_e, S_d, S_ve) with a T_v marker draggable onto `verticalPeriodS`.
- BR-03: rows `en2.q.{rc-pier,shear-span-full,shear-span-min}` (3.5, 3, 1), `part_2::{shear_span_ratio, rc_pier_behaviour_factor}`; check `pier.<p>.behaviour` (alpha_s = H / max(b,h); below 1 fails with a height remedy); diagram `pier-q:<b>` with pier markers draggable onto `piers[].heightM`.
- BR-05 (coordinator request): field `pierCrackedInertiaRatio` (1 = gross, no value invented), `bridge::cracked_ratio`; `bridge::supports(.., ratio)` now scales the pier inertia, so the analysis, the demand displacement and the pier-capacity diagram use the same cracked stiffness. The printed EN 1998-2 2.3.6.1 / Annex C value is not in the repo spec ("0.08 ... I_g" is garbled), so it is declared by the designer, not a registry row.
- BR-06: row `en2.combination.minor-factor` 0.3, `part_2::component_combination`; check `pier.<p>.combination` (max(M_x + 0.3 M_y, 0.3 M_x + M_y) against M_Rd). The vertical component enters only the uplift check.
- BR-08: already implemented (`pier.<p>.shear`, V_CD = gamma_0 M_Rd / L_s against V_Rd); trace row added.
- TW-02: already implemented (`.period` of the tower modal check); trace row added.
- Schema/all layers via field-adder (`specs/en1998-part2.json`; schema, diff, sqlite, field-meta, dsl, 611 fixtures, struct literals) and `🧰️scripts/en1998-part2-kinds.py` (leaf kinds `change-bridge-{importance-class,vertical-period-s,pier-cracked-inertia-ratio}` with tags 283-285). Showcase fixture hand-set (vertical 0.25 s). `compliant_de_structures` has T_v = 0.2 s so the check is emitted.
- Tests: `🧪️tests/🏗️structures` oracle test (`bridge_action_rules_match_the_independent_oracle`, python `bridge_action_reference` with itertools/plain arithmetic) and check test (class III raises the bearing displacement, squat pier fails with a heightM remedy, combination fails with a momentRdNm remedy, uplift utilisation = S_ve/g); feature scenario in `🧬️schema-en1998-1/🥒️.feature`. Hand estimate of the compliant structures subject: flexure utilisation about 0.3, combined about 0.4, alpha_s = 4, so it still complies.
- Trace: BR-01/02/03/05/06/08 and TW-02 added to `🧭️spec-trace/🔣️.json`.

## Not done (open)
BR-10 (isolation T_eff, xi_eff, eta, d_cd needs per-bearing hysteretic energy fields), BR-11 (soil-structure springs), BR-12 (spatial variability), ST-03 (buried pipelines), TW-01 (damping/eta per structure type; eta on a design spectrum needs a normative rule), TW-05 (guyed masts). No 3D glyphs/grips for the new fields beyond the 2D diagram grips.

## Unverified
All eight new registry rows (`ndp.en1998.en2.*`, from the repo spec, not the printed standard); the uplift criterion F_v <= G is a project policy; alpha_s uses max(b,h) as the bending depth (conservative); vertical corner periods 0.05/0.15/1.0 come from the existing EN 1998-1 function. Compile errors are possible (`super::Analysis`, `scene::utilisation_tone`, `norm::ndp_common` paths in the two new modules).

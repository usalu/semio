@capability-en1990-1-evaluate
@oracle-en1990-1-compliance-independent
@comparison-numerical-v1
Feature: EN 1990 check outcome of committed documents
  A committed document (mutation snapshots and the hand-built `annex-*` fixtures) is evaluated by the production report and by the document oracle `../../🔮️oracles/🚦️check-outcome/🐍️.py`, which wraps the scipy combination/FORM/Table D1 oracle and divides by the member resistance or criterion for STR/GEO/EQU, 6.11b/6.12b, SLS, A1.4, fatigue, Annex B/C/D and E90.10 to E90.12 (third-party: scipy.stats, anastruct, PyNite).
  Each row names one check id, its status (pass at a utilisation of at most 1, fail above 1) and its utilisation; rows were picked away from the near-limit band so the status is unambiguous.

  @id-check-outcome
  @level-quick
  @mode-differential
  Scenario Outline: <check> reports <status> on <fixture>
    Given the committed document <fixture>
    When the check "<check>" is evaluated
    Then its status is "<status>" with utilisation <utilisation> within relative tolerance <tolerance>
    Examples:
      | id | fixture | check | status | utilisation | tolerance |
      | pass-01 | shared://🧬️mutations/⏱️change-reference-period-years/✅apply/🔣️.after.json | en1990.str.beam-B1.6.10 | pass | 0.664 | 1e-6 |
      | pass-02 | shared://🧬️mutations/⏱️change-reference-period-years/✅apply/🔣️.before.json | en1990.geo.beam-B1.6.10 | pass | 0.664 | 1e-6 |
      | pass-03 | shared://🧬️mutations/⚓️change-permanent-gk/✅apply/🔣️.after.json | en1990.sls.beam-B1.6.14b | pass | 0.768 | 1e-6 |
      | pass-04 | shared://🧬️mutations/📐change-beta-computed/✅apply/🔣️.after.json | en1990.annex-c.beta | pass | 0.88372093 | 1e-6 |
      | fail-05 | shared://🧬️mutations/⏱️change-reference-period-years/✅apply/🔣️.after.json | en1990.annex-c.beta | fail | 1.23684211 | 1e-6 |
      | fail-06 | shared://🧬️mutations/⚓️change-permanent-gk/✅apply/🔣️.after.json | en1990.str.beam-B1.6.10 | fail | 1.096 | 1e-6 |
      | fail-07 | shared://🧬️mutations/⚓️change-permanent-gk/✅apply/🔣️.after.json | en1990.geo.beam-B1.6.10 | fail | 1.096 | 1e-6 |
      | pass-v2-08 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.str.beam-B1.6.10 | pass | 0.415 | 1e-3 |
      | pass-v2-09 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.geo.beam-B1.6.10 | pass | 0.415 | 1e-3 |
      | pass-v2-10 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.geo.beam-B1.set-c | pass | 0.319 | 1e-3 |
      | pass-v2-11 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.6.11b.beam-B1 | pass | 0.325 | 1e-3 |
      | pass-v2-12 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.6.12b.beam-B1 | pass | 0.33 | 1e-3 |
      | pass-v2-13 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.a2.transient.beam-B1 | pass | 0.27625 | 1e-3 |
      | pass-v2-14 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.sls.beam-B1.6.14b | pass | 0.448 | 1e-3 |
      | pass-v2-15 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.sls.beam-B1.6.15b | pass | 0.32 | 1e-3 |
      | pass-v2-16 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.sls.beam-B1.6.16b | pass | 0.288 | 1e-3 |
      | pass-v2-17 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.equ.destab.beam-B1 | pass | 0.822222222 | 1e-3 |
      | pass-v2-18 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.equ.stab.beam-B1 | pass | 0.76146789 | 1e-3 |
      | pass-v2-19 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.a14.deflection.beam-B1 | pass | 0.583333333 | 1e-3 |
      | pass-v2-20 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.a14.deflection-w3.beam-B1 | pass | 0.45 | 1e-3 |
      | pass-v2-21 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.a14.vibration.beam-B1 | pass | 0.545454545 | 1e-3 |
      | pass-v2-22 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.fat.beam-B1 | pass | 0.333333333 | 1e-3 |
      | pass-v2-23 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.fat.damage.beam-B1 | pass | 0.406295581 | 1e-3 |
      | pass-v2-24 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.e10.material.beam-B1 | pass | 0.875 | 1e-3 |
      | pass-v2-25 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.e11.geometry.beam-B1 | pass | 0.5 | 1e-3 |
      | pass-v2-26 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.e12.analysis.beam-B1 | pass | 0.833333333 | 1e-3 |
      | pass-v2-27 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.annex-d.tes-1 | pass | 0.859460946 | 1e-3 |
      | pass-v2-28 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.annex-c.form.rel-1 | pass | 0.89566859 | 1e-3 |
      | pass-v2-29 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.annex-c.beta | pass | 0.88372093 | 1e-3 |
      | pass-v2-30 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.annex-b.dsl | pass | 0.666666667 | 1e-3 |
      | pass-v2-31 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.annex-b.il | pass | 0.666666667 | 1e-3 |
      | pass-v2-32 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.table-2.1.design-working-life | pass | 0.666666667 | 1e-3 |
      | fail-v2-33 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.str.beam-B1.6.10 | fail | 1.84444444 | 1e-3 |
      | fail-v2-34 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.geo.beam-B1.6.10 | fail | 1.84444444 | 1e-3 |
      | fail-v2-35 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.geo.beam-B1.set-c | fail | 1.41777778 | 1e-3 |
      | fail-v2-36 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.6.11b.beam-B1 | fail | 1.44444444 | 1e-3 |
      | fail-v2-37 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.6.12b.beam-B1 | fail | 1.46666667 | 1e-3 |
      | fail-v2-38 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.a2.transient.beam-B1 | fail | 1.22777778 | 1e-3 |
      | fail-v2-39 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.sls.beam-B1.6.14b | fail | 3.73333333 | 1e-3 |
      | fail-v2-40 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.sls.beam-B1.6.15b | fail | 3.2 | 1e-3 |
      | fail-v2-41 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.sls.beam-B1.6.16b | fail | 3.6 | 1e-3 |
      | fail-v2-42 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.equ.destab.beam-B1 | fail | 4.93333333 | 1e-3 |
      | fail-v2-43 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.equ.stab.beam-B1 | fail | 7.2173913 | 1e-3 |
      | fail-v2-44 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.a14.deflection.beam-B1 | fail | 1.45833333 | 1e-3 |
      | fail-v2-45 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.a14.deflection-w3.beam-B1 | fail | 1.5 | 1e-3 |
      | fail-v2-46 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.a14.vibration.beam-B1 | fail | 1.5 | 1e-3 |
      | fail-v2-47 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.fat.damage.beam-B1 | fail | 39.0625 | 1e-3 |
      | fail-v2-48 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.e10.material.beam-B1 | fail | 1.875 | 1e-3 |
      | fail-v2-49 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.e11.geometry.beam-B1 | fail | 2 | 1e-3 |
      | fail-v2-50 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.e12.analysis.beam-B1 | fail | 1.66666667 | 1e-3 |
      | fail-v2-51 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.annex-d.tes-1 | fail | 2.40500241 | 1e-3 |
      | fail-v2-52 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.annex-c.form.rel-1 | fail | 2.3031478 | 1e-3 |
      | fail-v2-53 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.annex-c.beta | fail | 1.31034483 | 1e-3 |
      | pass-v2-54 | shared://🚦️check-outcome/annex-en-610ab-pass/🔣️.json | en1990.str.beam-B1.6.10a | pass | 0.493333333 | 1e-3 |
      | pass-v2-55 | shared://🚦️check-outcome/annex-en-610ab-pass/🔣️.json | en1990.str.beam-B1.6.10b | pass | 0.499333333 | 1e-3 |
      | fail-v2-56 | shared://🚦️check-outcome/annex-en-610ab-fail/🔣️.json | en1990.str.beam-B1.6.10a | fail | 1.48 | 1e-3 |
      | fail-v2-57 | shared://🚦️check-outcome/annex-en-610ab-fail/🔣️.json | en1990.str.beam-B1.6.10b | fail | 1.498 | 1e-3 |
      | pass-a2-01 | shared://🚦️check-outcome/bridge-sls-pass/🔣️.json | en1990.a2.acceleration.beam-B1.bri-1 | pass | 0.6 | 1e-6 |
      | pass-a2-02 | shared://🚦️check-outcome/bridge-sls-pass/🔣️.json | en1990.a2.twist.beam-B1.bri-1 | pass | 0.7 | 1e-6 |
      | pass-a2-03 | shared://🚦️check-outcome/bridge-sls-pass/🔣️.json | en1990.a2.deflection.beam-B1.bri-1 | pass | 0.8 | 1e-6 |
      | fail-a2-04 | shared://🚦️check-outcome/bridge-sls-fail/🔣️.json | en1990.a2.acceleration.beam-B1.bri-1 | fail | 1.4 | 1e-6 |
      | fail-a2-05 | shared://🚦️check-outcome/bridge-sls-fail/🔣️.json | en1990.a2.twist.beam-B1.bri-1 | fail | 1.2 | 1e-6 |
      | fail-a2-06 | shared://🚦️check-outcome/bridge-sls-fail/🔣️.json | en1990.a2.deflection.beam-B1.bri-1 | fail | 1.6 | 1e-6 |
      | pass-a2-07 | shared://🚦️check-outcome/traffic-groups-pass/🔣️.json | en1990.a2.traffic-groups | pass | 1 | 1e-6 |
      | fail-a2-08 | shared://🚦️check-outcome/traffic-groups-fail/🔣️.json | en1990.a2.traffic-groups | fail | 2 | 1e-6 |
      | pass-a2-09 | shared://🚦️check-outcome/traffic-group-table-pass/🔣️.json | en1990.a2.traffic-groups | pass | 1 | 1e-6 |
      | pass-a2-10 | shared://🚦️check-outcome/traffic-group-table-pass/🔣️.json | en1990.a2.traffic-group.TG-ts | pass | 1 | 1e-6 |
      | pass-a2-11 | shared://🚦️check-outcome/traffic-group-table-pass/🔣️.json | en1990.a2.traffic-group.TG-sa | pass | 1 | 1e-6 |
      | fail-a2-12 | shared://🚦️check-outcome/traffic-group-table-fail/🔣️.json | en1990.a2.traffic-groups | fail | 2 | 1e-6 |
      | pass-08 | shared://🚦️check-outcome/annex-en-610ab-pass/🔣️.json | en1990.6.11b.beam-B1 | pass | 0.433333333 | 1e-6 |
      | fail-09 | shared://🚦️check-outcome/annex-en-610ab-fail/🔣️.json | en1990.6.11b.beam-B1 | fail | 1.3 | 1e-6 |
      | pass-10 | shared://🚦️check-outcome/annex-en-610ab-pass/🔣️.json | en1990.6.12b.beam-B1 | pass | 0.44 | 1e-6 |
      | fail-11 | shared://🚦️check-outcome/annex-en-610ab-fail/🔣️.json | en1990.6.12b.beam-B1 | fail | 1.32 | 1e-6 |
      | pass-12 | shared://🚦️check-outcome/annex-en-610ab-pass/🔣️.json | en1990.a2.transient.beam-B1 | pass | 0.368333333 | 1e-6 |
      | fail-13 | shared://🚦️check-outcome/annex-en-610ab-fail/🔣️.json | en1990.a2.transient.beam-B1 | fail | 1.105 | 1e-6 |
      | pass-14 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.annex-b.dsl | pass | 0.666666667 | 1e-6 |
      | pass-15 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.annex-b.il | pass | 0.666666667 | 1e-6 |
      | pass-16 | shared://🚦️check-outcome/annex-all-fail/🔣️.json | en1990.fat.beam-B1 | pass | 0.333333333 | 1e-6 |
      | pass-17 | shared://🚦️check-outcome/annex-en-610ab-pass/🔣️.json | en1990.geo.beam-B1.6.10a | pass | 0.493333333 | 1e-6 |
      | fail-18 | shared://🚦️check-outcome/annex-en-610ab-fail/🔣️.json | en1990.geo.beam-B1.6.10a | fail | 1.48 | 1e-6 |
      | pass-19 | shared://🚦️check-outcome/annex-en-610ab-pass/🔣️.json | en1990.geo.beam-B1.6.10b | pass | 0.499333333 | 1e-6 |
      | fail-20 | shared://🚦️check-outcome/annex-en-610ab-fail/🔣️.json | en1990.geo.beam-B1.6.10b | fail | 1.498 | 1e-6 |
      | pass-21 | shared://🚦️check-outcome/annex-en-610ab-pass/🔣️.json | en1990.geo.beam-B1.set-c | pass | 0.425333333 | 1e-6 |
      | fail-22 | shared://🚦️check-outcome/annex-en-610ab-fail/🔣️.json | en1990.geo.beam-B1.set-c | fail | 1.276 | 1e-6 |
      | fail-23 | shared://🚦️check-outcome/annex-limits-fail/🔣️.json | en1990.annex-b.dsl | fail | 3 | 1e-6 |
      | fail-24 | shared://🚦️check-outcome/annex-limits-fail/🔣️.json | en1990.annex-b.il | fail | 3 | 1e-6 |
      | fail-25 | shared://🚦️check-outcome/annex-limits-fail/🔣️.json | en1990.fat.beam-B1 | fail | 3.33333333 | 1e-6 |
      | fail-26 | shared://🚦️check-outcome/annex-limits-fail/🔣️.json | en1990.table-2.1.design-working-life | fail | 2 | 1e-6 |
      | pass-rb-01 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.a2.deflection.span-1.sls-s1 | pass | 0.8 | 1e-3 |
      | pass-rb-02 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.a2.deflection.span-2.sls-s2 | pass | 0.8 | 1e-3 |
      | pass-rb-03 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.a2.traffic-groups | pass | 1 | 1e-3 |
      | pass-rb-04 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.a2.traffic-group.TG-ts | pass | 1 | 1e-3 |
      | pass-rb-05 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.a2.traffic-group.TG-udl | pass | 1 | 1e-3 |
      | pass-rb-06 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.a2.traffic-group.TG-ped | pass | 1 | 1e-3 |
      | pass-rb-07 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.a2.traffic-group.TG-sa | pass | 1 | 1e-3 |
      | pass-rb-08 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.a2.traffic-group.TG-brake | pass | 1 | 1e-3 |
      | pass-rb-09 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.str.span-1.6.10 | pass | 0.528869048 | 1e-3 |
      | pass-rb-10 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.geo.span-1.6.10 | pass | 0.528869048 | 1e-3 |
      | pass-rb-11 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.geo.span-1.set-c | pass | 0.432511905 | 1e-3 |
      | pass-rb-12 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.6.11b.span-1 | pass | 0.326190476 | 1e-3 |
      | pass-rb-13 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.6.12b.span-1 | pass | 0.127619048 | 1e-3 |
      | pass-rb-14 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.sls.span-1.6.14b | pass | 0.381190476 | 1e-3 |
      | pass-rb-15 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.sls.span-1.6.15b | pass | 0.29047619 | 1e-3 |
      | pass-rb-16 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.sls.span-1.6.16b | pass | 0.0761904762 | 1e-3 |
      | pass-rb-17 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.equ.destab.span-1 | pass | 0.879479167 | 1e-3 |
      | pass-rb-18 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.equ.stab.span-1 | pass | 0.715303346 | 1e-3 |
      | pass-rb-19 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.a14.deflection.span-1 | pass | 0.8 | 1e-3 |
      | pass-rb-20 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.a14.deflection-w3.span-1 | pass | 0.24 | 1e-3 |
      | pass-rb-21 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.a14.vibration.span-1 | pass | 0.714285714 | 1e-3 |
      | pass-rb-22 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.fat.span-1 | pass | 0.64 | 1e-3 |
      | pass-rb-23 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.fat.damage.span-1 | pass | 0.0727546591 | 1e-3 |
      | pass-rb-24 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.e10.material.span-1 | pass | 0.845070423 | 1e-3 |
      | pass-rb-25 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.str.span-2.6.10 | pass | 0.528869048 | 1e-3 |
      | pass-rb-26 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.a14.deflection.span-2 | pass | 0.8 | 1e-3 |
      | pass-rb-27 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.fat.damage.span-2 | pass | 0.0478567141 | 1e-3 |
      | pass-rb-28 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.annex-d.tes-1 | pass | 0.859460946 | 1e-3 |
      | pass-rb-29 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.annex-d.tes-2 | pass | 0.856895393 | 1e-3 |
      | pass-rb-30 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.annex-c.form.rel-1 | pass | 0.854332164 | 1e-3 |
      | pass-rb-31 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.annex-c.beta | pass | 0.77119668 | 1e-3 |
      | pass-rb-32 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.annex-b.dsl | pass | 0.666666667 | 1e-3 |
      | pass-rb-33 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.annex-b.il | pass | 0.666666667 | 1e-3 |
      | fail-rb-34 | shared://🚦️check-outcome/road-bridge-showcase-fail/🔣️.json | en1990.a2.deflection.span-1.sls-s1 | fail | 1.6 | 1e-3 |
      | fail-rb-35 | shared://🚦️check-outcome/road-bridge-showcase-fail/🔣️.json | en1990.a2.traffic-groups | fail | 1.8 | 1e-3 |
      | fail-rb-36 | shared://🚦️check-outcome/road-bridge-showcase-fail/🔣️.json | en1990.str.span-1.6.10 | fail | 2.13402778 | 1e-3 |
      | fail-rb-37 | shared://🚦️check-outcome/road-bridge-showcase-fail/🔣️.json | en1990.geo.span-1.6.10 | fail | 2.13402778 | 1e-3 |
      | fail-rb-38 | shared://🚦️check-outcome/road-bridge-showcase-fail/🔣️.json | en1990.geo.span-1.set-c | fail | 1.77586111 | 1e-3 |
      | fail-rb-39 | shared://🚦️check-outcome/road-bridge-showcase-fail/🔣️.json | en1990.6.11b.span-1 | fail | 1.21277778 | 1e-3 |
      | fail-rb-40 | shared://🚦️check-outcome/road-bridge-showcase-fail/🔣️.json | en1990.a14.deflection.span-1 | fail | 1.2 | 1e-3 |
      | fail-rb-41 | shared://🚦️check-outcome/road-bridge-showcase-fail/🔣️.json | en1990.fat.span-1 | fail | 5.91 | 1e-3 |
      | fail-rb-42 | shared://🚦️check-outcome/road-bridge-showcase-fail/🔣️.json | en1990.fat.damage.span-1 | fail | 1.82291042 | 1e-3 |
      | fail-rb-43 | shared://🚦️check-outcome/road-bridge-showcase-fail/🔣️.json | en1990.annex-d.tes-1 | fail | 1.37513751 | 1e-3 |
      | fail-rb-44 | shared://🚦️check-outcome/road-bridge-showcase-fail/🔣️.json | en1990.annex-c.beta | fail | 1.20820813 | 1e-3 |
      | pass-v30-1 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.annex-c.alpha.rel-1 | info | 0 | 1e-2 |
      | pass-v30-2 | shared://🚦️check-outcome/annex-all-pass/🔣️.json | en1990.annex-c.partial-factors.rel-1 | info | 0 | 1e-2 |
      | pass-v30-3 | shared://🚦️check-outcome/annex-tests-known/🔣️.json | en1990.annex-d.tes-1.direct | info | 0 | 1e-2 |
      | pass-v30-4 | shared://🚦️check-outcome/annex-tests-known/🔣️.json | en1990.annex-d.tes-2.direct | info | 0 | 1e-2 |
      | pass-v30-5 | shared://🚦️check-outcome/annex-tests-known/🔣️.json | en1990.annex-d.tes-2.sample | warning | 0 | 1e-2 |
      | pass-v30-6 | shared://🚦️check-outcome/bridge-life-rc-warning/🔣️.json | en1990.a2.working-life | warning | 0.6 | 1e-2 |
      | pass-v30-7 | shared://🚦️check-outcome/bridge-life-rc-warning/🔣️.json | en1990.annex-b.rc-vs-cc | warning | 0 | 1e-2 |
      | pass-v30-8 | shared://🚦️check-outcome/annex-tests-known/🔣️.json | en1990.annex-b.rc-vs-cc | pass | 0 | 1e-2 |
      | pass-v30-9 | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | en1990.a2.working-life | pass | 1 | 1e-2 |

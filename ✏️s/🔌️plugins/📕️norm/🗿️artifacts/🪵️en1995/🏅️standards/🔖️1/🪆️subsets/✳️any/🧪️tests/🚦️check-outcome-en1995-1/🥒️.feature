@capability-en1995-1-evaluate
@oracle-en1995-1-compliance-independent
@comparison-numerical-v1
Feature: EN 1995 check outcome of committed documents
  A committed before-snapshot of the mutation fixtures is evaluated by the production report and by the independent timber utilisations of `../../🔮️oracles/🚦️check-outcome/🐍️.py` (wrapping the compliance oracle with the design-case families T-14, T-18, T-30 to T-33, T-45, T-46; self-check against sectionproperties, scipy and PyNite).
  Each row names one check id, its status (pass at a utilisation of at most 1, fail above 1) and its utilisation; rows were picked away from the near-limit band so the status is unambiguous. A check that reads k_mod (Table 3.1) or k_def (Table 3.2) is `unverifiedBasis` instead of `pass` while the registry rows `ndp.en1995.kmod.*` and `ndp.en1995.kdef.*` are unverified (no independent source); the oracle counts those reads.

  @id-check-outcome
  @level-quick
  @mode-differential
  Scenario Outline: <check> reports <status> on <fixture>
    Given the committed document <fixture>
    When the check "<check>" is evaluated
    Then its status is "<status>" with utilisation <utilisation> within relative tolerance <tolerance>
    Examples:
      | id | fixture | check | status | utilisation | tolerance |
      | pass-01 | shared://🧬️mutations/↔️change-connection-diameter/✅apply/🔣️.after.json | en1995.6.1.6.bending.beam-B1 | unverifiedBasis | 0.257150909 | 1e-6 |
      | pass-02 | shared://🧬️mutations/↔️change-connection-diameter/✅apply/🔣️.before.json | en1995.7.2.winst.beam-B1 | pass | 0.10899135 | 1e-6 |
      | pass-03 | shared://🧬️mutations/↔️change-connection-edge-distance/✅apply/🔣️.after.json | en1995.7.3.f1.beam-B1 | pass | 0.285339812 | 1e-6 |
      | pass-04 | shared://🧬️mutations/↔️change-connection-end-distance/✅apply/🔣️.after.json | en1995.8.2.2.johansen.conn-C1 | unverifiedBasis | 0.799166323 | 1e-6 |
      | fail-05 | shared://🧬️mutations/↔️change-connection-diameter/✅apply/🔣️.after.json | en1995.8.spacing.a3t.conn-C1 | fail | 1.16666667 | 1e-6 |
      | fail-06 | shared://🧬️mutations/➕️insert-connection-action/✅apply/🔣️.after.json | en1995.8.2.2.johansen.conn-C1 | fail | 1.13370106 | 1e-6 |
      | fail-07 | shared://🧬️mutations/📍️change-member-support/✅apply/🔣️.after.json | en1995.6.1.6.bending.beam-B1 | fail | 1.02860364 | 1e-6 |
      | pass-08 | shared://🧬️mutations/🏠️change-floor-damping/✅apply/🔣️.before.json | en1995.7.2.floor-creep.floor-1 | unverifiedBasis | 0.66 | 1e-6 |
      | pass-09 | shared://🧬️mutations/🏠️change-floor-damping/✅apply/🔣️.before.json | en1995.t-63.f1.floors:floor-1:T-63 | pass | 0.748508929 | 1e-6 |
      | pass-32 | shared://🧬️mutations/➖️remove-panel-layer/✅apply/🔣️.before.json | en1995.t-97.rolling_shear.panels:panel-1:T-97 | unverifiedBasis | 0.233129 | 1e-4 |
      | pass-33 | shared://🧬️mutations/➖️remove-panel-layer/✅apply/🔣️.before.json | en1995.t-97.bending.panels:panel-1:T-97 | unverifiedBasis | 0.465876 | 1e-4 |
      | pass-34 | shared://🚦️check-outcome/deck-pass/🔣️.json | en1995.t-122.prestress_min.decks:deck-1:T-122 | unverifiedBasis | 0.610022 | 1e-4 |
      | pass-35 | shared://🚦️check-outcome/deck-pass/🔣️.json | en1995.t-122.friction.decks:deck-1:T-122 | unverifiedBasis | 0.580973 | 1e-4 |
      | pass-10 | shared://🚦️check-outcome/design-cases-pass/🔣️.json | en1995.t-18.net_section.net | unverifiedBasis | 0.078747 | 1e-3 |
      | pass-11 | shared://🚦️check-outcome/design-cases-pass/🔣️.json | en1995.t-30.tapered_edge.taper | unverifiedBasis | 0.168572 | 1e-3 |
      | pass-12 | shared://🚦️check-outcome/design-cases-pass/🔣️.json | en1995.t-30.tapered_edge.taperc | unverifiedBasis | 0.130167 | 1e-3 |
      | pass-13 | shared://🚦️check-outcome/design-cases-pass/🔣️.json | en1995.t-31.apex_bending.apexb | unverifiedBasis | 0.318749 | 1e-3 |
      | pass-14 | shared://🚦️check-outcome/design-cases-pass/🔣️.json | en1995.t-32.apex_tension90.apext | unverifiedBasis | 0.602905 | 1e-3 |
      | pass-15 | shared://🚦️check-outcome/design-cases-pass/🔣️.json | en1995.t-33.apex_reinforcement.apexr | unverifiedBasis | 0.111249 | 1e-3 |
      | pass-16 | shared://🚦️check-outcome/design-cases-pass/🔣️.json | en1995.t-45.notch_shear.notch | unverifiedBasis | 0.870284 | 1e-3 |
      | pass-17 | shared://🚦️check-outcome/design-cases-pass/🔣️.json | en1995.t-46.notch_reinforcement.notchr | unverifiedBasis | 0.44616 | 1e-3 |
      | pass-18 | shared://🚦️check-outcome/design-cases-pass/🔣️.json | en1995.t-14.bending_6_11.bend | unverifiedBasis | 0.409841 | 1e-3 |
      | pass-19 | shared://🚦️check-outcome/column-pass/🔣️.json | en1995.6.3.2.compression.beam-B1 | unverifiedBasis | 0.134512 | 1e-3 |
      | pass-20 | shared://🚦️check-outcome/column-pass/🔣️.json | en1995.6.3.2.c.combined.beam-B1 | unverifiedBasis | 0.356638 | 1e-3 |
      | pass-21 | shared://🚦️check-outcome/column-pass/🔣️.json | en1995.1-2.4.fire.beam-B1 | pass | 0.332519 | 1e-3 |
      | fail-22 | shared://🚦️check-outcome/design-cases-fail/🔣️.json | en1995.t-30.tapered_edge.taper | fail | 1.685723 | 1e-3 |
      | fail-23 | shared://🚦️check-outcome/design-cases-fail/🔣️.json | en1995.t-31.apex_bending.apexb | fail | 3.18749 | 1e-3 |
      | fail-24 | shared://🚦️check-outcome/design-cases-fail/🔣️.json | en1995.t-32.apex_tension90.apext | fail | 6.02905 | 1e-3 |
      | fail-25 | shared://🚦️check-outcome/design-cases-fail/🔣️.json | en1995.t-33.apex_reinforcement.apexr | fail | 1.11249 | 1e-3 |
      | fail-26 | shared://🚦️check-outcome/design-cases-fail/🔣️.json | en1995.t-45.notch_shear.notch | fail | 8.70284 | 1e-3 |
      | fail-27 | shared://🚦️check-outcome/design-cases-fail/🔣️.json | en1995.t-46.notch_reinforcement.notchr | fail | 4.4616 | 1e-3 |
      | fail-28 | shared://🚦️check-outcome/design-cases-fail/🔣️.json | en1995.t-14.bending_6_11.bend | fail | 4.09841 | 1e-3 |
      | fail-29 | shared://🚦️check-outcome/column-fail/🔣️.json | en1995.6.3.2.c.combined.beam-B1 | fail | 2.003356 | 1e-3 |
      | fail-30 | shared://🚦️check-outcome/column-fail/🔣️.json | en1995.1-2.4.fire.beam-B1 | fail | 1.662595 | 1e-3 |
      | fail-31 | shared://🚦️check-outcome/column-fail/🔣️.json | en1995.6.1.6.bending.beam-B1 | fail | 1.153602 | 1e-3 |
      | fail-32 | shared://🚦️check-outcome/deck-fail/🔣️.json | en1995.t-122.prestress_min.decks:deck-1:T-122 | fail | 1.143791 | 1e-4 |
      | fail-33 | shared://🚦️check-outcome/deck-fail/🔣️.json | en1995.t-122.friction.decks:deck-1:T-122 | fail | 1.089325 | 1e-4 |
      | fail-34 | shared://🚦️check-outcome/deck-fail/🔣️.json | en1995.t-122.bar_force.decks:deck-1:T-122 | fail | 1.243398 | 1e-4 |
      | fail-35 | shared://🚦️check-outcome/deck-fail/🔣️.json | en1995.t-122.plate_bearing.decks:deck-1:T-122 | fail | 8.658009 | 1e-4 |
      | pass-36 | shared://🚦️check-outcome/snow-pass/🔣️.json | en1995.7.2.wfreq.beam-B1 | pass | 0.145322 | 1e-4 |
      | pass-37 | shared://🚦️check-outcome/snow-high-pass/🔣️.json | en1995.7.2.wfreq.beam-B1 | pass | 0.156221 | 1e-4 |
      | pass-38 | shared://🚦️check-outcome/snow-pass/🔣️.json | en1995.7.2.wfin.beam-B1 | unverifiedBasis | 0.255070 | 1e-4 |
      | pass-39 | shared://🚦️check-outcome/snow-high-pass/🔣️.json | en1995.7.2.wfin.beam-B1 | unverifiedBasis | 0.265788 | 1e-4 |
      | pass-40 | shared://🚦️check-outcome/snow-high-pass/🔣️.json | en1995.6.1.5.c90.beam-B1 | unverifiedBasis | 0.208333333 | 1e-6 |
      | pass-41 | shared://🚦️check-outcome/snow-high-pass/🔣️.json | en1995.6.1.7.shear.beam-B1 | unverifiedBasis | 0.262360075 | 1e-6 |
      | pass-42 | shared://🚦️check-outcome/design-cases-fail/🔣️.json | en1995.7.3.stiffness.beam-B1 | pass | 0.0620959048 | 1e-6 |
      | pass-43 | shared://🚦️check-outcome/design-cases-fail/🔣️.json | en1995.7.3.velocity.beam-B1 | pass | 0.00483210947 | 1e-6 |
      | pass-44 | shared://🚦️check-outcome/column-fail/🔣️.json | en1995.8.spacing.a1.conn-C1 | pass | 0.666666667 | 1e-6 |
      | pass-45 | shared://🚦️check-outcome/column-fail/🔣️.json | en1995.8.spacing.a3t.conn-C1 | pass | 0.875 | 1e-6 |
      | pass-46 | shared://🚦️check-outcome/column-fail/🔣️.json | en1995.8.spacing.a4t.conn-C1 | pass | 0.75 | 1e-6 |
      | pass-47 | shared://🚦️check-outcome/deck-pass/🔣️.json | en1995.t-122.bar_force.decks:deck-1:T-122 | pass | 0.355256569 | 1e-6 |
      | pass-48 | shared://🚦️check-outcome/deck-fail/🔣️.json | en1995.t-122.wheel_spread.decks:deck-1:T-122 | pass | 0.18037518 | 1e-6 |
      | pass-49 | shared://🚦️check-outcome/design-cases-pass/🔣️.json | en1995.t-14.bending_6_12.bend | unverifiedBasis | 0.317208133 | 1e-6 |
      | fail-50 | shared://🚦️check-outcome/design-cases-fail/🔣️.json | en1995.t-14.bending_6_12.bend | fail | 3.17208133 | 1e-6 |
      | pass-51 | shared://🚦️check-outcome/design-cases-fail/🔣️.json | en1995.t-18.hole_limit.net | unverifiedBasis | 0.3 | 1e-6 |
      | pass-52 | shared://🚦️check-outcome/design-cases-pass/🔣️.json | en1995.t-30.parallel_edge.taper | unverifiedBasis | 0.117603623 | 1e-6 |
      | fail-53 | shared://🚦️check-outcome/design-cases-fail/🔣️.json | en1995.t-30.parallel_edge.taper | fail | 1.17603623 | 1e-6 |
      | pass-54 | shared://🚦️check-outcome/design-cases-fail/🔣️.json | en1995.t-30.taper_angle.taper | unverifiedBasis | 0.5 | 1e-6 |
      | fail-55 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1995.6.1.5.c90.beam-B1 | fail | 2.734375 | 1e-6 |
      | fail-56 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1995.6.1.7.shear.beam-B1 | fail | 8.60868995 | 1e-6 |
      | fail-57 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1995.7.2.winst.beam-B1 | fail | 46.7300415 | 1e-6 |
      | fail-58 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1995.7.2.wfin.beam-B1 | fail | 97.8734758 | 1e-6 |
      | fail-59 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1995.7.2.wfreq.beam-B1 | fail | 62.306722 | 1e-6 |
      | fail-60 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1995.7.3.f1.beam-B1 | fail | 8.5089617 | 1e-6 |
      | fail-61 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1995.7.3.stiffness.beam-B1 | fail | 1.13074991 | 1e-6 |
      | fail-62 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1995.8.spacing.a1.conn-C1 | fail | 2 | 1e-6 |
      | fail-63 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1995.8.spacing.a4t.conn-C1 | fail | 3 | 1e-6 |
      | fail-64 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1995.t-18.hole_limit.net | fail | 1.5 | 1e-6 |
      | fail-65 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1995.t-18.net_section.net | fail | 13.0290368 | 1e-6 |
      | fail-66 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1995.t-30.taper_angle.taper | fail | 1.5 | 1e-6 |
      | fail-67 | shared://🚦️check-outcome/column-overload-fail/🔣️.json | en1995.6.3.2.compression.beam-B1 | fail | 5.47705798 | 1e-6 |
      | fail-68 | shared://🚦️check-outcome/vibration-fail/🔣️.json | en1995.7.3.velocity.beam-B1 | fail | 2.63756818 | 1e-6 |
      | fail-69 | shared://🚦️check-outcome/deck-wheel-fail/🔣️.json | en1995.t-122.wheel_spread.decks:deck-1:T-122 | fail | 7.21500722 | 1e-6 |
      | pass-70 | shared://🚦️check-outcome/deck-roomy-pass/🔣️.json | en1995.t-122.plate_bearing.decks:deck-1:T-122 | unverifiedBasis | 0.346320346 | 1e-6 |
      | fail-71 | shared://🚦️check-outcome/panel-overload-fail/🔣️.json | en1995.t-97.bending.panels:panel-1:T-97 | fail | 14.1909879 | 1e-6 |
      | fail-72 | shared://🚦️check-outcome/panel-overload-fail/🔣️.json | en1995.t-97.rolling_shear.panels:panel-1:T-97 | fail | 6.00843503 | 1e-6 |
      | fail-73 | shared://🚦️check-outcome/panel-overload-fail/🔣️.json | en1995.t-97.gross_shear.panels:panel-1:T-97 | fail | 1.21875 | 1e-6 |
      | fail-74 | shared://🚦️check-outcome/floor-overload-fail/🔣️.json | en1995.7.2.floor-creep.floor-1 | fail | 101.833333 | 1e-6 |
      | fail-75 | shared://🚦️check-outcome/floor-overload-fail/🔣️.json | en1995.t-63.f1.floors:floor-1:T-63 | fail | 12.964555 | 1e-6 |
      | pass-76 | shared://🧬️mutations/➖️remove-panel-layer/✅apply/🔣️.before.json | en1995.t-97.gross_shear.panels:panel-1:T-97 | unverifiedBasis | 0.05078125 | 1e-6 |
      | pass-v30-1 | shared://🚦️check-outcome/tcc-beam-pass/🔣️.json | en1995.t-90.fibre_1.tcc | pass | 0.411925409 | 1e-2 |
      | pass-v30-2 | shared://🚦️check-outcome/tcc-beam-pass/🔣️.json | en1995.t-90.fibre_2.tcc | pass | 0.582419302 | 1e-2 |
      | pass-v30-3 | shared://🚦️check-outcome/tcc-beam-pass/🔣️.json | en1995.t-90.fastener_1.tcc | pass | 0.606400884 | 1e-2 |
      | pass-v30-4 | shared://🚦️check-outcome/tcc-beam-pass/🔣️.json | en1995.t-90.web_shear.tcc | pass | 0.308370032 | 1e-2 |
      | pass-v30-5 | shared://🚦️check-outcome/tcc-beam-pass/🔣️.json | en1995.t-90.deflection.tcc | pass | 0.370617997 | 1e-2 |
      | fail-v30-6 | shared://🚦️check-outcome/tcc-beam-fail/🔣️.json | en1995.t-90.fibre_1.tcc | fail | 1.23577623 | 1e-2 |
      | fail-v30-7 | shared://🚦️check-outcome/tcc-beam-fail/🔣️.json | en1995.t-90.fibre_2.tcc | fail | 1.74725791 | 1e-2 |
      | fail-v30-8 | shared://🚦️check-outcome/tcc-beam-fail/🔣️.json | en1995.t-90.fastener_1.tcc | fail | 2.42560354 | 1e-2 |
      | fail-v30-9 | shared://🚦️check-outcome/tcc-beam-fail/🔣️.json | en1995.t-90.web_shear.tcc | fail | 1.23348013 | 1e-2 |
      | fail-v30-10 | shared://🚦️check-outcome/tcc-beam-fail/🔣️.json | en1995.t-90.deflection.tcc | fail | 1.35893266 | 1e-2 |
      | pass-v30-11 | shared://🚦️check-outcome/entities-pass/🔣️.json | en1995.t-94.racking.diaphragms:diaphragm-1:T-94 | pass | 0.03125 | 1e-2 |
      | pass-v30-12 | shared://🚦️check-outcome/entities-pass/🔣️.json | en1995.t-94.hold_down.diaphragms:diaphragm-1:T-94 | pass | 0.3 | 1e-2 |
      | pass-v30-13 | shared://🚦️check-outcome/entities-pass/🔣️.json | en1995.t-94.diaphragm_shear.diaphragms:diaphragm-2:T-94 | pass | 0.416666667 | 1e-2 |
      | pass-v30-14 | shared://🚦️check-outcome/entities-pass/🔣️.json | en1995.t-95.bracing_capacity.trusses:truss-1:T-95 | unverifiedBasis | 0.0740740741 | 1e-2 |
      | pass-v30-15 | shared://🚦️check-outcome/entities-pass/🔣️.json | en1995.t-96.bracing_force.bracings:bracing-1:T-96 | unverifiedBasis | 0.333333333 | 1e-2 |
      | pass-v30-16 | shared://🚦️check-outcome/entities-pass/🔣️.json | en1995.t-96.bracing_deflection.bracings:bracing-1:T-96 | unverifiedBasis | 0.444444444 | 1e-2 |
      | pass-v30-17 | shared://🚦️check-outcome/entities-fail/🔣️.json | en1995.t-94.racking.diaphragms:diaphragm-1:T-94 | pass | 0.625 | 1e-2 |
      | fail-v30-18 | shared://🚦️check-outcome/entities-fail/🔣️.json | en1995.t-94.hold_down.diaphragms:diaphragm-1:T-94 | fail | 8 | 1e-2 |
      | fail-v30-19 | shared://🚦️check-outcome/entities-fail/🔣️.json | en1995.t-94.diaphragm_shear.diaphragms:diaphragm-2:T-94 | fail | 1.66666667 | 1e-2 |
      | fail-v30-20 | shared://🚦️check-outcome/entities-fail/🔣️.json | en1995.t-95.bracing_capacity.trusses:truss-1:T-95 | fail | 1.85185185 | 1e-2 |
      | fail-v30-21 | shared://🚦️check-outcome/entities-fail/🔣️.json | en1995.t-96.bracing_force.bracings:bracing-1:T-96 | fail | 1.33333333 | 1e-2 |
      | fail-v30-22 | shared://🚦️check-outcome/entities-fail/🔣️.json | en1995.t-96.bracing_deflection.bracings:bracing-1:T-96 | fail | 2.22222222 | 1e-2 |

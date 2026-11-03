@capability-en1999-1-evaluate
@oracle-en1999-1-compliance-independent
@comparison-numerical-v1
Feature: EN 1999 check outcome of committed documents
  A committed before-snapshot of the mutation fixtures is evaluated by the production report and by the independent aluminium utilisations of `../../🔮️oracles/🚦️check-outcome/🐍️.py` (wrapping the compliance oracle; self-check against sectionproperties, scipy, PyNite).
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
      | pass-01 | shared://🧬️mutations/⚗️change-material-designation/✅apply/🔣️.after.json | en1999.1-3.fat.range.fat-1 | pass | 0.627946054 | 1e-6 |
      | pass-02 | shared://🧬️mutations/⚗️change-material-designation/✅apply/🔣️.before.json | en1999.6.1.4.class.purlin-1.flange-top | pass | 0.63227842 | 1e-6 |
      | pass-03 | shared://🧬️mutations/⚗️change-material-product-form/✅apply/🔣️.after.json | en1999.6.2.3.n.purlin-1 | pass | 0.011037844 | 1e-6 |
      | pass-04 | shared://🧬️mutations/❄️change-sheet-action-category/✅apply/🔣️.after.json | en1999.6.7.4.vb.purlin-1 | pass | 0.0515310964 | 1e-6 |
      | pass-08 | shared://🧬️mutations/⬇️change-patch-position/✅apply/🔣️.after.json | en1999.6.7.5.web.patch-1 | unverifiedBasis | 0.04913374609575917 | 1e-6 |
      | fail-05 | shared://🧬️mutations/⚗️change-material-designation/✅apply/🔣️.before.json | en1999.7.2.defl.purlin-1 | fail | 1.50114312 | 1e-6 |
      | fail-06 | shared://🧬️mutations/⚗️change-material-product-form/✅apply/🔣️.after.json | en1999.8.5.gauge.support-weld | fail | 2.1 | 1e-6 |
      | fail-07 | shared://🧬️mutations/❄️change-sheet-action-category/✅apply/🔣️.after.json | en1999.1-2.fire.fire-1 | fail | 2000000 | 1e-6 |
      | fail-08 | shared://🧬️mutations/❄️change-sheet-action-category/✅apply/🔣️.after.json | en1999.1-4.defl.sheet-pass | fail | 5.357142857 | 1e-6 |
      | fail-09 | shared://🧬️mutations/❄️change-sheet-action-category/✅apply/🔣️.after.json | en1999.1-2.fire.theta-cr.fire-1 | fail | 2.092276028 | 1e-6 |
      | pass-10 | shared://🚦️check-outcome/weld-run-haz-pass/🔣️.json | en1999.6.1.6.haz.weld-1 | pass | 0.333333333 | 1e-6 |
      | pass-11 | shared://🚦️check-outcome/weld-run-haz-pass/🔣️.json | en1999.6.3.1.fb.purlin-1 | pass | 0.20407 | 1e-3 |
      | pass-12 | shared://🚦️check-outcome/weld-run-haz-pass/🔣️.json | en1999.6.3.1.haz.purlin-1 | pass | 0.57166 | 1e-3 |
      | pass-13 | shared://🚦️check-outcome/weld-run-haz-pass/🔣️.json | en1999.6.3.2.ltb.purlin-1 | pass | 0.64457 | 1e-3 |
      | fail-14 | shared://🚦️check-outcome/weld-run-haz-fail/🔣️.json | en1999.6.1.6.haz.weld-1 | fail | 1.93548387 | 1e-6 |
      | fail-15 | shared://🚦️check-outcome/column-welded-fail/🔣️.json | en1999.6.3.1.fb.purlin-1 | fail | 2.17783 | 1e-3 |
      | fail-16 | shared://🚦️check-outcome/column-welded-fail/🔣️.json | en1999.6.3.1.haz.purlin-1 | fail | 24.04904 | 1e-3 |
      | fail-17 | shared://🚦️check-outcome/column-welded-fail/🔣️.json | en1999.6.2.9.nm.purlin-1 | fail | 1.42863 | 1e-3 |
      | pass-18 | shared://🚦️check-outcome/web-stiffener-pass/🔣️.json | en1999.6.7.6.web-stiffness.stiffener-1 | unverifiedBasis | 0.07201157292147886 | 1e-6 |
      | pass-19 | shared://🚦️check-outcome/web-stiffener-pass/🔣️.json | en1999.6.7.6.web-strength.stiffener-1 | unverifiedBasis | 0.10050507072670933 | 1e-6 |
      | pass-20 | shared://🚦️check-outcome/web-stiffener-pass/🔣️.json | en1999.6.7.6.web-stiffness.stiffener-2 | unverifiedBasis | 0.15826006266621281 | 1e-6 |
      | pass-21 | shared://🚦️check-outcome/web-stiffener-pass/🔣️.json | en1999.6.7.6.web-strength.stiffener-2 | unverifiedBasis | 0.11502645896991881 | 1e-6 |
      | fail-22 | shared://🚦️check-outcome/web-stiffener-fail/🔣️.json | en1999.6.7.6.web-stiffness.stiffener-thin | fail | 4.19944270155647 | 1e-6 |
      | fail-23 | shared://🚦️check-outcome/web-stiffener-fail/🔣️.json | en1999.6.7.6.web-strength.stiffener-thin | fail | 1.2228620058873103 | 1e-6 |
      | pass-24 | shared://🚦️check-outcome/calm-pass/🔣️.json | en1999.1-2.fire.fire-1 | pass | 0.0286665447 | 1e-6 |
      | pass-25 | shared://🚦️check-outcome/calm-pass/🔣️.json | en1999.1-2.fire.buckling.fire-1 | pass | 0.0394943449 | 1e-6 |
      | fail-26 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.1-2.fire.buckling.fire-1 | fail | 1000000 | 1e-6 |
      | pass-27 | shared://🚦️check-outcome/calm-pass/🔣️.json | en1999.1-2.fire.ltb.fire-1 | pass | 0.0151339545 | 1e-6 |
      | fail-28 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.1-2.fire.ltb.fire-1 | fail | 1000000 | 1e-6 |
      | pass-29 | shared://🚦️check-outcome/calm-pass/🔣️.json | en1999.1-2.fire.theta-cr.fire-1 | pass | 0.204702235 | 1e-6 |
      | pass-30 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.1-3.fat.fat-1 | pass | 0 | 1e-6 |
      | fail-31 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.1-3.fat.fat-1 | fail | 8.3167404 | 1e-6 |
      | fail-32 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.1-3.fat.range.fat-1 | fail | 1.61471842 | 1e-6 |
      | pass-33 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.1-4.axial.sheet-pass | pass | 0.0880011 | 1e-6 |
      | fail-34 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.1-4.axial.sheet-pass | fail | 36.1030154 | 1e-6 |
      | pass-35 | shared://🚦️check-outcome/calm-pass/🔣️.json | en1999.1-4.bend.sheet-pass | pass | 0.1062072 | 1e-6 |
      | fail-36 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.1-4.bend.sheet-pass | fail | 796.554 | 1e-6 |
      | pass-37 | shared://🚦️check-outcome/calm-pass/🔣️.json | en1999.1-4.defl.sheet-pass | pass | 0.535714286 | 1e-6 |
      | pass-38 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.1-4.local.sheet-pass | pass | 0.727272727 | 1e-6 |
      | fail-39 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.1-4.local.sheet-pass | fail | 3.63636364 | 1e-6 |
      | pass-40 | shared://🚦️check-outcome/calm-pass/🔣️.json | en1999.1-4.nm.sheet-pass | pass | 0.11500731 | 1e-6 |
      | fail-41 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.1-4.nm.sheet-pass | fail | 1.1500731 | 1e-6 |
      | pass-42 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.1-4.shear.sheet-pass | pass | 0.00154325727 | 1e-6 |
      | fail-43 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.1-4.shear.sheet-pass | fail | 13.0919658 | 1e-6 |
      | pass-44 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.1-5.buckle.shell-pass | pass | 0.223714 | 1e-6 |
      | fail-45 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.1-5.buckle.shell-pass | fail | 6.71142 | 1e-6 |
      | pass-46 | shared://🚦️check-outcome/calm-pass/🔣️.json | en1999.1-5.int.shell-pass | pass | 0.0543781547 | 1e-6 |
      | fail-47 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.1-5.int.shell-pass | fail | 1.22249483 | 1e-6 |
      | pass-48 | shared://🚦️check-outcome/calm-pass/🔣️.json | en1999.1-5.ring.shell-pass | pass | 0.111378926 | 1e-6 |
      | fail-49 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.1-5.ring.shell-pass | fail | 1.11378926 | 1e-6 |
      | pass-50 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.1-5.shear.shell-pass | pass | 0.0541299161 | 1e-6 |
      | fail-51 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.1-5.shear.shell-pass | fail | 1.62389748 | 1e-6 |
      | fail-52 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.6.1.4.class.purlin-1.flange-top | fail | 12.4 | 1e-6 |
      | fail-53 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.6.2.3.n.purlin-1 | fail | 7.86885713 | 1e-6 |
      | pass-54 | shared://🚦️check-outcome/weld-transverse-column/🔣️.json | en1999.6.2.5.m.purlin-1 | pass | 0.655469481 | 1e-6 |
      | fail-55 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.6.2.5.m.purlin-1 | fail | 32.1103528 | 1e-6 |
      | pass-56 | shared://🚦️check-outcome/weld-transverse-column/🔣️.json | en1999.6.2.5.mz.purlin-1 | pass | 0.0322631942 | 1e-6 |
      | fail-57 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.6.2.5.mz.purlin-1 | fail | 3.81680869 | 1e-6 |
      | pass-58 | shared://🚦️check-outcome/weld-transverse-column/🔣️.json | en1999.6.2.6.v.purlin-1 | pass | 0.0910740558 | 1e-6 |
      | fail-59 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.6.2.6.v.purlin-1 | fail | 1.60777021 | 1e-6 |
      | pass-60 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.6.2.6.vy.purlin-1 | pass | 0.000727745027 | 1e-6 |
      | fail-61 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.6.2.6.vy.purlin-1 | fail | 1.39818191 | 1e-6 |
      | pass-62 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.6.2.7.t.purlin-1 | pass | 0 | 1e-6 |
      | fail-63 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.6.2.7.t.purlin-1 | fail | 51.0741749 | 1e-6 |
      | pass-64 | shared://🚦️check-outcome/weld-transverse-column/🔣️.json | en1999.6.2.7.vt.purlin-1 | pass | 0.0910740558 | 1e-6 |
      | fail-65 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.6.2.7.vt.purlin-1 | fail | 1000000 | 1e-6 |
      | pass-66 | shared://🚦️check-outcome/weld-transverse-column/🔣️.json | en1999.6.2.8.mv.purlin-1 | pass | 0.655469481 | 1e-6 |
      | fail-67 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.6.2.8.mv.purlin-1 | fail | 1000000 | 1e-6 |
      | pass-68 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.6.2.9.nm.purlin-1 | pass | 0.859011826 | 1e-6 |
      | pass-69 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.6.3.1.fb.z.purlin-1 | pass | 0.571661939 | 1e-6 |
      | fail-70 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.6.3.1.fb.z.purlin-1 | fail | 17.724498 | 1e-6 |
      | pass-71 | shared://🚦️check-outcome/column-welded-fail/🔣️.json | en1999.6.3.1.tb.purlin-1 | pass | 0.898809392 | 1e-6 |
      | fail-72 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.6.3.1.tb.purlin-1 | fail | 8.01041573 | 1e-6 |
      | fail-73 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.6.3.2.ltb.purlin-1 | fail | 33.9901373 | 1e-6 |
      | pass-74 | shared://🚦️check-outcome/calm-pass/🔣️.json | en1999.6.3.3.int.purlin-1 | pass | 0.118435777 | 1e-6 |
      | fail-75 | shared://🚦️check-outcome/weld-run-haz-pass/🔣️.json | en1999.6.3.3.int.purlin-1 | fail | 1.16231234 | 1e-6 |
      | fail-76 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.6.7.4.vb.purlin-1 | fail | 1.60777021 | 1e-6 |
      | pass-77 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.6.7.5.patch.purlin-1 | pass | 0.400839432 | 1e-6 |
      | fail-78 | shared://🚦️check-outcome/web-stiffener-fail/🔣️.json | en1999.6.7.5.patch.purlin-1 | fail | 3.00629574 | 1e-6 |
      | fail-79 | shared://🚦️check-outcome/web-panel-patch-fail/🔣️.json | en1999.6.7.5.web.patch-1 | fail | 3.68503096 | 1e-6 |
      | pass-80 | shared://🚦️check-outcome/calm-pass/🔣️.json | en1999.7.2.defl.purlin-1 | pass | 0.150114312 | 1e-6 |
      | pass-81 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.7.2.defl-qp.purlin-1 | pass | 0.454127331 | 1e-6 |
      | fail-82 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.7.2.defl-qp.purlin-1 | fail | 23.1670946 | 1e-6 |
      | pass-83 | shared://🚦️check-outcome/column-welded-fail/🔣️.json | en1999.7.2.sls-freq.purlin-1 | pass | 0.636673568 | 1e-6 |
      | fail-84 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.7.2.sls-freq.purlin-1 | fail | 13.6764652 | 1e-6 |
      | pass-85 | shared://🚦️check-outcome/column-welded-fail/🔣️.json | en1999.7.2.stress.purlin-1 | pass | 0.803867939 | 1e-6 |
      | fail-86 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.7.2.stress.purlin-1 | fail | 25.5894611 | 1e-6 |
      | pass-87 | shared://🚦️check-outcome/column-welded-fail/🔣️.json | en1999.8.5.block.support-weld | pass | 0.113847747 | 1e-6 |
      | fail-88 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.8.5.block.support-weld | fail | 11.8845267 | 1e-6 |
      | pass-89 | shared://🚦️check-outcome/column-welded-fail/🔣️.json | en1999.8.5.bolt.support-weld | pass | 0.0670900952 | 1e-6 |
      | fail-90 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.8.5.bolt.support-weld | fail | 17.2839576 | 1e-6 |
      | pass-91 | shared://🚦️check-outcome/calm-pass/🔣️.json | en1999.8.5.edge.support-weld | pass | 0.7 | 1e-6 |
      | fail-92 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.8.5.edge.support-weld | fail | 1.4 | 1e-6 |
      | pass-93 | shared://🚦️check-outcome/calm-pass/🔣️.json | en1999.8.5.emax.support-weld | pass | 0.333333333 | 1e-6 |
      | fail-94 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.8.5.emax.support-weld | fail | 6.94444444 | 1e-6 |
      | pass-95 | shared://🚦️check-outcome/calm-pass/🔣️.json | en1999.8.5.gauge.support-weld | pass | 0.933333333 | 1e-6 |
      | pass-96 | shared://🚦️check-outcome/calm-pass/🔣️.json | en1999.8.5.pitch.support-weld | pass | 0.855555556 | 1e-6 |
      | fail-97 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.8.5.pitch.support-weld | fail | 1.54 | 1e-6 |
      | pass-98 | shared://🚦️check-outcome/calm-pass/🔣️.json | en1999.8.5.pmax.support-weld | pass | 0.321428571 | 1e-6 |
      | fail-99 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.8.5.pmax.support-weld | fail | 4.46428571 | 1e-6 |
      | pass-100 | shared://🚦️check-outcome/column-welded-fail/🔣️.json | en1999.8.5.punch.support-weld | pass | 0.284880979 | 1e-6 |
      | fail-101 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.8.5.punch.support-weld | fail | 15.1231946 | 1e-6 |
      | pass-102 | shared://🚦️check-outcome/column-welded-fail/🔣️.json | en1999.8.5.tension.support-weld | pass | 0.39493779 | 1e-6 |
      | fail-103 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.8.5.tension.support-weld | fail | 20.9656716 | 1e-6 |
      | pass-104 | shared://🚦️check-outcome/column-welded-fail/🔣️.json | en1999.8.5.vt.support-weld | pass | 0.349188517 | 1e-6 |
      | fail-105 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.8.5.vt.support-weld | fail | 32.2594374 | 1e-6 |
      | pass-106 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.8.6.haz.support-weld | pass | 0.0458138538 | 1e-6 |
      | fail-107 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.8.6.haz.support-weld | fail | 19.1845513 | 1e-6 |
      | pass-108 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.8.6.length.support-weld | pass | 0.25 | 1e-6 |
      | fail-109 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.8.6.length.support-weld | fail | 2 | 1e-6 |
      | pass-110 | shared://🚦️check-outcome/bolts-spread-fail/🔣️.json | en1999.8.6.weld.support-weld | pass | 0.544271119 | 1e-6 |
      | fail-111 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.8.6.weld.support-weld | fail | 981.344052 | 1e-6 |
      | pass-112 | shared://🚦️check-outcome/column-welded-fail/🔣️.json | en1999.8.sls-freq.support-weld | pass | 0.0623932395 | 1e-6 |
      | fail-113 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.8.sls-freq.support-weld | fail | 14.9743775 | 1e-6 |
      | pass-114 | shared://🚦️check-outcome/ring-stiffener-pass/🔣️.json | en1999.1-5.stiffener-position.ring-1 | pass | 0.5 | 1e-6 |
      | fail-115 | shared://🚦️check-outcome/ring-stiffener-pass/🔣️.json | en1999.1-5.stiffener-position.ring-2 | fail | 1.5 | 1e-6 |
      | pass-116 | shared://🚦️check-outcome/ring-stiffener-pass/🔣️.json | en1999.1-5.stiffener.ring-1 | unverifiedBasis | 0.0664873192 | 1e-6 |
      | pass-117 | shared://🚦️check-outcome/ring-stiffener-pass/🔣️.json | en1999.1-5.stiffener.ring-2 | unverifiedBasis | 0.0332436596 | 1e-6 |
      | fail-118 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.1-5.stiffener.ring-1 | fail | 28.2101139 | 1e-6 |
      | pass-v30-1 | shared://🚦️check-outcome/edge-stiffener-pass/🔣️.json | en1999.6.7.7.edge.beam-slender.flange-top | unverifiedBasis | 0.101407204 | 1e-2 |
      | fail-v30-2 | shared://🚦️check-outcome/edge-stiffener-fail/🔣️.json | en1999.6.7.7.edge.beam-slender.flange-top | fail | 1.63532435 | 1e-2 |
      | pass-v30-3 | shared://🚦️check-outcome/edge-stiffener-fail/🔣️.json | en1999.6.7.7.edge.beam-slender.flange-bot | unverifiedBasis | 0.101407204 | 1e-2 |
      | pass-v30-4 | shared://🚦️check-outcome/edge-stiffener-pass/🔣️.json | en1999.1-4.web-crippling.patch-sheet | unverifiedBasis | 0 | 1e-2 |
      | pass-119 | shared://🚦️check-outcome/calm-pass/🔣️.json | en1999.8.5.net.support-weld | pass | 0.00868770358 | 1e-6 |
      | pass-120 | shared://🚦️check-outcome/web-stiffener-pass/🔣️.json | en1999.8.5.net.support-weld | pass | 0.135142056 | 1e-6 |
      | fail-121 | shared://🚦️check-outcome/overload-fail/🔣️.json | en1999.8.5.net.support-weld | fail | 12.162785 | 1e-6 |

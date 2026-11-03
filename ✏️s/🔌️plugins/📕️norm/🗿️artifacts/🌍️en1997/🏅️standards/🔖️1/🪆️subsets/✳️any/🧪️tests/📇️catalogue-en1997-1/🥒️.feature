@capability-en1997-1-evaluate
@oracle-en1997-1-compliance-independent
@comparison-method-spread-v1
Feature: Evaluate the EN 1997 verification catalogue against an independent Python formulation
  Every catalogue entry of the EN 1997 target specification (PF, G, GEO-F, EQU, UPL, HYD, SLS, PILE, RW, SLOPE, EMB, ANC, DEW, GI)
  is evaluated by the Rust kernel (`🧬️schema/💡️inferences/⚖️checks/📇️catalogue`) on a committed input case and by
  `🔮️oracles/⚖️compliance/🐍️.py --catalogue`, which re-derives the same quantities with a different formulation:
  trial-wedge maximisation for K_a, work-rate integration about the pole of a logarithmic spiral for the curved passive
  coefficient, Boussinesq quadrature for stresses and settlements, Student-t inversion (scipy when present, quadrature
  otherwise) for the characteristic value, `statistics.linear_regression` for the laboratory envelopes, closed-form
  moment polynomials for embedded walls. Both read the same committed cases at
  `shared://📇️catalogue/🔣️.json`; every value the oracle names must agree within one percent.

  Limit outcomes compare their utilisation (demand over resistance), information outcomes their value in SI units
  (N, Pa, m, N·m, degrees). The specification examples EX97-01 … EX97-15 are rows of the same table and are also asserted
  against their hand-check numbers by the Rust unit tests.

  Further specification assertions, asserted by the Rust unit tests of the artifact and not by the platform adapters:
  - The repaired snapshot checks agree with the oracle on the compliant and the non-compliant package: Given the compliant and the non-compliant example snapshot When both implementations evaluate the snapshot Then every check the oracle reproduces agrees within half a percent

  @id-catalogue-fixture-case
  @level-quick
  @mode-differential
  Scenario Outline: Evaluate <entry> case <index> and compare every named result
    Given the committed catalogue case shared://📇️catalogue/🔣️.json at index <index>
    When both implementations evaluate catalogue entry <entry> on that case
    Then every result named by the oracle agrees within the comparison profile
    Examples:
      | id           | index | entry     | label                             |
      | pf-01-0      | 0     | PF-01     | De da2 bsP                        |
      | pf-01-1      | 1     | PF-01     | De da2 bsT                        |
      | pf-01-2      | 2     | PF-01     | De da2 bsA                        |
      | pf-01-3      | 3     | PF-01     | De da2star bsP                    |
      | pf-01-4      | 4     | PF-01     | De da3 bsP                        |
      | pf-01-5      | 5     | PF-01     | De geo3 bsT                       |
      | pf-01-6      | 6     | PF-01     | De da1str bsP                     |
      | pf-01-7      | 7     | PF-01     | En da1str bsP                     |
      | pf-01-8      | 8     | PF-01     | En da1geo bsP                     |
      | pf-01-9      | 9     | PF-01     | En da2 bsP                        |
      | pf-01-10     | 10    | PF-01     | En da3 bsP                        |
      | pf-01-11     | 11    | PF-01     | En geo3 bsP                       |
      | pf-01-12     | 12    | PF-01     | En da2 bsA                        |
      | rw-01-13     | 13    | RW-01     | EX97-11                           |
      | rw-01-14     | 14    | RW-01     | inclined wall and slope           |
      | rw-02-15     | 15    | RW-02     | cohesive Rankine                  |
      | rw-02-16     | 16    | RW-02     | sloped Rankine                    |
      | rw-03-17     | 17    | RW-03     | Rankine passive                   |
      | rw-03-18     | 18    | RW-03     | curved delta phi/2                |
      | rw-03-19     | 19    | RW-03     | curved delta phi                  |
      | rw-04-20     | 20    | RW-04     | K0                                |
      | rw-05-21     | 21    | RW-05     | water and strip                   |
      | rw-06-22     | 22    | RW-06     | gravity wall                      |
      | rw-07-23     | 23    | RW-07     | propped sheet pile                |
      | rw-07-24     | 24    | RW-07     | cantilever sheet pile             |
      | rw-08-25     | 25    | RW-08     | Mononobe-Okabe                    |
      | rw-09-26     | 26    | RW-09     | reinforced soil                   |
      | rw-10-27     | 27    | RW-10     | traffic                           |
      | geo-f01-28   | 28    | GEO-F01   | EX97-01 strip (per metre scaled)  |
      | geo-f01-29   | 29    | GEO-F01   | EX97-03                           |
      | geo-f01-30   | 30    | GEO-F01   | EX97-04                           |
      | geo-f01-31   | 31    | GEO-F01   | two-way H, tilted base, submerged |
      | geo-f01-32   | 32    | GEO-F01   | DA1-C2 M2                         |
      | geo-f02-33   | 33    | GEO-F02   | EX97-02                           |
      | geo-f02-34   | 34    | GEO-F02   | undrained inclined                |
      | geo-f03-35   | 35    | GEO-F03   | DIN 4017                          |
      | geo-f04-36   | 36    | GEO-F04   | EX97-05                           |
      | geo-f04-37   | 37    | GEO-F04   | undrained with passive            |
      | equ-01-38    | 38    | EQU-01    | EX97-06 DE                        |
      | equ-01-39    | 39    | EQU-01    | EX97-06 EN                        |
      | upl-01-40    | 40    | UPL-01    | EX97-07                           |
      | upl-01-41    | 41    | UPL-01    | UPL head                          |
      | hyd-01-42    | 42    | HYD-01    | EX97-08 DE                        |
      | hyd-01-43    | 43    | HYD-01    | EX97-08 EN                        |
      | sls-01-44    | 44    | SLS-01    | EX97-09                           |
      | sls-01-45    | 45    | SLS-01    | rect Newmark                      |
      | sls-01-46    | 46    | SLS-01    | strip                             |
      | sls-01-47    | 47    | SLS-01    | 2:1                               |
      | sls-01b-48   | 48    | SLS-01B   | circle flexible                   |
      | sls-01b-49   | 49    | SLS-01B   | circle rigid                      |
      | sls-01b-50   | 50    | SLS-01B   | influence factor                  |
      | sls-02-51    | 51    | SLS-02    | EX97-10                           |
      | sls-02-52    | 52    | SLS-02    | OC and time                       |
      | sls-03-53    | 53    | SLS-03    | creep                             |
      | sls-04-54    | 54    | SLS-04    | tilt                              |
      | pile-01-55   | 55    | PILE-01   | EX97-12 DE                        |
      | pile-01-56   | 56    | PILE-01   | EX97-12 EN stiff                  |
      | pile-01-57   | 57    | PILE-01   | n=5                               |
      | pile-02-58   | 58    | PILE-02   | ground tests n=3                  |
      | pile-02-59   | 59    | PILE-02   | n=6 interpolated                  |
      | pile-03-60   | 60    | PILE-03   | tension direct                    |
      | pile-03-61   | 61    | PILE-03   | tension tests                     |
      | pile-04-62   | 62    | PILE-04   | Winkler and Broms                 |
      | pile-05-63   | 63    | PILE-05   | downdrag                          |
      | pile-06-64   | 64    | PILE-06   | group                             |
      | pile-07-65   | 65    | PILE-07   | buckling                          |
      | pile-08-66   | 66    | PILE-08   | dynamic                           |
      | pile-09-67   | 67    | PILE-09   | resistance-settlement             |
      | anc-01-68    | 68    | ANC-01    | anchor tests                      |
      | anc-02-69    | 69    | ANC-02    | anchor empirical                  |
      | anc-03-70    | 70    | ANC-03    | Kranz                             |
      | anc-04-71    | 71    | ANC-04    | tendon                            |
      | dew-01-72    | 72    | DEW-01    | EX97-15                           |
      | dew-01-73    | 73    | DEW-01    | confined Kusakin                  |
      | dew-02-74    | 74    | DEW-02    | well group                        |
      | dew-03-75    | 75    | DEW-03    | drawdown settlement               |
      | slope-01-76  | 76    | SLOPE-01  | EX97-13 circle                    |
      | slope-01-77  | 77    | SLOPE-01  | Bishop with water                 |
      | slope-01b-78 | 78    | SLOPE-01B | Fellenius                         |
      | slope-02-79  | 79    | SLOPE-02  | Janbu                             |
      | slope-03-80  | 80    | SLOPE-03  | EX97-13 dry                       |
      | slope-03-81  | 81    | SLOPE-03  | seepage                           |
      | slope-03-82  | 82    | SLOPE-03  | c-phi                             |
      | slope-04-83  | 83    | SLOPE-04  | wedge                             |
      | g-01-84      | 84    | G-01      | stress profile                    |
      | g-02-85      | 85    | G-02      | EX97-14 mean                      |
      | g-02-86      | 86    | G-02      | local n=12                        |
      | g-02-87      | 87    | G-02      | known V                           |
      | gi-01-88     | 88    | GI-01     | investigation footing             |
      | gi-01-89     | 89    | GI-01     | investigation pile                |
      | gi-02-90     | 90    | GI-02     | CPT classification                |
      | gi-03-91     | 91    | GI-03     | CPT cu                            |
      | gi-04-92     | 92    | GI-04     | CPT phi                           |
      | gi-05-93     | 93    | GI-05     | CPT Eoed                          |
      | gi-06-94     | 94    | GI-06     | SPT correction                    |
      | gi-07-95     | 95    | GI-07     | SPT phi                           |
      | gi-08-96     | 96    | GI-08     | SPT Dr                            |
      | gi-09-97     | 97    | GI-09     | DPH                               |
      | gi-10-98     | 98    | GI-10     | oedometer                         |
      | gi-11-99     | 99    | GI-11     | triaxial                          |
      | gi-12-100    | 100   | GI-12     | direct shear                      |
      | gi-13-101    | 101   | GI-13     | characteristic                    |
      | gi-14-102    | 102   | GI-14     | Menard                            |
      | gi-15-103    | 103   | GI-15     | grain size                        |
      | gi-16-104    | 104   | GI-16     | Thiem                             |
      | geo-f01-105  | 105   | GEO-F01   | Hansen depth factors              |
      | hyd-01-106   | 106   | HYD-01    | filter criteria                   |
      | gi-17-107    | 107   | GI-17     | direct shear lower bound         |
      | gi-17-108    | 108   | GI-17     | triaxial lower bound             |
      | pile-10-109  | 109   | PILE-10   | criterion reached, three tests   |
      | pile-10-110  | 110   | PILE-10   | criterion not reached EN         |
      | pile-11-111  | 111   | PILE-11   | two profiles                     |
      | hyd-02-112   | 112   | HYD-02    | Lane creep rule                  |
      | hyd-02-113   | 113   | HYD-02    | sheet pile exit gradient         |
      | hyd-02-114   | 114   | HYD-02    | flow-net exit gradient EN        |
      | slope-05-115 | 115   | SLOPE-05  | cohesive end planes              |
      | slope-05-116 | 116   | SLOPE-05  | wide body, high cohesion         |
      | hyd-03-117   | 117   | HYD-03    | finite-difference flow net 9 m   |
      | hyd-03-118   | 118   | HYD-03    | flow net, deep layer EN          |
      | rw-11-119    | 119   | RW-11     | heel wall with surcharge         |
      | rw-11-120    | 120   | RW-11     | heel wall, wall friction EN      |
      | rw-12-121    | 121   | RW-12     | free earth support in water      |
      | rw-12-122    | 122   | RW-12     | Blum fixed earth support, water  |
      | rw-12-123    | 123   | RW-12     | cantilever sheet pile, dry EN    |
      | emb-01-124   | 124   | EMB-01    | embankment on soft clay          |
      | emb-01-125   | 125   | EMB-01    | high embankment, very soft EN    |
      | rw-03-126    | 126   | RW-03     | M2 Rankine passive DA3 with c'     |
      | rw-03-127    | 127   | RW-03     | M2 passive DA1-C2 with surcharge   |
      | hyd-03-128   | 128   | HYD-03    | heave and internal erosion, shallow |
      | gi-03-129    | 129   | GI-03     | CPT cu, N_kt from registry DE    |
      | gi-03-130    | 130   | GI-03     | CPT cu, N_kt from registry EN    |
      | sls-05-131   | 131   | SLS-05    | laboratory c_v, U(t) and residual |
      | sls-05-132   | 132   | SLS-05    | direct c_v, t50 and t90           |
      | pile-12-133  | 133   | PILE-12   | 3x4 group on two layers           |
      | pile-12-134  | 134   | PILE-12   | 2x5 group, three layers           |
      | slope-06-135 | 135   | SLOPE-06  | Spencer with water and surcharge  |
      | slope-06-136 | 136   | SLOPE-06  | Spencer dry cohesionless          |
      | slope-06-137 | 137   | SLOPE-06  | Spencer with pore-pressure ratio  |

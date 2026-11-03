@capability-en1995-1-evaluate
@oracle-en1995-1-compliance-independent
@comparison-method-spread-v1
Feature: Evaluate the EN 1995 verification catalogue against an independent Python formulation
  Every catalogue entry T-nn of the EN 1995 target specification is evaluated by the Rust kernel on a committed
  input case and by `🔮️oracles/⚖️compliance/🐍️.py --catalogue`, which re-derives the same quantity with a
  different formulation (exact elastic critical moment instead of the 0.78 coefficient, shear analogy instead of
  the gamma method for CLT, Newmark series instead of the three-part gamma sum, Rayleigh quotient for the
  floor frequency, an angle-free table for spacings). Both read the same committed cases at
  `shared://📇️catalogue/🔣️.json`; values must agree within one percent.

  Further specification assertions, asserted by the Rust unit tests of the artifact and not by the platform adapters:
  - The pinned worked examples of the specification are reproduced: Given the specification examples E2 (column C24 100 x 100, L = 3.0 m, SC2) and E6 (M12 bolt in C24 60/60) When the kernel evaluates T-20 and T-71 Then the buckling resistance is 36.77 kN and the characteristic single shear load is 7.532 kN with design value 4.635 kN
  - A catalogue case stored in the document is evaluated and follows its inputs: Given the committed document shared://🧫️fixtures/🧬️mutations/🔢️change-case-number/✅apply/🔣️.before.json with the case case-kser of entry T-83 (d = 12 mm, rho_m1 = 420 kg/m3) When both implementations evaluate the document cases, before and after the committed change of d to 16 mm Then K_ser is rho_m1^1.5 * d / 23 in N/mm for each document and the ratio of the two K_ser values is 16 / 12
  - Equations 6.10a and 6.10b are selectable and lower the leading combination: Given the committed documents shared://🧫️fixtures/🧬️mutations/🧮️change-uls-equation/✅apply/🔣️.before.json and shared://🧫️fixtures/🧬️mutations/🧮️change-uls-equation/✅apply/🔣️.after.json When both implementations evaluate the bending check of the first member Then the utilisation with 6.10ab is smaller than with 6.10 because xi = 0.85 reduces the permanent part of the leading combination and both implementations agree within one percent
  - Catalogue entry <entry> is drawn from its inputs with its domain geometry: Given a document case of entry <entry> with the committed inputs of the catalogue fixture When the scene of the document is projected Then the scene contains the entity <entity> and a diagram <diagram> with a table twin
  - The response curves of the diagrams agree with the numpy formulation of the Python oracle (`--curves`): Given the cases T-20 (C24 100 x 100, beta_c 0.2), T-22 (C24 120 x 400), T-61 (C24 100 x 200, L = 4 m, w_G for 1.5 N/mm, w_Q for 2.0 N/mm), T-70 (M12 bolt, rho_k 350) and T-103 (20 mm type F board, 30 mm type A board) When the kernel probes (column, lateral buckling, deflection, embedment, protection) and the oracle are evaluated at the same abscissae (lambda_rel 0 to 3.3, l_ef 1.5 to 27 m, k_def 0 to 2.5, alpha 0 to 90 degrees, t 0 to 120 min) Then every value agrees to 1e-9 and the hand values hold: k_c = 0.28457 at lambda_rel 1.7622, k_crit = 0.79533 at l_ef = 9 m, f_h,90,k = f_h,0,k / 1.53 = 16.507 N/mm2, no char before t_ch = 42 min and 23.4 mm after 60 min behind a type F board.
  - The diagrams case-buckling, case-ltb, case-deflection, case-embedment and case-cladding are pure projections of their case: Given document cases of T-20, T-22, T-61, T-70 and T-103 with the inputs above When the diagrams of the document are projected Then each has a marker bound to the leaf it drags (lef_y and lef_z, lef, k_def, alpha_deg, t_min), a table twin of equal column lengths, and the nail case of T-70 has no embedment diagram because its embedment strength does not depend on the angle.
  - A timber-concrete composite beam (T-90 with a concrete slab as part 1: A 48000 mm2, E 33000 N/mm2, 150 mm connector spacing, K 15000 N/mm, l 6 m, M 20 kNm) gets gamma_1 0.1872, (EI)_ef 6.439e12 against 9.447e12 rigid, slab fibres -5.72 and 2.48 N/mm2, timber fibres -1.59 and 6.99 N/mm2 and a slip of 0.389 mm at the support: Then the Newmark formulation of the Python oracle (`--joint`, numpy) gives the same values to 1e-9, K_u = 2/3 K_ser lowers (EI)_ef to 5.732e12, and the diagrams case-joint-stress and case-joint-slip are projections of the case.
  - The shared action factors give the imposed categories F (0.7, 0.7, 0.6), G (0.7, 0.5, 0.3) and H (0, 0, 0) in both annexes, and in the DE annex a floor with 6 Hz <= f1 < 8 Hz asks for the unit-load criterion T-64 while the EN annex does not.
  - The entity curves k_v of 6.62 over the notch depth (k_n 6.5 for glulam, h 400 mm, x 100 mm, i 0.5), F_t,90,d / V_d = 1.3 (3 (1 - alpha)^2 - 2 (1 - alpha)^3) of the reinforced notch (0.0364 at alpha 0.9, 0.203125 at alpha 0.75), the apex factors k_l (6.43 to 6.47), k_p (6.48 to 6.51) and k_r = 0.76 + 0.001 r_in / t below r_in / t = 240 over the inner radius (h_ap 900 mm, 10 degrees, t 33 mm) are reproduced by the Python oracle with numpy polynomials to 1e-9: Given the curve cases of the oracle When the Rust probes and the oracle are evaluated at the same abscissae Then 19 values agree and the limits d <= 0.15 h of the NCI hole rule and h_ef >= 0.5 h of the NA-DE notch rule are the constants of the checks and of the diagrams
  - A single tapered beam is checked at the section its moment line governs (EN 1995-1-1 6.4.2): Given a glulam GL28h beam b 160 mm, h 800 mm at the shallow end, tan alpha 0.05, span l 8 m under a uniform load with the peak design moment M 100 kNm and the tension edge tapered When the kernel evaluates T-30 with load case udl_ss Then the governing section lies at x = l / (2 + l tan alpha / h) = 3.2 m and not at midspan, h(x) is 960 mm, M(x) is 0.96 M, sigma = 6 M / (b h^2 (1 + l tan alpha / h)) = 3.90625 N/mm2 against 3.75 N/mm2 at midspan, the utilisation is 0.2489 And the same case without span and load case is refused (not applicable, reason "the moment line of the member is unknown" / "Die Momentenlinie des Bauteils ist unbekannt") because there is no constant-moment fallback And the numpy oracle (`--curves`, T-30 and T-30-governing, grid argmax) reproduces the governing section and the utilisation for the uniform load, the point load at midspan, the cantilever and their mixture
  - A continuous two-span tapered beam (GL28h, b 160 mm, h 800 mm, tan alpha 0.05, two spans of 6 m, peak moment M 100 kNm at the interior support, tension edge tapered at the peak) is swept over both spans with both edge states (EN 1995-1-1 6.38 and 6.39): Given the load cases two_span_udl and two_span_point When the kernel evaluates T-30 Then under the uniform load the support governs (x = 6 m, h 1100 mm, sigma = 6 M / (b h^2) = 3.0992 N/mm2, tension edge k_m,alpha 0.87571, utilisation 0.1975) while the field at x = 2.25 m has M = 9/16 M, h 912.5 mm, a compression edge k_m,alpha 0.96588 and 0.1464, and under midspan loads the field below the load governs (x = 3 m, M = 5/6 M, h 950 mm, compression edge, sigma 3.4626 N/mm2, utilisation 0.2001) And the numpy oracle (`--curves`, three-moment statics instead of the closed forms) reproduces the governing section and nine abscissae of the curve for the uniform load, the midspan loads and their mixture to 1e-9
  - A member whose action is given as forces without loads is resolved to a line load and a midspan point load by statics: Given the peak moment and shear force of 10 kN/m with 8 kN on 4 m (28 kNm, 24 kN), of 5 kN/m with 6 kN on a 3 m cantilever (40.5 kNm, 21 kN) and of 4 kN/m with 10 kN on two spans of 4.5 m (18.5625 kNm, 18.125 kN) When the effect profile solves the two equilibrium relations Then the loads are recovered and numpy.linalg.solve over numerically derived unit-load peaks gives the same loads and point share, and forces that need a negative line or point load (a moment without shear force, V L above 4 M) are refused with the localized moment-line reason instead of a constant moment, as are the apex checks T-31 to T-33 without a moment line, whose apex moment is the peak moment times the moment line at midlength (the cantilever apex carries q l^2 / 8)

  @id-catalogue-fixture-case
  @level-quick
  @mode-differential
  Scenario Outline: Evaluate <entry> case <index> and compare every named result
    Given the committed catalogue case shared://📇️catalogue/🔣️.json at index <index>
    When both implementations evaluate catalogue entry <entry> on that case
    Then every result named by the oracle agrees within the comparison profile
    Examples:
      | id       | index | entry |
      | t-13-0   | 0     | T-13  |
      | t-14-1   | 1     | T-14  |
      | t-20-2   | 2     | T-20  |
      | t-20-3   | 3     | T-20  |
      | t-22-4   | 4     | T-22  |
      | t-30-5   | 5     | T-30  |
      | t-31-6   | 6     | T-31  |
      | t-45-7   | 7     | T-45  |
      | t-63-8   | 8     | T-63  |
      | t-70-9   | 9     | T-70  |
      | t-71-10  | 10    | T-71  |
      | t-74-11  | 11    | T-74  |
      | t-83-12  | 12    | T-83  |
      | t-90-13  | 13    | T-90  |
      | t-97-14  | 14    | T-97  |
      | t-121-15 | 15    | T-121 |
      | t-120-16 | 16    | T-120 |
      | t-101-17 | 17    | T-101 |
      | t-106-18 | 18    | T-106 |
      | t-107-19 | 19    | T-107 |
      | t-90-20  | 20    | T-90  |
      | t-93-21  | 21    | T-93  |
      | t-104-22 | 22    | T-104 |
      | t-120-23 | 23    | T-120 |
      | t-121-24 | 24    | T-121 |
      | t-122-25 | 25    | T-122 |
      | t-03-26 | 26 | T-03 |
      | t-11-27 | 27 | T-11 |
      | t-01-28 | 28 | T-01 |
      | t-02-29 | 29 | T-02 |
      | t-04-30 | 30 | T-04 |
      | t-05-31 | 31 | T-05 |
      | t-06-32 | 32 | T-06 |
      | t-10-33 | 33 | T-10 |
      | t-12-34 | 34 | T-12 |
      | t-15-35 | 35 | T-15 |
      | t-16-36 | 36 | T-16 |
      | t-17-37 | 37 | T-17 |
      | t-18-38 | 38 | T-18 |
      | t-50-39 | 39 | T-50 |
      | t-66-40 | 40 | T-66 |
      | t-123-41 | 41 | T-123 |
      | t-124-42 | 42 | T-124 |
      | t-32-43 | 43 | T-32 |
      | t-33-44 | 44 | T-33 |
      | t-46-45 | 45 | T-46 |
      | t-72-46 | 46 | T-72 |
      | t-73-47 | 47 | T-73 |
      | t-75-48 | 48 | T-75 |
      | t-76-49 | 49 | T-76 |
      | t-77-50 | 50 | T-77 |
      | t-78-51 | 51 | T-78 |
      | t-79-52 | 52 | T-79 |
      | t-80-53 | 53 | T-80 |
      | t-81-54 | 54 | T-81 |
      | t-82-55 | 55 | T-82 |
      | t-60-56 | 56 | T-60 |
      | t-61-57 | 57 | T-61 |
      | t-62-58 | 58 | T-62 |
      | t-64-59 | 59 | T-64 |
      | t-65-60 | 60 | T-65 |
      | t-21-61 | 61 | T-21 |
      | t-23-62 | 62 | T-23 |
      | t-24-63 | 63 | T-24 |
      | t-100-64 | 64 | T-100 |
      | t-102-65 | 65 | T-102 |
      | t-103-66 | 66 | T-103 |
      | t-105-67 | 67 | T-105 |
      | t-91-68 | 68 | T-91 |
      | t-92-69 | 69 | T-92 |
      | t-94-70 | 70 | T-94 |
      | t-95-71 | 71 | T-95 |
      | t-96-72 | 72 | T-96 |
      | t-98-73 | 73 | T-98 |
      | t-95-74 | 74 | T-95 |
      | t-95-75 | 75 | T-95 |
      | t-96-76 | 76 | T-96 |

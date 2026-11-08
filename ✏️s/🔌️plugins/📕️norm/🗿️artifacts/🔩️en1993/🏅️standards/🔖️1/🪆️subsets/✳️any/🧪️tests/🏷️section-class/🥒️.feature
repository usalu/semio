@capability-en1993-1-evaluate
@oracle-en1993-1-compliance-independent
@comparison-numerical-v1
Feature: EN 1993 section classification, shear resistance and the Annex A interaction reproduce the independent oracle
  The Rust clause functions of `🧮️clauses` and the stdlib closed forms of `🧮️reference/🐍️.py` (section area from the third-party `sectionproperties` package) must agree for every quantity, in any implementation language.
  The subject is an IPE 300 in S355 (V-030 internal parts, V-031 outstand flanges, V-033 highest class of the compressed parts), welded plate outstands (V-031), circular and angle sections (V-032), the classification V-028 as the delegating clause, the plastic shear resistance of 6.2.6 (V-063) and the Annex A factors k_yy, k_yz, k_zy, k_zz with the equations 6.61 and 6.62 of 6.3.3 (V-120).

  @id-clause-vector
  @level-quick
  @mode-differential
  Scenario Outline: <vector> <quantity> equals the oracle value
    Given the clause vector "<vector>" of "<part>"
    When the quantity "<quantity>" is evaluated
    Then it equals the independent oracle value within relative tolerance <tolerance>
    Examples:
      | id                        | vector | part        | quantity | tolerance |
      | epsilon-355               | V-030 | EN 1993-1-1 | epsilon_355 | 1e-12 |
      | class-web-compression     | V-030 | EN 1993-1-1 | class_web_compression | 1e-12 |
      | class-web-bending         | V-030 | EN 1993-1-1 | class_web_bending | 1e-12 |
      | class-flange              | V-031 | EN 1993-1-1 | class_flange | 1e-12 |
      | class-plate-outstand-10   | V-031 | EN 1993-1-1 | class_plate_outstand_10 | 1e-12 |
      | class-plate-outstand-12   | V-031 | EN 1993-1-1 | class_plate_outstand_12 | 1e-12 |
      | class-section-compression | V-033 | EN 1993-1-1 | class_section_compression | 1e-12 |
      | class-chs-139-5           | V-032 | EN 1993-1-1 | class_chs_139_5 | 1e-12 |
      | class-chs-273-63          | V-032 | EN 1993-1-1 | class_chs_273_63 | 1e-12 |
      | class-chs-508-5           | V-032 | EN 1993-1-1 | class_chs_508_5 | 1e-12 |
      | class-angle-s235          | V-032 | EN 1993-1-1 | class_angle_s235 | 1e-12 |
      | class-angle-s355          | V-032 | EN 1993-1-1 | class_angle_s355 | 1e-12 |
      | a-v                       | V-063 | EN 1993-1-1 | a_v | 5e-4 |
      | v-pl-rd                   | V-063 | EN 1993-1-1 | v_pl_rd | 5e-4 |
      | annex-a-kyy               | V-120 | EN 1993-1-1 | kyy | 1e-9 |
      | annex-a-kyz               | V-120 | EN 1993-1-1 | kyz | 1e-9 |
      | annex-a-kzy               | V-120 | EN 1993-1-1 | kzy | 1e-9 |
      | annex-a-kzz               | V-120 | EN 1993-1-1 | kzz | 1e-9 |
      | annex-a-eta-61            | V-120 | EN 1993-1-1 | eta_61 | 1e-9 |
      | annex-a-eta-62            | V-120 | EN 1993-1-1 | eta_62 | 1e-9 |

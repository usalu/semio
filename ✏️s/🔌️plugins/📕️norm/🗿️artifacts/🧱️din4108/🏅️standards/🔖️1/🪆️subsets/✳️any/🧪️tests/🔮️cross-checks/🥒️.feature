@capability-din4108-1-evaluate
@oracle-din4108-1-cross-check
@comparison-numerical-v1
Feature: DIN 4108 U-value, temperature factor and Glaser checks reproduce the third-party cross-checks
  The independent oracle `🔮️oracles/🌐️cross-checks/🐍️.py` (exact fractions per EN ISO 6946 6.7.2, scipy brentq, IAPWS-IF97 and R14-08 vapour pressure, ISO 13788 condensation-plane iteration) writes the vectors in `🧫️fixtures/🔮️cross-checks/🔣️.json`; every implementation must reproduce the rows tagged @agree.
  Every row is tagged @agree: the Rust source reproduces the standard (R_si 0.25 m2K/W for f_Rsi, EN ISO 6946 6.7.2 for every inhomogeneous layer, DIN 4108-3:2018 Annex A climate and the flux g = (p_i - p_c)/Z in seconds).

  @id-u-combined
  @level-quick
  @mode-differential
  @agree
  Scenario Outline: the combined-method resistance equals the exact fraction result
    Given the vector "<vector>" of the section "uValues"
    When the total resistance of the layer stack is evaluated
    Then it equals rTotal within the relative tolerance <tolerance>
    Examples:
      | vector | tolerance |
      | 0 | 1e-9 |
      | 1 | 1e-9 |
      | 2 | 1e-9 |
      | 3 | 1e-9 |
      | 4 | 1e-9 |
      | 5 | 1e-9 |
      | 6 | 1e-9 |
      | 7 | 1e-9 |
  @id-u-ratio
  @level-quick
  @mode-differential
  @agree
  Scenario Outline: the combined method warns when the upper and lower limit differ by more than 1.5
    Given the vector "<vector>" of the section "uValues"
    When the validity of the combined method is evaluated
    Then a localized English and German warning is emitted exactly when ratioUpperLower exceeds 1.5
    Examples:
      | vector |
      | 0 |
      | 4 |
      | 6 |
  @id-frsi-standard
  @level-quick
  @mode-differential
  @agree
  Scenario Outline: the temperature factor uses R_si = 0.25 m2K/W at 20 and -5 degrees Celsius and the verdict requires 0.70
    Given the vector "<vector>" of the section "fRsi"
    When the temperature factor is evaluated
    Then it equals fRsi within the absolute tolerance <tolerance>
    And the verdict equals passes
    Examples:
      | vector | tolerance |
      | 0 | 1e-12 |
      | 1 | 1e-12 |
      | 2 | 1e-12 |
      | 3 | 1e-12 |
      | 4 | 1e-12 |
  @id-glaser-mass
  @level-quick
  @mode-differential
  @agree
  Scenario Outline: the Glaser condensed and evaporated mass follow DIN 4108-3 Annex A
    Given the vector "<vector>" of the section "glaser"
    When the winter condensed mass and the summer evaporated mass are evaluated
    Then they equal reference within the relative tolerance <tolerance>
    Examples:
      | vector | tolerance |
      | 0 | 1e-2 |
      | 1 | 1e-2 |
      | 2 | 1e-2 |
      | 3 | 1e-2 |
      | 4 | 1e-2 |

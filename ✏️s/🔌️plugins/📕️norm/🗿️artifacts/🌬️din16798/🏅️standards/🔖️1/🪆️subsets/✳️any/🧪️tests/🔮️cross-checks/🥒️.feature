@capability-din16798-1-evaluate
@oracle-din16798-1-cross-check
@comparison-numerical-v1
Feature: DIN EN 16798-1 comfort and ventilation reproduce the third-party cross-checks
  The independent oracle `🔮️oracles/🌐️cross-checks/🐍️.py` (scipy brentq for PMV and its inversion, scipy solve_ivp for the CO2 balance, IAPWS-IF97 vapour pressure) writes the vectors in `🧫️fixtures/🔮️cross-checks/🔣️.json`; every implementation must reproduce the rows tagged @agree.
  Rows tagged @known-divergence document a difference between the Rust source and the standard (fixture section `divergences`, currently only the informational Table B.2 and adaptive notes) and are not asserted.

  @id-pmv-brent
  @level-quick
  @mode-differential
  @agree
  Scenario Outline: the predicted mean vote equals the clothing heat balance root
    Given the vector "<vector>" of the section "pmv"
    When the predicted mean vote is evaluated
    Then it equals pmvIso7730Vapour within the absolute tolerance <tolerance>
    Examples:
      | vector | tolerance |
      | 0 | 5e-3 |
      | 1 | 5e-3 |
      | 2 | 5e-3 |
      | 3 | 5e-3 |
      | 4 | 5e-3 |
      | 5 | 5e-3 |
      | 6 | 5e-3 |
      | 7 | 5e-3 |
  @id-invert
  @level-quick
  @mode-differential
  @agree
  Scenario Outline: the operative temperature at the PMV limit equals the root
    Given the vector "<vector>" of the section "inversion"
    When the temperature at the category PMV limit is evaluated
    Then it equals referenceC within the absolute tolerance <tolerance>
    Examples:
      | vector | tolerance |
      | 0 | 5e-3 |
      | 1 | 5e-3 |
      | 2 | 5e-3 |
      | 3 | 5e-3 |
      | 4 | 5e-3 |
      | 5 | 5e-3 |
      | 6 | 5e-3 |
      | 7 | 5e-3 |
      | 8 | 5e-3 |
      | 9 | 5e-3 |
      | 10 | 5e-3 |
      | 11 | 5e-3 |
      | 12 | 5e-3 |
      | 13 | 5e-3 |
      | 14 | 5e-3 |
      | 15 | 5e-3 |
      | 16 | 5e-3 |
      | 17 | 5e-3 |
      | 18 | 5e-3 |
      | 19 | 5e-3 |
      | 20 | 5e-3 |
      | 21 | 5e-3 |
      | 22 | 5e-3 |
      | 23 | 5e-3 |
      | 24 | 5e-3 |
      | 25 | 5e-3 |
      | 26 | 5e-3 |
      | 27 | 5e-3 |
      | 28 | 5e-3 |
      | 29 | 5e-3 |
      | 30 | 5e-3 |
      | 31 | 5e-3 |
      | 32 | 5e-3 |
      | 33 | 5e-3 |
      | 34 | 5e-3 |
      | 35 | 5e-3 |
      | 36 | 5e-3 |
      | 37 | 5e-3 |
      | 38 | 5e-3 |
      | 39 | 5e-3 |
  @id-co2
  @level-quick
  @mode-differential
  @agree
  Scenario Outline: the steady-state CO2 concentration equals the integrated mass balance
    Given the vector "<vector>" of the section "co2"
    When the required outdoor air of method 2 is evaluated
    Then it equals flowM3H within the relative tolerance <tolerance>
    Examples:
      | vector | tolerance |
      | 0 | 1e-6 |
      | 1 | 1e-6 |
      | 2 | 1e-6 |

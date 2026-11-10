@capability-din18599-1-evaluate
@oracle-din18599-1-cross-check
@comparison-numerical-v1
Feature: DIN V 18599-2 monthly balance reproduces the third-party cross-checks
  The independent oracle `🔮️oracles/🌐️cross-checks/🐍️.py` (mpmath at 50 digits for the utilisation factor, EN ISO 13790 monthly method with a = a0 + tau / tau0 and calendar month hours) writes the vectors in `🧫️fixtures/🔮️cross-checks/🔣️.json`; every implementation must reproduce the rows tagged @agree.
  Rows tagged @known-divergence document a difference between the Rust source and the standard (fixture section `divergences`, currently empty) and are not asserted.

  @id-eta-heating
  @level-quick
  @mode-differential
  @agree
  Scenario Outline: the gain utilisation factor of the heating balance
    Given the vector "<vector>" of the section "utilisationFactor"
    When the utilisation factor of the gain-to-loss ratio gamma and the parameter a is evaluated
    Then it equals etaHeating within the relative tolerance <tolerance>
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
      | 8 | 1e-9 |
      | 9 | 1e-9 |
      | 10 | 1e-9 |
      | 11 | 1e-9 |
      | 12 | 1e-9 |
      | 13 | 1e-9 |
      | 14 | 1e-9 |
      | 15 | 1e-9 |
      | 16 | 1e-9 |
      | 17 | 1e-9 |
      | 18 | 1e-9 |
      | 19 | 1e-9 |
      | 20 | 1e-9 |
      | 21 | 1e-9 |
      | 22 | 1e-9 |
      | 23 | 1e-9 |
      | 24 | 1e-9 |
      | 25 | 1e-9 |
      | 26 | 1e-9 |
      | 27 | 1e-9 |
      | 28 | 1e-9 |
      | 29 | 1e-9 |
      | 30 | 1e-9 |
      | 31 | 1e-9 |
      | 32 | 1e-9 |
      | 33 | 1e-9 |
      | 34 | 1e-9 |
  @id-eta-cooling
  @level-quick
  @mode-differential
  @agree
  Scenario Outline: the loss utilisation factor of the cooling balance
    Given the vector "<vector>" of the section "utilisationFactor"
    When the cooling utilisation factor of gamma and a is evaluated
    Then it equals etaCooling within the relative tolerance <tolerance>
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
      | 8 | 1e-9 |
      | 9 | 1e-9 |
      | 10 | 1e-9 |
      | 11 | 1e-9 |
      | 12 | 1e-9 |
      | 13 | 1e-9 |
      | 14 | 1e-9 |
      | 15 | 1e-9 |
      | 16 | 1e-9 |
      | 17 | 1e-9 |
      | 18 | 1e-9 |
      | 19 | 1e-9 |
      | 20 | 1e-9 |
      | 21 | 1e-9 |
      | 22 | 1e-9 |
      | 23 | 1e-9 |
      | 24 | 1e-9 |
      | 25 | 1e-9 |
      | 26 | 1e-9 |
      | 27 | 1e-9 |
      | 28 | 1e-9 |
      | 29 | 1e-9 |
      | 30 | 1e-9 |
      | 31 | 1e-9 |
      | 32 | 1e-9 |
      | 33 | 1e-9 |
      | 34 | 1e-9 |
  @id-month-standard
  @level-quick
  @mode-differential
  @agree
  Scenario Outline: the monthly heating demand follows the standard method
    Given the vector "<vector>" of the section "monthsStandard"
    When the monthly balance of the reference zone is evaluated
    Then Q_T, Q_V, Q_i, Q_s, eta, Q_H and Q_C equal the values within the relative tolerance <tolerance>
    Examples:
      | vector | tolerance |
      | 0 | 1e-6 |
      | 1 | 1e-6 |
      | 2 | 1e-6 |
      | 3 | 1e-6 |
      | 4 | 1e-6 |
      | 5 | 1e-6 |
      | 6 | 1e-6 |
      | 7 | 1e-6 |
      | 8 | 1e-6 |
      | 9 | 1e-6 |
      | 10 | 1e-6 |
      | 11 | 1e-6 |
  @id-zero-gain
  @level-quick
  @mode-differential
  @agree
  Scenario Outline: without gains the heating demand equals transmission plus ventilation loss
    Given the vector "<vector>" of the section "zeroGain"
    When the monthly balance of the reference zone without internal and solar gains is evaluated
    Then Q_H equals Q_T + Q_V within the relative tolerance <tolerance>
    Examples:
      | vector | tolerance |
      | 0 | 1e-12 |
  @id-month-heavy
  @level-quick
  @mode-differential
  @agree
  Scenario Outline: the monthly heating demand follows the heavy thermal mass class (130 Wh/(m2 K))
    Given the vector "<vector>" of the section "monthsHeavy"
    When the monthly balance of the reference zone with thermal mass class Heavy is evaluated
    Then Q_T, Q_V, Q_i, Q_s, eta, Q_H and Q_C equal the values within the relative tolerance <tolerance>
    Examples:
      | vector | tolerance |
      | 0 | 1e-6 |
      | 1 | 1e-6 |
      | 2 | 1e-6 |
      | 3 | 1e-6 |
      | 4 | 1e-6 |
      | 5 | 1e-6 |
      | 6 | 1e-6 |
      | 7 | 1e-6 |
      | 8 | 1e-6 |
      | 9 | 1e-6 |
      | 10 | 1e-6 |
      | 11 | 1e-6 |
  @id-month-custom
  @level-quick
  @mode-differential
  @agree
  Scenario Outline: the monthly heating demand follows the explicit C_wirk of the custom class (60 Wh/(m2 K))
    Given the vector "<vector>" of the section "monthsCustom"
    When the monthly balance of the reference zone with thermal mass class Custom and C_wirk 60 Wh/(m2 K) is evaluated
    Then Q_T, Q_V, Q_i, Q_s, eta, Q_H and Q_C equal the values within the relative tolerance <tolerance>
    Examples:
      | vector | tolerance |
      | 0 | 1e-6 |
      | 1 | 1e-6 |
      | 2 | 1e-6 |
      | 3 | 1e-6 |
      | 4 | 1e-6 |
      | 5 | 1e-6 |
      | 6 | 1e-6 |
      | 7 | 1e-6 |
      | 8 | 1e-6 |
      | 9 | 1e-6 |
      | 10 | 1e-6 |
      | 11 | 1e-6 |
  @id-heat-capacity
  @level-quick
  @mode-differential
  @agree
  Scenario Outline: the effective heat capacity of a zone follows its thermal mass class
    Given the vector "<vector>" of the section "heatCapacity"
    When the effective heat capacity of the zone is evaluated
    Then it equals capacityWhK within the relative tolerance <tolerance>
    Examples:
      | vector | tolerance |
      | 0 | 1e-12 |
      | 1 | 1e-12 |
      | 2 | 1e-12 |
      | 3 | 1e-12 |

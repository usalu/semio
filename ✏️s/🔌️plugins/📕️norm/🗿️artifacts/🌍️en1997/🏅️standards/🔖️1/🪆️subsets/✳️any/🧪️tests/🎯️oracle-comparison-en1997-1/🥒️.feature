@capability-en1997-1-third-party-cases
@oracle-en1997-1-scipy
@comparison-method-spread-v1
Feature: EN 1997 blessed third-party oracle cases against the production evaluators
  Every committed oracle case file of the artifact (`🧪️tests/🧫️fixtures/🔮️oracle-cases/*.json`, schema `🔮️oracles/🏃️harness/🧬️schema/🔣️.json`) holds SI inputs and quantities blessed by an independent third-party library. The Rust subject feeds each case to the production evaluator through the adapter registry of `oracle_wiring` and projects `<case>.<quantity>`; the Python oracle role projects the blessed values of the same cases. Quantities whose check has no evaluator entry point yet, and the documented known differences of the registry, are left out on both sides and stay reported by the `oracle_wiring` unit test. Per-quantity profiles and tolerances of the case file are asserted there; this feature compares under the widest profile.

  @id-oracle-comparison
  @level-quick
  @mode-differential
  Scenario Outline: Compare the quantities of <fixture>
    Given the committed oracle input shared://🔮️oracle-cases/<fixture>
    When the Python oracle and the kernel both evaluate it
    Then every quantity named by the oracle agrees within the comparison profile
    Examples:
      | id | fixture |
      | third-party-extended | third-party-extended.json |
      | third-party-geotech-v4 | third-party-geotech-v4.json |
      | third-party-sls-pairs | third-party-sls-pairs.json |
      | third-party-v2-extended | third-party-v2-extended.json |
      | third-party-w3 | third-party-w3.json |
      | third-party | third-party.json |

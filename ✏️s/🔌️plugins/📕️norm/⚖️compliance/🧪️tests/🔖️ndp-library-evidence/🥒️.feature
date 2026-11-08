@capability-norm-ndp-library-evidence @oracle-norm-ndp-library-evidence-python-independent
Feature: A verified NDP flag is backed by a third-party library and every conflict ledger row keeps its evidence basis
  The registry `📜️annex-and-tables/🔖️ndp/🔣️.json` is read as text by the independent Python oracle `🐍️.py`. The oracle reads the
  generated fixtures `🧫️fixtures/🔖️ndp-library-evidence/🔣️.json` and `🧫️fixtures/🔖️ndp-conflict-ledger/🔣️.json` and, when the
  libraries blue-prints and structuralcodes are installed, recomputes the library numbers itself without the ticket scripts.

  Scenario: Every verified flag cites a named library version
    Given the committed NDP registry
    When the evidence of every verified row and of every verified annex (annexStatus) is scanned for a library name with a version
    Then each of them names blue-prints, structuralcodes, groundhog, concreteproperties or sectionproperties with a version
    And a row that is verified for one annex only is not verified as a row

  Scenario: The library fixture equals the registry
    Given the fixture of library-backed rows
    When each row is looked up in the registry
    Then the EN recommended value equals the library value in the unit of the registry
    And the row or its EN annex is verified and the evidence names the library and its version

  Scenario: The library numbers are recomputed live
    Given the installed libraries blue-prints and structuralcodes
    When EN 338 C24 f_t,0,k, the shear k1 default, the Table 4.3N threshold of XC1, the Eq. 8.2 coefficient and the cot theta limits are computed by the oracle
    Then they equal the registry EN values of the ids named in the oracle

  Scenario: The conflict ledger keeps values and evidence basis
    Given the reviewed conflict ledger
    When each ledger row is compared with the registry
    Then the EN and NA-DE values are equal
    And a row with the basis library has a verified EN value
    And a row with the basis public or open has no verified flag at all

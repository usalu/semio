@capability-norm-ndp-registry-law @oracle-norm-ndp-registry-law-python-independent
Feature: A verified NDP row never carries a contradicting note and never cites a foreign code family
  The registry `📜️annex-and-tables/🔖️ndp/🔣️.json` is read as text by the independent Python oracle `🐍️.py`.

  Scenario: Verified rows carry no contradiction markers
    Given the committed NDP registry
    When the notes (specRef, asWritten, evidence) of every verified row are scanned for the markers "contradicts", "differs", "disagree", "should be", "TODO", "FIXME" and "?"
    Then no verified row contains a marker
    And a row whose asWritten value differs from its recommended value is not verified

  Scenario: Every row cites a source document of its own code family
    Given the committed NDP registry
    When the source document of each row is compared with the row code and the row id prefix
    Then the id prefix equals the code and the document names the same EN number or is a product standard of that family

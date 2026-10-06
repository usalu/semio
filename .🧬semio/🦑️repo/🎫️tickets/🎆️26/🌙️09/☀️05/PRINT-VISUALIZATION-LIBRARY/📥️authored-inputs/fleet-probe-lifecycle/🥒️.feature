Feature: Probe output follows actual document activation and pages
  Scenario Outline: Native probe lifecycle preserves authored pages and numeric records
    Given the neutral lifecycle document <id>
    When the pinned native compiler renders the document
    Then the independent PDF reader finds the declared page and identity counts
    And the independent JSON validator finds the declared probe records
    And inactive probe inclusion adds no output

    Examples:
      | id                          |
      | inactive-visible-article    |
      | inactive-blank-article      |
      | inactive-visible-semio      |
      | active-visible-article      |
      | active-numeric-article      |
      | early-end-numeric-article   |
      | early-end-visible-article   |
      | active-multipage-article    |
      | explicit-identity-article   |
      | active-visible-semio        |
      | active-numeric-semio        |
      | active-numeric-signature    |
      | early-end-numeric-signature |
      | inactive-visible-signature |

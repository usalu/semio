Feature: The runtime registry pins runtimes per platform
  Scenario: The runtime registry pins runtimes per platform
    Given the pinned runtime registry
    When a host selects its distribution
    Then exactly the pinned archive of that operating system, architecture and C library is selected
    And an unknown host selects nothing
    And a registry with a future contract, an unknown field, a plain-http source, a malformed digest or a missing text is refused

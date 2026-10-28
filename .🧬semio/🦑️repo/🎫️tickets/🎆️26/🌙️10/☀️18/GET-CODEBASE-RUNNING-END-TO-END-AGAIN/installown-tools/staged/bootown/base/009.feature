Feature: Installed native repository command
  Scenario: Every supported host exposes the real native executable
    Given an owned canonical native repository build
    When its user installation is published
    Then Unix shells select the native executable in the user bin directory
    And Windows selects an immutable native executable through persisted user PATH
    And all modes and MCP profiles retain the same native entrypoint

  Scenario: Installation preserves user configuration and original cancellation
    Given unrelated existing shell configuration
    When installation is repeated or fails before atomic publication
    Then the managed PATH block is unique and unrelated content remains
    And a failed publication cleans only its owned temporary file
    And the original signal and deadline govern both native help probes

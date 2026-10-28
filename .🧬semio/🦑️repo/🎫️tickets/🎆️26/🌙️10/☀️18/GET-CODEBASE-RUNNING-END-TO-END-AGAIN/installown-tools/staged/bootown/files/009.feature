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

  Scenario: The installation owns the PATH repo command and replaces only the semio bootstrap launcher link
    Given a user bin repo that is a symbolic link chain into the semio bootstrap launcher directory
    When its user installation is published
    Then the link is replaced atomically by the native executable and the bootstrap launcher is not written through
    And the Windows user PATH loses the bootstrap launcher directory and keeps the native directory first

  Scenario: Every other foreign destination is refused with an actionable message
    Given a user bin repo that is a symbolic link elsewhere, a dangling link, a directory or an executable no semio installation published
    When its user installation is published
    Then nothing is published
    And the refusal names the path and the command that repeats the installation, in English and German

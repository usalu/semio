Feature: The resident service answers plain task commands before any runtime starts
  Scenario: A served request streams its answer
    Given a resident service for this worktree
    When the launcher runs a plain run or test command
    Then the service receives one hello and one request with the language preference, the bound environment and a launcher capability
    And its output arrives byte for byte and its exit code is the exit code of the launcher
  Scenario: Anything else steps aside silently
    Given no service, an untrusted directory, no key, an ineligible command or the mode off
    When the launcher runs the command
    Then nothing is printed and the normal path runs
  Scenario: require and busy
    Given the mode require without a service or a busy service
    When the launcher runs the command
    Then it refuses in the chosen language, exit 1 for require and exit 75 for busy

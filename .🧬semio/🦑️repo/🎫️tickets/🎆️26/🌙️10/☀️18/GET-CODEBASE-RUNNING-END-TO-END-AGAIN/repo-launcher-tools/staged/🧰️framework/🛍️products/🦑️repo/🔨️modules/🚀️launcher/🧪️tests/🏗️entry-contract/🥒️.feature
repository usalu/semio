Feature: The implementation starts through the runtime unchanged
  Scenario: The implementation starts through the runtime unchanged
    Given a provisioned runtime
    When the launcher runs a build, workspace or script route
    Then the runtime receives the script, the entry prefix and the unchanged arguments
    And the runtime and launcher directories lead the PATH
    And a provisioned start prints nothing
    And a missing language refuses before any download

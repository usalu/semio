Feature: A pinned runtime is fetched once and verified
  Scenario: A pinned runtime is fetched once and verified
    Given a synthetic runtime archive on a loopback server
    When several launchers start at once on an empty cache
    Then one download happens and every launcher gets the same executable
    And a flipped, truncated or oversized archive installs nothing
    And a cancelled download leaves no staging directory
    And the tools partition is preferred over the user cache

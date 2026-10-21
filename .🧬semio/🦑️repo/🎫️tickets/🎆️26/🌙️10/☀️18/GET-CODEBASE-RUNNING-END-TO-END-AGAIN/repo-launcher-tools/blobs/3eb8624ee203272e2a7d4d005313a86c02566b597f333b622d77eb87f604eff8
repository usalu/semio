Feature: Versions of launcher, runtime and implementation
  Scenario: Versions of launcher, runtime and implementation
    Given a repository with and without a provisioned runtime
    When repo --version runs in English and in German
    Then launcher, runtime and implementation versions are named in that language
    And the runtime is never fetched for it

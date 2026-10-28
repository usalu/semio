Feature: The repository root is found the same way everywhere
  Scenario: The repository root is found the same way everywhere
    Given the shared workspace corpus
    When the launcher resolves each start directory
    Then it finds the nearest directory with paired markers and declared workspaces
    And a malformed package.json is refused instead of skipped
  Scenario: A Windows final path reads as the shell spelling
    Given a final path of the kernel with or without the Win32 file namespace prefix
    When the launcher names the worktree
    Then it strips the prefix exactly as the service's runtime does

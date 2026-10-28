Feature: The launcher behaves on native Windows as it does on POSIX
  Scenario: The runtime ends with the launcher
    Given a launcher that runs the runtime as a child
    When the launcher is terminated from outside
    Then the runtime ends too, as it would after exec on POSIX
  Scenario: The worktree has one name
    Given the same checkout spelled with another drive-letter and component case
    When the launcher and the runtime name the worktree
    Then both answer the kernel's final path, so they reach the same service
  Scenario: Only the own account serves the pipe
    Given a pipe of the service
    When the launcher dials it
    Then it keeps the pipe only when the serving process runs as the caller and waits briefly while every instance is busy
  Scenario: A scanner's brief hold is transient
    Given access denied or a sharing violation on a rename
    Then provisioning repeats the rename and treats every other error as final

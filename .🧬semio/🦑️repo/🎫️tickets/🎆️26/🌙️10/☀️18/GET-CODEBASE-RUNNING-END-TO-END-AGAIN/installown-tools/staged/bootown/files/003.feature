Feature: A bare machine gets the launcher without a pre-installed Go
  Scenario: The installer finds or fetches the pinned Go toolchain itself
    Given a machine with neither Go nor the repository tools partition
    When the launcher installer runs against a pin table and a loopback mirror
    Then it downloads the pinned archive once, checks its SHA-256 before unpacking and installs the toolchain by one rename
    And a second run reuses the installed toolchain without downloading
    And progress and every message are given in English and German

  Scenario: A wrong archive, a foreign mirror and a cancellation leave nothing behind
    Given a mirror that serves bytes other than the pinned ones, a plain http mirror that is not loopback and a mirror that never finishes
    When the launcher installer runs against each of them
    Then the checksum mismatch and the foreign mirror are refused in both languages
    And the cancelled run exits with 130 and leaves no partial directory, lock or launcher

  Scenario: A running launcher is replaced on Windows
    Given a repo.exe that a build still runs
    When the Windows installer installs the new launcher
    Then the old file is renamed aside, the new one takes its name, and the next install removes the moved-aside copy

  Scenario: The new launcher cannot take the name on Windows
    Given a repo.exe that is renamed aside and a new file that Defender still holds
    When the new file cannot take the name after the retries of 10 to 1280 ms
    Then the old launcher is moved back and the install fails, so repo.exe is never missing

  Scenario: One command takes a bare machine to a working repo and repo mcp
    Given a machine without Go, without a launcher and without a native repo command
    When the launcher installer runs
    Then it builds the launcher, then runs the native installation with that launcher from the checkout root, with progress in English and German
    And it neither puts the launcher on PATH nor advises a link
    And a failed native installation fails with its exit code and leaves the launcher, and a cancellation stops the native installation and exits with 130

  Scenario: The launcher alone is built on request
    Given a CI runner or the full bootstrap, which chains the native installation itself
    When the launcher installer runs with the launcher-only option
    Then only the launcher is built and the native installation command is named in English and German

  Scenario: A PATH repo that is not the native command is named
    Given a native installation receipt and several repo commands on PATH
    When the launcher installer finishes
    Then every PATH repo other than the receipt's executable is named with the command that installs the native one, in English and German
    And the receipt's executable and links to it stay silent

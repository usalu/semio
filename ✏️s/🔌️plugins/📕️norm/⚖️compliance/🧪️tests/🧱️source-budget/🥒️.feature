@capability-norm-source-budget
Feature: Every crate stays inside the source boundary
  A crate may span at most 32 files, 262144 bytes and 4000 lines of its module closure. The Python check `🐍️.py` reads the Cargo manifests and Rust sources as text and is independent of the Rust kernel; it warns from 85 % of a limit so owners split along domain lines before a build fails, and it fails above 100 %.

  Scenario: No crate is over a limit
    Given the committed norm crates
    When the module closure of every crate is measured
    Then no crate exceeds a limit

  Scenario: Crates close to a limit are reported
    Given a crate above 85 % of a limit
    When the check runs
    Then it prints a warning for the crate and still passes

  Scenario: Embedded fixtures count against the limit
    Given a crate that embeds a showcase document with include_str
    When the module closure of the crate is measured
    Then the bytes and lines of the document count toward the crate, so a showcase fixture lives in its own crate and is written compact

@capability-norm-path-budget
Feature: No path of the norm plugin exceeds the byte budget
  Every file and folder below `✏️s/🔌️plugins/📕️norm` has a path of at most 240 UTF-8 bytes relative to the repository root, so native Windows, macOS and Linux checkouts stay below 256 characters with 16 bytes of headroom for the checkout prefix and the next kind. The Python check `🐍️.py` walks the tree twice with independent walkers and cross-checks `git ls-files`; it is independent of the Rust kernel and of the mutation vector harness.

  Scenario: Every norm path fits the budget
    Given the committed norm plugin tree
    When every path is measured as UTF-8 bytes relative to the repository root
    Then no path exceeds 240 bytes

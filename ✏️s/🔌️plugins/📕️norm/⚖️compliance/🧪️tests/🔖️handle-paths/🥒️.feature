@capability-norm-handle-paths
Feature: Every handle writes a path that has an edit rule
  Each artifact's scene handles write fields by path. The path literal, normalised to `[]` indices, must be a `SetFieldRule` path of the artifact's edit rules, a member-relative suffix of one, or the entity container of one. The Python check `🐍️.py` reads the Rust sources as text and is independent of the Rust kernel.

  Scenario: Handle write paths resolve to edit rules
    Given the committed norm artifacts
    When every handle write path literal of a scene is normalised
    Then each path is covered by an edit rule of the same artifact

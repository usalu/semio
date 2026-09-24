@capability-wfc-wfc3d-1-any-editor-transient-mutate
@no-oracle-wfc-wfc3d-1-any-editor-transient-state-lane-semantics
@comparison-ordered-json-v1
@mutations-wfc-wfc3d-1-any-editor-transient
Feature: Apply every transient state-lane mutation of s.wfc.wfc3d's ✏️editor/🫧️transient to its committed vector
  `Wfc3dTransientMutation` is the transient lane of this subset's editor: it holds the last solve's slot assignments and whether it ended in a contradiction. It is this
  repository's own state record, so the case records the no-oracle decision `wfc-wfc3d-1-any-editor-transient-state-lane-semantics` in
  `✏️editor/🫧️transient/🔮️oracles/🔣️.json` and asserts every law inside the subject handlers through the shared
  `semio_s_plugin_stdio_test_oracle::law::vector` module: the applied snapshot is the committed after-snapshot, the
  produced delta is the committed `🔺️diff`, the diagnostics are the ones the committed `🎯️outcome` declares, an
  applied vector really moves the snapshot, and the mutation's own computed inverse restores the committed
  before-snapshot. Production dispatch is reached through `wfc3d_transient_mutation_report_json`, which runs
  `Mutation::diff(..).apply_to` exactly as the store does.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> and land on the committed after-snapshot, diff and outcome
    Given the committed <id> vector under ✏️editor/🫧️transient/🧫️fixtures
    When <id> is applied to that vector's before-snapshot through wfc3d_transient_mutation_report_json
    Then the applied snapshot, the produced diff and the diagnostics are exactly what the vector commits, and the snapshot moved
    Examples:
      | id |
      | set-solve |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed <id> vector under ✏️editor/🫧️transient/🧫️fixtures
    When <id> is applied to that vector's before-snapshot through wfc3d_transient_mutation_report_json
    Then the mutation's own inverse steps apply without refusal and restore the before-snapshot exactly
    Examples:
      | id |
      | set-solve |

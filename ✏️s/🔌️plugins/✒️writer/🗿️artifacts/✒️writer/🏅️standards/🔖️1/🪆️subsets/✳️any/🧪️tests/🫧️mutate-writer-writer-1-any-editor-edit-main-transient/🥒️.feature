@capability-writer-writer-1-any-editor-edit-main-transient-mutate
@no-oracle-writer-writer-1-any-editor-edit-main-transient-state-lane-semantics
@comparison-ordered-json-v1
@mutations-writer-writer-1-any-editor-edit-main-transient
Feature: Apply every transient state-lane mutation of s.writer.writer's ✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient to its committed vector
  `WriterMainWindowTransientMutation` is the transient lane of this subset's editor: it holds the editor selection, the lint generation counter and the engagement input of the main window. It is this
  repository's own state record, so the case records the no-oracle decision `writer-writer-1-any-editor-edit-main-transient-state-lane-semantics` in
  `✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🔮️oracles/🔣️.json` and asserts every law inside the subject handlers through the shared
  `semio_s_plugin_stdio_test_oracle::law::vector` module: the applied snapshot is the committed after-snapshot, the
  produced delta is the committed `🔺️diff`, the diagnostics are the ones the committed `🎯️outcome` declares, an
  applied vector really moves the snapshot, and the mutation's own computed inverse restores the committed
  before-snapshot. Production dispatch is reached through `writer_main_window_transient_mutation_report_json`, which runs
  `Mutation::diff(..).apply_to` exactly as the store does.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> and land on the committed after-snapshot, diff and outcome
    Given the committed <id> vector under ✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧫️fixtures
    When <id> is applied to that vector's before-snapshot through writer_main_window_transient_mutation_report_json
    Then the applied snapshot, the produced diff and the diagnostics are exactly what the vector commits, and the snapshot moved
    Examples:
      | id |
      | set-engagement-input |
      | set-editor-selection |
      | set-lint-generation |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed <id> vector under ✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧫️fixtures
    When <id> is applied to that vector's before-snapshot through writer_main_window_transient_mutation_report_json
    Then the mutation's own inverse steps apply without refusal and restore the before-snapshot exactly
    Examples:
      | id |
      | set-engagement-input |
      | set-editor-selection |
      | set-lint-generation |

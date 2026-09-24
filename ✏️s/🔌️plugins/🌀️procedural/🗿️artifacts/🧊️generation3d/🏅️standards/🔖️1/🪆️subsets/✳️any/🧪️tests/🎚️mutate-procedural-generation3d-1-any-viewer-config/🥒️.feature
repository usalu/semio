@capability-procedural-generation3d-1-any-viewer-config-mutate
@no-oracle-procedural-generation3d-1-any-viewer-config-state-lane-semantics
@comparison-ordered-json-v1
@mutations-procedural-generation3d-1-any-viewer-config
Feature: Apply every config state-lane mutation of s.procedural.generation3d's 👁️viewer/🎚️config to its committed vector
  `Generation3dViewConfigMutation` is the config lane of this subset's viewer: it holds the LOD tier, the preview shading mode, the 3d preview camera, the sun and which bundled example the read-only surface shows. It is this
  repository's own state record, so the case records the no-oracle decision `procedural-generation3d-1-any-viewer-config-state-lane-semantics` in
  `👁️viewer/🎚️config/🔮️oracles/🔣️.json` and asserts every law inside the subject handlers through the shared
  `semio_s_plugin_stdio_test_oracle::law::vector` module: the applied snapshot is the committed after-snapshot, the
  produced delta is the committed `🔺️diff`, the diagnostics are the ones the committed `🎯️outcome` declares, an
  applied vector really moves the snapshot, and the mutation's own computed inverse restores the committed
  before-snapshot. Production dispatch is reached through `generation3d_view_config_mutation_report_json`, which runs
  `Mutation::diff(..).apply_to` exactly as the store does.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> and land on the committed after-snapshot, diff and outcome
    Given the committed <id> vector under 👁️viewer/🎚️config/🧫️fixtures
    When <id> is applied to that vector's before-snapshot through generation3d_view_config_mutation_report_json
    Then the applied snapshot, the produced diff and the diagnostics are exactly what the vector commits, and the snapshot moved
    Examples:
      | id |
      | set-sun |
      | set-active-example |
      | set-show-mode |
      | set-preview-camera |
      | set-lod-mode |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed <id> vector under 👁️viewer/🎚️config/🧫️fixtures
    When <id> is applied to that vector's before-snapshot through generation3d_view_config_mutation_report_json
    Then the mutation's own inverse steps apply without refusal and restore the before-snapshot exactly
    Examples:
      | id |
      | set-sun |
      | set-active-example |
      | set-show-mode |
      | set-preview-camera |
      | set-lod-mode |

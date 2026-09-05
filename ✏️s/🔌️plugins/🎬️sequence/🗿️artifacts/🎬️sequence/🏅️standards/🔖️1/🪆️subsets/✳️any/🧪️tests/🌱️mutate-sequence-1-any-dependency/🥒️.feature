@capability-sequence-1-dependency-mutate
@no-oracle-sequence-step-graph-mutation-semantics
@comparison-ordered-json-v1
@mutations-sequence-1-any-dependency
Feature: Apply every typed SEQUENCE dependency mutation through the shared envelope-level dispatch
  `s.sequence.sequence`'s two dependency-edge kinds were relocated to `🔗️dependency` in ticket
  `SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`, with its own
  subset-owned case. `✳️any` still owns the SHARED aggregate `SequenceMutation` enum and its wire
  codec (`🧬️schema/🧬️mutations`, `🚪️io/🧬️mutations`) both of those kinds dispatch through —
  `apply_sequence_mutation`/`inverse_sequence_mutation`, the same functions `🔗️dependency`'s own case
  itself calls. This case exhaustively exercises that shared dispatch surface directly, one kind at a
  time, over a local copy of the same real committed step graph `🔗️dependency`'s own case uses — the
  lawful, separate-implementation answer B3 already established for a shared fixture reused across
  case boundaries, not a duplicate of `🔗️dependency`'s own coverage: that measures each kind's OWN
  semantics; this measures that the shared aggregate wrapper dispatches both of them correctly. It
  claims the SAME `sequence-1-dependency-mutate` capability `🔗️dependency`'s own case claims, since
  both cases genuinely test that one capability, from two different owners.

  This case carries the same recorded no-oracle decision (`sequence-step-graph-mutation-semantics`,
  in `../🔮️oracle/🔣️.json`) as `🔗️dependency`'s own case, for the identical reason: `s.sequence.sequence`
  is semio-native, so no third-party reader or writer exists for the aggregate wrapper either.

  📄️ The base document is real and committed. `local://🗣️.dsl.semio` is parsed by production's own
  `parse_dsl` and supplies the document skeleton every scenario starts from; the composed content
  child (steps and edges) is seeded from `local://🎬️base-scene.json`, a local copy of the identical
  fixture `🔗️dependency`'s own case already carries.

  ⚖️ The projection is `(schema, steps, edges)` read back through `sequence_working_scene`, exactly as
  in `🔗️dependency`'s own case.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> through the shared aggregate dispatch and observe it move
    Given the real committed sequence artifact local://🗣️.dsl.semio
    And its composed content child seeded from local://🎬️base-scene.json
    When the <id> mutation is applied through apply_sequence_mutation
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the resulting projection differs from the base projection
    Examples:
      | id                    | params                                                                            |
      | connect-steps         | {"mutation":"connectSteps","id":"edge-tail","from":"step-sink","to":"step-tail"} |
      | disconnect-steps      | {"mutation":"disconnectSteps","id":"edge-main"}                                  |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the shared aggregate document exactly
    Given the real committed sequence artifact local://🗣️.dsl.semio
    And its composed content child seeded from local://🎬️base-scene.json
    When the <id> mutation is applied through apply_sequence_mutation
      """
      {"kind": "<id>", "params": <params>}
      """
    And every step of its own computed inverse is applied through apply_sequence_mutation
    Then the projection equals the base projection again
    Examples:
      | id                    | params                                                                            |
      | connect-steps         | {"mutation":"connectSteps","id":"edge-tail","from":"step-sink","to":"step-tail"} |
      | disconnect-steps      | {"mutation":"disconnectSteps","id":"edge-main"}                                  |

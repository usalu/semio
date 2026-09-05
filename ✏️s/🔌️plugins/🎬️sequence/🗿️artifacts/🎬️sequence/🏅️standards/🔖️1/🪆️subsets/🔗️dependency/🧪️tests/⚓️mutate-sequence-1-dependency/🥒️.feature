@capability-sequence-1-dependency-mutate
@no-oracle-sequence-step-graph-mutation-semantics
@comparison-ordered-json-v1
@mutations-sequence-1-dependency
Feature: Apply every typed SEQUENCE dependency mutation to the real committed step graph
  `s.sequence.sequence` is a semio-NATIVE artifact — the `sequence.sequence.dsl` envelope is defined
  by this repository alone and no package in any ecosystem reads it — so this case carries a recorded
  no-oracle decision (`sequence-step-graph-mutation-semantics`, in
  `../../../✳️any/🔮️oracle/🔣️.json`) whose survey names and DECLINES the BPMN and Graphviz readers
  on the substantive ground that neither models this graph.

  ⚠️ THIS NO-ORACLE DECISION IS A DEBT, NOT A VERDICT, and is recorded as one. What blocks a second
  implementation TODAY is stated in the decision: this case's vectors are not declared as `asset://`
  fixtures — the `Examples` table carries the payloads inline — so the plan pins none of their digests
  and a Python reference cannot read them at all. Until that is done, every assertion below still
  lives in the SUBJECT role.

  📄️ The base document is real and committed. `local://🗣️.dsl.semio`
  is parsed by production's own `parse_dsl` and supplies the document skeleton every scenario starts
  from. What the committed artifact cannot supply is the graph itself: `SequenceSnapshot` keeps its
  steps and edges in a composed `s.stdio.semio.flow` CHILD and the `.sequence` DSL persists the child
  HANDLE, not the child, so the three steps and one edge are committed once in
  `local://🎬️base-scene.json`, derived from this vocabulary's own committed per-kind leaf fixtures
  under the sibling `🪜️step` subset for the step verbs and this subset's own committed leaves for
  `connect`/`disconnect`.

  🧬️ The step-to-step relationship is owned here because it is genuinely a different address space
  from a step's own identity: `connect-steps`/`disconnect-steps` name an EDGE by its own id and the
  two step ids it joins, never a list index or a step's coordinate. There is no `no-mutation` and no
  `set-snapshot`: whole-document replacement is not expressible as an in-history mutation in this
  generation of the taxonomy and goes through `ArtifactStore::reset` instead.

  ⚖️ The projection is `(schema, steps, edges)` read back through `sequence_working_scene`. The
  composed content handle is deliberately NOT projected: `sequence_content_child_handle`
  content-addresses exactly that pair with `std`'s deliberately unspecified `DefaultHasher`, so
  projecting the handle would compare the same content twice and pin a value the standard library
  does not promise.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to the real committed step graph and observe it move
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
  Scenario Outline: Undoing <id> restores the real committed step graph exactly
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

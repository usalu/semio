@capability-sequence-1-step-mutate
@no-oracle-sequence-step-graph-mutation-semantics
@comparison-ordered-json-v1
@mutations-sequence-1-step
Feature: Apply every typed SEQUENCE step mutation to the real committed step graph
  `s.sequence.sequence` is a semio-NATIVE artifact — the `sequence.sequence.dsl` envelope is defined
  by this repository alone and no package in any ecosystem reads it — so this case carries a recorded
  no-oracle decision (`sequence-step-graph-mutation-semantics`, in
  `../../../✳️any/🧪️oracle/🔣️.json`) whose survey names and DECLINES the BPMN and Graphviz readers
  on the substantive ground that neither models this graph.

  ⚠️ THIS NO-ORACLE DECISION IS A DEBT, NOT A VERDICT, and is recorded as one. `mutate-puzzle-2d-1` and
  `mutate-puzzle-3d-1` took Python second implementations over this same `.dsl.semio` carrier in this
  wave, so the same is writable for this subset. What blocks it TODAY is stated in the decision: this
  case's vectors are not declared as `asset://` fixtures — the `Examples` table carries the payloads
  inline and the adapter reads them through the scenario's own doc string — so the plan pins none of
  their digests and a Python reference cannot read them at all. Until that is done, every assertion
  below still lives in the SUBJECT role.

  📄️ The base document is real and committed. `local://🗣️.dsl.semio`
  is parsed by production's own `parse_dsl` and supplies the document skeleton every scenario starts
  from. What the committed artifact cannot supply is the graph itself: `SequenceSnapshot` keeps its
  steps and edges in a composed `s.stdio.semio.flow` CHILD and the `.sequence` DSL persists the child
  HANDLE, not the child, so the three steps and one edge are committed once in
  `local://🎬️base-scene.json`, derived from this vocabulary's own committed per-kind leaf fixtures.

  🧬️ `steps` is an id-keyed ORDERED collection with a spatial position, so this subset owns
  `create-step`/`delete-step`/`move-step`/`edit-step-params`/`change-step-collapsed` plus the composite
  `duplicate-step` — there is no reorder verb here, unlike the sibling `flow` and `present`
  vocabularies, because a sequence step is addressed by its canvas coordinate and never by a list
  index. Every `params` cell below is the mutation's own internally-tagged JSON and is chosen to MOVE
  the projection against the base; `delete-step` deliberately addresses `step-tail`, the one step no
  edge touches, so the kind is measured on its own and not on the edge cascade its diff also performs.

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
      | id                    | params                                                                                                                                               |
      | create-step           | {"mutation":"createStep","step":{"id":"step-fan","kind":"log.print","params":{"message":"fan out"},"x":840.0,"y":0.0,"slot":null,"collapsed":false}} |
      | delete-step           | {"mutation":"deleteStep","id":"step-tail"}                                                                                                           |
      | move-step             | {"mutation":"moveStep","id":"step-log","x":280.0,"y":0.0}                                                                                            |
      | edit-step-params      | {"mutation":"editStepParams","id":"step-log","params":{"message":"goodbye sequence"}}                                                                |
      | change-step-collapsed | {"mutation":"changeStepCollapsed","id":"step-log","collapsed":true}                                                                                  |
      | duplicate-step        | {"mutation":"duplicateStep","sourceId":"step-log","newId":"step-copy","x":560.0,"y":0.0}                                                             |

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
      | id                    | params                                                                                                                                               |
      | create-step           | {"mutation":"createStep","step":{"id":"step-fan","kind":"log.print","params":{"message":"fan out"},"x":840.0,"y":0.0,"slot":null,"collapsed":false}} |
      | delete-step           | {"mutation":"deleteStep","id":"step-tail"}                                                                                                           |
      | move-step             | {"mutation":"moveStep","id":"step-log","x":280.0,"y":0.0}                                                                                            |
      | edit-step-params      | {"mutation":"editStepParams","id":"step-log","params":{"message":"goodbye sequence"}}                                                                |
      | change-step-collapsed | {"mutation":"changeStepCollapsed","id":"step-log","collapsed":true}                                                                                  |
      | duplicate-step        | {"mutation":"duplicateStep","sourceId":"step-log","newId":"step-copy","x":560.0,"y":0.0}                                                             |

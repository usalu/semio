@capability-sequence-1-mutate
@no-oracle-sequence-step-graph-mutation-semantics
@comparison-ordered-json-v1
@mutations-sequence-1-any
Feature: Apply every typed SEQUENCE mutation to the real committed step graph
  `s.sequence.sequence` is a semio-NATIVE artifact — the `sequence.sequence.dsl` envelope is defined
  by this repository alone and no package in any ecosystem reads it — so this case carries a recorded
  no-oracle decision (`sequence-step-graph-mutation-semantics`, in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`) whose survey names and DECLINES
  the BPMN and Graphviz readers on the substantive ground that neither models this graph.

  ⚠️ THIS NO-ORACLE DECISION IS A DEBT, NOT A VERDICT, and is recorded as one. Declining a third-party
  LIBRARY is a different judgement from declining a SECOND IMPLEMENTATION, and only the first was ever
  made here. `mutate-puzzle-2d-1` and `mutate-puzzle-3d-1` took Python second
  implementations over this same `.dsl.semio` carrier in this wave, so the same is writable for this
  subset from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`, the rules of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` and the
  committed vectors this feature already replays. What blocks it TODAY is stated in the decision and
  is one edit: this case's vectors are not declared as `asset://` fixtures — the `Examples` table
  carries the payloads inline and the adapter reads the committed files through `include_str!` — so
  the plan pins none of their digests and a Python reference cannot read them at all. Separately, `identity-round-trip` would still be refused: this subset's committed
  snapshot text grammar is the repository-wide placeholder `payload = OCTET+`, whose header production
  declares `"schema" SP "stdio.json"` against an artifact whose own first line says otherwise.
  Until that is done, every assertion below still lives in the SUBJECT role, and the ceiling is the
  one this decision has always had: no second producer runs beside it, so a mistake shared by the
  handcrafted vector and the production code passes unseen.

  📄️ The base document is real and committed. `asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` is
  parsed by production's own `parse_dsl` and supplies the document skeleton every scenario starts from.
  What the committed artifact cannot supply is the graph itself: `SequenceSnapshot` keeps its steps and
  edges in a composed `s.stdio.semio.flow` CHILD and the `.sequence` DSL persists the child HANDLE, not
  the child — the whole committed file is a schema line and two bracketed hex handles — so a case that
  only parsed it would find an empty graph and every id-keyed kind would address nothing. The three steps
  and one edge are therefore committed once in `local://🎬️base-scene.json`, derived from this
  vocabulary's OWN committed per-kind leaf fixtures under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/`; that file records which id,
  kind, params and coordinate came from which committed payload and which are this case's derivation.

  🧬️ The vocabulary is `SequenceMutation`'s eight variants in declaration order and it is genuinely this
  subset's own: `steps` is an id-keyed ORDERED collection with a spatial position, so it takes
  `create`/`delete`/`move`/`edit-step-params`/`change-step-collapsed` plus the composite
  `duplicate-step`, while the step-to-step relationship takes `connect`/`disconnect` — there is no
  reorder verb here at all, unlike the sibling `flow` and `present` vocabularies, because a sequence step
  is addressed by its canvas coordinate and never by a list index. There is no `no-mutation` and no
  `set-snapshot`: whole-document replacement is not expressible as an in-history mutation in this
  generation of the taxonomy and goes through `ArtifactStore::reset` instead. Every `params` cell below
  is the mutation's own internally-tagged JSON and is chosen to MOVE the projection against that base;
  `delete-step` deliberately addresses `step-tail`, the one step no edge touches, so the kind is measured
  on its own and not on the edge cascade its diff also performs.

  ⚖️ The projection is `(schema, steps, edges)` read back through `sequence_working_scene`. The composed
  content handle is deliberately NOT projected: `sequence_content_child_handle` content-addresses exactly
  that pair with `std`'s deliberately unspecified `DefaultHasher`, so projecting the handle would compare
  the same content twice and pin a value the standard library does not promise.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to the real committed step graph and observe it move
    Given the real committed sequence artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
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
      | connect-steps         | {"mutation":"connectSteps","id":"edge-tail","from":"step-sink","to":"step-tail"}                                                                     |
      | disconnect-steps      | {"mutation":"disconnectSteps","id":"edge-main"}                                                                                                      |
      | duplicate-step        | {"mutation":"duplicateStep","sourceId":"step-log","newId":"step-copy","x":560.0,"y":0.0}                                                             |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real committed step graph exactly
    Given the real committed sequence artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
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
      | connect-steps         | {"mutation":"connectSteps","id":"edge-tail","from":"step-sink","to":"step-tail"}                                                                     |
      | disconnect-steps      | {"mutation":"disconnectSteps","id":"edge-main"}                                                                                                      |
      | duplicate-step        | {"mutation":"duplicateStep","sourceId":"step-log","newId":"step-copy","x":560.0,"y":0.0}                                                             |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real committed sequence artifact
    Given the real committed sequence artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When it is parsed with parse_dsl and printed back with print_dsl
    Then the printed bytes are identical to the committed bytes and reparsing preserves the projection
      """
      {"kind": "identity-round-trip", "params": {"carrier": "byte-exact"}}
      """

@capability-flow-1-mutate
@no-oracle-flow-widget-graph-mutation-semantics
@comparison-ordered-json-v1
@mutations-flow-1-any
Feature: Apply every typed FLOW mutation to the real committed widget graph
  `s.flow.flow` is a semio-NATIVE artifact — the `flow.flow` envelope is defined by this repository
  alone and no package in any ecosystem reads it — so this case carries a recorded no-oracle decision
  (`flow-widget-graph-mutation-semantics`, in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`). That decision's third-party
  survey is argued rather than assumed: this document's body is plain JSON and `json-rust` is already
  linked into the stdio oracle crate, and it is declined because a generic DOM reader knows nothing of
  a widget discriminant, a synapse port pair or the cascade `delete-widget` performs.

  ⚠️ THIS NO-ORACLE DECISION IS A DEBT, NOT A VERDICT, and is recorded as one. Declining a third-party
  LIBRARY is a different judgement from declining a SECOND IMPLEMENTATION, and only the first was ever
  made here. `mutate-procedural-2d-1` and `mutate-procedural-3d-1` took Python second
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
  parsed by production's own `parse_dsl` and supplies the schema and the CAMERA every scenario starts
  from — the camera is inline document state on `FlowSnapshot`, not composed content, so it comes from
  the file and from nowhere else. What the committed artifact cannot supply is the graph:
  `FlowSnapshot` keeps its widgets, synapses and layout in a composed `s.stdio.semio.flow` CHILD and the
  `.flow` DSL persists the child HANDLE, not the child, so a case that only parsed it would find an empty
  graph and every id-keyed kind would address nothing. The four widgets, two synapses and two layout
  entries are therefore committed once in `local://🌊️base-scene.json`, derived from this vocabulary's OWN
  committed per-kind leaf fixtures under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/`; that file records which
  widget, synapse, port and layout entry came from which committed payload and which are this case's
  derivation, and why each derived one had to exist.

  🧬️ The vocabulary is `FlowMutation`'s ten variants in declaration order and it is genuinely this
  subset's own. Two id-keyed ORDERED collections each take the full per-collection recipe — `widgets`
  gets `create`/`delete`/`reorder`/`replace`, `synapses` gets `connect`/`disconnect`/`reorder`/
  `update-synapse-endpoints` — plus one whole-layout verb (`move-widgets`, which carries a batch of
  layout entries because a canvas drag moves a selection, not a node) and one COMPOSITE
  (`duplicate-widget`, which plans a widget insert and a synapse insert together and is the one variant
  with no framework-generic counterpart at all). There is no `no-mutation` and no `set-snapshot`:
  whole-document replacement is banned by the taxonomy and goes through `ArtifactStore::reset`. Every
  `params` cell below is the mutation's own internally-tagged JSON and is chosen to MOVE the projection
  against that base. Note the two spellings: every payload but one renames its fields to camelCase, while
  `duplicate-widget` — the composite, whose bytes are canonical JSON of its own payload rather than a
  framework-bridged op — carries none and is therefore snake_case, exactly as its own committed leaf
  fixture is. Two rows are deliberately NOT the committed leaf payload: `move-widgets` and
  `update-synapse-endpoints` each have a committed fixture that pins a NO-OP branch (re-applying the
  current layout, re-declaring the same endpoints), and a no-op cannot satisfy the observability law.
  `delete-widget` deliberately addresses `note-omega`, the one widget no synapse touches, so the kind is
  measured on its own and not on the cascade its diff also performs.

  ⚖️ The projection is `(schema, camera, widgets, synapses, layout)` read back through
  `flow_working_scene`. The content handle is deliberately NOT projected: `flow_content_child_handle`
  content-addresses exactly that triple with domain-separated SHA-256. Dedicated cross-language
  identity fixtures pin canonical bytes and digests; this mutation projection measures semantic
  behavior without comparing the same content twice.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to the real committed widget graph and observe it move
    Given the real committed flow artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    And its composed content child seeded from local://🌊️base-scene.json
    When the <id> mutation is applied through apply_flow_mutation
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the resulting projection differs from the base projection
    Examples:
      | id                       | params                                                                                                                                             |
      | create-widget            | {"mutation":"createWidget","index":0,"widget":{"kind":"inputNote","id":"note-delta","text":"Delta"}}                                               |
      | delete-widget            | {"mutation":"deleteWidget","id":"note-omega"}                                                                                                      |
      | reorder-widgets          | {"mutation":"reorderWidgets","id":"note-beta","toIndex":9}                                                                                         |
      | replace-widget           | {"mutation":"replaceWidget","id":"note-alpha","widget":{"kind":"inputNote","id":"note-alpha","text":"Alpha Prime"}}                                |
      | connect-widgets          | {"mutation":"connectWidgets","index":2,"id":"synapse-2","from":"note-gamma","fromPort":"out","to":"note-omega","toPort":"in"}                      |
      | disconnect-widgets       | {"mutation":"disconnectWidgets","id":"synapse-3"}                                                                                                  |
      | reorder-synapses         | {"mutation":"reorderSynapses","id":"synapse-1","toIndex":1}                                                                                        |
      | update-synapse-endpoints | {"mutation":"updateSynapseEndpoints","id":"synapse-1","from":"note-alpha","fromPort":"out","to":"note-gamma","toPort":"in"}                        |
      | move-widgets             | {"mutation":"moveWidgets","entries":[{"id":"note-alpha","layout":{"x":40.0,"y":80.0}}]}                                                            |
      | duplicate-widget         | {"mutation":"duplicateWidget","source_id":"note-alpha","new_id":"note-copy","synapse_id":"synapse-alpha-to-copy","from_port":"out","to_port":"in"} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real committed widget graph exactly
    Given the real committed flow artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    And its composed content child seeded from local://🌊️base-scene.json
    When the <id> mutation is applied through apply_flow_mutation
      """
      {"kind": "<id>", "params": <params>}
      """
    And every step of its own computed inverse is applied through apply_flow_mutation
    Then the projection equals the base projection again
    Examples:
      | id                       | params                                                                                                                                             |
      | create-widget            | {"mutation":"createWidget","index":0,"widget":{"kind":"inputNote","id":"note-delta","text":"Delta"}}                                               |
      | delete-widget            | {"mutation":"deleteWidget","id":"note-omega"}                                                                                                      |
      | reorder-widgets          | {"mutation":"reorderWidgets","id":"note-beta","toIndex":9}                                                                                         |
      | replace-widget           | {"mutation":"replaceWidget","id":"note-alpha","widget":{"kind":"inputNote","id":"note-alpha","text":"Alpha Prime"}}                                |
      | connect-widgets          | {"mutation":"connectWidgets","index":2,"id":"synapse-2","from":"note-gamma","fromPort":"out","to":"note-omega","toPort":"in"}                      |
      | disconnect-widgets       | {"mutation":"disconnectWidgets","id":"synapse-3"}                                                                                                  |
      | reorder-synapses         | {"mutation":"reorderSynapses","id":"synapse-1","toIndex":1}                                                                                        |
      | update-synapse-endpoints | {"mutation":"updateSynapseEndpoints","id":"synapse-1","from":"note-alpha","fromPort":"out","to":"note-gamma","toPort":"in"}                        |
      | move-widgets             | {"mutation":"moveWidgets","entries":[{"id":"note-alpha","layout":{"x":40.0,"y":80.0}}]}                                                            |
      | duplicate-widget         | {"mutation":"duplicateWidget","source_id":"note-alpha","new_id":"note-copy","synapse_id":"synapse-alpha-to-copy","from_port":"out","to_port":"in"} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real committed flow artifact
    Given the real committed flow artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When it is parsed with parse_dsl and printed back with print_dsl
    Then the printed bytes are identical to the committed bytes and reparsing preserves the projection
      """
      {"kind": "identity-round-trip", "params": {"carrier": "byte-exact"}}
      """

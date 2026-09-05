@capability-flow-1-mutate
@oracle-flow-1-python-independent
@comparison-ordered-json-v1
@mutations-flow-1-any
Feature: Apply every typed FLOW mutation to the real committed widget graph and against an independent Python implementation
  `s.flow.flow` is a semio-NATIVE artifact — the `flow.flow` envelope is defined by this repository
  alone and no package in any ecosystem reads it. This subset's own third-party survey is argued
  rather than assumed: this document's body is plain JSON and `json-rust` is already linked into the
  stdio oracle crate, and it is declined because a generic DOM reader knows nothing of a widget
  discriminant, a synapse port pair or the cascade `delete-widget` performs. The second producer a
  differential comparison needs is therefore a second IMPLEMENTATION, and `🐍️component.py` beside
  this file is it: all ten kinds of this vocabulary, written in Python from this subset's own
  committed `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json` and from
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
  `create`/`delete`/`reorder`/`replace`/`connect`/`disconnect`/`update`/`move`/`duplicate` verb
  entries and `📓️derivation-rules.md`'s per-ordered-collection recipe. It imports nothing from the
  Rust it judges and transliterates none of it. The no-oracle decision this replaces
  (`flow-widget-graph-mutation-semantics`) is narrowed to an empty `capabilities` list rather than
  deleted, because its own investigation remains the honest record of what was checked.

  Both implementations now read the SAME committed base graph — `local://🔣️.json` below, already
  declared as this case's local fixture — and apply the SAME ten committed `params` payloads to it.

  📄️ The base document is real and committed. `asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio` is
  parsed by production's own `parse_dsl` and supplies the schema and the CAMERA every scenario starts
  from — the camera is inline document state on `FlowSnapshot`, not composed content, so it comes from
  the file and from nowhere else. What the committed artifact cannot supply is the graph:
  `FlowSnapshot` keeps its widgets, synapses and layout in a composed `s.stdio.semio.flow` CHILD and the
  `.flow` DSL persists the child HANDLE, not the child, so a case that only parsed it would find an empty
  graph and every id-keyed kind would address nothing. The four widgets, two synapses and two layout
  entries are therefore committed once in `local://🔣️.json`, derived from this vocabulary's OWN
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
  @mode-differential
  Scenario Outline: Apply <id> to the real committed widget graph and observe it move
    Given the real committed flow artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    And its composed content child seeded from local://🔣️.json
    When the <id> mutation is applied through apply_flow_mutation, and separately by the Python reference
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the resulting projection differs from the base projection in both implementations
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
      | duplicate-widget         | {"mutation":"duplicateWidget","sourceId":"note-alpha","newId":"note-copy","synapseId":"synapse-alpha-to-copy","fromPort":"out","toPort":"in"} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real committed widget graph exactly
    Given the real committed flow artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    And its composed content child seeded from local://🔣️.json
    When the <id> mutation is applied through apply_flow_mutation, and separately by the Python reference
      """
      {"kind": "<id>", "params": <params>}
      """
    And every step of its own computed inverse is applied through apply_flow_mutation, and separately by the Python reference
    Then the projection equals the base projection again in both implementations
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
      | duplicate-widget         | {"mutation":"duplicateWidget","sourceId":"note-alpha","newId":"note-copy","synapseId":"synapse-alpha-to-copy","fromPort":"out","toPort":"in"} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real committed flow artifact
    Given the real committed flow artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When it is parsed with parse_dsl and printed back with print_dsl
    Then the printed bytes are identical to the committed bytes and reparsing preserves the projection
      """
      {"kind": "identity-round-trip", "params": {"carrier": "byte-exact"}}
      """
